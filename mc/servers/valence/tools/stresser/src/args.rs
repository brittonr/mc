use std::path::PathBuf;

use clap::Parser;

use crate::config::{
    RawStresserConfig, DEFAULT_NAME_PREFIX, DEFAULT_READ_BUFFER_SIZE_BYTES, DEFAULT_SCENARIO,
    DEFAULT_SPAWN_COOLDOWN_MILLIS,
};

#[derive(Parser)]
#[command(author, version, about)]
pub(crate) struct StresserArgs {
    /// IP socket address of a server, or localhost:PORT.
    #[arg(short = 't', long = "target")]
    pub(crate) target_host: String,

    /// Number of concurrent sessions.
    #[arg(short = 'c', long = "count")]
    pub(crate) sessions_count: usize,

    /// Name prefix of sessions.
    #[arg(default_value = DEFAULT_NAME_PREFIX)]
    #[arg(short = 'n', long = "name")]
    pub(crate) name_prefix: String,

    /// Spawn cooldown of sessions in milliseconds.
    /// The lower the value, the more frequently sessions are spawned.
    #[arg(default_value_t = DEFAULT_SPAWN_COOLDOWN_MILLIS)]
    #[arg(long = "cooldown")]
    pub(crate) spawn_cooldown: u64,

    /// Read buffer size in bytes.
    #[arg(default_value_t = DEFAULT_READ_BUFFER_SIZE_BYTES)]
    #[arg(long = "read-buffer")]
    pub(crate) read_buffer_size: usize,

    /// Optional per-session timeout in milliseconds.
    #[arg(long = "session-timeout-ms")]
    pub(crate) session_timeout_millis: Option<u64>,

    /// Validate configuration, emit a report, and do not open network connections.
    #[arg(long = "dry-run")]
    pub(crate) dry_run: bool,

    /// Bound the total number of session starts before exiting.
    #[arg(long = "max-starts")]
    pub(crate) max_starts: Option<usize>,

    /// Allow non-loopback targets only when paired with an authorization note.
    #[arg(long = "allow-non-loopback")]
    pub(crate) allow_non_loopback: bool,

    /// Human-readable authorization note required for non-loopback targets.
    #[arg(long = "authorization-note")]
    pub(crate) authorization_note: Option<String>,

    /// Write the structured load report to this path instead of stdout.
    #[arg(long = "report")]
    pub(crate) report_path: Option<PathBuf>,

    /// Scenario label recorded in structured output.
    #[arg(default_value = DEFAULT_SCENARIO)]
    #[arg(long = "scenario")]
    pub(crate) scenario: String,
}

impl From<StresserArgs> for RawStresserConfig {
    fn from(args: StresserArgs) -> Self {
        Self {
            target_host: args.target_host,
            sessions_count: args.sessions_count,
            name_prefix: args.name_prefix,
            spawn_cooldown_millis: args.spawn_cooldown,
            read_buffer_size: args.read_buffer_size,
            session_timeout_millis: args.session_timeout_millis,
            dry_run: args.dry_run,
            max_starts: args.max_starts,
            allow_non_loopback: args.allow_non_loopback,
            authorization_note: args.authorization_note,
            report_path: args.report_path,
            scenario: args.scenario,
        }
    }
}
