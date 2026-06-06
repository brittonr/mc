use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::process;

const ROW_ID_KEY: &str = "row.id";
const ROW_ID: &str = "inventory-stack-split-merge";
const ACTOR_KEY: &str = "contract.actor";
const ACTOR: &str = "compatbot";
const ITEM_KEY: &str = "contract.item";
const ITEM: &str = "RedWool";
const SOURCE_SLOT_KEY: &str = "contract.source_slot";
const DESTINATION_SLOT_KEY: &str = "contract.destination_slot";
const SOURCE_SLOT: &str = "37";
const DESTINATION_SLOT: &str = "38";
const INITIAL_COUNT_KEY: &str = "contract.initial_count";
const SPLIT_COUNT_KEY: &str = "contract.split_count";
const FINAL_SOURCE_COUNT_KEY: &str = "contract.final_source_count";
const FINAL_DESTINATION_COUNT_KEY: &str = "contract.final_destination_count";
const INITIAL_COUNT: &str = "64";
const SPLIT_COUNT: &str = "32";
const FINAL_SOURCE_COUNT: &str = "64";
const FINAL_DESTINATION_COUNT: &str = "0";
const OK_VALUE: &str = "ok";
const CLEAN_STATUS: &str = "clean";
const DRY_RUN_REV: &str = "dry-run";
const UNKNOWN_REV: &str = "unknown";
const STEVENARELLA_REV_KEY: &str = "child.stevenarella.rev";
const STEVENARELLA_STATUS_KEY: &str = "child.stevenarella.status";
const VALENCE_REV_KEY: &str = "child.valence.rev";
const VALENCE_STATUS_KEY: &str = "child.valence.status";
const CLIENT_SEQUENCE_KEY: &str = "state_id.client_sequence";
const SERVER_SEQUENCE_KEY: &str = "state_id.server_sequence";
const STATE_ID_SEQUENCE_LENGTH: usize = 4;
const MIN_STATE_ID: i32 = 0;
const REQUIRED_NONCLAIMS: &[&str] = &[
    "nonclaim.drag_transactions",
    "nonclaim.creative_inventory",
    "nonclaim.all_windows_click_modes",
    "nonclaim.full_inventory_semantics",
    "nonclaim.full_protocol_compatibility",
    "nonclaim.production_readiness",
];
const REQUIRED_OK_METRICS: &[&str] = &[
    "metric.initial_slot_item_count",
    "metric.split_action",
    "metric.carried_stack_count",
    "metric.destination_slot_count",
    "metric.merge_action",
    "metric.final_slot_counts",
    "metric.state_id_sequence",
    "metric.server_click_slot_correlation",
    "server.click_slot.split",
    "server.click_slot.merge",
];
const BROAD_OVERCLAIM_KEYS: &[&str] = &[
    "claim.drag_transactions",
    "claim.creative_inventory",
    "claim.all_windows_click_modes",
    "claim.full_inventory_semantics",
    "claim.full_protocol_compatibility",
    "claim.production_readiness",
    "claim.production_ready",
];
const TRUTHY_OVERCLAIM_VALUES: &[&str] = &["true", "yes", "ok", "claimed", "1"];

#[derive(Debug, Clone)]
struct Evidence {
    values: BTreeMap<String, String>,
}

impl Evidence {
    fn parse(text: &str) -> Result<Self, String> {
        let mut values = BTreeMap::new();
        for (index, raw_line) in text.lines().enumerate() {
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let Some((key, value)) = line.split_once('=') else {
                return Err(format!("line {} is not key=value", index + 1));
            };
            let key = key.trim();
            let value = value.trim();
            if key.is_empty() {
                return Err(format!("line {} has empty key", index + 1));
            }
            if values.insert(key.to_string(), value.to_string()).is_some() {
                return Err(format!("duplicate key {key}"));
            }
        }
        Ok(Self { values })
    }

    fn value(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(String::as_str)
    }
}

fn require_exact(evidence: &Evidence, diagnostics: &mut Vec<String>, key: &str, expected: &str) {
    match evidence.value(key) {
        Some(actual) if actual == expected => {}
        Some(actual) => diagnostics.push(format!("{key} expected {expected}, got {actual}")),
        None => diagnostics.push(format!("missing {key}")),
    }
}

fn require_ok(evidence: &Evidence, diagnostics: &mut Vec<String>, key: &str) {
    require_exact(evidence, diagnostics, key, OK_VALUE);
}

