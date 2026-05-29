use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

const SELF_TEST_FLAG: &str = "--self-test";
const CHANGE_ROOT: &str = "cairn/changes";
const TASKS_FILE_NAME: &str = "tasks.md";
const COMPLETED_TASK_MARKER: &str = "- [x]";
const OPEN_TASK_MARKER: &str = "- [ ]";
const EVIDENCE_LABEL: &str = "evidence:";
const DOCS_EVIDENCE_PREFIX: &str = "docs/evidence/";
const RUN_LOG_SUFFIX: &str = ".run.log";
const B3_SUFFIX: &str = ".b3";
const BLAKE3_LABEL: &str = "blake3";
const BLAKE3_HEX_LENGTH: usize = 64;
const TEST_BLAKE3_DIGEST: &str =
    "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

#[derive(Debug, Clone, PartialEq, Eq)]
struct TaskFile {
    path: String,
    text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TaskBlock {
    path: String,
    line_number: usize,
    checked: bool,
    title: String,
    body: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TaskEvidence {
    has_evidence_label: bool,
    paths: Vec<String>,
    has_inline_blake3: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ValidationSummary {
    task_files: usize,
    completed_tasks: usize,
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.iter().any(|arg| arg == SELF_TEST_FLAG) {
        return match run_self_tests() {
            Ok(summary) => {
                println!("task evidence self-test passed: {summary}");
                ExitCode::SUCCESS
            }
            Err(errors) => {
                print_errors(&errors);
                ExitCode::FAILURE
            }
        };
    }

    match run_repo_check(Path::new(".")) {
        Ok(summary) => {
            println!(
                "task evidence gate passed: {} completed tasks checked across {} active task files",
                summary.completed_tasks, summary.task_files
            );
            ExitCode::SUCCESS
        }
        Err(errors) => {
            print_errors(&errors);
            ExitCode::FAILURE
        }
    }
}

fn print_errors(errors: &[String]) {
    for error in errors {
        eprintln!("task evidence gate failed: {error}");
    }
}

fn run_repo_check(root: &Path) -> Result<ValidationSummary, Vec<String>> {
    let task_files = read_active_task_files(root)?;
    let evidence_files = read_evidence_file_set(root)?;
    validate_task_files(&task_files, &evidence_files)
}

fn read_active_task_files(root: &Path) -> Result<Vec<TaskFile>, Vec<String>> {
    let changes_root = root.join(CHANGE_ROOT);
    if !changes_root.exists() {
        return Ok(Vec::new());
    }

    let entries = fs::read_dir(&changes_root)
        .map_err(|err| vec![format!("{}: {err}", changes_root.display())])?;
    let mut task_paths = Vec::new();
    for entry_result in entries {
        let entry = entry_result.map_err(|err| vec![format!("{}: {err}", changes_root.display())])?;
        let metadata = entry
            .metadata()
            .map_err(|err| vec![format!("{}: {err}", entry.path().display())])?;
        if metadata.is_dir() {
            let task_path = entry.path().join(TASKS_FILE_NAME);
            if task_path.exists() {
                task_paths.push(task_path);
            }
        }
    }
    task_paths.sort();

    let mut task_files = Vec::new();
    for path in task_paths {
        let text = fs::read_to_string(&path)
            .map_err(|err| vec![format!("{}: {err}", path.display())])?;
        task_files.push(TaskFile {
            path: relative_path(root, &path)?,
            text,
        });
    }
    Ok(task_files)
}

fn read_evidence_file_set(root: &Path) -> Result<BTreeSet<String>, Vec<String>> {
    let evidence_root = root.join(DOCS_EVIDENCE_PREFIX);
    let mut files = BTreeSet::new();
    if !evidence_root.exists() {
        return Ok(files);
    }
    collect_files(root, &evidence_root, &mut files)?;
    Ok(files)
}

fn collect_files(
    root: &Path,
    current: &Path,
    files: &mut BTreeSet<String>,
) -> Result<(), Vec<String>> {
    let entries =
        fs::read_dir(current).map_err(|err| vec![format!("{}: {err}", current.display())])?;
    for entry_result in entries {
        let entry = entry_result.map_err(|err| vec![format!("{}: {err}", current.display())])?;
        let path = entry.path();
        let metadata = entry
            .metadata()
            .map_err(|err| vec![format!("{}: {err}", path.display())])?;
        if metadata.is_dir() {
            collect_files(root, &path, files)?;
        } else if metadata.is_file() {
            files.insert(relative_path(root, &path)?);
        }
    }
    Ok(())
}

fn relative_path(root: &Path, path: &Path) -> Result<String, Vec<String>> {
    path.strip_prefix(root)
        .map_err(|err| vec![format!("{}: {err}", path.display())])?
        .to_str()
        .map(|text| text.replace('\\', "/"))
        .ok_or_else(|| vec![format!("{} is not valid UTF-8", path.display())])
}

fn validate_task_files(
    task_files: &[TaskFile],
    evidence_files: &BTreeSet<String>,
) -> Result<ValidationSummary, Vec<String>> {
    let mut errors = Vec::new();
    let mut completed_tasks = 0;
    for task_file in task_files {
        for task in parse_task_file(task_file) {
            if task.checked {
                completed_tasks += 1;
                errors.extend(validate_completed_task(&task, evidence_files));
            }
        }
    }

    if errors.is_empty() {
        Ok(ValidationSummary {
            task_files: task_files.len(),
            completed_tasks,
        })
    } else {
        Err(errors)
    }
}

fn parse_task_file(task_file: &TaskFile) -> Vec<TaskBlock> {
    let mut tasks = Vec::new();
    let mut current: Option<TaskBlock> = None;

    for (line_index, line) in task_file.text.lines().enumerate() {
        let trimmed = line.trim_start();
        if is_task_marker(trimmed) {
            if let Some(task) = current.take() {
                tasks.push(task);
            }
            current = Some(TaskBlock {
                path: task_file.path.clone(),
                line_number: line_index + 1,
                checked: trimmed.starts_with(COMPLETED_TASK_MARKER),
                title: trimmed.to_string(),
                body: String::new(),
            });
        } else if let Some(task) = current.as_mut() {
            task.body.push_str(line);
            task.body.push('\n');
        }
    }

    if let Some(task) = current {
        tasks.push(task);
    }
    tasks
}

fn is_task_marker(trimmed: &str) -> bool {
    trimmed.starts_with(COMPLETED_TASK_MARKER) || trimmed.starts_with(OPEN_TASK_MARKER)
}

fn validate_completed_task(task: &TaskBlock, evidence_files: &BTreeSet<String>) -> Vec<String> {
    let evidence = inspect_task_evidence(task);
    let location = format!("{}:{}", task.path, task.line_number);
    let mut errors = Vec::new();

    if !evidence.has_evidence_label {
        errors.push(format!(
            "{location} completed task lacks an Evidence/Validation evidence line"
        ));
    }
    if !has_non_manifest_evidence_path(&evidence.paths) {
        errors.push(format!(
            "{location} completed task lacks a copied docs/evidence artifact path"
        ));
    }
    if !has_run_log_path(&evidence.paths) {
        errors.push(format!(
            "{location} completed task lacks verification command output as a docs/evidence/*.run.log path"
        ));
    }
    if !has_blake3_evidence(&evidence) {
        errors.push(format!(
            "{location} completed task lacks a docs/evidence/*.b3 manifest path or inline BLAKE3 digest"
        ));
    }

    for path in &evidence.paths {
        if !evidence_files.contains(path) {
            errors.push(format!(
                "{location} cites missing evidence artifact {path}"
            ));
        }
    }

    errors
}

fn inspect_task_evidence(task: &TaskBlock) -> TaskEvidence {
    let text = format!("{}\n{}", task.title, task.body);
    let lowercase = text.to_ascii_lowercase();
    TaskEvidence {
        has_evidence_label: lowercase.contains(EVIDENCE_LABEL),
        paths: extract_docs_evidence_paths(&text),
        has_inline_blake3: contains_inline_blake3(&text),
    }
}

fn has_non_manifest_evidence_path(paths: &[String]) -> bool {
    paths.iter().any(|path| !path.ends_with(B3_SUFFIX))
}

fn has_run_log_path(paths: &[String]) -> bool {
    paths.iter().any(|path| path.ends_with(RUN_LOG_SUFFIX))
}

fn has_blake3_evidence(evidence: &TaskEvidence) -> bool {
    evidence.has_inline_blake3 || evidence.paths.iter().any(|path| path.ends_with(B3_SUFFIX))
}

fn extract_docs_evidence_paths(text: &str) -> Vec<String> {
    let mut paths = BTreeSet::new();
    let mut search_start = 0;
    while let Some(relative_start) = text[search_start..].find(DOCS_EVIDENCE_PREFIX) {
        let start = search_start + relative_start;
        let mut end = start;
        for (offset, character) in text[start..].char_indices() {
            if is_path_character(character) {
                end = start + offset + character.len_utf8();
            } else {
                break;
            }
        }
        let candidate = text[start..end].trim_end_matches(is_trailing_path_punctuation);
        if is_artifact_path(candidate) {
            paths.insert(candidate.to_string());
        }
        search_start = end;
    }
    paths.into_iter().collect()
}

fn is_artifact_path(candidate: &str) -> bool {
    !candidate.is_empty() && candidate != DOCS_EVIDENCE_PREFIX && !candidate.ends_with('/')
}

fn is_path_character(character: char) -> bool {
    character.is_ascii_alphanumeric()
        || matches!(character, '/' | '.' | '_' | '-' | '+' | '=' | ':')
}

fn is_trailing_path_punctuation(character: char) -> bool {
    matches!(character, '.' | ',' | ';' | ':' | ')' | ']')
}

fn contains_inline_blake3(text: &str) -> bool {
    let lowercase = text.to_ascii_lowercase();
    lowercase.contains(BLAKE3_LABEL) && contains_hex_digest(&lowercase)
}

fn contains_hex_digest(text: &str) -> bool {
    let mut run_length = 0;
    for character in text.chars() {
        if character.is_ascii_hexdigit() {
            run_length += 1;
            if run_length == BLAKE3_HEX_LENGTH {
                return true;
            }
        } else {
            run_length = 0;
        }
    }
    false
}

fn run_self_tests() -> Result<String, Vec<String>> {
    let mut errors = Vec::new();
    errors.extend(expect_ok("complete task with manifest", positive_manifest_fixture()));
    errors.extend(expect_ok("complete task with inline digest", positive_inline_fixture()));
    errors.extend(expect_error(
        "missing evidence label",
        missing_evidence_label_fixture(),
        "lacks an Evidence/Validation evidence line",
    ));
    errors.extend(expect_error(
        "missing docs evidence path",
        missing_evidence_path_fixture(),
        "lacks a copied docs/evidence artifact path",
    ));
    errors.extend(expect_error(
        "missing run log",
        missing_run_log_fixture(),
        "lacks verification command output",
    ));
    errors.extend(expect_error(
        "missing blake3",
        missing_blake3_fixture(),
        "lacks a docs/evidence/*.b3 manifest path or inline BLAKE3 digest",
    ));
    errors.extend(expect_error(
        "missing artifact file",
        missing_artifact_file_fixture(),
        "cites missing evidence artifact docs/evidence/missing.run.log",
    ));
    errors.extend(expect_ok(
        "directory mention ignored",
        directory_mention_fixture(),
    ));

    if errors.is_empty() {
        Ok("positive fixtures and fail-closed mutations passed".to_string())
    } else {
        Err(errors)
    }
}

fn expect_ok(name: &str, fixture: Fixture) -> Vec<String> {
    match validate_task_files(&fixture.task_files, &fixture.evidence_files) {
        Ok(_) => Vec::new(),
        Err(errors) => vec![format!("{name}: expected ok, got {errors:?}")],
    }
}

fn expect_error(name: &str, fixture: Fixture, needle: &str) -> Vec<String> {
    match validate_task_files(&fixture.task_files, &fixture.evidence_files) {
        Ok(summary) => vec![format!("{name}: expected error, got {summary:?}")],
        Err(errors) => {
            if errors.iter().any(|error| error.contains(needle)) {
                Vec::new()
            } else {
                vec![format!(
                    "{name}: expected error containing {needle:?}, got {errors:?}"
                )]
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Fixture {
    task_files: Vec<TaskFile>,
    evidence_files: BTreeSet<String>,
}

fn fixture(task_text: &str, evidence_paths: &[&str]) -> Fixture {
    Fixture {
        task_files: vec![TaskFile {
            path: "cairn/changes/example/tasks.md".to_string(),
            text: task_text.to_string(),
        }],
        evidence_files: evidence_paths.iter().map(|path| path.to_string()).collect(),
    }
}

fn positive_manifest_fixture() -> Fixture {
    fixture(
        "## Tasks\n\n- [x] [serial] Ship gate.\n  - Evidence: `docs/evidence/gate.run.log` records `cargo test`; BLAKE3 manifest `docs/evidence/gate.b3`.\n",
        &["docs/evidence/gate.run.log", "docs/evidence/gate.b3"],
    )
}

fn positive_inline_fixture() -> Fixture {
    fixture(
        &format!(
            "## Tasks\n\n- [x] [serial] Ship gate.\n  - Validation evidence: docs/evidence/gate.run.log records checks. BLAKE3: {TEST_BLAKE3_DIGEST}\n"
        ),
        &["docs/evidence/gate.run.log"],
    )
}

fn missing_evidence_label_fixture() -> Fixture {
    fixture(
        "## Tasks\n\n- [x] [serial] Ship gate.\n  - Proof: `docs/evidence/gate.run.log`; BLAKE3 manifest `docs/evidence/gate.b3`.\n",
        &["docs/evidence/gate.run.log", "docs/evidence/gate.b3"],
    )
}

fn missing_evidence_path_fixture() -> Fixture {
    fixture(
        &format!(
            "## Tasks\n\n- [x] [serial] Ship gate.\n  - Evidence: Validation passed. BLAKE3: {TEST_BLAKE3_DIGEST}\n"
        ),
        &[],
    )
}

fn missing_run_log_fixture() -> Fixture {
    fixture(
        "## Tasks\n\n- [x] [serial] Ship gate.\n  - Evidence: `docs/evidence/gate.md`; BLAKE3 manifest `docs/evidence/gate.b3`.\n",
        &["docs/evidence/gate.md", "docs/evidence/gate.b3"],
    )
}

fn missing_blake3_fixture() -> Fixture {
    fixture(
        "## Tasks\n\n- [x] [serial] Ship gate.\n  - Evidence: `docs/evidence/gate.run.log` records `cargo test`.\n",
        &["docs/evidence/gate.run.log"],
    )
}

fn missing_artifact_file_fixture() -> Fixture {
    fixture(
        "## Tasks\n\n- [x] [serial] Ship gate.\n  - Evidence: `docs/evidence/missing.run.log`; BLAKE3 manifest `docs/evidence/gate.b3`.\n",
        &["docs/evidence/gate.b3"],
    )
}

fn directory_mention_fixture() -> Fixture {
    fixture(
        "## Tasks\n\n- [x] [serial] Record validation output under `docs/evidence/`.\n  - Evidence: `docs/evidence/gate.run.log`; BLAKE3 manifest `docs/evidence/gate.b3`.\n",
        &["docs/evidence/gate.run.log", "docs/evidence/gate.b3"],
    )
}
