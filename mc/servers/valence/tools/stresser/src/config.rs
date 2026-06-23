use std::fmt;
use std::net::{Ipv4Addr, SocketAddr};
use std::num::{NonZeroU64, NonZeroUsize};
use std::path::PathBuf;
use std::time::Duration;

pub(crate) const DEFAULT_NAME_PREFIX: &str = "Stresser";
pub(crate) const DEFAULT_SCENARIO: &str = "loopback-smoke";
pub(crate) const DEFAULT_SPAWN_COOLDOWN_MILLIS: u64 = 10;
pub(crate) const DEFAULT_READ_BUFFER_SIZE_BYTES: usize = 4096;
const LOOPBACK_HOSTNAME: &str = "localhost";
const TARGET_SEPARATOR: char = ':';
const CONFIGURED_TARGET_PORT_ZERO: u16 = 0;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct RawStresserConfig {
    pub(crate) target_host: String,
    pub(crate) sessions_count: usize,
    pub(crate) name_prefix: String,
    pub(crate) spawn_cooldown_millis: u64,
    pub(crate) read_buffer_size: usize,
    pub(crate) session_timeout_millis: Option<u64>,
    pub(crate) dry_run: bool,
    pub(crate) max_starts: Option<usize>,
    pub(crate) allow_non_loopback: bool,
    pub(crate) authorization_note: Option<String>,
    pub(crate) report_path: Option<PathBuf>,
    pub(crate) scenario: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct StresserConfig {
    pub(crate) target_addr: SocketAddr,
    pub(crate) sessions_count: NonZeroUsize,
    pub(crate) name_prefix: String,
    pub(crate) spawn_cooldown: Duration,
    pub(crate) read_buffer_size: NonZeroUsize,
    pub(crate) session_timeout: Option<Duration>,
    pub(crate) dry_run: bool,
    pub(crate) max_starts: Option<NonZeroUsize>,
    pub(crate) safety: TargetSafety,
    pub(crate) report_path: Option<PathBuf>,
    pub(crate) scenario: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum TargetSafety {
    Loopback,
    ExplicitAuthorization { note: String },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum ConfigError {
    EmptyScenario,
    InvalidTarget { target: String },
    PortZero { target: String },
    SessionsCountZero,
    ReadBufferSizeZero,
    MaxStartsZero,
    SessionTimeoutZero,
    UnsafeTarget { target: String },
    EmptyAuthorizationNote { target: String },
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyScenario => write!(f, "scenario must not be empty"),
            Self::InvalidTarget { target } => write!(
                f,
                "target {target:?} must be an IP socket address or localhost:PORT"
            ),
            Self::PortZero { target } => write!(f, "target {target:?} must not use port zero"),
            Self::SessionsCountZero => write!(f, "session count must be greater than zero"),
            Self::ReadBufferSizeZero => write!(f, "read buffer size must be greater than zero"),
            Self::MaxStartsZero => write!(f, "max starts must be greater than zero when set"),
            Self::SessionTimeoutZero => write!(f, "session timeout must be greater than zero when set"),
            Self::UnsafeTarget { target } => write!(
                f,
                "target {target:?} is outside loopback; pass --allow-non-loopback with --authorization-note before any connection is attempted"
            ),
            Self::EmptyAuthorizationNote { target } => write!(
                f,
                "target {target:?} is non-loopback and requires a non-empty authorization note"
            ),
        }
    }
}

impl std::error::Error for ConfigError {}

pub(crate) fn validate_config(raw: RawStresserConfig) -> Result<StresserConfig, ConfigError> {
    let scenario = raw.scenario.trim().to_owned();
    if scenario.is_empty() {
        return Err(ConfigError::EmptyScenario);
    }

    let target_addr = parse_target_addr(&raw.target_host)?;
    if target_addr.port() == CONFIGURED_TARGET_PORT_ZERO {
        return Err(ConfigError::PortZero {
            target: raw.target_host,
        });
    }

    let safety = classify_target_safety(
        target_addr,
        raw.allow_non_loopback,
        raw.authorization_note.as_deref(),
    )?;

    let sessions_count =
        NonZeroUsize::new(raw.sessions_count).ok_or(ConfigError::SessionsCountZero)?;
    let read_buffer_size =
        NonZeroUsize::new(raw.read_buffer_size).ok_or(ConfigError::ReadBufferSizeZero)?;
    let max_starts = match raw.max_starts {
        Some(value) => Some(NonZeroUsize::new(value).ok_or(ConfigError::MaxStartsZero)?),
        None => None,
    };
    let session_timeout = match raw.session_timeout_millis {
        Some(value) => Some(Duration::from_millis(
            NonZeroU64::new(value)
                .ok_or(ConfigError::SessionTimeoutZero)?
                .get(),
        )),
        None => None,
    };

    Ok(StresserConfig {
        target_addr,
        sessions_count,
        name_prefix: raw.name_prefix,
        spawn_cooldown: Duration::from_millis(raw.spawn_cooldown_millis),
        read_buffer_size,
        session_timeout,
        dry_run: raw.dry_run,
        max_starts,
        safety,
        report_path: raw.report_path,
        scenario,
    })
}

fn parse_target_addr(target: &str) -> Result<SocketAddr, ConfigError> {
    if let Ok(addr) = target.parse::<SocketAddr>() {
        return Ok(addr);
    }

    let Some((host, port_text)) = target.rsplit_once(TARGET_SEPARATOR) else {
        return Err(ConfigError::InvalidTarget {
            target: target.to_owned(),
        });
    };

    if host != LOOPBACK_HOSTNAME {
        return Err(ConfigError::InvalidTarget {
            target: target.to_owned(),
        });
    }

    let port = port_text
        .parse::<u16>()
        .map_err(|_| ConfigError::InvalidTarget {
            target: target.to_owned(),
        })?;

    Ok(SocketAddr::from((Ipv4Addr::LOCALHOST, port)))
}

fn classify_target_safety(
    target_addr: SocketAddr,
    allow_non_loopback: bool,
    authorization_note: Option<&str>,
) -> Result<TargetSafety, ConfigError> {
    if target_addr.ip().is_loopback() {
        return Ok(TargetSafety::Loopback);
    }

    if !allow_non_loopback {
        return Err(ConfigError::UnsafeTarget {
            target: target_addr.to_string(),
        });
    }

    let Some(note) = authorization_note
        .map(str::trim)
        .filter(|note| !note.is_empty())
    else {
        return Err(ConfigError::EmptyAuthorizationNote {
            target: target_addr.to_string(),
        });
    };

    Ok(TargetSafety::ExplicitAuthorization {
        note: note.to_owned(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const LOOPBACK_TARGET: &str = "127.0.0.1:25565";
    const LOCALHOST_TARGET: &str = "localhost:25565";
    const NON_LOOPBACK_TARGET: &str = "192.0.2.10:25565";
    const AUTHORIZATION_NOTE: &str = "owned lab host";
    const NAME_PREFIX: &str = "Load";
    const SCENARIO: &str = "loopback-smoke";
    const SESSION_COUNT: usize = 2;
    const READ_BUFFER_SIZE_BYTES: usize = 4096;
    const SPAWN_COOLDOWN_MILLIS: u64 = 10;
    const MAX_STARTS: usize = 3;
    const SESSION_TIMEOUT_MILLIS: u64 = 250;

    #[test]
    fn loopback_config_is_valid() {
        let config = validate_config(raw_config(LOOPBACK_TARGET)).unwrap();

        assert_eq!(config.target_addr.to_string(), LOOPBACK_TARGET);
        assert_eq!(config.sessions_count.get(), SESSION_COUNT);
        assert_eq!(config.read_buffer_size.get(), READ_BUFFER_SIZE_BYTES);
        assert_eq!(config.max_starts.unwrap().get(), MAX_STARTS);
        assert_eq!(
            config.session_timeout.unwrap(),
            Duration::from_millis(SESSION_TIMEOUT_MILLIS)
        );
        assert_eq!(config.safety, TargetSafety::Loopback);
    }

    #[test]
    fn localhost_target_maps_to_loopback_without_dns() {
        let config = validate_config(raw_config(LOCALHOST_TARGET)).unwrap();

        assert_eq!(config.target_addr.ip(), Ipv4Addr::LOCALHOST);
        assert_eq!(config.safety, TargetSafety::Loopback);
    }

    #[test]
    fn authorized_non_loopback_config_is_valid() {
        let mut raw = raw_config(NON_LOOPBACK_TARGET);
        raw.allow_non_loopback = true;
        raw.authorization_note = Some(AUTHORIZATION_NOTE.to_owned());

        let config = validate_config(raw).unwrap();

        assert_eq!(
            config.safety,
            TargetSafety::ExplicitAuthorization {
                note: AUTHORIZATION_NOTE.to_owned()
            }
        );
    }

    #[test]
    fn unsafe_non_loopback_target_is_rejected_before_connect() {
        let error = validate_config(raw_config(NON_LOOPBACK_TARGET)).unwrap_err();

        assert_eq!(
            error,
            ConfigError::UnsafeTarget {
                target: NON_LOOPBACK_TARGET.to_owned()
            }
        );
    }

    #[test]
    fn non_loopback_target_without_authorization_note_is_rejected() {
        let mut raw = raw_config(NON_LOOPBACK_TARGET);
        raw.allow_non_loopback = true;

        let error = validate_config(raw).unwrap_err();

        assert_eq!(
            error,
            ConfigError::EmptyAuthorizationNote {
                target: NON_LOOPBACK_TARGET.to_owned()
            }
        );
    }

    #[test]
    fn dns_target_is_rejected_to_avoid_implicit_resolution() {
        let error = validate_config(raw_config("example.com:25565")).unwrap_err();

        assert_eq!(
            error,
            ConfigError::InvalidTarget {
                target: "example.com:25565".to_owned()
            }
        );
    }

    #[test]
    fn zero_session_count_is_rejected() {
        let mut raw = raw_config(LOOPBACK_TARGET);
        raw.sessions_count = 0;

        assert_eq!(
            validate_config(raw).unwrap_err(),
            ConfigError::SessionsCountZero
        );
    }

    #[test]
    fn zero_read_buffer_is_rejected() {
        let mut raw = raw_config(LOOPBACK_TARGET);
        raw.read_buffer_size = 0;

        assert_eq!(
            validate_config(raw).unwrap_err(),
            ConfigError::ReadBufferSizeZero
        );
    }

    #[test]
    fn zero_session_timeout_is_rejected() {
        let mut raw = raw_config(LOOPBACK_TARGET);
        raw.session_timeout_millis = Some(0);

        assert_eq!(
            validate_config(raw).unwrap_err(),
            ConfigError::SessionTimeoutZero
        );
    }

    fn raw_config(target_host: &str) -> RawStresserConfig {
        RawStresserConfig {
            target_host: target_host.to_owned(),
            sessions_count: SESSION_COUNT,
            name_prefix: NAME_PREFIX.to_owned(),
            spawn_cooldown_millis: SPAWN_COOLDOWN_MILLIS,
            read_buffer_size: READ_BUFFER_SIZE_BYTES,
            session_timeout_millis: Some(SESSION_TIMEOUT_MILLIS),
            dry_run: true,
            max_starts: Some(MAX_STARTS),
            allow_non_loopback: false,
            authorization_note: None,
            report_path: None,
            scenario: SCENARIO.to_owned(),
        }
    }
}
