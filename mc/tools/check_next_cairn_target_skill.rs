#!/usr/bin/env -S CARGO_TARGET_DIR=target/check-next-cairn-target-skill-script nix shell "github:nix-community/fenix?rev=092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain" nixpkgs#gcc -c cargo -q -Zscript

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const ROOT_FLAG: &str = "--root";
const SELF_TEST_FLAG: &str = "--self-test";
const DEFAULT_ROOT: &str = ".";
const SKILL_PATH: &str = ".pi/skills/next-cairn-target/SKILL.md";
const DOC_PATH: &str = "docs/next-cairn-target-skill.md";
const EXPECTED_SKILL_NAME: &str = "next-cairn-target";
const FRONTMATTER_DELIMITER: &str = "---";
const NAME_FIELD: &str = "name";
const DESCRIPTION_FIELD: &str = "description";
const FIELD_SEPARATOR: char = ':';
const NAME_MAX_CHARS: usize = 64;
const DESCRIPTION_MAX_CHARS: usize = 1024;
const SUCCESS_MESSAGE: &str = "next cairn target skill check passed";
const SELF_TEST_SUCCESS_MESSAGE: &str = "next cairn target skill self-test passed";
const SUCCESS: ExitCode = ExitCode::SUCCESS;
const FAILURE: ExitCode = ExitCode::FAILURE;

const SKILL_REQUIRED_GROUPS: &[RequiredGroup] = &[
    RequiredGroup {
        code: "skill_missing_trigger_scope",
        path: SKILL_PATH,
        phrases: &[
            "what to do next",
            "hunt/find/select the next implementation target",
            "write a Cairn/change package",
            "not an implementation unless explicitly requested",
        ],
    },
    RequiredGroup {
        code: "skill_missing_preflight",
        path: SKILL_PATH,
        phrases: &[
            "Check status and preserve user changes",
            "nix run .#cairn -- change list --root .",
            "If active changes exist",
            "stop target selection",
        ],
    },
    RequiredGroup {
        code: "skill_missing_candidate_sources",
        path: SKILL_PATH,
        phrases: &[
            "Roadmap sequence rows",
            "Accepted specs",
            "Behavior cards",
            "Evidence gaps",
            "Small checker/doc gaps",
        ],
    },
    RequiredGroup {
        code: "skill_missing_scoring",
        path: SKILL_PATH,
        phrases: &[
            "Bounded scope",
            "Clear predecessor",
            "Testability",
            "Evidence path",
            "Low schedule risk",
            "Non-claims are clear",
        ],
    },
    RequiredGroup {
        code: "skill_missing_output_shape",
        path: SKILL_PATH,
        phrases: &[
            "## Candidate decision",
            "proposal.md",
            "design.md",
            "tasks.md",
            "specs/<accepted-spec>/spec.md",
        ],
    },
    RequiredGroup {
        code: "skill_missing_validation_gates",
        path: SKILL_PATH,
        phrases: &[
            "nix run .#cairn -- validate --root .",
            "gate proposal",
            "gate design",
            "gate tasks",
            "Exactly one active Cairn package",
        ],
    },
    RequiredGroup {
        code: "skill_missing_non_claims",
        path: SKILL_PATH,
        phrases: &[
            "Do not overclaim",
            "broad vanilla parity",
            "all recipes",
            "public-server safety",
            "production readiness",
            "No implementation, sync, archive, or push",
        ],
    },
];

const DOC_REQUIRED_GROUPS: &[RequiredGroup] = &[
    RequiredGroup {
        code: "doc_missing_path_decision",
        path: DOC_PATH,
        phrases: &[
            ".pi/skills/next-cairn-target/SKILL.md",
            "name: next-cairn-target",
            "Pi project skill discovery",
            "not installed as a global Pi skill",
        ],
    },
    RequiredGroup {
        code: "doc_missing_workflow",
        path: DOC_PATH,
        phrases: &[
            "Preserve user changes",
            "nix run .#cairn -- change list --root .",
            "Collect candidates",
            "Reject broad or risky scopes",
            "Run Cairn validate plus proposal/design/tasks gates",
        ],
    },
    RequiredGroup {
        code: "doc_missing_scoring",
        path: DOC_PATH,
        phrases: &[
            "bounded scope",
            "clear predecessor artifact",
            "positive and negative testability",
            "reviewable evidence path",
            "explicit non-claims",
        ],
    },
    RequiredGroup {
        code: "doc_missing_output_shape",
        path: DOC_PATH,
        phrases: &[
            "## Candidate decision",
            "proposal.md",
            "design.md",
            "tasks.md",
            "specs/<accepted-spec>/spec.md",
        ],
    },
    RequiredGroup {
        code: "doc_missing_non_claims",
        path: DOC_PATH,
        phrases: &[
            "rank every possible project task",
            "replace user priorities",
            "implement selected targets",
            "prove vanilla parity",
            "global Pi skills",
        ],
    },
];

