#!/usr/bin/env -S CARGO_TARGET_DIR=target/check-furnace-smelting-receipt-handoff-script nix shell "github:nix-community/fenix?rev=092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain" nixpkgs#gcc nixpkgs#nickel -c cargo -q -Zscript

use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

const SELF_TEST_FLAG: &str = "--self-test";
const HELP_FLAG: &str = "--help";
const FIXTURE_FLAG: &str = "--fixture";
const PAPER_FLAG: &str = "--paper";
const VALENCE_FLAG: &str = "--valence";
const HELP_TEXT: &str = "usage: check_furnace_smelting_receipt_handoff.rs [--self-test] [--fixture PATH --paper PATH --valence PATH]";
const SUCCESS_MESSAGE: &str = "furnace smelting selected-row receipt handoff check passed";
const SELF_TEST_SUCCESS_MESSAGE: &str =
    "furnace smelting selected-row receipt handoff self-test passed";
const SUCCESS: ExitCode = ExitCode::SUCCESS;
const FAILURE: ExitCode = ExitCode::FAILURE;

const ARGUMENT_STEP: usize = 1;
const KEY_VALUE_SEPARATOR: char = '=';
const METRIC_PREFIX: &str = "metric.";
const CLAIM_PREFIX: &str = "claim.";
const TRUE_VALUE: &str = "true";
const FALSE_VALUE: &str = "false";
const CLEAN_REVISION_STATUS: &str = "clean";
const UNKNOWN_REVISION: &str = "unknown";
const DIRTY_REVISION: &str = "dirty";
const NICKEL_COMMAND: &str = "nickel";
const NICKEL_EXPORT_SUBCOMMAND: &str = "export";

const TARGET_EDITION: &str = "Java Edition";
const TARGET_GAME_VERSION: &str = "1.20.1";
const TARGET_PROTOCOL: u32 = 763;
const STANDARD_FURNACE_KIND: &str = "standard";
const EXPECTED_RECEIPT_ROW: &str = "survival-furnace-smelting-breadth-parity";
const EXPECTED_MATRIX_VERSION: &str = "2026-06-20";
const EXPECTED_FIXTURE_RECIPE_ID: &str = "minecraft:iron_ingot_from_raw_iron_smelting";
const EXPECTED_RECEIPT_RECIPE_ID: &str = "minecraft:iron_ingot";
const RAW_IRON_ITEM: &str = "minecraft:raw_iron";
const COAL_ITEM: &str = "minecraft:coal";
const IRON_INGOT_ITEM: &str = "minecraft:iron_ingot";
const RAW_IRON_RECEIPT_ALIAS: &str = "RawIron";
const COAL_RECEIPT_ALIAS: &str = "Coal";
const IRON_INGOT_RECEIPT_ALIAS: &str = "IronIngot";
const GOLD_INGOT_RECEIPT_ALIAS: &str = "GoldIngot";
const GOLD_INGOT_ITEM: &str = "minecraft:gold_ingot";
const PAPER_BACKEND: &str = "paper";
const VALENCE_BACKEND: &str = "valence";
const FIXTURE_REVISION: &str = "abc1234";
const INVALID_FUEL_OUTCOME: &str = "no_burn";

const SELECTED_RECIPE_INPUT_ITEM_BINDING: &str = "selected_recipe_input_item";
const SELECTED_RECIPE_OUTPUT_ITEM_BINDING: &str = "selected_recipe_output_item";
const SELECTED_FUEL_ITEM_BINDING: &str = "selected_fuel_item";
const SELECTED_RECIPE_OUTPUT_COUNT_BINDING: &str = "selected_recipe_output_count_value";
const SELECTED_STANDARD_FURNACE_COOK_TICKS_BINDING: &str = "selected_standard_furnace_cook_ticks";
const SELECTED_COAL_BURN_TICKS_BINDING: &str = "selected_coal_burn_ticks";
const TARGET_EDITION_BINDING: &str = "target_edition";
const TARGET_GAME_VERSION_BINDING: &str = "target_game_version";
const TARGET_PROTOCOL_BINDING: &str = "target_protocol";
const STANDARD_FURNACE_KIND_BINDING: &str = "standard_furnace_kind";
const RECIPE_ID_FIELD: &str = "recipe_id";

const METRIC_MATRIX_VERSION: &str = "matrix.version";
const METRIC_RECIPE_ID: &str = "smelt.recipe_id";
const METRIC_INPUT_ITEM: &str = "smelt.input_item";
const METRIC_FUEL_ITEM: &str = "smelt.fuel_item";
const METRIC_BURN_TICKS: &str = "smelt.burn_ticks";
const METRIC_COOK_TICKS: &str = "smelt.cook_ticks";
const METRIC_OUTPUT_ITEM: &str = "smelt.output_item";
const METRIC_OUTPUT_COUNT: &str = "smelt.output_count";
const METRIC_INVALID_FUEL_ITEM: &str = "invalid_fuel.item";
const METRIC_INVALID_FUEL_OUTCOME: &str = "invalid_fuel.outcome";
const METRIC_NONCLAIM_ALL_FURNACES: &str = "nonclaim.all_furnaces";
const CLAIM_ALL_FURNACES: &str = "claim.all_furnaces";

