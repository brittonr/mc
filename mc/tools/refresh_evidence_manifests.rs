use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

const CHECK_FLAG: &str = "--check";
const REFRESH_FLAG: &str = "--refresh";
const ROOT_FLAG: &str = "--root";
const EVIDENCE_DIR_FLAG: &str = "--evidence-dir";
const MAX_PASSES_FLAG: &str = "--max-passes";
const SELF_TEST_FLAG: &str = "--self-test";
const HELP_FLAG: &str = "--help";
const DEFAULT_ROOT: &str = ".";
const DEFAULT_EVIDENCE_DIR: &str = "docs/evidence";
const MANIFEST_EXTENSION: &str = "b3";
const MANIFEST_SEPARATOR: &str = "  ";
const BLAKE3_HEX_LENGTH: usize = 64;
const LINE_NUMBER_OFFSET: usize = 1;
const DEFAULT_MAX_REFRESH_PASSES: usize = 20;
const NON_CONVERGENCE_FIXTURE_MAX_PASSES: usize = 2;
const TEMP_DIR_PREFIX: &str = "evidence-manifest-refresh-self-test";
const TOOL_NAME: &str = "evidence manifest refresh";
const GIT_COMMAND: &str = "git";
const GIT_CWD_FLAG: &str = "-C";
const GIT_LS_FILES_SUBCOMMAND: &str = "ls-files";
const GIT_CACHED_FLAG: &str = "--cached";
const GIT_PATHSPEC_SEPARATOR: &str = "--";
const SUCCESS: ExitCode = ExitCode::SUCCESS;
const FAILURE: ExitCode = ExitCode::FAILURE;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    Check,
    Refresh,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Config {
    root: PathBuf,
    evidence_dir: PathBuf,
    mode: Mode,
    max_passes: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ManifestInput {
    absolute_path: PathBuf,
    display_path: String,
    text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ParsedManifest {
    absolute_path: PathBuf,
    display_path: String,
    rows: Vec<ParsedRow>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ParsedRow {
    Blank {
        original: String,
    },
    Entry {
        line_number: usize,
        original_digest: String,
        relative_path: String,
        original: String,
    },
    Malformed {
        line_number: usize,
        message: String,
        original: String,
    },
    OutsideRoot {
        line_number: usize,
        relative_path: String,
        original: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum FileState {
    CurrentFile { digest: String },
    Missing,
    NotFile,
    OutsideRoot,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct RefreshSummary {
    manifests: usize,
    entries: usize,
    current_rows: usize,
    stale_rows: usize,
    missing_rows: usize,
    malformed_rows: usize,
    outside_root_rows: usize,
    not_file_rows: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ManifestOutput {
    absolute_path: PathBuf,
    display_path: String,
    text: String,
    changed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RefreshPlan {
    summary: RefreshSummary,
    outputs: Vec<ManifestOutput>,
    diagnostics: Vec<String>,
}

impl RefreshPlan {
    fn has_blocking_errors(&self) -> bool {
        self.summary.missing_rows > 0
            || self.summary.malformed_rows > 0
            || self.summary.outside_root_rows > 0
            || self.summary.not_file_rows > 0
    }

    fn has_stale_rows(&self) -> bool {
        self.summary.stale_rows > 0
    }
}

fn main() -> ExitCode {
    let args = env::args().skip(1).collect::<Vec<_>>();
    if args.iter().any(|arg| arg == HELP_FLAG) {
        print_usage();
        return SUCCESS;
    }
    if args.iter().any(|arg| arg == SELF_TEST_FLAG) {
        return match run_self_tests() {
            Ok(summary) => {
                println!("{TOOL_NAME} self-test ok: {summary}");
                SUCCESS
            }
            Err(errors) => {
                print_errors(&errors);
                FAILURE
            }
        };
    }

    let config = match parse_config(&args) {
        Ok(config) => config,
        Err(errors) => {
            print_errors(&errors);
            return FAILURE;
        }
    };

    match run_config(&config) {
        Ok(summary) => {
            println!("{TOOL_NAME} ok: {summary}");
            SUCCESS
        }
        Err(errors) => {
            print_errors(&errors);
            FAILURE
        }
    }
}

fn print_usage() {
    println!(
        "Usage: refresh-evidence-manifests [--check|--refresh] [--root PATH] [--evidence-dir PATH] [--max-passes N]\n\
         \n\
         Defaults to check-only mode over docs/evidence/*.b3. Refresh mode rewrites stale digest fields only after every row is parseable, in-repository, present, and a regular file."
    );
}

fn print_errors(errors: &[String]) {
    for error in errors {
        eprintln!("{TOOL_NAME} failed: {error}");
    }
}

fn parse_config(args: &[String]) -> Result<Config, Vec<String>> {
    let mut root = PathBuf::from(DEFAULT_ROOT);
    let mut evidence_dir = PathBuf::from(DEFAULT_EVIDENCE_DIR);
    let mut mode = Mode::Check;
    let mut mode_seen = false;
    let mut max_passes = DEFAULT_MAX_REFRESH_PASSES;
    let mut errors = Vec::new();
    let mut index = 0;

    while index < args.len() {
        let arg = &args[index];
        match arg.as_str() {
            CHECK_FLAG => {
                if mode_seen && mode != Mode::Check {
                    errors.push(format!("choose only one of {CHECK_FLAG} or {REFRESH_FLAG}"));
                }
                mode = Mode::Check;
                mode_seen = true;
                index += LINE_NUMBER_OFFSET;
            }
            REFRESH_FLAG => {
                if mode_seen && mode != Mode::Refresh {
                    errors.push(format!("choose only one of {CHECK_FLAG} or {REFRESH_FLAG}"));
                }
                mode = Mode::Refresh;
                mode_seen = true;
                index += LINE_NUMBER_OFFSET;
            }
            ROOT_FLAG => match value_after(args, &mut index, ROOT_FLAG) {
                Some(value) => root = PathBuf::from(value),
                None => errors.push(format!("{ROOT_FLAG} requires a path")),
            },
            EVIDENCE_DIR_FLAG => match value_after(args, &mut index, EVIDENCE_DIR_FLAG) {
                Some(value) => evidence_dir = PathBuf::from(value),
                None => errors.push(format!("{EVIDENCE_DIR_FLAG} requires a path")),
            },
            MAX_PASSES_FLAG => match value_after(args, &mut index, MAX_PASSES_FLAG) {
                Some(value) => match value.parse::<usize>() {
                    Ok(parsed) if parsed > 0 => max_passes = parsed,
                    Ok(_) | Err(_) => {
                        errors.push(format!("{MAX_PASSES_FLAG} requires a positive integer"))
                    }
                },
                None => errors.push(format!("{MAX_PASSES_FLAG} requires a positive integer")),
            },
            SELF_TEST_FLAG | HELP_FLAG => {
                index += LINE_NUMBER_OFFSET;
            }
            _ => {
                errors.push(format!("unknown argument {arg:?}"));
                index += LINE_NUMBER_OFFSET;
            }
        }
    }

    if errors.is_empty() {
        Ok(Config {
            root,
            evidence_dir,
            mode,
            max_passes,
        })
    } else {
        Err(errors)
    }
}

fn value_after(args: &[String], index: &mut usize, flag: &str) -> Option<String> {
    *index += LINE_NUMBER_OFFSET;
    let value = args.get(*index).cloned();
    if value.is_some() {
        *index += LINE_NUMBER_OFFSET;
    } else {
        eprintln!("{TOOL_NAME} failed: missing value after {flag}");
    }
    value
}

fn run_config(config: &Config) -> Result<String, Vec<String>> {
    let root = canonicalize_existing_root(&config.root)?;
    let evidence_dir = resolve_evidence_dir(&root, &config.evidence_dir)?;
    match config.mode {
        Mode::Check => run_check(&root, &evidence_dir),
        Mode::Refresh => run_refresh(&root, &evidence_dir, config.max_passes),
    }
}

fn canonicalize_existing_root(root: &Path) -> Result<PathBuf, Vec<String>> {
    root.canonicalize()
        .map_err(|error| vec![format!("root {}: {error}", root.display())])
}

fn resolve_evidence_dir(root: &Path, evidence_dir: &Path) -> Result<PathBuf, Vec<String>> {
    let candidate = if evidence_dir.is_absolute() {
        evidence_dir.to_path_buf()
    } else {
        root.join(evidence_dir)
    };
    let canonical = candidate
        .canonicalize()
        .map_err(|error| vec![format!("evidence dir {}: {error}", candidate.display())])?;
    if canonical.starts_with(root) {
        Ok(canonical)
    } else {
        Err(vec![format!(
            "evidence dir {} is outside repository {}",
            canonical.display(),
            root.display()
        )])
    }
}

fn run_check(root: &Path, evidence_dir: &Path) -> Result<String, Vec<String>> {
    let plan = scan_once(root, evidence_dir)?;
    if plan.has_blocking_errors() || plan.has_stale_rows() {
        Err(plan.diagnostics)
    } else {
        Ok(summary_text(&plan.summary, "check", 0))
    }
}

fn run_refresh(root: &Path, evidence_dir: &Path, max_passes: usize) -> Result<String, Vec<String>> {
    let mut passes_run = 0;
    let mut writes = 0;
    loop {
        passes_run += LINE_NUMBER_OFFSET;
        let plan = scan_once(root, evidence_dir)?;
        if plan.has_blocking_errors() {
            return Err(plan.diagnostics);
        }
        if !plan.has_stale_rows() {
            return Ok(format!(
                "{}; passes={passes_run}; manifest_writes={writes}",
                summary_text(&plan.summary, "refresh", passes_run)
            ));
        }
        if passes_run >= max_passes {
            let mut diagnostics = plan.diagnostics;
            diagnostics.push(format!(
                "refresh did not converge after {max_passes} passes; refusing to continue"
            ));
            return Err(diagnostics);
        }
        writes += write_changed_outputs(&plan.outputs)?;
    }
}

fn scan_once(root: &Path, evidence_dir: &Path) -> Result<RefreshPlan, Vec<String>> {
    let inputs = read_manifest_inputs(root, evidence_dir)?;
    let parsed = parse_all_manifests(inputs);
    let relative_paths = referenced_relative_paths(&parsed);
    let file_states = load_file_states(root, &relative_paths)?;
    Ok(plan_from_parsed(&parsed, &file_states))
}

fn read_manifest_inputs(
    root: &Path,
    evidence_dir: &Path,
) -> Result<Vec<ManifestInput>, Vec<String>> {
    let manifests = discover_manifests(root, evidence_dir);
    if manifests.is_empty() {
        return Err(vec![format!(
            "no *.{MANIFEST_EXTENSION} manifests found under {}",
            evidence_dir.display()
        )]);
    }

    let mut inputs = Vec::new();
    let mut errors = Vec::new();
    for absolute_path in manifests {
        match fs::read_to_string(&absolute_path) {
            Ok(text) => inputs.push(ManifestInput {
                display_path: display_path(root, &absolute_path),
                absolute_path,
                text,
            }),
            Err(error) => errors.push(format!("{}: {error}", absolute_path.display())),
        }
    }
    if errors.is_empty() {
        Ok(inputs)
    } else {
        Err(errors)
    }
}

fn discover_manifests(root: &Path, evidence_dir: &Path) -> Vec<PathBuf> {
    if let Some(paths) = git_cached_manifest_paths(root, evidence_dir) {
        return paths;
    }
    let mut manifests = Vec::new();
    collect_manifest_paths(evidence_dir, &mut manifests);
    manifests.sort();
    manifests
}

fn git_cached_manifest_paths(root: &Path, evidence_dir: &Path) -> Option<Vec<PathBuf>> {
    let evidence_pathspec = display_path(root, evidence_dir);
    let output = Command::new(GIT_COMMAND)
        .arg(GIT_CWD_FLAG)
        .arg(root)
        .arg(GIT_LS_FILES_SUBCOMMAND)
        .arg(GIT_CACHED_FLAG)
        .arg(GIT_PATHSPEC_SEPARATOR)
        .arg(evidence_pathspec)
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let stdout = String::from_utf8(output.stdout).ok()?;
    let mut manifests = stdout
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .filter(|line| Path::new(line).extension() == Some(OsStr::new(MANIFEST_EXTENSION)))
        .map(|line| root.join(line))
        .collect::<Vec<_>>();
    if manifests.is_empty() {
        None
    } else {
        manifests.sort();
        Some(manifests)
    }
}

fn collect_manifest_paths(current: &Path, manifests: &mut Vec<PathBuf>) {
    if let Ok(entries) = fs::read_dir(current) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                collect_manifest_paths(&path, manifests);
            } else if path.is_file() && path.extension() == Some(OsStr::new(MANIFEST_EXTENSION)) {
                manifests.push(path);
            }
        }
    }
}

fn parse_all_manifests(inputs: Vec<ManifestInput>) -> Vec<ParsedManifest> {
    inputs
        .into_iter()
        .map(|input| ParsedManifest {
            absolute_path: input.absolute_path,
            display_path: input.display_path,
            rows: parse_manifest_text(&input.text),
        })
        .collect()
}

fn parse_manifest_text(text: &str) -> Vec<ParsedRow> {
    let mut rows = Vec::new();
    for (line_index, raw_line) in text.lines().enumerate() {
        let line_number = line_index + LINE_NUMBER_OFFSET;
        let original = raw_line.to_string();
        let trimmed = raw_line.trim();
        if trimmed.is_empty() {
            rows.push(ParsedRow::Blank { original });
            continue;
        }
        let Some((digest, path_text)) = trimmed.split_once(MANIFEST_SEPARATOR) else {
            rows.push(ParsedRow::Malformed {
                line_number,
                message: "expected '<b3>  <path>'".to_string(),
                original,
            });
            continue;
        };
        if !is_blake3_digest(digest) {
            rows.push(ParsedRow::Malformed {
                line_number,
                message: format!("invalid BLAKE3 digest {digest:?}"),
                original,
            });
            continue;
        }
        let relative_path = path_text.trim().to_string();
        if relative_path.is_empty() {
            rows.push(ParsedRow::Malformed {
                line_number,
                message: "path is empty".to_string(),
                original,
            });
            continue;
        }
        if path_is_outside_root(&relative_path) {
            rows.push(ParsedRow::OutsideRoot {
                line_number,
                relative_path,
                original,
            });
            continue;
        }
        rows.push(ParsedRow::Entry {
            line_number,
            original_digest: digest.to_string(),
            relative_path,
            original,
        });
    }
    if rows.is_empty() {
        rows.push(ParsedRow::Malformed {
            line_number: LINE_NUMBER_OFFSET,
            message: "manifest has no entries".to_string(),
            original: String::new(),
        });
    }
    rows
}

fn path_is_outside_root(path_text: &str) -> bool {
    let path = Path::new(path_text);
    path.is_absolute()
        || path
            .components()
            .any(|component| component.as_os_str() == OsStr::new(".."))
}

fn is_blake3_digest(value: &str) -> bool {
    value.len() == BLAKE3_HEX_LENGTH && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn referenced_relative_paths(parsed: &[ParsedManifest]) -> BTreeSet<String> {
    let mut paths = BTreeSet::new();
    for manifest in parsed {
        for row in &manifest.rows {
            if let ParsedRow::Entry { relative_path, .. } = row {
                paths.insert(relative_path.clone());
            }
        }
    }
    paths
}

fn load_file_states(
    root: &Path,
    relative_paths: &BTreeSet<String>,
) -> Result<BTreeMap<String, FileState>, Vec<String>> {
    let mut states = BTreeMap::new();
    let mut errors = Vec::new();
    for relative_path in relative_paths {
        match classify_file(root, relative_path) {
            Ok(FileState::CurrentFile { digest }) => {
                states.insert(relative_path.clone(), FileState::CurrentFile { digest });
            }
            Ok(state) => {
                states.insert(relative_path.clone(), state);
            }
            Err(error) => errors.push(error),
        }
    }
    if errors.is_empty() {
        Ok(states)
    } else {
        Err(errors)
    }
}

fn classify_file(root: &Path, relative_path: &str) -> Result<FileState, String> {
    let absolute_path = root.join(relative_path);
    if !absolute_path.exists() {
        return Ok(FileState::Missing);
    }
    if !absolute_path.is_file() {
        return Ok(FileState::NotFile);
    }
    let canonical = absolute_path
        .canonicalize()
        .map_err(|error| format!("{}: {error}", absolute_path.display()))?;
    if !canonical.starts_with(root) {
        return Ok(FileState::OutsideRoot);
    }
    let digest = b3sum_digest(&absolute_path)?;
    Ok(FileState::CurrentFile { digest })
}

fn plan_from_parsed(
    parsed: &[ParsedManifest],
    file_states: &BTreeMap<String, FileState>,
) -> RefreshPlan {
    let mut summary = RefreshSummary {
        manifests: parsed.len(),
        ..RefreshSummary::default()
    };
    let mut outputs = Vec::new();
    let mut diagnostics = Vec::new();

    for manifest in parsed {
        let mut changed = false;
        let mut output_lines = Vec::new();
        for row in &manifest.rows {
            match row {
                ParsedRow::Blank { original } => output_lines.push(original.clone()),
                ParsedRow::Malformed {
                    line_number,
                    message,
                    original,
                } => {
                    summary.malformed_rows += LINE_NUMBER_OFFSET;
                    diagnostics.push(format!(
                        "{}:{line_number}: {message}",
                        manifest.display_path
                    ));
                    output_lines.push(original.clone());
                }
                ParsedRow::OutsideRoot {
                    line_number,
                    relative_path,
                    original,
                } => {
                    summary.outside_root_rows += LINE_NUMBER_OFFSET;
                    diagnostics.push(format!(
                        "{}:{line_number}: path outside repository: {relative_path}",
                        manifest.display_path
                    ));
                    output_lines.push(original.clone());
                }
                ParsedRow::Entry {
                    line_number,
                    original_digest,
                    relative_path,
                    original,
                } => {
                    summary.entries += LINE_NUMBER_OFFSET;
                    match file_states.get(relative_path) {
                        Some(FileState::CurrentFile { digest }) if digest == original_digest => {
                            summary.current_rows += LINE_NUMBER_OFFSET;
                            output_lines.push(original.clone());
                        }
                        Some(FileState::CurrentFile { digest }) => {
                            summary.stale_rows += LINE_NUMBER_OFFSET;
                            diagnostics.push(format!(
                                "{}:{line_number}: stale digest for {relative_path}: expected {digest}, found {original_digest}",
                                manifest.display_path
                            ));
                            output_lines
                                .push(format!("{digest}{MANIFEST_SEPARATOR}{relative_path}"));
                            changed = true;
                        }
                        Some(FileState::Missing) => {
                            summary.missing_rows += LINE_NUMBER_OFFSET;
                            diagnostics.push(format!(
                                "{}:{line_number}: missing file: {relative_path}",
                                manifest.display_path
                            ));
                            output_lines.push(original.clone());
                        }
                        Some(FileState::NotFile) => {
                            summary.not_file_rows += LINE_NUMBER_OFFSET;
                            diagnostics.push(format!(
                                "{}:{line_number}: referenced path is not a file: {relative_path}",
                                manifest.display_path
                            ));
                            output_lines.push(original.clone());
                        }
                        Some(FileState::OutsideRoot) | None => {
                            summary.outside_root_rows += LINE_NUMBER_OFFSET;
                            diagnostics.push(format!(
                                "{}:{line_number}: path resolves outside repository: {relative_path}",
                                manifest.display_path
                            ));
                            output_lines.push(original.clone());
                        }
                    }
                }
            }
        }
        outputs.push(ManifestOutput {
            absolute_path: manifest.absolute_path.clone(),
            display_path: manifest.display_path.clone(),
            text: join_manifest_lines(&output_lines),
            changed,
        });
    }

    RefreshPlan {
        summary,
        outputs,
        diagnostics,
    }
}

fn join_manifest_lines(lines: &[String]) -> String {
    let mut text = String::new();
    for line in lines {
        text.push_str(line);
        text.push('\n');
    }
    text
}

fn write_changed_outputs(outputs: &[ManifestOutput]) -> Result<usize, Vec<String>> {
    let mut writes = 0;
    let mut errors = Vec::new();
    for output in outputs.iter().filter(|output| output.changed) {
        match fs::write(&output.absolute_path, &output.text) {
            Ok(()) => writes += LINE_NUMBER_OFFSET,
            Err(error) => errors.push(format!("{}: {error}", output.display_path)),
        }
    }
    if errors.is_empty() {
        Ok(writes)
    } else {
        Err(errors)
    }
}

fn summary_text(summary: &RefreshSummary, mode: &str, passes: usize) -> String {
    format!(
        "mode={mode}; passes={passes}; manifests={}; entries={}; current={}; stale={}; missing={}; malformed={}; outside_root={}; not_file={}",
        summary.manifests,
        summary.entries,
        summary.current_rows,
        summary.stale_rows,
        summary.missing_rows,
        summary.malformed_rows,
        summary.outside_root_rows,
        summary.not_file_rows,
    )
}

fn b3sum_digest(path: &Path) -> Result<String, String> {
    let output = Command::new("b3sum")
        .arg(path)
        .output()
        .map_err(|error| format!("b3sum {}: {error}", path.display()))?;
    if !output.status.success() {
        return Err(format!(
            "b3sum {} exited with {}",
            path.display(),
            output.status
        ));
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout
        .split_whitespace()
        .next()
        .map(ToOwned::to_owned)
        .ok_or_else(|| format!("b3sum {} produced no digest", path.display()))
}

fn display_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .into_owned()
}

fn run_self_tests() -> Result<String, Vec<String>> {
    unchanged_manifest_fixture()?;
    stale_manifest_check_does_not_write_fixture()?;
    refresh_updates_stale_manifest_fixture()?;
    cascading_manifest_refresh_fixture()?;
    partitioned_manifest_fixture()?;
    missing_file_fixture()?;
    malformed_manifest_fixture()?;
    outside_root_fixture()?;
    non_converging_manifest_fixture()?;
    Ok("positive and negative fixtures exercised".to_string())
}

fn unchanged_manifest_fixture() -> Result<(), Vec<String>> {
    let root = make_temp_root("unchanged")?;
    let evidence_dir = create_evidence_dir(&root)?;
    let note = evidence_dir.join("note.md");
    write_file(&note, "unchanged evidence\n")?;
    let digest = b3sum_digest(&note).map_err(single_error)?;
    write_manifest(
        &evidence_dir.join("note.b3"),
        &digest,
        "docs/evidence/note.md",
    )?;

    let summary = run_check(&root, &evidence_dir)?;
    assert_text_contains(&summary, "stale=0")?;
    remove_temp_root(&root)
}

fn stale_manifest_check_does_not_write_fixture() -> Result<(), Vec<String>> {
    let root = make_temp_root("check-stale")?;
    let evidence_dir = create_evidence_dir(&root)?;
    let note = evidence_dir.join("stale.md");
    write_file(&note, "before\n")?;
    let original_digest = b3sum_digest(&note).map_err(single_error)?;
    let manifest = evidence_dir.join("stale.b3");
    write_manifest(&manifest, &original_digest, "docs/evidence/stale.md")?;
    write_file(&note, "after\n")?;
    let before = read_file(&manifest)?;

    assert_error_contains(&run_check(&root, &evidence_dir), "stale digest")?;
    let after = read_file(&manifest)?;
    assert_equal_text(&after, &before, "check mode changed a stale manifest")?;
    remove_temp_root(&root)
}

fn refresh_updates_stale_manifest_fixture() -> Result<(), Vec<String>> {
    let root = make_temp_root("refresh-stale")?;
    let evidence_dir = create_evidence_dir(&root)?;
    let note = evidence_dir.join("refresh.md");
    write_file(&note, "before\n")?;
    let original_digest = b3sum_digest(&note).map_err(single_error)?;
    let manifest = evidence_dir.join("refresh.b3");
    write_manifest(&manifest, &original_digest, "docs/evidence/refresh.md")?;
    write_file(&note, "after\n")?;
    let current_digest = b3sum_digest(&note).map_err(single_error)?;

    let summary = run_refresh(&root, &evidence_dir, DEFAULT_MAX_REFRESH_PASSES)?;
    assert_text_contains(&summary, "manifest_writes=1")?;
    let manifest_text = read_file(&manifest)?;
    assert_text_contains(&manifest_text, &current_digest)?;
    run_check(&root, &evidence_dir)?;
    remove_temp_root(&root)
}

fn cascading_manifest_refresh_fixture() -> Result<(), Vec<String>> {
    let root = make_temp_root("cascade")?;
    let evidence_dir = create_evidence_dir(&root)?;
    let note = evidence_dir.join("leaf.md");
    write_file(&note, "leaf\n")?;
    let zero_digest = zero_digest();
    let leaf_manifest = evidence_dir.join("leaf.b3");
    let index_manifest = evidence_dir.join("index.b3");
    write_manifest(&leaf_manifest, &zero_digest, "docs/evidence/leaf.md")?;
    write_manifest(&index_manifest, &zero_digest, "docs/evidence/leaf.b3")?;

    let summary = run_refresh(&root, &evidence_dir, DEFAULT_MAX_REFRESH_PASSES)?;
    assert_text_contains(&summary, "passes=3")?;
    run_check(&root, &evidence_dir)?;
    remove_temp_root(&root)
}

fn partitioned_manifest_fixture() -> Result<(), Vec<String>> {
    let root = make_temp_root("partitioned")?;
    let evidence_dir = create_evidence_dir(&root)?;
    let run_log_dir = evidence_dir.join("run-logs").join("2026-06-24");
    let manifest_dir = evidence_dir.join("manifests").join("2026-06-24");
    fs::create_dir_all(&run_log_dir)
        .map_err(|error| vec![format!("{}: {error}", run_log_dir.display())])?;
    fs::create_dir_all(&manifest_dir)
        .map_err(|error| vec![format!("{}: {error}", manifest_dir.display())])?;
    let run_log = run_log_dir.join("partitioned.run.log");
    write_file(&run_log, "partitioned evidence\nexit_status=0\n")?;
    let digest = b3sum_digest(&run_log).map_err(single_error)?;
    write_manifest(
        &manifest_dir.join("partitioned.b3"),
        &digest,
        "docs/evidence/run-logs/2026-06-24/partitioned.run.log",
    )?;

    let summary = run_check(&root, &evidence_dir)?;
    assert_text_contains(&summary, "manifests=1")?;
    assert_text_contains(&summary, "entries=1")?;
    remove_temp_root(&root)
}

fn missing_file_fixture() -> Result<(), Vec<String>> {
    let root = make_temp_root("missing")?;
    let evidence_dir = create_evidence_dir(&root)?;
    let manifest = evidence_dir.join("missing.b3");
    write_manifest(&manifest, &zero_digest(), "docs/evidence/missing.run.log")?;
    let before = read_file(&manifest)?;

    assert_error_contains(&run_check(&root, &evidence_dir), "missing file")?;
    assert_error_contains(
        &run_refresh(&root, &evidence_dir, DEFAULT_MAX_REFRESH_PASSES),
        "missing file",
    )?;
    let after = read_file(&manifest)?;
    assert_equal_text(&after, &before, "refresh mode rewrote a missing-file row")?;
    remove_temp_root(&root)
}

fn malformed_manifest_fixture() -> Result<(), Vec<String>> {
    let root = make_temp_root("malformed")?;
    let evidence_dir = create_evidence_dir(&root)?;
    write_file(&evidence_dir.join("bad.b3"), "not a manifest row\n")?;

    assert_error_contains(&run_check(&root, &evidence_dir), "expected '<b3>  <path>'")?;
    assert_error_contains(
        &run_refresh(&root, &evidence_dir, DEFAULT_MAX_REFRESH_PASSES),
        "expected '<b3>  <path>'",
    )?;
    remove_temp_root(&root)
}

fn outside_root_fixture() -> Result<(), Vec<String>> {
    let root = make_temp_root("outside")?;
    let evidence_dir = create_evidence_dir(&root)?;
    write_manifest(
        &evidence_dir.join("outside.b3"),
        &zero_digest(),
        "../secret",
    )?;

    assert_error_contains(&run_check(&root, &evidence_dir), "path outside repository")?;
    assert_error_contains(
        &run_refresh(&root, &evidence_dir, DEFAULT_MAX_REFRESH_PASSES),
        "path outside repository",
    )?;
    remove_temp_root(&root)
}

fn non_converging_manifest_fixture() -> Result<(), Vec<String>> {
    let root = make_temp_root("non-converging")?;
    let evidence_dir = create_evidence_dir(&root)?;
    write_manifest(
        &evidence_dir.join("self.b3"),
        &zero_digest(),
        "docs/evidence/self.b3",
    )?;

    assert_error_contains(
        &run_refresh(&root, &evidence_dir, NON_CONVERGENCE_FIXTURE_MAX_PASSES),
        "did not converge",
    )?;
    remove_temp_root(&root)
}

fn make_temp_root(case_name: &str) -> Result<PathBuf, Vec<String>> {
    let root = env::temp_dir().join(format!(
        "{TEMP_DIR_PREFIX}-{case_name}-{}",
        std::process::id()
    ));
    if root.exists() {
        fs::remove_dir_all(&root).map_err(|error| vec![format!("{}: {error}", root.display())])?;
    }
    fs::create_dir_all(&root).map_err(|error| vec![format!("{}: {error}", root.display())])?;
    root.canonicalize()
        .map_err(|error| vec![format!("{}: {error}", root.display())])
}

fn create_evidence_dir(root: &Path) -> Result<PathBuf, Vec<String>> {
    let evidence_dir = root.join(DEFAULT_EVIDENCE_DIR);
    fs::create_dir_all(&evidence_dir)
        .map_err(|error| vec![format!("{}: {error}", evidence_dir.display())])?;
    Ok(evidence_dir)
}

fn remove_temp_root(root: &Path) -> Result<(), Vec<String>> {
    fs::remove_dir_all(root).map_err(|error| vec![format!("{}: {error}", root.display())])
}

fn write_file(path: &Path, text: &str) -> Result<(), Vec<String>> {
    fs::write(path, text).map_err(|error| vec![format!("{}: {error}", path.display())])
}

fn read_file(path: &Path) -> Result<String, Vec<String>> {
    fs::read_to_string(path).map_err(|error| vec![format!("{}: {error}", path.display())])
}

fn write_manifest(path: &Path, digest: &str, relative_path: &str) -> Result<(), Vec<String>> {
    write_file(
        path,
        &format!("{digest}{MANIFEST_SEPARATOR}{relative_path}\n"),
    )
}

fn zero_digest() -> String {
    "0".repeat(BLAKE3_HEX_LENGTH)
}

fn single_error(error: String) -> Vec<String> {
    vec![error]
}

fn assert_error_contains(
    result: &Result<String, Vec<String>>,
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

fn assert_text_contains(text: &str, needle: &str) -> Result<(), Vec<String>> {
    if text.contains(needle) {
        Ok(())
    } else {
        Err(vec![format!("missing text {needle:?}: {text:?}")])
    }
}

fn assert_equal_text(actual: &str, expected: &str, context: &str) -> Result<(), Vec<String>> {
    if actual == expected {
        Ok(())
    } else {
        Err(vec![format!(
            "{context}: expected {expected:?}, got {actual:?}"
        )])
    }
}
