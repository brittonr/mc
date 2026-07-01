#!/usr/bin/env -S CARGO_TARGET_DIR=target/check-crafting-recipe-receipt-handoff-script nix shell "github:nix-community/fenix?rev=092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain" nixpkgs#gcc nixpkgs#nickel -c cargo -q -Zscript

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
const HELP_TEXT: &str = "usage: check_crafting_recipe_receipt_handoff.rs [--self-test] [--fixture PATH --paper PATH --valence PATH]";
const SUCCESS_MESSAGE: &str = "crafting recipe selected-matrix receipt handoff check passed";
const SELF_TEST_SUCCESS_MESSAGE: &str =
    "crafting recipe selected-matrix receipt handoff self-test passed";
const SUCCESS: ExitCode = ExitCode::SUCCESS;
const FAILURE: ExitCode = ExitCode::FAILURE;

const ARGUMENT_STEP: usize = 1;
const KEY_VALUE_SEPARATOR: char = '=';
const PATH_PREFIX: &str = "docs/evidence/";
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
const GRID_WIDTH: u32 = 3;
const GRID_HEIGHT: u32 = 3;
const FIRST_GRID_SLOT: u32 = 1;
const RESULT_SLOT: u32 = 0;
const EMPTY_COUNT: u32 = 0;
const CHEST_OUTPUT_COUNT: u32 = 1;
const OAK_PLANKS_OUTPUT_COUNT: u32 = 4;
const CHEST_TARGET_SLOT: u32 = 36;
const OAK_PLANKS_TARGET_SLOT: u32 = 37;
const WRONG_TARGET_SLOT: &str = "38";
const WRONG_OUTPUT_COUNT: &str = "3";

const SHAPED_KIND: &str = "shaped";
const SHAPELESS_KIND: &str = "shapeless";
const REJECTED_NO_RESULT_KIND: &str = "rejected_no_result";
const COLLECTION_MODE: &str = "primary_click";
const UNSUPPORTED_COLLECTION_MODE: &str = "shift_click";

const EXPECTED_RECEIPT_ROW: &str = "survival-crafting-recipe-breadth-parity";
const EXPECTED_MATRIX_VERSION: &str = "2026-06-20";
const PAPER_BACKEND: &str = "paper";
const VALENCE_BACKEND: &str = "valence";
const FIXTURE_REVISION: &str = "abc1234";

const CHEST_RECIPE_ID: &str = "minecraft:chest";
const OAK_PLANKS_RECIPE_ID: &str = "minecraft:oak_planks";
const INVALID_PROBE_ID: &str = "minecraft:stick_insufficient_input_rejection";
const OAK_PLANKS_ITEM: &str = "minecraft:oak_planks";
const OAK_LOG_ITEM: &str = "minecraft:oak_log";
const CHEST_ITEM: &str = "minecraft:chest";
const NO_RESULT_ITEM: &str = "minecraft:none";
const INVALID_PROBE_DIAGNOSTIC: &str = "no_result";

const OAK_PLANKS_RECEIPT_ALIAS: &str = "OakPlanks";
const OAK_LOG_RECEIPT_ALIAS: &str = "OakLog";
const CHEST_RECEIPT_ALIAS: &str = "Chest";
const NO_RESULT_RECEIPT_ALIAS: &str = "None";

const TARGET_EDITION_BINDING: &str = "target_edition";
const TARGET_GAME_VERSION_BINDING: &str = "target_game_version";
const TARGET_PROTOCOL_BINDING: &str = "target_protocol";
const GRID_WIDTH_BINDING: &str = "selected_grid_width";
const GRID_HEIGHT_BINDING: &str = "selected_grid_height";
const CHEST_OUTPUT_COUNT_BINDING: &str = "selected_chest_output_count_value";
const OAK_PLANKS_OUTPUT_COUNT_BINDING: &str = "selected_oak_planks_output_count_value";
const CHEST_TARGET_SLOT_BINDING: &str = "selected_chest_target_slot_value";
const OAK_PLANKS_TARGET_SLOT_BINDING: &str = "selected_oak_planks_target_slot_value";
const SHAPED_KIND_BINDING: &str = "selected_shaped_recipe_kind";
const SHAPELESS_KIND_BINDING: &str = "selected_shapeless_recipe_kind";
const REJECTED_NO_RESULT_KIND_BINDING: &str = "selected_rejected_no_result_kind";
const COLLECTION_MODE_BINDING: &str = "selected_collection_mode";
const CHEST_RECIPE_ID_BINDING: &str = "selected_chest_recipe_id";
const CHEST_KEY_SYMBOL_BINDING: &str = "selected_chest_key_symbol";
const CHEST_PATTERN_TOP_BINDING: &str = "selected_chest_pattern_top";
const CHEST_PATTERN_MIDDLE_BINDING: &str = "selected_chest_pattern_middle";
const CHEST_PATTERN_BOTTOM_BINDING: &str = "selected_chest_pattern_bottom";
const CHEST_KEY_ITEM_BINDING: &str = "selected_chest_key_item";
const CHEST_KEY_COUNT_BINDING: &str = "selected_chest_key_count";
const CHEST_OUTPUT_ITEM_BINDING: &str = "selected_chest_output_item";
const OAK_PLANKS_RECIPE_ID_BINDING: &str = "selected_oak_planks_recipe_id";
const SHAPELESS_INPUT_ITEM_BINDING: &str = "selected_shapeless_input_item";
const SHAPELESS_INPUT_COUNT_BINDING: &str = "selected_shapeless_input_count";
const SHAPELESS_OUTPUT_ITEM_BINDING: &str = "selected_shapeless_output_item";
const INVALID_PROBE_ID_BINDING: &str = "selected_invalid_probe_id";
const INVALID_PROBE_INPUT_ITEM_BINDING: &str = "selected_invalid_probe_input_item";
const INVALID_PROBE_INPUT_COUNT_BINDING: &str = "selected_invalid_probe_input_count";
const INVALID_PROBE_DIAGNOSTIC_BINDING: &str = "selected_invalid_probe_diagnostic";

const FIELD_ROW: &str = "row";
const FIELD_BACKEND: &str = "backend";
const FIELD_REVISION_STATUS: &str = "revision_status";
const FIELD_CHILD_REVISION: &str = "child_revision";
const FIELD_RECEIPT: &str = "receipt";
const FIELD_TYPED_EVENTS: &str = "typed_events";
const FIELD_CLIENT_LOG: &str = "client_log";
const FIELD_SERVER_LOG: &str = "server_log";
const FIELD_RUN_LOG: &str = "run_log";