const SELECTED_OUTPUT_COUNT: u32 = 1;
const STANDARD_FURNACE_COOK_TICKS: u32 = 200;
const COAL_BURN_TICKS: u32 = 1_600;
const WRONG_OUTPUT_COUNT: &str = "2";
const WRONG_COOK_TICKS: &str = "199";
const WRONG_BURN_TICKS: &str = "1200";

const REQUIRED_FIXTURE_NON_CLAIMS: &[&str] = &[
    "no broad Minecraft compatibility",
    "no broad vanilla parity",
    "no Paper/vanilla parity",
    "no all-recipe breadth",
    "no smoker behavior",
    "no blast-furnace behavior",
    "no hopper automation",
    "no XP behavior",
    "no recipe-book synchronization",
    "no chunk-unload semantics",
    "no Valence runtime integration",
    "no DefaultPlugins membership changes",
    "no public-server safety",
    "no production readiness",
];

const REQUIRED_FALSE_CLAIMS: &[&str] = &[
    "claims_paper_vanilla_parity",
    "claims_all_recipe_breadth",
    "claims_valence_runtime_integration",
    "claims_default_plugin_membership",
    "claims_public_server_safety",
    "claims_production_readiness",
];

const RECEIPT_METRICS_FOR_AGREEMENT: &[&str] = &[
    METRIC_MATRIX_VERSION,
    METRIC_RECIPE_ID,
    METRIC_INPUT_ITEM,
    METRIC_FUEL_ITEM,
    METRIC_BURN_TICKS,
    METRIC_COOK_TICKS,
    METRIC_OUTPUT_ITEM,
    METRIC_OUTPUT_COUNT,
    METRIC_INVALID_FUEL_ITEM,
    METRIC_INVALID_FUEL_OUTCOME,
    METRIC_NONCLAIM_ALL_FURNACES,
];

