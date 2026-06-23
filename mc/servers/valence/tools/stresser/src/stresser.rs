use std::fmt;
use std::io::{self, ErrorKind};
use std::net::SocketAddr;
use std::num::NonZeroUsize;

use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use uuid::Uuid;
use valence_protocol::packets::handshaking::handshake_c2s::HandshakeNextState;
use valence_protocol::packets::handshaking::HandshakeC2s;
use valence_protocol::packets::login::{
    LoginCompressionS2c, LoginHelloC2s, LoginHelloS2c, LoginSuccessS2c,
};
use valence_protocol::packets::play::{
    KeepAliveC2s, KeepAliveS2c, PlayerPositionLookS2c, PositionAndOnGroundC2s, TeleportConfirmC2s,
};
use valence_protocol::var_int::VarInt;
use valence_protocol::{
    CompressionThreshold, Packet, PacketDecoder, PacketEncoder, PROTOCOL_VERSION,
};

use crate::report::LoadPhase;

const SESSION_POSITION_ON_GROUND: bool = true;

pub(crate) struct SessionParams {
    pub(crate) socket_addr: SocketAddr,
    pub(crate) session_name: String,
    pub(crate) read_buffer_size: NonZeroUsize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SessionFailure {
    phase: LoadPhase,
    message: String,
}

impl SessionFailure {
    pub(crate) fn phase(&self) -> LoadPhase {
        self.phase
    }

    pub(crate) fn message(&self) -> &str {
        &self.message
    }

    fn from_error(phase: LoadPhase, err: impl fmt::Display) -> Self {
        Self {
            phase,
            message: err.to_string(),
        }
    }

    fn from_message(phase: LoadPhase, message: impl Into<String>) -> Self {
        Self {
            phase,
            message: message.into(),
        }
    }

    pub(crate) fn task_join(message: impl Into<String>) -> Self {
        Self::from_message(LoadPhase::Play, message)
    }

    pub(crate) fn timeout(timeout: std::time::Duration) -> Self {
        Self::from_message(
            LoadPhase::Timeout,
            format!("session exceeded configured timeout of {timeout:?}"),
        )
    }
}

impl fmt::Display for SessionFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} phase failed: {}", self.phase.label(), self.message)
    }
}

impl std::error::Error for SessionFailure {}

impl LoadPhase {
    fn label(self) -> &'static str {
        match self {
            Self::ConfigValidation => "config_validation",
            Self::Connect => "connect",
            Self::Handshake => "handshake",
            Self::Login => "login",
            Self::Play => "play",
            Self::Timeout => "timeout",
            Self::DryRun => "dry_run",
            Self::Completed => "completed",
        }
    }
}

