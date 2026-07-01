#!/usr/bin/env -S CARGO_TARGET_DIR=target/check-furnace-smelting-valence-shell-contract-script nix shell "github:nix-community/fenix?rev=092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain" nixpkgs#gcc -c cargo -q -Zscript

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const ROOT_FLAG: &str = "--root";
const CONTRACT_FLAG: &str = "--contract";
const SELF_TEST_FLAG: &str = "--self-test";
const HELP_FLAG: &str = "--help";
const DEFAULT_ROOT: &str = ".";
const DEFAULT_CONTRACT_PATH: &str = "docs/furnace-smelting-valence-shell-contract.md";
const SUCCESS_MESSAGE: &str = "furnace smelting Valence shell contract check passed";
const SELF_TEST_SUCCESS_MESSAGE: &str = "furnace smelting Valence shell contract self-test passed";
const HELP_TEXT: &str = "usage: check_furnace_smelting_valence_shell_contract.rs [--self-test] [--root PATH] [--contract PATH]";
const SUCCESS: ExitCode = ExitCode::SUCCESS;
const FAILURE: ExitCode = ExitCode::FAILURE;
const ARGUMENT_STEP: usize = 1;

const REQUIRED_GROUPS: &[RequiredGroup] = &[
    RequiredGroup {
        code: "contract_missing_target_scope",
        phrases: &[
            "## Target scope",
            "selected-row Valence shell contract",
            "Java Edition 1.20.1 / protocol 763",
            "standard furnace",
            "minecraft:raw_iron",
            "minecraft:coal",
            "minecraft:iron_ingot",
            "No Valence runtime integration is implemented",
        ],
    },
    RequiredGroup {
        code: "contract_missing_prerequisite_inventory",
        phrases: &[
            "## Inventory and prerequisite evidence",
            "docs/furnace-smelting-behavior-card.md",
            "docs/furnace-smelting-selected-row-core.md",
            "docs/furnace-smelting-selected-row-data-fixture.md",
            "docs/furnace-smelting-selected-row-receipt-handoff.md",
            "r[vanilla_composable_plugins.furnace_smelting_selected_row_receipts]",
            "r[valence_bevy_ecs.schedule_hygiene.policy]",
            "r[valence_bevy_ecs.gameplay_plugin_contracts.phase_contract]",
            "r[valence_bevy_ecs.inventory_sets.contract]",
        ],
    },
    RequiredGroup {
        code: "contract_missing_core_shell_boundary",
        phrases: &[
            "## Core and shell boundary",
            "future ECS systems snapshot",
            "call the selected-row core",
            "apply only the returned state, transition, or typed error",
            "pure core must not read files",
            "mutate Bevy world state",
            "write packets",
            "log",
        ],
    },
    RequiredGroup {
        code: "contract_missing_core_io_mapping",
        phrases: &[
            "## Core input mapping",
            "## Core output mapping",
            "FurnaceKind",
            "FurnaceState",
            "RecipeRow",
            "FuelRow",
            "FurnaceLimits",
            "FurnaceTransition",
        ],
    },
    RequiredGroup {
        code: "contract_missing_shell_ownership",
        phrases: &[
            "## Opt-in plugin ownership",
            "FurnaceSmeltingPlugin",
            "explicit opt-in",
            "GameplayInstallMode::ExplicitOptIn",
            "GameplayScopeModel::ArenaOwnedLayer",
            "FurnaceBlockEntity",
            "FurnaceRecipeTableResource",
            "FurnaceFuelTableResource",
            "FurnaceStateChangedEvent",
            "FurnaceDiagnosticEvent",
        ],
    },
    RequiredGroup {
        code: "contract_missing_schedule_facts",
        phrases: &[
            "## Schedule contract",
            "Update",
            "GameplayPhase::RuleEvaluation",
            "GameplayPhase::WorldMutation",
            "InventoryMutationSet",
            "InventoryPresentationSet",
            "FlushPacketsSet",
            "schedule hygiene evidence",
            "disabled-plugin comparison",
        ],
    },
    RequiredGroup {
        code: "contract_missing_disabled_plugin_behavior",
        phrases: &[
            "## Disabled-plugin behavior",
            "When FurnaceSmeltingPlugin is not installed",
            "no furnace resources are inserted",
            "no furnace events are registered",
            "no furnace systems run",
            "no furnace packets or milestones are emitted",
            "pure core remains callable by tests",
        ],
    },
    RequiredGroup {
        code: "contract_missing_data_loading_boundary",
        phrases: &[
            "## Data loading boundary",
            "startup shell or source adapter",
            "Nickel/exported fixture",
            "runtime data-pack parsing is not implemented",
            "must not parse data packs",
            "read files",
        ],
    },
    RequiredGroup {
        code: "contract_missing_mutation_packet_logging_boundary",
        phrases: &[
            "## Mutation, packet, and logging boundaries",
            "snapshot before rule evaluation",
            "single commit step",
            "Packet writes remain outside the core",
            "before packet flush",
            "Logging remains outside the core",
        ],
    },
    RequiredGroup {
        code: "contract_missing_validation_contract",
        phrases: &[
            "## Validation contract",
            "positive validation",
            "negative validation",
            "missing target scope",
            "missing core/shell boundary",
            "missing schedule facts",
            "missing disabled-plugin behavior",
            "DefaultPlugins membership overclaims",
            "broad recipe/furnace parity overclaims",
        ],
    },
    RequiredGroup {
        code: "contract_missing_future_evidence",
        phrases: &[
            "## Future closeout prerequisites",
            "before any Valence runtime behavior claim",
            "positive runtime tests",
            "negative runtime tests",
            "focused schedule evidence",
            "accepted-spec sync verification",
            "task-evidence validation",
            "evidence-manifest checks",
        ],
    },
    RequiredGroup {
        code: "contract_missing_non_claims",
        phrases: &[
            "## Non-claims",
            "No broad Minecraft compatibility",
            "No broad vanilla parity",
            "No all-recipe breadth",
            "No all-fuel breadth",
            "No broad furnace parity",
            "No Valence runtime integration",
            "No DefaultPlugins membership change",
            "No public-server safety",
            "No production readiness",
        ],
    },
];

