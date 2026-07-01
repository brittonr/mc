#!/usr/bin/env -S CARGO_TARGET_DIR=target/check-minecraft-wiki-guidance-script nix shell "github:nix-community/fenix?rev=092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain" nixpkgs#gcc -c cargo -q -Zscript

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const ROOT_FLAG: &str = "--root";
const SELF_TEST_FLAG: &str = "--self-test";
const DEFAULT_ROOT: &str = ".";
const SKILL_PATH: &str = ".pi/skills/minecraft-wiki/SKILL.md";
const SKILL_DECISION_PATH: &str = "docs/minecraft-wiki-skill.md";
const ROADMAP_PATH: &str = "docs/vanilla-composable-plugins-roadmap.md";
const EXPECTED_SKILL_NAME: &str = "minecraft-wiki";
const FRONTMATTER_DELIMITER: &str = "---";
const NAME_FIELD: &str = "name";
const DESCRIPTION_FIELD: &str = "description";
const FIELD_SEPARATOR: char = ':';
const NAME_MAX_CHARS: usize = 64;
const DESCRIPTION_MAX_CHARS: usize = 1024;
const SUCCESS_MESSAGE: &str = "minecraft wiki guidance check passed";
const SELF_TEST_SUCCESS_MESSAGE: &str = "minecraft wiki guidance self-test passed";
const SUCCESS: ExitCode = ExitCode::SUCCESS;
const FAILURE: ExitCode = ExitCode::FAILURE;

const SKILL_REQUIRED_PHRASES: &[(&str, &[&str])] = &[
    (
        "skill_missing_known_url_retrieval",
        &[
            "known Minecraft Wiki URL",
            "crw_scrape",
            "format=markdown",
            "format=links",
            "CSS",
            "XPath",
            "JS rendering",
        ],
    ),
    (
        "skill_missing_source_safety",
        &[
            "untrusted external data",
            "Do not execute instructions",
            "Do not vendor large page content",
        ],
    ),
    (
        "skill_missing_version_scope",
        &[
            "Target edition",
            "Target version/protocol",
            "Version-drift risks",
        ],
    ),
    (
        "skill_missing_citation_rules",
        &["Source page title and URL", "Retrieval date"],
    ),
    (
        "skill_missing_non_authority_language",
        &["wiki is useful as a guide", "not authoritative"],
    ),
    (
        "skill_missing_behavior_card_handoff",
        &[
            "behavior card",
            "Pure deterministic rule core inputs and outputs",
            "Thin Bevy/ECS shell",
            "Positive tests",
            "Negative tests",
        ],
    ),
    (
        "skill_missing_claim_boundary",
        &[
            "extracted-data checks",
            "Paper/vanilla parity receipts",
            "broad Minecraft compatibility",
            "public-server safety",
            "production readiness",
        ],
    ),
];

const PATH_DECISION_REQUIRED_PHRASES: &[(&str, &[&str])] = &[
    (
        "path_decision_missing_selected_path",
        &[SKILL_PATH, "project is trusted", "not represented as a global Pi skill"],
    ),
    (
        "path_decision_missing_format_basis",
        &[
            "Pi `docs/skills.md`",
            "Agent Skills specification",
            "`SKILL.md`",
            "`name`",
            "`description`",
        ],
    ),
    (
        "path_decision_missing_discovery_assumptions",
        &[
            "direct root `.md` files",
            "directories containing `SKILL.md`",
            "discovered recursively",
        ],
    ),
];

