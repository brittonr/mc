use std::collections::{BTreeMap, BTreeSet};
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
const TARGET_ARTIFACT_PREFIX: &str = "target/";
const TARGET_ARTIFACT_INFIX: &str = "/target/";
const STEVENARELLA_CHILD_PREFIX: &str = "stevenarella/";
const VALENCE_CHILD_PREFIX: &str = "valence/";
const HYPERION_CHILD_PREFIX: &str = "hyperion/";
const LEAFISH_CHILD_PREFIX: &str = "Leafish/";
const FORBIDDEN_ARTIFACT_PREFIXES: &[&str] = &[
    TARGET_ARTIFACT_PREFIX,
    STEVENARELLA_CHILD_PREFIX,
    VALENCE_CHILD_PREFIX,
    HYPERION_CHILD_PREFIX,
    LEAFISH_CHILD_PREFIX,
];
const EXIT_STATUS_TOKEN: &str = "exit_status=";
const SUCCESS_EXIT_STATUS_TOKEN: &str = "exit_status=0";
const B3_MANIFEST_SEPARATOR: &str = "  ";
const BLAKE3_LABEL: &str = "blake3";
const BLAKE3_HEX_LENGTH: usize = 64;
const TEST_BLAKE3_DIGEST: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

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
    forbidden_paths: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct EvidenceCatalog {
    paths: BTreeSet<String>,
    text_by_path: BTreeMap<String, String>,
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

    let task_path_args = args.iter().skip(1).map(String::as_str).collect::<Vec<_>>();
    let result = if task_path_args.is_empty() {
        run_repo_check(Path::new("."))
    } else {
        run_explicit_task_file_check(Path::new("."), &task_path_args)
    };

    match result {
        Ok(summary) => {
            println!(
                "task evidence gate passed: {} completed tasks checked across {} task files",
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
    let evidence_catalog = read_evidence_catalog(root)?;
    validate_task_files(&task_files, &evidence_catalog)
}

fn run_explicit_task_file_check(
    root: &Path,
    task_path_args: &[&str],
) -> Result<ValidationSummary, Vec<String>> {
    let task_files = read_explicit_task_files(root, task_path_args)?;
    let evidence_catalog = read_evidence_catalog(root)?;
    validate_task_files(&task_files, &evidence_catalog)
}

fn read_explicit_task_files(
    root: &Path,
    task_path_args: &[&str],
) -> Result<Vec<TaskFile>, Vec<String>> {
    let mut errors = Vec::new();
    let mut task_files = Vec::new();
    for path_arg in task_path_args {
        if path_arg.starts_with('-') {
            errors.push(format!("unknown flag {path_arg}"));
            continue;
        }
        let path = root.join(path_arg);
        match fs::read_to_string(&path) {
            Ok(text) => match relative_path(root, &path) {
                Ok(relative) => task_files.push(TaskFile {
                    path: relative,
                    text,
                }),
                Err(mut path_errors) => errors.append(&mut path_errors),
            },
            Err(err) => errors.push(format!("{}: {err}", path.display())),
        }
    }
    if errors.is_empty() {
        Ok(task_files)
    } else {
        Err(errors)
    }
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
        let entry =
            entry_result.map_err(|err| vec![format!("{}: {err}", changes_root.display())])?;
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
        let text =
            fs::read_to_string(&path).map_err(|err| vec![format!("{}: {err}", path.display())])?;
        task_files.push(TaskFile {
            path: relative_path(root, &path)?,
            text,
        });
    }
    Ok(task_files)
}

fn read_evidence_catalog(root: &Path) -> Result<EvidenceCatalog, Vec<String>> {
    let evidence_root = root.join(DOCS_EVIDENCE_PREFIX);
    let mut catalog = EvidenceCatalog {
        paths: BTreeSet::new(),
        text_by_path: BTreeMap::new(),
    };
    if !evidence_root.exists() {
        return Ok(catalog);
    }
    collect_files(root, &evidence_root, &mut catalog)?;
    Ok(catalog)
}

fn collect_files(
    root: &Path,
    current: &Path,
    catalog: &mut EvidenceCatalog,
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
            collect_files(root, &path, catalog)?;
        } else if metadata.is_file() {
            let relative = relative_path(root, &path)?;
            if should_read_evidence_text(&relative) {
                let text = fs::read_to_string(&path)
                    .map_err(|err| vec![format!("{}: {err}", path.display())])?;
                catalog.text_by_path.insert(relative.clone(), text);
            }
            catalog.paths.insert(relative);
        }
    }
    Ok(())
}