pub(crate) async fn make_session(params: &SessionParams) -> Result<(), SessionFailure> {
    let sock_addr = params.socket_addr;
    let sess_name = params.session_name.as_str();
    let rb_size = params.read_buffer_size.get();

    let mut conn = match TcpStream::connect(sock_addr).await {
        Ok(conn) => {
            println!("{sess_name} connected");
            conn
        }
        Err(err) => {
            eprintln!("{sess_name} connection failed");
            return Err(SessionFailure::from_error(LoadPhase::Connect, err));
        }
    };

    conn.set_nodelay(true)
        .map_err(|err| SessionFailure::from_error(LoadPhase::Connect, err))?;

    let mut dec = PacketDecoder::new();
    let mut enc = PacketEncoder::new();

    let server_addr_str = sock_addr.ip().to_string();

    let handshake_pkt = HandshakeC2s {
        protocol_version: VarInt(PROTOCOL_VERSION),
        server_address: server_addr_str.as_str().into(),
        server_port: sock_addr.port(),
        next_state: HandshakeNextState::Login,
    };

    enc.append_packet(&handshake_pkt)
        .map_err(|err| SessionFailure::from_error(LoadPhase::Handshake, err))?;

    enc.append_packet(&LoginHelloC2s {
        username: sess_name.into(),
        profile_id: Some(Uuid::new_v4()),
    })
    .map_err(|err| SessionFailure::from_error(LoadPhase::Handshake, err))?;

    let write_buf = enc.take();
    conn.write_all(&write_buf)
        .await
        .map_err(|err| SessionFailure::from_error(LoadPhase::Handshake, err))?;

    loop {
        dec.reserve(rb_size);

        let mut read_buf = dec.take_capacity();

        conn.readable()
            .await
            .map_err(|err| SessionFailure::from_error(LoadPhase::Login, err))?;

        match conn.try_read_buf(&mut read_buf) {
            Ok(0) => {
                return Err(SessionFailure::from_error(
                    LoadPhase::Login,
                    io::Error::from(ErrorKind::UnexpectedEof),
                ));
            }
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => continue,
            Err(e) => return Err(SessionFailure::from_error(LoadPhase::Login, e)),
            Ok(_) => (),
        };

        dec.queue_bytes(read_buf);

        if let Some(frame) = dec
            .try_next_packet()
            .map_err(|err| SessionFailure::from_error(LoadPhase::Login, err))?
        {
            match frame.id {
                LoginCompressionS2c::ID => {
                    let packet: LoginCompressionS2c = frame
                        .decode()
                        .map_err(|err| SessionFailure::from_error(LoadPhase::Login, err))?;
                    let threshold = packet.threshold.0;

                    dec.set_compression(CompressionThreshold(threshold));
                    enc.set_compression(CompressionThreshold(threshold));
                }

                LoginSuccessS2c::ID => {
                    break;
                }

                LoginHelloS2c::ID => {
                    return Err(SessionFailure::from_message(
                        LoadPhase::Login,
                        "encryption not implemented",
                    ));
                }

                _ => (),
            }
        }
    }

    println!("{sess_name} logged in");

    loop {
        while let Some(frame) = dec
            .try_next_packet()
            .map_err(|err| SessionFailure::from_error(LoadPhase::Play, err))?
        {
            match frame.id {
                KeepAliveS2c::ID => {
                    let packet: KeepAliveS2c = frame
                        .decode()
                        .map_err(|err| SessionFailure::from_error(LoadPhase::Play, err))?;
                    enc.clear();

                    enc.append_packet(&KeepAliveC2s { id: packet.id })
                        .map_err(|err| SessionFailure::from_error(LoadPhase::Play, err))?;
                    conn.write_all(&enc.take())
                        .await
                        .map_err(|err| SessionFailure::from_error(LoadPhase::Play, err))?;
                }

                PlayerPositionLookS2c::ID => {
                    let packet: PlayerPositionLookS2c = frame
                        .decode()
                        .map_err(|err| SessionFailure::from_error(LoadPhase::Play, err))?;
                    enc.clear();

                    enc.append_packet(&TeleportConfirmC2s {
                        teleport_id: packet.teleport_id,
                    })
                    .map_err(|err| SessionFailure::from_error(LoadPhase::Play, err))?;

                    enc.append_packet(&PositionAndOnGroundC2s {
                        position: packet.position,
                        on_ground: SESSION_POSITION_ON_GROUND,
                    })
                    .map_err(|err| SessionFailure::from_error(LoadPhase::Play, err))?;

                    conn.write_all(&enc.take())
                        .await
                        .map_err(|err| SessionFailure::from_error(LoadPhase::Play, err))?;
                }
                _ => (),
            }
        }

        dec.reserve(rb_size);

        let mut read_buf = dec.take_capacity();

        conn.readable()
            .await
            .map_err(|err| SessionFailure::from_error(LoadPhase::Play, err))?;

        match conn.try_read_buf(&mut read_buf) {
            Ok(0) => {
                return Err(SessionFailure::from_error(
                    LoadPhase::Play,
                    io::Error::from(ErrorKind::UnexpectedEof),
                ));
            }
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => continue,
            Err(e) => return Err(SessionFailure::from_error(LoadPhase::Play, e)),
            Ok(_) => (),
        };

        dec.queue_bytes(read_buf);
    }
}

#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;

    use super::*;

    const TEST_PORT: u16 = 25565;
    const READ_BUFFER_SIZE_BYTES: usize = 4096;
    const SESSION_TIMEOUT_MILLIS: u64 = 250;

    #[test]
    fn session_failure_includes_phase_and_message() {
        let failure = SessionFailure::from_message(LoadPhase::Login, "online mode required");

        assert_eq!(failure.phase(), LoadPhase::Login);
        assert_eq!(failure.message(), "online mode required");
        assert_eq!(
            failure.to_string(),
            "login phase failed: online mode required"
        );
    }

    #[test]
    fn session_timeout_failure_is_structured() {
        let failure =
            SessionFailure::timeout(std::time::Duration::from_millis(SESSION_TIMEOUT_MILLIS));

        assert_eq!(failure.phase(), LoadPhase::Timeout);
        assert!(failure
            .message()
            .contains("session exceeded configured timeout"));
    }

    #[test]
    fn session_params_hold_owned_name_and_nonzero_buffer() {
        let params = SessionParams {
            socket_addr: SocketAddr::from((Ipv4Addr::LOCALHOST, TEST_PORT)),
            session_name: "Load0".to_owned(),
            read_buffer_size: NonZeroUsize::new(READ_BUFFER_SIZE_BYTES).unwrap(),
        };

        assert_eq!(params.session_name, "Load0");
        assert_eq!(params.read_buffer_size.get(), READ_BUFFER_SIZE_BYTES);
    }
}
