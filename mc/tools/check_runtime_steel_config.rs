use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

const INVENTORY_PATH: &str = "docs/evidence/runtime-config-inventory-2026-05-27.tsv";
const CONTRACT_PATH: &str = "docs/evidence/steel-runtime-config-contract-2026-05-27.md";
const STEEL_MODULE_PATH: &str = "config/mc-compat/steel/default.scm";
const SNAPSHOT_PATH: &str = "docs/evidence/steel-runtime-config-default.snapshot.json";
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
    "server.protocol",
    "server.port",
    "client.timeout_secs",
    "client.success_patterns",
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
const REQUIRED_CONTRACT_TOKENS: &[&str] = &[
    "Steel module contract",
    "Rust-owned typed boundary",
    "Arrow damage policy contract",
    "Sandbox contract",
    "mc-compat/pure-v1",
    "damage-linear",
    "No live runtime file watcher",
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
            && text.contains("(damage-linear ctx arrow-base-damage arrow-velocity-multiplier arrow-max-damage)"),
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
        Some(value) => issues.push(format!("config-version expected {SUPPORTED_SCHEMA_VERSION}, found {value}")),
        None => {}
    }
    match module.exports.get("sandbox-profile") {
        Some(value) if value == &format!("\"{SANDBOX_PROFILE}\"") => {}
        Some(value) => issues.push(format!("sandbox-profile expected {SANDBOX_PROFILE}, found {value}")),
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
        issues.push(format!("snapshot missing current module hash: {module_hash}"));
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
    Ok(stdout.split_whitespace().next().unwrap_or_default().to_string())
}

fn run_self_tests() -> Result<String, Vec<String>> {
    let valid_inventory = format!(
        "{INVENTORY_HEADER}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n",
        row("runtime.config_version", "restart-only"),
        row("runtime.steel.sandbox_profile", "restart-only"),
        row("server.backend", "next-run"),
        row("server.protocol", "next-run"),
        row("server.port", "restart-only"),
        row("client.timeout_secs", "hot"),
        row("client.success_patterns", "hot"),
        row("combat.arrow.base_damage", "hot"),
        row("combat.arrow.velocity_multiplier", "hot"),
        row("combat.arrow.max_damage", "hot")
    );
    let mut issues = Vec::new();
    let rows = parse_inventory(&valid_inventory, &mut issues);
    issues.extend(validate_inventory(&rows));
    assert!(issues.is_empty(), "valid inventory failed: {issues:?}");

    let bad_inventory = valid_inventory.replace("combat.arrow.max_damage", "combat.arrow.missing_max_damage");
    let mut bad_issues = Vec::new();
    let bad_rows = parse_inventory(&bad_inventory, &mut bad_issues);
    bad_issues.extend(validate_inventory(&bad_rows));
    assert!(
        bad_issues
            .iter()
            .any(|issue| issue.contains("missing required inventory row")),
        "missing inventory row not rejected: {bad_issues:?}"
    );

    let invalid_mutability = valid_inventory.replace("hot\tconsumer", "sometimes\tconsumer");
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
    assert!(module_issues.is_empty(), "valid module failed: {module_issues:?}");

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

    let bad_policy = valid_module.replace("(damage-linear ctx arrow-base-damage arrow-velocity-multiplier arrow-max-damage)", "42");
    let module_issues = validate_steel_module(&parse_steel_module(&bad_policy));
    assert!(
        module_issues
            .iter()
            .any(|issue| issue.contains("arrow-damage policy")),
        "bad arrow policy not rejected: {module_issues:?}"
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

fn row(config_path: &str, mutability: &str) -> String {
    format!("{config_path}\tagent\tsource\tdefault\ttype\tcontract\t{mutability}\tconsumer\tcontracted\tevidence")
}

fn required_module_text() -> String {
    let mut text = String::new();
    for export in REQUIRED_STEEL_EXPORTS {
        let value = match *export {
            "config-version" => "1".to_string(),
            "sandbox-profile" => format!("\"{SANDBOX_PROFILE}\""),
            _ if export.contains("damage") || export.contains("multiplier") => "1.0".to_string(),
            _ if export.contains("protocol") || export.contains("port") || export.contains("timeout") => "1".to_string(),
            _ => "\"value\"".to_string(),
        };
        text.push_str(&format!("(define {export} {value})\n"));
    }
    text.push_str("(define (arrow-damage ctx)\n");
    text.push_str("  (damage-linear ctx arrow-base-damage arrow-velocity-multiplier arrow-max-damage))\n");
    text
}
