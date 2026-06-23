#!/usr/bin/env -S nix shell "github:nix-community/fenix?rev=092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain" nixpkgs#gcc -c cargo -q -Zscript

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const ROOT_FLAG: &str = "--root";
const SELF_TEST_FLAG: &str = "--self-test";
const DEFAULT_ROOT: &str = ".";
const AGENTS_PATH: &str = "AGENTS.md";
const README_PATH: &str = "README.md";
const ARCHITECTURE_PATH: &str = "docs/architecture.md";
const CHECKLIST_PATH: &str = "docs/layout-checklist.md";
const STEVENARELLA_AGENTS_PATH: &str = "clients/stevenarella/AGENTS.md";
const VALENCE_AGENTS_PATH: &str = "servers/valence/AGENTS.md";
const HYPERION_GIT_PATH: &str = "hyperion/.git";
const LEAFISH_GIT_PATH: &str = "Leafish/.git";
const STEVENARELLA_LABEL: &str = "clients/stevenarella/";
const VALENCE_LABEL: &str = "servers/valence/";
const LEAFISH_LABEL: &str = "Leafish/";
const HYPERION_LABEL: &str = "hyperion/";
const REFERENCE_ONLY_TEXT: &str = "reference-only";
const NESTED_GIT_TEXT: &str = "nested Git";
const DEFAULT_GATE_EXCLUSION_TEXT: &str = "excluded from default compatibility gates";
const OPT_IN_TEXT: &str = "explicit opt-in";
const PARENT_STATUS_TEXT: &str = "parent repo status";
const LEAFISH_WAIVER_TEXT: &str = "Waived here because the nested checkout is not parent-owned";
const SUCCESS_MESSAGE: &str = "layout boundary checks passed";
const SELF_TEST_SUCCESS_MESSAGE: &str = "layout boundary self-test passed";
const SUCCESS: ExitCode = ExitCode::SUCCESS;
const FAILURE: ExitCode = ExitCode::FAILURE;

#[derive(Debug, Clone)]
struct LayoutDocs {
    agents: String,
    readme: String,
    architecture: String,
    checklist: String,
    stevenarella_agents_exists: bool,
    valence_agents_exists: bool,
    leafish_git_exists: bool,
    hyperion_git_exists: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Diagnostic {
    code: &'static str,
    message: String,
}

fn contains_all(text: &str, needles: &[&str]) -> bool {
    needles.iter().all(|needle| text.contains(needle))
}

fn push_missing_doc(
    diagnostics: &mut Vec<Diagnostic>,
    code: &'static str,
    path: &str,
    needs: &[&str],
) {
    let joined = needs.join(", ");
    diagnostics.push(Diagnostic {
        code,
        message: format!("{path} must mention: {joined}"),
    });
}

fn check_layout_docs(docs: &LayoutDocs) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    if !docs.stevenarella_agents_exists {
        push_missing_doc(
            &mut diagnostics,
            "missing_stevenarella_agents",
            STEVENARELLA_AGENTS_PATH,
            &["local agent notes for the core client"],
        );
    }

    if !docs.valence_agents_exists {
        push_missing_doc(
            &mut diagnostics,
            "missing_valence_agents",
            VALENCE_AGENTS_PATH,
            &["local agent notes for the core server"],
        );
    }

    if !contains_all(
        &docs.agents,
        &[
            STEVENARELLA_AGENTS_PATH,
            VALENCE_AGENTS_PATH,
            CHECKLIST_PATH,
        ],
    ) {
        push_missing_doc(
            &mut diagnostics,
            "agents_local_notes_links",
            AGENTS_PATH,
            &[
                STEVENARELLA_AGENTS_PATH,
                VALENCE_AGENTS_PATH,
                CHECKLIST_PATH,
            ],
        );
    }

    if !contains_all(
        &docs.readme,
        &[
            STEVENARELLA_AGENTS_PATH,
            VALENCE_AGENTS_PATH,
            CHECKLIST_PATH,
        ],
    ) {
        push_missing_doc(
            &mut diagnostics,
            "readme_local_notes_links",
            README_PATH,
            &[
                STEVENARELLA_AGENTS_PATH,
                VALENCE_AGENTS_PATH,
                CHECKLIST_PATH,
            ],
        );
    }

    if !contains_all(
        &docs.architecture,
        &[
            STEVENARELLA_AGENTS_PATH,
            VALENCE_AGENTS_PATH,
            CHECKLIST_PATH,
        ],
    ) {
        push_missing_doc(
            &mut diagnostics,
            "architecture_local_notes_links",
            ARCHITECTURE_PATH,
            &[
                STEVENARELLA_AGENTS_PATH,
                VALENCE_AGENTS_PATH,
                CHECKLIST_PATH,
            ],
        );
    }

