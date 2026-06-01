use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

const SELF_TEST_FLAG: &str = "--self-test";
const MATRIX_DOC_PATH: &str = "docs/evidence/protocol-763-survival-coverage-matrix-2026-05-28.md";
const CURRENT_BUNDLE_PATH: &str = "docs/evidence/protocol-763-current-evidence-bundle.md";
const ACCEPTANCE_MATRIX_PATH: &str = "docs/evidence/protocol-763-acceptance-matrix.md";
const SUCCESS: ExitCode = ExitCode::SUCCESS;
const FAILURE: ExitCode = ExitCode::FAILURE;
const TABLE_HEADER: &str = "| Survival system | Status | Valence evidence | Reference evidence | Promotion requirement | Explicit non-claim | Next action |";
const TABLE_CELL_COUNT: usize = 7;
const REQUIRED_ROW_COUNT: usize = 9;
const STATUS_MISSING: &str = "missing";
const REFERENCE_NONE: &str = "none";
const COVERED_STATUS: &str = "reference_parity_covered";
const BREAK_PLACE_PICKUP_ROW: &str = "break/place/pickup";
const CRAFTING_ROW: &str = "crafting";
const CHEST_ROW: &str = "chest persistence";
const BIOME_DIMENSION_ROW: &str = "biome/dimension";

const REQUIRED_SYSTEMS: &[&str] = &[
    BREAK_PLACE_PICKUP_ROW,
    CRAFTING_ROW,
    CHEST_ROW,
    "furnace persistence",
    "hunger/food",
    "mob drops",
    "redstone",
    BIOME_DIMENSION_ROW,
    "world persistence",
];

const REQUIRED_TEXT: &[&str] = &[
    "full_survival_compatibility remains a non-claim",
    "No full survival compatibility or broader vanilla parity",
    "No full survival compatibility from crafting row",
    "No full survival compatibility from chest persistence row",
    "No furnace coverage",
    "No hunger or food coverage",
    "No mob AI or mob drop coverage",
    "No redstone coverage",
    "No full survival compatibility from biome/dimension row",
    "No world persistence coverage",
    "paired reference receipt",
    "BLAKE3 manifest entries",
];

const FORBIDDEN_CLAIMS: &[&str] = &[
    "full_survival_compatibility is covered",
    "full survival compatibility is covered",
    "vanilla parity is covered",
    "full survival compatibility passes",
];

const BREAK_PLACE_PICKUP_PAPER_RECEIPT: &str =
    "docs/evidence/protocol-763-survival-reference-paper-2026-05-28.receipt.json";
const BREAK_PLACE_PICKUP_VALENCE_RECEIPT: &str =
    "docs/evidence/protocol-763-survival-reference-valence-2026-05-28.receipt.json";
const BREAK_PLACE_PICKUP_EVIDENCE_DOC: &str =
    "docs/evidence/protocol-763-survival-reference-parity-2026-05-28.md";
const CRAFTING_PAPER_RECEIPT: &str =
    "docs/evidence/protocol-763-survival-crafting-table-paper-2026-05-31.receipt.json";
const CRAFTING_VALENCE_RECEIPT: &str =
    "docs/evidence/protocol-763-survival-crafting-table-valence-2026-05-31.receipt.json";
const CRAFTING_EVIDENCE_DOC: &str =
    "docs/evidence/protocol-763-survival-crafting-table-2026-05-31.md";
const CHEST_PAPER_RECEIPT: &str =
    "docs/evidence/protocol-763-survival-chest-persistence-paper-2026-05-29.receipt.json";
const CHEST_VALENCE_RECEIPT: &str =
    "docs/evidence/protocol-763-survival-chest-persistence-valence-2026-05-29.receipt.json";
const CHEST_EVIDENCE_DOC: &str =
    "docs/evidence/protocol-763-survival-chest-persistence-2026-05-29.md";
const BIOME_DIMENSION_PAPER_RECEIPT: &str =
    "docs/evidence/survival-biome-dimension-paper-2026-06-01.receipt.json";
