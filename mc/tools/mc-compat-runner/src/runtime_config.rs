use std::collections::BTreeMap;

const SUPPORTED_SCHEMA_VERSION: u32 = 1;
const SUPPORTED_SANDBOX_PROFILE: &str = "mc-compat/pure-v1";
const MIN_PORT: u32 = 1;
const MAX_PORT: u32 = u16::MAX as u32;
const MIN_TIMEOUT_SECS: u32 = 1;
const MIN_DAMAGE: f64 = 0.0;
const MAX_DAMAGE: f64 = 100.0;
const MIN_MULTIPLIER: f64 = 0.0;
const MAX_MULTIPLIER: f64 = 100.0;
const ZERO_DAMAGE: f64 = 0.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum MutabilityClass {
    Hot,
    NextRun,
    RestartOnly,
    FixedProtocolFact,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum SteelValue {
    String(String),
    U32(u32),
    F64(f64),
    StringList(Vec<String>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct SteelSource {
    pub(crate) path: String,
    pub(crate) module_blake3: String,
    pub(crate) sandbox_profile: String,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct RuntimeConfigSnapshot {
    pub(crate) schema_version: u32,
    pub(crate) source: SteelSource,
    pub(crate) server_backend: String,
    pub(crate) server_protocol: u32,
    pub(crate) server_port: u16,
    pub(crate) client_timeout_secs: u32,
    pub(crate) client_success_patterns: Vec<String>,
    pub(crate) scenario: String,
    pub(crate) arrow_damage: ArrowDamagePolicy,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ArrowDamagePolicy {
    pub(crate) base_damage: f64,
    pub(crate) velocity_multiplier: f64,
    pub(crate) max_damage: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ProjectileDamageContext {
    pub(crate) projectile_velocity: f64,
    pub(crate) pull_strength: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ArrowDamageDecision {
    pub(crate) damage: f64,
    pub(crate) policy: &'static str,
    pub(crate) clamped: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ConfigDiagnostic {
    pub(crate) path: &'static str,
    pub(crate) message: String,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct FieldDiff {
    pub(crate) path: &'static str,
    pub(crate) before: String,
    pub(crate) after: String,
    pub(crate) mutability: MutabilityClass,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ApplyPlan {
    pub(crate) hot: Vec<FieldDiff>,
    pub(crate) next_run: Vec<FieldDiff>,
    pub(crate) restart_only: Vec<FieldDiff>,
    pub(crate) rejected: Vec<ConfigDiagnostic>,
}

pub(crate) fn normalize_steel_exports(
    source: SteelSource,
    exports: &BTreeMap<String, SteelValue>,
) -> Result<RuntimeConfigSnapshot, Vec<ConfigDiagnostic>> {
    let mut diagnostics = Vec::new();
    if source.sandbox_profile != SUPPORTED_SANDBOX_PROFILE {
        diagnostics.push(ConfigDiagnostic {
            path: "runtime.steel.sandbox_profile",
            message: format!(
                "unsupported sandbox profile {}, expected {}",
                source.sandbox_profile, SUPPORTED_SANDBOX_PROFILE
            ),
        });
    }

    let schema_version = required_u32(
        exports,
        "config-version",
        "runtime.config_version",
        &mut diagnostics,
    );
    if let Some(version) = schema_version {
        if version != SUPPORTED_SCHEMA_VERSION {
            diagnostics.push(ConfigDiagnostic {
                path: "runtime.config_version",
                message: format!(
                    "unsupported schema version {version}, expected {SUPPORTED_SCHEMA_VERSION}"
                ),
            });
        }
    }

    let server_backend = required_string(
        exports,
        "server-backend",
        "server.backend",
        &mut diagnostics,
    );
    if let Some(backend) = &server_backend {
        if backend != "valence" && backend != "paper" {
            diagnostics.push(ConfigDiagnostic {
                path: "server.backend",
                message: format!("unsupported server backend {backend}"),
            });
        }
    }

    let server_protocol = required_u32(
        exports,
        "server-protocol",
        "server.protocol",
        &mut diagnostics,
    );
    let server_port = required_u32(exports, "server-port", "server.port", &mut diagnostics)
        .and_then(|value| validate_u16_port(value, &mut diagnostics));
    let client_timeout_secs = required_u32(
        exports,
        "client-timeout-secs",
        "client.timeout_secs",
        &mut diagnostics,
    )
    .and_then(|value| {
        validate_min_u32(
            "client.timeout_secs",
            value,
            MIN_TIMEOUT_SECS,
            &mut diagnostics,
        )
    });
    let client_success_patterns = required_string_list(
        exports,
        "client-success-patterns",
        "client.success_patterns",
        &mut diagnostics,
    );
    if let Some(patterns) = &client_success_patterns {
        if patterns.is_empty() || patterns.iter().any(|value| value.is_empty()) {
            diagnostics.push(ConfigDiagnostic {
                path: "client.success_patterns",
                message: "success patterns must be nonempty strings".to_string(),
            });
        }
    }
    let scenario = required_string(exports, "scenario", "scenario.name", &mut diagnostics);
    let arrow_damage = normalize_arrow_damage(exports, &mut diagnostics);

    if diagnostics.is_empty() {
        Ok(RuntimeConfigSnapshot {
            schema_version: schema_version.expect("diagnostics checked"),
            source,
            server_backend: server_backend.expect("diagnostics checked"),
            server_protocol: server_protocol.expect("diagnostics checked"),
            server_port: server_port.expect("diagnostics checked"),
            client_timeout_secs: client_timeout_secs.expect("diagnostics checked"),
            client_success_patterns: client_success_patterns.expect("diagnostics checked"),
            scenario: scenario.expect("diagnostics checked"),
            arrow_damage: arrow_damage.expect("diagnostics checked"),
        })
    } else {
        Err(diagnostics)
    }
}

pub(crate) fn evaluate_arrow_damage(
    policy: &ArrowDamagePolicy,
    context: &ProjectileDamageContext,
) -> ArrowDamageDecision {
    let scaled_velocity =
        context.projectile_velocity.max(ZERO_DAMAGE) * context.pull_strength.max(ZERO_DAMAGE);
    let raw_damage = policy.base_damage + scaled_velocity * policy.velocity_multiplier;
    let bounded_damage = raw_damage.clamp(MIN_DAMAGE, policy.max_damage.min(MAX_DAMAGE));
    ArrowDamageDecision {
        damage: bounded_damage,
        policy: "damage-linear",
        clamped: (bounded_damage - raw_damage).abs() > f64::EPSILON,
    }
}

pub(crate) fn diff_snapshots(
    before: &RuntimeConfigSnapshot,
    after: &RuntimeConfigSnapshot,
) -> Vec<FieldDiff> {
    let mut diffs = Vec::new();
    push_diff(
        &mut diffs,
        "server.backend",
        &before.server_backend,
        &after.server_backend,
        MutabilityClass::NextRun,
    );
    push_diff(
        &mut diffs,
        "server.protocol",
        &before.server_protocol.to_string(),
        &after.server_protocol.to_string(),
        MutabilityClass::NextRun,
    );
    push_diff(
        &mut diffs,
        "server.port",
        &before.server_port.to_string(),
        &after.server_port.to_string(),
        MutabilityClass::RestartOnly,
    );
    push_diff(
        &mut diffs,
        "client.timeout_secs",
        &before.client_timeout_secs.to_string(),
        &after.client_timeout_secs.to_string(),
        MutabilityClass::Hot,
    );
    push_diff(
        &mut diffs,
        "client.success_patterns",
        &format!("{:?}", before.client_success_patterns),
        &format!("{:?}", after.client_success_patterns),
        MutabilityClass::Hot,
    );
    push_diff(
        &mut diffs,
        "scenario.name",
        &before.scenario,
        &after.scenario,
        MutabilityClass::NextRun,
    );
    push_diff(
        &mut diffs,
        "combat.arrow.base_damage",
        &before.arrow_damage.base_damage.to_string(),
        &after.arrow_damage.base_damage.to_string(),
        MutabilityClass::Hot,
    );
    push_diff(
        &mut diffs,
        "combat.arrow.velocity_multiplier",
        &before.arrow_damage.velocity_multiplier.to_string(),
        &after.arrow_damage.velocity_multiplier.to_string(),
        MutabilityClass::Hot,
    );
    push_diff(
        &mut diffs,
        "combat.arrow.max_damage",
        &before.arrow_damage.max_damage.to_string(),
        &after.arrow_damage.max_damage.to_string(),
        MutabilityClass::Hot,
    );
    diffs
}

pub(crate) fn build_apply_plan(diffs: Vec<FieldDiff>, allow_restart_only: bool) -> ApplyPlan {
    let mut plan = ApplyPlan {
        hot: Vec::new(),
        next_run: Vec::new(),
        restart_only: Vec::new(),
        rejected: Vec::new(),
    };
    for diff in diffs {
        match diff.mutability {
            MutabilityClass::Hot => plan.hot.push(diff),
            MutabilityClass::NextRun => plan.next_run.push(diff),
            MutabilityClass::RestartOnly if allow_restart_only => plan.restart_only.push(diff),
            MutabilityClass::RestartOnly => plan.rejected.push(ConfigDiagnostic {
                path: diff.path,
                message: "restart-only field cannot be hot-applied".to_string(),
            }),
            MutabilityClass::FixedProtocolFact => plan.rejected.push(ConfigDiagnostic {
                path: diff.path,
                message: "fixed protocol fact cannot be changed by config".to_string(),
            }),
        }
    }
    plan
}

fn normalize_arrow_damage(
    exports: &BTreeMap<String, SteelValue>,
    diagnostics: &mut Vec<ConfigDiagnostic>,
) -> Option<ArrowDamagePolicy> {
    let base_damage = required_f64(
        exports,
        "arrow-base-damage",
        "combat.arrow.base_damage",
        diagnostics,
    )
    .and_then(|value| {
        validate_range(
            "combat.arrow.base_damage",
            value,
            MIN_DAMAGE,
            MAX_DAMAGE,
            diagnostics,
        )
    });
    let velocity_multiplier = required_f64(
        exports,
        "arrow-velocity-multiplier",
        "combat.arrow.velocity_multiplier",
        diagnostics,
    )
    .and_then(|value| {
        validate_range(
            "combat.arrow.velocity_multiplier",
            value,
            MIN_MULTIPLIER,
            MAX_MULTIPLIER,
            diagnostics,
        )
    });
    let max_damage = required_f64(
        exports,
        "arrow-max-damage",
        "combat.arrow.max_damage",
        diagnostics,
    )
    .and_then(|value| {
        validate_range(
            "combat.arrow.max_damage",
            value,
            MIN_DAMAGE,
            MAX_DAMAGE,
            diagnostics,
        )
    });
    match (base_damage, velocity_multiplier, max_damage) {
        (Some(base_damage), Some(velocity_multiplier), Some(max_damage)) => {
            Some(ArrowDamagePolicy {
                base_damage,
                velocity_multiplier,
                max_damage,
            })
        }
        _ => None,
    }
}

fn required_string(
    exports: &BTreeMap<String, SteelValue>,
    export_name: &'static str,
    path: &'static str,
    diagnostics: &mut Vec<ConfigDiagnostic>,
) -> Option<String> {
    match exports.get(export_name) {
        Some(SteelValue::String(value)) => Some(value.clone()),
        Some(other) => {
            diagnostics.push(ConfigDiagnostic {
                path,
                message: format!("expected string from {export_name}, found {other:?}"),
            });
            None
        }
        None => {
            diagnostics.push(ConfigDiagnostic {
                path,
                message: format!("missing Steel export {export_name}"),
            });
            None
        }
    }
}

fn required_string_list(
    exports: &BTreeMap<String, SteelValue>,
    export_name: &'static str,
    path: &'static str,
    diagnostics: &mut Vec<ConfigDiagnostic>,
) -> Option<Vec<String>> {
    match exports.get(export_name) {
        Some(SteelValue::StringList(value)) => Some(value.clone()),
        Some(other) => {
            diagnostics.push(ConfigDiagnostic {
                path,
                message: format!("expected string list from {export_name}, found {other:?}"),
            });
            None
        }
        None => {
            diagnostics.push(ConfigDiagnostic {
                path,
                message: format!("missing Steel export {export_name}"),
            });
            None
        }
    }
}

fn required_u32(
    exports: &BTreeMap<String, SteelValue>,
    export_name: &'static str,
    path: &'static str,
    diagnostics: &mut Vec<ConfigDiagnostic>,
) -> Option<u32> {
    match exports.get(export_name) {
        Some(SteelValue::U32(value)) => Some(*value),
        Some(other) => {
            diagnostics.push(ConfigDiagnostic {
                path,
                message: format!("expected u32 from {export_name}, found {other:?}"),
            });
            None
        }
        None => {
            diagnostics.push(ConfigDiagnostic {
                path,
                message: format!("missing Steel export {export_name}"),
            });
            None
        }
    }
}

fn required_f64(
    exports: &BTreeMap<String, SteelValue>,
    export_name: &'static str,
    path: &'static str,
    diagnostics: &mut Vec<ConfigDiagnostic>,
) -> Option<f64> {
    match exports.get(export_name) {
        Some(SteelValue::F64(value)) => Some(*value),
        Some(other) => {
            diagnostics.push(ConfigDiagnostic {
                path,
                message: format!("expected f64 from {export_name}, found {other:?}"),
            });
            None
        }
        None => {
            diagnostics.push(ConfigDiagnostic {
                path,
                message: format!("missing Steel export {export_name}"),
            });
            None
        }
    }
}

fn validate_u16_port(value: u32, diagnostics: &mut Vec<ConfigDiagnostic>) -> Option<u16> {
    if (MIN_PORT..=MAX_PORT).contains(&value) {
        Some(value as u16)
    } else {
        diagnostics.push(ConfigDiagnostic {
            path: "server.port",
            message: format!("port {value} outside {MIN_PORT}..={MAX_PORT}"),
        });
        None
    }
}

fn validate_min_u32(
    path: &'static str,
    value: u32,
    minimum: u32,
    diagnostics: &mut Vec<ConfigDiagnostic>,
) -> Option<u32> {
    if value >= minimum {
        Some(value)
    } else {
        diagnostics.push(ConfigDiagnostic {
            path,
            message: format!("value {value} below minimum {minimum}"),
        });
        None
    }
}

fn validate_range(
    path: &'static str,
    value: f64,
    minimum: f64,
    maximum: f64,
    diagnostics: &mut Vec<ConfigDiagnostic>,
) -> Option<f64> {
    if value.is_finite() && (minimum..=maximum).contains(&value) {
        Some(value)
    } else {
        diagnostics.push(ConfigDiagnostic {
            path,
            message: format!("value {value} outside {minimum}..={maximum}"),
        });
        None
    }
}

fn push_diff(
    diffs: &mut Vec<FieldDiff>,
    path: &'static str,
    before: &str,
    after: &str,
    mutability: MutabilityClass,
) {
    if before != after {
        diffs.push(FieldDiff {
            path,
            before: before.to_string(),
            after: after.to_string(),
            mutability,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_MODULE_HASH: &str =
        "06373a43c99d0463611c9ef55dc7e72a624c1335a71078eea6d5d4b5d6998b16";
    const TEST_MODULE_PATH: &str = "config/mc-compat/steel/default.scm";
    const TEST_SERVER_PROTOCOL: u32 = 763;
    const TEST_SERVER_PORT: u32 = 25565;
    const TEST_CLIENT_TIMEOUT_SECS: u32 = 120;
    const TEST_ARROW_BASE_DAMAGE: f64 = 3.0;
    const TEST_ARROW_VELOCITY_MULTIPLIER: f64 = 1.0;
    const TEST_ARROW_MAX_DAMAGE: f64 = 10.0;
    const TEST_PROJECTILE_VELOCITY: f64 = 3.0;
    const TEST_PULL_STRENGTH: f64 = 2.0;
    const TEST_CLAMPING_PROJECTILE_VELOCITY: f64 = 200.0;

    fn source() -> SteelSource {
        SteelSource {
            path: TEST_MODULE_PATH.to_string(),
            module_blake3: TEST_MODULE_HASH.to_string(),
            sandbox_profile: SUPPORTED_SANDBOX_PROFILE.to_string(),
        }
    }

    fn valid_exports() -> BTreeMap<String, SteelValue> {
        BTreeMap::from([
            (
                "config-version".to_string(),
                SteelValue::U32(SUPPORTED_SCHEMA_VERSION),
            ),
            (
                "server-backend".to_string(),
                SteelValue::String("valence".to_string()),
            ),
            (
                "server-protocol".to_string(),
                SteelValue::U32(TEST_SERVER_PROTOCOL),
            ),
            ("server-port".to_string(), SteelValue::U32(TEST_SERVER_PORT)),
            (
                "client-timeout-secs".to_string(),
                SteelValue::U32(TEST_CLIENT_TIMEOUT_SECS),
            ),
            (
                "client-success-patterns".to_string(),
                SteelValue::StringList(vec!["Detected server protocol version".to_string()]),
            ),
            (
                "scenario".to_string(),
                SteelValue::String("projectile-damage-attribution".to_string()),
            ),
            (
                "arrow-base-damage".to_string(),
                SteelValue::F64(TEST_ARROW_BASE_DAMAGE),
            ),
            (
                "arrow-velocity-multiplier".to_string(),
                SteelValue::F64(TEST_ARROW_VELOCITY_MULTIPLIER),
            ),
            (
                "arrow-max-damage".to_string(),
                SteelValue::F64(TEST_ARROW_MAX_DAMAGE),
            ),
        ])
    }

    #[test]
    fn normalizes_valid_steel_exports() {
        let snapshot = normalize_steel_exports(source(), &valid_exports()).expect("valid exports");

        assert_eq!(snapshot.schema_version, SUPPORTED_SCHEMA_VERSION);
        assert_eq!(snapshot.server_backend, "valence");
        assert_eq!(snapshot.server_protocol, TEST_SERVER_PROTOCOL);
        assert_eq!(snapshot.server_port, TEST_SERVER_PORT as u16);
        assert_eq!(snapshot.client_timeout_secs, TEST_CLIENT_TIMEOUT_SECS);
        assert_eq!(snapshot.arrow_damage.base_damage, TEST_ARROW_BASE_DAMAGE);
    }

    #[test]
    fn rejects_missing_or_wrong_type_exports() {
        let mut missing = valid_exports();
        missing.remove("server-backend");
        let diagnostics = normalize_steel_exports(source(), &missing).unwrap_err();
        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.path == "server.backend"));

        let mut wrong_type = valid_exports();
        wrong_type.insert(
            "server-port".to_string(),
            SteelValue::String("25565".to_string()),
        );
        let diagnostics = normalize_steel_exports(source(), &wrong_type).unwrap_err();
        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.path == "server.port"));
    }

    #[test]
    fn rejects_range_and_sandbox_violations() {
        let mut bad_range = valid_exports();
        bad_range.insert(
            "arrow-max-damage".to_string(),
            SteelValue::F64(MAX_DAMAGE + TEST_ARROW_MAX_DAMAGE),
        );
        let diagnostics = normalize_steel_exports(source(), &bad_range).unwrap_err();
        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.path == "combat.arrow.max_damage"));

        let mut bad_source = source();
        bad_source.sandbox_profile = "ambient-io".to_string();
        let diagnostics = normalize_steel_exports(bad_source, &valid_exports()).unwrap_err();
        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.path == "runtime.steel.sandbox_profile"));
    }

    #[test]
    fn arrow_damage_policy_is_bounded_and_reports_clamping() {
        let snapshot = normalize_steel_exports(source(), &valid_exports()).expect("valid exports");
        let decision = evaluate_arrow_damage(
            &snapshot.arrow_damage,
            &ProjectileDamageContext {
                projectile_velocity: TEST_PROJECTILE_VELOCITY,
                pull_strength: TEST_PULL_STRENGTH,
            },
        );
        assert_eq!(decision.policy, "damage-linear");
        assert_eq!(
            decision.damage,
            TEST_ARROW_BASE_DAMAGE + TEST_PROJECTILE_VELOCITY * TEST_PULL_STRENGTH
        );
        assert!(!decision.clamped);

        let clamped = evaluate_arrow_damage(
            &snapshot.arrow_damage,
            &ProjectileDamageContext {
                projectile_velocity: TEST_CLAMPING_PROJECTILE_VELOCITY,
                pull_strength: TEST_PULL_STRENGTH,
            },
        );
        assert_eq!(clamped.damage, TEST_ARROW_MAX_DAMAGE);
        assert!(clamped.clamped);
    }

    #[test]
    fn diff_and_apply_plan_separate_hot_next_run_and_restart_only() {
        let before = normalize_steel_exports(source(), &valid_exports()).expect("valid exports");
        let mut after_exports = valid_exports();
        after_exports.insert(
            "client-timeout-secs".to_string(),
            SteelValue::U32(TEST_CLIENT_TIMEOUT_SECS + MIN_TIMEOUT_SECS),
        );
        after_exports.insert(
            "server-protocol".to_string(),
            SteelValue::U32(TEST_SERVER_PROTOCOL + MIN_TIMEOUT_SECS),
        );
        after_exports.insert(
            "server-port".to_string(),
            SteelValue::U32(TEST_SERVER_PORT + MIN_TIMEOUT_SECS),
        );
        let after =
            normalize_steel_exports(source(), &after_exports).expect("valid changed exports");

        let diffs = diff_snapshots(&before, &after);
        assert!(diffs.iter().any(|diff| diff.path == "client.timeout_secs"));
        assert!(diffs.iter().any(|diff| diff.path == "server.protocol"));
        assert!(diffs.iter().any(|diff| diff.path == "server.port"));

        let plan = build_apply_plan(diffs, false);
        assert_eq!(plan.hot.len(), 1);
        assert_eq!(plan.next_run.len(), 1);
        assert_eq!(plan.rejected.len(), 1);
        assert!(plan
            .rejected
            .iter()
            .any(|diagnostic| diagnostic.path == "server.port"));
    }

    #[test]
    fn fixed_protocol_fact_changes_are_rejected() {
        let plan = build_apply_plan(
            vec![FieldDiff {
                path: "protocol.packet_id.game_join",
                before: "0x28".to_string(),
                after: "0x29".to_string(),
                mutability: MutabilityClass::FixedProtocolFact,
            }],
            false,
        );

        assert_eq!(plan.hot.len(), 0);
        assert_eq!(plan.rejected.len(), 1);
        assert_eq!(plan.rejected[0].path, "protocol.packet_id.game_join");
    }
}