const REJECTED_PHRASES: &[RejectedPhrase] = &[
    RejectedPhrase {
        code: "contract_rejected_default_plugins_overclaim",
        lowercase_phrase: "adds furnacesmeltingplugin to defaultplugins",
    },
    RejectedPhrase {
        code: "contract_rejected_default_plugins_overclaim",
        lowercase_phrase: "defaultplugins membership change is allowed",
    },
    RejectedPhrase {
        code: "contract_rejected_all_recipe_overclaim",
        lowercase_phrase: "claims all-recipe breadth",
    },
    RejectedPhrase {
        code: "contract_rejected_all_fuel_overclaim",
        lowercase_phrase: "claims all-fuel breadth",
    },
    RejectedPhrase {
        code: "contract_rejected_broad_furnace_overclaim",
        lowercase_phrase: "claims all furnace parity",
    },
    RejectedPhrase {
        code: "contract_rejected_broad_furnace_overclaim",
        lowercase_phrase: "claims broad furnace parity",
    },
    RejectedPhrase {
        code: "contract_rejected_broad_vanilla_parity_overclaim",
        lowercase_phrase: "claims broad vanilla parity",
    },
    RejectedPhrase {
        code: "contract_rejected_public_server_safety_overclaim",
        lowercase_phrase: "claims public-server safety",
    },
    RejectedPhrase {
        code: "contract_rejected_production_readiness_overclaim",
        lowercase_phrase: "claims production readiness",
    },
    RejectedPhrase {
        code: "contract_rejected_runtime_overclaim",
        lowercase_phrase: "claims valence runtime integration",
    },
    RejectedPhrase {
        code: "contract_rejected_all_recipe_overclaim",
        lowercase_phrase: "all recipes are proven",
    },
];

const SELF_TEST_VALID_CONTRACT: &str = r#"# Furnace smelting Valence shell contract

## Target scope
This is a selected-row Valence shell contract for Java Edition 1.20.1 / protocol 763, standard furnace, minecraft:raw_iron, minecraft:coal, and minecraft:iron_ingot. No Valence runtime integration is implemented.

## Inventory and prerequisite evidence
docs/furnace-smelting-behavior-card.md docs/furnace-smelting-selected-row-core.md docs/furnace-smelting-selected-row-data-fixture.md docs/furnace-smelting-selected-row-receipt-handoff.md r[vanilla_composable_plugins.furnace_smelting_selected_row_receipts] r[valence_bevy_ecs.schedule_hygiene.policy] r[valence_bevy_ecs.gameplay_plugin_contracts.phase_contract] r[valence_bevy_ecs.inventory_sets.contract].

