use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

const SELF_TEST_FLAG: &str = "--self-test";
const MATRIX_PATH: &str = "docs/evidence/protocol-763-acceptance-matrix.md";
const EVIDENCE_TABLE_HEADER: &str = "| Seam | Maintained command | Receipt | Evidence doc | BLAKE3 | Landed commits | Scoped claim | Explicit non-claims |";
const EVIDENCE_ROW_CELLS: usize = 8;
const BLAKE3_HEX_LENGTH: usize = 64;
const JSON_SUFFIX: &str = ".json";
const MARKDOWN_SUFFIX: &str = ".md";
const DOCS_EVIDENCE_PREFIX: &str = "docs/evidence/";
const TARGET_PREFIX: &str = "target/";
const SUCCESS: ExitCode = ExitCode::SUCCESS;
const FAILURE: ExitCode = ExitCode::FAILURE;
const FORBIDDEN_COMMIT_MARKERS: &[&str] = &["current ", " diff", "untracked", "dirty"];

const REQUIRED_SEAMS: &[&str] = &[
    "RED/BLUE scoring soak",
    "Inventory/drop",
    "Block placement / use-item-on-block",
    "Pickup semantics",
    "Player-inventory click/container click",
    "Open-container semantics",
    "Inventory transaction packet family",
    "Two-client combat/damage",
    "Entity metadata packet family",
    "Flag-carrier death/return",
    "Reconnect flag-state",
    "Invalid flag pickup/ownership",
    "Invalid flag return/drop",
    "Score limit / win condition",
    "Simultaneous pickup/capture race",
    "Latency/jitter tolerance",
    "Combat knockback",
    "Armor equipment mitigation",
    "Equipment update observation",
    "Equipment permutation packet family",
    "Projectile use/loadout rail",
    "Projectile damage attribution",
    "Command/recipe/advancement packet family",
    "Chunk/biome packet family",
    "Survival break/place/pickup",
    "Survival chest persistence",
    "Survival crafting table",
    "Survival furnace persistence",
    "Survival biome/dimension join state",
    "MCP-controlled observability",
];

const REQUIRED_GAPS: &[&str] = &[
    "Residual combat breadth",
    "Broad protocol coverage",
    "Production load / multiplayer scale",
    "Full CTF correctness",
    "Full survival compatibility / vanilla parity",
];

const REQUIRED_TEXT: &[&str] = &[
    "Explicit non-claims",
    "Scoped claim",
    "BLAKE3",
    "Maintained command",
    "Receipt",
    "roi-04-latency-jitter-tolerance",
    "roi-05-projectile-armor-knockback-combat",
    "roi-07-post-drain-evidence-index",
    "projectile damage attribution",
    "projectile travel/collision simulation",
];

const REVIEWABLE_RECEIPT_SEAMS: &[&str] = &[
    "RED/BLUE scoring soak",
    "Invalid flag pickup/ownership",
    "Invalid flag return/drop",
    "Score limit / win condition",
    "Simultaneous pickup/capture race",
    "Inventory transaction packet family",
    "Entity metadata packet family",
    "Armor equipment mitigation",
    "Equipment update observation",
    "Equipment permutation packet family",
    "Projectile use/loadout rail",
    "Projectile damage attribution",
    "Command/recipe/advancement packet family",
    "Chunk/biome packet family",
    "Survival break/place/pickup",
    "Survival chest persistence",
    "Survival crafting table",
    "Survival furnace persistence",
    "Survival biome/dimension join state",
    "MCP-controlled observability",
];

const HISTORICAL_TARGET_RECEIPT_SEAMS: &[&str] = &[];

#[derive(Debug, Clone, PartialEq, Eq)]
struct EvidenceRow {
    seam: String,
    command: String,
    receipt: String,
    doc: String,
    blake3: String,
    commits: String,
    claim: String,
    non_claims: String,
}

