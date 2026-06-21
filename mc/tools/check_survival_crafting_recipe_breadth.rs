#!/usr/bin/env -S nix shell "github:nix-community/fenix?rev=092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain" nixpkgs#gcc -c cargo -q -Zscript

use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

const SELF_TEST_FLAG: &str = "--self-test";
const PAPER_FLAG: &str = "--paper";
const VALENCE_FLAG: &str = "--valence";
const PROGRAM_ARGUMENT_COUNT: usize = 1;
const FLAG_STRIDE: usize = 2;
const ARGUMENT_VALUE_OFFSET: usize = 1;
const PAIR_FLAG_COUNT: usize = 2;
const REQUIRED_ARGUMENT_COUNT: usize = PROGRAM_ARGUMENT_COUNT + (PAIR_FLAG_COUNT * FLAG_STRIDE);
const KEY_VALUE_SEPARATOR: char = '=';
const METRIC_PREFIX: &str = "metric.";
const CLAIM_PREFIX: &str = "claim.";
const TRUE_VALUE: &str = "true";
const CLEAN_REVISION_STATUS: &str = "clean";
const UNKNOWN_REVISION: &str = "unknown";
const DIRTY_REVISION: &str = "dirty";
const PAPER_BACKEND: &str = "paper";
const VALENCE_BACKEND: &str = "valence";
const ROW_ID: &str = "survival-crafting-recipe-breadth-parity";
const SUCCESS: ExitCode = ExitCode::SUCCESS;
const FAILURE: ExitCode = ExitCode::FAILURE;

const SHAPED_INPUT_SLOTS: &str = "1:OakPlanks:1,2:OakPlanks:1,3:OakPlanks:1,4:OakPlanks:1,6:OakPlanks:1,7:OakPlanks:1,8:OakPlanks:1,9:OakPlanks:1";
const SHAPELESS_INPUT_SLOTS: &str = "1:OakLog:1";
const INVALID_INPUT_SLOTS: &str = "1:OakPlanks:1";

const EXPECTED_METRICS: &[(&str, &str)] = &[
    ("matrix.version", "2026-06-20"),
    ("matrix.shaped.recipe_id", "minecraft:chest"),
    ("matrix.shaped.input_slots", SHAPED_INPUT_SLOTS),
    ("matrix.shaped.result_slot", "0"),
    ("matrix.shaped.result_item", "Chest"),
    ("matrix.shaped.result_count", "1"),
    ("matrix.shaped.collection_mode", "primary_click"),
    ("matrix.shaped.final_inventory_slot", "36"),
    ("matrix.shaped.final_inventory_item", "Chest"),
    ("matrix.shaped.final_inventory_count", "1"),
    ("matrix.shapeless.recipe_id", "minecraft:oak_planks"),
    ("matrix.shapeless.input_slots", SHAPELESS_INPUT_SLOTS),
    ("matrix.shapeless.result_slot", "0"),
    ("matrix.shapeless.result_item", "OakPlanks"),
    ("matrix.shapeless.result_count", "4"),
    ("matrix.shapeless.collection_mode", "primary_click"),
    ("matrix.shapeless.final_inventory_slot", "37"),
    ("matrix.shapeless.final_inventory_item", "OakPlanks"),
    ("matrix.shapeless.final_inventory_count", "4"),
    (
        "matrix.invalid.recipe_id",
        "minecraft:stick_insufficient_input_rejection",
    ),
    ("matrix.invalid.input_slots", INVALID_INPUT_SLOTS),
    ("matrix.invalid.result_slot", "0"),
    ("matrix.invalid.result_item", "None"),
    ("matrix.invalid.result_count", "0"),
    ("matrix.invalid.rejection_outcome", "no_result"),
    ("matrix.collection_modes", "primary_click"),
    ("nonclaim.all_recipes", TRUE_VALUE),
    ("nonclaim.recipe_book_ui", TRUE_VALUE),
    ("nonclaim.arbitrary_collection_modes", TRUE_VALUE),
    ("nonclaim.full_survival_compatibility", TRUE_VALUE),
    ("nonclaim.broad_vanilla_parity", TRUE_VALUE),
];

const REQUIRED_FIELDS: &[&str] = &["row", "backend", "revision_status", "child_revision"];