    if !contains_all(
        &docs.checklist,
        &[
            STEVENARELLA_LABEL,
            VALENCE_LABEL,
            STEVENARELLA_AGENTS_PATH,
            VALENCE_AGENTS_PATH,
        ],
    ) {
        push_missing_doc(
            &mut diagnostics,
            "checklist_local_notes_inventory",
            CHECKLIST_PATH,
            &[
                STEVENARELLA_LABEL,
                VALENCE_LABEL,
                STEVENARELLA_AGENTS_PATH,
                VALENCE_AGENTS_PATH,
            ],
        );
    }

    if docs.leafish_git_exists
        && !contains_all(
            &docs.agents,
            &[
                LEAFISH_LABEL,
                REFERENCE_ONLY_TEXT,
                NESTED_GIT_TEXT,
                PARENT_STATUS_TEXT,
            ],
        )
    {
        push_missing_doc(
            &mut diagnostics,
            "agents_leafish_boundary",
            AGENTS_PATH,
            &[
                LEAFISH_LABEL,
                REFERENCE_ONLY_TEXT,
                NESTED_GIT_TEXT,
                PARENT_STATUS_TEXT,
            ],
        );
    }

    if docs.leafish_git_exists
        && !contains_all(
            &docs.readme,
            &[
                LEAFISH_LABEL,
                REFERENCE_ONLY_TEXT,
                DEFAULT_GATE_EXCLUSION_TEXT,
                OPT_IN_TEXT,
            ],
        )
    {
        push_missing_doc(
            &mut diagnostics,
            "readme_leafish_boundary",
            README_PATH,
            &[
                LEAFISH_LABEL,
                REFERENCE_ONLY_TEXT,
                DEFAULT_GATE_EXCLUSION_TEXT,
                OPT_IN_TEXT,
            ],
        );
    }

    if docs.leafish_git_exists
        && !contains_all(
            &docs.architecture,
            &[
                LEAFISH_LABEL,
                REFERENCE_ONLY_TEXT,
                NESTED_GIT_TEXT,
                CHECKLIST_PATH,
            ],
        )
    {
        push_missing_doc(
            &mut diagnostics,
            "architecture_leafish_boundary",
            ARCHITECTURE_PATH,
            &[
                LEAFISH_LABEL,
                REFERENCE_ONLY_TEXT,
                NESTED_GIT_TEXT,
                CHECKLIST_PATH,
            ],
        );
    }

    if docs.leafish_git_exists
        && !contains_all(
            &docs.checklist,
            &[
                LEAFISH_LABEL,
                REFERENCE_ONLY_TEXT,
                LEAFISH_WAIVER_TEXT,
                "Excluded from default gates unless explicitly selected",
            ],
        )
    {
        push_missing_doc(
            &mut diagnostics,
            "checklist_leafish_boundary",
            CHECKLIST_PATH,
            &[
                LEAFISH_LABEL,
                REFERENCE_ONLY_TEXT,
                LEAFISH_WAIVER_TEXT,
                "Excluded from default gates unless explicitly selected",
            ],
        );
    }

    if docs.hyperion_git_exists
        && !contains_all(
            &docs.checklist,
            &[HYPERION_LABEL, "independent", "separate jj/git workflow"],
        )
    {
        push_missing_doc(
            &mut diagnostics,
            "checklist_hyperion_boundary",
            CHECKLIST_PATH,
            &[HYPERION_LABEL, "independent", "separate jj/git workflow"],
        );
    }

    diagnostics
}

fn read_to_string(root: &Path, relative: &str) -> Result<String, String> {
    let path = root.join(relative);
    fs::read_to_string(&path).map_err(|error| format!("failed to read {}: {error}", path.display()))
}

fn load_docs(root: &Path) -> Result<LayoutDocs, String> {
    Ok(LayoutDocs {
        agents: read_to_string(root, AGENTS_PATH)?,
        readme: read_to_string(root, README_PATH)?,
        architecture: read_to_string(root, ARCHITECTURE_PATH)?,
        checklist: read_to_string(root, CHECKLIST_PATH)?,
        stevenarella_agents_exists: root.join(STEVENARELLA_AGENTS_PATH).exists(),
        valence_agents_exists: root.join(VALENCE_AGENTS_PATH).exists(),
        leafish_git_exists: root.join(LEAFISH_GIT_PATH).exists(),
        hyperion_git_exists: root.join(HYPERION_GIT_PATH).exists(),
    })
}

