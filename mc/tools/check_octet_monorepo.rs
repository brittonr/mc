use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::process::ExitCode;

const CHECK_NAME: &str = "mc-octet-monorepo";
const OCTET_SOURCE_ENV: &str = "OCTET_SOURCE_DIR";
const REVIEWED_BASELINE_SCHEMA: &str = "mc.octet.reviewed-baseline.v1";
const CARGO_DENY_FINDINGS_EXIT_CODE: i32 = 101;
const FAILURE_EXIT_CODE: u8 = 1;
const SUCCESS_EXIT: ExitCode = ExitCode::SUCCESS;
const LINT_INVENTORY_MACRO: &str = "lint_inventory!";
const WORKSPACE_METADATA_HEADER: &str = "[workspace.metadata.octet]";
const OCTET_HEADER: &str = "[octet]";
const OCTET_LINT_LEVELS_HEADER: &str = "[octet.lint_levels]";
const DENY_LEVEL: &str = "deny";
const DISABLED_LINTS_KEY: &str = "disabled_lints";
const DEFAULT_SCOPE_KEY: &str = "default_scope";
const CARGO_CHECK_ARGS_KEY: &str = "cargo_check_args";
const RESULTS_JSONL: &str = "results.jsonl";
const STATUS_JSON: &str = "status.json";
const STDOUT_LOG_SUFFIX: &str = ".stdout.jsonl";
const STDERR_LOG_SUFFIX: &str = ".stderr.log";
const DEFAULT_ARTIFACT_ROOT: &str = "target/octet-monorepo";
const COMPAT_RUNNER_BASELINE: &str = "compat/octet-baselines/compat-runner.reviewed-baseline.json";
const STEVENARELLA_BASELINE: &str = "compat/octet-baselines/stevenarella.reviewed-baseline.json";
const VALENCE_BASELINE: &str = "compat/octet-baselines/valence.reviewed-baseline.json";
const COMPAT_RUNNER_WORKSPACE: &str = "compat/runner";
const STEVENARELLA_WORKSPACE: &str = "clients/stevenarella";
const VALENCE_WORKSPACE: &str = "servers/valence";
const DEFAULT_CARGO_TOML: &str = "Cargo.toml";
const DEFAULT_DYLINT_TOML: &str = "dylint.toml";
const OCTET_LIB_RS: &str = "src/lib.rs";
const ARG_ROOT: &str = "--root";
const ARG_OCTET_SOURCE: &str = "--octet-source";
const ARG_CARGO_OCTET: &str = "--cargo-octet";
const ARG_ARTIFACT_ROOT: &str = "--artifact-root";
const ARG_RUN_OCTET: &str = "--run-octet";
const ARG_SELF_TEST: &str = "--self-test";
const ARG_HELP: &str = "--help";
const CARGO_OCTET_FLAKE_REF_PREFIX: &str = "path:";
const CARGO_OCTET_FLAKE_REF_SUFFIX: &str = "#cargo-octet";
const NIX_PROGRAM: &str = "nix";
const NIX_RUN_ARG: &str = "run";
const NIX_SEPARATOR_ARG: &str = "--";
const CARGO_OCTET_CHECK_ARG: &str = "check";
const OUTPUT_FORMAT_ARG: &str = "--output-format";
const JSON_FORMAT_ARG: &str = "json";
const ARTIFACT_DIR_ARG: &str = "--artifact-dir";
const JSON_KEY_SCHEMA: &str = "schema";
const JSON_KEY_STABLE_ID: &str = "stable_id";
const JSON_KEY_ACCEPTED_STABLE_IDS: &str = "accepted_stable_ids";
const JSON_KEY_TOTAL_FINDINGS: &str = "total_findings";
const JSON_KEY_CARGO_PROCESS_EXIT: &str = "cargo_process_exit";
const JSON_KEY_CODE: &str = "code";
const JSON_KEY_STATUS: &str = "status";
const JSON_KEY_WORKSPACE: &str = "workspace";
const JSON_KEY_OWNER: &str = "owner";
const JSON_KEY_RATIONALE: &str = "rationale";
const JSON_KEY_REMOVAL_CONDITION: &str = "removal_condition";
const STATUS_INTEGRATION_FAILURE: &str = "integration-failure";
const STATUS_LINT_FAILURE: &str = "lint-failure";
const STATUS_WARNING_ONLY: &str = "warning-only";
const STATUS_CLEAN: &str = "clean";

#[derive(Debug, Clone, Copy)]
struct WorkspaceSpec {
    name: &'static str,
    path: &'static str,
    baseline_path: &'static str,
}

const WORKSPACES: &[WorkspaceSpec] = &[
    WorkspaceSpec {
        name: "compat-runner",
        path: COMPAT_RUNNER_WORKSPACE,
        baseline_path: COMPAT_RUNNER_BASELINE,
    },
    WorkspaceSpec {
        name: "stevenarella",
        path: STEVENARELLA_WORKSPACE,
        baseline_path: STEVENARELLA_BASELINE,
    },
    WorkspaceSpec {
        name: "valence",
        path: VALENCE_WORKSPACE,
        baseline_path: VALENCE_BASELINE,
    },
];

