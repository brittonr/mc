use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

const MANIFEST_PATH: &str = "config/mc-compat/scenario-manifest.ncl";
const GENERATED_RUST_PATH: &str = "tools/mc-compat-runner/src/scenario_manifest_generated.rs";
const RUNNER_MAIN_PATH: &str = "tools/mc-compat-runner/src/main.rs";
const RUNNER_SCENARIO_CORE_PATH: &str = "tools/mc-compat-runner/src/scenario_core.rs";
const RUNNER_SURFACE_PATH: &str = "tools/mc-compat-runner/src/{main.rs,scenario_core.rs}";
const FLAKE_PATH: &str = "flake.nix";
const README_PATH: &str = "README.md";
const CURRENT_BUNDLE_PATH: &str = "docs/evidence/protocol-763-current-evidence-bundle.md";
const SUPPORTED_SCHEMA: &str = "mc.compat.scenario-manifest.v1";
const SUBSTRING_FALLBACK_MIGRATION: &str = "substring-fallback";
const TYPED_EVENT_READY_MIGRATION: &str = "typed-event-ready";
const SELF_TEST_FLAG: &str = "--self-test";
const MINIMUM_POSITIVE_COUNT: u32 = 1;
const STRING_FIELD_DELIMITER: &str = " = \"";
const ARRAY_START: &str = "[";
const ARRAY_END: &str = "]";
const ROW_START: &str = "{";
const ROW_END: &str = "},";
const SCENARIOS_START: &str = "scenarios = [";
const SCENARIOS_END: &str = "],";
const EXIT_SUCCESS: ExitCode = ExitCode::SUCCESS;
const EXIT_FAILURE: ExitCode = ExitCode::FAILURE;
const LIVE_CAPABILITY_REGISTRY_TOKENS: &[&str] = &[
    "ScenarioLiveCapability",
    "SCENARIO_LIVE_CAPABILITIES",
    "validate_static_live_capabilities",
    "targeted-packet-live-blocker",
    "fixture-bounded-blocker",
    "creative-inventory-action",
    "resource-pack-status",
    "sign-editor-open-update",
];

