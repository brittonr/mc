#!/usr/bin/env -S CARGO_TARGET_DIR=target/check-onixresearch-workspace-namespace-script nix shell "github:nix-community/fenix?rev=092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain" nixpkgs#gcc -c cargo -q -Zscript

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const SELF_TEST_FLAG: &str = "--self-test";
const ROOT_FLAG: &str = "--root";
const DEFAULT_ROOT: &str = ".";
const CONTRACT_PATH: &str = "docs/onixresearch-workspace-namespace.md";
const INVENTORY_PATH: &str = "docs/evidence/onixresearch-workspace-namespace-inventory-2026-07-07.md";
const CANONICAL_ROOT_ABSOLUTE: &str = "/home/brittonr/git/OnixResearch";
const LEGACY_ROOT_ABSOLUTE: &str = "/home/brittonr/git";
const COMPAT_LINK_PREFIX: &str = "OnixResearch/";
const SUCCESS_MESSAGE: &str = "OnixResearch workspace namespace checks passed";
const SELF_TEST_SUCCESS_MESSAGE: &str = "OnixResearch workspace namespace self-test passed";
const SUCCESS: ExitCode = ExitCode::SUCCESS;
const FAILURE: ExitCode = ExitCode::FAILURE;

const MIGRATED_REPOS: &[&str] = &["cairn", "valence", "octet", "mantle", "trellis"];
const DEFERRED_REPOS: &[&str] = &["mc"];
const CLASSIFICATIONS: &[&str] = &[
    "active",
    "historical",
    "generated",
    "blocked",
    "removable",
    "intentionally retained",
];
const NON_CLAIMS: &[&str] = &[
    "does not change remotes",
    "does not rewrite history",
    "does not prove release eligibility",
    "does not prove behavioral correctness",
    "does not prove whole-stack safety",
    "does not retire compatibility links",
];
const CONTRACT_PHRASES: &[&str] = &[
    "ONIX_RESEARCH_ROOT",
    CANONICAL_ROOT_ABSOLUTE,
    "temporary compatibility symlink",
    "legacy commands",
    "Do not remove compatibility links",
    "rollback",
    "path-reference inventory freshness",
    "canonical-path command smoke",
    "compatibility-path shell/Git command smoke",
    "selected Nix path-input validation",
];
const INVENTORY_PHRASES: &[&str] = &[
    "Pi skills",
    "Nix path inputs",
    "flake locks",
    "evidence notes",
    "validation commands",
    "scratch scripts",
];