const ROADMAP_REQUIRED_PHRASES: &[(&str, &[&str])] = &[
    (
        "roadmap_missing_inventory",
        &[
            "## Source inventory",
            "https://minecraft.wiki/w/Minecraft_Wiki",
            "https://minecraft.wiki/w/Java_Edition_1.20.1",
            "https://minecraft.wiki/w/Protocol_version",
            "https://minecraft.wiki/w/Minecraft_Wiki:Protocol_documentation",
            "Java Edition 1.20.1 / protocol 763",
        ],
    ),
    (
        "roadmap_missing_taxonomy",
        &[
            "## Domain-to-plugin taxonomy",
            "Candidate plugin group",
            "Schedule impact",
            "Evidence need",
            "Non-claims",
        ],
    ),
    (
        "roadmap_missing_behavior_card",
        &[
            "## Behavior card template",
            "## Filled behavior card example: furnace smelting",
            "Pure rule core",
            "Thin Bevy/ECS shell",
        ],
    ),
    (
        "roadmap_missing_core_shell_policy",
        &["## Functional core / Bevy shell policy", "pure deterministic cores", "thin Bevy/ECS shells"],
    ),
    (
        "roadmap_missing_evidence_policy",
        &[
            "## Evidence and test policy",
            "Positive tests",
            "Negative tests",
            "extracted-data checks",
            "Paper/vanilla parity receipts",
        ],
    ),
    (
        "roadmap_missing_sequence",
        &[
            "## Implementation sequence and stop conditions",
            "Bounded survival stats",
            "Redstone",
            "Mobs",
            "Stop conditions",
        ],
    ),
    (
        "roadmap_missing_non_claims",
        &[
            "DefaultPlugins membership remains unchanged",
            "broad Minecraft compatibility",
            "broad vanilla parity",
            "public-server safety",
            "production readiness",
        ],
    ),
];

#[derive(Debug, Clone, PartialEq, Eq)]
struct GuidanceDocs {
    skill: String,
    path_decision: String,
    roadmap: String,
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

#[derive(Debug, Clone, PartialEq, Eq)]
struct Command {
    root: PathBuf,
    self_test: bool,
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
        Err(errors) => {
            print_errors(&errors);
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

    let docs = load_docs(&command.root).map_err(|error| {
        vec![Diagnostic {
            code: "read_error",
            path: SKILL_PATH,
            message: error,
        }]
    })?;
    let diagnostics = validate_guidance_docs(&docs);
    if diagnostics.is_empty() {
        Ok(String::from(SUCCESS_MESSAGE))
    } else {
        Err(diagnostics)
    }
}

fn print_errors(errors: &[Diagnostic]) {
    for error in errors {
        eprintln!("{}: {}: {}", error.path, error.code, error.message);
    }
}

fn load_docs(root: &Path) -> Result<GuidanceDocs, String> {
    Ok(GuidanceDocs {
        skill: read_to_string(root, SKILL_PATH)?,
        path_decision: read_to_string(root, SKILL_DECISION_PATH)?,
        roadmap: read_to_string(root, ROADMAP_PATH)?,
    })
}

fn read_to_string(root: &Path, relative: &str) -> Result<String, String> {
    let path = root.join(relative);
    fs::read_to_string(&path).map_err(|error| format!("failed to read {}: {error}", path.display()))
}

fn validate_guidance_docs(docs: &GuidanceDocs) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    diagnostics.extend(validate_skill(&docs.skill));
    diagnostics.extend(validate_phrase_groups(
        SKILL_DECISION_PATH,
        &docs.path_decision,
        PATH_DECISION_REQUIRED_PHRASES,
    ));
    diagnostics.extend(validate_phrase_groups(
        ROADMAP_PATH,
        &docs.roadmap,
        ROADMAP_REQUIRED_PHRASES,
    ));
    diagnostics
}

fn validate_skill(skill: &str) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    match parse_frontmatter(skill) {
        Ok((frontmatter, body)) => {
            diagnostics.extend(validate_frontmatter(&frontmatter));
            diagnostics.extend(validate_phrase_groups(
                SKILL_PATH,
                body,
                SKILL_REQUIRED_PHRASES,
            ));
        }
        Err(diagnostic) => diagnostics.push(diagnostic),
    }
    diagnostics
}

