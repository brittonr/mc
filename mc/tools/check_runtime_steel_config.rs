use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

const INVENTORY_PATH: &str = "docs/evidence/runtime-config-inventory-2026-05-27.tsv";
const CONTRACT_PATH: &str = "docs/evidence/steel-runtime-config-contract-2026-05-27.md";
const STEEL_MODULE_PATH: &str = "config/mc-compat/steel/default.scm";
const SNAPSHOT_PATH: &str = "docs/evidence/steel-runtime-config-default.snapshot.json";
const RUNNER_MAIN_PATH: &str = "tools/mc-compat-runner/src/main.rs";
const RUNTIME_CORE_PATH: &str = "tools/mc-compat-runner/src/runtime_config.rs";
const SUPPORTED_SCHEMA_VERSION: &str = "1";
const SANDBOX_PROFILE: &str = "mc-compat/pure-v1";
const MODULE_BLAKE3_HEX_LENGTH: usize = 64;
const INVENTORY_COLUMN_COUNT: usize = 10;

const INVENTORY_HEADER: &str = "config_path\towner\tsource\tcurrent_default\ttype\tcontract\tmutability\truntime_consumer\tmigration_status\tevidence_path";
const MUTABILITY_VALUES: &[&str] = &["hot", "next-run", "restart-only", "fixed-protocol-fact"];
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
const REQUIRED_INVENTORY_ROWS: &[&str] = &[
    "runtime.config_version",
    "runtime.steel.sandbox_profile",
    "server.backend",
    "server.version",
    "server.protocol",
    "server.port",
    "valence.rev",
    "valence.example",
    "valence.worktree",
    "valence.target_dir",
    "valence.log",
    "valence.pid_file",
    "client.username",
    "client.timeout_secs",
    "client.success_patterns",
    "receipt.dir",
    "scenario.name",
    "combat.arrow.base_damage",
    "combat.arrow.velocity_multiplier",
    "combat.arrow.max_damage",
];
const REQUIRED_STEEL_EXPORTS: &[&str] = &[
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
const GLOBAL_CODE_TOKENS: &[&str] = &[
    "--steel-config",
    "MC_COMPAT_STEEL_CONFIG",
    "apply_steel_config_file",
    "evaluate_scenario_for_config",
    "projectile_damage_amount_needle",
    "RuntimeConfigController",
    "reload_with",
    "redacted_value",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ManagedPathSpec {
    config_path: &'static str,
    mutability: &'static str,
    steel_export: &'static str,
    snapshot_token: &'static str,
    runtime_core_tokens: &'static [&'static str],
    runner_tokens: &'static [&'static str],
}

