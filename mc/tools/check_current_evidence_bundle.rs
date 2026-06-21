use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

const SELF_TEST_FLAG: &str = "--self-test";
const MATRIX_PATH: &str = "docs/evidence/protocol-763-acceptance-matrix.md";
const BUNDLE_PATH: &str = "docs/evidence/protocol-763-current-evidence-bundle.md";
const BLAKE3_HEX_LENGTH: usize = 64;
const MATRIX_DIGEST_CELL_INDEX: usize = 4;
const BUNDLE_DIGEST_CELL_INDEX: usize = 2;
const SUCCESS: ExitCode = ExitCode::SUCCESS;
const FAILURE: ExitCode = ExitCode::FAILURE;

const REQUIRED_SEAMS: &[&str] = &[
    "RED/BLUE scoring soak",
    "Inventory/drop",
    "Block placement / use-item-on-block",
    "Pickup semantics",
    "Player-inventory click/container click",
    "Open-container semantics",
    "Inventory transaction packet family",
    "Inventory stack split/merge",
    "Inventory drag transactions",
    "Two-client combat/damage",
    "Entity metadata packet family",
    "Flag-carrier death/return",
    "Reconnect flag-state",
    "Invalid flag pickup/ownership",
    "Invalid flag return/drop",
    "CTF invalid-action breadth fixture",
    "Score limit / win condition",
    "Simultaneous pickup/capture race",
    "Spawn/team balance/resource reset",
    "Latency/jitter tolerance",
    "Combat knockback",
    "Bounded Paper-reference combat parity",
    "Bounded Paper-reference armor combat parity",
    "Armor equipment mitigation",
    "Equipment update observation",
    "Equipment permutation packet family",
    "Projectile use/loadout rail",
    "Projectile damage attribution",
    "Command/recipe/advancement packet family",
    "Chunk/biome packet family",
    "Scoreboard/team packet family",
    "Movement packet family",
    "Block-entity sign packet family",
    "Block-entity update breadth",
    "Chat/command containment",
    "Chunk biome data packet",
    "Creative inventory action",
    "Entity status-effect packets",
    "Recipe-book client settings",
    "Resource-pack status",
    "Sign editor open/update",
    "Survival break/place/pickup",
    "Survival chest persistence",
    "Survival crafting table",
    "Survival crafting recipe breadth",
    "Survival furnace smelting breadth",
    "Survival furnace persistence",
    "Survival hunger/food",
    "Survival mob drops",
    "Survival redstone toggle",
    "Survival biome/dimension join state",
    "Survival world persistence restart",
    "Survival crash recovery",
    "Survival sign block-entity persistence",
    "MCP-controlled observability",
];

const REQUIRED_TEXT: &[&str] = &[
    "tools/check_acceptance_matrix.rs",
    "tools/check_current_evidence_bundle.rs",
    "nix run --no-update-lock-file .#cairn -- validate --root .",
    "full Minecraft compatibility",
    "full survival compatibility",
    "vanilla parity",
    "armor loadouts",
    "projectile damage attribution",
    "projectile travel/collision simulation",
    "Invalid flag pickup/ownership",
    "Invalid flag return/drop",
    "CTF invalid-action breadth fixture",
    "Score limit / win condition",
    "Simultaneous pickup/capture race",
    "Spawn/team balance/resource reset",
    "Movement packet family",
    "movement physics",
    "all movement packet variants",
    "Block-entity sign packet family",
    "BlockEntityUpdateS2CPacket",
    "SignEditorOpenS2CPacket",
    "UpdateSignC2SPacket",
    "tools/check_targeted_packet_promotions.rs",
    "CreativeInventoryActionC2SPacket",
    "RecipeBookDataC2SPacket",
    "ResourcePackStatusC2SPacket",
    "MCP-controlled observability",
    "Survival crafting recipe breadth",
    "Survival furnace smelting breadth",
];

#[derive(Debug, Clone, PartialEq, Eq)]
struct BundleRow {
    seam: String,
    digest: String,
}

