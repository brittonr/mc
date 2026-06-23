use std::net::SocketAddr;

use clap::Parser;
use packet_inspector::capture::{summarize_packet, CapturePolicy, DEFAULT_MAX_PACKET_BYTES};
use packet_inspector::DisconnectionReason;
use packet_inspector::Packet;
use packet_inspector::Proxy;
use packet_inspector::ProxyLog;
use tracing::Level;

const PACKET_ID_HEX_WIDTH: usize = 2;

#[derive(Parser, Clone, Debug)]
#[clap(author, version, about)]
struct CliArgs {
    /// The socket address to listen for connections on. This is the address clients should connect to.
    listener_addr: SocketAddr,
    /// The socket address the proxy will connect to. This is the address of the server.
    server_addr: SocketAddr,
    /// Maximum packet payload bytes to include when payload previews are enabled.
    #[arg(long = "max-packet-bytes", default_value_t = DEFAULT_MAX_PACKET_BYTES)]
    max_packet_bytes: usize,
    /// Include bounded hexadecimal payload previews. Payloads are redacted by default.
    #[arg(long = "include-payload-preview")]
    include_payload_preview: bool,
}

pub(crate) async fn run() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .init();

    let args = CliArgs::parse();
    let capture_policy = CapturePolicy::new(args.max_packet_bytes, !args.include_payload_preview)?;

    let proxy = Proxy::start(args.listener_addr, args.server_addr).await?;
    let receiver = proxy.subscribe().await;
    let Proxy {
        main_task,
        message_tx: _message_tx,
        logs_rx,
        packet_registry: _packet_registry,
    } = proxy;

    tokio::spawn(async move {
        main_task.await??;
        Ok::<(), anyhow::Error>(())
    });

    tokio::spawn(async move {
        while let Ok(packet) = receiver.recv_async().await {
            log(&packet, &capture_policy);
        }
    });

    tokio::spawn(async move {
        loop {
            let next = logs_rx.recv_async().await?;
            match next {
                ProxyLog::ClientConnected(addr) => {
                    tracing::trace!("Accepted a new client {addr}.");
                }
                ProxyLog::ClientDisconnected(addr, DisconnectionReason::Error(err)) => {
                    tracing::trace!("Client {addr} disconnected with error: {err}.");
                }
                ProxyLog::ClientDisconnected(addr, DisconnectionReason::OnlineModeRequired) => {
                    tracing::error!(
                        "Client {addr} was disconnected due to a server encryption request. \
                        The packet inspector does not support online mode."
                    );
                }
            }
        }

        #[allow(unreachable_code)]
        Ok::<(), anyhow::Error>(())
    });

    tokio::signal::ctrl_c().await.unwrap();

    Ok(())
}

fn log(packet: &Packet, capture_policy: &CapturePolicy) {
    let summary = summarize_packet(packet, capture_policy);
    tracing::debug!(
        "{:?} -> [{:?}] 0x{:0>width$X} \"{}\" len={} redacted={} truncated={} payload={}",
        summary.side,
        summary.state,
        summary.id,
        summary.name,
        summary.body_len,
        summary.redacted,
        summary.truncated,
        summary.body_preview,
        width = PACKET_ID_HEX_WIDTH
    );
}