fn fixture_valid_docs() -> LayoutDocs {
    LayoutDocs {
        agents: format!(
            "{STEVENARELLA_AGENTS_PATH} {VALENCE_AGENTS_PATH} {CHECKLIST_PATH} {LEAFISH_LABEL} is {REFERENCE_ONLY_TEXT} {NESTED_GIT_TEXT}; do not use {PARENT_STATUS_TEXT}."
        ),
        readme: format!(
            "{STEVENARELLA_AGENTS_PATH} {VALENCE_AGENTS_PATH} {CHECKLIST_PATH} {LEAFISH_LABEL} is {REFERENCE_ONLY_TEXT}, {DEFAULT_GATE_EXCLUSION_TEXT}, and uses {OPT_IN_TEXT} commands."
        ),
        architecture: format!(
            "{STEVENARELLA_AGENTS_PATH} {VALENCE_AGENTS_PATH} {CHECKLIST_PATH} {LEAFISH_LABEL} is {REFERENCE_ONLY_TEXT} {NESTED_GIT_TEXT}; see {CHECKLIST_PATH}."
        ),
        checklist: format!(
            "{STEVENARELLA_LABEL} {VALENCE_LABEL} {STEVENARELLA_AGENTS_PATH} {VALENCE_AGENTS_PATH}\n{LEAFISH_LABEL} {REFERENCE_ONLY_TEXT} {LEAFISH_WAIVER_TEXT} Excluded from default gates unless explicitly selected\n{HYPERION_LABEL} independent separate jj/git workflow"
        ),
        stevenarella_agents_exists: true,
        valence_agents_exists: true,
        leafish_git_exists: true,
        hyperion_git_exists: true,
    }
}

fn run_self_test() -> Result<(), String> {
    let valid = fixture_valid_docs();
    let valid_diagnostics = check_layout_docs(&valid);
    if !valid_diagnostics.is_empty() {
        return Err(format!(
            "positive fixture unexpectedly failed: {:?}",
            valid_diagnostics
        ));
    }

    let mut missing_stevenarella = valid.clone();
    missing_stevenarella.stevenarella_agents_exists = false;
    let missing_stevenarella_diagnostics = check_layout_docs(&missing_stevenarella);
    if !missing_stevenarella_diagnostics
        .iter()
        .any(|diagnostic| diagnostic.code == "missing_stevenarella_agents")
    {
        return Err(format!(
            "negative fixture did not report missing Stevenarella agent docs: {:?}",
            missing_stevenarella_diagnostics
        ));
    }

    let mut missing_root_links = valid.clone();
    missing_root_links.agents = String::from("workspace guidance without local links");
    let missing_root_link_diagnostics = check_layout_docs(&missing_root_links);
    if !missing_root_link_diagnostics
        .iter()
        .any(|diagnostic| diagnostic.code == "agents_local_notes_links")
    {
        return Err(format!(
            "negative fixture did not report missing root local-note links: {:?}",
            missing_root_link_diagnostics
        ));
    }

    let mut missing_leafish = valid.clone();
    missing_leafish.readme = String::from("core components only");
    let missing_diagnostics = check_layout_docs(&missing_leafish);
    if !missing_diagnostics
        .iter()
        .any(|diagnostic| diagnostic.code == "readme_leafish_boundary")
    {
        return Err(format!(
            "negative fixture did not report missing README Leafish boundary: {:?}",
            missing_diagnostics
        ));
    }

    let mut missing_hyperion = valid;
    missing_hyperion.checklist = format!(
        "{LEAFISH_LABEL} {REFERENCE_ONLY_TEXT} {LEAFISH_WAIVER_TEXT} Excluded from default gates unless explicitly selected"
    );
    let hyperion_diagnostics = check_layout_docs(&missing_hyperion);
    if !hyperion_diagnostics
        .iter()
        .any(|diagnostic| diagnostic.code == "checklist_hyperion_boundary")
    {
        return Err(format!(
            "negative fixture did not report missing Hyperion checklist boundary: {:?}",
            hyperion_diagnostics
        ));
    }

    Ok(())
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

#[derive(Debug, Clone)]
struct Command {
    root: PathBuf,
    self_test: bool,
}

fn run(command: Command) -> Result<String, String> {
    if command.self_test {
        run_self_test()?;
        return Ok(String::from(SELF_TEST_SUCCESS_MESSAGE));
    }

    let docs = load_docs(&command.root)?;
    let diagnostics = check_layout_docs(&docs);
    if diagnostics.is_empty() {
        Ok(String::from(SUCCESS_MESSAGE))
    } else {
        let rendered = diagnostics
            .into_iter()
            .map(|diagnostic| format!("{}: {}", diagnostic.code, diagnostic.message))
            .collect::<Vec<_>>()
            .join("\n");
        Err(rendered)
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