fn should_read_evidence_text(relative: &str) -> bool {
    relative.ends_with(RUN_LOG_SUFFIX) || relative.ends_with(B3_SUFFIX)
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
    evidence_catalog: &EvidenceCatalog,
) -> Result<ValidationSummary, Vec<String>> {
    let mut errors = Vec::new();
    let mut completed_tasks = 0;
    for task_file in task_files {
        for task in parse_task_file(task_file) {
            if task.checked {
                completed_tasks += 1;
                errors.extend(validate_completed_task(&task, evidence_catalog));
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

fn validate_completed_task(task: &TaskBlock, evidence_catalog: &EvidenceCatalog) -> Vec<String> {
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
        if !evidence_catalog.paths.contains(path) {
            errors.push(format!("{location} cites missing evidence artifact {path}"));
        }
    }

    for path in &evidence.forbidden_paths {
        errors.push(format!(
            "{location} cites non-reviewable artifact path {path}; copy review-critical bytes under docs/evidence/"
        ));
    }

    errors.extend(validate_run_log_contents(
        &location,
        &evidence,
        evidence_catalog,
    ));
    errors
}

fn validate_run_log_contents(
    location: &str,
    evidence: &TaskEvidence,
    evidence_catalog: &EvidenceCatalog,
) -> Vec<String> {
    let mut errors = Vec::new();
    let manifest_paths = evidence
        .paths
        .iter()
        .filter(|path| path.ends_with(B3_SUFFIX))
        .collect::<Vec<_>>();

    for run_log_path in evidence
        .paths
        .iter()
        .filter(|path| path.ends_with(RUN_LOG_SUFFIX))
    {
        match evidence_catalog.text_by_path.get(run_log_path) {
            Some(text) => validate_run_log_exit_status(location, run_log_path, text, &mut errors),
            None => errors.push(format!(
                "{location} cites run log {run_log_path} but its text is unavailable"
            )),
        }

        if !evidence.has_inline_blake3
            && !manifest_paths.iter().any(|manifest_path| {
                manifest_covers_path(evidence_catalog, manifest_path, run_log_path)
            })
        {
            errors.push(format!(
                "{location} cites run log {run_log_path} without a cited .b3 manifest covering it"
            ));
        }
    }

    errors
}

fn validate_run_log_exit_status(
    location: &str,
    run_log_path: &str,
    text: &str,
    errors: &mut Vec<String>,
) {
    let status_lines = text
        .lines()
        .filter(|line| line.contains(EXIT_STATUS_TOKEN))
        .collect::<Vec<_>>();
    if status_lines.is_empty() {
        errors.push(format!(
            "{location} cites run log {run_log_path} without explicit {SUCCESS_EXIT_STATUS_TOKEN} evidence"
        ));
        return;
    }

    for line in status_lines {
        if !line_has_success_exit_status(line) {
            errors.push(format!(
                "{location} cites run log {run_log_path} with nonzero exit status line: {line}"
            ));
        }
    }
}

fn line_has_success_exit_status(line: &str) -> bool {
    line.rsplit_once(EXIT_STATUS_TOKEN)
        .is_some_and(|(_, status)| status.trim() == "0")
}

fn manifest_covers_path(
    evidence_catalog: &EvidenceCatalog,
    manifest_path: &str,
    run_log_path: &str,
) -> bool {
    evidence_catalog
        .text_by_path
        .get(manifest_path)
        .is_some_and(|text| manifest_text_covers_path(text, run_log_path))
}

fn manifest_text_covers_path(text: &str, run_log_path: &str) -> bool {
    text.lines().any(|line| {
        line.split_once(B3_MANIFEST_SEPARATOR)
            .is_some_and(|(_, path)| path.trim() == run_log_path)
    })
}

fn inspect_task_evidence(task: &TaskBlock) -> TaskEvidence {
    let text = format!("{}\n{}", task.title, task.body);
    let lowercase = text.to_ascii_lowercase();
    TaskEvidence {
        has_evidence_label: lowercase.contains(EVIDENCE_LABEL),
        paths: extract_docs_evidence_paths(&text),
        has_inline_blake3: contains_inline_blake3(&text),
        forbidden_paths: extract_forbidden_artifact_paths(&text),
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

fn extract_forbidden_artifact_paths(text: &str) -> Vec<String> {
    let mut paths = BTreeSet::new();
    for prefix in FORBIDDEN_ARTIFACT_PREFIXES {
        let mut search_start = 0;
        while let Some(relative_start) = text[search_start..].find(prefix) {
            let start = search_start + relative_start;
            let candidate = extract_path_at(text, start);
            if is_forbidden_artifact_path(&candidate) {
                paths.insert(candidate);
            }
            search_start = start + prefix.len();
        }
    }
    paths.into_iter().collect()
}

fn extract_path_at(text: &str, start: usize) -> String {
    let mut end = start;
    for (offset, character) in text[start..].char_indices() {
        if is_path_character(character) {
            end = start + offset + character.len_utf8();
        } else {
            break;
        }
    }
    text[start..end]
        .trim_end_matches(is_trailing_path_punctuation)
        .to_string()
}

fn is_forbidden_artifact_path(candidate: &str) -> bool {
    candidate.starts_with(TARGET_ARTIFACT_PREFIX)
        || candidate.contains(TARGET_ARTIFACT_INFIX)
        || candidate.starts_with(STEVENARELLA_CHILD_PREFIX)
        || candidate.starts_with(VALENCE_CHILD_PREFIX)
        || candidate.starts_with(HYPERION_CHILD_PREFIX)
        || candidate.starts_with(LEAFISH_CHILD_PREFIX)
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
    errors.extend(expect_ok(
        "complete task with manifest",
        positive_manifest_fixture(),
    ));
    errors.extend(expect_ok(
        "complete task with inline digest",
        positive_inline_fixture(),
    ));
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
    errors.extend(expect_error(
        "run log missing exit status",
        run_log_missing_exit_status_fixture(),
        "without explicit exit_status=0 evidence",
    ));
    errors.extend(expect_error(
        "run log failed exit status",
        run_log_failed_exit_status_fixture(),
        "nonzero exit status line",
    ));
    errors.extend(expect_error(
        "manifest missing run log",
        manifest_missing_run_log_fixture(),
        "without a cited .b3 manifest covering it",
    ));
    errors.extend(expect_error(
        "target artifact rejected",
        target_artifact_fixture(),
        "non-reviewable artifact path target/mc-compat-smoke.json",
    ));
    errors.extend(expect_error(
        "nested child artifact rejected",
        nested_child_artifact_fixture(),
        "non-reviewable artifact path stevenarella/src/main.rs",
    ));

    if errors.is_empty() {
        Ok("positive fixtures and fail-closed mutations passed".to_string())
    } else {
        Err(errors)
    }
}

fn expect_ok(name: &str, fixture: Fixture) -> Vec<String> {
    match validate_task_files(&fixture.task_files, &fixture.evidence_catalog) {
        Ok(_) => Vec::new(),
        Err(errors) => vec![format!("{name}: expected ok, got {errors:?}")],
    }
}

fn expect_error(name: &str, fixture: Fixture, needle: &str) -> Vec<String> {
    match validate_task_files(&fixture.task_files, &fixture.evidence_catalog) {
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
    evidence_catalog: EvidenceCatalog,
}

fn fixture(task_text: &str, evidence_paths: &[&str]) -> Fixture {
    let mut catalog = EvidenceCatalog {
        paths: evidence_paths.iter().map(|path| path.to_string()).collect(),
        text_by_path: BTreeMap::new(),
    };
    for path in evidence_paths {
        if path.ends_with(RUN_LOG_SUFFIX) {
            catalog
                .text_by_path
                .insert((*path).to_string(), success_run_log_text());
        } else if path.ends_with(B3_SUFFIX) {
            catalog
                .text_by_path
                .insert((*path).to_string(), success_manifest_text(evidence_paths));
        }
    }

    Fixture {
        task_files: vec![TaskFile {
            path: "cairn/changes/example/tasks.md".to_string(),
            text: task_text.to_string(),
        }],
        evidence_catalog: catalog,
    }
}

fn fixture_with_texts(
    task_text: &str,
    evidence_paths: &[&str],
    text_by_path: &[(&str, &str)],
) -> Fixture {
    let mut fixture = fixture(task_text, evidence_paths);
    for (path, text) in text_by_path {
        fixture
            .evidence_catalog
            .text_by_path
            .insert((*path).to_string(), (*text).to_string());
    }
    fixture
}

fn success_run_log_text() -> String {
    "command=cargo test\ncargo_test_exit_status=0\n".to_string()
}

fn success_manifest_text(evidence_paths: &[&str]) -> String {
    evidence_paths
        .iter()
        .map(|path| format!("{TEST_BLAKE3_DIGEST}{B3_MANIFEST_SEPARATOR}{path}\n"))
        .collect()
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

fn run_log_missing_exit_status_fixture() -> Fixture {
    fixture_with_texts(
        "## Tasks\n\n- [x] [serial] Ship gate.\n  - Evidence: `docs/evidence/gate.run.log`; BLAKE3 manifest `docs/evidence/gate.b3`.\n",
        &["docs/evidence/gate.run.log", "docs/evidence/gate.b3"],
        &[("docs/evidence/gate.run.log", "command=cargo test\ntest result: ok\n")],
    )
}

fn run_log_failed_exit_status_fixture() -> Fixture {
    fixture_with_texts(
        "## Tasks\n\n- [x] [serial] Ship gate.\n  - Evidence: `docs/evidence/gate.run.log`; BLAKE3 manifest `docs/evidence/gate.b3`.\n",
        &["docs/evidence/gate.run.log", "docs/evidence/gate.b3"],
        &[("docs/evidence/gate.run.log", "command=cargo test\ncargo_test_exit_status=1\n")],
    )
}

fn manifest_missing_run_log_fixture() -> Fixture {
    fixture_with_texts(
        "## Tasks\n\n- [x] [serial] Ship gate.\n  - Evidence: `docs/evidence/gate.run.log`; BLAKE3 manifest `docs/evidence/gate.b3`.\n",
        &["docs/evidence/gate.run.log", "docs/evidence/gate.b3"],
        &[(
            "docs/evidence/gate.b3",
            "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef  docs/evidence/other.run.log\n",
        )],
    )
}

fn target_artifact_fixture() -> Fixture {
    fixture(
        "## Tasks\n\n- [x] [serial] Ship gate.\n  - Evidence: target/mc-compat-smoke.json was copied to `docs/evidence/gate.run.log`; BLAKE3 manifest `docs/evidence/gate.b3`.\n",
        &["docs/evidence/gate.run.log", "docs/evidence/gate.b3"],
    )
}

fn nested_child_artifact_fixture() -> Fixture {
    fixture(
        "## Tasks\n\n- [x] [serial] Ship gate.\n  - Evidence: stevenarella/src/main.rs was copied to `docs/evidence/gate.run.log`; BLAKE3 manifest `docs/evidence/gate.b3`.\n",
        &["docs/evidence/gate.run.log", "docs/evidence/gate.b3"],
    )
}