const MANAGED_PATH_SPECS: &[ManagedPathSpec] = &[
    ManagedPathSpec {
        config_path: "runtime.config_version",
        mutability: "restart-only",
        steel_export: "config-version",
        snapshot_token: "\"schema_version\"",
        runtime_core_tokens: &[
            "schema_version",
            "\"runtime.config_version\"",
            "\"config-version\"",
        ],
        runner_tokens: &["runtime_config::evaluate_steel_module"],
    },
    ManagedPathSpec {
        config_path: "runtime.steel.sandbox_profile",
        mutability: "restart-only",
        steel_export: "sandbox-profile",
        snapshot_token: "\"sandbox_profile\"",
        runtime_core_tokens: &[
            "sandbox_profile",
            "\"runtime.steel.sandbox_profile\"",
            "\"sandbox-profile\"",
        ],
        runner_tokens: &["sandbox_profile: \"mc-compat/pure-v1\""],
    },
    ManagedPathSpec {
        config_path: "server.backend",
        mutability: "next-run",
        steel_export: "server-backend",
        snapshot_token: "\"server_backend\"",
        runtime_core_tokens: &["server_backend", "\"server.backend\"", "\"server-backend\""],
        runner_tokens: &["cfg.server_backend", "snapshot.server_backend"],
    },
    ManagedPathSpec {
        config_path: "server.version",
        mutability: "next-run",
        steel_export: "server-version",
        snapshot_token: "\"server_version\"",
        runtime_core_tokens: &["server_version", "\"server.version\"", "\"server-version\""],
        runner_tokens: &["cfg.server_version", "snapshot.server_version"],
    },
    ManagedPathSpec {
        config_path: "server.protocol",
        mutability: "next-run",
        steel_export: "server-protocol",
        snapshot_token: "\"server_protocol\"",
        runtime_core_tokens: &[
            "server_protocol",
            "\"server.protocol\"",
            "\"server-protocol\"",
        ],
        runner_tokens: &["cfg.server_protocol", "snapshot.server_protocol"],
    },
    ManagedPathSpec {
        config_path: "server.port",
        mutability: "restart-only",
        steel_export: "server-port",
        snapshot_token: "\"server_port\"",
        runtime_core_tokens: &["server_port", "\"server.port\"", "\"server-port\""],
        runner_tokens: &["cfg.server_port", "snapshot.server_port"],
    },
    ManagedPathSpec {
        config_path: "valence.rev",
        mutability: "next-run",
        steel_export: "valence-rev",
        snapshot_token: "\"valence_rev\"",
        runtime_core_tokens: &["valence_rev", "\"valence.rev\"", "\"valence-rev\""],
        runner_tokens: &["cfg.valence_rev", "snapshot.valence_rev"],
    },
    ManagedPathSpec {
        config_path: "valence.example",
        mutability: "next-run",
        steel_export: "valence-example",
        snapshot_token: "\"valence_example\"",
        runtime_core_tokens: &[
            "valence_example",
            "\"valence.example\"",
            "\"valence-example\"",
        ],
        runner_tokens: &["cfg.valence_example", "snapshot.valence_example"],
    },
    ManagedPathSpec {
        config_path: "valence.worktree",
        mutability: "next-run",
        steel_export: "valence-worktree",
        snapshot_token: "\"valence_worktree\"",
        runtime_core_tokens: &[
            "valence_worktree",
            "\"valence.worktree\"",
            "\"valence-worktree\"",
        ],
        runner_tokens: &["cfg.valence_worktree", "snapshot.valence_worktree"],
    },
    ManagedPathSpec {
        config_path: "valence.target_dir",
        mutability: "next-run",
        steel_export: "valence-target-dir",
        snapshot_token: "\"valence_target_dir\"",
        runtime_core_tokens: &[
            "valence_target_dir",
            "\"valence.target_dir\"",
            "\"valence-target-dir\"",
        ],
        runner_tokens: &["cfg.valence_target_dir", "snapshot.valence_target_dir"],
    },
    ManagedPathSpec {
        config_path: "valence.log",
        mutability: "hot",
        steel_export: "valence-log",
        snapshot_token: "\"valence_log\"",
        runtime_core_tokens: &["valence_log", "\"valence.log\"", "\"valence-log\""],
        runner_tokens: &["cfg.valence_log", "snapshot.valence_log"],
    },
    ManagedPathSpec {
        config_path: "valence.pid_file",
        mutability: "restart-only",
        steel_export: "valence-pid-file",
        snapshot_token: "\"valence_pid_file\"",
        runtime_core_tokens: &[
            "valence_pid_file",
            "\"valence.pid_file\"",
            "\"valence-pid-file\"",
        ],
        runner_tokens: &["cfg.valence_pid_file", "snapshot.valence_pid_file"],
    },
    ManagedPathSpec {
        config_path: "client.username",
        mutability: "next-run",
        steel_export: "client-username",
        snapshot_token: "\"client_username\"",
        runtime_core_tokens: &[
            "client_username",
            "\"client.username\"",
            "\"client-username\"",
        ],
        runner_tokens: &["cfg.client_username", "snapshot.client_username"],
    },
    ManagedPathSpec {
        config_path: "client.timeout_secs",
        mutability: "hot",
        steel_export: "client-timeout-secs",
        snapshot_token: "\"client_timeout_secs\"",
        runtime_core_tokens: &[
            "client_timeout_secs",
            "\"client.timeout_secs\"",
            "\"client-timeout-secs\"",
        ],
        runner_tokens: &["cfg.client_timeout", "snapshot.client_timeout_secs"],
    },
    ManagedPathSpec {
        config_path: "client.success_patterns",
        mutability: "hot",
        steel_export: "client-success-patterns",
        snapshot_token: "\"client_success_patterns\"",
        runtime_core_tokens: &[
            "client_success_patterns",
            "\"client.success_patterns\"",
            "\"client-success-patterns\"",
        ],
        runner_tokens: &[
            "cfg.client_success_needles",
            "snapshot.client_success_patterns",
        ],
    },
    ManagedPathSpec {
        config_path: "receipt.dir",
        mutability: "next-run",
        steel_export: "receipt-dir",
        snapshot_token: "\"receipt_dir\"",
        runtime_core_tokens: &["receipt_dir", "\"receipt.dir\"", "\"receipt-dir\""],
        runner_tokens: &["cfg.receipt_dir", "snapshot.receipt_dir"],
    },
    ManagedPathSpec {
        config_path: "scenario.name",
        mutability: "next-run",
        steel_export: "scenario",
        snapshot_token: "\"scenario\"",
        runtime_core_tokens: &["scenario", "\"scenario.name\"", "\"scenario\""],
        runner_tokens: &["cfg.scenario", "snapshot.scenario"],
    },
    ManagedPathSpec {
        config_path: "combat.arrow.base_damage",
        mutability: "hot",
        steel_export: "arrow-base-damage",
        snapshot_token: "\"arrow_base_damage\"",
        runtime_core_tokens: &[
            "base_damage",
            "\"combat.arrow.base_damage\"",
            "\"arrow-base-damage\"",
        ],
        runner_tokens: &["cfg.arrow_damage_policy", "snapshot.arrow_damage"],
    },
    ManagedPathSpec {
        config_path: "combat.arrow.velocity_multiplier",
        mutability: "hot",
        steel_export: "arrow-velocity-multiplier",
        snapshot_token: "\"arrow_velocity_multiplier\"",
        runtime_core_tokens: &[
            "velocity_multiplier",
            "\"combat.arrow.velocity_multiplier\"",
            "\"arrow-velocity-multiplier\"",
        ],
        runner_tokens: &["cfg.arrow_damage_policy", "snapshot.arrow_damage"],
    },
    ManagedPathSpec {
        config_path: "combat.arrow.max_damage",
        mutability: "hot",
        steel_export: "arrow-max-damage",
        snapshot_token: "\"arrow_max_damage\"",
        runtime_core_tokens: &[
            "max_damage",
            "\"combat.arrow.max_damage\"",
            "\"arrow-max-damage\"",
        ],
        runner_tokens: &["cfg.arrow_damage_policy", "snapshot.arrow_damage"],
    },
];