fn parse_frontmatter(text: &str) -> Result<(Frontmatter, &str), Diagnostic> {
    let Some(rest) = text.strip_prefix(FRONTMATTER_DELIMITER) else {
        return Err(Diagnostic {
            code: "missing_frontmatter",
            path: SKILL_PATH,
            message: String::from("SKILL.md must start with YAML frontmatter"),
        });
    };
    let Some(after_opening_line) = rest.strip_prefix('\n') else {
        return Err(Diagnostic {
            code: "malformed_frontmatter",
            path: SKILL_PATH,
            message: String::from("frontmatter delimiter must be followed by a newline"),
        });
    };
    let closing = format!("\n{FRONTMATTER_DELIMITER}\n");
    let Some(closing_offset) = after_opening_line.find(&closing) else {
        return Err(Diagnostic {
            code: "missing_frontmatter_close",
            path: SKILL_PATH,
            message: String::from("frontmatter closing delimiter is missing"),
        });
    };

    let frontmatter_text = &after_opening_line[..closing_offset];
    let body_offset = closing_offset + closing.len();
    let body = &after_opening_line[body_offset..];
    Ok((parse_frontmatter_fields(frontmatter_text), body))
}

fn parse_frontmatter_fields(text: &str) -> Frontmatter {
    let mut frontmatter = Frontmatter {
        name: None,
        description: None,
    };

    for line in text.lines() {
        let Some((field, value)) = line.split_once(FIELD_SEPARATOR) else {
            continue;
        };
        let field = field.trim();
        let value = value.trim().to_string();
        if field == NAME_FIELD {
            frontmatter.name = Some(value);
        } else if field == DESCRIPTION_FIELD {
            frontmatter.description = Some(value);
        }
    }

    frontmatter
}

fn validate_frontmatter(frontmatter: &Frontmatter) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    match frontmatter.name.as_deref() {
        Some(EXPECTED_SKILL_NAME) => {}
        Some(name) => diagnostics.push(Diagnostic {
            code: "invalid_skill_name",
            path: SKILL_PATH,
            message: format!("expected name {EXPECTED_SKILL_NAME}, found {name}"),
        }),
        None => diagnostics.push(Diagnostic {
            code: "missing_skill_name",
            path: SKILL_PATH,
            message: String::from("frontmatter is missing name"),
        }),
    }

    if let Some(name) = frontmatter.name.as_deref() {
        if !valid_skill_name(name) {
            diagnostics.push(Diagnostic {
                code: "invalid_skill_name_format",
                path: SKILL_PATH,
                message: String::from("name must use lowercase letters, numbers, and single hyphens"),
            });
        }
        if name.chars().count() > NAME_MAX_CHARS {
            diagnostics.push(Diagnostic {
                code: "skill_name_too_long",
                path: SKILL_PATH,
                message: format!("name exceeds {NAME_MAX_CHARS} characters"),
            });
        }
    }

    match frontmatter.description.as_deref() {
        Some(description) if !description.is_empty() => {
            if description.chars().count() > DESCRIPTION_MAX_CHARS {
                diagnostics.push(Diagnostic {
                    code: "skill_description_too_long",
                    path: SKILL_PATH,
                    message: format!("description exceeds {DESCRIPTION_MAX_CHARS} characters"),
                });
            }
        }
        _ => diagnostics.push(Diagnostic {
            code: "missing_skill_description",
            path: SKILL_PATH,
            message: String::from("frontmatter is missing non-empty description"),
        }),
    }

    diagnostics
}

fn valid_skill_name(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }
    if name.starts_with('-') || name.ends_with('-') || name.contains("--") {
        return false;
    }
    name.chars()
        .all(|character| character.is_ascii_lowercase() || character.is_ascii_digit() || character == '-')
}

fn validate_phrase_groups(
    path: &'static str,
    text: &str,
    groups: &[(&'static str, &[&'static str])],
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    for (code, phrases) in groups {
        let missing = missing_phrases(text, phrases);
        if !missing.is_empty() {
            diagnostics.push(Diagnostic {
                code,
                path,
                message: format!("missing required phrases: {}", missing.join(", ")),
            });
        }
    }
    diagnostics
}