fn main() -> ExitCode {
    let args = env::args().collect::<Vec<_>>();
    if args.iter().any(|arg| arg == SELF_TEST_FLAG) {
        return match run_self_tests() {
            Ok(summary) => {
                println!("current evidence bundle self-test ok: {summary}");
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
            println!("current evidence bundle check passed: {summary}");
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
        eprintln!("current evidence bundle check failed: {error}");
    }
}

fn run_repo_check(root: &Path) -> Result<String, Vec<String>> {
    let matrix_path = root.join(MATRIX_PATH);
    let bundle_path = root.join(BUNDLE_PATH);
    let matrix_text = fs::read_to_string(&matrix_path)
        .map_err(|error| vec![format!("{}: {error}", matrix_path.display())])?;
    let bundle_text = fs::read_to_string(&bundle_path)
        .map_err(|error| vec![format!("{}: {error}", bundle_path.display())])?;
    let errors = validate_bundle_text(&matrix_text, &bundle_text, REQUIRED_SEAMS);
    if errors.is_empty() {
        Ok(format!("{} rows validated", REQUIRED_SEAMS.len()))
    } else {
        Err(errors)
    }
}

fn validate_bundle_text(
    matrix_text: &str,
    bundle_text: &str,
    required_seams: &[&str],
) -> Vec<String> {
    let matrix_rows = matrix_evidence_rows(matrix_text);
    let bundle_rows = bundle_evidence_rows(bundle_text);
    let mut errors = Vec::new();

    let expected_rows = required_seams.len();
    if matrix_rows.len() != expected_rows {
        errors.push(format!(
            "expected {expected_rows} matrix evidence rows, found {}",
            matrix_rows.len()
        ));
    }
    if bundle_rows.len() != matrix_rows.len() {
        errors.push(format!(
            "bundle row count {} does not match matrix {}",
            bundle_rows.len(),
            matrix_rows.len()
        ));
    }

    let matrix_by_seam = rows_by_seam(&matrix_rows);
    let bundle_by_seam = rows_by_seam(&bundle_rows);
    for seam in required_seams {
        if !matrix_by_seam.contains_key(seam) {
            errors.push(format!("matrix missing seam: {seam}"));
        }
        if !bundle_by_seam.contains_key(seam) {
            errors.push(format!("bundle missing seam: {seam}"));
        }
    }
    for row in &matrix_rows {
        match bundle_by_seam.get(row.seam.as_str()) {
            Some(bundle_row) if bundle_row.digest == row.digest => {}
            Some(bundle_row) => errors.push(format!(
                "bundle hash mismatch for {}: {} != {}",
                row.seam, bundle_row.digest, row.digest
            )),
            None => errors.push(format!("bundle missing seam: {}", row.seam)),
        }
    }
    for token in REQUIRED_TEXT {
        if !bundle_text.contains(token) {
            errors.push(format!("bundle missing required text: {token}"));
        }
    }
    errors
}

fn rows_by_seam<'a>(rows: &'a [BundleRow]) -> BTreeMap<&'a str, &'a BundleRow> {
    rows.iter().map(|row| (row.seam.as_str(), row)).collect()
}

fn matrix_evidence_rows(text: &str) -> Vec<BundleRow> {
    table_rows_until_next_heading(text)
        .into_iter()
        .filter_map(|cells| {
            let seam = cells.first()?.clone();
            let digest = extract_digest(cells.get(MATRIX_DIGEST_CELL_INDEX)?)?;
            Some(BundleRow { seam, digest })
        })
        .collect()
}

fn bundle_evidence_rows(text: &str) -> Vec<BundleRow> {
    table_rows_until_next_heading(text)
        .into_iter()
        .filter_map(|cells| {
            let seam = cells.first()?.clone();
            let digest = extract_digest(cells.get(BUNDLE_DIGEST_CELL_INDEX)?)?;
            Some(BundleRow { seam, digest })
        })
        .collect()
}

fn table_rows_until_next_heading(text: &str) -> Vec<Vec<String>> {
    let mut rows = Vec::new();
    let mut in_table = false;
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("| Seam |") {
            in_table = true;
            continue;
        }
        if in_table && trimmed.starts_with("## ") {
            break;
        }
        if !in_table {
            continue;
        }
        let Some(cells) = table_row(trimmed) else {
            continue;
        };
        rows.push(cells);
    }
    rows
}