const MIGRATED_STATUS: &str = "steel-startup-migrated";
const SNAPSHOT_MUTABILITY_BUCKETS: &[&str] =
    &["hot", "next_run", "restart_only", "fixed_protocol_fact"];

const REQUIRED_CONTRACT_TOKENS: &[&str] = &[
    "Steel module contract",
    "Rust-owned typed boundary",
    "Arrow damage policy contract",
    "Sandbox contract",
    "mc-compat/pure-v1",
    "damage-linear",
    "No filesystem watcher",
];

#[derive(Debug, Clone, PartialEq, Eq)]
struct InventoryRow {
    config_path: String,
    owner: String,
    source: String,
    current_default: String,
    value_type: String,
    contract: String,
    mutability: String,
    runtime_consumer: String,
    migration_status: String,
    evidence_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SteelModuleCheck {
    exports: BTreeMap<String, String>,
    has_arrow_policy: bool,
    forbidden_tokens: Vec<&'static str>,
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    let result = if args.iter().any(|arg| arg == "--self-test") {
        run_self_tests()
    } else {
        run_repo_checks(Path::new("."))
    };

    match result {
        Ok(message) => {
            println!("{message}");
            ExitCode::SUCCESS
        }
        Err(issues) => {
            for issue in issues {
                eprintln!("{issue}");
            }
            ExitCode::from(1)
        }
    }
}

fn run_repo_checks(root: &Path) -> Result<String, Vec<String>> {
    let inventory_text = read(root, INVENTORY_PATH)?;
    let contract_text = read(root, CONTRACT_PATH)?;
    let steel_text = read(root, STEEL_MODULE_PATH)?;
    let snapshot_text = read(root, SNAPSHOT_PATH)?;
    let runner_main_text = read(root, RUNNER_MAIN_PATH)?;
    let runtime_core_text = read(root, RUNTIME_CORE_PATH)?;
    let mut issues = Vec::new();

    let rows = parse_inventory(&inventory_text, &mut issues);
    issues.extend(validate_inventory(&rows));
    issues.extend(validate_contract_doc(&contract_text));
    let module = parse_steel_module(&steel_text);
    issues.extend(validate_steel_module(&module));
    let module_hash = compute_blake3(root.join(STEEL_MODULE_PATH)).unwrap_or_else(|err| {
        issues.push(err);
        String::new()
    });
    issues.extend(validate_snapshot(&snapshot_text, &module_hash));
    issues.extend(validate_migration_agreement(
        &rows,
        &module,
        &contract_text,
        &snapshot_text,
        &runner_main_text,
        &runtime_core_text,
    ));

    if issues.is_empty() {
        Ok(format!(
            "runtime steel config ok: {} inventory rows, {} Steel exports",
            rows.len(),
            module.exports.len()
        ))
    } else {
        Err(issues)
    }
}

fn read(root: &Path, relative: &str) -> Result<String, Vec<String>> {
    let path = root.join(relative);
    fs::read_to_string(&path).map_err(|err| vec![format!("read {}: {err}", path.display())])
}

fn parse_inventory(text: &str, issues: &mut Vec<String>) -> Vec<InventoryRow> {
    let mut lines = text.lines();
    let header = lines.next().unwrap_or_default();
    if header != INVENTORY_HEADER {
        issues.push(format!("inventory header mismatch: {header}"));
    }
    let mut rows = Vec::new();
    for (line_index, line) in lines.enumerate() {
        let columns: Vec<&str> = line.split('\t').collect();
        if columns.len() != INVENTORY_COLUMN_COUNT {
            issues.push(format!(
                "inventory line {} has {} columns, expected {}",
                line_index + 2,
                columns.len(),
                INVENTORY_COLUMN_COUNT
            ));
            continue;
        }
        rows.push(InventoryRow {
            config_path: columns[0].to_string(),
            owner: columns[1].to_string(),
            source: columns[2].to_string(),
            current_default: columns[3].to_string(),
            value_type: columns[4].to_string(),
            contract: columns[5].to_string(),
            mutability: columns[6].to_string(),
            runtime_consumer: columns[7].to_string(),
            migration_status: columns[8].to_string(),
            evidence_path: columns[9].to_string(),
        });
    }
    rows
}

fn validate_inventory(rows: &[InventoryRow]) -> Vec<String> {
    let mut issues = Vec::new();
    let mut seen = BTreeSet::new();
    for row in rows {
        if !seen.insert(row.config_path.clone()) {
            issues.push(format!("duplicate inventory row: {}", row.config_path));
        }
        for (field, value) in [
            ("owner", &row.owner),
            ("source", &row.source),
            ("current_default", &row.current_default),
            ("type", &row.value_type),
            ("contract", &row.contract),
            ("runtime_consumer", &row.runtime_consumer),
            ("migration_status", &row.migration_status),
            ("evidence_path", &row.evidence_path),
        ] {
            if value.trim().is_empty() {
                issues.push(format!("{} missing {field}", row.config_path));
            }
        }
        if !MUTABILITY_VALUES.contains(&row.mutability.as_str()) {
            issues.push(format!(
                "{} has invalid mutability {}",
                row.config_path, row.mutability
            ));
        }
    }
    let by_path: BTreeSet<&str> = rows.iter().map(|row| row.config_path.as_str()).collect();
    for required in REQUIRED_INVENTORY_ROWS {
        if !by_path.contains(required) {
            issues.push(format!("missing required inventory row: {required}"));
        }
    }
    issues
}

fn parse_steel_module(text: &str) -> SteelModuleCheck {
    let mut exports = BTreeMap::new();
    for line in text.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("(define ") {
            if rest.starts_with('(') {
                continue;
            }
            let name_end = rest
                .find(|ch: char| ch.is_whitespace() || ch == ')')
                .unwrap_or(rest.len());
            let name = &rest[..name_end];
            let value = rest[name_end..]
                .trim()
                .trim_end_matches(')')
                .trim()
                .to_string();
            exports.insert(name.to_string(), value);
        }
    }
    SteelModuleCheck {
        exports,
        has_arrow_policy: text.contains("(define (arrow-damage ctx)")
            && text.contains(
                "(damage-linear ctx arrow-base-damage arrow-velocity-multiplier arrow-max-damage)",
            ),
        forbidden_tokens: FORBIDDEN_STEEL_TOKENS
            .iter()
            .copied()
            .filter(|token| text.contains(token))
            .collect(),
    }
}