#[derive(Debug, Clone)]
struct Args {
    root: PathBuf,
    octet_source: Option<PathBuf>,
    cargo_octet: Option<PathBuf>,
    artifact_root: PathBuf,
    run_octet: bool,
    self_test: bool,
    help: bool,
}

#[derive(Debug, Clone)]
struct WorkspaceInput {
    spec: WorkspaceSpec,
    cargo_toml: Option<String>,
    dylint_toml: Option<String>,
    baseline: Option<String>,
}

#[derive(Debug, Clone)]
struct StaticWorkspaceReport {
    name: &'static str,
    accepted_stable_ids: BTreeSet<String>,
    diagnostics: Vec<String>,
}

#[derive(Debug, Clone)]
struct StatusSummary {
    status: String,
    total_findings: usize,
    cargo_process_code: Option<i32>,
}

#[derive(Debug, Clone)]
struct BaselineComparison {
    current_total: usize,
    accepted_current: usize,
    stale_total: usize,
    new_ids: Vec<String>,
}

#[derive(Debug, Clone)]
struct DynamicWorkspaceReport {
    name: &'static str,
    command_status: Option<i32>,
    status: StatusSummary,
    comparison: BaselineComparison,
    artifact_dir: PathBuf,
    stdout_log: PathBuf,
    stderr_log: PathBuf,
    diagnostics: Vec<String>,
}

fn main() -> ExitCode {
    let args = match parse_args(env::args_os().skip(1)) {
        Ok(args) => args,
        Err(error) => {
            eprintln!("{CHECK_NAME}: {error}");
            return ExitCode::from(FAILURE_EXIT_CODE);
        }
    };

    if args.help {
        print_usage();
        return SUCCESS_EXIT;
    }

    if args.self_test {
        return match run_self_test() {
            Ok(()) => SUCCESS_EXIT,
            Err(error) => {
                eprintln!("{CHECK_NAME} self-test failed: {error}");
                ExitCode::from(FAILURE_EXIT_CODE)
            }
        };
    }

    match run_check(&args) {
        Ok(()) => SUCCESS_EXIT,
        Err(errors) => {
            for error in errors {
                eprintln!("{CHECK_NAME}: {error}");
            }
            ExitCode::from(FAILURE_EXIT_CODE)
        }
    }
}

fn parse_args(raw_args: impl Iterator<Item = OsString>) -> Result<Args, String> {
    let mut root = PathBuf::from(".");
    let mut octet_source = None;
    let mut cargo_octet = None;
    let mut artifact_root = PathBuf::from(DEFAULT_ARTIFACT_ROOT);
    let mut run_octet = false;
    let mut self_test = false;
    let mut help = false;
    let args = raw_args.collect::<Vec<_>>();
    let mut index = 0;

    while index < args.len() {
        let arg = args[index].to_string_lossy();
        match arg.as_ref() {
            ARG_ROOT => {
                index += 1;
                root = PathBuf::from(required_arg_value(&args, index, ARG_ROOT)?);
            }
            ARG_OCTET_SOURCE => {
                index += 1;
                octet_source = Some(PathBuf::from(required_arg_value(
                    &args,
                    index,
                    ARG_OCTET_SOURCE,
                )?));
            }
            ARG_CARGO_OCTET => {
                index += 1;
                cargo_octet = Some(PathBuf::from(required_arg_value(
                    &args,
                    index,
                    ARG_CARGO_OCTET,
                )?));
            }
            ARG_ARTIFACT_ROOT => {
                index += 1;
                artifact_root = PathBuf::from(required_arg_value(&args, index, ARG_ARTIFACT_ROOT)?);
            }
            ARG_RUN_OCTET => run_octet = true,
            ARG_SELF_TEST => self_test = true,
            ARG_HELP => help = true,
            other => return Err(format!("unknown argument `{other}`")),
        }
        index += 1;
    }

    if octet_source.is_none() {
        octet_source = env::var_os(OCTET_SOURCE_ENV).map(PathBuf::from);
    }

    Ok(Args {
        root,
        octet_source,
        cargo_octet,
        artifact_root,
        run_octet,
        self_test,
        help,
    })
}

fn required_arg_value(args: &[OsString], index: usize, flag: &str) -> Result<String, String> {
    let value = args
        .get(index)
        .ok_or_else(|| format!("missing value after `{flag}`"))?;
    Ok(value.to_string_lossy().to_string())
}

fn print_usage() {
    println!("Usage: tools/check_octet_monorepo.rs [--root PATH] --octet-source PATH [--run-octet] [--cargo-octet PATH] [--artifact-root PATH]");
    println!("       tools/check_octet_monorepo.rs --self-test");
    println!("Validates Octet metadata/dylint/baseline coverage for owned mc Rust workspaces.");
    println!("With --run-octet, runs repo-pinned cargo-octet gates and rejects new unaccepted stable IDs.");
}