const SELF_TEST_VALID_SKILL: &str = r#"---
name: next-cairn-target
description: Use this skill when the user asks what to do next, hunt/find/select the next implementation target, or write a Cairn/change package; not an implementation unless explicitly requested.
---
# Next Cairn Target
Check status and preserve user changes. Run nix run .#cairn -- change list --root . If active changes exist, stop target selection.
Roadmap sequence rows. Accepted specs. Behavior cards. Evidence gaps. Small checker/doc gaps.
Bounded scope. Clear predecessor. Testability. Evidence path. Low schedule risk. Non-claims are clear.
## Candidate decision
Create proposal.md, design.md, tasks.md, specs/<accepted-spec>/spec.md.
Run nix run .#cairn -- validate --root . plus gate proposal, gate design, gate tasks. Exactly one active Cairn package.
Do not overclaim broad vanilla parity, all recipes, public-server safety, production readiness. No implementation, sync, archive, or push.
"#;

const SELF_TEST_VALID_DOC: &str = r#"# Next Cairn target project skill
.pi/skills/next-cairn-target/SKILL.md uses name: next-cairn-target. Pi project skill discovery loads it when trusted. It is not installed as a global Pi skill.
Preserve user changes. Run nix run .#cairn -- change list --root . Collect candidates. Reject broad or risky scopes. Run Cairn validate plus proposal/design/tasks gates.
bounded scope. clear predecessor artifact. positive and negative testability. reviewable evidence path. explicit non-claims.
## Candidate decision
proposal.md design.md tasks.md specs/<accepted-spec>/spec.md
Non-claims: rank every possible project task, replace user priorities, implement selected targets, prove vanilla parity, global Pi skills.
"#;

