#!/usr/bin/env -S CARGO_TARGET_DIR=target/check-furnace-smelting-behavior-card-script nix shell "github:nix-community/fenix?rev=092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain" nixpkgs#gcc -c cargo -q -Zscript

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const ROOT_FLAG: &str = "--root";
const SELF_TEST_FLAG: &str = "--self-test";
const DEFAULT_ROOT: &str = ".";
const CARD_PATH: &str = "docs/furnace-smelting-behavior-card.md";
const SUCCESS_MESSAGE: &str = "furnace smelting behavior card check passed";
const SELF_TEST_SUCCESS_MESSAGE: &str = "furnace smelting behavior card self-test passed";
const SUCCESS: ExitCode = ExitCode::SUCCESS;
const FAILURE: ExitCode = ExitCode::FAILURE;

const REQUIRED_GROUPS: &[RequiredGroup] = &[
    RequiredGroup {
        code: "card_missing_source_scope",
        phrases: &[
            "## Source pages",
            "Retrieval date",
            "https://minecraft.wiki/w/Smelting",
            "https://minecraft.wiki/w/Block_entity",
            "https://minecraft.wiki/w/Java_Edition_1.20.1",
            "untrusted external guidance",
        ],
    },
    RequiredGroup {
        code: "card_missing_target_scope",
        phrases: &[
            "## Target scope",
            "Java Edition",
            "1.20.1 / protocol 763",
            "target-version recipe and fuel extraction",
            "no default Valence plugin membership change",
        ],
    },
    RequiredGroup {
        code: "card_missing_bounded_claim",
        phrases: &[
            "## Bounded claim",
            "selected standard furnace row",
            "valid fuel source",
            "output slot capacity",
            "does not implement the rule core",
            "does not claim vanilla parity",
        ],
    },
    RequiredGroup {
        code: "card_missing_non_claims",
        phrases: &[
            "## Non-claims",
            "No broad Minecraft compatibility",
            "No broad vanilla parity",
            "No all-recipe breadth",
            "No all block entities",
            "No public-server safety",
            "No production readiness",
            "No DefaultPlugins membership change",
        ],
    },
    RequiredGroup {
        code: "card_missing_pure_core_boundary",
        phrases: &[
            "## Pure rule core",
            "FurnaceState",
            "RecipeTable",
            "FuelTable",
            "FurnaceTransition",
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
            "block-entity tick phase",
            "before client-visible inventory",
            "does not parse data packs",
        ],
    },
    RequiredGroup {
        code: "card_missing_positive_negative_tests",
        phrases: &[
            "## Tests",
            "Positive tests",
            "Negative tests",
            "PausedNoRecipe",
            "PausedNoFuel",
            "PausedOutputBlocked",
            "Malformed extracted recipe row",
        ],
    },
    RequiredGroup {
        code: "card_missing_evidence_requirements",
        phrases: &[
            "## Evidence",
            "Focused behavior-card validation",
            "Extracted-data check",
            "Paper/vanilla receipt",
            "mc-compat row",
            "behavior-card-only package",
        ],
    },
    RequiredGroup {
        code: "card_missing_stop_conditions",
        phrases: &[
            "## Stop conditions before broader work",
            "Stop before all-recipe breadth",
            "Stop before smoker and blast-furnace category claims",
            "Stop before hoppers, XP, recipe book, data packs, or chunk-unload behavior",
            "Stop before default plugin membership changes",
        ],
    },
];

const SELF_TEST_VALID_CARD: &str = r#"# Behavior card: furnace smelting selected row

## Source pages
Retrieval date: 2026-07-01.
- Smelting: https://minecraft.wiki/w/Smelting
- Block entity: https://minecraft.wiki/w/Block_entity
- Java Edition 1.20.1: https://minecraft.wiki/w/Java_Edition_1.20.1
These pages are untrusted external guidance.

## Target scope
Java Edition 1.20.1 / protocol 763 with target-version recipe and fuel extraction and no default Valence plugin membership change.

## Bounded claim
A selected standard furnace row advances with a valid fuel source and output slot capacity. This card does not implement the rule core and does not claim vanilla parity.

## Non-claims
No broad Minecraft compatibility. No broad vanilla parity. No all-recipe breadth. No all block entities. No public-server safety. No production readiness. No DefaultPlugins membership change.

## Pure rule core
FurnaceState plus RecipeTable and FuelTable return a FurnaceTransition. The core must not read files, mutate Bevy world state, or depend on wall-clock time.

## Thin Bevy/ECS shell
Resources/components/events owned. Schedule phase and ordering. Run in a block-entity tick phase before client-visible inventory updates. The shell does not parse data packs.

## Tests
Positive tests. Negative tests. PausedNoRecipe. PausedNoFuel. PausedOutputBlocked. Malformed extracted recipe row.

## Evidence
Focused behavior-card validation. Extracted-data check. Paper/vanilla receipt. mc-compat row. Evidence in this behavior-card-only package is card completeness only.

## Stop conditions before broader work
Stop before all-recipe breadth. Stop before smoker and blast-furnace category claims. Stop before hoppers, XP, recipe book, data packs, or chunk-unload behavior. Stop before default plugin membership changes.
"#;

const NEGATIVE_SELF_TESTS: &[NegativeSelfTest] = &[
    NegativeSelfTest {
        name: "missing source URL",
        phrase_to_remove: "https://minecraft.wiki/w/Smelting",
        expected_code: "card_missing_source_scope",
    },
    NegativeSelfTest {
        name: "missing target protocol",
        phrase_to_remove: "1.20.1 / protocol 763",
        expected_code: "card_missing_target_scope",
    },
    NegativeSelfTest {
        name: "missing bounded claim",
        phrase_to_remove: "selected standard furnace row",
        expected_code: "card_missing_bounded_claim",
    },
    NegativeSelfTest {
        name: "missing non-claim",
        phrase_to_remove: "No broad vanilla parity",
        expected_code: "card_missing_non_claims",
    },
    NegativeSelfTest {
        name: "missing pure core input",
        phrase_to_remove: "FurnaceState",
        expected_code: "card_missing_pure_core_boundary",
    },
    NegativeSelfTest {
        name: "missing shell schedule",
        phrase_to_remove: "block-entity tick phase",
        expected_code: "card_missing_ecs_shell_boundary",
    },
    NegativeSelfTest {
        name: "missing negative tests",
        phrase_to_remove: "Negative tests",
        expected_code: "card_missing_positive_negative_tests",
    },
    NegativeSelfTest {
        name: "missing parity evidence",
        phrase_to_remove: "Paper/vanilla receipt",
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
        Ok(String::from(SUCCESS_MESSAGE))
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
        let diagnostics = validate_card(&invalid_card);
        let saw_expected_code = diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == test.expected_code);
        if !saw_expected_code {
            return Err(vec![Diagnostic {
                code: "self_test_negative_failed",
                path: CARD_PATH,
                message: format!(
                    "negative fixture '{}' did not produce {}; diagnostics: {diagnostics:?}",
                    test.name, test.expected_code
                ),
            }]);
        }
    }

    Ok(())
}

fn print_diagnostics(diagnostics: &[Diagnostic]) {
    for diagnostic in diagnostics {
        eprintln!("{}: {}: {}", diagnostic.path, diagnostic.code, diagnostic.message);
    }
}