fn run_check(args: &Args) -> Result<(), Vec<String>> {
    let args = normalized_paths(args);
    let mut errors = Vec::new();
    let Some(octet_source) = args.octet_source.as_ref() else {
        return Err(vec![format!(
            "missing Octet source; pass `{ARG_OCTET_SOURCE}` or set `{OCTET_SOURCE_ENV}`"
        )]);
    };

    let octet_lib_path = octet_source.join(OCTET_LIB_RS);
    let octet_lib = read_to_string_for_check(&octet_lib_path, &mut errors).unwrap_or_default();
    let lint_inventory = extract_lint_inventory(&octet_lib).unwrap_or_else(|error| {
        errors.push(format!("{}: {error}", octet_lib_path.display()));
        BTreeSet::new()
    });

    if !lint_inventory.is_empty() {
        println!(
            "{CHECK_NAME}: lint_inventory_count={}",
            lint_inventory.len()
        );
    }

    let static_reports = WORKSPACES
        .iter()
        .map(|spec| load_workspace_input(&args.root, *spec, &mut errors))
        .map(|input| validate_workspace_static(&lint_inventory, &input))
        .collect::<Vec<_>>();

    for report in &static_reports {
        println!(
            "{CHECK_NAME}: workspace={} accepted_baseline_ids={}",
            report.name,
            report.accepted_stable_ids.len()
        );
        errors.extend(report.diagnostics.clone());
    }

    if args.run_octet {
        match run_dynamic_gates(&args, &static_reports) {
            Ok(dynamic_reports) => {
                for report in dynamic_reports {
                    println!(
                        "{CHECK_NAME}: workspace={} total_findings={} current_unique_ids={} accepted_current={} stale={} new={} artifact={}",
                        report.name,
                        report.status.total_findings,
                        report.comparison.current_total,
                        report.comparison.accepted_current,
                        report.comparison.stale_total,
                        report.comparison.new_ids.len(),
                        report.artifact_dir.display()
                    );
                    if let Some(status) = report.command_status {
                        println!(
                            "{CHECK_NAME}: workspace={} command_exit_status={status}",
                            report.name
                        );
                    }
                    println!(
                        "{CHECK_NAME}: workspace={} octet_status={}",
                        report.name, report.status.status
                    );
                    println!(
                        "{CHECK_NAME}: workspace={} stdout_log={}",
                        report.name,
                        report.stdout_log.display()
                    );
                    println!(
                        "{CHECK_NAME}: workspace={} stderr_log={}",
                        report.name,
                        report.stderr_log.display()
                    );
                    errors.extend(report.diagnostics);
                }
            }
            Err(dynamic_errors) => errors.extend(dynamic_errors),
        }
    }

    if errors.is_empty() {
        println!("{CHECK_NAME}: PASS");
        Ok(())
    } else {
        Err(errors)
    }
}

fn normalized_paths(args: &Args) -> Args {
    let root = fs::canonicalize(&args.root).unwrap_or_else(|_| args.root.clone());
    let artifact_root = if args.artifact_root.is_absolute() {
        args.artifact_root.clone()
    } else {
        root.join(&args.artifact_root)
    };

    Args {
        root,
        octet_source: args.octet_source.clone(),
        cargo_octet: args.cargo_octet.clone(),
        artifact_root,
        run_octet: args.run_octet,
        self_test: args.self_test,
        help: args.help,
    }
}

fn read_to_string_for_check(path: &Path, errors: &mut Vec<String>) -> Option<String> {
    match fs::read_to_string(path) {
        Ok(text) => Some(text),
        Err(error) => {
            errors.push(format!("read {}: {error}", path.display()));
            None
        }
    }
}

fn load_workspace_input(
    root: &Path,
    spec: WorkspaceSpec,
    errors: &mut Vec<String>,
) -> WorkspaceInput {
    let workspace_root = root.join(spec.path);
    let cargo_toml = read_to_string_for_check(&workspace_root.join(DEFAULT_CARGO_TOML), errors);
    let dylint_toml = read_to_string_for_check(&workspace_root.join(DEFAULT_DYLINT_TOML), errors);
    let baseline = read_to_string_for_check(&root.join(spec.baseline_path), errors);

    WorkspaceInput {
        spec,
        cargo_toml,
        dylint_toml,
        baseline,
    }
}

fn extract_lint_inventory(source: &str) -> Result<BTreeSet<String>, String> {
    let macro_start = source
        .find(LINT_INVENTORY_MACRO)
        .ok_or_else(|| format!("missing `{LINT_INVENTORY_MACRO}` macro"))?;
    let block = macro_body(source, macro_start)
        .ok_or_else(|| format!("unterminated `{LINT_INVENTORY_MACRO}` macro"))?;
    let mut names = BTreeSet::new();
    let mut cursor = 0;

    while let Some(relative_quote) = block[cursor..].find('"') {
        let quote_start = cursor + relative_quote + 1;
        let Some(relative_end) = block[quote_start..].find('"') else {
            return Err("unterminated lint name string".to_string());
        };
        let quote_end = quote_start + relative_end;
        let after_quote = block[quote_end + 1..].trim_start();
        if after_quote.starts_with(',') {
            names.insert(block[quote_start..quote_end].to_string());
        }
        cursor = quote_end + 1;
    }

    if names.is_empty() {
        return Err("lint inventory did not contain any registered lint names".to_string());
    }

    Ok(names)
}

