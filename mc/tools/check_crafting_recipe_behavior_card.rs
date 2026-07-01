#!/usr/bin/env -S CARGO_TARGET_DIR=target/check-crafting-recipe-behavior-card-script nix shell "github:nix-community/fenix?rev=092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain" nixpkgs#gcc -c cargo -q -Zscript

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const ROOT_FLAG: &str = "--root";
const SELF_TEST_FLAG: &str = "--self-test";
const DEFAULT_ROOT: &str = ".";
const CARD_PATH: &str = "docs/crafting-recipe-behavior-card.md";
const SELECTED_MATRIX_SUMMARY: &str = "selected matrix shaped=minecraft:chest shapeless=minecraft:oak_planks invalid=minecraft:stick_insufficient_input_rejection collection=primary-click";
const SUCCESS_MESSAGE: &str = "crafting recipe behavior card check passed";
const SELF_TEST_SUCCESS_MESSAGE: &str = "crafting recipe behavior card self-test passed";
const SUCCESS: ExitCode = ExitCode::SUCCESS;
const FAILURE: ExitCode = ExitCode::FAILURE;

const REQUIRED_GROUPS: &[RequiredGroup] = &[
    RequiredGroup {
        code: "card_missing_source_scope",
        phrases: &[
            "## Source pages",
            "Retrieval date: 2026-07-01",
            "https://minecraft.wiki/w/Crafting",
            "https://minecraft.wiki/w/Recipe_(Java_Edition)",
            "https://minecraft.wiki/w/Java_Edition_1.20.1",
            "https://minecraft.wiki/w/Protocol_version",
            "untrusted external guidance",
            "version-drift risk",
        ],
    },
    RequiredGroup {
        code: "card_missing_target_scope",
        phrases: &[
            "## Target scope",
            "Java Edition",
            "1.20.1 / protocol 763",
            "target-version recipe JSON",
            "no default Valence plugin membership change",
        ],
    },
    RequiredGroup {
        code: "card_missing_selected_recipe_matrix",
        phrases: &[
            "## Selected recipe matrix",
            "minecraft:chest",
            "minecraft:oak_planks",
            "minecraft:stick_insufficient_input_rejection",
            "primary-click collection mode",
            "predecessor receipt evidence",
        ],
    },
    RequiredGroup {
        code: "card_missing_bounded_claim",
        phrases: &[
            "## Bounded claim",
            "selected crafting recipe matrix",
            "does not implement the rule core",
            "does not claim vanilla parity",
        ],
    },
    RequiredGroup {
        code: "card_missing_non_claims",
        phrases: &[
            "## Non-claims",
            "No all-recipe breadth",
            "No arbitrary collection modes",
            "No shift-click, drag, or split breadth",
            "No data-pack loading",
            "No recipe-book UI behavior",
            "No automated crafter behavior",
            "No DefaultPlugins membership change",
            "No broad vanilla parity",
            "No public-server safety",
            "No production readiness",
        ],
    },
    RequiredGroup {
        code: "card_missing_pure_core_boundary",
        phrases: &[
            "## Pure recipe core",
            "CraftingGridState",
            "SelectedRecipeRow",
            "RecipeMatrix",
            "OutputSlotState",
            "CollectionRequest",
            "CraftingDecision",
            "MalformedRecipeError",
            "must not read files",
            "mutate Bevy world state",
            "wall-clock time",
        ],
    },
    RequiredGroup {
        code: "card_missing_ecs_shell_boundary",
        phrases: &[
            "## Thin Bevy/ECS shell",
            "Resources/components/events owned",
            "Schedule phase and ordering",
            "inventory click",
            "before client-visible inventory",
            "disabled-plugin behavior",
            "does not parse data packs",
        ],
    },
    RequiredGroup {
        code: "card_missing_positive_tests",
        phrases: &[
            "Positive tests",
            "shaped chest",
            "shapeless oak-planks",
            "primary-click collection",
        ],
    },
    RequiredGroup {
        code: "card_missing_negative_tests",
        phrases: &[
            "Negative tests",
            "insufficient stick input",
            "output slot blocked",
            "malformed recipe row",
            "unsupported collection mode",
        ],
    },
    RequiredGroup {
        code: "card_missing_evidence_requirements",
        phrases: &[
            "## Evidence",
            "Focused behavior-card validation",
            "Extracted-data check",
            "Paper/reference and Valence receipts",
            "survival-crafting-recipe-breadth",
            "typed-event migration evidence",
            "behavior-card-only package",
        ],
    },
    RequiredGroup {
        code: "card_missing_stop_conditions",
        phrases: &[
            "## Stop conditions before broader work",
            "Stop before all-recipe breadth",
            "Stop before data-pack loading",
            "Stop before recipe-book UI behavior",
            "Stop before automated crafter behavior",
            "Stop before default plugin membership changes",
        ],
    },
];

