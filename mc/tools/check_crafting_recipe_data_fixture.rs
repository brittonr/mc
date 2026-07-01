#!/usr/bin/env -S CARGO_TARGET_DIR=target/check-crafting-recipe-data-fixture-script nix shell "github:nix-community/fenix?rev=092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain" nixpkgs#gcc nixpkgs#nickel -c cargo -q -Zscript

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

const FIXTURE_FLAG: &str = "--fixture";
const SELF_TEST_FLAG: &str = "--self-test";
const HELP_FLAG: &str = "--help";
const DEFAULT_FIXTURE_PATH: &str = "compat/config/crafting-recipe-selected-matrix-fixture.ncl";
const HELP_TEXT: &str =
    "usage: check_crafting_recipe_data_fixture.rs [--self-test] [--fixture PATH]";
const SUCCESS_MESSAGE: &str = "crafting recipe selected-matrix data fixture check passed";
const SELF_TEST_SUCCESS_MESSAGE: &str =
    "crafting recipe selected-matrix data fixture self-test passed";
const SUCCESS: ExitCode = ExitCode::SUCCESS;
const FAILURE: ExitCode = ExitCode::FAILURE;
const NICKEL_COMMAND: &str = "nickel";
const NICKEL_EXPORT_SUBCOMMAND: &str = "export";
const VALID_FIXTURE_TEXT: &str =
    include_str!("../compat/config/crafting-recipe-selected-matrix-fixture.ncl");

const TARGET_EDITION_BINDING: &str = "target_edition";
const TARGET_GAME_VERSION_BINDING: &str = "target_game_version";
const TARGET_PROTOCOL_BINDING: &str = "target_protocol";
const GRID_WIDTH_BINDING: &str = "selected_grid_width";
const GRID_HEIGHT_BINDING: &str = "selected_grid_height";
const MAX_STACK_SIZE_BINDING: &str = "selected_max_stack_size";
const RESULT_SLOT_STACK_LIMIT_BINDING: &str = "selected_result_slot_stack_limit";
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

const TARGET_EDITION: &str = "Java Edition";
const TARGET_GAME_VERSION: &str = "1.20.1";
const TARGET_PROTOCOL: u32 = 763;
const GRID_WIDTH: u32 = 3;
const GRID_HEIGHT: u32 = 3;
const MAX_STACK_SIZE: u32 = 64;
const RESULT_SLOT_STACK_LIMIT: u32 = 64;
const CHEST_OUTPUT_COUNT: u32 = 1;
const OAK_PLANKS_OUTPUT_COUNT: u32 = 4;
const CHEST_TARGET_SLOT: u32 = 36;
const OAK_PLANKS_TARGET_SLOT: u32 = 37;
const SHAPED_KIND: &str = "shaped";
const SHAPELESS_KIND: &str = "shapeless";
const REJECTED_NO_RESULT_KIND: &str = "rejected_no_result";
const COLLECTION_MODE: &str = "primary_click";
const CHEST_RECIPE_ID: &str = "minecraft:chest";
const OAK_PLANKS_RECIPE_ID: &str = "minecraft:oak_planks";
const INVALID_PROBE_ID: &str = "minecraft:stick_insufficient_input_rejection";
const CHEST_KEY_SYMBOL: &str = "P";
const CHEST_PATTERN_TOP: &str = "PPP";
const CHEST_PATTERN_MIDDLE: &str = "P P";
const CHEST_PATTERN_BOTTOM: &str = "PPP";
const OAK_PLANKS: &str = "minecraft:oak_planks";
const OAK_LOG: &str = "minecraft:oak_log";
const CHEST: &str = "minecraft:chest";
const INVALID_PROBE_DIAGNOSTIC: &str = "no_result";
const CONTRACT_IMPORT: &str = "import \"crafting-recipe-selected-matrix-fixture-contracts.ncl\"";
const SCHEMA_VALUE: &str = "mc.compat.crafting-recipe.selected-matrix.fixture.v1";
const SELECTED_SHAPED_ROLE: &str = "fixture_role = \"selected_shaped_recipe\"";
const SELECTED_SHAPELESS_ROLE: &str = "fixture_role = \"selected_shapeless_recipe\"";
const SELECTED_INVALID_PROBE_ROLE: &str = "fixture_role = \"selected_invalid_probe\"";
const SELECTED_ROW_COUNT: usize = 1;
const SHAPED_AND_SHAPELESS_ROW_COUNT: usize = 2;
const RADIX_TEN: u32 = 10;
const LINE_NUMBER_OFFSET: usize = 1;
const MINECRAFT_ITEM_PREFIX: &str = "minecraft:";