## Core and shell boundary
future ECS systems snapshot values, call the selected-row core, and apply only the returned state, transition, or typed error. The pure core must not read files, mutate Bevy world state, write packets, or log.

## Core input mapping
FurnaceKind FurnaceState RecipeRow FuelRow FurnaceLimits.

## Core output mapping
FurnaceTransition.

## Opt-in plugin ownership
FurnaceSmeltingPlugin is explicit opt-in with GameplayInstallMode::ExplicitOptIn and GameplayScopeModel::ArenaOwnedLayer. FurnaceBlockEntity FurnaceRecipeTableResource FurnaceFuelTableResource FurnaceStateChangedEvent FurnaceDiagnosticEvent.

## Schedule contract
Update GameplayPhase::RuleEvaluation GameplayPhase::WorldMutation InventoryMutationSet InventoryPresentationSet FlushPacketsSet schedule hygiene evidence disabled-plugin comparison.

## Disabled-plugin behavior
When FurnaceSmeltingPlugin is not installed no furnace resources are inserted, no furnace events are registered, no furnace systems run, no furnace packets or milestones are emitted, and pure core remains callable by tests.

## Data loading boundary
startup shell or source adapter. Nickel/exported fixture. runtime data-pack parsing is not implemented. Future systems must not parse data packs or read files.

## Mutation, packet, and logging boundaries
snapshot before rule evaluation. single commit step. Packet writes remain outside the core and happen before packet flush. Logging remains outside the core.

## Validation contract
positive validation. negative validation. missing target scope. missing core/shell boundary. missing schedule facts. missing disabled-plugin behavior. DefaultPlugins membership overclaims. broad recipe/furnace parity overclaims.

## Future closeout prerequisites
before any Valence runtime behavior claim: positive runtime tests, negative runtime tests, focused schedule evidence, accepted-spec sync verification, task-evidence validation, evidence-manifest checks.

## Non-claims
No broad Minecraft compatibility. No broad vanilla parity. No all-recipe breadth. No all-fuel breadth. No broad furnace parity. No Valence runtime integration. No DefaultPlugins membership change. No public-server safety. No production readiness.
"#;

const NEGATIVE_REMOVAL_SELF_TESTS: &[NegativeRemovalSelfTest] = &[
    NegativeRemovalSelfTest {
        name: "missing target scope",
        phrase_to_remove: "Java Edition 1.20.1 / protocol 763",
        expected_code: "contract_missing_target_scope",
    },
    NegativeRemovalSelfTest {
        name: "missing prerequisite inventory",
        phrase_to_remove: "docs/furnace-smelting-selected-row-receipt-handoff.md",
        expected_code: "contract_missing_prerequisite_inventory",
    },
    NegativeRemovalSelfTest {
        name: "missing core shell boundary",
        phrase_to_remove: "call the selected-row core",
        expected_code: "contract_missing_core_shell_boundary",
    },
    NegativeRemovalSelfTest {
        name: "missing core mapping",
        phrase_to_remove: "FurnaceLimits",
        expected_code: "contract_missing_core_io_mapping",
    },
    NegativeRemovalSelfTest {
        name: "missing shell ownership",
        phrase_to_remove: "FurnaceBlockEntity",
        expected_code: "contract_missing_shell_ownership",
    },
    NegativeRemovalSelfTest {
        name: "missing schedule facts",
        phrase_to_remove: "InventoryPresentationSet",
        expected_code: "contract_missing_schedule_facts",
    },
    NegativeRemovalSelfTest {
        name: "missing disabled behavior",
        phrase_to_remove: "no furnace systems run",
        expected_code: "contract_missing_disabled_plugin_behavior",
    },
    NegativeRemovalSelfTest {
        name: "missing data loading boundary",
        phrase_to_remove: "runtime data-pack parsing is not implemented",
        expected_code: "contract_missing_data_loading_boundary",
    },
    NegativeRemovalSelfTest {
        name: "missing packet boundary",
        phrase_to_remove: "Packet writes remain outside the core",
        expected_code: "contract_missing_mutation_packet_logging_boundary",
    },
    NegativeRemovalSelfTest {
        name: "missing positive validation",
        phrase_to_remove: "positive validation",
        expected_code: "contract_missing_validation_contract",
    },
    NegativeRemovalSelfTest {
        name: "missing negative validation",
        phrase_to_remove: "negative validation",
        expected_code: "contract_missing_validation_contract",
    },
    NegativeRemovalSelfTest {
        name: "missing future evidence",
        phrase_to_remove: "accepted-spec sync verification",
        expected_code: "contract_missing_future_evidence",
    },
    NegativeRemovalSelfTest {
        name: "missing non claim",
        phrase_to_remove: "No all-recipe breadth",
        expected_code: "contract_missing_non_claims",
    },
];

