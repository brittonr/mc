#!/usr/bin/env -S nix shell "github:nix-community/fenix?rev=092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain" nixpkgs#gcc -c cargo -q -Zscript

use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

const SELF_TEST_FLAG: &str = "--self-test";
const ROW_FLAG: &str = "--row";
const PAPER_FLAG: &str = "--paper";
const VALENCE_FLAG: &str = "--valence";
const PROGRAM_ARGUMENT_COUNT: usize = 1;
const FLAG_STRIDE: usize = 2;
const ARGUMENT_VALUE_OFFSET: usize = 1;
const ROW_PAIR_FLAG_COUNT: usize = 3;
const REQUIRED_ARGUMENT_COUNT: usize = PROGRAM_ARGUMENT_COUNT + (ROW_PAIR_FLAG_COUNT * FLAG_STRIDE);
const KEY_VALUE_SEPARATOR: char = '=';
const METRIC_PREFIX: &str = "metric.";
const CLAIM_PREFIX: &str = "claim.";
const TRUE_VALUE: &str = "true";
const CLEAN_REVISION_STATUS: &str = "clean";
const UNKNOWN_REVISION: &str = "unknown";
const DIRTY_REVISION: &str = "dirty";
const PAPER_BACKEND: &str = "paper";
const VALENCE_BACKEND: &str = "valence";
const FIXTURE_REVISION: &str = "abc1234";
const SUCCESS: ExitCode = ExitCode::SUCCESS;
const FAILURE: ExitCode = ExitCode::FAILURE;

const FURNACE_METRICS: &[(&str, &str)] = &[
    ("matrix.version", "2026-06-20"),
    ("smelt.recipe_id", "minecraft:iron_ingot"),
    ("smelt.input_item", "RawIron"),
    ("smelt.fuel_item", "Coal"),
    ("smelt.burn_ticks", "1600"),
    ("smelt.cook_ticks", "200"),
    ("smelt.output_item", "IronIngot"),
    ("smelt.output_count", "1"),
    ("invalid_fuel.item", "RawIron"),
    ("invalid_fuel.outcome", "no_burn"),
    ("nonclaim.all_furnaces", TRUE_VALUE),
];
const HUNGER_METRICS: &[(&str, &str)] = &[
    ("matrix.version", "2026-06-20"),
    ("food.item", "Bread"),
    ("checkpoint.pre_food", "15"),
    ("checkpoint.post_food", "20"),
    ("checkpoint.pre_health", "18.0"),
    ("checkpoint.post_health", "20.0"),
    ("checkpoint.pre_saturation", "0.0"),
    ("checkpoint.post_saturation", "6.0"),
    ("inventory.item_decrement", "1"),
    ("nonclaim.all_hunger", TRUE_VALUE),
];
const MOB_METRICS: &[(&str, &str)] = &[
    ("matrix.version", "2026-06-20"),
    ("mob.identity", "Zombie"),
    ("mob.ai_checkpoint", "approach_player"),
    ("mob.kill_method", "player_attack"),
    ("loot.drop_item", "RottenFlesh"),
    ("loot.drop_count", "1"),
    ("loot.pickup", "observed"),
    ("inventory.increment", "1"),
    ("nonclaim.all_mobs", TRUE_VALUE),
];
const REDSTONE_METRICS: &[(&str, &str)] = &[
    ("matrix.version", "2026-06-20"),
    ("circuit.kind", "lever_lamp_repeater"),
    ("checkpoint.initial_powered", "false"),
    ("checkpoint.after_input_powered", "true"),
    ("checkpoint.after_return_powered", "false"),
    ("tick_sequence", "0:false,2:true,4:false"),
    ("nonclaim.general_redstone", TRUE_VALUE),
];
const BIOME_METRICS: &[(&str, &str)] = &[
    ("matrix.version", "2026-06-20"),
    ("origin.dimension", "minecraft:overworld"),
    ("origin.biome", "minecraft:plains"),
    ("destination.dimension", "minecraft:the_nether"),
    ("destination.biome", "minecraft:nether_wastes"),
    ("transition.kind", "nether_portal"),
    ("transition.client_checkpoint", "dimension_changed"),
    ("transition.server_checkpoint", "environment_changed"),
    ("nonclaim.all_dimensions", TRUE_VALUE),
];
const WORLD_METRICS: &[(&str, &str)] = &[
    ("matrix.version", "2026-06-20"),
    ("chunk.coordinates", "0,0;2,0"),
    ("mutation.primary", "0,64,0:Dirt"),
    ("mutation.secondary", "32,64,0:OakPlanks"),
    ("restart.kind", "controlled_reload"),
    ("post_restart.primary_observed", "present"),
    ("post_restart.secondary_observed", "present"),
    ("auxiliary_marker_only", "false"),
    ("nonclaim.arbitrary_durability", TRUE_VALUE),
];
const CONTAINER_METRICS: &[(&str, &str)] = &[
    ("matrix.version", "2026-06-20"),
    ("container.kind", "Barrel"),
    ("container.position", "34,64,0"),
    ("transfer.item", "Dirt"),
    ("transfer.count", "1"),
    ("payload.summary", "slot0:Dirt:1"),
    ("metadata.summary", "custom_name:MC Compat Barrel"),
    ("reopen.observation", "payload_present"),
    ("nonclaim.all_containers", TRUE_VALUE),
    ("nonclaim.arbitrary_nbt", TRUE_VALUE),
];
const SIGN_METRICS: &[(&str, &str)] = &[
    ("matrix.version", "2026-06-20"),
    ("sign.position", "28,64,0"),
    ("sign.side", "front"),
    ("sign.payload", "MC|Compat|Sign|Edit"),
    ("client.open_milestone", "sign_editor_open_observed"),
    ("client.update_milestone", "sign_update_sent"),
    ("server.acceptance", "sign_update_accepted_observed"),
    ("post_update.observation", "text_visible"),
    ("nonclaim.all_sign_ui", TRUE_VALUE),
];