const NEGATIVE_SELF_TESTS: &[NegativeSelfTest] = &[
    NegativeSelfTest {
        name: "missing frontmatter name",
        target: FixtureTarget::Skill,
        phrase_to_remove: "name: next-cairn-target",
        expected_code: "frontmatter_name_missing",
    },
    NegativeSelfTest {
        name: "missing preflight",
        target: FixtureTarget::Skill,
        phrase_to_remove: "nix run .#cairn -- change list --root .",
        expected_code: "skill_missing_preflight",
    },
    NegativeSelfTest {
        name: "missing candidate source",
        target: FixtureTarget::Skill,
        phrase_to_remove: "Behavior cards",
        expected_code: "skill_missing_candidate_sources",
    },
    NegativeSelfTest {
        name: "missing scoring",
        target: FixtureTarget::Skill,
        phrase_to_remove: "Low schedule risk",
        expected_code: "skill_missing_scoring",
    },
    NegativeSelfTest {
        name: "missing output shape",
        target: FixtureTarget::Skill,
        phrase_to_remove: "specs/<accepted-spec>/spec.md",
        expected_code: "skill_missing_output_shape",
    },
    NegativeSelfTest {
        name: "missing non-claim",
        target: FixtureTarget::Skill,
        phrase_to_remove: "No implementation, sync, archive, or push",
        expected_code: "skill_missing_non_claims",
    },
    NegativeSelfTest {
        name: "missing doc path",
        target: FixtureTarget::Doc,
        phrase_to_remove: ".pi/skills/next-cairn-target/SKILL.md",
        expected_code: "doc_missing_path_decision",
    },
    NegativeSelfTest {
        name: "missing doc scoring",
        target: FixtureTarget::Doc,
        phrase_to_remove: "positive and negative testability",
        expected_code: "doc_missing_scoring",
    },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RequiredGroup {
    code: &'static str,
    path: &'static str,
    phrases: &'static [&'static str],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct NegativeSelfTest {
    name: &'static str,
    target: FixtureTarget,
    phrase_to_remove: &'static str,
    expected_code: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FixtureTarget {
    Skill,
    Doc,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SkillDocs {
    skill: String,
    doc: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Command {
    root: PathBuf,
    self_test: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Frontmatter {
    name: Option<String>,
    description: Option<String>,
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

    let docs = load_docs(&command.root).map_err(|message| {
        vec![Diagnostic {
            code: "read_error",
            path: SKILL_PATH,
            message,
        }]
    })?;
    let diagnostics = validate_docs(&docs);
    if diagnostics.is_empty() {
        Ok(String::from(SUCCESS_MESSAGE))
    } else {
        Err(diagnostics)
    }
}

fn load_docs(root: &Path) -> Result<SkillDocs, String> {
    let skill_path = root.join(SKILL_PATH);
    let doc_path = root.join(DOC_PATH);
    let skill = fs::read_to_string(&skill_path)
        .map_err(|error| format!("failed to read {}: {error}", skill_path.display()))?;
    let doc = fs::read_to_string(&doc_path)
        .map_err(|error| format!("failed to read {}: {error}", doc_path.display()))?;
    Ok(SkillDocs { skill, doc })
}

fn validate_docs(docs: &SkillDocs) -> Vec<Diagnostic> {
    let mut diagnostics = validate_frontmatter(&docs.skill);
    diagnostics.extend(validate_groups(&docs.skill, SKILL_REQUIRED_GROUPS));
    diagnostics.extend(validate_groups(&docs.doc, DOC_REQUIRED_GROUPS));
    diagnostics
}

fn validate_frontmatter(skill: &str) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let frontmatter = parse_frontmatter(skill);

    match frontmatter.name.as_deref() {
        Some(EXPECTED_SKILL_NAME) => {}
        Some(_) => diagnostics.push(Diagnostic {
            code: "frontmatter_name_mismatch",
            path: SKILL_PATH,
            message: format!("name must be {EXPECTED_SKILL_NAME}"),
        }),
        None => diagnostics.push(Diagnostic {
            code: "frontmatter_name_missing",
            path: SKILL_PATH,
            message: String::from("missing name frontmatter"),
        }),
    }

    match frontmatter.description.as_deref() {
        Some(description) if !description.is_empty() && description.chars().count() <= DESCRIPTION_MAX_CHARS => {}
        Some(_) => diagnostics.push(Diagnostic {
            code: "frontmatter_description_invalid",
            path: SKILL_PATH,
            message: format!("description must be 1..={DESCRIPTION_MAX_CHARS} chars"),
        }),
        None => diagnostics.push(Diagnostic {
            code: "frontmatter_description_missing",
            path: SKILL_PATH,
            message: String::from("missing description frontmatter"),
        }),
    }

    if EXPECTED_SKILL_NAME.chars().count() > NAME_MAX_CHARS {
        diagnostics.push(Diagnostic {
            code: "frontmatter_name_too_long",
            path: SKILL_PATH,
            message: format!("name exceeds {NAME_MAX_CHARS} chars"),
        });
    }

    diagnostics
}

fn parse_frontmatter(skill: &str) -> Frontmatter {
    let mut lines = skill.lines();
    if lines.next() != Some(FRONTMATTER_DELIMITER) {
        return Frontmatter {
            name: None,
            description: None,
        };
    }

    let mut name = None;
    let mut description = None;

    for line in lines {
        if line == FRONTMATTER_DELIMITER {
            break;
        }
        let Some((field, value)) = line.split_once(FIELD_SEPARATOR) else {
            continue;
        };
        let trimmed_value = value.trim().to_string();
        match field.trim() {
            NAME_FIELD => name = Some(trimmed_value),
            DESCRIPTION_FIELD => description = Some(trimmed_value),
            _ => {}
        }
    }

    Frontmatter { name, description }
}

fn validate_groups(content: &str, groups: &[RequiredGroup]) -> Vec<Diagnostic> {
    groups
        .iter()
        .filter_map(|group| {
            let missing = missing_phrases(content, group.phrases);
            if missing.is_empty() {
                None
            } else {
                Some(Diagnostic {
                    code: group.code,
                    path: group.path,
                    message: format!("missing required phrase(s): {}", missing.join(", ")),
                })
            }
        })
        .collect()
}

fn missing_phrases(content: &str, phrases: &[&'static str]) -> Vec<&'static str> {
    phrases
        .iter()
        .copied()
        .filter(|phrase| !content.contains(phrase))
        .collect()
}

fn run_self_test() -> Result<(), Vec<Diagnostic>> {
    let valid_docs = SkillDocs {
        skill: String::from(SELF_TEST_VALID_SKILL),
        doc: String::from(SELF_TEST_VALID_DOC),
    };
    let positive_diagnostics = validate_docs(&valid_docs);
    if !positive_diagnostics.is_empty() {
        return Err(vec![Diagnostic {
            code: "self_test_positive_failed",
            path: SKILL_PATH,
            message: format!("valid fixture failed: {positive_diagnostics:?}"),
        }]);
    }

    for test in NEGATIVE_SELF_TESTS {
        let mut fixture = valid_docs.clone();
        match test.target {
            FixtureTarget::Skill => {
                fixture.skill = fixture.skill.replace(test.phrase_to_remove, "");
            }
            FixtureTarget::Doc => {
                fixture.doc = fixture.doc.replace(test.phrase_to_remove, "");
            }
        }

        let diagnostics = validate_docs(&fixture);
        let saw_expected_code = diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == test.expected_code);
        if !saw_expected_code {
            return Err(vec![Diagnostic {
                code: "self_test_negative_failed",
                path: SKILL_PATH,
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