fn validate_steel_module(module: &SteelModuleCheck) -> Vec<String> {
    let mut issues = Vec::new();
    for export in REQUIRED_STEEL_EXPORTS {
        if !module.exports.contains_key(*export) {
            issues.push(format!("missing Steel export: {export}"));
        }
    }
    match module.exports.get("config-version") {
        Some(value) if value == SUPPORTED_SCHEMA_VERSION => {}
        Some(value) => issues.push(format!(
            "config-version expected {SUPPORTED_SCHEMA_VERSION}, found {value}"
        )),
        None => {}
    }
    match module.exports.get("sandbox-profile") {
        Some(value) if value == &format!("\"{SANDBOX_PROFILE}\"") => {}
        Some(value) => issues.push(format!(
            "sandbox-profile expected {SANDBOX_PROFILE}, found {value}"
        )),
        None => {}
    }
    if !module.has_arrow_policy {
        issues.push("missing or invalid arrow-damage policy shape".to_string());
    }
    for token in &module.forbidden_tokens {
        issues.push(format!("Steel module uses forbidden token: {token}"));
    }
    issues
}

fn validate_contract_doc(text: &str) -> Vec<String> {
    REQUIRED_CONTRACT_TOKENS
        .iter()
        .filter(|token| !text.contains(**token))
        .map(|token| format!("contract doc missing token: {token}"))
        .collect()
}