#[derive(Debug, Clone, PartialEq, Eq)]
struct DryRun {
    check: String,
    wrapper: String,
    receipt_shape_check: bool,
    exclusion_reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ScenarioRow {
    name: String,
    aliases: Vec<String>,
    client_milestones: Vec<String>,
    server_milestones: Vec<String>,
    forbidden_patterns: Vec<String>,
    client_count: u32,
    session_count: u32,
    maintained: bool,
    dry_run: DryRun,
    receipt_expectations: Vec<String>,
    migration_state: String,
    current_bundle_row: String,
    current_bundle_exclusion_reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Manifest {
    schema: String,
    rows: Vec<ScenarioRow>,
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.iter().any(|arg| arg == SELF_TEST_FLAG) {
        return match run_self_tests() {
            Ok(summary) => {
                println!("scenario manifest self-test passed: {summary}");
                EXIT_SUCCESS
            }
            Err(errors) => {
                print_errors(&errors);
                EXIT_FAILURE
            }
        };
    }

    match run_repo_check(Path::new(".")) {
        Ok(summary) => {
            println!("scenario manifest check passed: {summary}");
            EXIT_SUCCESS
        }
        Err(errors) => {
            print_errors(&errors);
            EXIT_FAILURE
        }
    }
}

fn print_errors(errors: &[String]) {
    for error in errors {
        eprintln!("scenario manifest check failed: {error}");
    }
}

fn run_repo_check(root: &Path) -> Result<String, Vec<String>> {
    let manifest_text = read_repo_file(root, MANIFEST_PATH)?;
    let manifest = parse_manifest(&manifest_text)?;
    validate_manifest(&manifest)?;

    let generated = read_repo_file(root, GENERATED_RUST_PATH)?;
    let runner_main = read_repo_file(root, RUNNER_MAIN_PATH)?;
    let runner_scenario_core = read_repo_file(root, RUNNER_SCENARIO_CORE_PATH)?;
    let runner_surface = combined_runner_surface(&runner_main, &runner_scenario_core);
    let flake = read_repo_file(root, FLAKE_PATH)?;
    let readme = read_repo_file(root, README_PATH)?;
    let current_bundle = read_repo_file(root, CURRENT_BUNDLE_PATH)?;

    let mut errors = Vec::new();
    errors.extend(validate_generated_tables(&manifest.rows, &generated));
    errors.extend(validate_runner_surfaces(&manifest.rows, &runner_surface));
    errors.extend(validate_live_capability_registry_surface(
        &runner_scenario_core,
    ));
    errors.extend(validate_flake_surfaces(&manifest.rows, &flake));
    errors.extend(validate_readme_surfaces(&manifest.rows, &readme));
    errors.extend(validate_current_bundle_surfaces(
        &manifest.rows,
        &current_bundle,
    ));

    if errors.is_empty() {
        Ok(format!("{} rows validated", manifest.rows.len()))
    } else {
        Err(errors)
    }
}

fn read_repo_file(root: &Path, relative: &str) -> Result<String, Vec<String>> {
    let path = root.join(relative);
    fs::read_to_string(&path).map_err(|err| vec![format!("{}: {err}", path.display())])
}

fn parse_manifest(text: &str) -> Result<Manifest, Vec<String>> {
    let schema = parse_top_level_string(text, "schema")?;
    let rows = parse_scenario_rows(text)?;
    Ok(Manifest { schema, rows })
}

fn parse_top_level_string(text: &str, field: &str) -> Result<String, Vec<String>> {
    for raw_line in text.lines() {
        let line = raw_line.trim();
        if line.starts_with(&format!("{field}{STRING_FIELD_DELIMITER}")) {
            return parse_string_value(line, field).map_err(|err| vec![err]);
        }
    }
    Err(vec![format!("missing top-level string field {field}")])
}

fn parse_scenario_rows(text: &str) -> Result<Vec<ScenarioRow>, Vec<String>> {
    let mut rows = Vec::new();
    let mut errors = Vec::new();
    let mut in_scenarios = false;
    let mut in_row = false;
    let mut block = Vec::new();

    for raw_line in text.lines() {
        let line = raw_line.trim();
        if !in_scenarios {
            if line == SCENARIOS_START {
                in_scenarios = true;
            }
            continue;
        }
        if !in_row && line == SCENARIOS_END {
            break;
        }
        if line == ROW_START && !in_row {
            in_row = true;
            block.clear();
            continue;
        }
        if in_row {
            if line == ROW_END {
                match parse_scenario_row(&block) {
                    Ok(row) => rows.push(row),
                    Err(mut row_errors) => errors.append(&mut row_errors),
                }
                in_row = false;
                block.clear();
                continue;
            }
            block.push(line.to_string());
        }
    }

    if in_row {
        errors.push("unterminated scenario row".to_string());
    }
    if rows.is_empty() {
        errors.push("manifest has no scenario rows".to_string());
    }
    if errors.is_empty() {
        Ok(rows)
    } else {
        Err(errors)
    }
}

fn parse_scenario_row(lines: &[String]) -> Result<ScenarioRow, Vec<String>> {
    let mut errors = Vec::new();

    let name = collect_string(lines, "name", &mut errors);
    let aliases = collect_array(lines, "aliases", &mut errors);
    let client_milestones = collect_array(lines, "client_milestones", &mut errors);
    let server_milestones = collect_array(lines, "server_milestones", &mut errors);
    let forbidden_patterns = collect_array(lines, "forbidden_patterns", &mut errors);
    let client_count = collect_number(lines, "client_count", &mut errors);
    let session_count = collect_number(lines, "session_count", &mut errors);
    let maintained = collect_bool(lines, "maintained", &mut errors);
    let dry_run = collect_dry_run(lines, &mut errors);
    let receipt_expectations = collect_array(lines, "receipt_expectations", &mut errors);
    let migration_state = collect_string(lines, "migration_state", &mut errors);
    let current_bundle_row = collect_string(lines, "current_bundle_row", &mut errors);
    let current_bundle_exclusion_reason =
        collect_string(lines, "current_bundle_exclusion_reason", &mut errors);

    if errors.is_empty() {
        Ok(ScenarioRow {
            name,
            aliases,
            client_milestones,
            server_milestones,
            forbidden_patterns,
            client_count,
            session_count,
            maintained,
            dry_run,
            receipt_expectations,
            migration_state,
            current_bundle_row,
            current_bundle_exclusion_reason,
        })
    } else {
        Err(errors)
    }
}

fn collect_string(lines: &[String], field: &str, errors: &mut Vec<String>) -> String {
    match find_field_line(lines, field).and_then(|line| parse_string_value(line, field).ok()) {
        Some(value) => value,
        None => {
            errors.push(format!("missing or invalid string field {field}"));
            String::new()
        }
    }
}

fn collect_array(lines: &[String], field: &str, errors: &mut Vec<String>) -> Vec<String> {
    match find_field_line(lines, field).and_then(|line| parse_string_array(line, field).ok()) {
        Some(value) => value,
        None => {
            errors.push(format!("missing or invalid array field {field}"));
            Vec::new()
        }
    }
}

fn collect_number(lines: &[String], field: &str, errors: &mut Vec<String>) -> u32 {
    match find_field_line(lines, field).and_then(|line| parse_u32_value(line, field).ok()) {
        Some(value) => value,
        None => {
            errors.push(format!("missing or invalid number field {field}"));
            u32::MIN
        }
    }
}

fn collect_bool(lines: &[String], field: &str, errors: &mut Vec<String>) -> bool {
    match find_field_line(lines, field).and_then(|line| parse_bool_value(line, field).ok()) {
        Some(value) => value,
        None => {
            errors.push(format!("missing or invalid bool field {field}"));
            false
        }
    }
}

fn collect_dry_run(lines: &[String], errors: &mut Vec<String>) -> DryRun {
    let line = match find_field_line(lines, "dry_run") {
        Some(line) => line,
        None => {
            errors.push("missing dry_run record".to_string());
            return empty_dry_run();
        }
    };
    let check = parse_inline_record_string(line, "check").unwrap_or_else(|err| {
        errors.push(err);
        String::new()
    });
    let wrapper = parse_inline_record_string(line, "wrapper").unwrap_or_else(|err| {
        errors.push(err);
        String::new()
    });
    let receipt_shape_check =
        parse_inline_record_bool(line, "receipt_shape_check").unwrap_or_else(|err| {
            errors.push(err);
            false
        });
    let exclusion_reason =
        parse_inline_record_string(line, "exclusion_reason").unwrap_or_else(|err| {
            errors.push(err);
            String::new()
        });
    DryRun {
        check,
        wrapper,
        receipt_shape_check,
        exclusion_reason,
    }
}

fn empty_dry_run() -> DryRun {
    DryRun {
        check: String::new(),
        wrapper: String::new(),
        receipt_shape_check: false,
        exclusion_reason: String::new(),
    }
}

fn find_field_line<'a>(lines: &'a [String], field: &str) -> Option<&'a str> {
    let prefix = format!("{field} =");
    lines
        .iter()
        .map(String::as_str)
        .find(|line| line.starts_with(&prefix))
}

