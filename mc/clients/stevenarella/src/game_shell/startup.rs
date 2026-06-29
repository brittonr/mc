use std::path::PathBuf;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Stevenarella")]
pub struct Opt {
    /// Server to connect to
    #[structopt(short = "s", long = "server")]
    pub server: Option<String>,

    /// Username for offline servers
    #[structopt(short = "u", long = "username")]
    pub username: Option<String>,

    /// Log decoded packets received from network
    #[structopt(short = "n", long = "network-debug")]
    pub network_debug: bool,

    /// Parse a network packet from a file
    #[structopt(short = "N", long = "network-parse-packet")]
    pub network_parse_packet: Option<String>,

    /// Protocol version to use in the autodetection ping
    #[structopt(short = "p", long = "default-protocol-version")]
    pub default_protocol_version: Option<String>,

    /// Enable MCP over stdio. Stdout is reserved for JSON-RPC while active.
    #[cfg(not(target_arch = "wasm32"))]
    #[structopt(long = "mcp-stdio")]
    pub mcp_stdio: bool,

    /// Enable MCP over a TCP socket, e.g. 127.0.0.1:4700.
    #[cfg(not(target_arch = "wasm32"))]
    #[structopt(long = "mcp-listen")]
    pub mcp_listen: Option<String>,

    /// Environment variable containing the MCP token for non-loopback TCP binds.
    #[cfg(not(target_arch = "wasm32"))]
    #[structopt(long = "mcp-token-env")]
    pub mcp_token_env: Option<String>,

    /// Directory for durable frame capture artifacts.
    #[cfg(not(target_arch = "wasm32"))]
    #[structopt(long = "capture-dir", parse(from_os_str))]
    pub capture_dir: Option<PathBuf>,

    /// Startup recording frame rate. Requires --capture-dir and duration or frame count.
    #[cfg(not(target_arch = "wasm32"))]
    #[structopt(long = "capture-record-fps")]
    pub capture_record_fps: Option<u16>,

    /// Startup recording frame count bound.
    #[cfg(not(target_arch = "wasm32"))]
    #[structopt(long = "capture-record-frames")]
    pub capture_record_frames: Option<u32>,

    /// Startup recording duration bound in milliseconds.
    #[cfg(not(target_arch = "wasm32"))]
    #[structopt(long = "capture-record-duration-ms")]
    pub capture_record_duration_millis: Option<u64>,
}