fn validate_migration_agreement(
    rows: &[InventoryRow],
    module: &SteelModuleCheck,
    contract_text: &str,
    snapshot_text: &str,
    runner_main_text: &str,
    runtime_core_text: &str,
) -> Vec<String> {
    let mut issues = Vec::new();
    let rows_by_path: BTreeMap<&str, &InventoryRow> = rows
        .iter()
        .map(|row| (row.config_path.as_str(), row))
        .collect();
    let snapshot_mutability_summary = match parse_snapshot_mutability_summary(snapshot_text) {
        Ok(summary) => summary,
        Err(issue) => {
            issues.push(issue);
            BTreeMap::new()
        }
    };

    let combined_code = format!("{runner_main_text}\n{runtime_core_text}");
    for token in GLOBAL_CODE_TOKENS {
        if !combined_code.contains(token) {
            issues.push(format!("migration code missing global token: {token}"));
        }
    }

    for spec in MANAGED_PATH_SPECS {
        match rows_by_path.get(spec.config_path) {
            Some(row) => {
                if row.mutability != spec.mutability {
                    issues.push(format!(
                        "{} inventory mutability {}, expected {}",
                        spec.config_path, row.mutability, spec.mutability
                    ));
                }
                if row.migration_status != MIGRATED_STATUS {
                    issues.push(format!(
                        "{} inventory migration status {}, expected {}",
                        spec.config_path, row.migration_status, MIGRATED_STATUS
                    ));
                }
            }
            None => issues.push(format!(
                "{} missing inventory row for managed Steel path",
                spec.config_path
            )),
        }

        if !module.exports.contains_key(spec.steel_export) {
            issues.push(format!(
                "{} missing matching Steel export {}",
                spec.config_path, spec.steel_export
            ));
        }

        match contract_row(contract_text, spec.config_path) {
            Some(line) if line.contains(&format!("| {} |", spec.mutability)) => {}
            Some(line) => issues.push(format!(
                "{} contract mutability mismatch: {line}",
                spec.config_path
            )),
            None => issues.push(format!("{} missing contract table row", spec.config_path)),
        }

        if !snapshot_text.contains(spec.snapshot_token) {
            issues.push(format!(
                "{} snapshot missing evaluated token {}",
                spec.config_path, spec.snapshot_token
            ));
        }
        let expected_bucket = snapshot_mutability_bucket(spec.mutability);
        let actual_buckets =
            snapshot_buckets_for_path(&snapshot_mutability_summary, spec.config_path);
        if !actual_buckets.contains(&expected_bucket) {
            issues.push(format!(
                "{} snapshot mutability missing expected bucket {}; actual {:?}",
                spec.config_path, expected_bucket, actual_buckets
            ));
        }
        for actual_bucket in &actual_buckets {
            if actual_bucket != &expected_bucket {
                issues.push(format!(
                    "{} snapshot mutability has unexpected bucket {}; expected {}",
                    spec.config_path, actual_bucket, expected_bucket
                ));
            }
        }

        for token in spec.runtime_core_tokens {
            if !runtime_core_text.contains(token) {
                issues.push(format!(
                    "{} typed boundary missing token {token}",
                    spec.config_path
                ));
            }
        }
        for token in spec.runner_tokens {
            if !runner_main_text.contains(token) {
                issues.push(format!(
                    "{} runtime consumer missing token {token}",
                    spec.config_path
                ));
            }
        }
    }
    issues
}

fn contract_row<'a>(text: &'a str, config_path: &str) -> Option<&'a str> {
    let needle = format!("| `{config_path}` |");
    text.lines().find(|line| line.contains(&needle))
}