fn main() -> ExitCode {
    let args = env::args().collect::<Vec<_>>();
    if args.iter().any(|arg| arg == SELF_TEST_FLAG) {
        return match run_self_tests() {
            Ok(summary) => {
                println!("acceptance matrix self-test ok: {summary}");
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
            println!("acceptance matrix check passed: {summary}");
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
        eprintln!("acceptance matrix check failed: {error}");
    }
}

fn run_repo_check(root: &Path) -> Result<String, Vec<String>> {
    let matrix_path = root.join(MATRIX_PATH);
    let matrix_text = fs::read_to_string(&matrix_path)
        .map_err(|error| vec![format!("{}: {error}", matrix_path.display())])?;
    let errors = validate_matrix_text(&matrix_text, Some(root), REQUIRED_SEAMS);
    if errors.is_empty() {
        Ok(format!("{} rows validated", REQUIRED_SEAMS.len()))
    } else {
        Err(errors)
    }
}

fn validate_matrix_text(text: &str, root: Option<&Path>, required_seams: &[&str]) -> Vec<String> {
    let (rows, mut errors) = parse_evidence_rows(text);
    let expected_rows = required_seams.len();
    if rows.len() != expected_rows {
        errors.push(format!(
            "expected {expected_rows} matrix evidence rows, found {}",
            rows.len()
        ));
    }

    let mut seen = BTreeSet::new();
    let row_by_seam = rows
        .iter()
        .map(|row| (row.seam.as_str(), row))
        .collect::<BTreeMap<_, _>>();
    for seam in required_seams {
        if !row_by_seam.contains_key(seam) {
            errors.push(format!("matrix missing seam: {seam}"));
        }
    }

    for row in &rows {
        if !seen.insert(row.seam.as_str()) {
            errors.push(format!("duplicate seam row: {}", row.seam));
        }
        validate_row(row, root, &mut errors);
    }

    for token in REQUIRED_GAPS.iter().chain(REQUIRED_TEXT.iter()) {
        if !text.contains(token) {
            errors.push(format!("matrix missing required text: {token}"));
        }
    }
    errors
}

fn parse_evidence_rows(text: &str) -> (Vec<EvidenceRow>, Vec<String>) {
    let mut rows = Vec::new();
    let mut errors = Vec::new();
    let mut in_table = false;
    for raw_line in text.lines() {
        let line = raw_line.trim();
        if line == EVIDENCE_TABLE_HEADER {
            in_table = true;
            continue;
        }
        if in_table && line.starts_with("## ") {
            break;
        }
        if !in_table || line.starts_with("| ---") || !line.starts_with("| ") {
            continue;
        }
        let cells = line
            .trim_matches('|')
            .split('|')
            .map(str::trim)
            .map(ToOwned::to_owned)
            .collect::<Vec<_>>();
        if cells.len() < EVIDENCE_ROW_CELLS {
            errors.push(format!("evidence row has too few cells: {line}"));
            continue;
        }
        rows.push(EvidenceRow {
            seam: cells[0].clone(),
            command: cells[1].clone(),
            receipt: cells[2].clone(),
            doc: cells[3].clone(),
            blake3: strip_code_span(&cells[4]),
            commits: cells[5].clone(),
            claim: cells[6].clone(),
            non_claims: cells[7].clone(),
        });
    }
    (rows, errors)
}

fn validate_row(row: &EvidenceRow, root: Option<&Path>, errors: &mut Vec<String>) {
    if row.command.is_empty() || !row.command.contains("nix run") {
        errors.push(format!("{} missing maintained command", row.seam));
    }
    if row.claim.is_empty() || row.non_claims.is_empty() {
        errors.push(format!("{} missing claim or non-claims", row.seam));
    }
    if !is_hex_digest(&row.blake3) {
        errors.push(format!(
            "{} has invalid BLAKE3 digest {}",
            row.seam, row.blake3
        ));
    }
    for marker in FORBIDDEN_COMMIT_MARKERS {
        if row.commits.contains(marker) {
            errors.push(format!(
                "{} commits cell contains unstable marker {marker:?}",
                row.seam
            ));
        }
    }
    let reviewable = REVIEWABLE_RECEIPT_SEAMS.contains(&row.seam.as_str());
    let historical_target = HISTORICAL_TARGET_RECEIPT_SEAMS.contains(&row.seam.as_str());
    let receipt_path = strip_code_span(&row.receipt);
    if reviewable && !receipt_path.starts_with(DOCS_EVIDENCE_PREFIX) {
        errors.push(format!(
            "{} receipt must be reviewable under docs/evidence",
            row.seam
        ));
    }
    if !historical_target && !receipt_path.ends_with(JSON_SUFFIX) {
        errors.push(format!("{} receipt must be a JSON receipt", row.seam));
    }
    let doc_path = strip_code_span(&row.doc);
    if !doc_path.starts_with(DOCS_EVIDENCE_PREFIX) || !doc_path.ends_with(MARKDOWN_SUFFIX) {
        errors.push(format!(
            "{} evidence doc must be markdown under docs/evidence",
            row.seam
        ));
    }
    if let Some(root) = root {
        if receipt_path.starts_with(DOCS_EVIDENCE_PREFIX) && !root.join(&receipt_path).exists() {
            errors.push(format!("{} receipt path missing: {receipt_path}", row.seam));
        }
        if receipt_path.starts_with(TARGET_PREFIX) && !historical_target {
            errors.push(format!(
                "{} non-historical receipt points at target/: {receipt_path}",
                row.seam
            ));
        }
        if !root.join(&doc_path).exists() {
            errors.push(format!("{} evidence doc missing: {doc_path}", row.seam));
        }
    }
}

fn strip_code_span(value: &str) -> String {
    value.trim().trim_matches('`').to_string()
}

fn is_hex_digest(value: &str) -> bool {
    value.len() == BLAKE3_HEX_LENGTH && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn run_self_tests() -> Result<String, Vec<String>> {
    let seam = "Fixture seam";
    let digest = "0".repeat(BLAKE3_HEX_LENGTH);
    let text = fixture_text(seam, &digest);
    let errors = validate_matrix_text(&text, None, &[seam]);
    if !errors.is_empty() {
        return Err(errors);
    }

    let missing = validate_matrix_text(&text.replace(seam, "Other seam"), None, &[seam]);
    assert_contains(&missing, "matrix missing seam")?;

    let bad_digest = validate_matrix_text(&text.replace(&digest, "not-a-digest"), None, &[seam]);
    assert_contains(&bad_digest, "invalid BLAKE3")?;

    let overclaim = validate_matrix_text(
        &text.replace("parent `abc1234`", "parent `abc1234` plus current diff"),
        None,
        &[seam],
    );
    assert_contains(&overclaim, "unstable marker")?;

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

fn fixture_text(seam: &str, digest: &str) -> String {
    format!(
        "# Matrix\n\n{EVIDENCE_TABLE_HEADER}\n| --- | --- | --- | --- | --- | --- | --- | --- |\n| {seam} | `nix run .#x` | `docs/evidence/x.receipt.json` | `docs/evidence/x.md` | `{digest}` | parent `abc1234` | Bounded claim. | No broad claim. |\n\n## Remaining gaps and non-claims\n\n| Gap | Status | Why it remains | Next ROI |\n| --- | --- | --- | --- |\n| Residual combat breadth | Non-claim | projectile damage attribution and projectile travel/collision simulation remain bounded. | roi-05-projectile-armor-knockback-combat |\n| Broad protocol coverage | Non-claim | scoped. | roi-07-post-drain-evidence-index |\n| Production load / multiplayer scale | Non-claim | scoped. | roi-04-latency-jitter-tolerance |\n| Full CTF correctness | Non-claim | scoped. | none |\n| Full survival compatibility / vanilla parity | Non-claim | scoped. | none |\n\nExplicit non-claims\nScoped claim\nBLAKE3\nMaintained command\nReceipt\n"
    )
}