fn macro_body(source: &str, macro_start: usize) -> Option<&str> {
    let search = &source[macro_start..];
    let body_start_relative = search.find('{')?;
    let body_start = macro_start + body_start_relative;
    let mut depth = 0usize;
    for (relative_index, byte) in source[body_start..].bytes().enumerate() {
        match byte {
            b'{' => depth += 1,
            b'}' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    let body_end = body_start + relative_index;
                    return Some(&source[body_start + 1..body_end]);
                }
            }
            _ => {}
        }
    }
    None
}

fn validate_workspace_static(
    lint_inventory: &BTreeSet<String>,
    input: &WorkspaceInput,
) -> StaticWorkspaceReport {
    let mut diagnostics = Vec::new();
    let mut accepted_stable_ids = BTreeSet::new();

    match input.cargo_toml.as_deref() {
        Some(cargo_toml) => validate_cargo_metadata(input.spec, cargo_toml, &mut diagnostics),
        None => diagnostics.push(format!("{}: missing Cargo.toml", input.spec.path)),
    }

    match input.dylint_toml.as_deref() {
        Some(dylint_toml) => {
            validate_dylint_config(input.spec, lint_inventory, dylint_toml, &mut diagnostics)
        }
        None => diagnostics.push(format!("{}: missing dylint.toml", input.spec.path)),
    }

    match input.baseline.as_deref() {
        Some(baseline) => match validate_reviewed_baseline(input.spec, baseline) {
            Ok(ids) => accepted_stable_ids = ids,
            Err(baseline_errors) => diagnostics.extend(baseline_errors),
        },
        None => diagnostics.push(format!(
            "{}: missing reviewed Octet baseline {}",
            input.spec.path, input.spec.baseline_path
        )),
    }

    StaticWorkspaceReport {
        name: input.spec.name,
        accepted_stable_ids,
        diagnostics,
    }
}

fn validate_cargo_metadata(spec: WorkspaceSpec, cargo_toml: &str, diagnostics: &mut Vec<String>) {
    let Some(section) = section_body(cargo_toml, WORKSPACE_METADATA_HEADER) else {
        diagnostics.push(format!(
            "{}: missing {WORKSPACE_METADATA_HEADER}",
            spec.path
        ));
        return;
    };
    if !section_contains_key(section, DEFAULT_SCOPE_KEY) {
        diagnostics.push(format!(
            "{}: {WORKSPACE_METADATA_HEADER} missing `{DEFAULT_SCOPE_KEY}`",
            spec.path
        ));
    }
    if !section_contains_key(section, CARGO_CHECK_ARGS_KEY) {
        diagnostics.push(format!(
            "{}: {WORKSPACE_METADATA_HEADER} missing `{CARGO_CHECK_ARGS_KEY}`",
            spec.path
        ));
    }
}

fn validate_dylint_config(
    spec: WorkspaceSpec,
    lint_inventory: &BTreeSet<String>,
    dylint_toml: &str,
    diagnostics: &mut Vec<String>,
) {
    if section_body(dylint_toml, OCTET_HEADER).is_none() {
        diagnostics.push(format!("{}: dylint.toml missing {OCTET_HEADER}", spec.path));
    }

    if let Some(octet_section) = section_body(dylint_toml, OCTET_HEADER) {
        if section_contains_key(octet_section, DISABLED_LINTS_KEY) {
            diagnostics.push(format!(
                "{}: dylint.toml uses `{DISABLED_LINTS_KEY}`; use reviewed baselines or narrow documented source suppressions instead",
                spec.path
            ));
        }
    }

    let Some(lint_levels_section) = section_body(dylint_toml, OCTET_LINT_LEVELS_HEADER) else {
        diagnostics.push(format!(
            "{}: dylint.toml missing {OCTET_LINT_LEVELS_HEADER}",
            spec.path
        ));
        return;
    };

    let lint_levels = parse_lint_levels(lint_levels_section);
    for lint_name in lint_inventory {
        match lint_levels.get(lint_name) {
            Some(level) if level == DENY_LEVEL => {}
            Some(level) => diagnostics.push(format!(
                "{}: lint `{lint_name}` must be deny, found `{level}`",
                spec.path
            )),
            None => diagnostics.push(format!(
                "{}: missing deny level for Octet lint `{lint_name}`",
                spec.path
            )),
        }
    }

    for lint_name in lint_levels.keys() {
        if !lint_inventory.contains(lint_name) {
            diagnostics.push(format!(
                "{}: unknown Octet lint level `{lint_name}`",
                spec.path
            ));
        }
    }
}