fn parse_snapshot_mutability_summary(
    text: &str,
) -> Result<BTreeMap<&'static str, BTreeSet<String>>, String> {
    let summary = text
        .split_once("\"mutability_summary\"")
        .map(|(_, after_key)| after_key)
        .ok_or_else(|| "snapshot missing mutability_summary object".to_string())?;
    let mut buckets = BTreeMap::new();
    for bucket in SNAPSHOT_MUTABILITY_BUCKETS {
        let paths = parse_snapshot_bucket_paths(summary, bucket)
            .ok_or_else(|| format!("snapshot mutability_summary missing bucket {bucket}"))?;
        buckets.insert(*bucket, paths);
    }
    Ok(buckets)
}

fn parse_snapshot_bucket_paths(text: &str, bucket: &str) -> Option<BTreeSet<String>> {
    let bucket_key = format!("\"{bucket}\"");
    let after_key = text.split_once(&bucket_key)?.1;
    let after_array_start = after_key.split_once('[')?.1;
    let array_body = after_array_start.split_once(']')?.0;
    Some(parse_json_string_literals(array_body))
}

fn parse_json_string_literals(text: &str) -> BTreeSet<String> {
    let mut values = BTreeSet::new();
    let mut rest = text;
    while let Some((_, after_open_quote)) = rest.split_once('"') {
        let Some((value, after_close_quote)) = after_open_quote.split_once('"') else {
            break;
        };
        values.insert(value.to_string());
        rest = after_close_quote;
    }
    values
}

fn snapshot_buckets_for_path<'a>(
    summary: &'a BTreeMap<&'static str, BTreeSet<String>>,
    config_path: &str,
) -> BTreeSet<&'a str> {
    summary
        .iter()
        .filter_map(|(bucket, paths)| paths.contains(config_path).then_some(*bucket))
        .collect()
}

fn snapshot_mutability_bucket(mutability: &str) -> &'static str {
    match mutability {
        "hot" => "hot",
        "next-run" => "next_run",
        "restart-only" => "restart_only",
        "fixed-protocol-fact" => "fixed_protocol_fact",
        _ => "invalid",
    }
}

fn validate_snapshot(text: &str, module_hash: &str) -> Vec<String> {
    let mut issues = Vec::new();
    for token in [
        "mc.compat.runtime_config.snapshot.v1",
        "config/mc-compat/steel/default.scm",
        SANDBOX_PROFILE,
        "\"arrow-damage\"",
        "\"arrow_base_damage\"",
        "\"hot\"",
        "\"next_run\"",
        "\"restart_only\"",
    ] {
        if !text.contains(token) {
            issues.push(format!("snapshot missing token: {token}"));
        }
    }
    if module_hash.len() == MODULE_BLAKE3_HEX_LENGTH && !text.contains(module_hash) {
        issues.push(format!(
            "snapshot missing current module hash: {module_hash}"
        ));
    }
    issues
}

fn compute_blake3(path: PathBuf) -> Result<String, String> {
    let output = Command::new("b3sum")
        .arg(&path)
        .output()
        .map_err(|err| format!("run b3sum {}: {err}", path.display()))?;
    if !output.status.success() {
        return Err(format!("b3sum {} failed", path.display()));
    }
    let stdout = String::from_utf8(output.stdout)
        .map_err(|err| format!("b3sum output was not utf8: {err}"))?;
    Ok(stdout
        .split_whitespace()
        .next()
        .unwrap_or_default()
        .to_string())
}