#[derive(Debug, Clone, PartialEq, Eq)]
struct PathFact {
    repo: String,
    legacy_exists: bool,
    legacy_is_symlink: bool,
    legacy_target: Option<String>,
    canonical_exists: bool,
    deferred: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct NamespaceModel {
    contract: String,
    inventory: String,
    path_facts: Vec<PathFact>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Diagnostic {
    code: &'static str,
    subject: String,
    message: String,
}

fn main() -> ExitCode {
    let args = env::args().collect::<Vec<_>>();
    if args.iter().any(|arg| arg == SELF_TEST_FLAG) {
        return match run_self_tests() {
            Ok(()) => {
                println!("{SELF_TEST_SUCCESS_MESSAGE}");
                SUCCESS
            }
            Err(errors) => {
                print_errors(&errors);
                FAILURE
            }
        };
    }

    let root = parse_root(&args).unwrap_or_else(|| PathBuf::from(DEFAULT_ROOT));
    match read_model(&root).and_then(|model| validate_namespace(&model)) {
        Ok(()) => {
            println!("{SUCCESS_MESSAGE}");
            SUCCESS
        }
        Err(errors) => {
            print_errors(&errors);
            FAILURE
        }
    }
}

fn parse_root(args: &[String]) -> Option<PathBuf> {
    args.windows(ARG_PAIR_WIDTH)
        .find(|window| window[ARG_FLAG_INDEX] == ROOT_FLAG)
        .map(|window| PathBuf::from(&window[ARG_VALUE_INDEX]))
}

const ARG_PAIR_WIDTH: usize = 2;
const ARG_FLAG_INDEX: usize = 0;
const ARG_VALUE_INDEX: usize = 1;

fn print_errors(errors: &[Diagnostic]) {
    for error in errors {
        eprintln!(
            "namespace check failed: code={} subject={} message={}",
            error.code, error.subject, error.message
        );
    }
}

fn read_model(root: &Path) -> Result<NamespaceModel, Vec<Diagnostic>> {
    let mut errors = Vec::new();
    let contract = read_required_text(root, CONTRACT_PATH, &mut errors);
    let inventory = read_required_text(root, INVENTORY_PATH, &mut errors);
    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(NamespaceModel {
        contract,
        inventory,
        path_facts: read_path_facts(),
    })
}

fn read_required_text(root: &Path, relative: &str, errors: &mut Vec<Diagnostic>) -> String {
    let path = root.join(relative);
    match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(err) => {
            errors.push(diagnostic(
                "missing_document",
                relative,
                format!("failed to read {}: {err}", path.display()),
            ));
            String::new()
        }
    }
}

fn read_path_facts() -> Vec<PathFact> {
    MIGRATED_REPOS
        .iter()
        .map(|repo| read_path_fact(repo, false))
        .chain(DEFERRED_REPOS.iter().map(|repo| read_path_fact(repo, true)))
        .collect()
}

fn read_path_fact(repo: &str, deferred: bool) -> PathFact {
    let legacy = Path::new(LEGACY_ROOT_ABSOLUTE).join(repo);
    let canonical = Path::new(CANONICAL_ROOT_ABSOLUTE).join(repo);
    let metadata = fs::symlink_metadata(&legacy).ok();
    let legacy_is_symlink = metadata
        .as_ref()
        .map(|data| data.file_type().is_symlink())
        .unwrap_or(false);
    let legacy_target = if legacy_is_symlink {
        fs::read_link(&legacy)
            .ok()
            .map(|path| path.to_string_lossy().to_string())
    } else {
        None
    };

    PathFact {
        repo: repo.to_string(),
        legacy_exists: metadata.is_some(),
        legacy_is_symlink,
        legacy_target,
        canonical_exists: canonical.is_dir(),
        deferred,
    }
}

fn validate_namespace(model: &NamespaceModel) -> Result<(), Vec<Diagnostic>> {
    let mut errors = Vec::new();
    require_phrases(
        &model.contract,
        CONTRACT_PATH,
        CONTRACT_PHRASES,
        "missing_contract_phrase",
        &mut errors,
    );
    require_phrases(
        &model.inventory,
        INVENTORY_PATH,
        INVENTORY_PHRASES,
        "missing_inventory_phrase",
        &mut errors,
    );
    require_phrases(
        &model.inventory,
        INVENTORY_PATH,
        CLASSIFICATIONS,
        "missing_inventory_classification",
        &mut errors,
    );
    require_phrases(
        &model.contract,
        CONTRACT_PATH,
        NON_CLAIMS,
        "missing_non_claim",
        &mut errors,
    );
    require_repo_mentions(model, &mut errors);
    validate_path_facts(&model.path_facts, &mut errors);

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn require_phrases(
    text: &str,
    subject: &str,
    phrases: &[&str],
    code: &'static str,
    errors: &mut Vec<Diagnostic>,
) {
    for phrase in phrases {
        if !text.contains(phrase) {
            errors.push(diagnostic(
                code,
                subject,
                format!("missing required phrase `{phrase}`"),
            ));
        }
    }
}

fn require_repo_mentions(model: &NamespaceModel, errors: &mut Vec<Diagnostic>) {
    for repo in MIGRATED_REPOS.iter().chain(DEFERRED_REPOS.iter()) {
        if !model.contract.contains(repo) {
            errors.push(diagnostic(
                "missing_repo_contract",
                CONTRACT_PATH,
                format!("contract does not mention `{repo}`"),
            ));
        }
        if !model.inventory.contains(repo) {
            errors.push(diagnostic(
                "missing_repo_inventory",
                INVENTORY_PATH,
                format!("inventory does not mention `{repo}`"),
            ));
        }
    }
}

fn validate_path_facts(path_facts: &[PathFact], errors: &mut Vec<Diagnostic>) {
    for fact in path_facts {
        if fact.deferred {
            if !fact.legacy_exists {
                errors.push(diagnostic(
                    "missing_deferred_legacy_path",
                    &fact.repo,
                    "deferred repository must remain available at its legacy path",
                ));
            }
            if fact.legacy_is_symlink {
                errors.push(diagnostic(
                    "unexpected_deferred_symlink",
                    &fact.repo,
                    "deferred repository must not be replaced with a symlink by this staged migration",
                ));
            }
            continue;
        }

        if !fact.canonical_exists {
            errors.push(diagnostic(
                "missing_canonical_path",
                &fact.repo,
                format!("{CANONICAL_ROOT_ABSOLUTE}/{} is missing", fact.repo),
            ));
        }
        if !fact.legacy_exists {
            errors.push(diagnostic(
                "missing_legacy_path",
                &fact.repo,
                format!("{LEGACY_ROOT_ABSOLUTE}/{} is missing", fact.repo),
            ));
        }
        if !fact.legacy_is_symlink {
            errors.push(diagnostic(
                "missing_compatibility_symlink",
                &fact.repo,
                "migrated repository legacy path must be a compatibility symlink",
            ));
        }
        let expected_target = format!("{COMPAT_LINK_PREFIX}{}", fact.repo);
        if fact.legacy_target.as_deref() != Some(expected_target.as_str()) {
            errors.push(diagnostic(
                "wrong_compatibility_symlink_target",
                &fact.repo,
                format!(
                    "legacy target must be `{expected_target}`, got `{:?}`",
                    fact.legacy_target
                ),
            ));
        }
    }
}

fn diagnostic(
    code: &'static str,
    subject: impl Into<String>,
    message: impl Into<String>,
) -> Diagnostic {
    Diagnostic {
        code,
        subject: subject.into(),
        message: message.into(),
    }
}

fn run_self_tests() -> Result<(), Vec<Diagnostic>> {
    let positive = NamespaceModel {
        contract: positive_contract_text(),
        inventory: positive_inventory_text(),
        path_facts: positive_path_facts(),
    };
    validate_namespace(&positive)?;

    expect_failure(
        "missing_non_claim",
        NamespaceModel {
            contract: positive_contract_text().replace("does not rewrite history", "history stays familiar"),
            inventory: positive_inventory_text(),
            path_facts: positive_path_facts(),
        },
    )?;
    expect_failure(
        "missing_inventory_classification",
        NamespaceModel {
            contract: positive_contract_text(),
            inventory: positive_inventory_text().replace("intentionally retained", "kept for now"),
            path_facts: positive_path_facts(),
        },
    )?;
    expect_failure(
        "missing_compatibility_symlink",
        NamespaceModel {
            contract: positive_contract_text(),
            inventory: positive_inventory_text(),
            path_facts: path_facts_with_broken_legacy(),
        },
    )?;

    Ok(())
}

fn expect_failure(expected_code: &'static str, model: NamespaceModel) -> Result<(), Vec<Diagnostic>> {
    match validate_namespace(&model) {
        Ok(()) => Err(vec![diagnostic(
            "negative_fixture_passed",
            expected_code,
            "negative fixture unexpectedly passed",
        )]),
        Err(errors) if errors.iter().any(|error| error.code == expected_code) => Ok(()),
        Err(errors) => Err(errors),
    }
}

fn positive_contract_text() -> String {
    let mut text = String::new();
    text.push_str("ONIX_RESEARCH_ROOT=/home/brittonr/git/OnixResearch\n");
    text.push_str("/home/brittonr/git/OnixResearch is canonical.\n");
    text.push_str("Use temporary compatibility symlink entries so legacy commands keep working.\n");
    text.push_str("Do not remove compatibility links before active consumers are migrated.\n");
    text.push_str("rollback notes are recorded.\n");
    text.push_str("path-reference inventory freshness, canonical-path command smoke, compatibility-path shell/Git command smoke, and selected Nix path-input validation are required.\n");
    text.push_str("cairn valence octet mantle trellis are migrated; mc is deferred.\n");
    for non_claim in NON_CLAIMS {
        text.push_str(non_claim);
        text.push('\n');
    }
    text
}

fn positive_inventory_text() -> String {
    let mut text = String::new();
    text.push_str("Pi skills, Nix path inputs, flake locks, evidence notes, validation commands, and scratch scripts were inventoried.\n");
    text.push_str("cairn valence octet mantle trellis moved; mc deferred.\n");
    for classification in CLASSIFICATIONS {
        text.push_str(classification);
        text.push('\n');
    }
    text
}

fn positive_path_facts() -> Vec<PathFact> {
    MIGRATED_REPOS
        .iter()
        .map(|repo| PathFact {
            repo: repo.to_string(),
            legacy_exists: true,
            legacy_is_symlink: true,
            legacy_target: Some(format!("{COMPAT_LINK_PREFIX}{repo}")),
            canonical_exists: true,
            deferred: false,
        })
        .chain(DEFERRED_REPOS.iter().map(|repo| PathFact {
            repo: repo.to_string(),
            legacy_exists: true,
            legacy_is_symlink: false,
            legacy_target: None,
            canonical_exists: false,
            deferred: true,
        }))
        .collect()
}

fn path_facts_with_broken_legacy() -> Vec<PathFact> {
    let mut facts = positive_path_facts();
    if let Some(first) = facts.iter_mut().find(|fact| !fact.deferred) {
        first.legacy_is_symlink = false;
        first.legacy_target = None;
    }
    facts
}
