use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

const SELF_TEST_FLAG: &str = "--self-test";
const MATRIX_DOC_PATH: &str = "docs/evidence/protocol-763-survival-coverage-matrix-2026-05-28.md";
const CURRENT_BUNDLE_PATH: &str = "docs/evidence/protocol-763-current-evidence-bundle.md";
const ACCEPTANCE_MATRIX_PATH: &str = "docs/evidence/protocol-763-acceptance-matrix.md";
const EVIDENCE_DIR: &str = "docs/evidence";
const B3_EXTENSION: &str = "b3";
const TABLE_HEADER: &str = "| Survival system | Status | Valence evidence | Reference evidence | Promotion requirement | Explicit non-claim | Next action |";
const TABLE_CELL_COUNT: usize = 7;
const REQUIRED_ROW_COUNT: usize = 9;
const STATUS_COVERED: &str = "reference_parity_covered";
const STATUS_MISSING: &str = "missing";
const EMPTY_EVIDENCE: &str = "none";
const DOCS_EVIDENCE_PREFIX: &str = "docs/evidence/";
const MANIFEST_SEPARATOR: &str = "  ";
const SUCCESS: ExitCode = ExitCode::SUCCESS;
const FAILURE: ExitCode = ExitCode::FAILURE;

const REQUIRED_SYSTEMS: &[&str] = &[
    "break/place/pickup",
    "crafting",
    "chest persistence",
    "furnace persistence",
    "hunger/food",
    "mob drops",
    "redstone",
    "biome/dimension",
    "world persistence",
];

const FORBIDDEN_WHILE_MISSING: &[&str] = &[
    "full_survival_compatibility is covered",
    "full survival compatibility is covered",
    "full survival compatibility passes",
    "full survival compatibility proven",
];

