use serde::Serialize;

use crate::config::{StresserConfig, TargetSafety};

const REPORT_SCHEMA: &str = "valence.load_tool.report.v1";
const TOOL_NAME: &str = "valence-stresser";
pub(crate) const SUCCESS_EXIT_STATUS: u8 = 0;
pub(crate) const CONFIG_ERROR_EXIT_STATUS: u8 = 2;
pub(crate) const LOAD_RUN_FAILURE_EXIT_STATUS: u8 = 70;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum LoadReportKind {
    DryRun,
    Failure,
    Completed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum LoadPhase {
    ConfigValidation,
    Connect,
    Handshake,
    Login,
    Play,
    Timeout,
    DryRun,
    Completed,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub(crate) struct LoadToolReport {
    schema: &'static str,
    tool: &'static str,
    kind: LoadReportKind,
    phase: LoadPhase,
    target: String,
    scenario: String,
    session_count: usize,
    max_starts: Option<usize>,
    session_timeout_millis: Option<u64>,
    target_safety: String,
    message: String,
    exit_status: u8,
    load_evidence_pass: bool,
    compatibility_evidence_pass: bool,
}

impl LoadToolReport {
    pub(crate) fn dry_run(config: &StresserConfig) -> Self {
        Self::from_config(
            config,
            LoadReportKind::DryRun,
            LoadPhase::DryRun,
            "dry run validated config and did not open network connections".to_owned(),
            SUCCESS_EXIT_STATUS,
            true,
        )
    }

    pub(crate) fn completed(config: &StresserConfig) -> Self {
        Self::from_config(
            config,
            LoadReportKind::Completed,
            LoadPhase::Completed,
            "bounded load run completed without a session failure".to_owned(),
            SUCCESS_EXIT_STATUS,
            true,
        )
    }

    pub(crate) fn session_failure(
        config: &StresserConfig,
        phase: LoadPhase,
        message: impl Into<String>,
    ) -> Self {
        Self::from_config(
            config,
            LoadReportKind::Failure,
            phase,
            message.into(),
            LOAD_RUN_FAILURE_EXIT_STATUS,
            false,
        )
    }

    pub(crate) fn config_failure(
        target: String,
        scenario: String,
        message: impl Into<String>,
    ) -> Self {
        Self {
            schema: REPORT_SCHEMA,
            tool: TOOL_NAME,
            kind: LoadReportKind::Failure,
            phase: LoadPhase::ConfigValidation,
            target,
            scenario,
            session_count: 0,
            max_starts: None,
            session_timeout_millis: None,
            target_safety: "unvalidated".to_owned(),
            message: message.into(),
            exit_status: CONFIG_ERROR_EXIT_STATUS,
            load_evidence_pass: false,
            compatibility_evidence_pass: false,
        }
    }

    pub(crate) fn exit_status(&self) -> u8 {
        self.exit_status
    }

    fn from_config(
        config: &StresserConfig,
        kind: LoadReportKind,
        phase: LoadPhase,
        message: String,
        exit_status: u8,
        load_evidence_pass: bool,
    ) -> Self {
        Self {
            schema: REPORT_SCHEMA,
            tool: TOOL_NAME,
            kind,
            phase,
            target: config.target_addr.to_string(),
            scenario: config.scenario.clone(),
            session_count: config.sessions_count.get(),
            max_starts: config.max_starts.map(|value| value.get()),
            session_timeout_millis: config.session_timeout.map(duration_millis_u64),
            target_safety: target_safety_label(&config.safety).to_owned(),
            message,
            exit_status,
            load_evidence_pass,
            compatibility_evidence_pass: false,
        }
    }
}

fn duration_millis_u64(timeout: std::time::Duration) -> u64 {
    u64::try_from(timeout.as_millis()).unwrap_or(u64::MAX)
}

fn target_safety_label(safety: &TargetSafety) -> &'static str {
    match safety {
        TargetSafety::Loopback => "loopback",
        TargetSafety::ExplicitAuthorization { .. } => "explicit_authorization",
    }
}

#[cfg(test)]
mod tests {
    use crate::config::{validate_config, RawStresserConfig};

    use super::*;

    const LOOPBACK_TARGET: &str = "127.0.0.1:25565";
    const SCENARIO: &str = "loopback-smoke";
    const NAME_PREFIX: &str = "Load";
    const SESSION_COUNT: usize = 1;
    const SPAWN_COOLDOWN_MILLIS: u64 = 10;
    const READ_BUFFER_SIZE_BYTES: usize = 4096;

    #[test]
    fn dry_run_report_is_not_compatibility_evidence() {
        let config = validate_config(raw_config()).unwrap();

        let report = LoadToolReport::dry_run(&config);

        assert_eq!(report.kind, LoadReportKind::DryRun);
        assert_eq!(report.phase, LoadPhase::DryRun);
        assert!(report.load_evidence_pass);
        assert!(!report.compatibility_evidence_pass);
        assert_eq!(report.exit_status(), SUCCESS_EXIT_STATUS);
    }

    #[test]
    fn config_failure_report_is_structured() {
        let message = "target rejected";

        let report = LoadToolReport::config_failure(
            LOOPBACK_TARGET.to_owned(),
            SCENARIO.to_owned(),
            message,
        );

        assert_eq!(report.kind, LoadReportKind::Failure);
        assert_eq!(report.phase, LoadPhase::ConfigValidation);
        assert_eq!(report.message, message);
        assert_eq!(report.exit_status(), CONFIG_ERROR_EXIT_STATUS);
        assert!(!report.load_evidence_pass);
        assert!(!report.compatibility_evidence_pass);
    }

    fn raw_config() -> RawStresserConfig {
        RawStresserConfig {
            target_host: LOOPBACK_TARGET.to_owned(),
            sessions_count: SESSION_COUNT,
            name_prefix: NAME_PREFIX.to_owned(),
            spawn_cooldown_millis: SPAWN_COOLDOWN_MILLIS,
            read_buffer_size: READ_BUFFER_SIZE_BYTES,
            session_timeout_millis: None,
            dry_run: true,
            max_starts: None,
            allow_non_loopback: false,
            authorization_note: None,
            report_path: None,
            scenario: SCENARIO.to_owned(),
        }
    }
}
