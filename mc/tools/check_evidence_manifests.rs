use std::collections::BTreeSet;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

const SELF_TEST_FLAG: &str = "--self-test";
const EVIDENCE_DIR: &str = "docs/evidence";
const MANIFEST_EXTENSION: &str = "b3";
const RECEIPT_SUFFIX: &str = ".receipt.json";
const MANIFEST_SEPARATOR: &str = "  ";
const BLAKE3_HEX_LENGTH: usize = 64;
const SUCCESS: ExitCode = ExitCode::SUCCESS;
const FAILURE: ExitCode = ExitCode::FAILURE;
const STALE_RECEIPT_MARKER: &str = "equipment_packet_observed";
const STALE_RECEIPT_REPLACEMENT: &str = "use entity_equipment_update";
const TEMP_DIR_PREFIX: &str = "evidence-manifest-self-test-";
const CHECKER_NAME: &str = "evidence manifest";

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct ManifestEntry {
    manifest: PathBuf,
    digest: String,
    relative_path: PathBuf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CheckSummary {
    manifests: usize,
    entries: usize,
    receipts_scanned: usize,
}

fn main() -> ExitCode {
    let args = env::args().collect::<Vec<_>>();
    if args.iter().any(|arg| arg == SELF_TEST_FLAG) {
        return match run_self_tests() {
            Ok(summary) => {
                println!("{CHECKER_NAME} self-test ok: {summary}");
                SUCCESS
            }
            Err(errors) => {
                print_errors(&errors);
                FAILURE
            }
        };
    }

    match check_evidence(Path::new("."), Path::new(EVIDENCE_DIR)) {
        Ok(summary) => {
            println!(
                "evidence manifests ok: {} manifests, {} entries, {} receipts scanned",
                summary.manifests, summary.entries, summary.receipts_scanned
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
        eprintln!("{CHECKER_NAME} check failed: {error}");
    }
}

fn check_evidence(root: &Path, evidence_dir: &Path) -> Result<CheckSummary, Vec<String>> {
    let manifests = discover_by_extension(evidence_dir, MANIFEST_EXTENSION);
    let mut entries = Vec::new();
    let mut errors = Vec::new();
    for manifest in &manifests {
        match fs::read_to_string(manifest) {
            Ok(text) => entries.extend(parse_manifest(manifest, &text, &mut errors)),
            Err(error) => errors.push(format!("{}: {error}", manifest.display())),
        }
    }
    errors.extend(validate_manifest_entries(root, &entries));
    errors.extend(run_b3sum_checks(root, &manifests));
    let receipts = receipt_paths(root, evidence_dir, &entries);
    errors.extend(stale_marker_errors(&receipts));

    let summary = CheckSummary {
        manifests: manifests.len(),
        entries: entries.len(),
        receipts_scanned: receipts.len(),
    };
    if errors.is_empty() {
        Ok(summary)
    } else {
        Err(errors)
    }
}

fn parse_manifest(manifest: &Path, text: &str, errors: &mut Vec<String>) -> Vec<ManifestEntry> {
    let mut entries = Vec::new();
    for (line_index, raw_line) in text.lines().enumerate() {
        let line_number = line_index + 1;
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }
        let Some((digest, path_text)) = line.split_once(MANIFEST_SEPARATOR) else {
            errors.push(format!(
                "{}:{line_number}: expected '<b3>  <path>'",
                manifest.display()
            ));
            continue;
        };
        if !is_blake3_digest(digest) {
            errors.push(format!(
                "{}:{line_number}: invalid BLAKE3 digest {digest:?}",
                manifest.display()
            ));
            continue;
        }
        let relative_path = PathBuf::from(path_text.trim());
        if relative_path.is_absolute()
            || relative_path
                .components()
                .any(|part| part.as_os_str() == OsStr::new(".."))
        {
            errors.push(format!(
                "{}:{line_number}: path must be repo-relative: {path_text:?}",
                manifest.display()
            ));
            continue;
        }
        entries.push(ManifestEntry {
            manifest: manifest.to_path_buf(),
            digest: digest.to_string(),
            relative_path,
        });
    }
    if entries.is_empty() && errors.is_empty() {
        errors.push(format!("{}: manifest has no entries", manifest.display()));
    }
    entries
}

fn is_blake3_digest(value: &str) -> bool {
    value.len() == BLAKE3_HEX_LENGTH && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn discover_by_extension(dir: &Path, extension: &str) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && path.extension() == Some(OsStr::new(extension)) {
                paths.push(path);
            }
        }
    }
    paths.sort();
    paths
}

