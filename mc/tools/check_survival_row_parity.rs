use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

const SELF_TEST_FLAG: &str = "--self-test";
const ROW_FLAG: &str = "--row";
const PAPER_FLAG: &str = "--paper";
const VALENCE_FLAG: &str = "--valence";
const SUCCESS: ExitCode = ExitCode::SUCCESS;
const FAILURE: ExitCode = ExitCode::FAILURE;
const PAPER_BACKEND: &str = "paper";
const VALENCE_BACKEND: &str = "valence";
const CLEAN_REVISION_STATUS: &str = "clean";
const KEY_VALUE_SEPARATOR: char = '=';
const REQUIRED_ARGUMENT_COUNT: usize = 7;
const PROGRAM_ARGUMENT_COUNT: usize = 1;
const SURVIVAL_BLOCK_ENTITY_ACTOR: &str = "compatbot";
const SURVIVAL_BLOCK_ENTITY_KIND: &str = "Sign";
const SURVIVAL_BLOCK_ENTITY_POSITION: &str = "28,64,0";
const SURVIVAL_BLOCK_ENTITY_TEXT_PAYLOAD: &str = "MC|Compat|Sign|Persist";
const SURVIVAL_BLOCK_ENTITY_PRE_RESTART_OBSERVATION: &str = "sign_text_visible";
const SURVIVAL_BLOCK_ENTITY_CLEAN_SHUTDOWN: &str = "graceful";
const SURVIVAL_BLOCK_ENTITY_BACKEND_RESTART: &str = "controlled_reload";
const SURVIVAL_BLOCK_ENTITY_RECONNECT: &str = "restart";
const SURVIVAL_BLOCK_ENTITY_POST_RESTART_OBSERVATION: &str = "sign_text_visible";
const SURVIVAL_BLOCK_ENTITY_SERVER_STATE: &str = "persisted";