const REQUIRED_STRING_BINDINGS: &[(&str, &str)] = &[
    (TARGET_EDITION_BINDING, TARGET_EDITION),
    (TARGET_GAME_VERSION_BINDING, TARGET_GAME_VERSION),
    (SHAPED_KIND_BINDING, SHAPED_KIND),
    (SHAPELESS_KIND_BINDING, SHAPELESS_KIND),
    (REJECTED_NO_RESULT_KIND_BINDING, REJECTED_NO_RESULT_KIND),
    (COLLECTION_MODE_BINDING, COLLECTION_MODE),
    (CHEST_RECIPE_ID_BINDING, CHEST_RECIPE_ID),
    (CHEST_KEY_SYMBOL_BINDING, CHEST_KEY_SYMBOL),
    (CHEST_PATTERN_TOP_BINDING, CHEST_PATTERN_TOP),
    (CHEST_PATTERN_MIDDLE_BINDING, CHEST_PATTERN_MIDDLE),
    (CHEST_PATTERN_BOTTOM_BINDING, CHEST_PATTERN_BOTTOM),
    (CHEST_KEY_ITEM_BINDING, OAK_PLANKS),
    (CHEST_OUTPUT_ITEM_BINDING, CHEST),
    (OAK_PLANKS_RECIPE_ID_BINDING, OAK_PLANKS_RECIPE_ID),
    (SHAPELESS_INPUT_ITEM_BINDING, OAK_LOG),
    (SHAPELESS_OUTPUT_ITEM_BINDING, OAK_PLANKS),
    (INVALID_PROBE_ID_BINDING, INVALID_PROBE_ID),
    (INVALID_PROBE_INPUT_ITEM_BINDING, OAK_PLANKS),
    (INVALID_PROBE_DIAGNOSTIC_BINDING, INVALID_PROBE_DIAGNOSTIC),
];

const REQUIRED_NUMBER_BINDINGS: &[(&str, u32)] = &[
    (TARGET_PROTOCOL_BINDING, TARGET_PROTOCOL),
    (GRID_WIDTH_BINDING, GRID_WIDTH),
    (GRID_HEIGHT_BINDING, GRID_HEIGHT),
    (MAX_STACK_SIZE_BINDING, MAX_STACK_SIZE),
    (RESULT_SLOT_STACK_LIMIT_BINDING, RESULT_SLOT_STACK_LIMIT),
    (CHEST_OUTPUT_COUNT_BINDING, CHEST_OUTPUT_COUNT),
    (OAK_PLANKS_OUTPUT_COUNT_BINDING, OAK_PLANKS_OUTPUT_COUNT),
    (CHEST_TARGET_SLOT_BINDING, CHEST_TARGET_SLOT),
    (OAK_PLANKS_TARGET_SLOT_BINDING, OAK_PLANKS_TARGET_SLOT),
    (CHEST_KEY_COUNT_BINDING, CHEST_OUTPUT_COUNT),
    (SHAPELESS_INPUT_COUNT_BINDING, CHEST_OUTPUT_COUNT),
    (INVALID_PROBE_INPUT_COUNT_BINDING, CHEST_OUTPUT_COUNT),
];

const REQUIRED_ITEM_BINDINGS: &[&str] = &[
    CHEST_KEY_ITEM_BINDING,
    CHEST_OUTPUT_ITEM_BINDING,
    SHAPELESS_INPUT_ITEM_BINDING,
    SHAPELESS_OUTPUT_ITEM_BINDING,
    INVALID_PROBE_INPUT_ITEM_BINDING,
];