const BIOME_DIMENSION_VALENCE_RECEIPT: &str =
    "docs/evidence/survival-biome-dimension-valence-2026-06-01.receipt.json";
const BIOME_DIMENSION_EVIDENCE_DOC: &str =
    "docs/evidence/survival-biome-dimension-receipts-2026-06-01.md";

#[derive(Debug, Clone, PartialEq, Eq)]
struct SurvivalRow {
    system: String,
    status: String,
    valence_evidence: String,
    reference_evidence: String,
    requirement: String,
    non_claim: String,
    next_action: String,
}

fn main() -> ExitCode {
    let args = env::args().collect::<Vec<_>>();
    if args.iter().any(|arg| arg == SELF_TEST_FLAG) {
        return match run_self_tests() {
            Ok(summary) => {
                println!("survival coverage matrix self-test ok: {summary}");
                SUCCESS
            }
            Err(errors) => {
                print_errors(&errors);
                FAILURE
            }
        };
    }

    match run_repo_check(Path::new(".")) {
        Ok(summary) => {
            println!("survival coverage matrix check passed: {summary}");
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
        eprintln!("survival coverage matrix check failed: {error}");
    }
}

fn run_repo_check(root: &Path) -> Result<String, Vec<String>> {
    let doc_text = read_file(root, MATRIX_DOC_PATH)?;
    let bundle_text = read_file(root, CURRENT_BUNDLE_PATH)?;
    let matrix_text = read_file(root, ACCEPTANCE_MATRIX_PATH)?;
    let errors = validate_text(&doc_text, &bundle_text, &matrix_text);
    if errors.is_empty() {
        Ok(format!("{} rows validated", REQUIRED_ROW_COUNT))
    } else {
        Err(errors)
    }
}

fn read_file(root: &Path, relative_path: &str) -> Result<String, Vec<String>> {
    let path = root.join(relative_path);
    fs::read_to_string(&path).map_err(|error| vec![format!("{}: {error}", path.display())])
}

fn validate_text(doc_text: &str, bundle_text: &str, matrix_text: &str) -> Vec<String> {
    let mut errors = Vec::new();
    let rows = parse_rows(doc_text, &mut errors);
    let rows_by_system = rows_by_system(&rows);

    if rows.len() != REQUIRED_ROW_COUNT {
        errors.push(format!(
            "expected {REQUIRED_ROW_COUNT} survival rows, found {}",
            rows.len()
        ));
    }
    for system in REQUIRED_SYSTEMS {
        if !rows_by_system.contains_key(system) {
            errors.push(format!("missing survival row: {system}"));
        }
    }
    for token in REQUIRED_TEXT {
        if !doc_text.contains(token) {
            errors.push(format!("survival matrix missing required text: {token}"));
        }
    }
    for forbidden in FORBIDDEN_CLAIMS {
        if doc_text.contains(forbidden)
            || bundle_text.contains(forbidden)
            || matrix_text.contains(forbidden)
        {
            errors.push(format!(
                "forbidden full-survival claim present: {forbidden}"
            ));
        }
    }

    for row in &rows {
        validate_row(row, &mut errors);
    }
    if !bundle_text
        .to_lowercase()
        .contains("full survival compatibility")
    {
        errors.push(
            "current bundle no longer names full survival compatibility non-claim".to_string(),
        );
    }
    if !matrix_text.contains("Full survival compatibility / vanilla parity") {
        errors
            .push("acceptance matrix no longer names full survival compatibility gap".to_string());
    }
    errors
}

fn rows_by_system<'a>(rows: &'a [SurvivalRow]) -> BTreeMap<&'a str, &'a SurvivalRow> {
    rows.iter().map(|row| (row.system.as_str(), row)).collect()
}

