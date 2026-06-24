#!/usr/bin/env -S CARGO_TARGET_DIR=target/check-evidence-partitions-script nix shell "github:nix-community/fenix?rev=092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain" nixpkgs#gcc -c cargo -q -Zscript

use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command as ProcessCommand, ExitCode};

const ROOT_FLAG: &str = "--root";
const SELF_TEST_FLAG: &str = "--self-test";
const WRITE_GENERATED_FLAG: &str = "--write-generated";
const DEFAULT_ROOT: &str = ".";
const EVIDENCE_DIR: &str = "docs/evidence";
const INVENTORY_PATH: &str = "docs/evidence/evidence-inventory.generated.md";
const INDEX_PATH: &str = "docs/evidence/evidence-index.generated.md";
const README_PATH: &str = "docs/evidence/README.md";
const ARCHITECTURE_PATH: &str = "docs/architecture.md";
const LAYOUT_CHECKLIST_PATH: &str = "docs/layout-checklist.md";
const CHECK_TIERS_PATH: &str = "docs/check-tiers.md";
const TOOL_PATH: &str = "tools/check_evidence_partitions.rs";
const CHECK_NAME: &str = "evidence partitions";
const GENERATED_BY: &str = "tools/check_evidence_partitions.rs --write-generated";
const SUCCESS: ExitCode = ExitCode::SUCCESS;
const FAILURE: ExitCode = ExitCode::FAILURE;
const PATH_SEPARATOR: char = '/';
const DATE_LENGTH: usize = 10;
const YEAR_PREFIX: &str = "20";
const GENERATED_ROW_SEPARATOR: &str = "|";
const EMPTY_CELL: &str = "-";

const REQUIRED_README_PHRASES: &[&str] = &[
    "# Evidence directory partitions",
    "run-logs/<yyyy-mm-dd>/",
    "manifests/<yyyy-mm-dd>/",
    "receipts/<yyyy-mm-dd>/",
    "oracles/<yyyy-mm-dd>/",
    "indexes/",
    "archive/",
    "Existing flat paths are citation-stable",
    "does not broaden compatibility claims",
];

const REQUIRED_CHECK_TIER_COMMANDS: &[&str] = &[
    "tools/check_evidence_partitions.rs --self-test",
    "tools/check_evidence_partitions.rs --root .",
];

const PREFERRED_PARTITIONS: &[&str] = &[
    "run-logs",
    "manifests",
    "receipts",
    "logs",
    "oracles",
    "indexes",
    "archive",
    "fixtures",
    "notes",
];

const GENERATED_SURFACES: &[&str] = &[INVENTORY_PATH, INDEX_PATH];
const GIT_COMMAND: &str = "git";
const GIT_CWD_FLAG: &str = "-C";
const GIT_LS_FILES_SUBCOMMAND: &str = "ls-files";
const GIT_CACHED_FLAG: &str = "--cached";
const GIT_PATHSPEC_SEPARATOR: &str = "--";

