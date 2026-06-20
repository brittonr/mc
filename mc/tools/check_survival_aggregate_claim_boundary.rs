use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const SELF_TEST_FLAG: &str = "--self-test";
const SURVIVAL_MATRIX_PATH: &str =
    "docs/evidence/protocol-763-survival-coverage-matrix-2026-05-28.md";
const CURRENT_BUNDLE_PATH: &str = "docs/evidence/protocol-763-current-evidence-bundle.md";
const ACCEPTANCE_MATRIX_PATH: &str = "docs/evidence/protocol-763-acceptance-matrix.md";
const EVIDENCE_DIR: &str = "docs/evidence";
const B3_EXTENSION: &str = "b3";
const B3_SUFFIX: &str = ".b3";
const DOCS_EVIDENCE_PREFIX: &str = "docs/evidence/";
const MANIFEST_SEPARATOR: &str = "  ";
const TABLE_HEADER: &str = "| Survival aggregate prerequisite | Status | Valence evidence | Reference evidence | Comparator/evidence doc | Manifest | Claim vocabulary |";
const TABLE_CELL_COUNT: usize = 7;
const STATUS_PENDING: &str = "pending_breadth_evidence";
const STATUS_AGGREGATE_COVERED: &str = "aggregate_reference_parity_covered";
const EMPTY_EVIDENCE: &str = "none";
const GATE_CHECK_NAME: &str = "mc-compat-survival-aggregate-claim-boundary";
const NON_CLAIM_TOKEN: &str = "full_survival_compatibility remains a non-claim";
const ROW_SCOPED_TOKEN: &str = "row-scoped reference parity";
const BOUNDED_ROW_TOKEN: &str = "bounded survival row";
const BLOCKED_TOKEN: &str = "aggregate survival claim blocked";
const AGGREGATE_BUNDLE_TOKEN: &str = "aggregate survival evidence bundle passed";
const SUCCESS: ExitCode = ExitCode::SUCCESS;
const FAILURE: ExitCode = ExitCode::FAILURE;

const REQUIRED_PREREQUISITES: &[&str] = &[
    "crafting recipe breadth",
    "furnace smelting breadth",
    "hunger health cycle",
    "mob AI loot breadth",
    "redstone circuit breadth",
    "biome dimension travel",
    "world multichunk durability",
    "container block entity breadth",
    "sign editing live parity",
];

const FORBIDDEN_PREMATURE_CLAIMS: &[&str] = &[
    "full_survival_compatibility is covered",
    "full survival compatibility is covered",
    "full survival compatibility passes",
    "full survival compatibility proven",
    "broad vanilla survival parity is covered",
    "broad vanilla parity is covered",
    "aggregate survival parity is covered",
];

const REQUIRED_BOUNDARY_TOKENS: &[&str] = &[
    GATE_CHECK_NAME,
    ROW_SCOPED_TOKEN,
    BOUNDED_ROW_TOKEN,
    BLOCKED_TOKEN,
    NON_CLAIM_TOKEN,
];

