#!/usr/bin/env -S nix shell "github:nix-community/fenix?rev=092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain" nixpkgs#gcc -c cargo -q -Zscript

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const ROOT_FLAG: &str = "--root";
const SELF_TEST_FLAG: &str = "--self-test";
const DEFAULT_ROOT: &str = ".";
const DOC_PATH: &str = "docs/hyperion-integration-boundaries.md";
const SUCCESS_MESSAGE: &str = "hyperion boundary docs passed";
const SELF_TEST_SUCCESS_MESSAGE: &str = "hyperion boundary docs self-test passed";
const SUCCESS: ExitCode = ExitCode::SUCCESS;
const FAILURE: ExitCode = ExitCode::FAILURE;

const REQUIRED_CLASSIFICATIONS: &[&str] = &["adopt", "port", "reference", "reject"];
const REQUIRED_SECTIONS: &[&str] = &[
    "Inventory template",
    "Classification rules",
    "Forbidden core-merge categories",
    "Gameplay and optional-plugin boundary",
    "Review gate checklist",
    "Positive and negative examples",
];
const FORBIDDEN_CATEGORIES: &[&str] = &[
    "Bedwars-specific game logic",
    "Full Hyperion runtime replacement",
    "Custom combat as Valence core behavior",
    "Unaudited nightly-only, unsafe-heavy",
];
const REQUIRED_NON_CLAIMS: &[&str] = &[
    "production-scale",
    "vanilla-parity",
    "Hyperion-compatibility",
    "default-behavior",
    "safety claims",
];
const REQUIRED_EXAMPLES: &[&str] = &[
    "Positive: reference-only routing idea",
    "Positive: port stable pure math helper",
    "Negative: direct Bedwars import",
    "Negative: unaudited unsafe runtime copy",
];

#[derive(Debug, Clone, PartialEq, Eq)]
struct Diagnostic {
    code: &'static str,
    message: String,
}

fn missing_items<'a>(text: &str, items: &'a [&str]) -> Vec<&'a str> {
    items
        .iter()
        .copied()
        .filter(|item| !text.contains(item))
        .collect()
}

fn push_missing(diagnostics: &mut Vec<Diagnostic>, code: &'static str, items: &[&str]) {
    if items.is_empty() {
        return;
    }

    diagnostics.push(Diagnostic {
        code,
        message: format!("{DOC_PATH} is missing: {}", items.join(", ")),
    });
}

fn check_boundary_doc(text: &str) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    push_missing(
        &mut diagnostics,
        "missing_sections",
        &missing_items(text, REQUIRED_SECTIONS),
    );
    push_missing(
        &mut diagnostics,
        "missing_classifications",
        &missing_items(text, REQUIRED_CLASSIFICATIONS),
    );
    push_missing(
        &mut diagnostics,
        "missing_forbidden_categories",
        &missing_items(text, FORBIDDEN_CATEGORIES),
    );
    push_missing(
        &mut diagnostics,
        "missing_non_claims",
        &missing_items(text, REQUIRED_NON_CLAIMS),
    );
    push_missing(
        &mut diagnostics,
        "missing_examples",
        &missing_items(text, REQUIRED_EXAMPLES),
    );

    diagnostics
}

fn read_doc(root: &Path) -> Result<String, String> {
    let path = root.join(DOC_PATH);
    fs::read_to_string(&path).map_err(|error| format!("failed to read {}: {error}", path.display()))
}

fn fixture_valid_doc() -> String {
    let mut parts = Vec::new();
    parts.extend(REQUIRED_SECTIONS.iter().copied());
    parts.extend(REQUIRED_CLASSIFICATIONS.iter().copied());
    parts.extend(FORBIDDEN_CATEGORIES.iter().copied());
    parts.extend(REQUIRED_NON_CLAIMS.iter().copied());
    parts.extend(REQUIRED_EXAMPLES.iter().copied());
    parts.join("\n")
}

fn run_self_test() -> Result<(), String> {
    let valid = fixture_valid_doc();
    let valid_diagnostics = check_boundary_doc(&valid);
    if !valid_diagnostics.is_empty() {
        return Err(format!(
            "positive fixture unexpectedly failed: {:?}",
            valid_diagnostics
        ));
    }

    let missing_forbidden = valid.replace("Bedwars-specific game logic", "Bedwars omitted");
    let forbidden_diagnostics = check_boundary_doc(&missing_forbidden);
    if !forbidden_diagnostics
        .iter()
        .any(|diagnostic| diagnostic.code == "missing_forbidden_categories")
    {
        return Err(format!(
            "negative fixture did not report missing forbidden category: {:?}",
            forbidden_diagnostics
        ));
    }

    let missing_example = valid.replace(
        "Negative: direct Bedwars import",
        "Negative example omitted",
    );
    let example_diagnostics = check_boundary_doc(&missing_example);
    if !example_diagnostics
        .iter()
        .any(|diagnostic| diagnostic.code == "missing_examples")
    {
        return Err(format!(
            "negative fixture did not report missing example: {:?}",
            example_diagnostics
        ));
    }

    Ok(())
}

#[derive(Debug, Clone)]
struct Command {
    root: PathBuf,
    self_test: bool,
}

fn parse_args() -> Result<Command, String> {
    let mut args = env::args().skip(1);
    let mut root = PathBuf::from(DEFAULT_ROOT);
    let mut self_test = false;

    while let Some(arg) = args.next() {
        if arg == ROOT_FLAG {
            let value = args
                .next()
                .ok_or_else(|| format!("{ROOT_FLAG} requires a path"))?;
            root = PathBuf::from(value);
        } else if arg == SELF_TEST_FLAG {
            self_test = true;
        } else {
            return Err(format!("unknown argument: {arg}"));
        }
    }

    Ok(Command { root, self_test })
}

fn run(command: Command) -> Result<String, String> {
    if command.self_test {
        run_self_test()?;
        return Ok(String::from(SELF_TEST_SUCCESS_MESSAGE));
    }

    let text = read_doc(&command.root)?;
    let diagnostics = check_boundary_doc(&text);
    if diagnostics.is_empty() {
        Ok(String::from(SUCCESS_MESSAGE))
    } else {
        Err(diagnostics
            .into_iter()
            .map(|diagnostic| format!("{}: {}", diagnostic.code, diagnostic.message))
            .collect::<Vec<_>>()
            .join("\n"))
    }
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
        Err(error) => {
            eprintln!("{error}");
            FAILURE
        }
    }
}