fn discover_receipts(evidence_dir: &Path) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    if let Ok(entries) = fs::read_dir(evidence_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && path.to_string_lossy().ends_with(RECEIPT_SUFFIX) {
                paths.push(path);
            }
        }
    }
    paths.sort();
    paths
}

fn validate_manifest_entries(root: &Path, entries: &[ManifestEntry]) -> Vec<String> {
    let mut errors = Vec::new();
    for entry in entries {
        let target = root.join(&entry.relative_path);
        if !target.exists() {
            errors.push(format!(
                "{}: referenced file missing: {}",
                entry.manifest.display(),
                entry.relative_path.display()
            ));
        } else if !target.is_file() {
            errors.push(format!(
                "{}: referenced path is not a file: {}",
                entry.manifest.display(),
                entry.relative_path.display()
            ));
        }
    }
    errors
}

fn run_b3sum_checks(root: &Path, manifests: &[PathBuf]) -> Vec<String> {
    let mut errors = Vec::new();
    for manifest in manifests {
        let relative_manifest = match manifest.strip_prefix(root) {
            Ok(path) => path,
            Err(_) => manifest.as_path(),
        };
        let output = Command::new("b3sum")
            .arg("--check")
            .arg(relative_manifest)
            .current_dir(root)
            .output();
        match output {
            Ok(completed) if completed.status.success() => {}
            Ok(completed) => {
                let mut text = String::from_utf8_lossy(&completed.stdout).to_string();
                text.push_str(&String::from_utf8_lossy(&completed.stderr));
                let trimmed = text.trim();
                let output_text = if trimmed.is_empty() {
                    "<no output>"
                } else {
                    trimmed
                };
                errors.push(format!(
                    "{}: b3sum --check failed:\n{output_text}",
                    manifest.display()
                ));
            }
            Err(error) => errors.push(format!("b3sum failed for {}: {error}", manifest.display())),
        }
    }
    errors
}

fn receipt_paths(root: &Path, evidence_dir: &Path, entries: &[ManifestEntry]) -> Vec<PathBuf> {
    let mut paths = BTreeSet::new();
    for path in discover_receipts(evidence_dir) {
        paths.insert(canonical_path(&path));
    }
    for entry in entries {
        if entry
            .relative_path
            .to_string_lossy()
            .ends_with(RECEIPT_SUFFIX)
        {
            paths.insert(canonical_path(&root.join(&entry.relative_path)));
        }
    }
    paths.into_iter().collect()
}

fn canonical_path(path: &Path) -> PathBuf {
    path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
}

fn stale_marker_errors(receipts: &[PathBuf]) -> Vec<String> {
    let mut errors = Vec::new();
    for receipt in receipts {
        if !receipt.exists() {
            continue;
        }
        match fs::read_to_string(receipt) {
            Ok(text) if text.contains(STALE_RECEIPT_MARKER) => errors.push(format!(
                "{}: stale marker {STALE_RECEIPT_MARKER:?}; {STALE_RECEIPT_REPLACEMENT}",
                receipt.display()
            )),
            Ok(_) => {}
            Err(error) => errors.push(format!("{}: {error}", receipt.display())),
        }
    }
    errors
}