fn run_self_tests() -> Result<String, Vec<String>> {
    let valid_inventory = valid_inventory_text();
    let mut issues = Vec::new();
    let rows = parse_inventory(&valid_inventory, &mut issues);
    issues.extend(validate_inventory(&rows));
    assert!(issues.is_empty(), "valid inventory failed: {issues:?}");

    let bad_inventory =
        valid_inventory.replace("combat.arrow.max_damage", "combat.arrow.missing_max_damage");
    let mut bad_issues = Vec::new();
    let bad_rows = parse_inventory(&bad_inventory, &mut bad_issues);
    bad_issues.extend(validate_inventory(&bad_rows));
    assert!(
        bad_issues
            .iter()
            .any(|issue| issue.contains("missing required inventory row")),
        "missing inventory row not rejected: {bad_issues:?}"
    );

    let invalid_mutability = valid_inventory.replacen("hot\tconsumer", "sometimes\tconsumer", 1);
    let mut invalid_issues = Vec::new();
    let invalid_rows = parse_inventory(&invalid_mutability, &mut invalid_issues);
    invalid_issues.extend(validate_inventory(&invalid_rows));
    assert!(
        invalid_issues
            .iter()
            .any(|issue| issue.contains("invalid mutability")),
        "bad mutability not rejected: {invalid_issues:?}"
    );

    let valid_module = required_module_text();
    let module = parse_steel_module(&valid_module);
    let module_issues = validate_steel_module(&module);
    assert!(
        module_issues.is_empty(),
        "valid module failed: {module_issues:?}"
    );

    let missing_export_module = valid_module.replace("(define arrow-max-damage 1.0)\n", "");
    let module_issues = validate_steel_module(&parse_steel_module(&missing_export_module));
    assert!(
        module_issues
            .iter()
            .any(|issue| issue.contains("missing Steel export: arrow-max-damage")),
        "missing Steel export not rejected: {module_issues:?}"
    );

    let forbidden_module = format!("{valid_module}\n(open-input-file \"/etc/passwd\")\n");
    let module_issues = validate_steel_module(&parse_steel_module(&forbidden_module));
    assert!(
        module_issues
            .iter()
            .any(|issue| issue.contains("forbidden token")),
        "forbidden token not rejected: {module_issues:?}"
    );

    let bad_policy = valid_module.replace(
        "(damage-linear ctx arrow-base-damage arrow-velocity-multiplier arrow-max-damage)",
        "42",
    );
    let module_issues = validate_steel_module(&parse_steel_module(&bad_policy));
    assert!(
        module_issues
            .iter()
            .any(|issue| issue.contains("arrow-damage policy")),
        "bad arrow policy not rejected: {module_issues:?}"
    );

    let agreement_issues = validate_migration_agreement(
        &rows,
        &module,
        &contract_text_for_specs(),
        &snapshot_text_for_specs(None),
        &runner_text_for_specs(),
        &runtime_text_for_specs(),
    );
    assert!(
        agreement_issues.is_empty(),
        "valid migration agreement failed: {agreement_issues:?}"
    );

    let wrong_bucket_snapshot =
        snapshot_text_for_specs(Some(("combat.arrow.max_damage", "next_run")));
    let wrong_bucket_issues = validate_migration_agreement(
        &rows,
        &module,
        &contract_text_for_specs(),
        &wrong_bucket_snapshot,
        &runner_text_for_specs(),
        &runtime_text_for_specs(),
    );
    assert!(
        wrong_bucket_issues
            .iter()
            .any(|issue| issue.contains("snapshot mutability")),
        "wrong snapshot bucket not rejected: {wrong_bucket_issues:?}"
    );

    let wrong_status_inventory = valid_inventory.replacen(MIGRATED_STATUS, "inventoried", 1);
    let mut wrong_status_parse_issues = Vec::new();
    let wrong_status_rows =
        parse_inventory(&wrong_status_inventory, &mut wrong_status_parse_issues);
    let wrong_status_issues = validate_migration_agreement(
        &wrong_status_rows,
        &module,
        &contract_text_for_specs(),
        &snapshot_text_for_specs(None),
        &runner_text_for_specs(),
        &runtime_text_for_specs(),
    );
    assert!(
        wrong_status_issues
            .iter()
            .any(|issue| issue.contains("migration status")),
        "wrong migration status not rejected: {wrong_status_issues:?}"
    );

    let missing_runtime_consumer_issues = validate_migration_agreement(
        &rows,
        &module,
        &contract_text_for_specs(),
        &snapshot_text_for_specs(None),
        "",
        &runtime_text_for_specs(),
    );
    assert!(
        missing_runtime_consumer_issues
            .iter()
            .any(|issue| issue.contains("runtime consumer missing token")),
        "missing runtime consumer token not rejected: {missing_runtime_consumer_issues:?}"
    );

    let snapshot_issues = validate_snapshot(
        "mc.compat.runtime_config.snapshot.v1 config/mc-compat/steel/default.scm mc-compat/pure-v1 \"arrow-damage\" \"arrow_base_damage\" \"hot\" \"next_run\" \"restart_only\" abc",
        "def0000000000000000000000000000000000000000000000000000000000000",
    );
    assert!(
        snapshot_issues
            .iter()
            .any(|issue| issue.contains("current module hash")),
        "snapshot hash mismatch not rejected: {snapshot_issues:?}"
    );

    Ok("runtime steel config self-test ok".to_string())
}

