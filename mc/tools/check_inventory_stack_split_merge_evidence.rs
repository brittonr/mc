mod checker_framework;

use checker_framework::{
    all_at_least, assert_self_test_fixtures, parse_i32_sequence, reject_truthy_overclaims,
    require_clean_child_revision, require_exact, require_ok, run_checker, Checker,
    KeyValueEvidence, KeyValueRecord, ValidationResult, TRUE_VALUE,
};

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

type Evidence = KeyValueRecord;

fn require_state_sequence(
    evidence: &Evidence,
    diagnostics: &mut Vec<String>,
    key: &str,
) -> Option<Vec<i32>> {
    match evidence.value(key) {
        Some(raw) => match parse_i32_sequence(raw) {
            Ok(values)
                if values.len() == STATE_ID_SEQUENCE_LENGTH
                    && all_at_least(&values, MIN_STATE_ID) =>
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

fn validate_evidence(evidence: &Evidence) -> ValidationResult {
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
        require_exact(evidence, &mut diagnostics, key, TRUE_VALUE);
    }
    reject_truthy_overclaims(
        evidence,
        &mut diagnostics,
        BROAD_OVERCLAIM_KEYS,
        TRUTHY_OVERCLAIM_VALUES,
    );

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

    assert_self_test_fixtures(&valid_fixture(), validate_evidence, negative_fixtures)
}

struct StackSplitMergeChecker;

impl Checker for StackSplitMergeChecker {
    fn usage(&self) -> &'static str {
        "usage: check_inventory_stack_split_merge_evidence (--self-test | <evidence.kv>)"
    }

    fn validate(&self, evidence: &KeyValueRecord) -> ValidationResult {
        validate_evidence(evidence)
    }

    fn self_test(&self) -> Result<(), String> {
        run_self_test()
    }
}

fn main() {
    run_checker(&StackSplitMergeChecker);
}