const FORBIDDEN_CLAIMS: &[ForbiddenClaim] = &[
    ForbiddenClaim {
        code: "card_overclaims_all_recipes",
        phrase: "Claims all recipes",
    },
    ForbiddenClaim {
        code: "card_overclaims_arbitrary_collection_modes",
        phrase: "arbitrary collection modes are supported",
    },
    ForbiddenClaim {
        code: "card_overclaims_shift_drag_split",
        phrase: "shift-click, drag, and split are supported",
    },
    ForbiddenClaim {
        code: "card_overclaims_data_pack_loading",
        phrase: "data-pack loading is supported",
    },
    ForbiddenClaim {
        code: "card_overclaims_recipe_book_ui",
        phrase: "recipe-book UI behavior is supported",
    },
    ForbiddenClaim {
        code: "card_overclaims_automated_crafter",
        phrase: "automated crafter behavior is supported",
    },
    ForbiddenClaim {
        code: "card_overclaims_default_plugins",
        phrase: "DefaultPlugins membership is changed",
    },
    ForbiddenClaim {
        code: "card_overclaims_broad_vanilla_parity",
        phrase: "claims broad vanilla parity",
    },
    ForbiddenClaim {
        code: "card_overclaims_public_server_safety",
        phrase: "public-server safety is proven",
    },
    ForbiddenClaim {
        code: "card_overclaims_production_readiness",
        phrase: "production readiness is proven",
    },
];

const SELF_TEST_VALID_CARD: &str = r#"# Behavior card: crafting recipe selected matrix

## Source pages
Retrieval date: 2026-07-01.
- Crafting: https://minecraft.wiki/w/Crafting
- Recipe (Java Edition): https://minecraft.wiki/w/Recipe_(Java_Edition)
- Java Edition 1.20.1: https://minecraft.wiki/w/Java_Edition_1.20.1
- Protocol version: https://minecraft.wiki/w/Protocol_version
These pages are untrusted external guidance and carry version-drift risk.

## Target scope
Java Edition 1.20.1 / protocol 763 with target-version recipe JSON and no default Valence plugin membership change.

## Selected recipe matrix
minecraft:chest, minecraft:oak_planks, minecraft:stick_insufficient_input_rejection, primary-click collection mode, predecessor receipt evidence.

## Bounded claim
A selected crafting recipe matrix may be validated later. This card does not implement the rule core and does not claim vanilla parity.

## Non-claims
No all-recipe breadth. No arbitrary collection modes. No shift-click, drag, or split breadth. No data-pack loading. No recipe-book UI behavior. No automated crafter behavior. No DefaultPlugins membership change. No broad vanilla parity. No public-server safety. No production readiness.

## Pure recipe core
CraftingGridState plus SelectedRecipeRow inside a RecipeMatrix and OutputSlotState with CollectionRequest return a CraftingDecision or MalformedRecipeError. The core must not read files, mutate Bevy world state, or depend on wall-clock time.