fn valid_inventory_text() -> String {
    let mut text = format!("{INVENTORY_HEADER}\n");
    for spec in MANAGED_PATH_SPECS {
        text.push_str(&row(spec.config_path, spec.mutability));
        text.push('\n');
    }
    text
}

fn contract_text_for_specs() -> String {
    let mut text = REQUIRED_CONTRACT_TOKENS.join("\n");
    text.push_str("\n| Path | Type | Contract | Mutability |\n");
    text.push_str("| --- | --- | --- | --- |\n");
    for spec in MANAGED_PATH_SPECS {
        text.push_str(&format!(
            "| `{}` | type | contract | {} |\n",
            spec.config_path, spec.mutability
        ));
    }
    text
}

fn snapshot_text_for_specs(bucket_override: Option<(&str, &str)>) -> String {
    let mut text = String::from(
        "{\n  \"schema\": \"mc.compat.runtime_config.snapshot.v1\",\n  \"source\": {\n    \"path\": \"config/mc-compat/steel/default.scm\",\n    \"sandbox_profile\": \"mc-compat/pure-v1\"\n  },\n  \"evaluated_exports\": {\n",
    );
    for spec in MANAGED_PATH_SPECS {
        text.push_str(&format!("    {}: \"value\",\n", spec.snapshot_token));
    }
    text.push_str("    \"arrow_base_damage\": 1.0\n  },\n  \"policy_exports\": [\"arrow-damage\"],\n  \"mutability_summary\": {\n");
    for bucket in SNAPSHOT_MUTABILITY_BUCKETS {
        text.push_str(&format!(
            "    \"{}\": {}{}\n",
            bucket,
            snapshot_bucket_array_for_specs(bucket, bucket_override),
            snapshot_bucket_separator(bucket)
        ));
    }
    text.push_str("  }\n}\n");
    text
}

fn snapshot_bucket_separator(bucket: &str) -> &'static str {
    if bucket == SNAPSHOT_MUTABILITY_BUCKETS[SNAPSHOT_MUTABILITY_BUCKETS.len() - 1] {
        ""
    } else {
        ","
    }
}

fn snapshot_bucket_array_for_specs(bucket: &str, bucket_override: Option<(&str, &str)>) -> String {
    let mut paths = Vec::new();
    for spec in MANAGED_PATH_SPECS {
        let actual_bucket = bucket_override
            .filter(|(config_path, _)| *config_path == spec.config_path)
            .map(|(_, override_bucket)| override_bucket)
            .unwrap_or_else(|| snapshot_mutability_bucket(spec.mutability));
        if actual_bucket == bucket {
            paths.push(spec.config_path);
        }
    }
    let quoted_paths: Vec<String> = paths.iter().map(|path| format!("\"{path}\"")).collect();
    format!("[{}]", quoted_paths.join(", "))
}

fn runtime_text_for_specs() -> String {
    let mut text = GLOBAL_CODE_TOKENS.join(" ");
    text.push(' ');
    for spec in MANAGED_PATH_SPECS {
        text.push_str(spec.steel_export);
        text.push(' ');
        for token in spec.runtime_core_tokens {
            text.push_str(token);
            text.push(' ');
        }
    }
    text
}

fn runner_text_for_specs() -> String {
    let mut text = GLOBAL_CODE_TOKENS.join(" ");
    text.push(' ');
    for spec in MANAGED_PATH_SPECS {
        for token in spec.runner_tokens {
            text.push_str(token);
            text.push(' ');
        }
    }
    text
}

fn row(config_path: &str, mutability: &str) -> String {
    format!("{config_path}\tagent\tsource\tdefault\ttype\tcontract\t{mutability}\tconsumer\t{MIGRATED_STATUS}\tevidence")
}

fn required_module_text() -> String {
    let mut text = String::new();
    for export in REQUIRED_STEEL_EXPORTS {
        let value = match *export {
            "config-version" => "1".to_string(),
            "sandbox-profile" => format!("\"{SANDBOX_PROFILE}\""),
            _ if export.contains("damage") || export.contains("multiplier") => "1.0".to_string(),
            _ if export.contains("protocol")
                || export.contains("port")
                || export.contains("timeout") =>
            {
                "1".to_string()
            }
            _ => "\"value\"".to_string(),
        };
        text.push_str(&format!("(define {export} {value})\n"));
    }
    text.push_str("(define (arrow-damage ctx)\n");
    text.push_str(
        "  (damage-linear ctx arrow-base-damage arrow-velocity-multiplier arrow-max-damage))\n",
    );
    text
}
