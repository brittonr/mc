#!/usr/bin/env -S CARGO_TARGET_DIR=target/check-check-tiers-script nix shell "github:nix-community/fenix?rev=092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain" nixpkgs#gcc -c cargo -q -Zscript

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const ROOT_FLAG: &str = "--root";
const SELF_TEST_FLAG: &str = "--self-test";
const DEFAULT_ROOT: &str = ".";
const CHECK_TIERS_PATH: &str = "docs/check-tiers.md";
const README_PATH: &str = "README.md";
const AGENTS_PATH: &str = "AGENTS.md";
const SUCCESS_MESSAGE: &str = "check tier docs passed";
const SELF_TEST_SUCCESS_MESSAGE: &str = "check tier docs self-test passed";
const SUCCESS: ExitCode = ExitCode::SUCCESS;
const FAILURE: ExitCode = ExitCode::FAILURE;

const TIER_IDS: &[&str] = &[
    "tier.docs-layout",
    "tier.generated",
    "tier.evidence",
    "tier.component",
    "tier.live-manual",
    "tier.archive",
];

const REQUIRED_COMMANDS: &[&str] = &[
    "tools/check_layout_boundaries.rs --self-test",
    "nix build .#checks.x86_64-linux.mc-compat-layout-boundaries --no-link -L",
    "tools/check_check_tiers.rs --self-test",
    "nix run .#cairn -- validate --root .",
    "nix run .#evidence-manifest-refresh -- --check",
    "tools/check_evidence_partitions.rs --self-test",
    "tools/check_evidence_partitions.rs --root .",
    "nix build .#checks.x86_64-linux.mc-compat-cairn-task-evidence --no-link -L",
    "nix build .#checks.x86_64-linux.mc-compat-evidence-manifest-refresh --no-link -L",
    "nix build .#checks.x86_64-linux.mc-compat-scenario-manifest --no-link -L",
    "nix run .#cairn -- policy export --check",
    "nix run .#stevenarella -- --dry-run",
    "nix run .#valence -- --dry-run",
    "nix run .#mc-compat-smoke -- --dry-run --server-backend valence --scenario smoke",
    "nix build .#checks.x86_64-linux.mc-compat-checker-framework --no-link -L",
    "nix build .#checks.x86_64-linux.mc-compat-current-evidence-bundle --no-link -L",
    "nix run .#mc-compat-smoke -- --run --server-backend paper --scenario <scenario>",
    "nix run .#mc-compat-smoke -- --run --server-backend valence --scenario <scenario>",
];

const REQUIRED_PHRASES: &[&str] = &[
    "Use the smallest tier",
    "Non-claims",
    "Inventory by tier",
    "Selecting the smallest sufficient tier",
    "Freshness contract",
    "does not add wrapper outputs or change evidence semantics",
];

#[derive(Debug, Clone)]
struct TierDocs {
    check_tiers: String,
    readme: String,
    agents: String,
}

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

fn push_missing(diagnostics: &mut Vec<Diagnostic>, code: &'static str, path: &str, items: &[&str]) {
    if items.is_empty() {
        return;
    }

    diagnostics.push(Diagnostic {
        code,
        message: format!("{path} is missing: {}", items.join(", ")),
    });
}

fn check_tier_docs(docs: &TierDocs) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    push_missing(
        &mut diagnostics,
        "missing_tier_ids",
        CHECK_TIERS_PATH,
        &missing_items(&docs.check_tiers, TIER_IDS),
    );
    push_missing(
        &mut diagnostics,
        "missing_required_commands",
        CHECK_TIERS_PATH,
        &missing_items(&docs.check_tiers, REQUIRED_COMMANDS),
    );
    push_missing(
        &mut diagnostics,
        "missing_required_phrases",
        CHECK_TIERS_PATH,
        &missing_items(&docs.check_tiers, REQUIRED_PHRASES),
    );
    push_missing(
        &mut diagnostics,
        "readme_missing_check_tiers_link",
        README_PATH,
        &missing_items(&docs.readme, &[CHECK_TIERS_PATH]),
    );
    push_missing(
        &mut diagnostics,
        "agents_missing_check_tiers_link",
        AGENTS_PATH,
        &missing_items(&docs.agents, &[CHECK_TIERS_PATH]),
    );

    diagnostics
}

fn read_to_string(root: &Path, relative: &str) -> Result<String, String> {
    let path = root.join(relative);
    fs::read_to_string(&path).map_err(|error| format!("failed to read {}: {error}", path.display()))
}

fn load_docs(root: &Path) -> Result<TierDocs, String> {
    Ok(TierDocs {
        check_tiers: read_to_string(root, CHECK_TIERS_PATH)?,
        readme: read_to_string(root, README_PATH)?,
        agents: read_to_string(root, AGENTS_PATH)?,
    })
}

fn fixture_doc() -> String {
    let mut parts = Vec::new();
    parts.extend(TIER_IDS.iter().copied());
    parts.extend(REQUIRED_COMMANDS.iter().copied());
    parts.extend(REQUIRED_PHRASES.iter().copied());
    parts.join("\n")
}

fn fixture_valid_docs() -> TierDocs {
    TierDocs {
        check_tiers: fixture_doc(),
        readme: String::from(CHECK_TIERS_PATH),
        agents: String::from(CHECK_TIERS_PATH),
    }
}

fn run_self_test() -> Result<(), String> {
    let valid = fixture_valid_docs();
    let valid_diagnostics = check_tier_docs(&valid);
    if !valid_diagnostics.is_empty() {
        return Err(format!(
            "positive fixture unexpectedly failed: {:?}",
            valid_diagnostics
        ));
    }

    let mut missing_tier = valid.clone();
    missing_tier.check_tiers = missing_tier
        .check_tiers
        .replace("tier.live-manual", "tier_live_manual_missing");
    let tier_diagnostics = check_tier_docs(&missing_tier);
    if !tier_diagnostics
        .iter()
        .any(|diagnostic| diagnostic.code == "missing_tier_ids")
    {
        return Err(format!(
            "negative fixture did not report missing tier ID: {:?}",
            tier_diagnostics
        ));
    }

    let mut missing_command = valid.clone();
    missing_command.check_tiers = missing_command.check_tiers.replace(
        "nix run .#stevenarella -- --dry-run",
        "stevenarella omitted",
    );
    let command_diagnostics = check_tier_docs(&missing_command);
    if !command_diagnostics
        .iter()
        .any(|diagnostic| diagnostic.code == "missing_required_commands")
    {
        return Err(format!(
            "negative fixture did not report missing command: {:?}",
            command_diagnostics
        ));
    }

    let mut missing_link = valid;
    missing_link.readme = String::from("no tier link");
    let link_diagnostics = check_tier_docs(&missing_link);
    if !link_diagnostics
        .iter()
        .any(|diagnostic| diagnostic.code == "readme_missing_check_tiers_link")
    {
        return Err(format!(
            "negative fixture did not report missing README link: {:?}",
            link_diagnostics
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

    let docs = load_docs(&command.root)?;
    let diagnostics = check_tier_docs(&docs);
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