#[derive(Debug, Clone, PartialEq, Eq)]
struct BoundaryInputs {
    survival_matrix: String,
    current_bundle: String,
    acceptance_matrix: String,
    manifest_catalog: ManifestCatalog,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ManifestCatalog {
    files: BTreeSet<String>,
    covered_paths: BTreeSet<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BoundaryRow {
    prerequisite: String,
    status: String,
    valence_evidence: String,
    reference_evidence: String,
    comparator_doc: String,
    manifest: String,
    claim_vocabulary: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BoundarySummary {
    prerequisites: usize,
    covered: usize,
    pending: usize,
}

fn main() -> ExitCode {
    let args = env::args().collect::<Vec<_>>();
    if args.iter().any(|arg| arg == SELF_TEST_FLAG) {
        return match run_self_tests() {
            Ok(summary) => {
                println!("survival aggregate claim boundary self-test ok: {summary}");
                SUCCESS
            }
            Err(errors) => {
                print_errors(&errors);
                FAILURE
            }
        };
    }

    match read_repo_inputs(Path::new(".")).and_then(|inputs| validate_boundary(&inputs)) {
        Ok(summary) => {
            println!(
                "survival aggregate claim boundary passed: {} prerequisites checked, {} covered, {} pending",
                summary.prerequisites, summary.covered, summary.pending
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
        eprintln!("survival aggregate claim boundary failed: {error}");
    }
}

fn read_repo_inputs(root: &Path) -> Result<BoundaryInputs, Vec<String>> {
    Ok(BoundaryInputs {
        survival_matrix: read_file(root, SURVIVAL_MATRIX_PATH)?,
        current_bundle: read_file(root, CURRENT_BUNDLE_PATH)?,
        acceptance_matrix: read_file(root, ACCEPTANCE_MATRIX_PATH)?,
        manifest_catalog: read_manifest_catalog(root, Path::new(EVIDENCE_DIR))?,
    })
}

fn read_file(root: &Path, relative_path: &str) -> Result<String, Vec<String>> {
    fs::read_to_string(root.join(relative_path))
        .map_err(|error| vec![format!("{relative_path}: {error}")])
}

fn read_manifest_catalog(root: &Path, evidence_dir: &Path) -> Result<ManifestCatalog, Vec<String>> {
    let mut catalog = ManifestCatalog {
        files: BTreeSet::new(),
        covered_paths: BTreeSet::new(),
    };
    let directory = root.join(evidence_dir);
    let entries = fs::read_dir(&directory)
        .map_err(|error| vec![format!("{}: {error}", directory.display())])?;
    for entry_result in entries {
        let entry =
            entry_result.map_err(|error| vec![format!("{}: {error}", directory.display())])?;
        let path = entry.path();
        if path.is_file() && path.extension() == Some(OsStr::new(B3_EXTENSION)) {
            let relative = relative_path(root, &path)?;
            catalog.files.insert(relative);
            catalog.covered_paths.extend(read_manifest_file(&path)?);
        }
    }
    Ok(catalog)
}

fn relative_path(root: &Path, path: &PathBuf) -> Result<String, Vec<String>> {
    path.strip_prefix(root)
        .map_err(|error| vec![format!("{}: {error}", path.display())])?
        .to_str()
        .map(|text| text.replace('\\', "/"))
        .ok_or_else(|| vec![format!("{} is not valid UTF-8", path.display())])
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

fn validate_boundary(inputs: &BoundaryInputs) -> Result<BoundarySummary, Vec<String>> {
    let mut errors = Vec::new();
    let rows = parse_boundary_rows(&inputs.survival_matrix, &mut errors);
    let rows_by_prerequisite = rows_by_prerequisite(&rows);

    validate_required_boundary_tokens(inputs, &mut errors);

    if rows.len() != REQUIRED_PREREQUISITES.len() {
        errors.push(format!(
            "expected {} aggregate survival prerequisites, found {}",
            REQUIRED_PREREQUISITES.len(),
            rows.len()
        ));
    }

    for prerequisite in REQUIRED_PREREQUISITES {
        if !rows_by_prerequisite.contains_key(prerequisite) {
            errors.push(format!(
                "missing aggregate survival prerequisite: {prerequisite}"
            ));
        }
    }

    let mut covered = 0;
    let mut pending = 0;
    for row in &rows {
        match row.status.as_str() {
            STATUS_PENDING => {
                pending += 1;
                validate_pending_row(row, &mut errors);
            }
            STATUS_AGGREGATE_COVERED => {
                covered += 1;
                validate_covered_row(row, &inputs.manifest_catalog, &mut errors);
            }
            other => errors.push(format!(
                "{} has unsupported aggregate survival status: {other}",
                row.prerequisite
            )),
        }
    }

    if pending == 0 && covered == REQUIRED_PREREQUISITES.len() {
        validate_aggregate_claim_enabled(inputs, &mut errors);
    } else {
        validate_aggregate_claim_blocked(inputs, &mut errors);
    }

    if errors.is_empty() {
        Ok(BoundarySummary {
            prerequisites: rows.len(),
            covered,
            pending,
        })
    } else {
        Err(errors)
    }
}

fn validate_required_boundary_tokens(inputs: &BoundaryInputs, errors: &mut Vec<String>) {
    let combined = combined_docs(inputs).to_lowercase();
    for token in REQUIRED_BOUNDARY_TOKENS {
        if !combined.contains(&token.to_lowercase()) {
            errors.push(format!(
                "aggregate boundary missing required token: {token}"
            ));
        }
    }
}

fn validate_pending_row(row: &BoundaryRow, errors: &mut Vec<String>) {
    for (label, value) in [
        ("Valence evidence", &row.valence_evidence),
        ("Paper/reference evidence", &row.reference_evidence),
        ("comparator/evidence doc", &row.comparator_doc),
        ("manifest", &row.manifest),
    ] {
        if value != EMPTY_EVIDENCE {
            errors.push(format!(
                "{} pending row unexpectedly cites {label}: {value}",
                row.prerequisite
            ));
        }
    }
    let vocabulary = row.claim_vocabulary.to_lowercase();
    if !vocabulary.contains("non-claim") && !vocabulary.contains("blocked") {
        errors.push(format!(
            "{} pending row lacks blocked/non-claim vocabulary",
            row.prerequisite
        ));
    }
}

fn validate_covered_row(
    row: &BoundaryRow,
    manifest_catalog: &ManifestCatalog,
    errors: &mut Vec<String>,
) {
    if row.valence_evidence == EMPTY_EVIDENCE {
        errors.push(format!(
            "{} covered row lacks Valence evidence",
            row.prerequisite
        ));
    }
    if row.reference_evidence == EMPTY_EVIDENCE {
        errors.push(format!(
            "{} covered row lacks Paper/reference evidence",
            row.prerequisite
        ));
    }
    if row.comparator_doc == EMPTY_EVIDENCE {
        errors.push(format!(
            "{} covered row lacks comparator/evidence doc",
            row.prerequisite
        ));
    }
    validate_manifest_cell(row, manifest_catalog, errors);
    validate_evidence_manifest_linkage(row, manifest_catalog, errors);
}

fn validate_manifest_cell(
    row: &BoundaryRow,
    manifest_catalog: &ManifestCatalog,
    errors: &mut Vec<String>,
) {
    let manifest_paths = collect_docs_evidence_paths(&row.manifest);
    if manifest_paths.is_empty() {
        errors.push(format!(
            "{} covered row lacks manifest path",
            row.prerequisite
        ));
        return;
    }
    for manifest_path in manifest_paths {
        if !manifest_path.ends_with(B3_SUFFIX) {
            errors.push(format!(
                "{} manifest path is not a .b3 file: {manifest_path}",
                row.prerequisite
            ));
        }
        if !manifest_catalog.files.contains(&manifest_path) {
            errors.push(format!(
                "{} manifest file is missing or stale: {manifest_path}",
                row.prerequisite
            ));
        }
    }
}

fn validate_evidence_manifest_linkage(
    row: &BoundaryRow,
    manifest_catalog: &ManifestCatalog,
    errors: &mut Vec<String>,
) {
    for evidence_path in evidence_paths_for_row(row) {
        if !manifest_catalog.covered_paths.contains(&evidence_path) {
            errors.push(format!(
                "{} evidence path lacks BLAKE3 manifest linkage: {evidence_path}",
                row.prerequisite
            ));
        }
    }
}

fn evidence_paths_for_row(row: &BoundaryRow) -> BTreeSet<String> {
    let mut paths = BTreeSet::new();
    paths.extend(collect_docs_evidence_paths(&row.valence_evidence));
    paths.extend(collect_docs_evidence_paths(&row.reference_evidence));
    paths.extend(collect_docs_evidence_paths(&row.comparator_doc));
    paths
        .into_iter()
        .filter(|path| !path.ends_with(B3_SUFFIX))
        .collect()
}

fn collect_docs_evidence_paths(text: &str) -> BTreeSet<String> {
    text.split(|character: char| character.is_whitespace() || character == '`')
        .map(|token| {
            token.trim_matches(|character: char| {
                matches!(character, ',' | '.' | ';' | ':' | ')' | ']' | '(' | '[')
            })
        })
        .filter(|candidate| candidate.starts_with(DOCS_EVIDENCE_PREFIX))
        .map(ToOwned::to_owned)
        .collect()
}

fn validate_aggregate_claim_enabled(inputs: &BoundaryInputs, errors: &mut Vec<String>) {
    let combined = combined_docs(inputs).to_lowercase();
    if !combined.contains(AGGREGATE_BUNDLE_TOKEN) {
        errors.push(format!(
            "all prerequisites are covered but {AGGREGATE_BUNDLE_TOKEN:?} is missing"
        ));
    }
}

fn validate_aggregate_claim_blocked(inputs: &BoundaryInputs, errors: &mut Vec<String>) {
    let combined = combined_docs(inputs).to_lowercase();
    for forbidden in FORBIDDEN_PREMATURE_CLAIMS {
        if combined.contains(forbidden) {
            errors.push(format!(
                "premature aggregate survival claim while prerequisites are pending: {forbidden}"
            ));
        }
    }
    if !combined.contains(NON_CLAIM_TOKEN) {
        errors.push(format!(
            "aggregate survival boundary lacks required non-claim token: {NON_CLAIM_TOKEN}"
        ));
    }
}

fn combined_docs(inputs: &BoundaryInputs) -> String {
    format!(
        "{}\n{}\n{}",
        inputs.survival_matrix, inputs.current_bundle, inputs.acceptance_matrix
    )
}

fn rows_by_prerequisite<'a>(rows: &'a [BoundaryRow]) -> BTreeMap<&'a str, &'a BoundaryRow> {
    rows.iter()
        .map(|row| (row.prerequisite.as_str(), row))
        .collect()
}

fn parse_boundary_rows(text: &str, errors: &mut Vec<String>) -> Vec<BoundaryRow> {
    table_rows(text)
        .into_iter()
        .filter_map(|cells| match cells.as_slice() {
            [
                prerequisite,
                status,
                valence_evidence,
                reference_evidence,
                comparator_doc,
                manifest,
                claim_vocabulary,
            ] => Some(BoundaryRow {
                prerequisite: prerequisite.clone(),
                status: status.clone(),
                valence_evidence: valence_evidence.clone(),
                reference_evidence: reference_evidence.clone(),
                comparator_doc: comparator_doc.clone(),
                manifest: manifest.clone(),
                claim_vocabulary: claim_vocabulary.clone(),
            }),
            _ => {
                errors.push(format!(
                    "aggregate survival boundary row has wrong cell count: expected {TABLE_CELL_COUNT}, found {}",
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
    let all_covered_inputs = BoundaryInputs {
        survival_matrix: fixture_doc(&all_covered_rows()),
        current_bundle: format!(
            "{AGGREGATE_BUNDLE_TOKEN}; full survival compatibility is covered after all prerequisites pass."
        ),
        acceptance_matrix: fixture_acceptance_matrix(),
        manifest_catalog: all_covered_manifest_catalog(),
    };
    let all_covered = validate_boundary(&all_covered_inputs)?;
    if all_covered.pending != 0 {
        return Err(vec![
            "all-covered aggregate fixture still has pending rows".to_string()
        ]);
    }

    let pending_inputs = BoundaryInputs {
        survival_matrix: fixture_doc(&pending_rows()),
        current_bundle: fixture_current_bundle(),
        acceptance_matrix: fixture_acceptance_matrix(),
        manifest_catalog: ManifestCatalog {
            files: BTreeSet::new(),
            covered_paths: BTreeSet::new(),
        },
    };
    validate_boundary(&pending_inputs)?;

    let missing_row = pending_rows().replacen(
        "| crafting recipe breadth | pending_breadth_evidence | none | none | none | none | aggregate survival claim blocked; non-claim |\n",
        "",
        1,
    );
    assert_contains(
        &validate_boundary(&BoundaryInputs {
            survival_matrix: fixture_doc(&missing_row),
            ..pending_inputs.clone()
        })
        .expect_err("missing prerequisite fixture should fail"),
        "missing aggregate survival prerequisite: crafting recipe breadth",
    )?;

    assert_contains(
        &validate_boundary(&BoundaryInputs {
            manifest_catalog: ManifestCatalog {
                files: BTreeSet::new(),
                covered_paths: BTreeSet::new(),
            },
            ..all_covered_inputs.clone()
        })
        .expect_err("stale manifest fixture should fail"),
        "BLAKE3 manifest linkage",
    )?;

    let valence_only_rows = all_covered_rows().replacen(
        "`docs/evidence/crafting-recipe-breadth-paper.receipt.json`",
        EMPTY_EVIDENCE,
        1,
    );
    assert_contains(
        &validate_boundary(&BoundaryInputs {
            survival_matrix: fixture_doc(&valence_only_rows),
            ..all_covered_inputs.clone()
        })
        .expect_err("Valence-only fixture should fail"),
        "lacks Paper/reference evidence",
    )?;

    assert_contains(
        &validate_boundary(&BoundaryInputs {
            current_bundle: "full survival compatibility is covered before prerequisites pass"
                .to_string(),
            ..pending_inputs.clone()
        })
        .expect_err("broad overclaim fixture should fail"),
        "premature aggregate survival claim",
    )?;

    Ok("all-covered success, pending success, missing row, stale evidence, Valence-only, and overclaim fixtures exercised".to_string())
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
        "# Fixture\n\n{GATE_CHECK_NAME} protects {ROW_SCOPED_TOKEN}; every {BOUNDED_ROW_TOKEN} keeps {BLOCKED_TOKEN}; {NON_CLAIM_TOKEN}.\n\n{TABLE_HEADER}\n| --- | --- | --- | --- | --- | --- | --- |\n{rows}\n"
    )
}

fn pending_rows() -> String {
    REQUIRED_PREREQUISITES
        .iter()
        .map(|prerequisite| {
            format!(
                "| {prerequisite} | {STATUS_PENDING} | {EMPTY_EVIDENCE} | {EMPTY_EVIDENCE} | {EMPTY_EVIDENCE} | {EMPTY_EVIDENCE} | {BLOCKED_TOKEN}; non-claim |"
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn all_covered_rows() -> String {
    REQUIRED_PREREQUISITES
        .iter()
        .map(|prerequisite| covered_row(prerequisite, &slug_for_prerequisite(prerequisite)))
        .collect::<Vec<_>>()
        .join("\n")
}

fn covered_row(prerequisite: &str, slug: &str) -> String {
    format!(
        "| {prerequisite} | {STATUS_AGGREGATE_COVERED} | `docs/evidence/{slug}-valence.receipt.json` | `docs/evidence/{slug}-paper.receipt.json` | `docs/evidence/{slug}-parity.md` | `docs/evidence/{slug}.b3` | aggregate claim prerequisite satisfied |"
    )
}

fn slug_for_prerequisite(prerequisite: &str) -> String {
    prerequisite.replace(['/', ' '], "-").to_lowercase()
}

fn fixture_current_bundle() -> String {
    format!(
        "{GATE_CHECK_NAME} records {BLOCKED_TOKEN}; {ROW_SCOPED_TOKEN} remains bounded and {NON_CLAIM_TOKEN}."
    )
}

fn fixture_acceptance_matrix() -> String {
    format!(
        "{GATE_CHECK_NAME} points broad survival claims at pending prerequisites; {ROW_SCOPED_TOKEN}; {NON_CLAIM_TOKEN}."
    )
}

fn all_covered_manifest_catalog() -> ManifestCatalog {
    let mut files = BTreeSet::new();
    let mut covered_paths = BTreeSet::new();
    for prerequisite in REQUIRED_PREREQUISITES {
        let slug = slug_for_prerequisite(prerequisite);
        files.insert(format!("docs/evidence/{slug}.b3"));
        covered_paths.insert(format!("docs/evidence/{slug}-valence.receipt.json"));
        covered_paths.insert(format!("docs/evidence/{slug}-paper.receipt.json"));
        covered_paths.insert(format!("docs/evidence/{slug}-parity.md"));
    }
    ManifestCatalog {
        files,
        covered_paths,
    }
}