fn run_self_tests() -> Result<String, Vec<String>> {
    let root = make_temp_root()?;
    let evidence_dir = root.join(EVIDENCE_DIR);
    fs::create_dir_all(&evidence_dir)
        .map_err(|error| vec![format!("{}: {error}", evidence_dir.display())])?;

    let receipt = evidence_dir.join("good.receipt.json");
    write_file(
        &receipt,
        "{\"required_milestones\":[\"entity_equipment_update\"]}\n",
    )?;
    let manifest = evidence_dir.join("good.b3");
    let receipt_digest = b3sum_digest(&receipt)?;
    write_file(
        &manifest,
        &format!("{receipt_digest}{MANIFEST_SEPARATOR}docs/evidence/good.receipt.json\n"),
    )?;

    let summary = check_evidence(&root, &evidence_dir)?;
    assert_summary(
        summary,
        CheckSummary {
            manifests: 1,
            entries: 1,
            receipts_scanned: 1,
        },
    )?;

    let stale = evidence_dir.join("stale.receipt.json");
    write_file(
        &stale,
        "{\"missing_milestones\":[\"equipment_packet_observed\"]}\n",
    )?;
    assert_contains(&check_evidence(&root, &evidence_dir), STALE_RECEIPT_MARKER)?;
    fs::remove_file(&stale).map_err(|error| vec![format!("{}: {error}", stale.display())])?;

    let run_log = evidence_dir.join("run.log");
    write_file(&run_log, "ok\n")?;
    let run_log_digest = b3sum_digest(&run_log)?;
    let log_manifest = evidence_dir.join("run-log.b3");
    write_file(
        &log_manifest,
        &format!("{run_log_digest}{MANIFEST_SEPARATOR}docs/evidence/run.log\n"),
    )?;
    let summary = check_evidence(&root, &evidence_dir)?;
    assert_summary(
        summary,
        CheckSummary {
            manifests: 2,
            entries: 2,
            receipts_scanned: 1,
        },
    )?;

    fs::remove_file(&run_log).map_err(|error| vec![format!("{}: {error}", run_log.display())])?;
    assert_contains(
        &check_evidence(&root, &evidence_dir),
        "referenced file missing",
    )?;
    fs::remove_file(&log_manifest)
        .map_err(|error| vec![format!("{}: {error}", log_manifest.display())])?;

    let missing_manifest = evidence_dir.join("missing.b3");
    write_file(
        &missing_manifest,
        &format!(
            "{}{MANIFEST_SEPARATOR}docs/evidence/missing.receipt.json\n",
            "0".repeat(BLAKE3_HEX_LENGTH)
        ),
    )?;
    assert_contains(
        &check_evidence(&root, &evidence_dir),
        "referenced file missing",
    )?;

    write_file(
        &receipt,
        "{\"required_milestones\":[\"entity_equipment_update\"],\"changed\":true}\n",
    )?;
    assert_contains(
        &check_evidence(&root, &evidence_dir),
        "b3sum --check failed",
    )?;

    let _ = fs::remove_dir_all(&root);
    Ok("positive and negative fixtures exercised".to_string())
}

fn make_temp_root() -> Result<PathBuf, Vec<String>> {
    let mut root = env::temp_dir();
    root.push(format!("{TEMP_DIR_PREFIX}{}", std::process::id()));
    if root.exists() {
        fs::remove_dir_all(&root).map_err(|error| vec![format!("{}: {error}", root.display())])?;
    }
    fs::create_dir_all(&root).map_err(|error| vec![format!("{}: {error}", root.display())])?;
    Ok(root)
}

fn write_file(path: &Path, text: &str) -> Result<(), Vec<String>> {
    fs::write(path, text).map_err(|error| vec![format!("{}: {error}", path.display())])
}

fn b3sum_digest(path: &Path) -> Result<String, Vec<String>> {
    let output = Command::new("b3sum")
        .arg(path)
        .output()
        .map_err(|error| vec![format!("b3sum {}: {error}", path.display())])?;
    if !output.status.success() {
        return Err(vec![format!(
            "b3sum {} exited with {}",
            path.display(),
            output.status
        )]);
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout
        .split_whitespace()
        .next()
        .map(ToOwned::to_owned)
        .ok_or_else(|| vec![format!("b3sum {} produced no digest", path.display())])
}

fn assert_summary(actual: CheckSummary, expected: CheckSummary) -> Result<(), Vec<String>> {
    if actual == expected {
        Ok(())
    } else {
        Err(vec![format!(
            "summary mismatch: got {actual:?}, expected {expected:?}"
        )])
    }
}

fn assert_contains(
    result: &Result<CheckSummary, Vec<String>>,
    needle: &str,
) -> Result<(), Vec<String>> {
    match result {
        Ok(summary) => Err(vec![format!(
            "expected diagnostic {needle:?}, got success {summary:?}"
        )]),
        Err(errors) if errors.iter().any(|error| error.contains(needle)) => Ok(()),
        Err(errors) => Err(vec![format!(
            "missing expected diagnostic {needle:?}: {errors:?}"
        )]),
    }
}
