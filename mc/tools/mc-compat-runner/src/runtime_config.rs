use std::collections::BTreeMap;

const SUPPORTED_SCHEMA_VERSION: u32 = 1;
const REQUIRED_ARROW_POLICY_NEEDLE: &str =
    "(damage-linear ctx arrow-base-damage arrow-velocity-multiplier arrow-max-damage)";
const STEEL_DEFINE_PREFIX: &str = "(define ";
const SUPPORTED_SANDBOX_PROFILE: &str = "mc-compat/pure-v1";
const MIN_PORT: u32 = 1;
const MAX_PORT: u32 = u16::MAX as u32;
const MIN_TIMEOUT_SECS: u32 = 1;
const MIN_DAMAGE: f64 = 0.0;
const MAX_DAMAGE: f64 = 100.0;
const MIN_MULTIPLIER: f64 = 0.0;
const MAX_MULTIPLIER: f64 = 100.0;
const ZERO_DAMAGE: f64 = 0.0;

const FORBIDDEN_STEEL_TOKENS: &[&str] = &[
    "open-input-file",
    "call-with-input-file",
    "delete-file",
    "system",
    "process",
    "tcp-connect",
    "current-second",
    "random",
];

const ALLOWED_STEEL_EXPORTS: &[&str] = &[
    "config-version",
    "sandbox-profile",
    "server-backend",
    "server-version",
    "server-protocol",
    "server-port",
    "valence-rev",
    "valence-example",
    "valence-worktree",
    "valence-target-dir",
    "valence-log",
    "valence-pid-file",
    "client-username",
    "client-timeout-secs",
    "client-success-patterns",
    "receipt-dir",
    "scenario",
    "arrow-base-damage",
    "arrow-velocity-multiplier",
    "arrow-max-damage",
];

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
    pub(crate) server_version: String,
    pub(crate) server_protocol: u32,
    pub(crate) server_port: u16,
    pub(crate) valence_rev: String,
    pub(crate) valence_example: String,
    pub(crate) valence_worktree: String,
    pub(crate) valence_target_dir: String,
    pub(crate) valence_log: String,
    pub(crate) valence_pid_file: String,
    pub(crate) client_username: String,
    pub(crate) client_timeout_secs: u32,
    pub(crate) client_success_patterns: Vec<String>,
    pub(crate) receipt_dir: String,
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

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ReloadOutcome {
    pub(crate) active_changed: bool,
    pub(crate) plan: ApplyPlan,
    pub(crate) diagnostics: Vec<ConfigDiagnostic>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct RuntimeConfigController {
    active: RuntimeConfigSnapshot,
}

impl RuntimeConfigController {
    pub(crate) fn new(active: RuntimeConfigSnapshot) -> Self {
        Self { active }
    }

    pub(crate) fn active(&self) -> &RuntimeConfigSnapshot {
        &self.active
    }

    pub(crate) fn reload_with<F>(
        &mut self,
        candidate: RuntimeConfigSnapshot,
        mut apply_hot: F,
    ) -> ReloadOutcome
    where
        F: FnMut(&[FieldDiff]) -> Result<(), String>,
    {
        let diffs = diff_snapshots(&self.active, &candidate);
        let plan = build_apply_plan(diffs, false);
        if !plan.rejected.is_empty() {
            return ReloadOutcome {
                active_changed: false,
                diagnostics: plan.rejected.clone(),
                plan,
            };
        }
        if let Err(message) = apply_hot(&plan.hot) {
            return ReloadOutcome {
                active_changed: false,
                diagnostics: vec![ConfigDiagnostic {
                    path: "runtime.reload.apply_hot",
                    message,
                }],
                plan,
            };
        }
        self.active = candidate;
        ReloadOutcome {
            active_changed: true,
            plan,
            diagnostics: Vec::new(),
        }
    }
}

pub(crate) fn evaluate_steel_module(
    source: SteelSource,
    module_text: &str,
) -> Result<RuntimeConfigSnapshot, Vec<ConfigDiagnostic>> {
    let mut diagnostics = validate_steel_module_text(module_text);
    let exports = parse_steel_literal_exports(module_text, &mut diagnostics);
    if diagnostics.is_empty() {
        normalize_steel_exports(source, &exports)
    } else {
        Err(diagnostics)
    }
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

    let server_version = required_string(
        exports,
        "server-version",
        "server.version",
        &mut diagnostics,
    );
    let server_protocol = required_u32(
        exports,
        "server-protocol",
        "server.protocol",
        &mut diagnostics,
    );
    let server_port = required_u32(exports, "server-port", "server.port", &mut diagnostics)
        .and_then(|value| validate_u16_port(value, &mut diagnostics));
    let valence_rev = required_string(exports, "valence-rev", "valence.rev", &mut diagnostics);
    let valence_example = required_string(
        exports,
        "valence-example",
        "valence.example",
        &mut diagnostics,
    );
    let valence_worktree = required_string(
        exports,
        "valence-worktree",
        "valence.worktree",
        &mut diagnostics,
    );
    let valence_target_dir = required_string(
        exports,
        "valence-target-dir",
        "valence.target_dir",
        &mut diagnostics,
    );
    let valence_log = required_string(exports, "valence-log", "valence.log", &mut diagnostics);
    let valence_pid_file = required_string(
        exports,
        "valence-pid-file",
        "valence.pid_file",
        &mut diagnostics,
    );
    let client_username = required_string(
        exports,
        "client-username",
        "client.username",
        &mut diagnostics,
    );
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
    let receipt_dir = required_string(exports, "receipt-dir", "receipt.dir", &mut diagnostics);
    let scenario = required_string(exports, "scenario", "scenario.name", &mut diagnostics);
    let arrow_damage = normalize_arrow_damage(exports, &mut diagnostics);

    if diagnostics.is_empty() {
        Ok(RuntimeConfigSnapshot {
            schema_version: schema_version.expect("diagnostics checked"),
            source,
            server_backend: server_backend.expect("diagnostics checked"),
            server_version: server_version.expect("diagnostics checked"),
            server_protocol: server_protocol.expect("diagnostics checked"),
            server_port: server_port.expect("diagnostics checked"),
            valence_rev: valence_rev.expect("diagnostics checked"),
            valence_example: valence_example.expect("diagnostics checked"),
            valence_worktree: valence_worktree.expect("diagnostics checked"),
            valence_target_dir: valence_target_dir.expect("diagnostics checked"),
            valence_log: valence_log.expect("diagnostics checked"),
            valence_pid_file: valence_pid_file.expect("diagnostics checked"),
            client_username: client_username.expect("diagnostics checked"),
            client_timeout_secs: client_timeout_secs.expect("diagnostics checked"),
            client_success_patterns: client_success_patterns.expect("diagnostics checked"),
            receipt_dir: receipt_dir.expect("diagnostics checked"),
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
        "server.version",
        &before.server_version,
        &after.server_version,
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
        "valence.rev",
        &before.valence_rev,
        &after.valence_rev,
        MutabilityClass::NextRun,
    );
    push_diff(
        &mut diffs,
        "valence.example",
        &before.valence_example,
        &after.valence_example,
        MutabilityClass::NextRun,
    );
    push_diff(
        &mut diffs,
        "valence.worktree",
        &before.valence_worktree,
        &after.valence_worktree,
        MutabilityClass::NextRun,
    );
    push_diff(
        &mut diffs,
        "valence.target_dir",
        &before.valence_target_dir,
        &after.valence_target_dir,
        MutabilityClass::NextRun,
    );
    push_diff(
        &mut diffs,
        "valence.log",
        &before.valence_log,
        &after.valence_log,
        MutabilityClass::Hot,
    );
    push_diff(
        &mut diffs,
        "valence.pid_file",
        &before.valence_pid_file,
        &after.valence_pid_file,
        MutabilityClass::RestartOnly,
    );
    push_diff(
        &mut diffs,
        "client.username",
        &before.client_username,
        &after.client_username,
        MutabilityClass::NextRun,
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
        "receipt.dir",
        &before.receipt_dir,
        &after.receipt_dir,
        MutabilityClass::NextRun,
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

pub(crate) fn redacted_value(path: &str, value: &str) -> String {
    if path.contains("secret") || path.contains("token") || path.contains("password") {
        "<redacted>".to_string()
    } else {
        value.to_string()
    }
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

fn validate_steel_module_text(module_text: &str) -> Vec<ConfigDiagnostic> {
    let mut diagnostics = Vec::new();
    for token in FORBIDDEN_STEEL_TOKENS {
        if module_text.contains(token) {
            diagnostics.push(ConfigDiagnostic {
                path: "runtime.steel.sandbox_profile",
                message: format!("forbidden Steel capability token {token}"),
            });
        }
    }
    if !module_text.contains("(define (arrow-damage ctx)")
        || !module_text.contains(REQUIRED_ARROW_POLICY_NEEDLE)
    {
        diagnostics.push(ConfigDiagnostic {
            path: "combat.arrow.policy",
            message: "missing supported arrow-damage policy shape".to_string(),
        });
    }
    diagnostics
}

fn parse_steel_literal_exports(
    module_text: &str,
    diagnostics: &mut Vec<ConfigDiagnostic>,
) -> BTreeMap<String, SteelValue> {
    let mut exports = BTreeMap::new();
    for export in ALLOWED_STEEL_EXPORTS {
        if let Some(body) = extract_define_body(module_text, export) {
            if let Some(value) = parse_steel_value(export, &body, diagnostics) {
                exports.insert((*export).to_string(), value);
            }
        }
    }

    let defined_constants = defined_constant_names(module_text);
    for defined in defined_constants {
        if !ALLOWED_STEEL_EXPORTS.contains(&defined.as_str()) {
            diagnostics.push(ConfigDiagnostic {
                path: "runtime.steel.exports",
                message: format!("unknown Steel export {defined}"),
            });
        }
    }
    exports
}

fn defined_constant_names(module_text: &str) -> Vec<String> {
    module_text
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            let rest = trimmed.strip_prefix(STEEL_DEFINE_PREFIX)?;
            if rest.starts_with('(') {
                return None;
            }
            let name_end = rest
                .find(|ch: char| ch.is_whitespace() || ch == ')')
                .unwrap_or(rest.len());
            Some(rest[..name_end].to_string())
        })
        .collect()
}

fn extract_define_body(module_text: &str, export: &str) -> Option<String> {
    let needle = format!("{STEEL_DEFINE_PREFIX}{export}");
    let start = module_text.find(&needle)?;
    let mut depth = 0_u32;
    let mut end = None;
    for (offset, ch) in module_text[start..].char_indices() {
        match ch {
            '(' => depth += 1,
            ')' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    end = Some(start + offset);
                    break;
                }
            }
            _ => {}
        }
    }
    let end = end?;
    Some(module_text[start + needle.len()..end].trim().to_string())
}

fn parse_steel_value(
    export: &'static str,
    body: &str,
    diagnostics: &mut Vec<ConfigDiagnostic>,
) -> Option<SteelValue> {
    match export {
        "config-version" | "server-protocol" | "server-port" | "client-timeout-secs" => body
            .parse::<u32>()
            .map(SteelValue::U32)
            .map_err(|err| {
                diagnostics.push(ConfigDiagnostic {
                    path: steel_export_path(export),
                    message: format!("parse {export} as u32: {err}"),
                });
            })
            .ok(),
        "arrow-base-damage" | "arrow-velocity-multiplier" | "arrow-max-damage" => body
            .parse::<f64>()
            .map(SteelValue::F64)
            .map_err(|err| {
                diagnostics.push(ConfigDiagnostic {
                    path: steel_export_path(export),
                    message: format!("parse {export} as f64: {err}"),
                });
            })
            .ok(),
        "client-success-patterns" => Some(SteelValue::StringList(parse_steel_string_list(body))),
        _ => parse_steel_string(body)
            .map(SteelValue::String)
            .or_else(|| {
                diagnostics.push(ConfigDiagnostic {
                    path: steel_export_path(export),
                    message: format!("parse {export} as string"),
                });
                None
            }),
    }
}

fn parse_steel_string(body: &str) -> Option<String> {
    let body = body.trim();
    let without_prefix = body.strip_prefix('"')?;
    let end = without_prefix.find('"')?;
    Some(without_prefix[..end].to_string())
}

fn parse_steel_string_list(body: &str) -> Vec<String> {
    let mut values = Vec::new();
    let mut rest = body;
    while let Some(start) = rest.find('"') {
        let after_start = &rest[start + 1..];
        if let Some(end) = after_start.find('"') {
            values.push(after_start[..end].to_string());
            rest = &after_start[end + 1..];
        } else {
            break;
        }
    }
    values
}

fn steel_export_path(export: &str) -> &'static str {
    match export {
        "config-version" => "runtime.config_version",
        "sandbox-profile" => "runtime.steel.sandbox_profile",
        "server-backend" => "server.backend",
        "server-version" => "server.version",
        "server-protocol" => "server.protocol",
        "server-port" => "server.port",
        "valence-rev" => "valence.rev",
        "valence-example" => "valence.example",
        "valence-worktree" => "valence.worktree",
        "valence-target-dir" => "valence.target_dir",
        "valence-log" => "valence.log",
        "valence-pid-file" => "valence.pid_file",
        "client-username" => "client.username",
        "client-timeout-secs" => "client.timeout_secs",
        "client-success-patterns" => "client.success_patterns",
        "receipt-dir" => "receipt.dir",
        "scenario" => "scenario.name",
        "arrow-base-damage" => "combat.arrow.base_damage",
        "arrow-velocity-multiplier" => "combat.arrow.velocity_multiplier",
        "arrow-max-damage" => "combat.arrow.max_damage",
        _ => "runtime.steel.exports",
    }
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

    fn valid_module_text() -> String {
        r#"
(define config-version 1)
(define sandbox-profile "mc-compat/pure-v1")
(define server-backend "valence")
(define server-version "1.20.1")
(define server-protocol 763)
(define server-port 25565)
(define valence-rev "main")
(define valence-example "ctf")
(define valence-worktree "/tmp/valence-compat-763")
(define valence-target-dir "/tmp/valence-compat-763-target")
(define valence-log "/tmp/mc-compat-valence.log")
(define valence-pid-file "/tmp/mc-compat-valence.pid")
(define client-username "compatbot")
(define client-timeout-secs 120)
(define client-success-patterns
  (list "Detected server protocol version"
        "Dimension type:"))
(define receipt-dir "target/mc-compat-steel")
(define scenario "projectile-damage-attribution")
(define arrow-base-damage 3.0)
(define arrow-velocity-multiplier 1.0)
(define arrow-max-damage 10.0)
(define (arrow-damage ctx)
  (damage-linear ctx arrow-base-damage arrow-velocity-multiplier arrow-max-damage))
"#
        .to_string()
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
                "server-version".to_string(),
                SteelValue::String("1.20.1".to_string()),
            ),
            (
                "server-protocol".to_string(),
                SteelValue::U32(TEST_SERVER_PROTOCOL),
            ),
            ("server-port".to_string(), SteelValue::U32(TEST_SERVER_PORT)),
            (
                "valence-rev".to_string(),
                SteelValue::String("main".to_string()),
            ),
            (
                "valence-example".to_string(),
                SteelValue::String("ctf".to_string()),
            ),
            (
                "valence-worktree".to_string(),
                SteelValue::String("/tmp/valence-compat-763".to_string()),
            ),
            (
                "valence-target-dir".to_string(),
                SteelValue::String("/tmp/valence-compat-763-target".to_string()),
            ),
            (
                "valence-log".to_string(),
                SteelValue::String("/tmp/mc-compat-valence.log".to_string()),
            ),
            (
                "valence-pid-file".to_string(),
                SteelValue::String("/tmp/mc-compat-valence.pid".to_string()),
            ),
            (
                "client-username".to_string(),
                SteelValue::String("compatbot".to_string()),
            ),
            (
                "client-timeout-secs".to_string(),
                SteelValue::U32(TEST_CLIENT_TIMEOUT_SECS),
            ),
            (
                "client-success-patterns".to_string(),
                SteelValue::StringList(vec!["Detected server protocol version".to_string()]),
            ),
            (
                "receipt-dir".to_string(),
                SteelValue::String("target/mc-compat-steel".to_string()),
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
    fn evaluates_restricted_steel_module() {
        let snapshot = evaluate_steel_module(source(), &valid_module_text()).expect("valid module");

        assert_eq!(snapshot.schema_version, SUPPORTED_SCHEMA_VERSION);
        assert_eq!(snapshot.server_backend, "valence");
        assert_eq!(snapshot.client_success_patterns.len(), 2);
        assert_eq!(snapshot.arrow_damage.max_damage, TEST_ARROW_MAX_DAMAGE);
    }

    #[test]
    fn rejects_steel_module_unknown_export_and_forbidden_token() {
        let unknown_export = format!("{}\n(define surprise-value 1)\n", valid_module_text());
        let diagnostics = evaluate_steel_module(source(), &unknown_export).unwrap_err();
        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.message.contains("unknown Steel export")));

        let forbidden = format!(
            "{}\n(open-input-file \"/etc/passwd\")\n",
            valid_module_text()
        );
        let diagnostics = evaluate_steel_module(source(), &forbidden).unwrap_err();
        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.message.contains("forbidden Steel capability")));

        let nondeterministic = format!("{}\n(random)\n", valid_module_text());
        let diagnostics = evaluate_steel_module(source(), &nondeterministic).unwrap_err();
        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.message.contains("random")));

        let malformed_policy = valid_module_text().replace(REQUIRED_ARROW_POLICY_NEEDLE, "42");
        let diagnostics = evaluate_steel_module(source(), &malformed_policy).unwrap_err();
        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.path == "combat.arrow.policy"));
    }

    #[test]
    fn normalizes_valid_steel_exports() {
        let snapshot = normalize_steel_exports(source(), &valid_exports()).expect("valid exports");

        assert_eq!(snapshot.schema_version, SUPPORTED_SCHEMA_VERSION);
        assert_eq!(snapshot.server_backend, "valence");
        assert_eq!(snapshot.server_version, "1.20.1");
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

    #[test]
    fn reload_request_applies_hot_changes_atomically() {
        let before = normalize_steel_exports(source(), &valid_exports()).expect("valid exports");
        let mut after_exports = valid_exports();
        after_exports.insert(
            "client-timeout-secs".to_string(),
            SteelValue::U32(TEST_CLIENT_TIMEOUT_SECS + MIN_TIMEOUT_SECS),
        );
        let after =
            normalize_steel_exports(source(), &after_exports).expect("valid changed exports");
        let mut controller = RuntimeConfigController::new(before);

        let outcome = controller.reload_with(after, |hot| {
            assert_eq!(hot.len(), 1);
            Ok(())
        });

        assert!(outcome.active_changed);
        assert!(outcome.diagnostics.is_empty());
        assert_eq!(
            controller.active().client_timeout_secs,
            TEST_CLIENT_TIMEOUT_SECS + MIN_TIMEOUT_SECS
        );
    }

    #[test]
    fn redaction_hides_secret_like_values() {
        assert_eq!(
            redacted_value("auth.token", "super-secret-token"),
            "<redacted>"
        );
        assert_eq!(redacted_value("server.backend", "valence"), "valence");
    }

    #[test]
    fn reload_request_rolls_back_on_apply_failure_or_restart_only_change() {
        let before = normalize_steel_exports(source(), &valid_exports()).expect("valid exports");
        let mut hot_after_exports = valid_exports();
        hot_after_exports.insert(
            "client-timeout-secs".to_string(),
            SteelValue::U32(TEST_CLIENT_TIMEOUT_SECS + MIN_TIMEOUT_SECS),
        );
        let hot_after =
            normalize_steel_exports(source(), &hot_after_exports).expect("valid hot change");
        let mut controller = RuntimeConfigController::new(before.clone());
        let outcome =
            controller.reload_with(hot_after, |_| Err("apply handler failed".to_string()));
        assert!(!outcome.active_changed);
        assert_eq!(
            controller.active().client_timeout_secs,
            before.client_timeout_secs
        );
        assert!(outcome
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.path == "runtime.reload.apply_hot"));

        let mut restart_after_exports = valid_exports();
        restart_after_exports.insert(
            "server-port".to_string(),
            SteelValue::U32(TEST_SERVER_PORT + MIN_TIMEOUT_SECS),
        );
        let restart_after = normalize_steel_exports(source(), &restart_after_exports)
            .expect("valid restart-only change");
        let outcome = controller.reload_with(restart_after, |_| Ok(()));
        assert!(!outcome.active_changed);
        assert_eq!(controller.active().server_port, before.server_port);
        assert!(outcome
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.path == "server.port"));
    }
}