fn parse_string_value(line: &str, field: &str) -> Result<String, String> {
    let prefix = format!("{field}{STRING_FIELD_DELIMITER}");
    let value = line
        .strip_prefix(&prefix)
        .ok_or_else(|| format!("{field}: expected string assignment"))?;
    let value = value
        .split('"')
        .next()
        .ok_or_else(|| format!("{field}: unterminated string"))?;
    Ok(value.to_string())
}

fn parse_string_array(line: &str, field: &str) -> Result<Vec<String>, String> {
    let prefix = format!("{field} = ");
    let rest = line
        .strip_prefix(&prefix)
        .ok_or_else(|| format!("{field}: expected array assignment"))?
        .trim_end_matches(',')
        .trim();
    let inner = rest
        .strip_prefix(ARRAY_START)
        .and_then(|value| value.strip_suffix(ARRAY_END))
        .ok_or_else(|| format!("{field}: expected one-line array"))?;
    if inner.trim().is_empty() {
        return Ok(Vec::new());
    }
    let mut values = Vec::new();
    for part in inner.split(',') {
        let value = part.trim();
        let value = value
            .strip_prefix('"')
            .and_then(|value| value.strip_suffix('"'))
            .ok_or_else(|| format!("{field}: invalid string array item {value}"))?;
        values.push(value.to_string());
    }
    Ok(values)
}