const NEGATIVE_OVERCLAIM_SELF_TESTS: &[NegativeOverclaimSelfTest] = &[
    NegativeOverclaimSelfTest {
        name: "default plugin overclaim",
        phrase_to_append: "adds FurnaceSmeltingPlugin to DefaultPlugins",
        expected_code: "contract_rejected_default_plugins_overclaim",
    },
    NegativeOverclaimSelfTest {
        name: "all recipe overclaim",
        phrase_to_append: "claims all-recipe breadth",
        expected_code: "contract_rejected_all_recipe_overclaim",
    },
    NegativeOverclaimSelfTest {
        name: "all fuel overclaim",
        phrase_to_append: "claims all-fuel breadth",
        expected_code: "contract_rejected_all_fuel_overclaim",
    },
    NegativeOverclaimSelfTest {
        name: "broad furnace overclaim",
        phrase_to_append: "claims all furnace parity",
        expected_code: "contract_rejected_broad_furnace_overclaim",
    },
    NegativeOverclaimSelfTest {
        name: "broad vanilla overclaim",
        phrase_to_append: "claims broad vanilla parity",
        expected_code: "contract_rejected_broad_vanilla_parity_overclaim",
    },
    NegativeOverclaimSelfTest {
        name: "public server overclaim",
        phrase_to_append: "claims public-server safety",
        expected_code: "contract_rejected_public_server_safety_overclaim",
    },
    NegativeOverclaimSelfTest {
        name: "production overclaim",
        phrase_to_append: "claims production readiness",
        expected_code: "contract_rejected_production_readiness_overclaim",
    },
    NegativeOverclaimSelfTest {
        name: "runtime overclaim",
        phrase_to_append: "claims Valence runtime integration",
        expected_code: "contract_rejected_runtime_overclaim",
    },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RequiredGroup {
    code: &'static str,
    phrases: &'static [&'static str],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RejectedPhrase {
    code: &'static str,
    lowercase_phrase: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct NegativeRemovalSelfTest {
    name: &'static str,
    phrase_to_remove: &'static str,
    expected_code: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct NegativeOverclaimSelfTest {
    name: &'static str,
    phrase_to_append: &'static str,
    expected_code: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum CommandMode {
    Check {
        root: PathBuf,
        contract_path: PathBuf,
    },
    SelfTest,
    Help,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Diagnostic {
    code: &'static str,
    path: String,
    message: String,
}

fn main() -> ExitCode {
    match parse_command(env::args().skip(1)) {
        Ok(CommandMode::Help) => {
            println!("{HELP_TEXT}");
            SUCCESS
        }
        Ok(CommandMode::SelfTest) => run_and_report_self_test(),
        Ok(CommandMode::Check {
            root,
            contract_path,
        }) => run_and_report_check(&root, &contract_path),
        Err(error) => {
            eprintln!("{error}");
            FAILURE
        }
    }
}

fn parse_command(args: impl Iterator<Item = String>) -> Result<CommandMode, String> {
    let args = args.collect::<Vec<_>>();
    let mut root = PathBuf::from(DEFAULT_ROOT);
    let mut contract_path = PathBuf::from(DEFAULT_CONTRACT_PATH);
    let mut self_test = false;
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
        if arg == ROOT_FLAG {
            index += ARGUMENT_STEP;
            let Some(value) = args.get(index) else {
                return Err(format!("{ROOT_FLAG} requires a path"));
            };
            root = PathBuf::from(value);
            index += ARGUMENT_STEP;
            continue;
        }
        if arg == CONTRACT_FLAG {
            index += ARGUMENT_STEP;
            let Some(value) = args.get(index) else {
                return Err(format!("{CONTRACT_FLAG} requires a path"));
            };
            contract_path = PathBuf::from(value);
            index += ARGUMENT_STEP;
            continue;
        }
        return Err(format!("unknown argument: {arg}"));
    }

    if self_test {
        Ok(CommandMode::SelfTest)
    } else {
        Ok(CommandMode::Check {
            root,
            contract_path,
        })
    }
}

fn run_and_report_self_test() -> ExitCode {
    match run_self_test() {
        Ok(summary) => {
            println!("{SELF_TEST_SUCCESS_MESSAGE}: {summary}");
            SUCCESS
        }
        Err(diagnostics) => {
            print_diagnostics(&diagnostics);
            FAILURE
        }
    }
}

fn run_and_report_check(root: &Path, contract_path: &Path) -> ExitCode {
    match run_check(root, contract_path) {
        Ok(summary) => {
            println!("{SUCCESS_MESSAGE}: {summary}");
            SUCCESS
        }
        Err(diagnostics) => {
            print_diagnostics(&diagnostics);
            FAILURE
        }
    }
}

fn run_check(root: &Path, contract_path: &Path) -> Result<String, Vec<Diagnostic>> {
    let display_path = contract_path.to_string_lossy().into_owned();
    let full_path = root.join(contract_path);
    let contract = fs::read_to_string(&full_path).map_err(|error| {
        vec![Diagnostic {
            code: "read_error",
            path: display_path.clone(),
            message: format!("failed to read {}: {error}", full_path.display()),
        }]
    })?;
    let diagnostics = validate_contract(&contract, &display_path);
    if diagnostics.is_empty() {
        Ok(format!("contract={display_path}"))
    } else {
        Err(diagnostics)
    }
}

fn validate_contract(contract: &str, path: &str) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    for group in REQUIRED_GROUPS {
        let missing_phrases = missing_phrases(contract, group.phrases);
        if !missing_phrases.is_empty() {
            diagnostics.push(Diagnostic {
                code: group.code,
                path: path.to_string(),
                message: format!("missing required phrase(s): {}", missing_phrases.join(", ")),
            });
        }
    }
    reject_overclaims(contract, path, &mut diagnostics);
    diagnostics
}

fn missing_phrases(contract: &str, phrases: &[&'static str]) -> Vec<&'static str> {
    phrases
        .iter()
        .copied()
        .filter(|phrase| !contract.contains(phrase))
        .collect()
}

fn reject_overclaims(contract: &str, path: &str, diagnostics: &mut Vec<Diagnostic>) {
    let lowercase_contract = contract.to_ascii_lowercase();
    for rejected in REJECTED_PHRASES {
        if lowercase_contract.contains(rejected.lowercase_phrase) {
            diagnostics.push(Diagnostic {
                code: rejected.code,
                path: path.to_string(),
                message: format!("rejected overclaim: {}", rejected.lowercase_phrase),
            });
        }
    }
}

fn run_self_test() -> Result<String, Vec<Diagnostic>> {
    let path = DEFAULT_CONTRACT_PATH;
    let positive_diagnostics = validate_contract(SELF_TEST_VALID_CONTRACT, path);
    if !positive_diagnostics.is_empty() {
        return Err(vec![Diagnostic {
            code: "self_test_positive_failed",
            path: path.to_string(),
            message: format!("valid fixture failed: {positive_diagnostics:?}"),
        }]);
    }

    let mut negative_count = 0;
    for test in NEGATIVE_REMOVAL_SELF_TESTS {
        let invalid_contract = SELF_TEST_VALID_CONTRACT.replace(test.phrase_to_remove, "");
        expect_diagnostic(test.name, &invalid_contract, path, test.expected_code)?;
        negative_count += ARGUMENT_STEP;
    }

    for test in NEGATIVE_OVERCLAIM_SELF_TESTS {
        let invalid_contract = format!("{SELF_TEST_VALID_CONTRACT}\n{}\n", test.phrase_to_append);
        expect_diagnostic(test.name, &invalid_contract, path, test.expected_code)?;
        negative_count += ARGUMENT_STEP;
    }

    Ok(format!(
        "positive complete contract and {negative_count} negative cases exercised"
    ))
}

fn expect_diagnostic(
    name: &str,
    contract: &str,
    path: &str,
    expected_code: &str,
) -> Result<(), Vec<Diagnostic>> {
    let diagnostics = validate_contract(contract, path);
    if diagnostics
        .iter()
        .any(|diagnostic| diagnostic.code == expected_code)
    {
        Ok(())
    } else {
        Err(vec![Diagnostic {
            code: "self_test_negative_failed",
            path: path.to_string(),
            message: format!(
                "negative fixture '{name}' did not produce {expected_code}; diagnostics: {diagnostics:?}"
            ),
        }])
    }
}

fn print_diagnostics(diagnostics: &[Diagnostic]) {
    for diagnostic in diagnostics {
        eprintln!(
            "{}: {}: {}",
            diagnostic.path, diagnostic.code, diagnostic.message
        );
    }
}