const ROWS: &[RowContract] = &[
    RowContract {
        id: "survival-furnace-smelting-breadth-parity",
        label: "furnace smelting breadth",
        metrics: FURNACE_METRICS,
        targeted_missing_metric: "invalid_fuel.outcome",
        targeted_mismatch_metric: "smelt.output_count",
        broad_claim_key: "claim.all_furnaces",
    },
    RowContract {
        id: "survival-hunger-health-cycle-parity",
        label: "hunger health cycle",
        metrics: HUNGER_METRICS,
        targeted_missing_metric: "checkpoint.post_saturation",
        targeted_mismatch_metric: "checkpoint.post_health",
        broad_claim_key: "claim.all_hunger",
    },
    RowContract {
        id: "survival-mob-ai-loot-breadth-parity",
        label: "mob AI loot breadth",
        metrics: MOB_METRICS,
        targeted_missing_metric: "mob.identity",
        targeted_mismatch_metric: "loot.drop_count",
        broad_claim_key: "claim.all_mobs",
    },
    RowContract {
        id: "survival-redstone-circuit-breadth-parity",
        label: "redstone circuit breadth",
        metrics: REDSTONE_METRICS,
        targeted_missing_metric: "checkpoint.after_input_powered",
        targeted_mismatch_metric: "tick_sequence",
        broad_claim_key: "claim.general_redstone",
    },
    RowContract {
        id: "survival-biome-dimension-travel-parity",
        label: "biome dimension travel",
        metrics: BIOME_METRICS,
        targeted_missing_metric: "destination.dimension",
        targeted_mismatch_metric: "transition.kind",
        broad_claim_key: "claim.all_dimensions",
    },
    RowContract {
        id: "survival-world-multichunk-durability-parity",
        label: "world multichunk durability",
        metrics: WORLD_METRICS,
        targeted_missing_metric: "chunk.coordinates",
        targeted_mismatch_metric: "post_restart.secondary_observed",
        broad_claim_key: "claim.arbitrary_durability",
    },
    RowContract {
        id: "survival-container-block-entity-breadth-parity",
        label: "container block entity breadth",
        metrics: CONTAINER_METRICS,
        targeted_missing_metric: "container.kind",
        targeted_mismatch_metric: "metadata.summary",
        broad_claim_key: "claim.all_containers",
    },
    RowContract {
        id: "survival-sign-editing-live-parity",
        label: "sign editing live parity",
        metrics: SIGN_METRICS,
        targeted_missing_metric: "client.open_milestone",
        targeted_mismatch_metric: "sign.payload",
        broad_claim_key: "claim.all_sign_ui",
    },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RowContract {
    id: &'static str,
    label: &'static str,
    metrics: &'static [(&'static str, &'static str)],
    targeted_missing_metric: &'static str,
    targeted_mismatch_metric: &'static str,
    broad_claim_key: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct EvidenceDoc {
    row: String,
    backend: String,
    revision_status: String,
    child_revision: String,
    claims: BTreeMap<String, String>,
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
                println!("survival breadth contracts self-test ok: {summary}");
                SUCCESS
            }
            Err(errors) => {
                print_errors(&errors);
                FAILURE
            }
        };
    }

    match parse_cli(&args).and_then(|config| run_config(&config)) {
        Ok(summary) => {
            println!("survival breadth contract check passed: {summary}");
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
        eprintln!("survival breadth contract check failed: {error}");
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
        let Some(value) = args.get(index + ARGUMENT_VALUE_OFFSET) else {
            return Err(vec![usage()]);
        };
        match flag {
            ROW_FLAG => row = Some(value.clone()),
            PAPER_FLAG => paper_path = Some(value.clone()),
            VALENCE_FLAG => valence_path = Some(value.clone()),
            _ => return Err(vec![format!("unknown argument: {flag}")]),
        }
        index += FLAG_STRIDE;
    }

    Ok(CliConfig {
        row: row.ok_or_else(|| vec![usage()])?,
        paper_path: paper_path.ok_or_else(|| vec![usage()])?,
        valence_path: valence_path.ok_or_else(|| vec![usage()])?,
    })
}

