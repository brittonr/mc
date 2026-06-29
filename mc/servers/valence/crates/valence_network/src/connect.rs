//! Handles new connections to the server and handshake dispatch.

use std::io;
use std::net::SocketAddr;
use std::time::Duration;

use anyhow::{bail, Context};
use tokio::net::{TcpListener, TcpStream};
use tracing::{error, trace, warn};
use valence_server::protocol::packets::handshaking::HandshakeC2s;
use valence_server::protocol::{PacketDecoder, PacketEncoder};

use crate::legacy_ping::try_handle_legacy_ping;
use crate::login::handle_login;
use crate::packet_io::PacketIo;
use crate::session_core::{
    classify_connection_io_error, classify_handshake_address, classify_initial_connection_timeout,
    classify_session_transition, phase_after_handshake, ConnectionIoDecision,
    HandshakeAddressDecision, InitialConnectionDecision, SessionPhase,
};
use crate::status::handle_status;
use crate::SharedNetworkState;

const INITIAL_CONNECTION_TIMEOUT_SECS: u64 = 5;

/// Accepts new connections to the server as they occur.
pub(super) async fn do_accept_loop(shared: SharedNetworkState) {
    let listener = match TcpListener::bind(shared.0.address).await {
        Ok(listener) => listener,
        Err(e) => {
            error!("failed to start TCP listener: {e}");
            return;
        }
    };

    let timeout = Duration::from_secs(INITIAL_CONNECTION_TIMEOUT_SECS);

    loop {
        match shared.0.connection_sema.clone().acquire_owned().await {
            Ok(permit) => match listener.accept().await {
                Ok((stream, remote_addr)) => {
                    let shared = shared.clone();

                    tokio::spawn(async move {
                        let timeout_result = tokio::time::timeout(
                            timeout,
                            handle_connection(shared, stream, remote_addr),
                        )
                        .await;

                        if let Err(e) = timeout_result {
                            if matches!(
                                classify_initial_connection_timeout(true),
                                InitialConnectionDecision::WarnTimeout
                            ) {
                                warn!("initial connection timed out: {e}");
                            }
                        }

                        drop(permit);
                    });
                }
                Err(e) => {
                    error!("failed to accept incoming connection: {e}");
                }
            },
            // Closed semaphore indicates server shutdown.
            Err(_) => return,
        }
    }
}

async fn handle_connection(
    shared: SharedNetworkState,
    mut stream: TcpStream,
    remote_addr: SocketAddr,
) {
    trace!("handling connection");

    if let Err(e) = stream.set_nodelay(true) {
        error!("failed to set TCP_NODELAY: {e}");
    }

    match try_handle_legacy_ping(&shared, &mut stream, remote_addr).await {
        Ok(true) => return,
        Ok(false) => {}
        Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => {}
        Err(e) => {
            warn!("legacy ping ended with error: {e:#}");
        }
    }

    let io = PacketIo::new(stream, PacketEncoder::new(), PacketDecoder::new());

    if let Err(e) = handle_handshake(shared, io, remote_addr).await {
        if let Some(e) = e.downcast_ref::<io::Error>() {
            if matches!(
                classify_connection_io_error(e.kind()),
                ConnectionIoDecision::IgnoreUnexpectedEof
            ) {
                return;
            }
        }

        warn!("connection ended with error: {e:#}");
    }
}

/// Basic information about a client, provided at the beginning of the
/// connection.
#[derive(Default, Debug)]
pub struct HandshakeData {
    /// The protocol version of the client.
    pub protocol_version: i32,
    /// The address that the client used to connect.
    pub server_address: String,
    /// The port that the client used to connect.
    pub server_port: u16,
}

async fn handle_handshake(
    shared: SharedNetworkState,
    mut io: PacketIo,
    remote_addr: SocketAddr,
) -> anyhow::Result<()> {
    let handshake_packet = io.recv_packet::<HandshakeC2s>().await?;
    let next_phase = phase_after_handshake(handshake_packet.next_state);

    classify_session_transition(SessionPhase::Handshake, next_phase).map_err(|transition| {
        anyhow::anyhow!(
            "invalid session transition from {:?} to {:?}",
            transition.from,
            transition.to
        )
    })?;

    let handshake = HandshakeData {
        protocol_version: handshake_packet.protocol_version.0,
        server_address: handshake_packet.server_address.0.to_owned(),
        server_port: handshake_packet.server_port,
    };

    match classify_handshake_address(&shared.0.connection_mode, &handshake.server_address) {
        HandshakeAddressDecision::Accept => {}
        HandshakeAddressDecision::RejectTooLong { .. } => {
            bail!("handshake server address is too long");
        }
    }

    match next_phase {
        SessionPhase::Status => handle_status(shared, io, remote_addr, handshake)
            .await
            .context("handling status"),
        SessionPhase::Login => {
            match handle_login(&shared, &mut io, remote_addr, handshake)
                .await
                .context("handling login")?
            {
                Some((info, cleanup)) => {
                    classify_session_transition(SessionPhase::Login, SessionPhase::Play).map_err(
                        |transition| {
                            anyhow::anyhow!(
                                "invalid session transition from {:?} to {:?}",
                                transition.from,
                                transition.to
                            )
                        },
                    )?;

                    let client = io.into_client_args(
                        info,
                        shared.0.incoming_byte_limit,
                        shared.0.outgoing_byte_limit,
                        cleanup,
                    );

                    let _ = shared.0.new_clients_send.send_async(client).await;

                    Ok(())
                }
                None => Ok(()),
            }
        }
        SessionPhase::Handshake | SessionPhase::Play => {
            bail!("invalid post-handshake session phase")
        }
    }
}