fn parse_u32_value(line: &str, field: &str) -> Result<u32, String> {
    let prefix = format!("{field} = ");
    let value = line
        .strip_prefix(&prefix)
        .ok_or_else(|| format!("{field}: expected number assignment"))?
        .trim_end_matches(',')
        .trim();
    value
        .parse::<u32>()
        .map_err(|err| format!("{field}: invalid number {value}: {err}"))
}

fn parse_bool_value(line: &str, field: &str) -> Result<bool, String> {
    let prefix = format!("{field} = ");
    let value = line
        .strip_prefix(&prefix)
        .ok_or_else(|| format!("{field}: expected bool assignment"))?
        .trim_end_matches(',')
        .trim();
    value
        .parse::<bool>()
        .map_err(|err| format!("{field}: invalid bool {value}: {err}"))
}

fn parse_inline_record_string(line: &str, field: &str) -> Result<String, String> {
    let needle = format!("{field}{STRING_FIELD_DELIMITER}");
    let rest = line
        .split(&needle)
        .nth(MINIMUM_POSITIVE_COUNT as usize)
        .ok_or_else(|| format!("dry_run.{field}: missing string field"))?;
    let value = rest
        .split('"')
        .next()
        .ok_or_else(|| format!("dry_run.{field}: unterminated string"))?;
    Ok(value.to_string())
}

fn parse_inline_record_bool(line: &str, field: &str) -> Result<bool, String> {
    let needle = format!("{field} = ");
    let rest = line
        .split(&needle)
        .nth(MINIMUM_POSITIVE_COUNT as usize)
        .ok_or_else(|| format!("dry_run.{field}: missing bool field"))?;
    let value = rest
        .split([',', '}'])
        .next()
        .ok_or_else(|| format!("dry_run.{field}: invalid bool field"))?
        .trim();
    value
        .parse::<bool>()
        .map_err(|err| format!("dry_run.{field}: invalid bool {value}: {err}"))
}