fn usage() -> String {
    format!("usage: check_survival_breadth_contracts {ROW_FLAG} <row-id> {PAPER_FLAG} <paper-evidence> {VALENCE_FLAG} <valence-evidence>")
}

fn run_config(config: &CliConfig) -> Result<String, Vec<String>> {
    let contract = row_contract(&config.row)?;
    let paper_text = read_file(&config.paper_path)?;
    let valence_text = read_file(&config.valence_path)?;
    validate_pair(contract, &paper_text, &valence_text)?;
    Ok(format!(
        "{}: {} metrics",
        contract.label,
        contract.metrics.len()
    ))
}

fn read_file(path: &str) -> Result<String, Vec<String>> {
    fs::read_to_string(Path::new(path)).map_err(|error| vec![format!("{path}: {error}")])
}

fn row_contract(row: &str) -> Result<RowContract, Vec<String>> {
    ROWS.iter()
        .copied()
        .find(|contract| contract.id == row)
        .ok_or_else(|| vec![format!("unknown survival breadth row: {row}")])
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
    let mut claims = BTreeMap::new();
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
        if let Some(metric_name) = key.strip_prefix(METRIC_PREFIX) {
            metrics.insert(metric_name.to_string(), value.to_string());
        } else if key.starts_with(CLAIM_PREFIX) {
            claims.insert(key.to_string(), value.to_string());
        } else {
            fields.insert(key.to_string(), value.to_string());
        }
    }

    Some(EvidenceDoc {
        row: required_field(&fields, "row", errors)?,
        backend: required_field(&fields, "backend", errors)?,
        revision_status: required_field(&fields, "revision_status", errors)?,
        child_revision: required_field(&fields, "child_revision", errors)?,
        claims,
        metrics,
    })
}

fn required_field(
    fields: &BTreeMap<String, String>,
    field: &str,
    errors: &mut Vec<String>,
) -> Option<String> {
    match fields.get(field) {
        Some(value) if !value.is_empty() => Some(value.clone()),
        _ => {
            errors.push(format!("missing required field: {field}"));
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
            "{expected_backend} evidence row expected {}, got {}",
            contract.id, evidence.row
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
            "{expected_backend} evidence stale revision status: {}",
            evidence.revision_status
        ));
    }
    if evidence.child_revision == UNKNOWN_REVISION || evidence.child_revision == DIRTY_REVISION {
        errors.push(format!(
            "{expected_backend} evidence lacks clean child revision metadata"
        ));
    }
    validate_claims(contract, evidence, expected_backend, errors);
    for (metric, expected) in contract.metrics {
        validate_metric(evidence, metric, expected, expected_backend, errors);
    }
}