fn validate_reviewed_baseline(
    spec: WorkspaceSpec,
    baseline: &str,
) -> Result<BTreeSet<String>, Vec<String>> {
    let mut diagnostics = Vec::new();
    if extract_json_string_value(baseline, JSON_KEY_SCHEMA).as_deref()
        != Some(REVIEWED_BASELINE_SCHEMA)
    {
        diagnostics.push(format!(
            "{}: reviewed baseline schema must be {REVIEWED_BASELINE_SCHEMA}",
            spec.baseline_path
        ));
    }
    if extract_json_string_value(baseline, JSON_KEY_WORKSPACE).as_deref() != Some(spec.name) {
        diagnostics.push(format!(
            "{}: reviewed baseline workspace must be `{}`",
            spec.baseline_path, spec.name
        ));
    }
    for required_key in [
        JSON_KEY_OWNER,
        JSON_KEY_RATIONALE,
        JSON_KEY_REMOVAL_CONDITION,
    ] {
        if extract_json_string_value(baseline, required_key)
            .is_none_or(|value| value.trim().is_empty())
        {
            diagnostics.push(format!(
                "{}: reviewed baseline missing non-empty `{required_key}`",
                spec.baseline_path
            ));
        }
    }

    let accepted_stable_ids = extract_json_string_array(baseline, JSON_KEY_ACCEPTED_STABLE_IDS)
        .into_iter()
        .collect::<BTreeSet<_>>();
    if accepted_stable_ids.is_empty() {
        diagnostics.push(format!(
            "{}: reviewed baseline has no accepted stable IDs",
            spec.baseline_path
        ));
    }

    let finding_stable_ids = extract_json_string_values_for_key(baseline, JSON_KEY_STABLE_ID)
        .into_iter()
        .collect::<BTreeSet<_>>();
    if finding_stable_ids != accepted_stable_ids {
        diagnostics.push(format!(
            "{}: accepted_stable_ids and accepted_findings stable IDs differ",
            spec.baseline_path
        ));
    }

    if diagnostics.is_empty() {
        Ok(accepted_stable_ids)
    } else {
        Err(diagnostics)
    }
}

fn section_body<'a>(text: &'a str, header: &str) -> Option<&'a str> {
    let start = text.find(header)?;
    let after_header = start + header.len();
    let rest = &text[after_header..];
    let next_section = rest
        .lines()
        .scan(0usize, |offset, line| {
            let current_offset = *offset;
            *offset += line.len() + 1;
            Some((current_offset, line))
        })
        .find_map(|(offset, line)| {
            let trimmed = line.trim();
            if trimmed.starts_with('[') && trimmed.ends_with(']') {
                Some(offset)
            } else {
                None
            }
        })
        .unwrap_or(rest.len());
    Some(&rest[..next_section])
}

fn section_contains_key(section: &str, key: &str) -> bool {
    section.lines().any(|line| {
        let trimmed = strip_comment(line).trim();
        trimmed.starts_with(key) && trimmed[key.len()..].trim_start().starts_with('=')
    })
}

fn parse_lint_levels(section: &str) -> BTreeMap<String, String> {
    let mut levels = BTreeMap::new();
    for line in section.lines() {
        let trimmed = strip_comment(line).trim();
        if trimmed.is_empty() {
            continue;
        }
        let Some((key, value)) = trimmed.split_once('=') else {
            continue;
        };
        levels.insert(
            key.trim().to_string(),
            trim_toml_string(value.trim()).to_string(),
        );
    }
    levels
}

fn strip_comment(line: &str) -> &str {
    line.split_once('#')
        .map(|(before_comment, _comment)| before_comment)
        .unwrap_or(line)
}

fn trim_toml_string(value: &str) -> &str {
    value
        .strip_prefix('"')
        .and_then(|without_prefix| without_prefix.strip_suffix('"'))
        .unwrap_or(value)
}

fn run_dynamic_gates(
    args: &Args,
    static_reports: &[StaticWorkspaceReport],
) -> Result<Vec<DynamicWorkspaceReport>, Vec<String>> {
    let mut errors = Vec::new();
    let mut reports = Vec::new();
    if let Err(error) = fs::create_dir_all(&args.artifact_root) {
        return Err(vec![format!(
            "create artifact root {}: {error}",
            args.artifact_root.display()
        )]);
    }

    for spec in WORKSPACES {
        let Some(static_report) = static_reports
            .iter()
            .find(|report| report.name == spec.name)
        else {
            errors.push(format!("{}: missing static report", spec.name));
            continue;
        };
        match run_one_dynamic_gate(args, *spec, &static_report.accepted_stable_ids) {
            Ok(report) => reports.push(report),
            Err(report_errors) => errors.extend(report_errors),
        }
    }

    if errors.is_empty() {
        Ok(reports)
    } else {
        Err(errors)
    }
}