## Thin Bevy/ECS shell
Resources/components/events owned. Schedule phase and ordering. Runs in an inventory click phase before client-visible inventory updates. Covers disabled-plugin behavior and does not parse data packs.

## Tests
Positive tests: shaped chest, shapeless oak-planks, primary-click collection.
Negative tests: insufficient stick input, output slot blocked, malformed recipe row, unsupported collection mode.

## Evidence
Focused behavior-card validation. Extracted-data check. Paper/reference and Valence receipts. survival-crafting-recipe-breadth. typed-event migration evidence. Evidence in this behavior-card-only package is card completeness only.

## Stop conditions before broader work
Stop before all-recipe breadth. Stop before data-pack loading. Stop before recipe-book UI behavior. Stop before automated crafter behavior. Stop before default plugin membership changes.
"#;

const NEGATIVE_SELF_TESTS: &[NegativeSelfTest] = &[
    NegativeSelfTest {
        name: "missing source URL",
        phrase_to_remove: "https://minecraft.wiki/w/Crafting",
        expected_code: "card_missing_source_scope",
    },
    NegativeSelfTest {
        name: "missing target protocol",
        phrase_to_remove: "1.20.1 / protocol 763",
        expected_code: "card_missing_target_scope",
    },
    NegativeSelfTest {
        name: "missing selected recipe matrix",
        phrase_to_remove: "minecraft:chest",
        expected_code: "card_missing_selected_recipe_matrix",
    },
    NegativeSelfTest {
        name: "missing bounded claim",
        phrase_to_remove: "selected crafting recipe matrix",
        expected_code: "card_missing_bounded_claim",
    },
    NegativeSelfTest {
        name: "missing non-claim",
        phrase_to_remove: "No all-recipe breadth",
        expected_code: "card_missing_non_claims",
    },
    NegativeSelfTest {
        name: "missing pure core input",
        phrase_to_remove: "CraftingGridState",
        expected_code: "card_missing_pure_core_boundary",
    },
    NegativeSelfTest {
        name: "missing shell boundary",
        phrase_to_remove: "inventory click",
        expected_code: "card_missing_ecs_shell_boundary",
    },
    NegativeSelfTest {
        name: "missing positive tests",
        phrase_to_remove: "Positive tests",
        expected_code: "card_missing_positive_tests",
    },
    NegativeSelfTest {
        name: "missing negative tests",
        phrase_to_remove: "Negative tests",
        expected_code: "card_missing_negative_tests",
    },
    NegativeSelfTest {
        name: "missing evidence requirements",
        phrase_to_remove: "Focused behavior-card validation",
        expected_code: "card_missing_evidence_requirements",
    },
    NegativeSelfTest {
        name: "missing stop condition",
        phrase_to_remove: "Stop before all-recipe breadth",
        expected_code: "card_missing_stop_conditions",
    },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RequiredGroup {
    code: &'static str,
    phrases: &'static [&'static str],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ForbiddenClaim {
    code: &'static str,
    phrase: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct NegativeSelfTest {
    name: &'static str,
    phrase_to_remove: &'static str,
    expected_code: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Command {
    root: PathBuf,
    self_test: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Diagnostic {
    code: &'static str,
    path: &'static str,
    message: String,
}

fn main() -> ExitCode {
    let command = match parse_args() {
        Ok(command) => command,
        Err(error) => {
            eprintln!("{error}");
            return FAILURE;
        }
    };

    match run(command) {
        Ok(message) => {
            println!("{message}");
            SUCCESS
        }
        Err(diagnostics) => {
            print_diagnostics(&diagnostics);
            FAILURE
        }
    }
}

fn parse_args() -> Result<Command, String> {
    let mut args = env::args().skip(1);
    let mut root = PathBuf::from(DEFAULT_ROOT);
    let mut self_test = false;

    while let Some(arg) = args.next() {
        if arg == ROOT_FLAG {
            let Some(value) = args.next() else {
                return Err(format!("{ROOT_FLAG} requires a path"));
            };
            root = PathBuf::from(value);
        } else if arg == SELF_TEST_FLAG {
            self_test = true;
        } else {
            return Err(format!("unknown argument: {arg}"));
        }
    }

    Ok(Command { root, self_test })
}

fn run(command: Command) -> Result<String, Vec<Diagnostic>> {
    if command.self_test {
        run_self_test()?;
        return Ok(String::from(SELF_TEST_SUCCESS_MESSAGE));
    }

    let card = load_card(&command.root).map_err(|message| {
        vec![Diagnostic {
            code: "read_error",
            path: CARD_PATH,
            message,
        }]
    })?;

    let diagnostics = validate_card(&card);
    if diagnostics.is_empty() {
        Ok(format!("{SUCCESS_MESSAGE}: {SELECTED_MATRIX_SUMMARY}"))
    } else {
        Err(diagnostics)
    }
}

fn load_card(root: &Path) -> Result<String, String> {
    let path = root.join(CARD_PATH);
    fs::read_to_string(&path).map_err(|error| format!("failed to read {}: {error}", path.display()))
}

fn validate_card(card: &str) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    for group in REQUIRED_GROUPS {
        let missing_phrases = missing_phrases(card, group.phrases);
        if !missing_phrases.is_empty() {
            diagnostics.push(Diagnostic {
                code: group.code,
                path: CARD_PATH,
                message: format!("missing required phrase(s): {}", missing_phrases.join(", ")),
            });
        }
    }

    for claim in FORBIDDEN_CLAIMS {
        if card.contains(claim.phrase) {
            diagnostics.push(Diagnostic {
                code: claim.code,
                path: CARD_PATH,
                message: format!("rejected overclaim phrase: {}", claim.phrase),
            });
        }
    }

    diagnostics
}

fn missing_phrases(card: &str, phrases: &[&'static str]) -> Vec<&'static str> {
    phrases
        .iter()
        .copied()
        .filter(|phrase| !card.contains(phrase))
        .collect()
}

fn run_self_test() -> Result<(), Vec<Diagnostic>> {
    let positive_diagnostics = validate_card(SELF_TEST_VALID_CARD);
    if !positive_diagnostics.is_empty() {
        return Err(vec![Diagnostic {
            code: "self_test_positive_failed",
            path: CARD_PATH,
            message: format!("valid fixture failed: {positive_diagnostics:?}"),
        }]);
    }

    for test in NEGATIVE_SELF_TESTS {
        let invalid_card = SELF_TEST_VALID_CARD.replace(test.phrase_to_remove, "");
        expect_diagnostic(test.name, &invalid_card, test.expected_code)?;
    }

    for claim in FORBIDDEN_CLAIMS {
        let invalid_card = format!("{SELF_TEST_VALID_CARD}\n{}\n", claim.phrase);
        expect_diagnostic(claim.phrase, &invalid_card, claim.code)?;
    }

    Ok(())
}

fn expect_diagnostic(
    fixture_name: &str,
    fixture_text: &str,
    expected_code: &'static str,
) -> Result<(), Vec<Diagnostic>> {
    let diagnostics = validate_card(fixture_text);
    let saw_expected_code = diagnostics
        .iter()
        .any(|diagnostic| diagnostic.code == expected_code);
    if saw_expected_code {
        Ok(())
    } else {
        Err(vec![Diagnostic {
            code: "self_test_negative_failed",
            path: CARD_PATH,
            message: format!(
                "negative fixture '{fixture_name}' did not produce {expected_code}; diagnostics: {diagnostics:?}"
            ),
        }])
    }
}

fn print_diagnostics(diagnostics: &[Diagnostic]) {
    for diagnostic in diagnostics {
        eprintln!("{}: {}: {}", diagnostic.path, diagnostic.code, diagnostic.message);
    }
}