fn table_row(line: &str) -> Option<Vec<String>> {
    let trimmed = line.trim();
    if !trimmed.starts_with("| ") || trimmed.starts_with("| ---") {
        return None;
    }
    Some(
        trimmed
            .trim_matches('|')
            .split('|')
            .map(str::trim)
            .map(ToOwned::to_owned)
            .collect(),
    )
}

fn extract_digest(text: &str) -> Option<String> {
    text.as_bytes()
        .windows(BLAKE3_HEX_LENGTH)
        .find(|candidate| candidate.iter().all(|byte| byte.is_ascii_hexdigit()))
        .and_then(|candidate| String::from_utf8(candidate.to_vec()).ok())
}

fn run_self_tests() -> Result<String, Vec<String>> {
    let seam = "Fixture seam";
    let digest = "0".repeat(BLAKE3_HEX_LENGTH);
    let (matrix, bundle) = fixture_text(seam, &digest);
    let errors = validate_bundle_text(&matrix, &bundle, &[seam]);
    if !errors.is_empty() {
        return Err(errors);
    }

    let missing_matrix =
        validate_bundle_text(&matrix.replace(seam, "Other seam"), &bundle, &[seam]);
    assert_contains(&missing_matrix, "matrix missing seam")?;

    let missing_bundle =
        validate_bundle_text(&matrix, &bundle.replace(seam, "Other seam"), &[seam]);
    assert_contains(&missing_bundle, "bundle missing seam")?;

    let mismatch = validate_bundle_text(
        &matrix,
        &bundle.replace(&digest, &"1".repeat(BLAKE3_HEX_LENGTH)),
        &[seam],
    );
    assert_contains(&mismatch, "bundle hash mismatch")?;

    let missing_text = validate_bundle_text(
        &matrix,
        &bundle.replace("tools/check_acceptance_matrix.rs", ""),
        &[seam],
    );
    assert_contains(&missing_text, "bundle missing required text")?;

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

fn fixture_text(seam: &str, digest: &str) -> (String, String) {
    let matrix = format!(
        "# Matrix\n\n| Seam | Maintained command | Receipt | Evidence doc | BLAKE3 | Landed commits | Scoped claim | Explicit non-claims |\n| --- | --- | --- | --- | --- | --- | --- | --- |\n| {seam} | `nix run .#x` | `docs/evidence/x.receipt.json` | `docs/evidence/x.md` | `{digest}` | parent `abc1234` | Bounded claim. | No broad claim. |\n"
    );
    let bundle = format!(
        "# Bundle\n\n| Seam | Maintained command | BLAKE3 |\n| --- | --- | --- |\n| {seam} | `nix run .#x` | `{digest}` |\n\ntools/check_acceptance_matrix.rs\ntools/check_current_evidence_bundle.rs\nnix run --no-update-lock-file .#cairn -- validate --root .\nfull Minecraft compatibility\nfull survival compatibility\nvanilla parity\narmor loadouts\nprojectile damage attribution\nprojectile travel/collision simulation\nInvalid flag pickup/ownership\nInvalid flag return/drop\nCTF invalid-action breadth fixture\nScore limit / win condition\nSimultaneous pickup/capture race\nSpawn/team balance/resource reset\nMovement packet family\nmovement physics\nall movement packet variants\nBlock-entity sign packet family\nBlockEntityUpdateS2CPacket\nSignEditorOpenS2CPacket\nUpdateSignC2SPacket\ntools/check_targeted_packet_promotions.rs\nCreativeInventoryActionC2SPacket\nRecipeBookDataC2SPacket\nResourcePackStatusC2SPacket\nMCP-controlled observability\n"
    );
    (matrix, bundle)
}