fn run_one_dynamic_gate(
    args: &Args,
    spec: WorkspaceSpec,
    accepted_stable_ids: &BTreeSet<String>,
) -> Result<DynamicWorkspaceReport, Vec<String>> {
    let workspace_root = args.root.join(spec.path);
    let artifact_dir = args.artifact_root.join(spec.name);
    let stdout_log = args
        .artifact_root
        .join(format!("{}{}", spec.name, STDOUT_LOG_SUFFIX));
    let stderr_log = args
        .artifact_root
        .join(format!("{}{}", spec.name, STDERR_LOG_SUFFIX));
    let mut diagnostics = Vec::new();

    if let Err(error) = fs::create_dir_all(&artifact_dir) {
        return Err(vec![format!(
            "{}: create artifact dir {}: {error}",
            spec.name,
            artifact_dir.display()
        )]);
    }

    let output = match run_cargo_octet_command(args, &workspace_root, &artifact_dir) {
        Ok(output) => output,
        Err(error) => return Err(vec![format!("{}: run cargo-octet: {error}", spec.name)]),
    };

    let command_status = output.status.code();
    write_log(
        &stdout_log,
        &output.stdout,
        &mut diagnostics,
        spec.name,
        "stdout",
    );
    write_log(
        &stderr_log,
        &output.stderr,
        &mut diagnostics,
        spec.name,
        "stderr",
    );

    let results_path = artifact_dir.join(RESULTS_JSONL);
    let status_path = artifact_dir.join(STATUS_JSON);
    let results_text = fs::read_to_string(&results_path).unwrap_or_else(|error| {
        diagnostics.push(format!(
            "{}: read {}: {error}",
            spec.name,
            results_path.display()
        ));
        String::new()
    });
    let status_text = fs::read_to_string(&status_path).unwrap_or_else(|error| {
        diagnostics.push(format!(
            "{}: read {}: {error}",
            spec.name,
            status_path.display()
        ));
        String::new()
    });

    let current_ids = extract_stable_ids_from_jsonl(&results_text);
    let status = parse_status_summary(&status_text).unwrap_or_else(|error| {
        diagnostics.push(format!("{}: parse status.json: {error}", spec.name));
        StatusSummary {
            status: String::new(),
            total_findings: current_ids.len(),
            cargo_process_code: command_status,
        }
    });
    let comparison = compare_baseline(accepted_stable_ids, &current_ids);

    validate_dynamic_outcome(spec, command_status, &status, &comparison, &mut diagnostics);

    Ok(DynamicWorkspaceReport {
        name: spec.name,
        command_status,
        status,
        comparison,
        artifact_dir,
        stdout_log,
        stderr_log,
        diagnostics,
    })
}

fn run_cargo_octet_command(
    args: &Args,
    workspace_root: &Path,
    artifact_dir: &Path,
) -> Result<std::process::Output, String> {
    let mut command = if let Some(cargo_octet) = args.cargo_octet.as_ref() {
        let mut command = Command::new(cargo_octet);
        command.arg(CARGO_OCTET_CHECK_ARG);
        command
    } else {
        let flake_ref = format!(
            "{CARGO_OCTET_FLAKE_REF_PREFIX}{}{CARGO_OCTET_FLAKE_REF_SUFFIX}",
            args.root.display()
        );
        let mut command = Command::new(NIX_PROGRAM);
        command
            .arg(NIX_RUN_ARG)
            .arg(flake_ref)
            .arg(NIX_SEPARATOR_ARG)
            .arg(CARGO_OCTET_CHECK_ARG);
        command
    };

    command
        .current_dir(workspace_root)
        .arg(OUTPUT_FORMAT_ARG)
        .arg(JSON_FORMAT_ARG)
        .arg(ARTIFACT_DIR_ARG)
        .arg(artifact_dir);

    command.output().map_err(|error| error.to_string())
}

fn write_log(
    path: &Path,
    bytes: &[u8],
    diagnostics: &mut Vec<String>,
    workspace: &str,
    stream: &str,
) {
    if let Err(error) = fs::write(path, bytes) {
        diagnostics.push(format!(
            "{workspace}: write {stream} log {}: {error}",
            path.display()
        ));
    }
}

fn parse_status_summary(text: &str) -> Result<StatusSummary, String> {
    let status = extract_json_string_value(text, JSON_KEY_STATUS)
        .ok_or_else(|| "missing status".to_string())?;
    let total_findings = extract_json_usize_value(text, JSON_KEY_TOTAL_FINDINGS)
        .ok_or_else(|| "missing total_findings".to_string())?;
    let cargo_section = extract_json_object(text, JSON_KEY_CARGO_PROCESS_EXIT).unwrap_or_default();
    let cargo_process_code = extract_json_i32_value(&cargo_section, JSON_KEY_CODE);

    Ok(StatusSummary {
        status,
        total_findings,
        cargo_process_code,
    })
}

fn compare_baseline(accepted: &BTreeSet<String>, current: &BTreeSet<String>) -> BaselineComparison {
    let new_ids = current.difference(accepted).cloned().collect::<Vec<_>>();
    let stale_total = accepted.difference(current).count();
    let accepted_current = current.intersection(accepted).count();

    BaselineComparison {
        current_total: current.len(),
        accepted_current,
        stale_total,
        new_ids,
    }
}

fn validate_dynamic_outcome(
    spec: WorkspaceSpec,
    command_status: Option<i32>,
    status: &StatusSummary,
    comparison: &BaselineComparison,
    diagnostics: &mut Vec<String>,
) {
    if status.total_findings > 0 && comparison.current_total == 0 {
        diagnostics.push(format!(
            "{}: status total_findings={} but no stable IDs were parsed",
            spec.name, status.total_findings
        ));
    }

    if !comparison.new_ids.is_empty() {
        diagnostics.push(format!(
            "{}: new unaccepted Octet findings: {}",
            spec.name,
            comparison.new_ids.join(", ")
        ));
    }

    let accepted_deny_lint_exit = status.status == STATUS_INTEGRATION_FAILURE
        && status.cargo_process_code == Some(CARGO_DENY_FINDINGS_EXIT_CODE)
        && status.total_findings > 0
        && comparison.new_ids.is_empty();
    let accepted_clean_exit = matches!(
        status.status.as_str(),
        STATUS_CLEAN | STATUS_WARNING_ONLY | STATUS_LINT_FAILURE
    ) && comparison.new_ids.is_empty();

    if !(accepted_deny_lint_exit || accepted_clean_exit) {
        diagnostics.push(format!(
            "{}: Octet status `{}` command_exit={:?} cargo_process_exit={:?} is not an accepted baseline outcome",
            spec.name, status.status, command_status, status.cargo_process_code
        ));
    }
}