const ROWS: &[RowContract] = &[
    RowContract {
        id: "survival-furnace-persistence",
        label: "furnace persistence",
        metrics: &[
            "furnace_open",
            "input_insert",
            "fuel_insert",
            "burn_progress_start",
            "output_available",
            "output_collect",
            "reconnect_reopen",
            "server_state",
        ],
        expected_metrics: &[],
    },
    RowContract {
        id: "survival-hunger-food",
        label: "hunger/food",
        metrics: &[
            "pre_consume_food",
            "consume_start",
            "consume_finish",
            "item_decrement",
            "post_consume_food",
            "saturation_update",
            "server_food_state",
        ],
        expected_metrics: &[],
    },
    RowContract {
        id: "survival-mob-drop",
        label: "mob drops",
        metrics: &[
            "mob_spawn",
            "client_attack",
            "server_death",
            "drop_spawn",
            "pickup",
            "inventory_increment",
            "server_drop_state",
        ],
        expected_metrics: &[],
    },
    RowContract {
        id: "survival-redstone-toggle",
        label: "redstone",
        metrics: &[
            "input_interaction",
            "powered_on",
            "client_state_update",
            "powered_off",
            "server_power_state",
        ],
        expected_metrics: &[],
    },
    RowContract {
        id: "survival-biome-dimension-state",
        label: "biome/dimension",
        metrics: &[
            "spawn_environment",
            "environment_identifier",
            "client_environment_update",
            "server_environment_state",
            "normalized_identifier",
        ],
        expected_metrics: &[],
    },
    RowContract {
        id: "survival-world-persistence-restart",
        label: "world persistence",
        metrics: &[
            "pre_restart_mutation",
            "clean_shutdown",
            "backend_restart",
            "reconnect",
            "post_restart_observation",
            "server_persistence_state",
        ],
        expected_metrics: &[],
    },
    RowContract {
        id: "survival-crash-recovery-parity",
        label: "crash recovery",
        metrics: &[
            "pre_crash_mutation",
            "crash_stop",
            "backend_restart",
            "reconnect",
            "post_crash_observation",
            "server_recovery_state",
        ],
        expected_metrics: &[],
    },
    RowContract {
        id: "survival-block-entity-persistence-parity",
        label: "block-entity persistence",
        metrics: &[
            "actor",
            "block_entity_kind",
            "position",
            "text_payload",
            "pre_restart_observation",
            "clean_shutdown",
            "backend_restart",
            "reconnect",
            "post_restart_observation",
            "server_persistence_state",
        ],
        expected_metrics: &[
            ("actor", SURVIVAL_BLOCK_ENTITY_ACTOR),
            ("block_entity_kind", SURVIVAL_BLOCK_ENTITY_KIND),
            ("position", SURVIVAL_BLOCK_ENTITY_POSITION),
            ("text_payload", SURVIVAL_BLOCK_ENTITY_TEXT_PAYLOAD),
            (
                "pre_restart_observation",
                SURVIVAL_BLOCK_ENTITY_PRE_RESTART_OBSERVATION,
            ),
            ("clean_shutdown", SURVIVAL_BLOCK_ENTITY_CLEAN_SHUTDOWN),
            ("backend_restart", SURVIVAL_BLOCK_ENTITY_BACKEND_RESTART),
            ("reconnect", SURVIVAL_BLOCK_ENTITY_RECONNECT),
            (
                "post_restart_observation",
                SURVIVAL_BLOCK_ENTITY_POST_RESTART_OBSERVATION,
            ),
            (
                "server_persistence_state",
                SURVIVAL_BLOCK_ENTITY_SERVER_STATE,
            ),
        ],
    },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RowContract {
    id: &'static str,
    label: &'static str,
    metrics: &'static [&'static str],
    expected_metrics: &'static [(&'static str, &'static str)],
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct EvidenceDoc {
    row: String,
    backend: String,
    revision_status: String,
    child_revision: String,
    metrics: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CliConfig {
    row: String,
    paper_path: String,
    valence_path: String,
}

fn main() -> ExitCode {
    let args = env::args().collect::<Vec<_>>();
    if args.iter().any(|arg| arg == SELF_TEST_FLAG) {
        return match run_self_tests() {
            Ok(summary) => {
                println!("survival row parity self-test ok: {summary}");
                SUCCESS
            }
            Err(errors) => {
                print_errors(&errors);
                FAILURE
            }
        };
    }

    match parse_cli(&args)
        .and_then(|config| run_config(&config))
        .map(|contract| format!("{} row check passed", contract.label))
    {
        Ok(summary) => {
            println!("{summary}");
            SUCCESS
        }
        Err(errors) => {
            print_errors(&errors);
            FAILURE
        }
    }
}

fn print_errors(errors: &[String]) {
    for error in errors {
        eprintln!("survival row parity check failed: {error}");
    }
}

fn parse_cli(args: &[String]) -> Result<CliConfig, Vec<String>> {
    if args.len() != REQUIRED_ARGUMENT_COUNT {
        return Err(vec![usage()]);
    }

    let mut row = None;
    let mut paper_path = None;
    let mut valence_path = None;
    let mut index = PROGRAM_ARGUMENT_COUNT;
    while index < args.len() {
        let flag = args[index].as_str();
        let Some(value) = args.get(index + PROGRAM_ARGUMENT_COUNT) else {
            return Err(vec![usage()]);
        };
        match flag {
            ROW_FLAG => row = Some(value.clone()),
            PAPER_FLAG => paper_path = Some(value.clone()),
            VALENCE_FLAG => valence_path = Some(value.clone()),
            _ => return Err(vec![format!("unknown argument: {flag}")]),
        }
        index += 2;
    }

    Ok(CliConfig {
        row: row.ok_or_else(|| vec![usage()])?,
        paper_path: paper_path.ok_or_else(|| vec![usage()])?,
        valence_path: valence_path.ok_or_else(|| vec![usage()])?,
    })
}

fn usage() -> String {
    format!("usage: check_survival_row_parity {ROW_FLAG} <row-id> {PAPER_FLAG} <paper-evidence> {VALENCE_FLAG} <valence-evidence>")
}

fn run_config(config: &CliConfig) -> Result<RowContract, Vec<String>> {
    let contract = row_contract(&config.row)?;
    let paper_text = read_file(&config.paper_path)?;
    let valence_text = read_file(&config.valence_path)?;
    validate_pair(contract, &paper_text, &valence_text)?;
    Ok(contract)
}

fn read_file(path: &str) -> Result<String, Vec<String>> {
    fs::read_to_string(Path::new(path)).map_err(|error| vec![format!("{path}: {error}")])
}

fn row_contract(row: &str) -> Result<RowContract, Vec<String>> {
    ROWS.iter()
        .copied()
        .find(|contract| contract.id == row)
        .ok_or_else(|| vec![format!("unknown survival row: {row}")])
}

fn validate_pair(
    contract: RowContract,
    paper_text: &str,
    valence_text: &str,
) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();
    let paper = parse_evidence(paper_text, &mut errors);
    let valence = parse_evidence(valence_text, &mut errors);

    if let Some(paper) = &paper {
        validate_document(contract, paper, PAPER_BACKEND, &mut errors);
    }
    if let Some(valence) = &valence {
        validate_document(contract, valence, VALENCE_BACKEND, &mut errors);
    }
    if let (Some(paper), Some(valence)) = (&paper, &valence) {
        validate_metric_agreement(contract, paper, valence, &mut errors);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn parse_evidence(text: &str, errors: &mut Vec<String>) -> Option<EvidenceDoc> {
    let mut fields = BTreeMap::new();
    let mut metrics = BTreeMap::new();
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let Some((key, value)) = trimmed.split_once(KEY_VALUE_SEPARATOR) else {
            errors.push(format!("expected key=value line: {trimmed}"));
            continue;
        };
        let key = key.trim();
        let value = value.trim();
        if let Some(metric_name) = key.strip_prefix("metric.") {
            metrics.insert(metric_name.to_string(), value.to_string());
        } else {
            fields.insert(key.to_string(), value.to_string());
        }
    }

    Some(EvidenceDoc {
        row: required_field(&fields, "row", errors)?,
        backend: required_field(&fields, "backend", errors)?,
        revision_status: required_field(&fields, "revision_status", errors)?,
        child_revision: required_field(&fields, "child_revision", errors)?,
        metrics,
    })
}

fn required_field(
    fields: &BTreeMap<String, String>,
    key: &str,
    errors: &mut Vec<String>,
) -> Option<String> {
    match fields.get(key) {
        Some(value) if !value.is_empty() => Some(value.clone()),
        _ => {
            errors.push(format!("missing required field: {key}"));
            None
        }
    }
}

fn validate_document(
    contract: RowContract,
    evidence: &EvidenceDoc,
    expected_backend: &str,
    errors: &mut Vec<String>,
) {
    if evidence.row != contract.id {
        errors.push(format!(
            "{} evidence has wrong row: {}",
            expected_backend, evidence.row
        ));
    }
    if evidence.backend != expected_backend {
        errors.push(format!(
            "expected {expected_backend} backend evidence, got {}",
            evidence.backend
        ));
    }
    if evidence.revision_status != CLEAN_REVISION_STATUS {
        errors.push(format!(
            "{} evidence has stale revision status: {}",
            expected_backend, evidence.revision_status
        ));
    }
    if evidence.child_revision == "unknown" || evidence.child_revision == "dirty" {
        errors.push(format!(
            "{} evidence lacks committed child revision metadata or oracle checkpoint",
            expected_backend
        ));
    }
    for metric in contract.metrics {
        if !evidence.metrics.contains_key(*metric) {
            errors.push(format!(
                "{} evidence missing metric for {}: {metric}",
                expected_backend, contract.label
            ));
        }
    }
    for (metric, expected) in contract.expected_metrics {
        if let Some(observed) = evidence.metrics.get(*metric) {
            if observed != expected {
                errors.push(format!(
                    "{} evidence has unexpected metric value for {metric}: expected {expected} got {observed}",
                    expected_backend
                ));
            }
        }
    }
}

fn validate_metric_agreement(
    contract: RowContract,
    paper: &EvidenceDoc,
    valence: &EvidenceDoc,
    errors: &mut Vec<String>,
) {
    for metric in contract.metrics {
        let paper_value = paper.metrics.get(*metric);
        let valence_value = valence.metrics.get(*metric);
        if let (Some(paper_value), Some(valence_value)) = (paper_value, valence_value) {
            if paper_value != valence_value {
                errors.push(format!(
                    "{} metric mismatch for {metric}: paper={paper_value} valence={valence_value}",
                    contract.label
                ));
            }
        }
    }
}

fn run_self_tests() -> Result<String, Vec<String>> {
    let mut exercised_rows = BTreeSet::new();
    for contract in ROWS {
        let paper = fixture_evidence(*contract, PAPER_BACKEND);
        let valence = fixture_evidence(*contract, VALENCE_BACKEND);
        validate_pair(*contract, &paper, &valence)?;
        exercised_rows.insert(contract.id);

        let first_metric = contract.metrics[0];
        let first_metric_value = fixture_metric_value(*contract, first_metric);
        let missing_metric =
            paper.replace(&format!("metric.{first_metric}={first_metric_value}\n"), "");
        assert_contains(
            &validate_pair(*contract, &missing_metric, &valence)
                .expect_err("missing metric fixture should fail"),
            "missing metric",
        )?;

        let mismatch = valence.replace(
            &format!("metric.{first_metric}={first_metric_value}\n"),
            &format!("metric.{first_metric}=different\n"),
        );
        assert_contains(
            &validate_pair(*contract, &paper, &mismatch)
                .expect_err("mismatched metric fixture should fail"),
            "metric mismatch",
        )?;

        let stale_revision = valence.replace("revision_status=clean", "revision_status=dirty");
        assert_contains(
            &validate_pair(*contract, &paper, &stale_revision)
                .expect_err("stale revision fixture should fail"),
            "stale revision",
        )?;

        let unknown_revision = valence.replace("child_revision=abc1234", "child_revision=unknown");
        assert_contains(
            &validate_pair(*contract, &paper, &unknown_revision)
                .expect_err("unknown child revision fixture should fail"),
            "child revision",
        )?;

        let valence_only = validate_pair(*contract, &valence, &valence)
            .expect_err("Valence-only fixture should fail");
        assert_contains(&valence_only, "expected paper backend evidence")?;
    }

    exercise_block_entity_expected_metric_fixtures()?;

    if exercised_rows.len() != ROWS.len() {
        return Err(vec![
            "not every survival row contract was exercised".to_string()
        ]);
    }

    assert_contains(
        &row_contract("unknown-survival-row").expect_err("unknown row should fail"),
        "unknown survival row",
    )?;

    Ok(format!("{} row contracts exercised", exercised_rows.len()))
}

fn exercise_block_entity_expected_metric_fixtures() -> Result<(), Vec<String>> {
    let contract = row_contract("survival-block-entity-persistence-parity")?;
    let paper = fixture_evidence(contract, PAPER_BACKEND);
    let valence = fixture_evidence(contract, VALENCE_BACKEND);

    let wrong_kind = fixture_with_metric_value(
        &paper,
        "block_entity_kind",
        SURVIVAL_BLOCK_ENTITY_KIND,
        "Chest",
    );
    assert_contains(
        &validate_pair(contract, &wrong_kind, &valence)
            .expect_err("wrong sign kind fixture should fail"),
        "unexpected metric value for block_entity_kind",
    )?;

    let wrong_position = fixture_with_metric_value(
        &paper,
        "position",
        SURVIVAL_BLOCK_ENTITY_POSITION,
        "29,64,0",
    );
    assert_contains(
        &validate_pair(contract, &wrong_position, &valence)
            .expect_err("wrong sign position fixture should fail"),
        "unexpected metric value for position",
    )?;

    let wrong_text = fixture_with_metric_value(
        &paper,
        "text_payload",
        SURVIVAL_BLOCK_ENTITY_TEXT_PAYLOAD,
        "MC|Compat|Wrong|Persist",
    );
    assert_contains(
        &validate_pair(contract, &wrong_text, &valence)
            .expect_err("wrong sign text fixture should fail"),
        "unexpected metric value for text_payload",
    )?;

    Ok(())
}

fn fixture_evidence(contract: RowContract, backend: &str) -> String {
    let mut text = format!(
        "row={}\nbackend={backend}\nrevision_status=clean\nchild_revision=abc1234\n",
        contract.id
    );
    for metric in contract.metrics {
        let value = fixture_metric_value(contract, metric);
        text.push_str(&format!("metric.{metric}={value}\n"));
    }
    text
}

fn fixture_metric_value(contract: RowContract, metric: &str) -> &'static str {
    contract
        .expected_metrics
        .iter()
        .find_map(|(expected_metric, expected_value)| {
            if *expected_metric == metric {
                Some(*expected_value)
            } else {
                None
            }
        })
        .unwrap_or("ok")
}

fn fixture_with_metric_value(
    fixture: &str,
    metric: &str,
    old_value: &str,
    new_value: &str,
) -> String {
    fixture.replace(
        &format!("metric.{metric}={old_value}\n"),
        &format!("metric.{metric}={new_value}\n"),
    )
}

fn assert_contains(errors: &[String], needle: &str) -> Result<(), Vec<String>> {
    if errors.iter().any(|error| error.contains(needle)) {
        Ok(())
    } else {
        Err(vec![format!(
            "missing expected diagnostic {needle:?}: {errors:?}"
        )])
    }
}