#[derive(Debug, Clone, PartialEq, Eq)]
struct EvidenceDoc {
    fields: BTreeMap<String, String>,
    metrics: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CliConfig {
    paper_path: String,
    valence_path: String,
}

fn main() -> ExitCode {
    let args = env::args().collect::<Vec<_>>();
    if args.iter().any(|arg| arg == SELF_TEST_FLAG) {
        return match run_self_tests() {
            Ok(summary) => {
                println!("survival crafting recipe breadth self-test ok: {summary}");
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
            println!("survival crafting recipe breadth check passed: {summary}");
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
        eprintln!("survival crafting recipe breadth check failed: {error}");
    }
}

fn parse_cli(args: &[String]) -> Result<CliConfig, Vec<String>> {
    if args.len() != REQUIRED_ARGUMENT_COUNT {
        return Err(vec![usage()]);
    }

    let mut paper_path = None;
    let mut valence_path = None;
    let mut index = PROGRAM_ARGUMENT_COUNT;
    while index < args.len() {
        let flag = args[index].as_str();
        let Some(value) = args.get(index + ARGUMENT_VALUE_OFFSET) else {
            return Err(vec![usage()]);
        };
        match flag {
            PAPER_FLAG => paper_path = Some(value.clone()),
            VALENCE_FLAG => valence_path = Some(value.clone()),
            _ => return Err(vec![format!("unknown argument: {flag}")]),
        }
        index += FLAG_STRIDE;
    }

    Ok(CliConfig {
        paper_path: paper_path.ok_or_else(|| vec![usage()])?,
        valence_path: valence_path.ok_or_else(|| vec![usage()])?,
    })
}

fn usage() -> String {
    format!("usage: check_survival_crafting_recipe_breadth {PAPER_FLAG} <paper-evidence> {VALENCE_FLAG} <valence-evidence>")
}

fn run_config(config: &CliConfig) -> Result<String, Vec<String>> {
    let paper_text = read_file(&config.paper_path)?;
    let valence_text = read_file(&config.valence_path)?;
    validate_pair(&paper_text, &valence_text)?;
    Ok(format!("{} normalized metrics", EXPECTED_METRICS.len()))
}

fn read_file(path: &str) -> Result<String, Vec<String>> {
    fs::read_to_string(Path::new(path)).map_err(|error| vec![format!("{path}: {error}")])
}

fn validate_pair(paper_text: &str, valence_text: &str) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();
    let paper = parse_evidence(paper_text, &mut errors);
    let valence = parse_evidence(valence_text, &mut errors);

    if let Some(paper) = &paper {
        validate_document(paper, PAPER_BACKEND, &mut errors);
    }
    if let Some(valence) = &valence {
        validate_document(valence, VALENCE_BACKEND, &mut errors);
    }
    if let (Some(paper), Some(valence)) = (&paper, &valence) {
        validate_metric_agreement(paper, valence, &mut errors);
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
        if let Some(metric_name) = key.strip_prefix(METRIC_PREFIX) {
            metrics.insert(metric_name.to_string(), value.to_string());
        } else {
            fields.insert(key.to_string(), value.to_string());
        }
    }

    let document = EvidenceDoc { fields, metrics };
    for field in REQUIRED_FIELDS {
        if required_field(&document.fields, field, errors).is_none() {
            return None;
        }
    }
    Some(document)
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

fn validate_document(evidence: &EvidenceDoc, expected_backend: &str, errors: &mut Vec<String>) {
    validate_field_value(evidence, "row", ROW_ID, expected_backend, errors);
    validate_field_value(
        evidence,
        "backend",
        expected_backend,
        expected_backend,
        errors,
    );
    validate_field_value(
        evidence,
        "revision_status",
        CLEAN_REVISION_STATUS,
        expected_backend,
        errors,
    );
    validate_child_revision(evidence, expected_backend, errors);
    validate_broad_claims(evidence, expected_backend, errors);
    for (metric, expected_value) in EXPECTED_METRICS {
        validate_metric_value(evidence, metric, expected_value, expected_backend, errors);
    }
}

fn validate_field_value(
    evidence: &EvidenceDoc,
    field: &str,
    expected_value: &str,
    expected_backend: &str,
    errors: &mut Vec<String>,
) {
    match evidence.fields.get(field) {
        Some(observed) if observed == expected_value => {}
        Some(observed) => errors.push(format!(
            "{expected_backend} evidence {field} expected {expected_value}, got {observed}"
        )),
        None => errors.push(format!("{expected_backend} evidence missing field {field}")),
    }
}

fn validate_child_revision(
    evidence: &EvidenceDoc,
    expected_backend: &str,
    errors: &mut Vec<String>,
) {
    match evidence.fields.get("child_revision").map(String::as_str) {
        Some(UNKNOWN_REVISION | DIRTY_REVISION) => errors.push(format!(
            "{expected_backend} evidence lacks clean child revision metadata"
        )),
        Some(value) if value.is_empty() => errors.push(format!(
            "{expected_backend} evidence has empty child revision metadata"
        )),
        Some(_) => {}
        None => errors.push(format!(
            "{expected_backend} evidence missing child revision metadata"
        )),
    }
}

fn validate_broad_claims(evidence: &EvidenceDoc, expected_backend: &str, errors: &mut Vec<String>) {
    for (key, value) in &evidence.fields {
        if key.starts_with(CLAIM_PREFIX) && value == TRUE_VALUE {
            errors.push(format!(
                "{expected_backend} evidence contains broad overclaim {key}={value}"
            ));
        }
    }
}

fn validate_metric_value(
    evidence: &EvidenceDoc,
    metric: &str,
    expected_value: &str,
    expected_backend: &str,
    errors: &mut Vec<String>,
) {
    match evidence.metrics.get(metric) {
        Some(observed) if observed == expected_value => {}
        Some(observed) => errors.push(format!(
            "{expected_backend} evidence metric {metric} expected {expected_value}, got {observed}"
        )),
        None => errors.push(format!(
            "{expected_backend} evidence missing metric {metric}"
        )),
    }
}

fn validate_metric_agreement(paper: &EvidenceDoc, valence: &EvidenceDoc, errors: &mut Vec<String>) {
    for (metric, _) in EXPECTED_METRICS {
        let paper_value = paper.metrics.get(*metric);
        let valence_value = valence.metrics.get(*metric);
        if let (Some(paper_value), Some(valence_value)) = (paper_value, valence_value) {
            if paper_value != valence_value {
                errors.push(format!(
                    "paired evidence metric mismatch for {metric}: paper={paper_value} valence={valence_value}"
                ));
            }
        }
    }
}

fn run_self_tests() -> Result<String, Vec<String>> {
    let paper = fixture_evidence(PAPER_BACKEND);
    let valence = fixture_evidence(VALENCE_BACKEND);
    validate_pair(&paper, &valence)?;

    assert_contains(
        &validate_pair(&valence, &valence).expect_err("Valence-only fixture should fail"),
        "backend expected paper",
    )?;

    let missing_recipe_id =
        remove_metric(&paper, "matrix.shapeless.recipe_id", "minecraft:oak_planks");
    assert_contains(
        &validate_pair(&missing_recipe_id, &valence)
            .expect_err("missing recipe id fixture should fail"),
        "missing metric matrix.shapeless.recipe_id",
    )?;

    let missing_slot_metric =
        remove_metric(&paper, "matrix.shaped.input_slots", SHAPED_INPUT_SLOTS);
    assert_contains(
        &validate_pair(&missing_slot_metric, &valence)
            .expect_err("missing slot metric fixture should fail"),
        "missing metric matrix.shaped.input_slots",
    )?;

    let missing_result_metric = remove_metric(&paper, "matrix.shaped.result_count", "1");
    assert_contains(
        &validate_pair(&missing_result_metric, &valence)
            .expect_err("missing result metric fixture should fail"),
        "missing metric matrix.shaped.result_count",
    )?;

    let mismatched_count = replace_metric(&valence, "matrix.shaped.result_count", "1", "2");
    assert_contains(
        &validate_pair(&paper, &mismatched_count).expect_err("mismatched count should fail"),
        "metric matrix.shaped.result_count expected 1, got 2",
    )?;

    let stale_revision = valence.replace("revision_status=clean", "revision_status=dirty");
    assert_contains(
        &validate_pair(&paper, &stale_revision).expect_err("stale revision should fail"),
        "revision_status expected clean",
    )?;

    let broad_overclaim = format!("{valence}claim.all_recipes=true\n");
    assert_contains(
        &validate_pair(&paper, &broad_overclaim).expect_err("broad overclaim should fail"),
        "broad overclaim claim.all_recipes=true",
    )?;

    Ok(format!("{} expected metrics", EXPECTED_METRICS.len()))
}

fn fixture_evidence(backend: &str) -> String {
    let mut text =
        format!("row={ROW_ID}\nbackend={backend}\nrevision_status=clean\nchild_revision=abc1234\n");
    for (metric, value) in EXPECTED_METRICS {
        text.push_str(&format!("metric.{metric}={value}\n"));
    }
    text
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
