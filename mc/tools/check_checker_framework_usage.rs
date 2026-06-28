use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const ROOT_FLAG: &str = "--root";
const SELF_TEST_FLAG: &str = "--self-test";
const DEFAULT_ROOT: &str = ".";
const TOOLS_DIR: &str = "tools";
const CHECKER_PREFIX: &str = "check_";
const PYTHON_SUFFIX: &str = ".py";
const SUCCESS: ExitCode = ExitCode::SUCCESS;
const FAILURE: ExitCode = ExitCode::FAILURE;
const FRAMEWORK_BACKED_CHECKERS: &[&str] = &[
    "tools/check_scoreboard_team_packet_family.rs",
    "tools/check_movement_packet_family.rs",
];
const LEGACY_PYTHON_CHECKERS: &[&str] = &[
    "tools/check_armor_modifier_matrix.py",
    "tools/check_ctf_rule_ledger.py",
    "tools/check_death_respawn_lifecycle.py",
    "tools/check_equipment_slot_item_matrix.py",
    "tools/check_inventory_semantics_matrix.py",
    "tools/check_load_network_safety.py",
    "tools/check_projectile_travel_collision.py",
    "tools/check_protocol_coverage_ledger.py",
    "tools/check_survival_reference_parity.py",
    "tools/check_vanilla_combat_parity.py",
];
const REQUIRED_FRAMEWORK_MARKERS: &[&str] = &["mod checker_framework;", "KeyValueRecord"];
const FORBIDDEN_AD_HOC_MARKERS: &[ForbiddenMarker] = &[
    ForbiddenMarker::new("ad_hoc_key_value_split", "split_once('=')"),
    ForbiddenMarker::new("local_btreemap_evidence", "BTreeMap<String, String>"),
    ForbiddenMarker::new("local_evidence_struct", "struct Evidence {"),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ForbiddenMarker {
    code: &'static str,
    marker: &'static str,
}

impl ForbiddenMarker {
    const fn new(code: &'static str, marker: &'static str) -> Self {
        Self { code, marker }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SourceFile {
    path: String,
    text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct UsageReport {
    framework_backed_checkers: usize,
    legacy_python_checkers: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Command {
    root: PathBuf,
    self_test: bool,
}

fn validate_framework_usage(
    framework_sources: &[SourceFile],
    python_checkers: &[String],
) -> Result<UsageReport, Vec<String>> {
    let mut diagnostics = Vec::new();
    validate_framework_backed_sources(framework_sources, &mut diagnostics);
    validate_legacy_python_inventory(python_checkers, &mut diagnostics);

    if diagnostics.is_empty() {
        Ok(UsageReport {
            framework_backed_checkers: framework_sources.len(),
            legacy_python_checkers: python_checkers.len(),
        })
    } else {
        Err(diagnostics)
    }
}

fn validate_framework_backed_sources(sources: &[SourceFile], diagnostics: &mut Vec<String>) {
    let actual_paths = sources
        .iter()
        .map(|source| source.path.as_str())
        .collect::<BTreeSet<_>>();
    for expected_path in FRAMEWORK_BACKED_CHECKERS {
        if !actual_paths.contains(expected_path) {
            diagnostics.push(format!(
                "missing framework-backed checker source {expected_path}"
            ));
        }
    }

    for source in sources {
        for marker in REQUIRED_FRAMEWORK_MARKERS {
            if !source.text.contains(marker) {
                diagnostics.push(format!(
                    "{} missing required checker framework marker {marker:?}",
                    source.path
                ));
            }
        }
        for forbidden in FORBIDDEN_AD_HOC_MARKERS {
            if source.text.contains(forbidden.marker) {
                diagnostics.push(format!(
                    "{} {}: replace {:?} with tools/checker_framework.rs",
                    source.path, forbidden.code, forbidden.marker
                ));
            }
        }
    }
}

fn validate_legacy_python_inventory(python_checkers: &[String], diagnostics: &mut Vec<String>) {
    let actual = python_checkers
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    let expected = LEGACY_PYTHON_CHECKERS
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();

    for missing in expected.difference(&actual) {
        diagnostics.push(format!(
            "missing legacy Python checker inventory entry {missing}"
        ));
    }
    for unexpected in actual.difference(&expected) {
        diagnostics.push(format!(
            "unexpected Python checker {unexpected}; migrate it to Rust/Steel or document it in LEGACY_PYTHON_CHECKERS"
        ));
    }
}

fn read_framework_sources(root: &Path) -> Result<Vec<SourceFile>, String> {
    let mut sources = Vec::new();
    for relative in FRAMEWORK_BACKED_CHECKERS {
        let path = root.join(relative);
        let text = fs::read_to_string(&path)
            .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
        sources.push(SourceFile {
            path: (*relative).to_string(),
            text,
        });
    }
    Ok(sources)
}

fn read_python_checker_paths(root: &Path) -> Result<Vec<String>, String> {
    let tools_dir = root.join(TOOLS_DIR);
    let entries = fs::read_dir(&tools_dir)
        .map_err(|error| format!("failed to read {}: {error}", tools_dir.display()))?;
    let mut paths = Vec::new();
    for entry_result in entries {
        let entry = entry_result
            .map_err(|error| format!("failed to read {} entry: {error}", tools_dir.display()))?;
        let path = entry.path();
        let metadata = entry
            .metadata()
            .map_err(|error| format!("failed to stat {}: {error}", path.display()))?;
        if !metadata.is_file() {
            continue;
        }
        let Some(file_name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        if file_name.starts_with(CHECKER_PREFIX) && file_name.ends_with(PYTHON_SUFFIX) {
            paths.push(format!("{TOOLS_DIR}/{file_name}"));
        }
    }
    paths.sort();
    Ok(paths)
}

fn parse_args() -> Result<Command, String> {
    let mut root = PathBuf::from(DEFAULT_ROOT);
    let mut self_test = false;
    let mut args = env::args().skip(1);

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

fn run_self_test() -> Result<(), String> {
    let valid_sources = FRAMEWORK_BACKED_CHECKERS
        .iter()
        .map(|path| SourceFile {
            path: (*path).to_string(),
            text: "mod checker_framework;\nuse checker_framework::KeyValueRecord;\n".to_string(),
        })
        .collect::<Vec<_>>();
    let valid_python = LEGACY_PYTHON_CHECKERS
        .iter()
        .map(|path| (*path).to_string())
        .collect::<Vec<_>>();
    validate_framework_usage(&valid_sources, &valid_python)
        .map_err(|diagnostics| format!("positive fixture failed: {diagnostics:?}"))?;

    let mut missing_marker = valid_sources.clone();
    missing_marker[0].text = "mod checker_framework;\n".to_string();
    expect_error(
        "missing framework marker",
        validate_framework_usage(&missing_marker, &valid_python),
        "missing required checker framework marker",
    )?;

    let mut ad_hoc_parser = valid_sources.clone();
    ad_hoc_parser[0].text =
        "mod checker_framework;\nuse checker_framework::KeyValueRecord;\nline.split_once('=');\n"
            .to_string();
    expect_error(
        "ad hoc parser",
        validate_framework_usage(&ad_hoc_parser, &valid_python),
        "ad_hoc_key_value_split",
    )?;

    let mut unexpected_python = valid_python;
    unexpected_python.push("tools/check_new_python_gate.py".to_string());
    expect_error(
        "unexpected python checker",
        validate_framework_usage(&valid_sources, &unexpected_python),
        "unexpected Python checker",
    )?;

    Ok(())
}

fn expect_error(
    name: &str,
    result: Result<UsageReport, Vec<String>>,
    expected: &str,
) -> Result<(), String> {
    match result {
        Ok(report) => Err(format!("{name}: unexpectedly passed: {report:?}")),
        Err(diagnostics) => {
            let rendered = diagnostics.join("; ");
            if rendered.contains(expected) {
                Ok(())
            } else {
                Err(format!(
                    "{name}: expected diagnostic containing {expected:?}, got {rendered}"
                ))
            }
        }
    }
}

fn run(command: Command) -> Result<String, String> {
    if command.self_test {
        run_self_test()?;
        return Ok("checker framework usage self-test passed".to_string());
    }

    let framework_sources = read_framework_sources(&command.root)?;
    let python_checkers = read_python_checker_paths(&command.root)?;
    let report = validate_framework_usage(&framework_sources, &python_checkers)
        .map_err(|diagnostics| diagnostics.join("\n"))?;
    Ok(format!(
        "checker framework usage passed: {} framework-backed checkers; {} legacy Python checkers inventoried",
        report.framework_backed_checkers, report.legacy_python_checkers
    ))
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