const CURRENT_COVERED_ROWS: &[CoveredRowExpectation] = &[
    CoveredRowExpectation {
        system: "break/place/pickup",
        acceptance_token: "Survival break/place/pickup",
    },
    CoveredRowExpectation {
        system: "crafting",
        acceptance_token: "Survival crafting table",
    },
    CoveredRowExpectation {
        system: "chest persistence",
        acceptance_token: "Survival chest persistence",
    },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CoveredRowExpectation {
    system: &'static str,
    acceptance_token: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SurvivalRow {
    system: String,
    status: String,
    valence_evidence: String,
    reference_evidence: String,
    requirement: String,
    non_claim: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct GateSummary {
    covered_rows: usize,
    missing_rows: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct GateInputs {
    survival_matrix: String,
    current_bundle: String,
    acceptance_matrix: String,
    manifest_paths: BTreeSet<String>,
}

fn main() -> ExitCode {
    let args = env::args().collect::<Vec<_>>();
    if args.iter().any(|arg| arg == SELF_TEST_FLAG) {
        return match run_self_tests() {
            Ok(summary) => {
                println!("full survival compatibility gate self-test ok: {summary}");
                SUCCESS
            }
            Err(errors) => {
                print_errors(&errors);
                FAILURE
            }
        };
    }

    match read_repo_inputs(Path::new(".")).and_then(|inputs| validate_gate(&inputs)) {
        Ok(summary) => {
            println!(
                "full survival compatibility gate passed: {} covered rows, {} missing rows",
                summary.covered_rows, summary.missing_rows
            );
            SUCCESS
        }
        Err(errors) => {
            print_errors(&errors);
            FAILURE
        }
    }
}

fn print_errors(errors: &[String]) {
    for error in errors {
        eprintln!("full survival compatibility gate failed: {error}");
    }
}

fn read_repo_inputs(root: &Path) -> Result<GateInputs, Vec<String>> {
    Ok(GateInputs {
        survival_matrix: read_file(root, MATRIX_DOC_PATH)?,
        current_bundle: read_file(root, CURRENT_BUNDLE_PATH)?,
        acceptance_matrix: read_file(root, ACCEPTANCE_MATRIX_PATH)?,
        manifest_paths: read_manifest_paths(root, Path::new(EVIDENCE_DIR))?,
    })
}

fn read_file(root: &Path, relative_path: &str) -> Result<String, Vec<String>> {
    fs::read_to_string(root.join(relative_path))
        .map_err(|error| vec![format!("{relative_path}: {error}")])
}

fn read_manifest_paths(root: &Path, evidence_dir: &Path) -> Result<BTreeSet<String>, Vec<String>> {
    let mut paths = BTreeSet::new();
    let directory = root.join(evidence_dir);
    let entries = fs::read_dir(&directory)
        .map_err(|error| vec![format!("{}: {error}", directory.display())])?;
    for entry_result in entries {
        let entry =
            entry_result.map_err(|error| vec![format!("{}: {error}", directory.display())])?;
        let path = entry.path();
        if path.is_file() && path.extension() == Some(OsStr::new(B3_EXTENSION)) {
            paths.extend(read_manifest_file(&path)?);
        }
    }
    Ok(paths)
}

fn read_manifest_file(path: &Path) -> Result<BTreeSet<String>, Vec<String>> {
    let text =
        fs::read_to_string(path).map_err(|error| vec![format!("{}: {error}", path.display())])?;
    Ok(manifest_paths_from_text(&text))
}

fn manifest_paths_from_text(text: &str) -> BTreeSet<String> {
    text.lines()
        .filter_map(|line| line.split_once(MANIFEST_SEPARATOR))
        .map(|(_, path)| path.trim().to_string())
        .collect()
}

fn validate_gate(inputs: &GateInputs) -> Result<GateSummary, Vec<String>> {
    let mut errors = Vec::new();
    let rows = parse_rows(&inputs.survival_matrix, &mut errors);
    let rows_by_system = rows_by_system(&rows);

    if rows.len() != REQUIRED_ROW_COUNT {
        errors.push(format!(
            "expected {REQUIRED_ROW_COUNT} survival rows, found {}",
            rows.len()
        ));
    }

    for required_system in REQUIRED_SYSTEMS {
        if !rows_by_system.contains_key(required_system) {
            errors.push(format!("missing required survival row: {required_system}"));
        }
    }

    let missing_rows = rows
        .iter()
        .filter(|row| row.status == STATUS_MISSING)
        .count();
    let covered_rows = rows
        .iter()
        .filter(|row| row.status == STATUS_COVERED)
        .count();

    for row in &rows {
        if row.status == STATUS_COVERED {
            validate_covered_row(row, inputs, &mut errors);
        } else if row.status == STATUS_MISSING {
            validate_missing_row(row, &mut errors);
        } else {
            errors.push(format!(
                "{} has unsupported survival status: {}",
                row.system, row.status
            ));
        }
    }

    if missing_rows == 0 {
        if covered_rows != REQUIRED_ROW_COUNT {
            errors.push(
                "all rows are no longer missing but required coverage is incomplete".to_string(),
            );
        }
    } else {
        validate_non_claims_while_missing(inputs, &mut errors);
    }

    if errors.is_empty() {
        Ok(GateSummary {
            covered_rows,
            missing_rows,
        })
    } else {
        Err(errors)
    }
}

fn validate_non_claims_while_missing(inputs: &GateInputs, errors: &mut Vec<String>) {
    let combined = format!(
        "{}\n{}\n{}",
        inputs.survival_matrix, inputs.current_bundle, inputs.acceptance_matrix
    )
    .to_lowercase();
    for forbidden in FORBIDDEN_WHILE_MISSING {
        if combined.contains(forbidden) {
            errors.push(format!(
                "premature full-survival claim while required rows are missing: {forbidden}"
            ));
        }
    }
    if !combined.contains("full_survival_compatibility remains a non-claim")
        && !combined.contains("full survival compatibility")
    {
        errors
            .push("full survival non-claim text is missing while rows are incomplete".to_string());
    }
}

fn validate_covered_row(row: &SurvivalRow, inputs: &GateInputs, errors: &mut Vec<String>) {
    if row.valence_evidence == EMPTY_EVIDENCE {
        errors.push(format!("{} covered row lacks Valence evidence", row.system));
    }
    if row.reference_evidence == EMPTY_EVIDENCE {
        errors.push(format!(
            "{} covered row lacks Paper/reference evidence",
            row.system
        ));
    }

    for evidence_path in evidence_paths_for_row(row) {
        if !inputs.manifest_paths.contains(&evidence_path) {
            errors.push(format!(
                "{} evidence path lacks BLAKE3 manifest linkage: {evidence_path}",
                row.system
            ));
        }
    }

    if let Some(expectation) = CURRENT_COVERED_ROWS
        .iter()
        .find(|expectation| expectation.system == row.system)
    {
        validate_acceptance_revision_metadata(expectation, &inputs.acceptance_matrix, errors);
    }
}

fn validate_acceptance_revision_metadata(
    expectation: &CoveredRowExpectation,
    acceptance_matrix: &str,
    errors: &mut Vec<String>,
) {
    let Some(line) = acceptance_matrix
        .lines()
        .find(|line| line.contains(expectation.acceptance_token))
    else {
        errors.push(format!(
            "{} missing acceptance matrix row",
            expectation.system
        ));
        return;
    };
    if !line.contains("Valence") || !line.contains("Stevenarella") {
        errors.push(format!(
            "{} acceptance row lacks child revision metadata or oracle text",
            expectation.system
        ));
    }
}

fn validate_missing_row(row: &SurvivalRow, errors: &mut Vec<String>) {
    if row.valence_evidence != EMPTY_EVIDENCE || row.reference_evidence != EMPTY_EVIDENCE {
        errors.push(format!(
            "{} missing row unexpectedly cites evidence",
            row.system
        ));
    }
    if !row.non_claim.to_lowercase().contains("no ") {
        errors.push(format!(
            "{} missing row lacks explicit non-claim",
            row.system
        ));
    }
}

fn evidence_paths_for_row(row: &SurvivalRow) -> BTreeSet<String> {
    let mut paths = BTreeSet::new();
    collect_paths(&row.valence_evidence, &mut paths);
    collect_paths(&row.reference_evidence, &mut paths);
    collect_paths(&row.requirement, &mut paths);
    paths
}

fn collect_paths(text: &str, paths: &mut BTreeSet<String>) {
    for token in text.split(|character: char| character.is_whitespace() || character == '`') {
        let candidate = token.trim_matches(|character: char| {
            character == ',' || character == '.' || character == ';' || character == ':'
        });
        if candidate.starts_with(DOCS_EVIDENCE_PREFIX) {
            paths.insert(candidate.to_string());
        }
    }
}

fn rows_by_system<'a>(rows: &'a [SurvivalRow]) -> BTreeMap<&'a str, &'a SurvivalRow> {
    rows.iter().map(|row| (row.system.as_str(), row)).collect()
}

fn parse_rows(text: &str, errors: &mut Vec<String>) -> Vec<SurvivalRow> {
    table_rows(text)
        .into_iter()
        .filter_map(|cells| match cells.as_slice() {
            [system, status, valence_evidence, reference_evidence, requirement, non_claim, _next_action] => {
                Some(SurvivalRow {
                    system: system.clone(),
                    status: status.clone(),
                    valence_evidence: valence_evidence.clone(),
                    reference_evidence: reference_evidence.clone(),
                    requirement: requirement.clone(),
                    non_claim: non_claim.clone(),
                })
            }
            _ => {
                errors.push(format!(
                    "survival coverage row has wrong cell count: expected {TABLE_CELL_COUNT}, found {}",
                    cells.len()
                ));
                None
            }
        })
        .collect()
}

fn table_rows(text: &str) -> Vec<Vec<String>> {
    let mut rows = Vec::new();
    let mut in_table = false;
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed == TABLE_HEADER {
            in_table = true;
            continue;
        }
        if in_table && trimmed.starts_with("## ") {
            break;
        }
        if !in_table || trimmed.starts_with("| ---") {
            continue;
        }
        if let Some(cells) = table_row(trimmed) {
            rows.push(cells);
        }
    }
    rows
}

fn table_row(line: &str) -> Option<Vec<String>> {
    if !line.starts_with("| ") {
        return None;
    }
    Some(
        line.trim_matches('|')
            .split('|')
            .map(str::trim)
            .map(ToOwned::to_owned)
            .collect(),
    )
}

fn run_self_tests() -> Result<String, Vec<String>> {
    let manifest_paths = manifest_paths_from_text(&all_rows_manifest_text());
    let all_covered_inputs = GateInputs {
        survival_matrix: fixture_doc(&all_covered_rows()),
        current_bundle: "Full survival compatibility is covered after all required rows passed."
            .to_string(),
        acceptance_matrix: fixture_acceptance_matrix(),
        manifest_paths: manifest_paths.clone(),
    };
    let all_covered = validate_gate(&all_covered_inputs)?;
    if all_covered.missing_rows != 0 {
        return Err(vec![
            "all-covered fixture still has missing rows".to_string()
        ]);
    }

    let current_missing_inputs = GateInputs {
        survival_matrix: fixture_doc(&current_missing_rows()),
        current_bundle: "This bundle still does not claim full survival compatibility.".to_string(),
        acceptance_matrix: fixture_acceptance_matrix(),
        manifest_paths: manifest_paths.clone(),
    };
    validate_gate(&current_missing_inputs)?;

    let missing_row = current_missing_rows().replacen("| crafting |", "| crafting-missing |", 1);
    assert_contains(
        &validate_gate(&GateInputs {
            survival_matrix: fixture_doc(&missing_row),
            current_bundle: current_missing_inputs.current_bundle.clone(),
            acceptance_matrix: current_missing_inputs.acceptance_matrix.clone(),
            manifest_paths: manifest_paths.clone(),
        })
        .expect_err("missing row fixture should fail"),
        "missing required survival row: crafting",
    )?;

    let valence_only =
        current_missing_rows().replacen("`docs/evidence/crafting-paper.receipt.json`", "none", 1);
    assert_contains(
        &validate_gate(&GateInputs {
            survival_matrix: fixture_doc(&valence_only),
            current_bundle: current_missing_inputs.current_bundle.clone(),
            acceptance_matrix: current_missing_inputs.acceptance_matrix.clone(),
            manifest_paths: manifest_paths.clone(),
        })
        .expect_err("Valence-only fixture should fail"),
        "lacks Paper/reference evidence",
    )?;

    let missing_manifest = validate_gate(&GateInputs {
        survival_matrix: fixture_doc(&current_missing_rows()),
        current_bundle: current_missing_inputs.current_bundle.clone(),
        acceptance_matrix: current_missing_inputs.acceptance_matrix.clone(),
        manifest_paths: BTreeSet::new(),
    })
    .expect_err("missing manifest fixture should fail");
    assert_contains(&missing_manifest, "BLAKE3 manifest linkage")?;

    let stale_revision = validate_gate(&GateInputs {
        survival_matrix: fixture_doc(&current_missing_rows()),
        current_bundle: current_missing_inputs.current_bundle.clone(),
        acceptance_matrix: fixture_acceptance_matrix().replace("Valence", "NoChildRev"),
        manifest_paths: manifest_paths.clone(),
    })
    .expect_err("stale revision fixture should fail");
    assert_contains(&stale_revision, "child revision")?;

    let stale_nonclaim = current_missing_rows().replacen(
        "| redstone | missing | none | none | Add redstone receipts. | No redstone coverage. | next |",
        "| redstone | missing | none | none | Add redstone receipts. | pending | next |",
        1,
    );
    assert_contains(
        &validate_gate(&GateInputs {
            survival_matrix: fixture_doc(&stale_nonclaim),
            current_bundle: current_missing_inputs.current_bundle.clone(),
            acceptance_matrix: current_missing_inputs.acceptance_matrix.clone(),
            manifest_paths: manifest_paths.clone(),
        })
        .expect_err("stale non-claim fixture should fail"),
        "lacks explicit non-claim",
    )?;

    let overclaim = validate_gate(&GateInputs {
        survival_matrix: format!(
            "{}\nfull survival compatibility is covered\n",
            fixture_doc(&current_missing_rows())
        ),
        current_bundle: current_missing_inputs.current_bundle,
        acceptance_matrix: current_missing_inputs.acceptance_matrix,
        manifest_paths,
    })
    .expect_err("premature full-survival claim should fail");
    assert_contains(&overclaim, "premature full-survival claim")?;

    Ok("all-covered success and fail-closed fixtures exercised".to_string())
}

fn assert_contains(errors: &[String], needle: &str) -> Result<(), Vec<String>> {
    if errors.iter().any(|error| error.contains(needle)) {
        Ok(())
    } else {
        Err(vec![format!(
            "missing expected diagnostic {needle:?}: {errors:?}"
        )])
    }
}

fn fixture_doc(rows: &str) -> String {
    format!(
        "# Fixture\n\n{TABLE_HEADER}\n| --- | --- | --- | --- | --- | --- | --- |\n{rows}\n\n## Gate\n\nfull_survival_compatibility remains a non-claim.\n"
    )
}

fn current_missing_rows() -> String {
    [
        covered_row("break/place/pickup", "break", "Break"),
        covered_row("crafting", "crafting", "Crafting"),
        covered_row("chest persistence", "chest", "Chest"),
        "| furnace persistence | missing | none | none | Add furnace receipts. | No furnace coverage. | next |".to_string(),
        "| hunger/food | missing | none | none | Add hunger receipts. | No hunger or food coverage. | next |".to_string(),
        "| mob drops | missing | none | none | Add mob receipts. | No mob AI or mob drop coverage. | next |".to_string(),
        "| redstone | missing | none | none | Add redstone receipts. | No redstone coverage. | next |".to_string(),
        "| biome/dimension | missing | none | none | Add biome receipts. | No biome or dimension coverage. | next |".to_string(),
        "| world persistence | missing | none | none | Add persistence receipts. | No world persistence coverage. | next |".to_string(),
    ]
    .join("\n")
}

fn all_covered_rows() -> String {
    REQUIRED_SYSTEMS
        .iter()
        .map(|system| covered_row(system, &system.replace(['/', ' '], "-"), system))
        .collect::<Vec<_>>()
        .join("\n")
}

fn covered_row(system: &str, slug: &str, label: &str) -> String {
    format!(
        "| {system} | {STATUS_COVERED} | `docs/evidence/{slug}-valence.receipt.json` | `docs/evidence/{slug}-paper.receipt.json` | Paired comparator evidence: `docs/evidence/{slug}.md`. | No broad claim beyond {label}. | next |"
    )
}

fn fixture_acceptance_matrix() -> String {
    [
        "| Survival break/place/pickup | command | receipt | doc | digest | parent, Valence, Stevenarella | claim | nonclaim |",
        "| Survival crafting table | command | receipt | doc | digest | parent, Valence, Stevenarella | claim | nonclaim |",
        "| Survival chest persistence | command | receipt | doc | digest | parent, Valence, Stevenarella | claim | nonclaim |",
    ]
    .join("\n")
}

fn all_rows_manifest_text() -> String {
    let mut text = String::new();
    for system in REQUIRED_SYSTEMS {
        let slug = system.replace(['/', ' '], "-");
        text.push_str(&format!(
            "digest  docs/evidence/{slug}-valence.receipt.json\n"
        ));
        text.push_str(&format!(
            "digest  docs/evidence/{slug}-paper.receipt.json\n"
        ));
        text.push_str(&format!("digest  docs/evidence/{slug}.md\n"));
    }
    text.push_str("digest  docs/evidence/break-valence.receipt.json\n");
    text.push_str("digest  docs/evidence/break-paper.receipt.json\n");
    text.push_str("digest  docs/evidence/break.md\n");
    text.push_str("digest  docs/evidence/crafting-valence.receipt.json\n");
    text.push_str("digest  docs/evidence/crafting-paper.receipt.json\n");
    text.push_str("digest  docs/evidence/crafting.md\n");
    text.push_str("digest  docs/evidence/chest-valence.receipt.json\n");
    text.push_str("digest  docs/evidence/chest-paper.receipt.json\n");
    text.push_str("digest  docs/evidence/chest.md\n");
    text
}