fn require_clean_child_revision(
    evidence: &Evidence,
    diagnostics: &mut Vec<String>,
    rev_key: &str,
    status_key: &str,
) {
    match evidence.value(rev_key) {
        Some(rev) if !rev.is_empty() && rev != DRY_RUN_REV && rev != UNKNOWN_REV => {}
        Some(rev) => diagnostics.push(format!("{rev_key} must be concrete, got {rev}")),
        None => diagnostics.push(format!("missing {rev_key}")),
    }
    require_exact(evidence, diagnostics, status_key, CLEAN_STATUS);
}

fn parse_state_sequence(raw: &str) -> Result<Vec<i32>, String> {
    let mut values = Vec::new();
    for part in raw.split(',') {
        let trimmed = part.trim();
        if trimmed.is_empty() {
            return Err("empty state id".to_string());
        }
        let parsed = trimmed
            .parse::<i32>()
            .map_err(|err| format!("invalid state id {trimmed}: {err}"))?;
        values.push(parsed);
    }
    Ok(values)
}

fn sequence_has_valid_state_ids(values: &[i32]) -> bool {
    values.iter().all(|value| *value >= MIN_STATE_ID)
}

fn require_state_sequence(
    evidence: &Evidence,
    diagnostics: &mut Vec<String>,
    key: &str,
) -> Option<Vec<i32>> {
    match evidence.value(key) {
        Some(raw) => match parse_state_sequence(raw) {
            Ok(values)
                if values.len() == STATE_ID_SEQUENCE_LENGTH
                    && sequence_has_valid_state_ids(&values) =>
            {
                Some(values)
            }
            Ok(values) => {
                diagnostics.push(format!(
                    "{key} must contain {STATE_ID_SEQUENCE_LENGTH} non-negative ids, got {:?}",
                    values
                ));
                None
            }
            Err(err) => {
                diagnostics.push(format!("{key} invalid: {err}"));
                None
            }
        },
        None => {
            diagnostics.push(format!("missing {key}"));
            None
        }
    }
}

fn reject_broad_overclaims(evidence: &Evidence, diagnostics: &mut Vec<String>) {
    for key in BROAD_OVERCLAIM_KEYS {
        if let Some(value) = evidence.value(key) {
            if TRUTHY_OVERCLAIM_VALUES
                .iter()
                .any(|truthy| value.eq_ignore_ascii_case(truthy))
            {
                diagnostics.push(format!("broad overclaim {key}={value}"));
            }
        }
    }
}

fn validate_evidence(evidence: &Evidence) -> Result<(), Vec<String>> {
    let mut diagnostics = Vec::new();
    require_exact(evidence, &mut diagnostics, ROW_ID_KEY, ROW_ID);
    require_clean_child_revision(
        evidence,
        &mut diagnostics,
        STEVENARELLA_REV_KEY,
        STEVENARELLA_STATUS_KEY,
    );
    require_clean_child_revision(
        evidence,
        &mut diagnostics,
        VALENCE_REV_KEY,
        VALENCE_STATUS_KEY,
    );
    require_exact(evidence, &mut diagnostics, ACTOR_KEY, ACTOR);
    require_exact(evidence, &mut diagnostics, ITEM_KEY, ITEM);
    require_exact(evidence, &mut diagnostics, SOURCE_SLOT_KEY, SOURCE_SLOT);
    require_exact(
        evidence,
        &mut diagnostics,
        DESTINATION_SLOT_KEY,
        DESTINATION_SLOT,
    );
    require_exact(evidence, &mut diagnostics, INITIAL_COUNT_KEY, INITIAL_COUNT);
    require_exact(evidence, &mut diagnostics, SPLIT_COUNT_KEY, SPLIT_COUNT);
    require_exact(
        evidence,
        &mut diagnostics,
        FINAL_SOURCE_COUNT_KEY,
        FINAL_SOURCE_COUNT,
    );
    require_exact(
        evidence,
        &mut diagnostics,
        FINAL_DESTINATION_COUNT_KEY,
        FINAL_DESTINATION_COUNT,
    );
    for key in REQUIRED_OK_METRICS {
        require_ok(evidence, &mut diagnostics, key);
    }
    let client_sequence = require_state_sequence(evidence, &mut diagnostics, CLIENT_SEQUENCE_KEY);
    let server_sequence = require_state_sequence(evidence, &mut diagnostics, SERVER_SEQUENCE_KEY);
    if let (Some(client), Some(server)) = (client_sequence, server_sequence) {
        if client != server {
            diagnostics.push(format!(
                "state id sequences differ: client={:?} server={:?}",
                client, server
            ));
        }
    }
    for key in REQUIRED_NONCLAIMS {
        require_exact(evidence, &mut diagnostics, key, "true");
    }
    reject_broad_overclaims(evidence, &mut diagnostics);

    if diagnostics.is_empty() {
        Ok(())
    } else {
        Err(diagnostics)
    }
}

