#!/usr/bin/env -S CARGO_TARGET_DIR=target/check-furnace-smelting-data-fixture-script nix shell "github:nix-community/fenix?rev=092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain" nixpkgs#gcc nixpkgs#nickel -c cargo -q -Zscript

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

const FIXTURE_FLAG: &str = "--fixture";
const SELF_TEST_FLAG: &str = "--self-test";
const HELP_FLAG: &str = "--help";
const DEFAULT_FIXTURE_PATH: &str = "compat/config/furnace-smelting-selected-row-fixture.ncl";
const HELP_TEXT: &str =
    "usage: check_furnace_smelting_data_fixture.rs [--self-test] [--fixture PATH]";
const SUCCESS_MESSAGE: &str = "furnace smelting selected-row data fixture check passed";
const SELF_TEST_SUCCESS_MESSAGE: &str =
    "furnace smelting selected-row data fixture self-test passed";
const SUCCESS: ExitCode = ExitCode::SUCCESS;
const FAILURE: ExitCode = ExitCode::FAILURE;
const NICKEL_COMMAND: &str = "nickel";
const NICKEL_EXPORT_SUBCOMMAND: &str = "export";

const VALID_FIXTURE_TEXT: &str =
    include_str!("../compat/config/furnace-smelting-selected-row-fixture.ncl");

const TARGET_EDITION_BINDING: &str = "target_edition";
const TARGET_GAME_VERSION_BINDING: &str = "target_game_version";
const TARGET_PROTOCOL_BINDING: &str = "target_protocol";
const STANDARD_FURNACE_KIND_BINDING: &str = "standard_furnace_kind";
const STANDARD_FURNACE_COOK_TICKS_BINDING: &str = "selected_standard_furnace_cook_ticks";
const COAL_BURN_TICKS_BINDING: &str = "selected_coal_burn_ticks";
const MAX_STACK_SIZE_BINDING: &str = "selected_max_stack_size";
const SELECTED_RECIPE_OUTPUT_COUNT_BINDING: &str = "selected_recipe_output_count_value";
const SELECTED_RECIPE_INPUT_ITEM_BINDING: &str = "selected_recipe_input_item";
const SELECTED_RECIPE_OUTPUT_ITEM_BINDING: &str = "selected_recipe_output_item";
const SELECTED_FUEL_ITEM_BINDING: &str = "selected_fuel_item";

const TARGET_EDITION: &str = "Java Edition";
const TARGET_GAME_VERSION: &str = "1.20.1";
const TARGET_PROTOCOL: u32 = 763;
const STANDARD_FURNACE_KIND: &str = "standard";
const STANDARD_FURNACE_COOK_TICKS: u32 = 200;
const COAL_BURN_TICKS: u32 = 1_600;
const MAX_STACK_SIZE: u32 = 64;
const SELECTED_RECIPE_OUTPUT_COUNT: u32 = 1;
const SELECTED_RECIPE_INPUT_ITEM: &str = "minecraft:raw_iron";
const SELECTED_RECIPE_OUTPUT_ITEM: &str = "minecraft:iron_ingot";
const SELECTED_FUEL_ITEM: &str = "minecraft:coal";
const SELECTED_RECIPE_ROLE: &str = "fixture_role = \"selected_standard_furnace_recipe\"";
const SELECTED_FUEL_ROLE: &str = "fixture_role = \"selected_fuel\"";
const SELECTED_ROW_COUNT: usize = 1;
const RADIX_TEN: u32 = 10;
const LINE_NUMBER_OFFSET: usize = 1;
const MINECRAFT_ITEM_PREFIX: &str = "minecraft:";
const SCHEMA_VALUE: &str = "mc.compat.furnace-smelting.selected-row.fixture.v1";
const CONTRACT_IMPORT: &str = "import \"furnace-smelting-selected-row-fixture-contracts.ncl\"";

const REQUIRED_SYMBOL_ASSIGNMENTS: &[(&str, &str)] = &[
    ("edition", TARGET_EDITION_BINDING),
    ("game_version", TARGET_GAME_VERSION_BINDING),
    ("protocol", TARGET_PROTOCOL_BINDING),
    ("furnace_kind", STANDARD_FURNACE_KIND_BINDING),
    ("input_item", SELECTED_RECIPE_INPUT_ITEM_BINDING),
    ("output_item", SELECTED_RECIPE_OUTPUT_ITEM_BINDING),
    ("output_count", SELECTED_RECIPE_OUTPUT_COUNT_BINDING),
    ("cook_ticks", STANDARD_FURNACE_COOK_TICKS_BINDING),
    ("fuel_item", SELECTED_FUEL_ITEM_BINDING),
    ("burn_ticks", COAL_BURN_TICKS_BINDING),
];