const MULTI_SUFFIXES: &[&str] = &[
    ".receipt.typed-events.log",
    ".typed-events.log",
    ".receipt.json",
    ".generated.md",
    ".run.log",
    ".server.log",
    ".client.log",
    ".log.stderr",
    ".compare.log",
    ".dry-run.log",
    ".matrix.receipt.json",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum ArtifactClass {
    PartitionDoc,
    RunLog,
    Manifest,
    GeneratedIndex,
    OracleNote,
    Receipt,
    ReviewNote,
    AuxLog,
    KeyValue,
    Json,
    SourceSnapshot,
    FixtureBinary,
    Record,
    Other,
}

impl ArtifactClass {
    fn label(self) -> &'static str {
        match self {
            Self::PartitionDoc => "partition-doc",
            Self::RunLog => "run-log",
            Self::Manifest => "manifest",
            Self::GeneratedIndex => "generated-index",
            Self::OracleNote => "oracle-note",
            Self::Receipt => "receipt",
            Self::ReviewNote => "review-note",
            Self::AuxLog => "aux-log",
            Self::KeyValue => "kv",
            Self::Json => "json",
            Self::SourceSnapshot => "source-snapshot",
            Self::FixtureBinary => "fixture-binary",
            Self::Record => "record",
            Self::Other => "other",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Disposition {
    StayFlat,
    MigrateNow,
    MigrateLater,
    GeneratedIndex,
    Manifest,
    OracleNote,
}

impl Disposition {
    fn label(self) -> &'static str {
        match self {
            Self::StayFlat => "stay-flat",
            Self::MigrateNow => "migrate-now",
            Self::MigrateLater => "migrate-later",
            Self::GeneratedIndex => "generated index",
            Self::Manifest => "manifest",
            Self::OracleNote => "oracle note",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct EvidenceArtifact {
    path: String,
    class: ArtifactClass,
    disposition: Disposition,
    date: String,
    key: String,
    reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct EvidenceModel {
    artifacts: Vec<EvidenceArtifact>,
    manifest_coverage: BTreeMap<String, Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct GeneratedSurfaces {
    inventory: String,
    index: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct EvidenceDocs {
    readme: String,
    architecture: String,
    layout_checklist: String,
    check_tiers: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LoadedRepository {
    paths: Vec<String>,
    manifest_texts: BTreeMap<String, String>,
    generated: BTreeMap<String, String>,
    docs: EvidenceDocs,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Diagnostic {
    code: &'static str,
    message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Command {
    root: PathBuf,
    self_test: bool,
    write_generated: bool,
}

fn classify_artifact(path: &str) -> ArtifactClass {
    let lower = path.to_ascii_lowercase();
    if path == README_PATH {
        ArtifactClass::PartitionDoc
    } else if lower.ends_with(".b3") {
        ArtifactClass::Manifest
    } else if lower.ends_with(".generated.md") {
        ArtifactClass::GeneratedIndex
    } else if lower.ends_with(".run.log") {
        ArtifactClass::RunLog
    } else if lower.ends_with(".receipt.json") {
        ArtifactClass::Receipt
    } else if lower.contains("oracle") || lower.contains("checkpoint") {
        ArtifactClass::OracleNote
    } else if lower.ends_with(".md") {
        ArtifactClass::ReviewNote
    } else if lower.ends_with(".typed-events.log")
        || lower.ends_with(".server.log")
        || lower.ends_with(".client.log")
        || lower.ends_with(".log")
        || lower.ends_with(".log.stderr")
    {
        ArtifactClass::AuxLog
    } else if lower.ends_with(".kv") || lower.ends_with(".tsv") || lower.ends_with(".evidence") {
        ArtifactClass::KeyValue
    } else if lower.ends_with(".json") {
        ArtifactClass::Json
    } else if lower.ends_with(".rs") || lower.ends_with(".java") || lower.ends_with(".patch") {
        ArtifactClass::SourceSnapshot
    } else if lower.ends_with(".jar") {
        ArtifactClass::FixtureBinary
    } else if lower.ends_with(".record") {
        ArtifactClass::Record
    } else {
        ArtifactClass::Other
    }
}

fn classify_disposition(path: &str, class: ArtifactClass) -> (Disposition, String) {
    match class {
        ArtifactClass::GeneratedIndex => (
            Disposition::GeneratedIndex,
            "machine-owned generated evidence navigation surface".to_string(),
        ),
        ArtifactClass::Manifest => (
            Disposition::Manifest,
            "BLAKE3 manifest remains valid at its cited path".to_string(),
        ),
        ArtifactClass::OracleNote => (
            Disposition::OracleNote,
            "human oracle/checkpoint note stays reviewable at its cited path".to_string(),
        ),
        ArtifactClass::PartitionDoc => (
            Disposition::StayFlat,
            "partition scheme documentation lives at the evidence root".to_string(),
        ),
        _ if is_preferred_partition_path(path) => (
            Disposition::StayFlat,
            "already stored under an approved evidence partition".to_string(),
        ),
        _ if is_legacy_subdirectory_path(path) => (
            Disposition::StayFlat,
            "legacy change-specific evidence directory is citation-stable".to_string(),
        ),
        ArtifactClass::RunLog | ArtifactClass::Receipt | ArtifactClass::AuxLog => (
            Disposition::MigrateLater,
            "legacy flat runtime evidence remains citation-stable until references migrate"
                .to_string(),
        ),
        _ => (
            Disposition::StayFlat,
            "legacy flat evidence remains citation-stable".to_string(),
        ),
    }
}

fn is_preferred_partition_path(path: &str) -> bool {
    let Some(after_prefix) = path
        .strip_prefix(EVIDENCE_DIR)
        .and_then(|rest| rest.strip_prefix(PATH_SEPARATOR))
    else {
        return false;
    };
    let Some((first, _)) = after_prefix.split_once(PATH_SEPARATOR) else {
        return false;
    };
    PREFERRED_PARTITIONS.contains(&first)
}

fn is_legacy_subdirectory_path(path: &str) -> bool {
    let Some(after_prefix) = path
        .strip_prefix(EVIDENCE_DIR)
        .and_then(|rest| rest.strip_prefix(PATH_SEPARATOR))
    else {
        return false;
    };
    after_prefix.contains(PATH_SEPARATOR) && !is_preferred_partition_path(path)
}

fn date_for_path(path: &str) -> String {
    if let Some(date) = date_from_partition(path) {
        return date;
    }
    if let Some(date) = first_date_substring(path) {
        return date;
    }
    "undated".to_string()
}

fn date_from_partition(path: &str) -> Option<String> {
    let parts = path.split(PATH_SEPARATOR).collect::<Vec<_>>();
    parts
        .windows(DATE_PARTITION_WINDOW)
        .find_map(|window| is_date(window[1]).then(|| window[1].to_string()))
}

const DATE_PARTITION_WINDOW: usize = 2;

fn first_date_substring(text: &str) -> Option<String> {
    let bytes = text.as_bytes();
    if bytes.len() < DATE_LENGTH {
        return None;
    }
    let last_start = bytes.len() - DATE_LENGTH;
    for start in 0..=last_start {
        let candidate = &text[start..start + DATE_LENGTH];
        if is_date(candidate) {
            return Some(candidate.to_string());
        }
    }
    None
}

fn is_date(candidate: &str) -> bool {
    let bytes = candidate.as_bytes();
    candidate.len() == DATE_LENGTH
        && candidate.starts_with(YEAR_PREFIX)
        && bytes[DATE_YEAR_MONTH_SEPARATOR] == b'-'
        && bytes[DATE_MONTH_DAY_SEPARATOR] == b'-'
        && bytes
            .iter()
            .enumerate()
            .all(|(index, byte)| is_date_separator_index(index) || byte.is_ascii_digit())
}

const DATE_YEAR_MONTH_SEPARATOR: usize = 4;
const DATE_MONTH_DAY_SEPARATOR: usize = 7;

fn is_date_separator_index(index: usize) -> bool {
    index == DATE_YEAR_MONTH_SEPARATOR || index == DATE_MONTH_DAY_SEPARATOR
}

fn key_for_path(path: &str) -> String {
    let basename = path.rsplit(PATH_SEPARATOR).next().unwrap_or(path);
    let without_suffix = strip_known_suffixes(basename);
    let without_dates = remove_date_substrings(without_suffix);
    normalize_key(&without_dates)
}

fn strip_known_suffixes(name: &str) -> &str {
    for suffix in MULTI_SUFFIXES {
        if let Some(stripped) = name.strip_suffix(suffix) {
            return stripped;
        }
    }
    match name.rsplit_once('.') {
        Some((stem, _)) => stem,
        None => name,
    }
}

fn remove_date_substrings(value: &str) -> String {
    let mut output = String::new();
    let mut index = 0;
    while index < value.len() {
        if index + DATE_LENGTH <= value.len() {
            let candidate = &value[index..index + DATE_LENGTH];
            if is_date(candidate) {
                index += DATE_LENGTH;
                continue;
            }
        }
        let character = value[index..]
            .chars()
            .next()
            .expect("index stays on UTF-8 boundary");
        output.push(character);
        index += character.len_utf8();
    }
    output
}

fn normalize_key(value: &str) -> String {
    let mut output = String::new();
    let mut previous_separator = false;
    for character in value.chars() {
        if character.is_ascii_alphanumeric() {
            output.push(character.to_ascii_lowercase());
            previous_separator = false;
        } else if !previous_separator {
            output.push('-');
            previous_separator = true;
        }
    }
    let trimmed = output.trim_matches('-').to_string();
    if trimmed.is_empty() {
        "evidence".to_string()
    } else {
        trimmed
    }
}

fn build_model(paths: &[String], manifest_texts: &BTreeMap<String, String>) -> EvidenceModel {
    let mut unique_paths = paths.iter().cloned().collect::<BTreeSet<_>>();
    for generated_path in GENERATED_SURFACES {
        unique_paths.insert((*generated_path).to_string());
    }

    let mut artifacts = unique_paths
        .into_iter()
        .filter(|path| path.starts_with(EVIDENCE_DIR))
        .map(|path| {
            let class = classify_artifact(&path);
            let (disposition, reason) = classify_disposition(&path, class);
            EvidenceArtifact {
                date: date_for_path(&path),
                key: key_for_path(&path),
                path,
                class,
                disposition,
                reason,
            }
        })
        .collect::<Vec<_>>();
    artifacts.sort_by(|left, right| {
        left.date
            .cmp(&right.date)
            .then(left.key.cmp(&right.key))
            .then(left.class.cmp(&right.class))
            .then(left.path.cmp(&right.path))
    });

    EvidenceModel {
        artifacts,
        manifest_coverage: manifest_coverage(manifest_texts),
    }
}

fn manifest_coverage(manifest_texts: &BTreeMap<String, String>) -> BTreeMap<String, Vec<String>> {
    let mut coverage: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for (manifest_path, text) in manifest_texts {
        for referenced_path in manifest_referenced_paths(text) {
            coverage
                .entry(referenced_path)
                .or_default()
                .push(manifest_path.clone());
        }
    }
    for manifests in coverage.values_mut() {
        manifests.sort();
        manifests.dedup();
    }
    coverage
}

fn manifest_referenced_paths(text: &str) -> Vec<String> {
    text.lines()
        .filter_map(|line| {
            line.split_once("  ")
                .map(|(_, path)| path.trim().to_string())
        })
        .filter(|path| !path.is_empty())
        .collect()
}

fn render_surfaces(model: &EvidenceModel) -> GeneratedSurfaces {
    GeneratedSurfaces {
        inventory: render_inventory(model),
        index: render_index(model),
    }
}

fn render_inventory(model: &EvidenceModel) -> String {
    let mut text = String::new();
    text.push_str("<!-- BEGIN: mc-evidence-inventory -->\n");
    text.push_str(&format!(
        "<!-- @generated by {GENERATED_BY}; edit docs/evidence/README.md partition rules instead. -->\n\n"
    ));
    text.push_str("# Evidence inventory\n\n");
    text.push_str("This machine-owned inventory classifies durable artifacts without moving legacy citations.\n\n");
    text.push_str("| Disposition | Class | Date | Key | Path | Reason |\n");
    text.push_str("| --- | --- | --- | --- | --- | --- |\n");
    for artifact in &model.artifacts {
        text.push_str(&format!(
            "| {} | {} | {} | {} | `{}` | {} |\n",
            artifact.disposition.label(),
            artifact.class.label(),
            markdown_cell(&artifact.date),
            markdown_cell(&artifact.key),
            artifact.path,
            markdown_cell(&artifact.reason)
        ));
    }
    text.push_str("\n<!-- END: mc-evidence-inventory -->\n");
    text
}

fn render_index(model: &EvidenceModel) -> String {
    let mut text = String::new();
    text.push_str("<!-- BEGIN: mc-evidence-index -->\n");
    text.push_str(&format!(
        "<!-- @generated by {GENERATED_BY}; edit docs/evidence/README.md partition rules instead. -->\n\n"
    ));
    text.push_str("# Evidence index\n\n");
    text.push_str("This machine-owned index maps dates and inferred change/scenario keys to durable evidence paths. It is navigation evidence only and does not broaden compatibility claims.\n\n");
    text.push_str("| Date | Key | Class | Path | Covering manifests |\n");
    text.push_str("| --- | --- | --- | --- | --- |\n");
    for artifact in &model.artifacts {
        let manifests = model
            .manifest_coverage
            .get(&artifact.path)
            .map(|paths| {
                paths
                    .iter()
                    .map(|path| format!("`{path}`"))
                    .collect::<Vec<_>>()
                    .join("<br>")
            })
            .unwrap_or_else(|| EMPTY_CELL.to_string());
        text.push_str(&format!(
            "| {} | {} | {} | `{}` | {} |\n",
            markdown_cell(&artifact.date),
            markdown_cell(&artifact.key),
            artifact.class.label(),
            artifact.path,
            manifests
        ));
    }
    text.push_str("\n<!-- END: mc-evidence-index -->\n");
    text
}

fn markdown_cell(value: &str) -> String {
    value.replace(GENERATED_ROW_SEPARATOR, "\\|")
}

fn validate_repository(repo: &LoadedRepository) -> Vec<Diagnostic> {
    let model = build_model(&repo.paths, &repo.manifest_texts);
    let generated = render_surfaces(&model);
    let mut diagnostics = Vec::new();
    diagnostics.extend(validate_docs(&repo.docs));
    diagnostics.extend(validate_generated_surface(
        INVENTORY_PATH,
        &generated.inventory,
        repo.generated.get(INVENTORY_PATH),
    ));
    diagnostics.extend(validate_generated_surface(
        INDEX_PATH,
        &generated.index,
        repo.generated.get(INDEX_PATH),
    ));
    diagnostics.extend(validate_no_path_escapes(&repo.paths));
    diagnostics.extend(validate_index_rows(&model, repo.generated.get(INDEX_PATH)));
    diagnostics
}

fn validate_docs(docs: &EvidenceDocs) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    push_missing(
        &mut diagnostics,
        "readme_missing_partition_rule",
        README_PATH,
        &docs.readme,
        REQUIRED_README_PHRASES,
    );
    push_missing(
        &mut diagnostics,
        "architecture_missing_partition_rule",
        ARCHITECTURE_PATH,
        &docs.architecture,
        &["Evidence partitions", "run-logs/<yyyy-mm-dd>/"],
    );
    push_missing(
        &mut diagnostics,
        "layout_missing_partition_rule",
        LAYOUT_CHECKLIST_PATH,
        &docs.layout_checklist,
        &["docs/evidence/README.md", "evidence-index.generated.md"],
    );
    push_missing(
        &mut diagnostics,
        "check_tiers_missing_partition_command",
        CHECK_TIERS_PATH,
        &docs.check_tiers,
        REQUIRED_CHECK_TIER_COMMANDS,
    );
    diagnostics
}

fn push_missing(
    diagnostics: &mut Vec<Diagnostic>,
    code: &'static str,
    path: &str,
    text: &str,
    required: &[&str],
) {
    let missing = required
        .iter()
        .copied()
        .filter(|phrase| !text.contains(phrase))
        .collect::<Vec<_>>();
    if missing.is_empty() {
        return;
    }
    diagnostics.push(Diagnostic {
        code,
        message: format!("{path} is missing: {}", missing.join(", ")),
    });
}

fn validate_generated_surface(
    path: &str,
    expected: &str,
    actual: Option<&String>,
) -> Vec<Diagnostic> {
    match actual {
        Some(text) if text == expected => Vec::new(),
        Some(_) => vec![Diagnostic {
            code: "stale_generated_surface",
            message: format!("{path} is stale; run {TOOL_PATH} {WRITE_GENERATED_FLAG}"),
        }],
        None => vec![Diagnostic {
            code: "missing_generated_surface",
            message: format!("{path} is missing; run {TOOL_PATH} {WRITE_GENERATED_FLAG}"),
        }],
    }
}

fn validate_no_path_escapes(paths: &[String]) -> Vec<Diagnostic> {
    paths
        .iter()
        .filter(|path| path_is_unsafe(path))
        .map(|path| Diagnostic {
            code: "unsafe_evidence_path",
            message: format!("evidence path escapes durable root: {path}"),
        })
        .collect()
}

fn path_is_unsafe(path: &str) -> bool {
    path.starts_with(PATH_SEPARATOR) || path.split(PATH_SEPARATOR).any(|part| part == "..")
}

fn validate_index_rows(model: &EvidenceModel, index_text: Option<&String>) -> Vec<Diagnostic> {
    let Some(index_text) = index_text else {
        return Vec::new();
    };
    let indexed_paths = generated_index_paths(index_text);
    model
        .artifacts
        .iter()
        .filter(|artifact| !indexed_paths.contains(&artifact.path))
        .map(|artifact| Diagnostic {
            code: "missing_index_row",
            message: format!("{} is absent from {INDEX_PATH}", artifact.path),
        })
        .collect()
}

fn generated_index_paths(text: &str) -> BTreeSet<String> {
    text.lines()
        .filter(|line| line.starts_with(GENERATED_ROW_SEPARATOR))
        .filter_map(path_from_generated_row)
        .collect()
}

fn path_from_generated_row(line: &str) -> Option<String> {
    let cells = line
        .split(GENERATED_ROW_SEPARATOR)
        .map(str::trim)
        .collect::<Vec<_>>();
    let path_cell = cells.get(INDEX_PATH_CELL)?;
    path_cell
        .strip_prefix('`')
        .and_then(|value| value.strip_suffix('`'))
        .map(ToOwned::to_owned)
}

const INDEX_PATH_CELL: usize = 4;

fn parse_args() -> Result<Command, String> {
    let mut root = PathBuf::from(DEFAULT_ROOT);
    let mut self_test = false;
    let mut write_generated = false;
    let mut args = env::args().skip(1);

    while let Some(arg) = args.next() {
        if arg == ROOT_FLAG {
            let value = args
                .next()
                .ok_or_else(|| format!("{ROOT_FLAG} requires a path"))?;
            root = PathBuf::from(value);
        } else if arg == SELF_TEST_FLAG {
            self_test = true;
        } else if arg == WRITE_GENERATED_FLAG {
            write_generated = true;
        } else {
            return Err(format!("unknown argument: {arg}"));
        }
    }

    Ok(Command {
        root,
        self_test,
        write_generated,
    })
}

fn run(command: Command) -> Result<String, Vec<Diagnostic>> {
    if command.self_test {
        run_self_tests()?;
        return Ok(format!("{CHECK_NAME} self-test passed"));
    }

    let repo = load_repository(&command.root)?;
    let model = build_model(&repo.paths, &repo.manifest_texts);
    let generated = render_surfaces(&model);
    if command.write_generated {
        write_generated_surfaces(&command.root, &generated)?;
        return Ok(format!(
            "{CHECK_NAME} generated surfaces refreshed: artifacts={}",
            model.artifacts.len()
        ));
    }

    let diagnostics = validate_repository(&repo);
    if diagnostics.is_empty() {
        Ok(format!(
            "{CHECK_NAME} passed: artifacts={}, manifests={}",
            model.artifacts.len(),
            repo.manifest_texts.len()
        ))
    } else {
        Err(diagnostics)
    }
}

fn load_repository(root: &Path) -> Result<LoadedRepository, Vec<Diagnostic>> {
    let (mut paths, read_errors) = evidence_paths(root);
    paths.sort();
    paths.dedup();

    let manifest_texts = read_texts(
        root,
        paths
            .iter()
            .filter(|path| path.ends_with(".b3"))
            .map(String::as_str),
    )?;
    let generated = read_texts(root, GENERATED_SURFACES.iter().copied())?;
    let docs = EvidenceDocs {
        readme: read_optional_text(root, README_PATH)?,
        architecture: read_optional_text(root, ARCHITECTURE_PATH)?,
        layout_checklist: read_optional_text(root, LAYOUT_CHECKLIST_PATH)?,
        check_tiers: read_optional_text(root, CHECK_TIERS_PATH)?,
    };

    if read_errors.is_empty() {
        Ok(LoadedRepository {
            paths,
            manifest_texts,
            generated,
            docs,
        })
    } else {
        Err(read_errors)
    }
}

fn evidence_paths(root: &Path) -> (Vec<String>, Vec<Diagnostic>) {
    match git_cached_evidence_paths(root) {
        Ok(Some(paths)) => (paths, Vec::new()),
        Ok(None) => filesystem_evidence_paths(root),
        Err(diagnostic) => {
            let (paths, mut diagnostics) = filesystem_evidence_paths(root);
            diagnostics.push(diagnostic);
            (paths, diagnostics)
        }
    }
}

fn git_cached_evidence_paths(root: &Path) -> Result<Option<Vec<String>>, Diagnostic> {
    let output = match ProcessCommand::new(GIT_COMMAND)
        .arg(GIT_CWD_FLAG)
        .arg(root)
        .arg(GIT_LS_FILES_SUBCOMMAND)
        .arg(GIT_CACHED_FLAG)
        .arg(GIT_PATHSPEC_SEPARATOR)
        .arg(EVIDENCE_DIR)
        .output()
    {
        Ok(output) => output,
        Err(_) => return Ok(None),
    };
    if !output.status.success() {
        return Ok(None);
    }
    let stdout = String::from_utf8(output.stdout).map_err(|error| Diagnostic {
        code: "git_ls_files_not_utf8",
        message: format!("git ls-files output is not UTF-8: {error}"),
    })?;
    let paths = stdout
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();
    if paths.is_empty() {
        Ok(None)
    } else {
        Ok(Some(paths))
    }
}

fn filesystem_evidence_paths(root: &Path) -> (Vec<String>, Vec<Diagnostic>) {
    let mut paths = Vec::new();
    let mut diagnostics = Vec::new();
    collect_evidence_paths(root, &root.join(EVIDENCE_DIR), &mut paths, &mut diagnostics);
    (paths, diagnostics)
}

fn collect_evidence_paths(
    root: &Path,
    current: &Path,
    paths: &mut Vec<String>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let entries = match fs::read_dir(current) {
        Ok(entries) => entries,
        Err(error) => {
            diagnostics.push(Diagnostic {
                code: "read_dir_failed",
                message: format!("{}: {error}", current.display()),
            });
            return;
        }
    };

    for entry_result in entries {
        let entry = match entry_result {
            Ok(entry) => entry,
            Err(error) => {
                diagnostics.push(Diagnostic {
                    code: "read_dir_entry_failed",
                    message: format!("{}: {error}", current.display()),
                });
                continue;
            }
        };
        let path = entry.path();
        let metadata = match entry.metadata() {
            Ok(metadata) => metadata,
            Err(error) => {
                diagnostics.push(Diagnostic {
                    code: "metadata_failed",
                    message: format!("{}: {error}", path.display()),
                });
                continue;
            }
        };
        if metadata.is_dir() {
            collect_evidence_paths(root, &path, paths, diagnostics);
        } else if metadata.is_file() {
            match repo_relative_path(root, &path) {
                Ok(relative) => paths.push(relative),
                Err(diagnostic) => diagnostics.push(diagnostic),
            }
        }
    }
}

fn repo_relative_path(root: &Path, path: &Path) -> Result<String, Diagnostic> {
    path.strip_prefix(root)
        .map_err(|error| Diagnostic {
            code: "path_strip_failed",
            message: format!("{}: {error}", path.display()),
        })?
        .to_str()
        .map(|path| path.replace('\\', "/"))
        .ok_or_else(|| Diagnostic {
            code: "path_not_utf8",
            message: format!("{} is not UTF-8", path.display()),
        })
}

fn read_texts<'a, I>(root: &Path, paths: I) -> Result<BTreeMap<String, String>, Vec<Diagnostic>>
where
    I: IntoIterator<Item = &'a str>,
{
    let mut texts = BTreeMap::new();
    let mut diagnostics = Vec::new();
    for path in paths {
        let absolute = root.join(path);
        if !absolute.exists() {
            continue;
        }
        match fs::read_to_string(&absolute) {
            Ok(text) => {
                texts.insert(path.to_string(), text);
            }
            Err(error) => diagnostics.push(Diagnostic {
                code: "read_text_failed",
                message: format!("{}: {error}", absolute.display()),
            }),
        }
    }
    if diagnostics.is_empty() {
        Ok(texts)
    } else {
        Err(diagnostics)
    }
}

fn read_optional_text(root: &Path, path: &str) -> Result<String, Vec<Diagnostic>> {
    let absolute = root.join(path);
    match fs::read_to_string(&absolute) {
        Ok(text) => Ok(text),
        Err(error) => Err(vec![Diagnostic {
            code: "read_required_doc_failed",
            message: format!("{}: {error}", absolute.display()),
        }]),
    }
}

fn write_generated_surfaces(
    root: &Path,
    generated: &GeneratedSurfaces,
) -> Result<(), Vec<Diagnostic>> {
    write_generated_file(root, INVENTORY_PATH, &generated.inventory)?;
    write_generated_file(root, INDEX_PATH, &generated.index)?;
    Ok(())
}

fn write_generated_file(root: &Path, path: &str, text: &str) -> Result<(), Vec<Diagnostic>> {
    let absolute = root.join(path);
    let Some(parent) = absolute.parent() else {
        return Err(vec![Diagnostic {
            code: "generated_parent_missing",
            message: format!("{} has no parent directory", absolute.display()),
        }]);
    };
    fs::create_dir_all(parent).map_err(|error| {
        vec![Diagnostic {
            code: "generated_parent_create_failed",
            message: format!("{}: {error}", parent.display()),
        }]
    })?;
    fs::write(&absolute, text).map_err(|error| {
        vec![Diagnostic {
            code: "generated_write_failed",
            message: format!("{}: {error}", absolute.display()),
        }]
    })
}

fn run_self_tests() -> Result<(), Vec<Diagnostic>> {
    let mut diagnostics = Vec::new();
    diagnostics.extend(expect_no_diagnostics(
        "positive generated fixtures",
        positive_generated_fixture(),
    ));
    diagnostics.extend(expect_diagnostic(
        "stale inventory fixture",
        stale_inventory_fixture(),
        "stale_generated_surface",
    ));
    diagnostics.extend(expect_diagnostic(
        "missing index row fixture",
        missing_index_row_fixture(),
        "stale_generated_surface",
    ));
    diagnostics.extend(expect_diagnostic(
        "path escape fixture",
        path_escape_fixture(),
        "unsafe_evidence_path",
    ));
    diagnostics.extend(expect_diagnostic(
        "missing docs fixture",
        missing_docs_fixture(),
        "readme_missing_partition_rule",
    ));
    diagnostics.extend(expect_classification_fixture());

    if diagnostics.is_empty() {
        Ok(())
    } else {
        Err(diagnostics)
    }
}

fn expect_no_diagnostics(name: &str, repo: LoadedRepository) -> Vec<Diagnostic> {
    let diagnostics = validate_repository(&repo);
    if diagnostics.is_empty() {
        Vec::new()
    } else {
        vec![Diagnostic {
            code: "self_test_unexpected_failure",
            message: format!("{name}: expected success, got {diagnostics:?}"),
        }]
    }
}

fn expect_diagnostic(name: &str, repo: LoadedRepository, code: &'static str) -> Vec<Diagnostic> {
    let diagnostics = validate_repository(&repo);
    if diagnostics.iter().any(|diagnostic| diagnostic.code == code) {
        Vec::new()
    } else {
        vec![Diagnostic {
            code: "self_test_missing_diagnostic",
            message: format!("{name}: expected {code}, got {diagnostics:?}"),
        }]
    }
}

fn positive_generated_fixture() -> LoadedRepository {
    let paths = vec![
        README_PATH.to_string(),
        "docs/evidence/run-logs/2026-06-24/example.run.log".to_string(),
        "docs/evidence/manifests/2026-06-24/example.b3".to_string(),
        INVENTORY_PATH.to_string(),
        INDEX_PATH.to_string(),
    ];
    let mut manifest_texts = BTreeMap::new();
    manifest_texts.insert(
        "docs/evidence/manifests/2026-06-24/example.b3".to_string(),
        "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef  docs/evidence/run-logs/2026-06-24/example.run.log\n".to_string(),
    );
    let model = build_model(&paths, &manifest_texts);
    let generated = render_surfaces(&model);
    let mut generated_text = BTreeMap::new();
    generated_text.insert(INVENTORY_PATH.to_string(), generated.inventory);
    generated_text.insert(INDEX_PATH.to_string(), generated.index);
    LoadedRepository {
        paths,
        manifest_texts,
        generated: generated_text,
        docs: valid_docs(),
    }
}

fn stale_inventory_fixture() -> LoadedRepository {
    let mut repo = positive_generated_fixture();
    repo.generated
        .insert(INVENTORY_PATH.to_string(), "stale inventory\n".to_string());
    repo
}

fn missing_index_row_fixture() -> LoadedRepository {
    let mut repo = positive_generated_fixture();
    let index = repo
        .generated
        .get_mut(INDEX_PATH)
        .expect("fixture has index");
    *index = index.replace(
        "`docs/evidence/run-logs/2026-06-24/example.run.log`",
        "`docs/evidence/run-logs/2026-06-24/missing.run.log`",
    );
    repo
}

fn path_escape_fixture() -> LoadedRepository {
    let mut repo = positive_generated_fixture();
    repo.paths
        .push("docs/evidence/../secret.run.log".to_string());
    let model = build_model(&repo.paths, &repo.manifest_texts);
    let generated = render_surfaces(&model);
    repo.generated
        .insert(INVENTORY_PATH.to_string(), generated.inventory);
    repo.generated
        .insert(INDEX_PATH.to_string(), generated.index);
    repo
}

fn missing_docs_fixture() -> LoadedRepository {
    let mut repo = positive_generated_fixture();
    repo.docs.readme = "# Missing\n".to_string();
    repo
}

fn valid_docs() -> EvidenceDocs {
    EvidenceDocs {
        readme: REQUIRED_README_PHRASES.join("\n"),
        architecture: "Evidence partitions\nrun-logs/<yyyy-mm-dd>/\n".to_string(),
        layout_checklist: "docs/evidence/README.md\nevidence-index.generated.md\n".to_string(),
        check_tiers: REQUIRED_CHECK_TIER_COMMANDS.join("\n"),
    }
}

fn expect_classification_fixture() -> Vec<Diagnostic> {
    let manifest = classify_artifact("docs/evidence/example.b3");
    let (manifest_disposition, _) = classify_disposition("docs/evidence/example.b3", manifest);
    let partitioned_log = classify_artifact("docs/evidence/run-logs/2026-06-24/example.run.log");
    let (partitioned_disposition, _) = classify_disposition(
        "docs/evidence/run-logs/2026-06-24/example.run.log",
        partitioned_log,
    );
    let flat_log = classify_artifact("docs/evidence/example.run.log");
    let (flat_disposition, _) = classify_disposition("docs/evidence/example.run.log", flat_log);

    let mut diagnostics = Vec::new();
    if manifest != ArtifactClass::Manifest || manifest_disposition != Disposition::Manifest {
        diagnostics.push(Diagnostic {
            code: "self_test_classification_failed",
            message: "manifest classification failed".to_string(),
        });
    }
    if partitioned_log != ArtifactClass::RunLog || partitioned_disposition != Disposition::StayFlat
    {
        diagnostics.push(Diagnostic {
            code: "self_test_classification_failed",
            message: "partitioned run log classification failed".to_string(),
        });
    }
    if flat_log != ArtifactClass::RunLog || flat_disposition != Disposition::MigrateLater {
        diagnostics.push(Diagnostic {
            code: "self_test_classification_failed",
            message: "flat run log migration classification failed".to_string(),
        });
    }
    if Disposition::MigrateNow.label() != "migrate-now" {
        diagnostics.push(Diagnostic {
            code: "self_test_classification_failed",
            message: "migrate-now disposition label failed".to_string(),
        });
    }
    diagnostics
}

fn print_diagnostics(diagnostics: &[Diagnostic]) {
    for diagnostic in diagnostics {
        eprintln!(
            "{CHECK_NAME} check failed: {}: {}",
            diagnostic.code, diagnostic.message
        );
    }
}

fn main() -> ExitCode {
    let command = match parse_args() {
        Ok(command) => command,
        Err(error) => {
            eprintln!("{CHECK_NAME} check failed: {error}");
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