fn valid_fixture() -> String {
    [
        "row.id=inventory-stack-split-merge",
        "child.stevenarella.rev=0123456789abcdef0123456789abcdef01234567",
        "child.stevenarella.status=clean",
        "child.valence.rev=89abcdef0123456789abcdef0123456789abcdef",
        "child.valence.status=clean",
        "contract.actor=compatbot",
        "contract.item=RedWool",
        "contract.source_slot=37",
        "contract.destination_slot=38",
        "contract.initial_count=64",
        "contract.split_count=32",
        "contract.final_source_count=64",
        "contract.final_destination_count=0",
        "metric.initial_slot_item_count=ok",
        "metric.split_action=ok",
        "metric.carried_stack_count=ok",
        "metric.destination_slot_count=ok",
        "metric.merge_action=ok",
        "metric.final_slot_counts=ok",
        "metric.state_id_sequence=ok",
        "metric.server_click_slot_correlation=ok",
        "server.click_slot.split=ok",
        "server.click_slot.merge=ok",
        "state_id.client_sequence=1,1,1,1",
        "state_id.server_sequence=1,1,1,1",
        "nonclaim.drag_transactions=true",
        "nonclaim.creative_inventory=true",
        "nonclaim.all_windows_click_modes=true",
        "nonclaim.full_inventory_semantics=true",
        "nonclaim.full_protocol_compatibility=true",
        "nonclaim.production_readiness=true",
    ]
    .join("\n")
}

fn fixture_with_replacement(old: &str, new: &str) -> String {
    valid_fixture().replace(old, new)
}

fn run_self_test() -> Result<(), String> {
    let valid = Evidence::parse(&valid_fixture())?;
    validate_evidence(&valid).map_err(|diagnostics| diagnostics.join("; "))?;

    let negative_fixtures: &[(&str, String, &str)] = &[
        (
            "missing row id",
            fixture_with_replacement("row.id=inventory-stack-split-merge\n", ""),
            "missing row.id",
        ),
        (
            "dirty child revision",
            fixture_with_replacement("child.valence.status=clean", "child.valence.status=dirty"),
            "child.valence.status expected clean",
        ),
        (
            "unknown child revision",
            fixture_with_replacement(
                "child.stevenarella.rev=0123456789abcdef0123456789abcdef01234567",
                "child.stevenarella.rev=unknown",
            ),
            "child.stevenarella.rev must be concrete",
        ),
        (
            "wrong split count",
            fixture_with_replacement("contract.split_count=32", "contract.split_count=31"),
            "contract.split_count expected 32",
        ),
        (
            "missing state id sequence",
            fixture_with_replacement("state_id.client_sequence=1,1,1,1\n", ""),
            "missing state_id.client_sequence",
        ),
        (
            "missing server click correlation",
            fixture_with_replacement(
                "server.click_slot.merge=ok",
                "server.click_slot.merge=missing",
            ),
            "server.click_slot.merge expected ok",
        ),
        (
            "broad overclaim",
            format!("{}\nclaim.full_inventory_semantics=true", valid_fixture()),
            "broad overclaim claim.full_inventory_semantics=true",
        ),
    ];

    for (name, text, expected) in negative_fixtures {
        let evidence = Evidence::parse(text).map_err(|err| format!("{name}: parse: {err}"))?;
        match validate_evidence(&evidence) {
            Ok(()) => return Err(format!("{name}: unexpectedly passed")),
            Err(diagnostics) => {
                let rendered = diagnostics.join("; ");
                if !rendered.contains(expected) {
                    return Err(format!(
                        "{name}: expected diagnostic containing {expected:?}, got {rendered}"
                    ));
                }
            }
        }
    }

    Ok(())
}

fn run_path(path: &str) -> Result<(), String> {
    let text = fs::read_to_string(path).map_err(|err| format!("read {path}: {err}"))?;
    let evidence = Evidence::parse(&text)?;
    validate_evidence(&evidence).map_err(|diagnostics| diagnostics.join("\n"))
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let result = if args.iter().any(|arg| arg == "--self-test") {
        run_self_test().map(|()| "self-test ok".to_string())
    } else if let Some(path) = args.first() {
        run_path(path).map(|()| format!("{path}: ok"))
    } else {
        Err(
            "usage: check_inventory_stack_split_merge_evidence (--self-test | <evidence.kv>)"
                .to_string(),
        )
    };

    match result {
        Ok(message) => println!("{message}"),
        Err(message) => {
            eprintln!("{message}");
            process::exit(1);
        }
    }
}