fn validate_claims(
    contract: RowContract,
    evidence: &EvidenceDoc,
    expected_backend: &str,
    errors: &mut Vec<String>,
) {
    for (key, value) in &evidence.claims {
        if value == TRUE_VALUE {
            errors.push(format!(
                "{expected_backend} evidence contains broad overclaim {key}={value}"
            ));
        }
    }
    if evidence.claims.contains_key(contract.broad_claim_key) {
        errors.push(format!(
            "{expected_backend} evidence must keep {} absent",
            contract.broad_claim_key
        ));
    }
}

fn validate_metric(
    evidence: &EvidenceDoc,
    metric: &str,
    expected: &str,
    expected_backend: &str,
    errors: &mut Vec<String>,
) {
    match evidence.metrics.get(metric) {
        Some(observed) if observed == expected => {}
        Some(observed) => errors.push(format!(
            "{expected_backend} evidence metric {metric} expected {expected}, got {observed}"
        )),
        None => errors.push(format!(
            "{expected_backend} evidence missing metric {metric}"
        )),
    }
}

fn validate_metric_agreement(
    contract: RowContract,
    paper: &EvidenceDoc,
    valence: &EvidenceDoc,
    errors: &mut Vec<String>,
) {
    for (metric, _) in contract.metrics {
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
        exercise_row_contract(*contract)?;
        exercised_rows.insert(contract.id);
    }
    if exercised_rows.len() != ROWS.len() {
        return Err(vec![
            "not every survival breadth row was exercised".to_string()
        ]);
    }
    assert_contains(
        &row_contract("unknown-breadth-row").expect_err("unknown row should fail"),
        "unknown survival breadth row",
    )?;
    Ok(format!(
        "{} breadth row contracts exercised",
        exercised_rows.len()
    ))
}

fn exercise_row_contract(contract: RowContract) -> Result<(), Vec<String>> {
    let paper = fixture_evidence(contract, PAPER_BACKEND);
    let valence = fixture_evidence(contract, VALENCE_BACKEND);
    validate_pair(contract, &paper, &valence)?;

    let valence_only =
        validate_pair(contract, &valence, &valence).expect_err("Valence-only fixture should fail");
    assert_contains(&valence_only, "expected paper backend evidence")?;

    let missing_metric = remove_metric(
        &paper,
        contract.targeted_missing_metric,
        metric_value(contract, contract.targeted_missing_metric),
    );
    assert_contains(
        &validate_pair(contract, &missing_metric, &valence)
            .expect_err("missing metric fixture should fail"),
        "missing metric",
    )?;

    let mismatched_metric = replace_metric(
        &valence,
        contract.targeted_mismatch_metric,
        metric_value(contract, contract.targeted_mismatch_metric),
        "different",
    );
    assert_contains(
        &validate_pair(contract, &paper, &mismatched_metric)
            .expect_err("mismatched metric fixture should fail"),
        contract.targeted_mismatch_metric,
    )?;

    let stale_revision = valence.replace("revision_status=clean", "revision_status=dirty");
    assert_contains(
        &validate_pair(contract, &paper, &stale_revision)
            .expect_err("stale revision fixture should fail"),
        "stale revision",
    )?;

    let overclaim = format!("{valence}{}=true\n", contract.broad_claim_key);
    assert_contains(
        &validate_pair(contract, &paper, &overclaim).expect_err("overclaim fixture should fail"),
        contract.broad_claim_key,
    )
}

fn fixture_evidence(contract: RowContract, backend: &str) -> String {
    let mut text = format!(
        "row={}\nbackend={backend}\nrevision_status=clean\nchild_revision={FIXTURE_REVISION}\n",
        contract.id
    );
    for (metric, value) in contract.metrics {
        text.push_str(&format!("metric.{metric}={value}\n"));
    }
    text
}

fn metric_value(contract: RowContract, metric: &str) -> &'static str {
    contract
        .metrics
        .iter()
        .find_map(|(name, value)| if *name == metric { Some(*value) } else { None })
        .unwrap_or("")
}

fn remove_metric(fixture: &str, metric: &str, value: &str) -> String {
    fixture.replace(&format!("metric.{metric}={value}\n"), "")
}

fn replace_metric(fixture: &str, metric: &str, old_value: &str, new_value: &str) -> String {
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