#[derive(Debug, Clone, PartialEq, Eq)]
enum CommandMode {
    Check {
        fixture_path: PathBuf,
        paper_path: PathBuf,
        valence_path: PathBuf,
    },
    SelfTest,
    Help,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SelectedFixtureRow {
    target_edition: String,
    target_game_version: String,
    target_protocol: u32,
    furnace_kind: String,
    recipe_id: String,
    input_item: String,
    fuel_item: String,
    output_item: String,
    output_count: u32,
    cook_ticks: u32,
    burn_ticks: u32,
    non_claims: BTreeSet<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ReceiptEvidence {
    row: String,
    backend: String,
    revision_status: String,
    child_revision: String,
    claims: BTreeMap<String, String>,
    metrics: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ReceiptHandoffDecision {
    input_item: String,
    fuel_item: String,
    output_item: String,
    output_count: u32,
    cook_ticks: u32,
    burn_ticks: u32,
    paper_child_revision: String,
    valence_child_revision: String,
}

fn main() -> ExitCode {
    match parse_command(env::args().skip(1)) {
        Ok(CommandMode::Help) => {
            println!("{HELP_TEXT}");
            SUCCESS
        }
        Ok(CommandMode::SelfTest) => run_and_report_self_tests(),
        Ok(CommandMode::Check {
            fixture_path,
            paper_path,
            valence_path,
        }) => run_and_report_check(&fixture_path, &paper_path, &valence_path),
        Err(error) => {
            eprintln!("{error}");
            FAILURE
        }
    }
}

fn parse_command(args: impl Iterator<Item = String>) -> Result<CommandMode, String> {
    let args = args.collect::<Vec<_>>();
    let mut self_test = false;
    let mut fixture_path = None;
    let mut paper_path = None;
    let mut valence_path = None;
    let mut index = 0;

    while index < args.len() {
        let arg = &args[index];
        if arg == HELP_FLAG {
            return Ok(CommandMode::Help);
        }
        if arg == SELF_TEST_FLAG {
            self_test = true;
            index += ARGUMENT_STEP;
            continue;
        }
        if arg == FIXTURE_FLAG {
            index += ARGUMENT_STEP;
            fixture_path = Some(required_argument(&args, index, FIXTURE_FLAG)?);
            index += ARGUMENT_STEP;
            continue;
        }
        if arg == PAPER_FLAG {
            index += ARGUMENT_STEP;
            paper_path = Some(required_argument(&args, index, PAPER_FLAG)?);
            index += ARGUMENT_STEP;
            continue;
        }
        if arg == VALENCE_FLAG {
            index += ARGUMENT_STEP;
            valence_path = Some(required_argument(&args, index, VALENCE_FLAG)?);
            index += ARGUMENT_STEP;
            continue;
        }
        return Err(format!("unknown argument: {arg}"));
    }

    if self_test {
        return Ok(CommandMode::SelfTest);
    }

    let Some(fixture_path) = fixture_path else {
        return Err(HELP_TEXT.to_string());
    };
    let Some(paper_path) = paper_path else {
        return Err(HELP_TEXT.to_string());
    };
    let Some(valence_path) = valence_path else {
        return Err(HELP_TEXT.to_string());
    };

    Ok(CommandMode::Check {
        fixture_path,
        paper_path,
        valence_path,
    })
}

fn required_argument(args: &[String], index: usize, flag: &str) -> Result<PathBuf, String> {
    args.get(index)
        .map(PathBuf::from)
        .ok_or_else(|| format!("{flag} requires a path"))
}

fn run_and_report_self_tests() -> ExitCode {
    match run_self_tests() {
        Ok(summary) => {
            println!("{SELF_TEST_SUCCESS_MESSAGE}: {summary}");
            SUCCESS
        }
        Err(errors) => {
            print_errors("self-test", &errors);
            FAILURE
        }
    }
}

fn run_and_report_check(fixture_path: &Path, paper_path: &Path, valence_path: &Path) -> ExitCode {
    match run_check(fixture_path, paper_path, valence_path) {
        Ok(summary) => {
            println!("{SUCCESS_MESSAGE}: {summary}");
            SUCCESS
        }
        Err(errors) => {
            print_errors("check", &errors);
            FAILURE
        }
    }
}

fn print_errors(scope: &str, errors: &[String]) {
    for error in errors {
        eprintln!("furnace smelting selected-row receipt handoff {scope} failed: {error}");
    }
}

fn run_check(
    fixture_path: &Path,
    paper_path: &Path,
    valence_path: &Path,
) -> Result<String, Vec<String>> {
    run_nickel_export(fixture_path)?;
    let fixture_text = read_file(fixture_path)?;
    let paper_text = read_file(paper_path)?;
    let valence_text = read_file(valence_path)?;

    let fixture = parse_selected_fixture(&fixture_text)?;
    let paper = parse_receipt_kv(&paper_text)?;
    let valence = parse_receipt_kv(&valence_text)?;
    let decision = validate_handoff(&fixture, Some(&paper), Some(&valence))?;

    Ok(format!(
        "target={} {} protocol {} row={} input={} fuel={} output={}x{} cook_ticks={} burn_ticks={} paper={} valence={} paper_path={} valence_path={}",
        fixture.target_edition,
        fixture.target_game_version,
        fixture.target_protocol,
        EXPECTED_RECEIPT_ROW,
        decision.input_item,
        decision.fuel_item,
        decision.output_item,
        decision.output_count,
        decision.cook_ticks,
        decision.burn_ticks,
        decision.paper_child_revision,
        decision.valence_child_revision,
        paper_path.display(),
        valence_path.display()
    ))
}

fn read_file(path: &Path) -> Result<String, Vec<String>> {
    fs::read_to_string(path).map_err(|error| vec![format!("{}: {error}", path.display())])
}

fn run_nickel_export(fixture_path: &Path) -> Result<(), Vec<String>> {
    let output = Command::new(NICKEL_COMMAND)
        .arg(NICKEL_EXPORT_SUBCOMMAND)
        .arg(fixture_path)
        .output()
        .map_err(|error| vec![format!("failed to run {NICKEL_COMMAND}: {error}")])?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(vec![format!(
            "{NICKEL_COMMAND} {NICKEL_EXPORT_SUBCOMMAND} {} failed: {stderr}",
            fixture_path.display()
        )])
    }
}

fn parse_selected_fixture(text: &str) -> Result<SelectedFixtureRow, Vec<String>> {
    let mut errors = Vec::new();
    let non_claims = parse_fixture_non_claims(text, &mut errors);
    require_false_claims(text, &mut errors);

    let row = SelectedFixtureRow {
        target_edition: required_string_binding(text, TARGET_EDITION_BINDING, &mut errors),
        target_game_version: required_string_binding(
            text,
            TARGET_GAME_VERSION_BINDING,
            &mut errors,
        ),
        target_protocol: required_number_binding(text, TARGET_PROTOCOL_BINDING, &mut errors),
        furnace_kind: required_string_binding(text, STANDARD_FURNACE_KIND_BINDING, &mut errors),
        recipe_id: required_string_field(text, RECIPE_ID_FIELD, &mut errors),
        input_item: required_string_binding(text, SELECTED_RECIPE_INPUT_ITEM_BINDING, &mut errors),
        fuel_item: required_string_binding(text, SELECTED_FUEL_ITEM_BINDING, &mut errors),
        output_item: required_string_binding(
            text,
            SELECTED_RECIPE_OUTPUT_ITEM_BINDING,
            &mut errors,
        ),
        output_count: required_number_binding(
            text,
            SELECTED_RECIPE_OUTPUT_COUNT_BINDING,
            &mut errors,
        ),
        cook_ticks: required_number_binding(
            text,
            SELECTED_STANDARD_FURNACE_COOK_TICKS_BINDING,
            &mut errors,
        ),
        burn_ticks: required_number_binding(text, SELECTED_COAL_BURN_TICKS_BINDING, &mut errors),
        non_claims,
    };

    validate_fixture_scope(&row, &mut errors);

    if errors.is_empty() {
        Ok(row)
    } else {
        Err(errors)
    }
}

fn parse_fixture_non_claims(text: &str, errors: &mut Vec<String>) -> BTreeSet<String> {
    let mut non_claims = BTreeSet::new();
    for non_claim in REQUIRED_FIXTURE_NON_CLAIMS {
        if text.contains(non_claim) {
            non_claims.insert((*non_claim).to_string());
        } else {
            errors.push(format!("fixture missing required non-claim: {non_claim}"));
        }
    }
    non_claims
}

fn require_false_claims(text: &str, errors: &mut Vec<String>) {
    for claim in REQUIRED_FALSE_CLAIMS {
        let expected = format!("{claim} = {FALSE_VALUE}");
        if !text.contains(&expected) {
            errors.push(format!("fixture missing false claim boundary: {expected}"));
        }
    }
}

fn required_string_binding(text: &str, binding: &str, errors: &mut Vec<String>) -> String {
    let prefix = format!("let {binding} = \"");
    let suffix = "\" in";
    for line in text.lines() {
        let trimmed = line.trim();
        if let Some(after_prefix) = trimmed.strip_prefix(&prefix) {
            if let Some(value) = after_prefix.strip_suffix(suffix) {
                return value.to_string();
            }
        }
    }
    errors.push(format!("fixture missing string binding: {binding}"));
    String::new()
}

fn required_number_binding(text: &str, binding: &str, errors: &mut Vec<String>) -> u32 {
    let prefix = format!("let {binding} = ");
    let suffix = " in";
    for line in text.lines() {
        let trimmed = line.trim();
        if let Some(after_prefix) = trimmed.strip_prefix(&prefix) {
            if let Some(value) = after_prefix.strip_suffix(suffix) {
                return parse_u32(value, &format!("fixture binding {binding}"), errors);
            }
        }
    }
    errors.push(format!("fixture missing number binding: {binding}"));
    u32::default()
}

fn required_string_field(text: &str, field: &str, errors: &mut Vec<String>) -> String {
    let prefix = format!("{field} = \"");
    let suffix = "\",";
    for line in text.lines() {
        let trimmed = line.trim();
        if let Some(after_prefix) = trimmed.strip_prefix(&prefix) {
            if let Some(value) = after_prefix.strip_suffix(suffix) {
                return value.to_string();
            }
        }
    }
    errors.push(format!("fixture missing string field: {field}"));
    String::new()
}

fn parse_receipt_kv(text: &str) -> Result<ReceiptEvidence, Vec<String>> {
    let mut errors = Vec::new();
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

    let evidence = ReceiptEvidence {
        row: required_field(&fields, "row", &mut errors),
        backend: required_field(&fields, "backend", &mut errors),
        revision_status: required_field(&fields, "revision_status", &mut errors),
        child_revision: required_field(&fields, "child_revision", &mut errors),
        claims,
        metrics,
    };

    if errors.is_empty() {
        Ok(evidence)
    } else {
        Err(errors)
    }
}

fn required_field(
    fields: &BTreeMap<String, String>,
    field: &str,
    errors: &mut Vec<String>,
) -> String {
    match fields.get(field) {
        Some(value) if !value.is_empty() => value.clone(),
        _ => {
            errors.push(format!("missing required field: {field}"));
            String::new()
        }
    }
}

fn validate_handoff(
    fixture: &SelectedFixtureRow,
    paper: Option<&ReceiptEvidence>,
    valence: Option<&ReceiptEvidence>,
) -> Result<ReceiptHandoffDecision, Vec<String>> {
    let mut errors = Vec::new();
    validate_fixture_scope(fixture, &mut errors);
    validate_fixture_non_claims(fixture, &mut errors);

    match paper {
        Some(evidence) => validate_receipt(fixture, evidence, PAPER_BACKEND, &mut errors),
        None => errors.push("missing Paper evidence".to_string()),
    }
    match valence {
        Some(evidence) => validate_receipt(fixture, evidence, VALENCE_BACKEND, &mut errors),
        None => errors.push("missing Valence evidence".to_string()),
    }
    if let (Some(paper), Some(valence)) = (paper, valence) {
        validate_metric_agreement(paper, valence, &mut errors);
    }

    if errors.is_empty() {
        let paper_child_revision = paper
            .map(|evidence| evidence.child_revision.clone())
            .unwrap_or_default();
        let valence_child_revision = valence
            .map(|evidence| evidence.child_revision.clone())
            .unwrap_or_default();
        Ok(ReceiptHandoffDecision {
            input_item: fixture.input_item.clone(),
            fuel_item: fixture.fuel_item.clone(),
            output_item: fixture.output_item.clone(),
            output_count: fixture.output_count,
            cook_ticks: fixture.cook_ticks,
            burn_ticks: fixture.burn_ticks,
            paper_child_revision,
            valence_child_revision,
        })
    } else {
        Err(errors)
    }
}

fn validate_fixture_scope(fixture: &SelectedFixtureRow, errors: &mut Vec<String>) {
    validate_text_field(
        "fixture target edition",
        &fixture.target_edition,
        TARGET_EDITION,
        errors,
    );
    validate_text_field(
        "fixture target game version",
        &fixture.target_game_version,
        TARGET_GAME_VERSION,
        errors,
    );
    validate_number_field(
        "fixture target protocol",
        fixture.target_protocol,
        TARGET_PROTOCOL,
        errors,
    );
    validate_text_field(
        "fixture furnace kind",
        &fixture.furnace_kind,
        STANDARD_FURNACE_KIND,
        errors,
    );
    validate_text_field(
        "fixture recipe id",
        &fixture.recipe_id,
        EXPECTED_FIXTURE_RECIPE_ID,
        errors,
    );
    validate_text_field(
        "fixture input item",
        &fixture.input_item,
        RAW_IRON_ITEM,
        errors,
    );
    validate_text_field("fixture fuel item", &fixture.fuel_item, COAL_ITEM, errors);
    validate_text_field(
        "fixture output item",
        &fixture.output_item,
        IRON_INGOT_ITEM,
        errors,
    );
    validate_number_field(
        "fixture output count",
        fixture.output_count,
        SELECTED_OUTPUT_COUNT,
        errors,
    );
    validate_number_field(
        "fixture cook ticks",
        fixture.cook_ticks,
        STANDARD_FURNACE_COOK_TICKS,
        errors,
    );
    validate_number_field(
        "fixture burn ticks",
        fixture.burn_ticks,
        COAL_BURN_TICKS,
        errors,
    );
}

fn validate_fixture_non_claims(fixture: &SelectedFixtureRow, errors: &mut Vec<String>) {
    for non_claim in REQUIRED_FIXTURE_NON_CLAIMS {
        if !fixture.non_claims.contains(*non_claim) {
            errors.push(format!("fixture missing required non-claim: {non_claim}"));
        }
    }
}

fn validate_receipt(
    fixture: &SelectedFixtureRow,
    evidence: &ReceiptEvidence,
    expected_backend: &str,
    errors: &mut Vec<String>,
) {
    validate_text_field(
        &format!("{expected_backend} row"),
        &evidence.row,
        EXPECTED_RECEIPT_ROW,
        errors,
    );
    validate_text_field(
        &format!("{expected_backend} backend"),
        &evidence.backend,
        expected_backend,
        errors,
    );
    validate_text_field(
        &format!("{expected_backend} revision status"),
        &evidence.revision_status,
        CLEAN_REVISION_STATUS,
        errors,
    );
    if evidence.child_revision.is_empty()
        || evidence.child_revision == UNKNOWN_REVISION
        || evidence.child_revision == DIRTY_REVISION
    {
        errors.push(format!(
            "{expected_backend} evidence lacks clean child revision metadata"
        ));
    }
    validate_claims(evidence, expected_backend, errors);
    validate_metric_text(
        evidence,
        METRIC_MATRIX_VERSION,
        EXPECTED_MATRIX_VERSION,
        expected_backend,
        errors,
    );
    validate_metric_text(
        evidence,
        METRIC_RECIPE_ID,
        EXPECTED_RECEIPT_RECIPE_ID,
        expected_backend,
        errors,
    );
    validate_metric_item(
        evidence,
        METRIC_INPUT_ITEM,
        &fixture.input_item,
        expected_backend,
        errors,
    );
    validate_metric_item(
        evidence,
        METRIC_FUEL_ITEM,
        &fixture.fuel_item,
        expected_backend,
        errors,
    );
    validate_metric_item(
        evidence,
        METRIC_OUTPUT_ITEM,
        &fixture.output_item,
        expected_backend,
        errors,
    );
    validate_metric_number(
        evidence,
        METRIC_OUTPUT_COUNT,
        fixture.output_count,
        expected_backend,
        errors,
    );
    validate_metric_number(
        evidence,
        METRIC_COOK_TICKS,
        fixture.cook_ticks,
        expected_backend,
        errors,
    );
    validate_metric_number(
        evidence,
        METRIC_BURN_TICKS,
        fixture.burn_ticks,
        expected_backend,
        errors,
    );
    validate_metric_item(
        evidence,
        METRIC_INVALID_FUEL_ITEM,
        &fixture.input_item,
        expected_backend,
        errors,
    );
    validate_metric_text(
        evidence,
        METRIC_INVALID_FUEL_OUTCOME,
        INVALID_FUEL_OUTCOME,
        expected_backend,
        errors,
    );
    validate_metric_text(
        evidence,
        METRIC_NONCLAIM_ALL_FURNACES,
        TRUE_VALUE,
        expected_backend,
        errors,
    );
}

fn validate_claims(evidence: &ReceiptEvidence, expected_backend: &str, errors: &mut Vec<String>) {
    for (key, value) in &evidence.claims {
        if value == TRUE_VALUE {
            errors.push(format!(
                "{expected_backend} evidence contains broad overclaim {key}={value}"
            ));
        }
    }
    if evidence
        .claims
        .get(CLAIM_ALL_FURNACES)
        .is_some_and(|value| value == TRUE_VALUE)
    {
        errors.push(format!(
            "{expected_backend} evidence must not claim all furnace behavior"
        ));
    }
}

fn validate_metric_text(
    evidence: &ReceiptEvidence,
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

fn validate_metric_item(
    evidence: &ReceiptEvidence,
    metric: &str,
    expected: &str,
    expected_backend: &str,
    errors: &mut Vec<String>,
) {
    match evidence.metrics.get(metric) {
        Some(observed) => match normalize_item_alias(observed) {
            Ok(normalized) if normalized == expected => {}
            Ok(normalized) => errors.push(format!(
                "{expected_backend} evidence metric {metric} expected {expected}, got {normalized} from {observed}"
            )),
            Err(error) => errors.push(format!(
                "{expected_backend} evidence metric {metric} rejected: {error}"
            )),
        },
        None => errors.push(format!(
            "{expected_backend} evidence missing metric {metric}"
        )),
    }
}

fn validate_metric_number(
    evidence: &ReceiptEvidence,
    metric: &str,
    expected: u32,
    expected_backend: &str,
    errors: &mut Vec<String>,
) {
    match evidence.metrics.get(metric) {
        Some(observed) => {
            let parsed = parse_u32(
                observed,
                &format!("{expected_backend} metric {metric}"),
                errors,
            );
            if parsed != expected {
                errors.push(format!(
                    "{expected_backend} evidence metric {metric} expected {expected}, got {parsed}"
                ));
            }
        }
        None => errors.push(format!(
            "{expected_backend} evidence missing metric {metric}"
        )),
    }
}

fn validate_metric_agreement(
    paper: &ReceiptEvidence,
    valence: &ReceiptEvidence,
    errors: &mut Vec<String>,
) {
    for metric in RECEIPT_METRICS_FOR_AGREEMENT {
        let paper_value = paper
            .metrics
            .get(*metric)
            .and_then(|value| canonical_metric_value(metric, value, PAPER_BACKEND, errors));
        let valence_value = valence
            .metrics
            .get(*metric)
            .and_then(|value| canonical_metric_value(metric, value, VALENCE_BACKEND, errors));
        if let (Some(paper_value), Some(valence_value)) = (paper_value, valence_value) {
            if paper_value != valence_value {
                errors.push(format!(
                    "receipt metric mismatch for {metric}: paper={paper_value} valence={valence_value}"
                ));
            }
        }
    }
}

fn canonical_metric_value(
    metric: &str,
    value: &str,
    backend: &str,
    errors: &mut Vec<String>,
) -> Option<String> {
    match metric {
        METRIC_INPUT_ITEM | METRIC_FUEL_ITEM | METRIC_OUTPUT_ITEM | METRIC_INVALID_FUEL_ITEM => {
            match normalize_item_alias(value) {
                Ok(normalized) => Some(normalized.to_string()),
                Err(error) => {
                    errors.push(format!(
                        "{backend} evidence metric {metric} rejected: {error}"
                    ));
                    None
                }
            }
        }
        METRIC_OUTPUT_COUNT | METRIC_COOK_TICKS | METRIC_BURN_TICKS => {
            let parsed = parse_u32(value, &format!("{backend} metric {metric}"), errors);
            Some(parsed.to_string())
        }
        _ => Some(value.to_string()),
    }
}

fn normalize_item_alias(value: &str) -> Result<&'static str, String> {
    match value {
        RAW_IRON_RECEIPT_ALIAS | RAW_IRON_ITEM => Ok(RAW_IRON_ITEM),
        COAL_RECEIPT_ALIAS | COAL_ITEM => Ok(COAL_ITEM),
        IRON_INGOT_RECEIPT_ALIAS | IRON_INGOT_ITEM => Ok(IRON_INGOT_ITEM),
        GOLD_INGOT_RECEIPT_ALIAS | GOLD_INGOT_ITEM => Ok(GOLD_INGOT_ITEM),
        _ => Err(format!("unsupported item alias: {value}")),
    }
}

fn validate_text_field(label: &str, observed: &str, expected: &str, errors: &mut Vec<String>) {
    if observed != expected {
        errors.push(format!("{label} expected {expected}, got {observed}"));
    }
}

fn validate_number_field(label: &str, observed: u32, expected: u32, errors: &mut Vec<String>) {
    if observed != expected {
        errors.push(format!("{label} expected {expected}, got {observed}"));
    }
}

fn parse_u32(value: &str, label: &str, errors: &mut Vec<String>) -> u32 {
    match value.parse::<u32>() {
        Ok(parsed) => parsed,
        Err(error) => {
            errors.push(format!("{label} must be an unsigned integer: {error}"));
            u32::default()
        }
    }
}

fn run_self_tests() -> Result<String, Vec<String>> {
    let fixture = fixture_row();
    let paper_text = fixture_receipt(PAPER_BACKEND);
    let valence_text = fixture_receipt(VALENCE_BACKEND);
    let paper = parse_receipt_kv(&paper_text)?;
    let valence = parse_receipt_kv(&valence_text)?;
    validate_handoff(&fixture, Some(&paper), Some(&valence))?;

    let mut negative_cases = 0;
    negative_cases += expect_handoff_failure(
        &fixture,
        None,
        Some(&valence),
        "missing Paper evidence",
        "missing paper evidence should fail",
    )?;
    negative_cases += expect_handoff_failure(
        &fixture,
        Some(&paper),
        None,
        "missing Valence evidence",
        "missing valence evidence should fail",
    )?;
    negative_cases += expect_replaced_receipt_failure(
        &fixture,
        &paper_text,
        &valence_text,
        PAPER_BACKEND,
        &format!("row={EXPECTED_RECEIPT_ROW}"),
        "row=stale-furnace-row",
        "paper row expected",
    )?;
    negative_cases += expect_replaced_receipt_failure(
        &fixture,
        &paper_text,
        &valence_text,
        VALENCE_BACKEND,
        "revision_status=clean",
        "revision_status=dirty",
        "revision status expected clean",
    )?;
    negative_cases += expect_replaced_receipt_failure(
        &fixture,
        &paper_text,
        &valence_text,
        PAPER_BACKEND,
        &format!("metric.{METRIC_INPUT_ITEM}={RAW_IRON_RECEIPT_ALIAS}"),
        &format!("metric.{METRIC_INPUT_ITEM}={GOLD_INGOT_RECEIPT_ALIAS}"),
        METRIC_INPUT_ITEM,
    )?;
    negative_cases += expect_replaced_receipt_failure(
        &fixture,
        &paper_text,
        &valence_text,
        VALENCE_BACKEND,
        &format!("metric.{METRIC_FUEL_ITEM}={COAL_RECEIPT_ALIAS}"),
        &format!("metric.{METRIC_FUEL_ITEM}={RAW_IRON_RECEIPT_ALIAS}"),
        METRIC_FUEL_ITEM,
    )?;
    negative_cases += expect_replaced_receipt_failure(
        &fixture,
        &paper_text,
        &valence_text,
        PAPER_BACKEND,
        &format!("metric.{METRIC_OUTPUT_ITEM}={IRON_INGOT_RECEIPT_ALIAS}"),
        &format!("metric.{METRIC_OUTPUT_ITEM}={GOLD_INGOT_RECEIPT_ALIAS}"),
        METRIC_OUTPUT_ITEM,
    )?;
    negative_cases += expect_replaced_receipt_failure(
        &fixture,
        &paper_text,
        &valence_text,
        PAPER_BACKEND,
        &format!("metric.{METRIC_OUTPUT_COUNT}={SELECTED_OUTPUT_COUNT}"),
        &format!("metric.{METRIC_OUTPUT_COUNT}={WRONG_OUTPUT_COUNT}"),
        METRIC_OUTPUT_COUNT,
    )?;
    negative_cases += expect_replaced_receipt_failure(
        &fixture,
        &paper_text,
        &valence_text,
        VALENCE_BACKEND,
        &format!("metric.{METRIC_COOK_TICKS}={STANDARD_FURNACE_COOK_TICKS}"),
        &format!("metric.{METRIC_COOK_TICKS}={WRONG_COOK_TICKS}"),
        METRIC_COOK_TICKS,
    )?;
    negative_cases += expect_replaced_receipt_failure(
        &fixture,
        &paper_text,
        &valence_text,
        PAPER_BACKEND,
        &format!("metric.{METRIC_BURN_TICKS}={COAL_BURN_TICKS}"),
        &format!("metric.{METRIC_BURN_TICKS}={WRONG_BURN_TICKS}"),
        METRIC_BURN_TICKS,
    )?;
    negative_cases += expect_parse_failure(
        "row=broken\nthis is not key value\n",
        "expected key=value line",
        "malformed row should fail",
    )?;
    negative_cases += expect_replaced_receipt_failure(
        &fixture,
        &paper_text,
        &valence_text,
        PAPER_BACKEND,
        &format!("metric.{METRIC_NONCLAIM_ALL_FURNACES}={TRUE_VALUE}\n"),
        "",
        METRIC_NONCLAIM_ALL_FURNACES,
    )?;
    negative_cases += expect_handoff_failure(
        &fixture_missing_non_claim(),
        Some(&paper),
        Some(&valence),
        "fixture missing required non-claim",
        "fixture missing non-claim should fail",
    )?;
    let overclaim_text = format!("{paper_text}{CLAIM_ALL_FURNACES}={TRUE_VALUE}\n");
    let overclaim_paper = parse_receipt_kv(&overclaim_text)?;
    negative_cases += expect_handoff_failure(
        &fixture,
        Some(&overclaim_paper),
        Some(&valence),
        "broad overclaim",
        "broad furnace overclaim should fail",
    )?;

    Ok(format!(
        "positive selected-row handoff and {negative_cases} negative cases exercised"
    ))
}

fn expect_replaced_receipt_failure(
    fixture: &SelectedFixtureRow,
    paper_text: &str,
    valence_text: &str,
    mutated_backend: &str,
    old: &str,
    new: &str,
    expected_diagnostic: &str,
) -> Result<usize, Vec<String>> {
    let mutated_text = match mutated_backend {
        PAPER_BACKEND => paper_text.replace(old, new),
        VALENCE_BACKEND => valence_text.replace(old, new),
        _ => return Err(vec![format!("unknown mutated backend: {mutated_backend}")]),
    };
    let paper = if mutated_backend == PAPER_BACKEND {
        parse_receipt_kv(&mutated_text)?
    } else {
        parse_receipt_kv(paper_text)?
    };
    let valence = if mutated_backend == VALENCE_BACKEND {
        parse_receipt_kv(&mutated_text)?
    } else {
        parse_receipt_kv(valence_text)?
    };
    expect_handoff_failure(
        fixture,
        Some(&paper),
        Some(&valence),
        expected_diagnostic,
        "mutated receipt should fail",
    )
}

fn expect_handoff_failure(
    fixture: &SelectedFixtureRow,
    paper: Option<&ReceiptEvidence>,
    valence: Option<&ReceiptEvidence>,
    expected_diagnostic: &str,
    context: &str,
) -> Result<usize, Vec<String>> {
    let errors = validate_handoff(fixture, paper, valence).expect_err(context);
    assert_contains(&errors, expected_diagnostic)?;
    Ok(ARGUMENT_STEP)
}

fn expect_parse_failure(
    text: &str,
    expected_diagnostic: &str,
    context: &str,
) -> Result<usize, Vec<String>> {
    let errors = parse_receipt_kv(text).expect_err(context);
    assert_contains(&errors, expected_diagnostic)?;
    Ok(ARGUMENT_STEP)
}

fn fixture_row() -> SelectedFixtureRow {
    SelectedFixtureRow {
        target_edition: TARGET_EDITION.to_string(),
        target_game_version: TARGET_GAME_VERSION.to_string(),
        target_protocol: TARGET_PROTOCOL,
        furnace_kind: STANDARD_FURNACE_KIND.to_string(),
        recipe_id: EXPECTED_FIXTURE_RECIPE_ID.to_string(),
        input_item: RAW_IRON_ITEM.to_string(),
        fuel_item: COAL_ITEM.to_string(),
        output_item: IRON_INGOT_ITEM.to_string(),
        output_count: SELECTED_OUTPUT_COUNT,
        cook_ticks: STANDARD_FURNACE_COOK_TICKS,
        burn_ticks: COAL_BURN_TICKS,
        non_claims: REQUIRED_FIXTURE_NON_CLAIMS
            .iter()
            .map(|claim| (*claim).to_string())
            .collect(),
    }
}

fn fixture_missing_non_claim() -> SelectedFixtureRow {
    let mut fixture = fixture_row();
    let removed_non_claim = REQUIRED_FIXTURE_NON_CLAIMS
        .first()
        .expect("non-claims exist");
    fixture.non_claims.remove(*removed_non_claim);
    fixture
}

fn fixture_receipt(backend: &str) -> String {
    format!(
        "row={EXPECTED_RECEIPT_ROW}\nbackend={backend}\nrevision_status=clean\nchild_revision={FIXTURE_REVISION}\nmetric.{METRIC_MATRIX_VERSION}={EXPECTED_MATRIX_VERSION}\nmetric.{METRIC_RECIPE_ID}={EXPECTED_RECEIPT_RECIPE_ID}\nmetric.{METRIC_INPUT_ITEM}={RAW_IRON_RECEIPT_ALIAS}\nmetric.{METRIC_FUEL_ITEM}={COAL_RECEIPT_ALIAS}\nmetric.{METRIC_BURN_TICKS}={COAL_BURN_TICKS}\nmetric.{METRIC_COOK_TICKS}={STANDARD_FURNACE_COOK_TICKS}\nmetric.{METRIC_OUTPUT_ITEM}={IRON_INGOT_RECEIPT_ALIAS}\nmetric.{METRIC_OUTPUT_COUNT}={SELECTED_OUTPUT_COUNT}\nmetric.{METRIC_INVALID_FUEL_ITEM}={RAW_IRON_RECEIPT_ALIAS}\nmetric.{METRIC_INVALID_FUEL_OUTCOME}={INVALID_FUEL_OUTCOME}\nmetric.{METRIC_NONCLAIM_ALL_FURNACES}={TRUE_VALUE}\n"
    )
}

fn assert_contains(errors: &[String], expected_diagnostic: &str) -> Result<(), Vec<String>> {
    if errors
        .iter()
        .any(|error| error.contains(expected_diagnostic))
    {
        Ok(())
    } else {
        Err(vec![format!(
            "missing expected diagnostic {expected_diagnostic:?}: {errors:?}"
        )])
    }
}