fn validate_row(row: &SurvivalRow, errors: &mut Vec<String>) {
    match row.system.as_str() {
        BREAK_PLACE_PICKUP_ROW => validate_covered_row(
            row,
            BREAK_PLACE_PICKUP_PAPER_RECEIPT,
            BREAK_PLACE_PICKUP_VALENCE_RECEIPT,
            BREAK_PLACE_PICKUP_EVIDENCE_DOC,
            "broader vanilla parity",
            "covered break/place/pickup row",
            errors,
        ),
        CRAFTING_ROW => validate_covered_row(
            row,
            CRAFTING_PAPER_RECEIPT,
            CRAFTING_VALENCE_RECEIPT,
            CRAFTING_EVIDENCE_DOC,
            "furnace",
            "covered crafting row",
            errors,
        ),
        CHEST_ROW => validate_covered_row(
            row,
            CHEST_PAPER_RECEIPT,
            CHEST_VALENCE_RECEIPT,
            CHEST_EVIDENCE_DOC,
            "all-container",
            "covered chest persistence row",
            errors,
        ),
        BIOME_DIMENSION_ROW => validate_covered_row(
            row,
            BIOME_DIMENSION_PAPER_RECEIPT,
            BIOME_DIMENSION_VALENCE_RECEIPT,
            BIOME_DIMENSION_EVIDENCE_DOC,
            "dimension travel",
            "covered biome/dimension row",
            errors,
        ),
        _ => validate_missing_row(row, errors),
    }
}

fn validate_covered_row(
    row: &SurvivalRow,
    paper_receipt: &str,
    valence_receipt: &str,
    evidence_doc: &str,
    scoped_nonclaim_token: &str,
    label: &str,
    errors: &mut Vec<String>,
) {
    if row.status != COVERED_STATUS {
        errors.push(format!("{label} has stale status: {}", row.status));
    }
    if !row.reference_evidence.contains(paper_receipt) {
        errors.push(format!("{label} missing Paper reference receipt"));
    }
    if !row.valence_evidence.contains(valence_receipt) {
        errors.push(format!("{label} missing Valence paired receipt"));
    }
    if !row.requirement.contains(evidence_doc) {
        errors.push(format!("{label} missing evidence doc"));
    }
    let non_claim = row.non_claim.to_lowercase();
    if !non_claim.contains("full survival compatibility")
        || !non_claim.contains(scoped_nonclaim_token)
    {
        errors.push(format!("{label} lacks scoped survival non-claim"));
    }
}

fn validate_missing_row(row: &SurvivalRow, errors: &mut Vec<String>) {
    if row.status != STATUS_MISSING {
        errors.push(format!(
            "unimplemented survival row is not marked missing: {}",
            row.system
        ));
    }
    if row.valence_evidence != REFERENCE_NONE || row.reference_evidence != REFERENCE_NONE {
        errors.push(format!(
            "unimplemented survival row unexpectedly cites evidence: {}",
            row.system
        ));
    }
    if !row.non_claim.contains("No ") {
        errors.push(format!(
            "unimplemented survival row lacks explicit non-claim: {}",
            row.system
        ));
    }
}