const REQUIRED_NON_CLAIMS: &[&str] = &[
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
            eprintln!("furnace smelting selected-row data fixture self-test failed: {error}");
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
                eprintln!("furnace smelting selected-row data fixture check failed: {error}");
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
    require_string_binding(
        text,
        TARGET_EDITION_BINDING,
        TARGET_EDITION,
        &mut diagnostics,
    );
    require_string_binding(
        text,
        TARGET_GAME_VERSION_BINDING,
        TARGET_GAME_VERSION,
        &mut diagnostics,
    );
    require_number_binding(
        text,
        TARGET_PROTOCOL_BINDING,
        TARGET_PROTOCOL,
        &mut diagnostics,
    );
    require_string_binding(
        text,
        STANDARD_FURNACE_KIND_BINDING,
        STANDARD_FURNACE_KIND,
        &mut diagnostics,
    );
    require_number_binding(
        text,
        STANDARD_FURNACE_COOK_TICKS_BINDING,
        STANDARD_FURNACE_COOK_TICKS,
        &mut diagnostics,
    );
    require_number_binding(
        text,
        COAL_BURN_TICKS_BINDING,
        COAL_BURN_TICKS,
        &mut diagnostics,
    );
    require_number_binding(
        text,
        MAX_STACK_SIZE_BINDING,
        MAX_STACK_SIZE,
        &mut diagnostics,
    );
    require_number_binding(
        text,
        SELECTED_RECIPE_OUTPUT_COUNT_BINDING,
        SELECTED_RECIPE_OUTPUT_COUNT,
        &mut diagnostics,
    );
    require_string_binding(
        text,
        SELECTED_RECIPE_INPUT_ITEM_BINDING,
        SELECTED_RECIPE_INPUT_ITEM,
        &mut diagnostics,
    );
    require_string_binding(
        text,
        SELECTED_RECIPE_OUTPUT_ITEM_BINDING,
        SELECTED_RECIPE_OUTPUT_ITEM,
        &mut diagnostics,
    );
    require_string_binding(
        text,
        SELECTED_FUEL_ITEM_BINDING,
        SELECTED_FUEL_ITEM,
        &mut diagnostics,
    );
    require_item_binding(text, SELECTED_RECIPE_INPUT_ITEM_BINDING, &mut diagnostics);
    require_item_binding(text, SELECTED_RECIPE_OUTPUT_ITEM_BINDING, &mut diagnostics);
    require_item_binding(text, SELECTED_FUEL_ITEM_BINDING, &mut diagnostics);
    require_occurrence_count(
        text,
        SELECTED_RECIPE_ROLE,
        SELECTED_ROW_COUNT,
        "selected standard-furnace recipe row",
        &mut diagnostics,
    );
    require_occurrence_count(
        text,
        SELECTED_FUEL_ROLE,
        SELECTED_ROW_COUNT,
        "selected fuel row",
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
        "missing recipe row",
        &VALID_FIXTURE_TEXT.replace(
            "fixture_role = \"selected_standard_furnace_recipe\"",
            "fixture_role = \"not_selected_recipe\"",
        ),
        "selected standard-furnace recipe row",
    )?;
    expect_invalid_fixture(
        "missing fuel row",
        &VALID_FIXTURE_TEXT.replace(
            "fixture_role = \"selected_fuel\"",
            "fixture_role = \"not_selected_fuel\"",
        ),
        "selected fuel row",
    )?;
    expect_invalid_fixture(
        "malformed item id",
        &VALID_FIXTURE_TEXT.replace("minecraft:raw_iron", "raw iron"),
        SELECTED_RECIPE_INPUT_ITEM_BINDING,
    )?;
    expect_invalid_fixture(
        "invalid output count",
        &VALID_FIXTURE_TEXT.replace(
            "let selected_recipe_output_count_value = 1",
            "let selected_recipe_output_count_value = 0",
        ),
        SELECTED_RECIPE_OUTPUT_COUNT_BINDING,
    )?;
    expect_invalid_fixture(
        "invalid cook ticks",
        &VALID_FIXTURE_TEXT.replace(
            "let selected_standard_furnace_cook_ticks = 200",
            "let selected_standard_furnace_cook_ticks = 0",
        ),
        STANDARD_FURNACE_COOK_TICKS_BINDING,
    )?;
    expect_invalid_fixture(
        "unsupported furnace kind",
        &VALID_FIXTURE_TEXT.replace(
            "let standard_furnace_kind = \"standard\"",
            "let standard_furnace_kind = \"smoker\"",
        ),
        STANDARD_FURNACE_KIND_BINDING,
    )?;
    expect_invalid_fixture(
        "missing non-claim boundary",
        &VALID_FIXTURE_TEXT.replace("no Paper/vanilla parity", "omitted Paper parity boundary"),
        "required non-claim no Paper/vanilla parity",
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