const REQUIRED_SYMBOL_ASSIGNMENTS: &[(&str, &str)] = &[
    ("edition", TARGET_EDITION_BINDING),
    ("game_version", TARGET_GAME_VERSION_BINDING),
    ("protocol", TARGET_PROTOCOL_BINDING),
    ("grid_width", GRID_WIDTH_BINDING),
    ("grid_height", GRID_HEIGHT_BINDING),
    ("max_stack_size", MAX_STACK_SIZE_BINDING),
    ("result_slot_stack_limit", RESULT_SLOT_STACK_LIMIT_BINDING),
    ("selected_chest_output_count", CHEST_OUTPUT_COUNT_BINDING),
    (
        "selected_oak_planks_output_count",
        OAK_PLANKS_OUTPUT_COUNT_BINDING,
    ),
    ("selected_chest_target_slot", CHEST_TARGET_SLOT_BINDING),
    (
        "selected_oak_planks_target_slot",
        OAK_PLANKS_TARGET_SLOT_BINDING,
    ),
    ("recipe_id", CHEST_RECIPE_ID_BINDING),
    ("recipe_kind", SHAPED_KIND_BINDING),
    ("output_item", CHEST_OUTPUT_ITEM_BINDING),
    ("output_count", CHEST_OUTPUT_COUNT_BINDING),
    ("recipe_id", OAK_PLANKS_RECIPE_ID_BINDING),
    ("recipe_kind", SHAPELESS_KIND_BINDING),
    ("output_item", SHAPELESS_OUTPUT_ITEM_BINDING),
    ("output_count", OAK_PLANKS_OUTPUT_COUNT_BINDING),
    ("probe_id", INVALID_PROBE_ID_BINDING),
    ("probe_kind", REJECTED_NO_RESULT_KIND_BINDING),
    ("input_item", INVALID_PROBE_INPUT_ITEM_BINDING),
    ("input_count", INVALID_PROBE_INPUT_COUNT_BINDING),
    ("expected_diagnostic", INVALID_PROBE_DIAGNOSTIC_BINDING),
];