fn validate_manifest(manifest: &Manifest) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();
    if manifest.schema != SUPPORTED_SCHEMA {
        errors.push(format!("unsupported schema {}", manifest.schema));
    }
    let mut names = BTreeSet::new();
    let mut aliases = BTreeMap::<String, String>::new();
    for row in &manifest.rows {
        validate_row(row, &mut errors);
        if !names.insert(row.name.clone()) {
            errors.push(format!("duplicate scenario name {}", row.name));
        }
        for alias in &row.aliases {
            if let Some(existing) = aliases.insert(alias.clone(), row.name.clone()) {
                errors.push(format!(
                    "duplicate alias {alias} shared by {existing} and {}",
                    row.name
                ));
            }
        }
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn validate_row(row: &ScenarioRow, errors: &mut Vec<String>) {
    if row.name.is_empty() {
        errors.push("scenario row has empty name".to_string());
    }
    if row.aliases.is_empty() || !row.aliases.iter().any(|alias| alias == &row.name) {
        errors.push(format!("{}: aliases must include canonical name", row.name));
    }
    if row.client_milestones.is_empty() {
        errors.push(format!("{}: client_milestones must be nonempty", row.name));
    }
    if row.forbidden_patterns.is_empty() {
        errors.push(format!("{}: forbidden_patterns must be nonempty", row.name));
    }
    if row.client_count < MINIMUM_POSITIVE_COUNT {
        errors.push(format!("{}: client_count must be positive", row.name));
    }
    if row.session_count < MINIMUM_POSITIVE_COUNT {
        errors.push(format!("{}: session_count must be positive", row.name));
    }
    if row.receipt_expectations.is_empty() {
        errors.push(format!(
            "{}: receipt_expectations must be nonempty",
            row.name
        ));
    }
    if !is_supported_migration_state(&row.migration_state) {
        errors.push(format!(
            "{}: unsupported migration_state {}",
            row.name, row.migration_state
        ));
    }
    if row.maintained && row.dry_run.check.is_empty() && row.dry_run.exclusion_reason.is_empty() {
        errors.push(format!(
            "{}: maintained row needs dry-run check or exclusion reason",
            row.name
        ));
    }
    if !row.dry_run.check.is_empty() && row.dry_run.wrapper.is_empty() {
        errors.push(format!("{}: dry-run wrapper metadata missing", row.name));
    }
    if row.dry_run.receipt_shape_check && row.dry_run.check.is_empty() {
        errors.push(format!(
            "{}: receipt_shape_check requires dry-run check",
            row.name
        ));
    }
    if row.current_bundle_row.is_empty() && row.current_bundle_exclusion_reason.is_empty() {
        errors.push(format!(
            "{}: current bundle row or exclusion reason required",
            row.name
        ));
    }
}

fn is_supported_migration_state(value: &str) -> bool {
    value == SUBSTRING_FALLBACK_MIGRATION || value == TYPED_EVENT_READY_MIGRATION
}

fn validate_generated_tables(rows: &[ScenarioRow], generated: &str) -> Vec<String> {
    let mut errors = Vec::new();
    for row in rows {
        require_contains(
            &mut errors,
            GENERATED_RUST_PATH,
            generated,
            &format!("name: \"{}\"", row.name),
        );
        require_contains(
            &mut errors,
            GENERATED_RUST_PATH,
            generated,
            &format!("migration_state: \"{}\"", row.migration_state),
        );
        for alias in &row.aliases {
            require_contains(
                &mut errors,
                GENERATED_RUST_PATH,
                generated,
                &format!("\"{alias}\""),
            );
        }
        for milestone in row
            .client_milestones
            .iter()
            .chain(row.server_milestones.iter())
        {
            require_contains(
                &mut errors,
                GENERATED_RUST_PATH,
                generated,
                &format!("\"{milestone}\""),
            );
        }
    }
    errors
}

fn combined_runner_surface(main: &str, scenario_core: &str) -> String {
    format!("{main}\n{scenario_core}")
}

fn validate_runner_surfaces(rows: &[ScenarioRow], runner: &str) -> Vec<String> {
    let mut errors = Vec::new();
    for row in rows {
        require_contains(
            &mut errors,
            RUNNER_SURFACE_PATH,
            runner,
            &format!("\"{}\"", row.name),
        );
        for alias in &row.aliases {
            require_contains(
                &mut errors,
                RUNNER_SURFACE_PATH,
                runner,
                &format!("\"{alias}\""),
            );
        }
        for milestone in row
            .client_milestones
            .iter()
            .chain(row.server_milestones.iter())
        {
            require_contains(
                &mut errors,
                RUNNER_SURFACE_PATH,
                runner,
                &format!("\"{milestone}\""),
            );
        }
        for forbidden in &row.forbidden_patterns {
            require_contains(
                &mut errors,
                RUNNER_SURFACE_PATH,
                runner,
                &format!("\"{forbidden}\""),
            );
        }
    }
    errors
}

fn validate_live_capability_registry_surface(scenario_core: &str) -> Vec<String> {
    let mut errors = Vec::new();
    for token in LIVE_CAPABILITY_REGISTRY_TOKENS {
        require_contains(&mut errors, RUNNER_SCENARIO_CORE_PATH, scenario_core, token);
    }
    errors
}

fn validate_flake_surfaces(rows: &[ScenarioRow], flake: &str) -> Vec<String> {
    let mut errors = Vec::new();
    for row in rows {
        if row.dry_run.check.is_empty() {
            continue;
        }
        require_contains(&mut errors, FLAKE_PATH, flake, &row.dry_run.check);
        require_contains(&mut errors, FLAKE_PATH, flake, &row.name);
    }
    errors
}

fn validate_readme_surfaces(rows: &[ScenarioRow], readme: &str) -> Vec<String> {
    let mut errors = Vec::new();
    for row in rows {
        if readme.contains(&row.name) || readme.contains(&row.dry_run.wrapper) {
            continue;
        }
        if row.dry_run.exclusion_reason.is_empty() {
            errors.push(format!(
                "{} missing from README command listings without exclusion",
                row.name
            ));
        }
    }
    errors
}

fn validate_current_bundle_surfaces(rows: &[ScenarioRow], current_bundle: &str) -> Vec<String> {
    let mut errors = Vec::new();
    let lower_bundle = current_bundle.to_ascii_lowercase();
    for row in rows {
        if row.current_bundle_row.is_empty() {
            continue;
        }
        let needle = row.current_bundle_row.to_ascii_lowercase();
        if !lower_bundle.contains(&needle) {
            errors.push(format!(
                "{} current bundle row marker {:?} missing",
                row.name, row.current_bundle_row
            ));
        }
    }
    errors
}

fn require_contains(errors: &mut Vec<String>, path: &str, haystack: &str, needle: &str) {
    if !haystack.contains(needle) {
        errors.push(format!("{path} missing {needle:?}"));
    }
}

fn run_self_tests() -> Result<String, Vec<String>> {
    let cases = [
        ("valid", valid_fixture(), true),
        ("duplicate", duplicate_fixture(), false),
        ("missing_alias", missing_alias_fixture(), false),
        ("missing_milestone", missing_milestone_fixture(), false),
        ("invalid_wrapper", invalid_wrapper_fixture(), false),
        (
            "unsupported_migration",
            unsupported_migration_fixture(),
            false,
        ),
    ];
    let mut errors = Vec::new();
    for (name, fixture, should_pass) in cases {
        let result = parse_manifest(&fixture).and_then(|manifest| validate_manifest(&manifest));
        if result.is_ok() != should_pass {
            errors.push(format!("self-test case {name} expected pass={should_pass}"));
        }
    }
    let manifest = parse_manifest(&valid_fixture()).expect("valid fixture parses");
    let split_surface = combined_runner_surface("", "\"smoke\"\n\"protocol_detected\"\n\"panic\"");
    if !validate_runner_surfaces(&manifest.rows, &split_surface).is_empty() {
        errors.push("self-test case split_runner_surface expected pass=true".to_string());
    }
    if validate_runner_surfaces(&manifest.rows, "\"smoke\"").is_empty() {
        errors.push("self-test case missing_split_runner_surface expected pass=false".to_string());
    }
    let live_registry_surface = LIVE_CAPABILITY_REGISTRY_TOKENS.join("\n");
    if !validate_live_capability_registry_surface(&live_registry_surface).is_empty() {
        errors
            .push("self-test case live_capability_registry_surface expected pass=true".to_string());
    }
    if validate_live_capability_registry_surface("ScenarioLiveCapability").is_empty() {
        errors.push(
            "self-test case missing_live_capability_registry_surface expected pass=false"
                .to_string(),
        );
    }

    if errors.is_empty() {
        Ok("positive and negative fixtures exercised".to_string())
    } else {
        Err(errors)
    }
}

fn fixture_with_row(row: &str) -> String {
    format!("schema = \"{SUPPORTED_SCHEMA}\"\nscenarios = [\n{{\n{row}\n}},\n],\n")
}

fn valid_row() -> &'static str {
    "name = \"smoke\",\naliases = [\"smoke\"],\nclient_milestones = [\"protocol_detected\"],\nserver_milestones = [],\nforbidden_patterns = [\"panic\"],\nclient_count = 1,\nsession_count = 1,\nmaintained = true,\ndry_run = { check = \"mc-compat-dry-run\", wrapper = \"mc-compat-smoke\", receipt_shape_check = true, exclusion_reason = \"\" },\nreceipt_expectations = [\"schema\"],\nmigration_state = \"substring-fallback\",\ncurrent_bundle_row = \"\",\ncurrent_bundle_exclusion_reason = \"harness row\","
}

fn valid_fixture() -> String {
    fixture_with_row(valid_row())
}

fn duplicate_fixture() -> String {
    let row = valid_row();
    format!("schema = \"{SUPPORTED_SCHEMA}\"\nscenarios = [\n{{\n{row}\n}},\n{{\n{row}\n}},\n],\n")
}

fn missing_alias_fixture() -> String {
    fixture_with_row(&valid_row().replace("aliases = [\"smoke\"]", "aliases = []"))
}

fn missing_milestone_fixture() -> String {
    fixture_with_row(&valid_row().replace(
        "client_milestones = [\"protocol_detected\"]",
        "client_milestones = []",
    ))
}

fn invalid_wrapper_fixture() -> String {
    fixture_with_row(&valid_row().replace(
        "dry_run = { check = \"mc-compat-dry-run\", wrapper = \"mc-compat-smoke\", receipt_shape_check = true, exclusion_reason = \"\" }",
        "dry_run = { check = \"\", wrapper = \"\", receipt_shape_check = false, exclusion_reason = \"\" }",
    ))
}

fn unsupported_migration_fixture() -> String {
    fixture_with_row(&valid_row().replace(
        "migration_state = \"substring-fallback\"",
        "migration_state = \"magic\"",
    ))
}