fn extract_stable_ids_from_jsonl(text: &str) -> BTreeSet<String> {
    text.lines()
        .flat_map(|line| extract_json_string_values_for_key(line, JSON_KEY_STABLE_ID))
        .collect::<BTreeSet<_>>()
}

fn extract_json_string_values_for_key(text: &str, key: &str) -> Vec<String> {
    let mut values = Vec::new();
    let needle = format!("\"{key}\"");
    let mut cursor = 0;
    while let Some(relative_key_start) = text[cursor..].find(&needle) {
        let key_start = cursor + relative_key_start + needle.len();
        let Some(colon_relative) = text[key_start..].find(':') else {
            break;
        };
        let value_start = key_start + colon_relative + 1;
        if let Some((value, next_cursor)) = parse_json_string_at(text, value_start) {
            values.push(value);
            cursor = next_cursor;
        } else {
            cursor = value_start;
        }
    }
    values
}

fn extract_json_string_value(text: &str, key: &str) -> Option<String> {
    extract_json_string_values_for_key(text, key)
        .into_iter()
        .next()
}

fn extract_json_string_array(text: &str, key: &str) -> Vec<String> {
    let needle = format!("\"{key}\"");
    let Some(key_start) = text.find(&needle).map(|index| index + needle.len()) else {
        return Vec::new();
    };
    let Some(colon_relative) = text[key_start..].find(':') else {
        return Vec::new();
    };
    let after_colon = key_start + colon_relative + 1;
    let Some(array_start_relative) = text[after_colon..].find('[') else {
        return Vec::new();
    };
    let array_start = after_colon + array_start_relative + 1;
    let Some(array_end_relative) = text[array_start..].find(']') else {
        return Vec::new();
    };
    let array = &text[array_start..array_start + array_end_relative];
    let mut values = Vec::new();
    let mut cursor = 0;
    while cursor < array.len() {
        if let Some((value, next_cursor)) = parse_json_string_at(array, cursor) {
            values.push(value);
            cursor = next_cursor;
        } else {
            cursor += 1;
        }
    }
    values
}

fn parse_json_string_at(text: &str, start: usize) -> Option<(String, usize)> {
    let mut cursor = start;
    while cursor < text.len() && text.as_bytes()[cursor].is_ascii_whitespace() {
        cursor += 1;
    }
    if text.as_bytes().get(cursor).copied()? != b'"' {
        return None;
    }
    cursor += 1;
    let mut value = String::new();
    let bytes = text.as_bytes();
    while cursor < text.len() {
        match bytes[cursor] {
            b'\\' => {
                let escaped_index = cursor + 1;
                let escaped = *bytes.get(escaped_index)?;
                value.push(escaped as char);
                cursor += 2;
            }
            b'"' => return Some((value, cursor + 1)),
            byte => {
                value.push(byte as char);
                cursor += 1;
            }
        }
    }
    None
}

fn extract_json_i32_value(text: &str, key: &str) -> Option<i32> {
    extract_json_number_string(text, key)?.parse::<i32>().ok()
}

fn extract_json_usize_value(text: &str, key: &str) -> Option<usize> {
    extract_json_number_string(text, key)?.parse::<usize>().ok()
}

fn extract_json_number_string(text: &str, key: &str) -> Option<String> {
    let needle = format!("\"{key}\"");
    let key_start = text.find(&needle)? + needle.len();
    let colon_relative = text[key_start..].find(':')?;
    let mut cursor = key_start + colon_relative + 1;
    while cursor < text.len() && text.as_bytes()[cursor].is_ascii_whitespace() {
        cursor += 1;
    }
    let number_start = cursor;
    while cursor < text.len() {
        let byte = text.as_bytes()[cursor];
        if byte.is_ascii_digit() || byte == b'-' {
            cursor += 1;
        } else {
            break;
        }
    }
    if number_start == cursor {
        None
    } else {
        Some(text[number_start..cursor].to_string())
    }
}

fn extract_json_object(text: &str, key: &str) -> Option<String> {
    let needle = format!("\"{key}\"");
    let key_start = text.find(&needle)? + needle.len();
    let colon_relative = text[key_start..].find(':')?;
    let object_search_start = key_start + colon_relative + 1;
    let object_start_relative = text[object_search_start..].find('{')?;
    let object_start = object_search_start + object_start_relative;
    let mut depth = 0usize;
    for (relative_index, byte) in text[object_start..].bytes().enumerate() {
        match byte {
            b'{' => depth += 1,
            b'}' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    let object_end = object_start + relative_index + 1;
                    return Some(text[object_start..object_end].to_string());
                }
            }
            _ => {}
        }
    }
    None
}