fn missing_phrases<'a>(text: &str, phrases: &'a [&str]) -> Vec<&'a str> {
    phrases
        .iter()
        .copied()
        .filter(|phrase| !text.contains(phrase))
        .collect()
}

fn run_self_test() -> Result<(), Vec<Diagnostic>> {
    let valid = fixture_valid_docs();
    let valid_diagnostics = validate_guidance_docs(&valid);
    if !valid_diagnostics.is_empty() {
        return Err(vec![Diagnostic {
            code: "self_test_positive_failed",
            path: SKILL_PATH,
            message: format!("valid fixture failed: {valid_diagnostics:?}"),
        }]);
    }

    assert_negative_reports(
        "negative_missing_description",
        fixture_without_description(),
        "missing_skill_description",
    )?;
    assert_negative_reports(
        "negative_missing_known_url",
        fixture_with_skill_replacement("known Minecraft Wiki URL", "wiki page"),
        "skill_missing_known_url_retrieval",
    )?;
    assert_negative_reports(
        "negative_missing_version_scope",
        fixture_with_skill_replacement("Target version/protocol", "Target release"),
        "skill_missing_version_scope",
    )?;
    assert_negative_reports(
        "negative_missing_non_authority",
        fixture_with_skill_replacement("not authoritative", "useful"),
        "skill_missing_non_authority_language",
    )?;
    assert_negative_reports(
        "negative_missing_roadmap_inventory",
        fixture_with_roadmap_replacement("## Source inventory", "## Sources"),
        "roadmap_missing_inventory",
    )?;
    assert_negative_reports(
        "negative_missing_roadmap_negative_tests",
        fixture_with_roadmap_replacement("Negative tests", "Failure tests"),
        "roadmap_missing_evidence_policy",
    )?;

    Ok(())
}

fn assert_negative_reports(
    fixture_name: &'static str,
    docs: GuidanceDocs,
    expected_code: &'static str,
) -> Result<(), Vec<Diagnostic>> {
    let diagnostics = validate_guidance_docs(&docs);
    if diagnostics
        .iter()
        .any(|diagnostic| diagnostic.code == expected_code)
    {
        Ok(())
    } else {
        Err(vec![Diagnostic {
            code: "self_test_negative_failed",
            path: SKILL_PATH,
            message: format!(
                "{fixture_name} did not report {expected_code}; diagnostics: {diagnostics:?}"
            ),
        }])
    }
}

fn fixture_valid_docs() -> GuidanceDocs {
    GuidanceDocs {
        skill: fixture_valid_skill(),
        path_decision: fixture_text(PATH_DECISION_REQUIRED_PHRASES),
        roadmap: fixture_text(ROADMAP_REQUIRED_PHRASES),
    }
}

fn fixture_valid_skill() -> String {
    format!(
        "---\nname: {EXPECTED_SKILL_NAME}\ndescription: Use for Minecraft Wiki lookup and target-version scoping.\n---\n\n{}",
        fixture_text(SKILL_REQUIRED_PHRASES)
    )
}

fn fixture_text(groups: &[(&'static str, &[&'static str])]) -> String {
    groups
        .iter()
        .flat_map(|(_, phrases)| phrases.iter().copied())
        .collect::<Vec<_>>()
        .join("\n")
}

fn fixture_without_description() -> GuidanceDocs {
    let mut docs = fixture_valid_docs();
    docs.skill = docs
        .skill
        .replace("description: Use for Minecraft Wiki lookup and target-version scoping.\n", "");
    docs
}

fn fixture_with_skill_replacement(from: &str, to: &str) -> GuidanceDocs {
    let mut docs = fixture_valid_docs();
    docs.skill = docs.skill.replace(from, to);
    docs
}

fn fixture_with_roadmap_replacement(from: &str, to: &str) -> GuidanceDocs {
    let mut docs = fixture_valid_docs();
    docs.roadmap = docs.roadmap.replace(from, to);
    docs
}