fn parse_rows(text: &str, errors: &mut Vec<String>) -> Vec<SurvivalRow> {
    table_rows(text)
        .into_iter()
        .filter_map(|cells| match cells.as_slice() {
            [system, status, valence_evidence, reference_evidence, requirement, non_claim, next_action] =>
            {
                Some(SurvivalRow {
                    system: system.clone(),
                    status: status.clone(),
                    valence_evidence: valence_evidence.clone(),
                    reference_evidence: reference_evidence.clone(),
                    requirement: requirement.clone(),
                    non_claim: non_claim.clone(),
                    next_action: next_action.clone(),
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
    let bundle = "Full survival compatibility remains a non-claim.";
    let matrix = "Full survival compatibility / vanilla parity";
    let doc = fixture_doc(&good_rows());
    let errors = validate_text(&doc, bundle, matrix);
    if !errors.is_empty() {
        return Err(errors);
    }

    let missing_row = good_rows().replacen("| crafting |", "| crafting-missing |", 1);
    assert_contains(
        &validate_text(&fixture_doc(&missing_row), bundle, matrix),
        "missing survival row: crafting",
    )?;

    let stale_status = good_rows().replacen(
        "| break/place/pickup | reference_parity_covered |",
        "| break/place/pickup | valence_covered_reference_missing |",
        1,
    );
    assert_contains(
        &validate_text(&fixture_doc(&stale_status), bundle, matrix),
        "stale status",
    )?;

    let valence_only = good_rows().replacen(CRAFTING_PAPER_RECEIPT, "none", 1);
    assert_contains(
        &validate_text(&fixture_doc(&valence_only), bundle, matrix),
        "missing Paper reference receipt",
    )?;

    let promoted_missing = good_rows().replacen(
        "| furnace persistence | missing | none | none |",
        "| furnace persistence | reference_parity_covered | `some-valence` | none |",
        1,
    );
    assert_contains(
        &validate_text(&fixture_doc(&promoted_missing), bundle, matrix),
        "unimplemented survival row is not marked missing",
    )?;

    let overclaim = format!(
        "{}\nfull survival compatibility is covered\n",
        fixture_doc(&good_rows())
    );
    assert_contains(
        &validate_text(&overclaim, bundle, matrix),
        "forbidden full-survival claim present",
    )?;

    Ok("positive and negative fixtures exercised".to_string())
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
        "# Fixture\n\n## Coverage rows\n\n{TABLE_HEADER}\n| --- | --- | --- | --- | --- | --- | --- |\n{rows}\n\n## Gate decision\n\nfull_survival_compatibility remains a non-claim.\n\npaired reference receipt\nBLAKE3 manifest entries\nNo vanilla parity or full survival compatibility\nNo full survival compatibility from crafting row\nNo full survival compatibility from chest persistence row\nNo furnace coverage\nNo hunger or food coverage\nNo mob AI or mob drop coverage\nNo redstone coverage\nNo full survival compatibility from biome/dimension row\nNo world persistence coverage\n"
    )
}

fn good_rows() -> String {
    [
        format!("| break/place/pickup | {COVERED_STATUS} | `{BREAK_PLACE_PICKUP_VALENCE_RECEIPT}` | `{BREAK_PLACE_PICKUP_PAPER_RECEIPT}` | Paired comparator evidence: `{BREAK_PLACE_PICKUP_EVIDENCE_DOC}`. | No full survival compatibility or broader vanilla parity. | next |"),
        format!("| crafting | {COVERED_STATUS} | `{CRAFTING_VALENCE_RECEIPT}` | `{CRAFTING_PAPER_RECEIPT}` | Paired comparator evidence: `{CRAFTING_EVIDENCE_DOC}`. | No full survival compatibility from crafting row; no furnace/hunger/mob/redstone/biome/dimension/world persistence coverage. | next |"),
        format!("| chest persistence | {COVERED_STATUS} | `{CHEST_VALENCE_RECEIPT}` | `{CHEST_PAPER_RECEIPT}` | Paired comparator evidence: `{CHEST_EVIDENCE_DOC}`. | No full survival compatibility from chest persistence row; no all-container behavior. | next |"),
        "| furnace persistence | missing | none | none | Add receipts. | No furnace coverage. | next |".to_string(),
        "| hunger/food | missing | none | none | Add receipts. | No hunger or food coverage. | next |".to_string(),
        "| mob drops | missing | none | none | Add receipts. | No mob AI or mob drop coverage. | next |".to_string(),
        "| redstone | missing | none | none | Add receipts. | No redstone coverage. | next |".to_string(),
        format!("| biome/dimension | {COVERED_STATUS} | `{BIOME_DIMENSION_VALENCE_RECEIPT}` | `{BIOME_DIMENSION_PAPER_RECEIPT}` | Paired comparator evidence: `{BIOME_DIMENSION_EVIDENCE_DOC}`. | No full survival compatibility from biome/dimension row; no biome lookup semantics, dimension travel, or world persistence coverage. | next |"),
        "| world persistence | missing | none | none | Add receipts. | No world persistence coverage. | next |".to_string(),
    ]
    .join("\n")
}