fn run_self_test() -> Result<(), String> {
    let inventory_source = r#"
lint_inventory! {
    "alpha", safety::alpha::ALPHA => |lint_store, _cfg| {},
    "beta", safety::beta::BETA => |lint_store, _cfg| {},
}
"#;
    let inventory = extract_lint_inventory(inventory_source)?;
    assert_contains(&inventory, "alpha", "inventory alpha")?;
    assert_contains(&inventory, "beta", "inventory beta")?;

    let valid_cargo = r#"
[workspace]
resolver = "2"

[workspace.metadata.octet]
default_scope = ["-p", "fixture"]
cargo_check_args = []
"#;
    let valid_dylint = r#"
[octet]

[octet.lint_levels]
alpha = "deny"
beta = "deny"
"#;
    let valid_baseline = r#"
{
  "schema": "mc.octet.reviewed-baseline.v1",
  "workspace": "compat-runner",
  "owner": "mc workspace maintainers",
  "rationale": "fixture rationale",
  "removal_condition": "fixture removal",
  "accepted_stable_ids": ["id-alpha"],
  "accepted_findings": [{"stable_id": "id-alpha"}]
}
"#;
    let valid_input = WorkspaceInput {
        spec: WORKSPACES[0],
        cargo_toml: Some(valid_cargo.to_string()),
        dylint_toml: Some(valid_dylint.to_string()),
        baseline: Some(valid_baseline.to_string()),
    };
    let report = validate_workspace_static(&inventory, &valid_input);
    assert_empty(&report.diagnostics, "valid static workspace")?;

    let drift_dylint = r#"
[octet]

[octet.lint_levels]
alpha = "deny"
"#;
    let drift_input = WorkspaceInput {
        spec: WORKSPACES[0],
        cargo_toml: Some(valid_cargo.to_string()),
        dylint_toml: Some(drift_dylint.to_string()),
        baseline: Some(valid_baseline.to_string()),
    };
    let drift_report = validate_workspace_static(&inventory, &drift_input);
    assert_error_contains(
        &drift_report.diagnostics,
        "missing deny level",
        "lint drift negative",
    )?;

    let missing_config_input = WorkspaceInput {
        spec: WORKSPACES[0],
        cargo_toml: Some("[workspace]\n".to_string()),
        dylint_toml: None,
        baseline: Some(valid_baseline.to_string()),
    };
    let missing_config_report = validate_workspace_static(&inventory, &missing_config_input);
    assert_error_contains(
        &missing_config_report.diagnostics,
        "missing dylint.toml",
        "missing config negative",
    )?;
    assert_error_contains(
        &missing_config_report.diagnostics,
        "missing [workspace.metadata.octet]",
        "missing metadata negative",
    )?;

    let accepted = BTreeSet::from(["id-alpha".to_string()]);
    let current_clean = BTreeSet::from(["id-alpha".to_string()]);
    let clean_comparison = compare_baseline(&accepted, &current_clean);
    assert_empty(&clean_comparison.new_ids, "accepted finding comparison")?;
    let current_with_new = BTreeSet::from(["id-alpha".to_string(), "id-beta".to_string()]);
    let new_comparison = compare_baseline(&accepted, &current_with_new);
    if new_comparison.new_ids != vec!["id-beta".to_string()] {
        return Err("new finding negative did not isolate id-beta".to_string());
    }

    let status = parse_status_summary(
        r#"{"status":"integration-failure","total_findings":1,"cargo_process_exit":{"classification":"exit-code","code":101}}"#,
    )?;
    let mut diagnostics = Vec::new();
    validate_dynamic_outcome(
        WORKSPACES[0],
        Some(2),
        &status,
        &clean_comparison,
        &mut diagnostics,
    );
    assert_empty(&diagnostics, "accepted deny-level dynamic outcome")?;
    validate_dynamic_outcome(
        WORKSPACES[0],
        Some(2),
        &status,
        &new_comparison,
        &mut diagnostics,
    );
    assert_error_contains(
        &diagnostics,
        "new unaccepted Octet findings",
        "new finding dynamic negative",
    )?;

    println!("ok - inventory positive");
    println!("ok - lint drift negative");
    println!("ok - missing config negative");
    println!("ok - new finding negative");
    Ok(())
}

fn assert_contains(set: &BTreeSet<String>, value: &str, label: &str) -> Result<(), String> {
    if set.contains(value) {
        Ok(())
    } else {
        Err(format!("{label}: missing `{value}`"))
    }
}

fn assert_empty<T: std::fmt::Debug>(values: &[T], label: &str) -> Result<(), String> {
    if values.is_empty() {
        Ok(())
    } else {
        Err(format!("{label}: expected empty, got {values:?}"))
    }
}

fn assert_error_contains(errors: &[String], needle: &str, label: &str) -> Result<(), String> {
    if errors.iter().any(|error| error.contains(needle)) {
        Ok(())
    } else {
        Err(format!(
            "{label}: missing error containing `{needle}` in {errors:?}"
        ))
    }
}