const METRIC_MATRIX_VERSION: &str = "matrix.version";
const METRIC_SHAPED_RECIPE_ID: &str = "matrix.shaped.recipe_id";
const METRIC_SHAPED_INPUT_SLOTS: &str = "matrix.shaped.input_slots";
const METRIC_SHAPED_RESULT_SLOT: &str = "matrix.shaped.result_slot";
const METRIC_SHAPED_RESULT_ITEM: &str = "matrix.shaped.result_item";
const METRIC_SHAPED_RESULT_COUNT: &str = "matrix.shaped.result_count";
const METRIC_SHAPED_COLLECTION_MODE: &str = "matrix.shaped.collection_mode";
const METRIC_SHAPED_FINAL_INVENTORY_SLOT: &str = "matrix.shaped.final_inventory_slot";
const METRIC_SHAPED_FINAL_INVENTORY_ITEM: &str = "matrix.shaped.final_inventory_item";
const METRIC_SHAPED_FINAL_INVENTORY_COUNT: &str = "matrix.shaped.final_inventory_count";
const METRIC_SHAPELESS_RECIPE_ID: &str = "matrix.shapeless.recipe_id";
const METRIC_SHAPELESS_INPUT_SLOTS: &str = "matrix.shapeless.input_slots";
const METRIC_SHAPELESS_RESULT_SLOT: &str = "matrix.shapeless.result_slot";
const METRIC_SHAPELESS_RESULT_ITEM: &str = "matrix.shapeless.result_item";
const METRIC_SHAPELESS_RESULT_COUNT: &str = "matrix.shapeless.result_count";
const METRIC_SHAPELESS_COLLECTION_MODE: &str = "matrix.shapeless.collection_mode";
const METRIC_SHAPELESS_FINAL_INVENTORY_SLOT: &str = "matrix.shapeless.final_inventory_slot";
const METRIC_SHAPELESS_FINAL_INVENTORY_ITEM: &str = "matrix.shapeless.final_inventory_item";
const METRIC_SHAPELESS_FINAL_INVENTORY_COUNT: &str = "matrix.shapeless.final_inventory_count";
const METRIC_INVALID_RECIPE_ID: &str = "matrix.invalid.recipe_id";
const METRIC_INVALID_INPUT_SLOTS: &str = "matrix.invalid.input_slots";
const METRIC_INVALID_RESULT_SLOT: &str = "matrix.invalid.result_slot";
const METRIC_INVALID_RESULT_ITEM: &str = "matrix.invalid.result_item";
const METRIC_INVALID_RESULT_COUNT: &str = "matrix.invalid.result_count";
const METRIC_INVALID_REJECTION_OUTCOME: &str = "matrix.invalid.rejection_outcome";
const METRIC_COLLECTION_MODES: &str = "matrix.collection_modes";
const METRIC_NONCLAIM_ALL_RECIPES: &str = "nonclaim.all_recipes";
const METRIC_NONCLAIM_RECIPE_BOOK_UI: &str = "nonclaim.recipe_book_ui";
const METRIC_NONCLAIM_ARBITRARY_COLLECTION_MODES: &str =
    "nonclaim.arbitrary_collection_modes";
const METRIC_NONCLAIM_FULL_SURVIVAL_COMPATIBILITY: &str =
    "nonclaim.full_survival_compatibility";
const METRIC_NONCLAIM_BROAD_VANILLA_PARITY: &str = "nonclaim.broad_vanilla_parity";

const REQUIRED_RECEIPT_PATH_FIELDS: &[&str] = &[
    FIELD_RECEIPT,
    FIELD_TYPED_EVENTS,
    FIELD_CLIENT_LOG,
    FIELD_SERVER_LOG,
    FIELD_RUN_LOG,
];

const REQUIRED_RECEIPT_NONCLAIM_METRICS: &[&str] = &[
    METRIC_NONCLAIM_ALL_RECIPES,
    METRIC_NONCLAIM_RECIPE_BOOK_UI,
    METRIC_NONCLAIM_ARBITRARY_COLLECTION_MODES,
    METRIC_NONCLAIM_FULL_SURVIVAL_COMPATIBILITY,
    METRIC_NONCLAIM_BROAD_VANILLA_PARITY,
];

const RECEIPT_METRICS_FOR_AGREEMENT: &[&str] = &[
    METRIC_MATRIX_VERSION,
    METRIC_SHAPED_RECIPE_ID,
    METRIC_SHAPED_INPUT_SLOTS,
    METRIC_SHAPED_RESULT_SLOT,
    METRIC_SHAPED_RESULT_ITEM,
    METRIC_SHAPED_RESULT_COUNT,
    METRIC_SHAPED_COLLECTION_MODE,
    METRIC_SHAPED_FINAL_INVENTORY_SLOT,
    METRIC_SHAPED_FINAL_INVENTORY_ITEM,
    METRIC_SHAPED_FINAL_INVENTORY_COUNT,
    METRIC_SHAPELESS_RECIPE_ID,
    METRIC_SHAPELESS_INPUT_SLOTS,
    METRIC_SHAPELESS_RESULT_SLOT,
    METRIC_SHAPELESS_RESULT_ITEM,
    METRIC_SHAPELESS_RESULT_COUNT,
    METRIC_SHAPELESS_COLLECTION_MODE,
    METRIC_SHAPELESS_FINAL_INVENTORY_SLOT,
    METRIC_SHAPELESS_FINAL_INVENTORY_ITEM,
    METRIC_SHAPELESS_FINAL_INVENTORY_COUNT,
    METRIC_INVALID_RECIPE_ID,
    METRIC_INVALID_INPUT_SLOTS,
    METRIC_INVALID_RESULT_SLOT,
    METRIC_INVALID_RESULT_ITEM,
    METRIC_INVALID_RESULT_COUNT,
    METRIC_INVALID_REJECTION_OUTCOME,
    METRIC_COLLECTION_MODES,
    METRIC_NONCLAIM_ALL_RECIPES,
    METRIC_NONCLAIM_RECIPE_BOOK_UI,
    METRIC_NONCLAIM_ARBITRARY_COLLECTION_MODES,
    METRIC_NONCLAIM_FULL_SURVIVAL_COMPATIBILITY,
    METRIC_NONCLAIM_BROAD_VANILLA_PARITY,
];

const REQUIRED_FIXTURE_NON_CLAIMS: &[&str] = &[
    "no broad Minecraft compatibility",
    "no broad vanilla parity",
    "no Paper/vanilla parity",
    "no all-recipe breadth",
    "no arbitrary collection modes",
    "no shift-click/drag/split handling",
    "no data-pack loading",
    "no recipe-book UI behavior",
    "no recipe discovery, advancement, or doLimitedCrafting behavior",
    "no automated crafter behavior",
    "no Valence runtime integration",
    "no DefaultPlugins membership changes",
    "no public-server safety",
    "no production readiness",
];