const REQUIRED_NON_CLAIMS: &[&str] = &[
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

const REQUIRED_PROVENANCE_MARKERS: &[&str] = &[
    "data_origin = ",
    "extraction_status = ",
    "Minecraft Wiki: Crafting",
    "Minecraft Wiki: Recipe (Java Edition)",
    "Minecraft Wiki: Java Edition 1.20.1",
    "docs/crafting-recipe-behavior-card.md",
    "docs/crafting-recipe-selected-matrix-core.md",
];

#[derive(Debug, Clone, PartialEq, Eq)]
enum CommandMode {
    Check { fixture_path: PathBuf },
    SelfTest,
    Help,
}

fn main() -> ExitCode {
    match parse_command(env::args().skip(1)) {
        Ok(CommandMode::Help) => {
            println!("{HELP_TEXT}");
            SUCCESS
        }
        Ok(CommandMode::SelfTest) => run_and_report_self_tests(),
        Ok(CommandMode::Check { fixture_path }) => run_and_report_fixture_check(&fixture_path),
        Err(error) => {
            eprintln!("{error}");
            FAILURE
        }
    }
}

fn parse_command(args: impl Iterator<Item = String>) -> Result<CommandMode, String> {
    let args = args.collect::<Vec<_>>();
    let mut fixture_path = PathBuf::from(DEFAULT_FIXTURE_PATH);
    let mut self_test = false;
    let mut index = 0;

    while index < args.len() {
        let arg = &args[index];
        if arg == HELP_FLAG {
            return Ok(CommandMode::Help);
        }
        if arg == SELF_TEST_FLAG {
            self_test = true;
            index += LINE_NUMBER_OFFSET;
            continue;
        }
        if arg == FIXTURE_FLAG {
            index += LINE_NUMBER_OFFSET;
            let Some(path) = args.get(index) else {
                return Err(format!("{FIXTURE_FLAG} requires a path"));
            };
            fixture_path = PathBuf::from(path);
            index += LINE_NUMBER_OFFSET;
            continue;
        }
        return Err(format!("unknown argument: {arg}"));
    }

    if self_test {
        Ok(CommandMode::SelfTest)
    } else {
        Ok(CommandMode::Check { fixture_path })
    }
}

fn run_and_report_self_tests() -> ExitCode {
    match run_self_tests() {
        Ok(()) => {
            println!("{SELF_TEST_SUCCESS_MESSAGE}");
            SUCCESS
        }
        Err(error) => {
            eprintln!("crafting recipe selected-matrix data fixture self-test failed: {error}");
            FAILURE
        }
    }
}

fn run_and_report_fixture_check(fixture_path: &Path) -> ExitCode {
    match run_fixture_check(fixture_path) {
        Ok(()) => {
            println!("{SUCCESS_MESSAGE}");
            SUCCESS
        }
        Err(errors) => {
            for error in errors {
                eprintln!("crafting recipe selected-matrix data fixture check failed: {error}");
            }
            FAILURE
        }
    }
}

fn run_fixture_check(fixture_path: &Path) -> Result<(), Vec<String>> {
    let text = fs::read_to_string(fixture_path)
        .map_err(|error| vec![format!("{}: {error}", fixture_path.display())])?;
    run_nickel_export(fixture_path)?;
    let diagnostics = validate_fixture_text(&text);
    if diagnostics.is_empty() {
        Ok(())
    } else {
        Err(diagnostics)
    }
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

fn validate_fixture_text(text: &str) -> Vec<String> {
    let mut diagnostics = Vec::new();

    require_contains(text, CONTRACT_IMPORT, "contract import", &mut diagnostics);
    require_contains(
        text,
        &quoted_schema_assignment(),
        "schema",
        &mut diagnostics,
    );

    for (binding, expected) in REQUIRED_STRING_BINDINGS {
        require_string_binding(text, binding, expected, &mut diagnostics);
    }

    for (binding, expected) in REQUIRED_NUMBER_BINDINGS {
        require_number_binding(text, binding, *expected, &mut diagnostics);
    }

    for item_binding in REQUIRED_ITEM_BINDINGS {
        require_item_binding(text, item_binding, &mut diagnostics);
    }

    require_pattern_binding(
        text,
        CHEST_PATTERN_TOP_BINDING,
        CHEST_PATTERN_TOP,
        &mut diagnostics,
    );
    require_pattern_binding(
        text,
        CHEST_PATTERN_MIDDLE_BINDING,
        CHEST_PATTERN_MIDDLE,
        &mut diagnostics,
    );
    require_pattern_binding(
        text,
        CHEST_PATTERN_BOTTOM_BINDING,
        CHEST_PATTERN_BOTTOM,
        &mut diagnostics,
    );
    require_unique_selected_ids(text, &mut diagnostics);

    require_occurrence_count(
        text,
        SELECTED_SHAPED_ROLE,
        SELECTED_ROW_COUNT,
        "selected shaped recipe row",
        &mut diagnostics,
    );
    require_occurrence_count(
        text,
        SELECTED_SHAPELESS_ROLE,
        SELECTED_ROW_COUNT,
        "selected shapeless recipe row",
        &mut diagnostics,
    );
    require_occurrence_count(
        text,
        SELECTED_INVALID_PROBE_ROLE,
        SELECTED_ROW_COUNT,
        "selected invalid/no-result probe",
        &mut diagnostics,
    );
    require_occurrence_count(
        text,
        &symbol_assignment("collection_mode", COLLECTION_MODE_BINDING),
        SHAPED_AND_SHAPELESS_ROW_COUNT,
        "supported primary-click collection mode assignment",
        &mut diagnostics,
    );

    for (field, binding) in REQUIRED_SYMBOL_ASSIGNMENTS {
        require_contains(
            text,
            &symbol_assignment(field, binding),
            &format!("{field} uses named binding {binding}"),
            &mut diagnostics,
        );
    }

    for provenance_marker in REQUIRED_PROVENANCE_MARKERS {
        require_contains(
            text,
            provenance_marker,
            &format!("required provenance marker {provenance_marker}"),
            &mut diagnostics,
        );
    }

    for non_claim in REQUIRED_NON_CLAIMS {
        require_contains(
            text,
            &quoted_value(non_claim),
            &format!("required non-claim {non_claim}"),
            &mut diagnostics,
        );
    }

    for claim in REQUIRED_FALSE_CLAIMS {
        require_contains(
            text,
            &false_assignment(claim),
            &format!("false claim boundary {claim}"),
            &mut diagnostics,
        );
    }

    diagnostics
}

fn require_contains(text: &str, needle: &str, label: &str, diagnostics: &mut Vec<String>) {
    if !text.contains(needle) {
        diagnostics.push(format!("missing or invalid {label}: expected {needle:?}"));
    }
}

fn require_string_binding(
    text: &str,
    binding: &str,
    expected: &str,
    diagnostics: &mut Vec<String>,
) {
    match read_string_binding(text, binding) {
        Some(actual) if actual == expected => {}
        Some(actual) => diagnostics.push(format!(
            "binding {binding} expected {expected:?}, got {actual:?}"
        )),
        None => diagnostics.push(format!("missing string binding {binding}")),
    }
}

fn require_number_binding(text: &str, binding: &str, expected: u32, diagnostics: &mut Vec<String>) {
    match read_number_binding(text, binding) {
        Some(actual) if actual == expected => {}
        Some(actual) => diagnostics.push(format!(
            "binding {binding} expected {expected}, got {actual}"
        )),
        None => diagnostics.push(format!("missing numeric binding {binding}")),
    }
}

fn require_item_binding(text: &str, binding: &str, diagnostics: &mut Vec<String>) {
    match read_string_binding(text, binding) {
        Some(actual) if is_minecraft_item_id(&actual) => {}
        Some(actual) => diagnostics.push(format!(
            "binding {binding} has malformed minecraft item id {actual:?}"
        )),
        None => diagnostics.push(format!("missing item binding {binding}")),
    }
}

fn require_pattern_binding(
    text: &str,
    binding: &str,
    expected: &str,
    diagnostics: &mut Vec<String>,
) {
    require_string_binding(text, binding, expected, diagnostics);
    let Some(actual) = read_string_binding(text, binding) else {
        return;
    };
    if actual.chars().count() != GRID_WIDTH as usize {
        diagnostics.push(format!(
            "binding {binding} has malformed shaped pattern width {actual:?}"
        ));
        return;
    }
    if actual
        .chars()
        .any(|character| character != 'P' && character != ' ')
    {
        diagnostics.push(format!(
            "binding {binding} has unsupported shaped pattern symbol {actual:?}"
        ));
    }
}

fn require_unique_selected_ids(text: &str, diagnostics: &mut Vec<String>) {
    let ids = [
        read_string_binding(text, CHEST_RECIPE_ID_BINDING),
        read_string_binding(text, OAK_PLANKS_RECIPE_ID_BINDING),
        read_string_binding(text, INVALID_PROBE_ID_BINDING),
    ];
    for left_index in 0..ids.len() {
        let Some(left) = &ids[left_index] else {
            continue;
        };
        for right in ids.iter().skip(left_index + LINE_NUMBER_OFFSET).flatten() {
            if left == right {
                diagnostics.push(format!("duplicate recipe or probe id {left:?}"));
            }
        }
    }
}

fn require_occurrence_count(
    text: &str,
    needle: &str,
    expected: usize,
    label: &str,
    diagnostics: &mut Vec<String>,
) {
    let actual = count_occurrences(text, needle);
    if actual != expected {
        diagnostics.push(format!(
            "{label} expected {expected} occurrence, got {actual}"
        ));
    }
}

fn read_string_binding(text: &str, binding: &str) -> Option<String> {
    let needle = format!("let {binding} = \"");
    let start = text.find(&needle)? + needle.len();
    let remainder = &text[start..];
    let end = remainder.find('"')?;
    Some(remainder[..end].to_string())
}

fn read_number_binding(text: &str, binding: &str) -> Option<u32> {
    let needle = format!("let {binding} = ");
    let start = text.find(&needle)? + needle.len();
    let remainder = &text[start..];
    let raw_number = remainder
        .chars()
        .take_while(|character| character.is_ascii_digit() || *character == '_')
        .collect::<String>();
    if raw_number.is_empty() {
        return None;
    }
    let normalized = raw_number.replace('_', "");
    u32::from_str_radix(&normalized, RADIX_TEN).ok()
}

fn is_minecraft_item_id(value: &str) -> bool {
    let Some(path) = value.strip_prefix(MINECRAFT_ITEM_PREFIX) else {
        return false;
    };
    !path.is_empty()
        && path.chars().all(|character| {
            character.is_ascii_lowercase()
                || character.is_ascii_digit()
                || character == '_'
                || character == '/'
        })
}

fn count_occurrences(text: &str, needle: &str) -> usize {
    text.match_indices(needle).count()
}

fn quoted_schema_assignment() -> String {
    format!("schema = {}", quoted_value(SCHEMA_VALUE))
}

fn quoted_value(value: &str) -> String {
    format!("\"{value}\"")
}

fn symbol_assignment(field: &str, binding: &str) -> String {
    format!("{field} = {binding}")
}

fn false_assignment(field: &str) -> String {
    format!("{field} = false")
}

fn run_self_tests() -> Result<(), String> {
    expect_valid_fixture("valid fixture", VALID_FIXTURE_TEXT)?;
    expect_invalid_fixture(
        "bad target protocol",
        &VALID_FIXTURE_TEXT.replace("let target_protocol = 763", "let target_protocol = 764"),
        TARGET_PROTOCOL_BINDING,
    )?;
    expect_invalid_fixture(
        "missing provenance",
        &VALID_FIXTURE_TEXT.replace("data_origin = ", "origin_data = "),
        "required provenance marker data_origin = ",
    )?;
    expect_invalid_fixture(
        "missing shaped row",
        &VALID_FIXTURE_TEXT.replace(
            "fixture_role = \"selected_shaped_recipe\"",
            "fixture_role = \"not_selected_shaped_recipe\"",
        ),
        "selected shaped recipe row",
    )?;
    expect_invalid_fixture(
        "missing shapeless row",
        &VALID_FIXTURE_TEXT.replace(
            "fixture_role = \"selected_shapeless_recipe\"",
            "fixture_role = \"not_selected_shapeless_recipe\"",
        ),
        "selected shapeless recipe row",
    )?;
    expect_invalid_fixture(
        "duplicate selected ids",
        &VALID_FIXTURE_TEXT.replace(
            "let selected_oak_planks_recipe_id = \"minecraft:oak_planks\"",
            "let selected_oak_planks_recipe_id = \"minecraft:chest\"",
        ),
        "duplicate recipe or probe id",
    )?;
    expect_invalid_fixture(
        "malformed shaped data",
        &VALID_FIXTURE_TEXT.replace(
            "let selected_chest_pattern_top = \"PPP\"",
            "let selected_chest_pattern_top = \"PP\"",
        ),
        CHEST_PATTERN_TOP_BINDING,
    )?;
    expect_invalid_fixture(
        "malformed shapeless data",
        &VALID_FIXTURE_TEXT.replace(
            "let selected_shapeless_input_count = 1",
            "let selected_shapeless_input_count = 0",
        ),
        SHAPELESS_INPUT_COUNT_BINDING,
    )?;
    expect_invalid_fixture(
        "invalid item id",
        &VALID_FIXTURE_TEXT.replace("minecraft:oak_log", "oak log"),
        SHAPELESS_INPUT_ITEM_BINDING,
    )?;
    expect_invalid_fixture(
        "zero output count",
        &VALID_FIXTURE_TEXT.replace(
            "let selected_oak_planks_output_count_value = 4",
            "let selected_oak_planks_output_count_value = 0",
        ),
        OAK_PLANKS_OUTPUT_COUNT_BINDING,
    )?;
    expect_invalid_fixture(
        "unsupported recipe kind",
        &VALID_FIXTURE_TEXT.replace(
            "let selected_shaped_recipe_kind = \"shaped\"",
            "let selected_shaped_recipe_kind = \"stonecutting\"",
        ),
        SHAPED_KIND_BINDING,
    )?;
    expect_invalid_fixture(
        "unsupported collection mode",
        &VALID_FIXTURE_TEXT.replace(
            "let selected_collection_mode = \"primary_click\"",
            "let selected_collection_mode = \"shift_click\"",
        ),
        COLLECTION_MODE_BINDING,
    )?;
    expect_invalid_fixture(
        "missing non-claim boundary",
        &VALID_FIXTURE_TEXT.replace("no Paper/vanilla parity", "omitted Paper parity boundary"),
        "required non-claim no Paper/vanilla parity",
    )?;
    expect_invalid_fixture(
        "overbroad claim boundary",
        &VALID_FIXTURE_TEXT.replace(
            "claims_all_recipe_breadth = false",
            "claims_all_recipe_breadth = true",
        ),
        "false claim boundary claims_all_recipe_breadth",
    )?;
    Ok(())
}

fn expect_valid_fixture(name: &str, text: &str) -> Result<(), String> {
    let diagnostics = validate_fixture_text(text);
    if diagnostics.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "{name}: expected valid fixture, got {diagnostics:?}"
        ))
    }
}

fn expect_invalid_fixture(name: &str, text: &str, expected_substring: &str) -> Result<(), String> {
    let diagnostics = validate_fixture_text(text);
    if diagnostics.is_empty() {
        return Err(format!("{name}: expected invalid fixture"));
    }
    if diagnostics
        .iter()
        .any(|diagnostic| diagnostic.contains(expected_substring))
    {
        Ok(())
    } else {
        Err(format!(
            "{name}: expected diagnostic containing {expected_substring:?}, got {diagnostics:?}"
        ))
    }
}