const REQUIRED_FALSE_CLAIMS: &[&str] = &[
    "claims_paper_vanilla_parity",
    "claims_all_recipe_breadth",
    "claims_arbitrary_collection_modes",
    "claims_shift_click_drag_split",
    "claims_data_pack_loading",
    "claims_recipe_book_ui",
    "claims_automated_crafter",
    "claims_valence_runtime_integration",
    "claims_default_plugin_membership",
    "claims_public_server_safety",
    "claims_production_readiness",
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct SlotStack {
    slot: u32,
    item: String,
    count: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ShapedRecipe {
    recipe_id: String,
    pattern: Vec<String>,
    key_symbol: char,
    key_item: String,
    key_count: u32,
    output_item: String,
    output_count: u32,
    collection_mode: String,
    target_slot: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ShapelessRecipe {
    recipe_id: String,
    input_item: String,
    input_count: u32,
    output_item: String,
    output_count: u32,
    collection_mode: String,
    target_slot: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct InvalidProbe {
    probe_id: String,
    input_item: String,
    input_count: u32,
    expected_diagnostic: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SelectedCraftingFixture {
    target_edition: String,
    target_game_version: String,
    target_protocol: u32,
    grid_width: u32,
    grid_height: u32,
    shaped: ShapedRecipe,
    shapeless: ShapelessRecipe,
    invalid: InvalidProbe,
    non_claims: BTreeSet<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ReceiptEvidence {
    row: String,
    backend: String,
    revision_status: String,
    child_revision: String,
    paths: BTreeMap<String, String>,
    claims: BTreeMap<String, String>,
    metrics: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ReceiptHandoffDecision {
    shaped_recipe_id: String,
    shapeless_recipe_id: String,
    invalid_probe_id: String,
    collection_mode: String,
    shaped_target_slot: u32,
    shapeless_target_slot: u32,
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
        eprintln!("crafting recipe selected-matrix receipt handoff {scope} failed: {error}");
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
    validate_referenced_paths_exist(&paper)?;
    validate_referenced_paths_exist(&valence)?;
    let decision = validate_handoff(&fixture, Some(&paper), Some(&valence))?;

    Ok(format!(
        "target={} {} protocol {} row={} shaped={} shaped_inputs={} shaped_output={}x{} shaped_slot={} shapeless={} shapeless_inputs={} shapeless_output={}x{} shapeless_slot={} invalid={} invalid_input={} invalid_outcome={} collection_mode={} paper_child={} valence_child={} paper_kv={} valence_kv={} retained_nonclaims={}",
        fixture.target_edition,
        fixture.target_game_version,
        fixture.target_protocol,
        EXPECTED_RECEIPT_ROW,
        decision.shaped_recipe_id,
        format_slot_stacks(&expected_shaped_input_slots(&fixture)?),
        fixture.shaped.output_item,
        fixture.shaped.output_count,
        decision.shaped_target_slot,
        decision.shapeless_recipe_id,
        format_slot_stacks(&expected_shapeless_input_slots(&fixture)),
        fixture.shapeless.output_item,
        fixture.shapeless.output_count,
        decision.shapeless_target_slot,
        decision.invalid_probe_id,
        format_slot_stacks(&expected_invalid_input_slots(&fixture)),
        fixture.invalid.expected_diagnostic,
        decision.collection_mode,
        decision.paper_child_revision,
        decision.valence_child_revision,
        paper_path.display(),
        valence_path.display(),
        REQUIRED_RECEIPT_NONCLAIM_METRICS.join(",")
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

fn validate_referenced_paths_exist(evidence: &ReceiptEvidence) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();
    for path_field in REQUIRED_RECEIPT_PATH_FIELDS {
        let Some(path) = evidence.paths.get(*path_field) else {
            continue;
        };
        if !path.starts_with(PATH_PREFIX) {
            errors.push(format!(
                "{} evidence path {path_field} must stay under {PATH_PREFIX}: {path}",
                evidence.backend
            ));
            continue;
        }
        if !Path::new(path).exists() {
            errors.push(format!(
                "{} evidence path {path_field} does not exist: {path}",
                evidence.backend
            ));
        }
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn parse_selected_fixture(text: &str) -> Result<SelectedCraftingFixture, Vec<String>> {
    let mut errors = Vec::new();
    let non_claims = parse_fixture_non_claims(text, &mut errors);
    require_false_claims(text, &mut errors);

    let shaped_kind = required_string_binding(text, SHAPED_KIND_BINDING, &mut errors);
    validate_text_field("fixture shaped kind", &shaped_kind, SHAPED_KIND, &mut errors);
    let shapeless_kind = required_string_binding(text, SHAPELESS_KIND_BINDING, &mut errors);
    validate_text_field(
        "fixture shapeless kind",
        &shapeless_kind,
        SHAPELESS_KIND,
        &mut errors,
    );
    let rejected_kind = required_string_binding(text, REJECTED_NO_RESULT_KIND_BINDING, &mut errors);
    validate_text_field(
        "fixture rejected/no-result kind",
        &rejected_kind,
        REJECTED_NO_RESULT_KIND,
        &mut errors,
    );

    let collection_mode = required_string_binding(text, COLLECTION_MODE_BINDING, &mut errors);
    let fixture = SelectedCraftingFixture {
        target_edition: required_string_binding(text, TARGET_EDITION_BINDING, &mut errors),
        target_game_version: required_string_binding(text, TARGET_GAME_VERSION_BINDING, &mut errors),
        target_protocol: required_number_binding(text, TARGET_PROTOCOL_BINDING, &mut errors),
        grid_width: required_number_binding(text, GRID_WIDTH_BINDING, &mut errors),
        grid_height: required_number_binding(text, GRID_HEIGHT_BINDING, &mut errors),
        shaped: ShapedRecipe {
            recipe_id: required_string_binding(text, CHEST_RECIPE_ID_BINDING, &mut errors),
            pattern: vec![
                required_string_binding(text, CHEST_PATTERN_TOP_BINDING, &mut errors),
                required_string_binding(text, CHEST_PATTERN_MIDDLE_BINDING, &mut errors),
                required_string_binding(text, CHEST_PATTERN_BOTTOM_BINDING, &mut errors),
            ],
            key_symbol: required_symbol_binding(text, CHEST_KEY_SYMBOL_BINDING, &mut errors),
            key_item: required_string_binding(text, CHEST_KEY_ITEM_BINDING, &mut errors),
            key_count: required_number_binding(text, CHEST_KEY_COUNT_BINDING, &mut errors),
            output_item: required_string_binding(text, CHEST_OUTPUT_ITEM_BINDING, &mut errors),
            output_count: required_number_binding(text, CHEST_OUTPUT_COUNT_BINDING, &mut errors),
            collection_mode: collection_mode.clone(),
            target_slot: required_number_binding(text, CHEST_TARGET_SLOT_BINDING, &mut errors),
        },
        shapeless: ShapelessRecipe {
            recipe_id: required_string_binding(text, OAK_PLANKS_RECIPE_ID_BINDING, &mut errors),
            input_item: required_string_binding(text, SHAPELESS_INPUT_ITEM_BINDING, &mut errors),
            input_count: required_number_binding(text, SHAPELESS_INPUT_COUNT_BINDING, &mut errors),
            output_item: required_string_binding(text, SHAPELESS_OUTPUT_ITEM_BINDING, &mut errors),
            output_count: required_number_binding(text, OAK_PLANKS_OUTPUT_COUNT_BINDING, &mut errors),
            collection_mode: collection_mode.clone(),
            target_slot: required_number_binding(text, OAK_PLANKS_TARGET_SLOT_BINDING, &mut errors),
        },
        invalid: InvalidProbe {
            probe_id: required_string_binding(text, INVALID_PROBE_ID_BINDING, &mut errors),
            input_item: required_string_binding(text, INVALID_PROBE_INPUT_ITEM_BINDING, &mut errors),
            input_count: required_number_binding(text, INVALID_PROBE_INPUT_COUNT_BINDING, &mut errors),
            expected_diagnostic: required_string_binding(
                text,
                INVALID_PROBE_DIAGNOSTIC_BINDING,
                &mut errors,
            ),
        },
        non_claims,
    };

    validate_fixture_scope(&fixture, &mut errors);
    if errors.is_empty() {
        Ok(fixture)
    } else {
        Err(errors)
    }
}

fn parse_fixture_non_claims(text: &str, errors: &mut Vec<String>) -> BTreeSet<String> {
    let mut non_claims = BTreeSet::new();
    for non_claim in REQUIRED_FIXTURE_NON_CLAIMS {
        if text.contains(&quoted_value(non_claim)) {
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
    let needle = format!("let {binding} = \"");
    let Some(start) = text.find(&needle).map(|start| start + needle.len()) else {
        errors.push(format!("fixture missing string binding: {binding}"));
        return String::new();
    };
    let remainder = &text[start..];
    let Some(end) = remainder.find('"') else {
        errors.push(format!("fixture string binding {binding} is unterminated"));
        return String::new();
    };
    remainder[..end].to_string()
}

fn required_symbol_binding(text: &str, binding: &str, errors: &mut Vec<String>) -> char {
    let value = required_string_binding(text, binding, errors);
    let mut chars = value.chars();
    let Some(symbol) = chars.next() else {
        errors.push(format!("fixture symbol binding {binding} is empty"));
        return char::default();
    };
    if chars.next().is_some() {
        errors.push(format!(
            "fixture symbol binding {binding} must contain one character, got {value:?}"
        ));
    }
    symbol
}

fn required_number_binding(text: &str, binding: &str, errors: &mut Vec<String>) -> u32 {
    let needle = format!("let {binding} = ");
    let Some(start) = text.find(&needle).map(|start| start + needle.len()) else {
        errors.push(format!("fixture missing number binding: {binding}"));
        return u32::default();
    };
    let remainder = &text[start..];
    let raw_number = remainder
        .chars()
        .take_while(|character| character.is_ascii_digit() || *character == '_')
        .collect::<String>();
    if raw_number.is_empty() {
        errors.push(format!("fixture number binding {binding} is empty"));
        return u32::default();
    }
    parse_u32(&raw_number.replace('_', ""), &format!("fixture binding {binding}"), errors)
}

fn quoted_value(value: &str) -> String {
    format!("\"{value}\"")
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

    let paths = REQUIRED_RECEIPT_PATH_FIELDS
        .iter()
        .filter_map(|field| fields.get(*field).map(|value| ((*field).to_string(), value.clone())))
        .collect::<BTreeMap<_, _>>();

    let evidence = ReceiptEvidence {
        row: required_field(&fields, FIELD_ROW, &mut errors),
        backend: required_field(&fields, FIELD_BACKEND, &mut errors),
        revision_status: required_field(&fields, FIELD_REVISION_STATUS, &mut errors),
        child_revision: required_field(&fields, FIELD_CHILD_REVISION, &mut errors),
        paths,
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
    fixture: &SelectedCraftingFixture,
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
        Ok(ReceiptHandoffDecision {
            shaped_recipe_id: fixture.shaped.recipe_id.clone(),
            shapeless_recipe_id: fixture.shapeless.recipe_id.clone(),
            invalid_probe_id: fixture.invalid.probe_id.clone(),
            collection_mode: fixture.shaped.collection_mode.clone(),
            shaped_target_slot: fixture.shaped.target_slot,
            shapeless_target_slot: fixture.shapeless.target_slot,
            paper_child_revision: paper
                .map(|evidence| evidence.child_revision.clone())
                .unwrap_or_default(),
            valence_child_revision: valence
                .map(|evidence| evidence.child_revision.clone())
                .unwrap_or_default(),
        })
    } else {
        Err(errors)
    }
}

fn validate_fixture_scope(fixture: &SelectedCraftingFixture, errors: &mut Vec<String>) {
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
    validate_number_field("fixture grid width", fixture.grid_width, GRID_WIDTH, errors);
    validate_number_field("fixture grid height", fixture.grid_height, GRID_HEIGHT, errors);
    validate_text_field(
        "fixture shaped recipe id",
        &fixture.shaped.recipe_id,
        CHEST_RECIPE_ID,
        errors,
    );
    validate_text_field(
        "fixture shaped input item",
        &fixture.shaped.key_item,
        OAK_PLANKS_ITEM,
        errors,
    );
    validate_number_field(
        "fixture shaped input count",
        fixture.shaped.key_count,
        CHEST_OUTPUT_COUNT,
        errors,
    );
    validate_text_field(
        "fixture shaped output item",
        &fixture.shaped.output_item,
        CHEST_ITEM,
        errors,
    );
    validate_number_field(
        "fixture shaped output count",
        fixture.shaped.output_count,
        CHEST_OUTPUT_COUNT,
        errors,
    );
    validate_text_field(
        "fixture shaped collection mode",
        &fixture.shaped.collection_mode,
        COLLECTION_MODE,
        errors,
    );
    validate_number_field(
        "fixture shaped target slot",
        fixture.shaped.target_slot,
        CHEST_TARGET_SLOT,
        errors,
    );
    validate_text_field(
        "fixture shapeless recipe id",
        &fixture.shapeless.recipe_id,
        OAK_PLANKS_RECIPE_ID,
        errors,
    );
    validate_text_field(
        "fixture shapeless input item",
        &fixture.shapeless.input_item,
        OAK_LOG_ITEM,
        errors,
    );
    validate_number_field(
        "fixture shapeless input count",
        fixture.shapeless.input_count,
        CHEST_OUTPUT_COUNT,
        errors,
    );
    validate_text_field(
        "fixture shapeless output item",
        &fixture.shapeless.output_item,
        OAK_PLANKS_ITEM,
        errors,
    );
    validate_number_field(
        "fixture shapeless output count",
        fixture.shapeless.output_count,
        OAK_PLANKS_OUTPUT_COUNT,
        errors,
    );
    validate_text_field(
        "fixture shapeless collection mode",
        &fixture.shapeless.collection_mode,
        COLLECTION_MODE,
        errors,
    );
    validate_number_field(
        "fixture shapeless target slot",
        fixture.shapeless.target_slot,
        OAK_PLANKS_TARGET_SLOT,
        errors,
    );
    validate_text_field(
        "fixture invalid probe id",
        &fixture.invalid.probe_id,
        INVALID_PROBE_ID,
        errors,
    );
    validate_text_field(
        "fixture invalid input item",
        &fixture.invalid.input_item,
        OAK_PLANKS_ITEM,
        errors,
    );
    validate_number_field(
        "fixture invalid input count",
        fixture.invalid.input_count,
        CHEST_OUTPUT_COUNT,
        errors,
    );
    validate_text_field(
        "fixture invalid diagnostic",
        &fixture.invalid.expected_diagnostic,
        INVALID_PROBE_DIAGNOSTIC,
        errors,
    );

    if let Err(input_errors) = expected_shaped_input_slots(fixture) {
        errors.extend(input_errors);
    }
}

fn validate_fixture_non_claims(fixture: &SelectedCraftingFixture, errors: &mut Vec<String>) {
    for non_claim in REQUIRED_FIXTURE_NON_CLAIMS {
        if !fixture.non_claims.contains(*non_claim) {
            errors.push(format!("fixture missing required non-claim: {non_claim}"));
        }
    }
}

fn validate_receipt(
    fixture: &SelectedCraftingFixture,
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

    validate_required_paths(evidence, expected_backend, errors);
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
        METRIC_SHAPED_RECIPE_ID,
        &fixture.shaped.recipe_id,
        expected_backend,
        errors,
    );
    validate_metric_input_slots(
        evidence,
        METRIC_SHAPED_INPUT_SLOTS,
        &expected_shaped_input_slots_or_empty(fixture, errors),
        expected_backend,
        errors,
    );
    validate_metric_number(evidence, METRIC_SHAPED_RESULT_SLOT, RESULT_SLOT, expected_backend, errors);
    validate_metric_item(
        evidence,
        METRIC_SHAPED_RESULT_ITEM,
        &fixture.shaped.output_item,
        expected_backend,
        errors,
    );
    validate_metric_number(
        evidence,
        METRIC_SHAPED_RESULT_COUNT,
        fixture.shaped.output_count,
        expected_backend,
        errors,
    );
    validate_metric_text(
        evidence,
        METRIC_SHAPED_COLLECTION_MODE,
        &fixture.shaped.collection_mode,
        expected_backend,
        errors,
    );
    validate_metric_number(
        evidence,
        METRIC_SHAPED_FINAL_INVENTORY_SLOT,
        fixture.shaped.target_slot,
        expected_backend,
        errors,
    );
    validate_metric_item(
        evidence,
        METRIC_SHAPED_FINAL_INVENTORY_ITEM,
        &fixture.shaped.output_item,
        expected_backend,
        errors,
    );
    validate_metric_number(
        evidence,
        METRIC_SHAPED_FINAL_INVENTORY_COUNT,
        fixture.shaped.output_count,
        expected_backend,
        errors,
    );

    validate_metric_text(
        evidence,
        METRIC_SHAPELESS_RECIPE_ID,
        &fixture.shapeless.recipe_id,
        expected_backend,
        errors,
    );
    validate_metric_input_slots(
        evidence,
        METRIC_SHAPELESS_INPUT_SLOTS,
        &expected_shapeless_input_slots(fixture),
        expected_backend,
        errors,
    );
    validate_metric_number(
        evidence,
        METRIC_SHAPELESS_RESULT_SLOT,
        RESULT_SLOT,
        expected_backend,
        errors,
    );
    validate_metric_item(
        evidence,
        METRIC_SHAPELESS_RESULT_ITEM,
        &fixture.shapeless.output_item,
        expected_backend,
        errors,
    );
    validate_metric_number(
        evidence,
        METRIC_SHAPELESS_RESULT_COUNT,
        fixture.shapeless.output_count,
        expected_backend,
        errors,
    );
    validate_metric_text(
        evidence,
        METRIC_SHAPELESS_COLLECTION_MODE,
        &fixture.shapeless.collection_mode,
        expected_backend,
        errors,
    );
    validate_metric_number(
        evidence,
        METRIC_SHAPELESS_FINAL_INVENTORY_SLOT,
        fixture.shapeless.target_slot,
        expected_backend,
        errors,
    );
    validate_metric_item(
        evidence,
        METRIC_SHAPELESS_FINAL_INVENTORY_ITEM,
        &fixture.shapeless.output_item,
        expected_backend,
        errors,
    );
    validate_metric_number(
        evidence,
        METRIC_SHAPELESS_FINAL_INVENTORY_COUNT,
        fixture.shapeless.output_count,
        expected_backend,
        errors,
    );

    validate_metric_text(
        evidence,
        METRIC_INVALID_RECIPE_ID,
        &fixture.invalid.probe_id,
        expected_backend,
        errors,
    );
    validate_metric_input_slots(
        evidence,
        METRIC_INVALID_INPUT_SLOTS,
        &expected_invalid_input_slots(fixture),
        expected_backend,
        errors,
    );
    validate_metric_number(evidence, METRIC_INVALID_RESULT_SLOT, RESULT_SLOT, expected_backend, errors);
    validate_metric_item(
        evidence,
        METRIC_INVALID_RESULT_ITEM,
        NO_RESULT_ITEM,
        expected_backend,
        errors,
    );
    validate_metric_number(
        evidence,
        METRIC_INVALID_RESULT_COUNT,
        EMPTY_COUNT,
        expected_backend,
        errors,
    );
    validate_metric_text(
        evidence,
        METRIC_INVALID_REJECTION_OUTCOME,
        &fixture.invalid.expected_diagnostic,
        expected_backend,
        errors,
    );
    validate_metric_text(
        evidence,
        METRIC_COLLECTION_MODES,
        &fixture.shaped.collection_mode,
        expected_backend,
        errors,
    );

    for metric in REQUIRED_RECEIPT_NONCLAIM_METRICS {
        validate_metric_text(evidence, metric, TRUE_VALUE, expected_backend, errors);
    }
}

fn validate_required_paths(
    evidence: &ReceiptEvidence,
    expected_backend: &str,
    errors: &mut Vec<String>,
) {
    for field in REQUIRED_RECEIPT_PATH_FIELDS {
        match evidence.paths.get(*field) {
            Some(path) if !path.is_empty() => {}
            _ => errors.push(format!(
                "{expected_backend} evidence missing receipt path field {field}"
            )),
        }
    }
}

fn validate_claims(evidence: &ReceiptEvidence, expected_backend: &str, errors: &mut Vec<String>) {
    for (key, value) in &evidence.claims {
        if value == TRUE_VALUE {
            errors.push(format!(
                "{expected_backend} evidence contains broad crafting overclaim {key}={value}"
            ));
        }
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

fn validate_metric_input_slots(
    evidence: &ReceiptEvidence,
    metric: &str,
    expected: &[SlotStack],
    expected_backend: &str,
    errors: &mut Vec<String>,
) {
    match evidence.metrics.get(metric) {
        Some(observed) => match parse_slot_stacks(observed, expected_backend, metric, errors) {
            Some(actual) if actual == expected => {}
            Some(actual) => errors.push(format!(
                "{expected_backend} evidence metric {metric} expected {}, got {}",
                format_slot_stacks(expected),
                format_slot_stacks(&actual)
            )),
            None => {}
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
        METRIC_SHAPED_INPUT_SLOTS | METRIC_SHAPELESS_INPUT_SLOTS | METRIC_INVALID_INPUT_SLOTS => {
            parse_slot_stacks(value, backend, metric, errors).map(|slots| format_slot_stacks(&slots))
        }
        METRIC_SHAPED_RESULT_ITEM
        | METRIC_SHAPED_FINAL_INVENTORY_ITEM
        | METRIC_SHAPELESS_RESULT_ITEM
        | METRIC_SHAPELESS_FINAL_INVENTORY_ITEM
        | METRIC_INVALID_RESULT_ITEM => match normalize_item_alias(value) {
            Ok(normalized) => Some(normalized.to_string()),
            Err(error) => {
                errors.push(format!("{backend} evidence metric {metric} rejected: {error}"));
                None
            }
        },
        METRIC_SHAPED_RESULT_SLOT
        | METRIC_SHAPED_RESULT_COUNT
        | METRIC_SHAPED_FINAL_INVENTORY_SLOT
        | METRIC_SHAPED_FINAL_INVENTORY_COUNT
        | METRIC_SHAPELESS_RESULT_SLOT
        | METRIC_SHAPELESS_RESULT_COUNT
        | METRIC_SHAPELESS_FINAL_INVENTORY_SLOT
        | METRIC_SHAPELESS_FINAL_INVENTORY_COUNT
        | METRIC_INVALID_RESULT_SLOT
        | METRIC_INVALID_RESULT_COUNT => {
            let parsed = parse_u32(value, &format!("{backend} metric {metric}"), errors);
            Some(parsed.to_string())
        }
        _ => Some(value.to_string()),
    }
}

fn expected_shaped_input_slots_or_empty(
    fixture: &SelectedCraftingFixture,
    errors: &mut Vec<String>,
) -> Vec<SlotStack> {
    match expected_shaped_input_slots(fixture) {
        Ok(slots) => slots,
        Err(slot_errors) => {
            errors.extend(slot_errors);
            Vec::new()
        }
    }
}

fn expected_shaped_input_slots(fixture: &SelectedCraftingFixture) -> Result<Vec<SlotStack>, Vec<String>> {
    let mut errors = Vec::new();
    let mut slots = Vec::new();
    if fixture.shaped.pattern.len() != fixture.grid_height as usize {
        errors.push(format!(
            "fixture shaped pattern height expected {}, got {}",
            fixture.grid_height,
            fixture.shaped.pattern.len()
        ));
        return Err(errors);
    }
    for (row_index, row) in fixture.shaped.pattern.iter().enumerate() {
        if row.chars().count() != fixture.grid_width as usize {
            errors.push(format!(
                "fixture shaped pattern row expected width {}, got {row:?}",
                fixture.grid_width
            ));
            continue;
        }
        let row_index = u32::try_from(row_index)
            .map_err(|error| vec![format!("row index conversion failed: {error}")])?;
        for (column_index, symbol) in row.chars().enumerate() {
            if symbol == ' ' {
                continue;
            }
            if symbol != fixture.shaped.key_symbol {
                errors.push(format!(
                    "fixture shaped pattern has unsupported symbol {symbol:?}"
                ));
                continue;
            }
            let column_index = u32::try_from(column_index)
                .map_err(|error| vec![format!("column index conversion failed: {error}")])?;
            slots.push(SlotStack {
                slot: FIRST_GRID_SLOT + row_index * fixture.grid_width + column_index,
                item: fixture.shaped.key_item.clone(),
                count: fixture.shaped.key_count,
            });
        }
    }
    slots.sort();
    if errors.is_empty() {
        Ok(slots)
    } else {
        Err(errors)
    }
}

fn expected_shapeless_input_slots(fixture: &SelectedCraftingFixture) -> Vec<SlotStack> {
    vec![SlotStack {
        slot: FIRST_GRID_SLOT,
        item: fixture.shapeless.input_item.clone(),
        count: fixture.shapeless.input_count,
    }]
}

fn expected_invalid_input_slots(fixture: &SelectedCraftingFixture) -> Vec<SlotStack> {
    vec![SlotStack {
        slot: FIRST_GRID_SLOT,
        item: fixture.invalid.input_item.clone(),
        count: fixture.invalid.input_count,
    }]
}

fn parse_slot_stacks(
    value: &str,
    backend: &str,
    metric: &str,
    errors: &mut Vec<String>,
) -> Option<Vec<SlotStack>> {
    let mut slots = Vec::new();
    for entry in value.split(',') {
        let trimmed = entry.trim();
        let Some((slot_value, rest)) = trimmed.split_once(':') else {
            errors.push(format!(
                "{backend} evidence metric {metric} has malformed slot entry {trimmed:?}"
            ));
            return None;
        };
        let Some((item_value, count_value)) = rest.rsplit_once(':') else {
            errors.push(format!(
                "{backend} evidence metric {metric} has malformed item/count entry {trimmed:?}"
            ));
            return None;
        };
        let slot = parse_u32(slot_value, &format!("{backend} metric {metric} slot"), errors);
        let count = parse_u32(count_value, &format!("{backend} metric {metric} count"), errors);
        let item = match normalize_item_alias(item_value) {
            Ok(normalized) => normalized.to_string(),
            Err(error) => {
                errors.push(format!(
                    "{backend} evidence metric {metric} rejected: {error}"
                ));
                return None;
            }
        };
        slots.push(SlotStack { slot, item, count });
    }
    slots.sort();
    Some(slots)
}

fn format_slot_stacks(slots: &[SlotStack]) -> String {
    slots
        .iter()
        .map(|slot| format!("{}:{}:{}", slot.slot, slot.item, slot.count))
        .collect::<Vec<_>>()
        .join(",")
}

fn normalize_item_alias(value: &str) -> Result<&'static str, String> {
    match value {
        OAK_PLANKS_RECEIPT_ALIAS | OAK_PLANKS_ITEM => Ok(OAK_PLANKS_ITEM),
        OAK_LOG_RECEIPT_ALIAS | OAK_LOG_ITEM => Ok(OAK_LOG_ITEM),
        CHEST_RECEIPT_ALIAS | CHEST_ITEM => Ok(CHEST_ITEM),
        NO_RESULT_RECEIPT_ALIAS | NO_RESULT_ITEM => Ok(NO_RESULT_ITEM),
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
        "row=stale-crafting-row",
        "row expected",
    )?;
    negative_cases += expect_parse_failure(
        "row=broken\nthis is not key value\n",
        "expected key=value line",
        "malformed receipt row should fail",
    )?;
    negative_cases += expect_replaced_receipt_failure(
        &fixture,
        &paper_text,
        &valence_text,
        PAPER_BACKEND,
        &format!("metric.{METRIC_SHAPED_INPUT_SLOTS}={}", fixture_shaped_input_slots_aliases()),
        &format!("metric.{METRIC_SHAPED_INPUT_SLOTS}=1:{OAK_LOG_RECEIPT_ALIAS}:1,2:{OAK_PLANKS_RECEIPT_ALIAS}:1,3:{OAK_PLANKS_RECEIPT_ALIAS}:1,4:{OAK_PLANKS_RECEIPT_ALIAS}:1,6:{OAK_PLANKS_RECEIPT_ALIAS}:1,7:{OAK_PLANKS_RECEIPT_ALIAS}:1,8:{OAK_PLANKS_RECEIPT_ALIAS}:1,9:{OAK_PLANKS_RECEIPT_ALIAS}:1"),
        METRIC_SHAPED_INPUT_SLOTS,
    )?;
    negative_cases += expect_replaced_receipt_failure(
        &fixture,
        &paper_text,
        &valence_text,
        VALENCE_BACKEND,
        &format!("metric.{METRIC_SHAPELESS_INPUT_SLOTS}=1:{OAK_LOG_RECEIPT_ALIAS}:1"),
        &format!("metric.{METRIC_SHAPELESS_INPUT_SLOTS}=1:{CHEST_RECEIPT_ALIAS}:1"),
        METRIC_SHAPELESS_INPUT_SLOTS,
    )?;
    negative_cases += expect_replaced_receipt_failure(
        &fixture,
        &paper_text,
        &valence_text,
        PAPER_BACKEND,
        &format!("metric.{METRIC_SHAPED_RESULT_ITEM}={CHEST_RECEIPT_ALIAS}"),
        &format!("metric.{METRIC_SHAPED_RESULT_ITEM}={OAK_PLANKS_RECEIPT_ALIAS}"),
        METRIC_SHAPED_RESULT_ITEM,
    )?;
    negative_cases += expect_replaced_receipt_failure(
        &fixture,
        &paper_text,
        &valence_text,
        PAPER_BACKEND,
        &format!("metric.{METRIC_SHAPELESS_RESULT_COUNT}={OAK_PLANKS_OUTPUT_COUNT}"),
        &format!("metric.{METRIC_SHAPELESS_RESULT_COUNT}={WRONG_OUTPUT_COUNT}"),
        METRIC_SHAPELESS_RESULT_COUNT,
    )?;
    negative_cases += expect_replaced_receipt_failure(
        &fixture,
        &paper_text,
        &valence_text,
        VALENCE_BACKEND,
        &format!("metric.{METRIC_SHAPELESS_FINAL_INVENTORY_SLOT}={OAK_PLANKS_TARGET_SLOT}"),
        &format!("metric.{METRIC_SHAPELESS_FINAL_INVENTORY_SLOT}={WRONG_TARGET_SLOT}"),
        METRIC_SHAPELESS_FINAL_INVENTORY_SLOT,
    )?;
    negative_cases += expect_replaced_receipt_failure(
        &fixture,
        &paper_text,
        &valence_text,
        PAPER_BACKEND,
        &format!("metric.{METRIC_SHAPED_COLLECTION_MODE}={COLLECTION_MODE}"),
        &format!("metric.{METRIC_SHAPED_COLLECTION_MODE}={UNSUPPORTED_COLLECTION_MODE}"),
        METRIC_SHAPED_COLLECTION_MODE,
    )?;
    negative_cases += expect_replaced_receipt_failure(
        &fixture,
        &paper_text,
        &valence_text,
        PAPER_BACKEND,
        &format!("metric.{METRIC_NONCLAIM_ALL_RECIPES}={TRUE_VALUE}\n"),
        "",
        METRIC_NONCLAIM_ALL_RECIPES,
    )?;
    let overclaim_text = format!("{paper_text}{CLAIM_PREFIX}all_recipes={TRUE_VALUE}\n");
    let overclaim_paper = parse_receipt_kv(&overclaim_text)?;
    negative_cases += expect_handoff_failure(
        &fixture,
        Some(&overclaim_paper),
        Some(&valence),
        "broad crafting overclaim",
        "broad crafting overclaim should fail",
    )?;
    negative_cases += expect_handoff_failure(
        &fixture_missing_non_claim(),
        Some(&paper),
        Some(&valence),
        "fixture missing required non-claim",
        "fixture missing non-claim should fail",
    )?;

    Ok(format!(
        "positive selected-matrix handoff and {negative_cases} negative cases exercised"
    ))
}

fn expect_replaced_receipt_failure(
    fixture: &SelectedCraftingFixture,
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
    fixture: &SelectedCraftingFixture,
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

fn fixture_row() -> SelectedCraftingFixture {
    SelectedCraftingFixture {
        target_edition: TARGET_EDITION.to_string(),
        target_game_version: TARGET_GAME_VERSION.to_string(),
        target_protocol: TARGET_PROTOCOL,
        grid_width: GRID_WIDTH,
        grid_height: GRID_HEIGHT,
        shaped: ShapedRecipe {
            recipe_id: CHEST_RECIPE_ID.to_string(),
            pattern: vec!["PPP".to_string(), "P P".to_string(), "PPP".to_string()],
            key_symbol: 'P',
            key_item: OAK_PLANKS_ITEM.to_string(),
            key_count: CHEST_OUTPUT_COUNT,
            output_item: CHEST_ITEM.to_string(),
            output_count: CHEST_OUTPUT_COUNT,
            collection_mode: COLLECTION_MODE.to_string(),
            target_slot: CHEST_TARGET_SLOT,
        },
        shapeless: ShapelessRecipe {
            recipe_id: OAK_PLANKS_RECIPE_ID.to_string(),
            input_item: OAK_LOG_ITEM.to_string(),
            input_count: CHEST_OUTPUT_COUNT,
            output_item: OAK_PLANKS_ITEM.to_string(),
            output_count: OAK_PLANKS_OUTPUT_COUNT,
            collection_mode: COLLECTION_MODE.to_string(),
            target_slot: OAK_PLANKS_TARGET_SLOT,
        },
        invalid: InvalidProbe {
            probe_id: INVALID_PROBE_ID.to_string(),
            input_item: OAK_PLANKS_ITEM.to_string(),
            input_count: CHEST_OUTPUT_COUNT,
            expected_diagnostic: INVALID_PROBE_DIAGNOSTIC.to_string(),
        },
        non_claims: REQUIRED_FIXTURE_NON_CLAIMS
            .iter()
            .map(|claim| (*claim).to_string())
            .collect(),
    }
}

fn fixture_missing_non_claim() -> SelectedCraftingFixture {
    let mut fixture = fixture_row();
    let removed_non_claim = REQUIRED_FIXTURE_NON_CLAIMS
        .first()
        .expect("non-claims exist");
    fixture.non_claims.remove(*removed_non_claim);
    fixture
}

fn fixture_receipt(backend: &str) -> String {
    format!(
        "row={EXPECTED_RECEIPT_ROW}\nbackend={backend}\nrevision_status=clean\nchild_revision={FIXTURE_REVISION}\nreceipt=docs/evidence/{backend}-fixture.receipt.json\ntyped_events=docs/evidence/{backend}-fixture.typed-events.log\nclient_log=docs/evidence/{backend}-fixture.client.log\nserver_log=docs/evidence/{backend}-fixture.server.log\nrun_log=docs/evidence/{backend}-fixture.run.log\nmetric.{METRIC_MATRIX_VERSION}={EXPECTED_MATRIX_VERSION}\nmetric.{METRIC_SHAPED_RECIPE_ID}={CHEST_RECIPE_ID}\nmetric.{METRIC_SHAPED_INPUT_SLOTS}={}\nmetric.{METRIC_SHAPED_RESULT_SLOT}={RESULT_SLOT}\nmetric.{METRIC_SHAPED_RESULT_ITEM}={CHEST_RECEIPT_ALIAS}\nmetric.{METRIC_SHAPED_RESULT_COUNT}={CHEST_OUTPUT_COUNT}\nmetric.{METRIC_SHAPED_COLLECTION_MODE}={COLLECTION_MODE}\nmetric.{METRIC_SHAPED_FINAL_INVENTORY_SLOT}={CHEST_TARGET_SLOT}\nmetric.{METRIC_SHAPED_FINAL_INVENTORY_ITEM}={CHEST_RECEIPT_ALIAS}\nmetric.{METRIC_SHAPED_FINAL_INVENTORY_COUNT}={CHEST_OUTPUT_COUNT}\nmetric.{METRIC_SHAPELESS_RECIPE_ID}={OAK_PLANKS_RECIPE_ID}\nmetric.{METRIC_SHAPELESS_INPUT_SLOTS}=1:{OAK_LOG_RECEIPT_ALIAS}:1\nmetric.{METRIC_SHAPELESS_RESULT_SLOT}={RESULT_SLOT}\nmetric.{METRIC_SHAPELESS_RESULT_ITEM}={OAK_PLANKS_RECEIPT_ALIAS}\nmetric.{METRIC_SHAPELESS_RESULT_COUNT}={OAK_PLANKS_OUTPUT_COUNT}\nmetric.{METRIC_SHAPELESS_COLLECTION_MODE}={COLLECTION_MODE}\nmetric.{METRIC_SHAPELESS_FINAL_INVENTORY_SLOT}={OAK_PLANKS_TARGET_SLOT}\nmetric.{METRIC_SHAPELESS_FINAL_INVENTORY_ITEM}={OAK_PLANKS_RECEIPT_ALIAS}\nmetric.{METRIC_SHAPELESS_FINAL_INVENTORY_COUNT}={OAK_PLANKS_OUTPUT_COUNT}\nmetric.{METRIC_INVALID_RECIPE_ID}={INVALID_PROBE_ID}\nmetric.{METRIC_INVALID_INPUT_SLOTS}=1:{OAK_PLANKS_RECEIPT_ALIAS}:1\nmetric.{METRIC_INVALID_RESULT_SLOT}={RESULT_SLOT}\nmetric.{METRIC_INVALID_RESULT_ITEM}={NO_RESULT_RECEIPT_ALIAS}\nmetric.{METRIC_INVALID_RESULT_COUNT}={EMPTY_COUNT}\nmetric.{METRIC_INVALID_REJECTION_OUTCOME}={INVALID_PROBE_DIAGNOSTIC}\nmetric.{METRIC_COLLECTION_MODES}={COLLECTION_MODE}\nmetric.{METRIC_NONCLAIM_ALL_RECIPES}={TRUE_VALUE}\nmetric.{METRIC_NONCLAIM_RECIPE_BOOK_UI}={TRUE_VALUE}\nmetric.{METRIC_NONCLAIM_ARBITRARY_COLLECTION_MODES}={TRUE_VALUE}\nmetric.{METRIC_NONCLAIM_FULL_SURVIVAL_COMPATIBILITY}={TRUE_VALUE}\nmetric.{METRIC_NONCLAIM_BROAD_VANILLA_PARITY}={TRUE_VALUE}\n",
        fixture_shaped_input_slots_aliases()
    )
}

fn fixture_shaped_input_slots_aliases() -> String {
    format!(
        "1:{OAK_PLANKS_RECEIPT_ALIAS}:1,2:{OAK_PLANKS_RECEIPT_ALIAS}:1,3:{OAK_PLANKS_RECEIPT_ALIAS}:1,4:{OAK_PLANKS_RECEIPT_ALIAS}:1,6:{OAK_PLANKS_RECEIPT_ALIAS}:1,7:{OAK_PLANKS_RECEIPT_ALIAS}:1,8:{OAK_PLANKS_RECEIPT_ALIAS}:1,9:{OAK_PLANKS_RECEIPT_ALIAS}:1"
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
