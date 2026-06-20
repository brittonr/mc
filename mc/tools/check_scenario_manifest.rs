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
const GENERATED_SCENARIO_INDEX_PATH: &str = "docs/evidence/mc-compat-scenario-index.generated.md";
const SUPPORTED_SCHEMA: &str = "mc.compat.scenario-manifest.v1";
const SUBSTRING_FALLBACK_MIGRATION: &str = "substring-fallback";
const TYPED_EVENT_READY_MIGRATION: &str = "typed-event-ready";
const SELF_TEST_FLAG: &str = "--self-test";
const CHECK_GENERATED_SURFACES_FLAG: &str = "--check-generated-surfaces";
const WRITE_GENERATED_SURFACES_FLAG: &str = "--write-generated-surfaces";
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
    "CreativeInventoryLiveContract",
    "CREATIVE_INVENTORY_LIVE_CONTRACT",
    "validate_creative_inventory_live_contract",
    "ResourcePackStatusLocalContract",
    "RESOURCE_PACK_STATUS_LOCAL_CONTRACT",
    "validate_resource_pack_status_local_contract",
    "SignEditorLiveContract",
    "SIGN_EDITOR_LIVE_CONTRACT",
    "validate_sign_editor_live_contract",
    "SCENARIO_LIVE_CAPABILITIES",
    "validate_static_live_capabilities",
    "targeted-packet-live-blocker",
    "fixture-bounded-blocker",
    "creative-inventory-action",
    "creative_slot_mutation_accepted",
    "resource-pack-status",
    "resource_pack_status_declined_observed",
    "sign-editor-open-update",
    "sign_update_accepted_observed",
];
const WAIVER_OWNER_FIELD: &str = "owner=";
const WAIVER_REASON_FIELD: &str = "reason=";
const WAIVER_NON_CLAIM_FIELD: &str = "non_claim=";
const WAIVER_NEXT_ACTION_FIELD: &str = "next_action=";
const REQUIRED_WAIVER_FIELDS: &[&str] = &[
    WAIVER_OWNER_FIELD,
    WAIVER_REASON_FIELD,
    WAIVER_NON_CLAIM_FIELD,
    WAIVER_NEXT_ACTION_FIELD,
];
const STALE_DRY_RUN_EXCLUSION_MARKERS: &[&str] = &[
    "not yet by a dedicated",
    "instead of a dedicated dry-run wrapper",
    "instead of a dry-run wrapper",
];
const TYPED_EVENT_FALLBACK_WAIVER_FIELD: &str = "typed_event_fallback_waiver";
const TYPED_EVENT_COMMON_FORBIDDEN_EVENTS: &[&str] = &[
    "panic",
    "unexpected_eof",
    "protocol_mismatch",
    "decode_error",
];
const TYPED_EVENT_EMPTY_EVENTS: &[&str] = &[];
const SMOKE_TYPED_EVENT_CLIENT_EVENTS: &[&str] = &["protocol_detected"];
const INVENTORY_TYPED_EVENT_CLIENT_EVENTS: &[&str] = &[
    "protocol_detected",
    "join_game",
    "render_tick",
    "team_red",
    "inventory_slot_update",
    "inventory_sword_slot",
    "inventory_wool_slot",
    "inventory_drop_sent",
    "inventory_pickup_seen",
    "inventory_click_sent",
    "inventory_open_container_seen",
    "inventory_container_click_sent",
    "inventory_block_place_sent",
];
const INVENTORY_TYPED_EVENT_SERVER_EVENTS: &[&str] = &[
    "server_username_seen",
    "server_inventory_hotbar_select",
    "server_inventory_drop",
    "server_inventory_pickup",
    "server_inventory_click",
    "server_inventory_open_container",
    "server_inventory_container_click",
    "server_block_place",
];
const TYPED_EVENT_READINESS_FIXTURES: &[TypedEventReadinessFixture<'static>] = &[
    TypedEventReadinessFixture {
        scenario: "smoke",
        client_events: SMOKE_TYPED_EVENT_CLIENT_EVENTS,
        server_events: TYPED_EVENT_EMPTY_EVENTS,
        forbidden_events: TYPED_EVENT_COMMON_FORBIDDEN_EVENTS,
        derivation_rules: TYPED_EVENT_EMPTY_EVENTS,
    },
    TypedEventReadinessFixture {
        scenario: "inventory-interaction",
        client_events: INVENTORY_TYPED_EVENT_CLIENT_EVENTS,
        server_events: INVENTORY_TYPED_EVENT_SERVER_EVENTS,
        forbidden_events: TYPED_EVENT_COMMON_FORBIDDEN_EVENTS,
        derivation_rules: TYPED_EVENT_EMPTY_EVENTS,
    },
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
    typed_event_fallback_waiver: String,
    rows: Vec<ScenarioRow>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TypedEventReadinessFixture<'a> {
    scenario: &'a str,
    client_events: &'a [&'a str],
    server_events: &'a [&'a str],
    forbidden_events: &'a [&'a str],
    derivation_rules: &'a [&'a str],
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct GeneratedSurface {
    path: &'static str,
    content: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct MissingLiveCapability<'a> {
    path: &'static str,
    token: &'a str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LiveCapabilityRegistryEvaluation<'a> {
    missing: Vec<MissingLiveCapability<'a>>,
}

impl LiveCapabilityRegistryEvaluation<'_> {
    fn is_complete(&self) -> bool {
        self.missing.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DryRunCoverageEvaluation {
    covered: usize,
    waived: usize,
    unmaintained: usize,
    issues: Vec<String>,
}

impl DryRunCoverageEvaluation {
    fn is_complete(&self) -> bool {
        self.issues.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TypedEventReadinessEvaluation {
    ready: usize,
    fallback: usize,
    issues: Vec<String>,
}

impl TypedEventReadinessEvaluation {
    fn is_complete(&self) -> bool {
        self.issues.is_empty()
    }
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
    if args.iter().any(|arg| arg == CHECK_GENERATED_SURFACES_FLAG) {
        return match run_generated_surfaces_check(Path::new(".")) {
            Ok(summary) => {
                println!("generated surface check passed: {summary}");
                EXIT_SUCCESS
            }
            Err(errors) => {
                print_errors(&errors);
                EXIT_FAILURE
            }
        };
    }
    if args.iter().any(|arg| arg == WRITE_GENERATED_SURFACES_FLAG) {
        return match run_generated_surfaces_write(Path::new(".")) {
            Ok(summary) => {
                println!("generated surface write passed: {summary}");
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
    let dry_run_coverage = evaluate_dry_run_coverage(&manifest.rows);
    let typed_event_readiness = evaluate_typed_event_readiness(&manifest);

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
        Ok(format!(
            "{} rows validated; dry-run coverage: {} covered, {} waived, {} unmaintained; typed-event readiness: {} ready, {} fallback",
            manifest.rows.len(),
            dry_run_coverage.covered,
            dry_run_coverage.waived,
            dry_run_coverage.unmaintained,
            typed_event_readiness.ready,
            typed_event_readiness.fallback
        ))
    } else {
        Err(errors)
    }
}

fn run_generated_surfaces_check(root: &Path) -> Result<String, Vec<String>> {
    let manifest_text = read_repo_file(root, MANIFEST_PATH)?;
    let manifest = parse_manifest(&manifest_text)?;
    validate_manifest(&manifest)?;
    let surfaces = render_generated_surfaces(&manifest.rows)?;
    let mut errors = Vec::new();
    for surface in &surfaces {
        let checked_in = read_repo_file(root, surface.path)?;
        if checked_in != surface.content {
            errors.push(format!(
                "{} is stale; run {WRITE_GENERATED_SURFACES_FLAG}",
                surface.path
            ));
        }
    }
    if errors.is_empty() {
        Ok(format!("{} generated surfaces current", surfaces.len()))
    } else {
        Err(errors)
    }
}

fn run_generated_surfaces_write(root: &Path) -> Result<String, Vec<String>> {
    let manifest_text = read_repo_file(root, MANIFEST_PATH)?;
    let manifest = parse_manifest(&manifest_text)?;
    validate_manifest(&manifest)?;
    let surfaces = render_generated_surfaces(&manifest.rows)?;
    let mut errors = Vec::new();
    for surface in &surfaces {
        let path = root.join(surface.path);
        if let Some(parent) = path.parent() {
            if let Err(err) = fs::create_dir_all(parent) {
                errors.push(format!("create {}: {err}", parent.display()));
                continue;
            }
        }
        if let Err(err) = fs::write(&path, &surface.content) {
            errors.push(format!("write {}: {err}", path.display()));
        }
    }
    if errors.is_empty() {
        Ok(format!("{} generated surfaces written", surfaces.len()))
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
    let typed_event_fallback_waiver =
        parse_top_level_string(text, TYPED_EVENT_FALLBACK_WAIVER_FIELD)?;
    let rows = parse_scenario_rows(text)?;
    Ok(Manifest {
        schema,
        typed_event_fallback_waiver,
        rows,
    })
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
    let dry_run_coverage = evaluate_dry_run_coverage(&manifest.rows);
    errors.extend(dry_run_coverage.issues);
    let typed_event_readiness = evaluate_typed_event_readiness(manifest);
    errors.extend(typed_event_readiness.issues);
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

fn evaluate_dry_run_coverage(rows: &[ScenarioRow]) -> DryRunCoverageEvaluation {
    let mut evaluation = DryRunCoverageEvaluation {
        covered: usize::MIN,
        waived: usize::MIN,
        unmaintained: usize::MIN,
        issues: Vec::new(),
    };
    for row in rows {
        evaluate_dry_run_coverage_row(row, &mut evaluation);
    }
    evaluation
}

fn evaluate_dry_run_coverage_row(row: &ScenarioRow, evaluation: &mut DryRunCoverageEvaluation) {
    if !row.maintained {
        evaluation.unmaintained += 1;
        return;
    }
    if row.dry_run.wrapper.is_empty() {
        evaluation
            .issues
            .push(format!("{}: dry-run wrapper metadata missing", row.name));
    }
    if !row.dry_run.check.is_empty() {
        evaluation.covered += 1;
        if !row.dry_run.receipt_shape_check {
            evaluation.issues.push(format!(
                "{}: dry-run check must set receipt_shape_check=true",
                row.name
            ));
        }
        if !row.dry_run.exclusion_reason.is_empty() {
            evaluation.issues.push(format!(
                "{}: covered row must not carry waiver metadata",
                row.name
            ));
        }
        return;
    }
    if !row.dry_run.exclusion_reason.is_empty() {
        evaluation.waived += 1;
        if row.dry_run.receipt_shape_check {
            evaluation.issues.push(format!(
                "{}: waived row must set receipt_shape_check=false",
                row.name
            ));
        }
        evaluation
            .issues
            .extend(validate_dry_run_waiver_metadata(row));
        return;
    }
    evaluation.issues.push(format!(
        "{}: maintained row needs dry-run check or waiver metadata",
        row.name
    ));
}

fn validate_dry_run_waiver_metadata(row: &ScenarioRow) -> Vec<String> {
    let mut errors = Vec::new();
    let metadata = row.dry_run.exclusion_reason.trim();
    for marker in STALE_DRY_RUN_EXCLUSION_MARKERS {
        if metadata.contains(marker) {
            errors.push(format!(
                "{}: waiver metadata contains stale exclusion marker {:?}",
                row.name, marker
            ));
        }
    }
    for field in REQUIRED_WAIVER_FIELDS {
        if waiver_field_value(metadata, field).is_none() {
            errors.push(format!(
                "{}: waiver metadata missing nonempty {field}",
                row.name
            ));
        }
    }
    errors
}

fn waiver_field_value<'a>(metadata: &'a str, field: &str) -> Option<&'a str> {
    metadata
        .split(';')
        .map(str::trim)
        .find_map(|part| part.strip_prefix(field).map(str::trim))
        .filter(|value| !value.is_empty())
}

fn evaluate_typed_event_readiness(manifest: &Manifest) -> TypedEventReadinessEvaluation {
    let mut evaluation = TypedEventReadinessEvaluation {
        ready: usize::MIN,
        fallback: usize::MIN,
        issues: Vec::new(),
    };
    for row in &manifest.rows {
        evaluate_typed_event_readiness_row(row, &mut evaluation);
    }
    if evaluation.fallback > usize::MIN {
        evaluation.issues.extend(validate_waiver_metadata_fields(
            TYPED_EVENT_FALLBACK_WAIVER_FIELD,
            &manifest.typed_event_fallback_waiver,
            &[],
        ));
    }
    evaluation
}

fn evaluate_typed_event_readiness_row(
    row: &ScenarioRow,
    evaluation: &mut TypedEventReadinessEvaluation,
) {
    if row.migration_state == TYPED_EVENT_READY_MIGRATION {
        evaluation.ready += 1;
        evaluation
            .issues
            .extend(validate_typed_event_ready_row(row));
        return;
    }
    if row.migration_state == SUBSTRING_FALLBACK_MIGRATION {
        evaluation.fallback += 1;
    }
}

fn validate_typed_event_ready_row(row: &ScenarioRow) -> Vec<String> {
    let Some(fixture) = typed_event_readiness_fixture(&row.name) else {
        return vec![format!(
            "{}: typed-event-ready row lacks readiness fixture",
            row.name
        )];
    };
    let mut errors = Vec::new();
    for milestone in &row.client_milestones {
        if !typed_event_surface_contains(fixture.client_events, fixture.derivation_rules, milestone)
        {
            errors.push(format!(
                "{}: missing client typed-event surface {milestone}",
                row.name
            ));
        }
    }
    for milestone in &row.server_milestones {
        if !typed_event_surface_contains(fixture.server_events, fixture.derivation_rules, milestone)
        {
            errors.push(format!(
                "{}: missing server typed-event surface {milestone}",
                row.name
            ));
        }
    }
    for forbidden in &row.forbidden_patterns {
        if !typed_event_surface_contains(
            fixture.forbidden_events,
            fixture.derivation_rules,
            forbidden,
        ) {
            errors.push(format!(
                "{}: missing forbidden typed-event surface {forbidden}",
                row.name
            ));
        }
    }
    errors
}

fn typed_event_surface_contains(events: &[&str], derivation_rules: &[&str], value: &str) -> bool {
    events.contains(&value) || derivation_rules.contains(&value)
}

fn typed_event_readiness_fixture(
    name: &str,
) -> Option<&'static TypedEventReadinessFixture<'static>> {
    TYPED_EVENT_READINESS_FIXTURES
        .iter()
        .find(|fixture| fixture.scenario == name)
}

fn validate_waiver_metadata_fields(
    label: &str,
    metadata: &str,
    stale_markers: &[&str],
) -> Vec<String> {
    let mut errors = Vec::new();
    let metadata = metadata.trim();
    for marker in stale_markers {
        if metadata.contains(marker) {
            errors.push(format!(
                "{label}: waiver metadata contains stale marker {:?}",
                marker
            ));
        }
    }
    for field in REQUIRED_WAIVER_FIELDS {
        if waiver_field_value(metadata, field).is_none() {
            errors.push(format!("{label}: waiver metadata missing nonempty {field}"));
        }
    }
    errors
}

fn render_generated_surfaces(rows: &[ScenarioRow]) -> Result<Vec<GeneratedSurface>, Vec<String>> {
    let surfaces = vec![
        GeneratedSurface {
            path: GENERATED_RUST_PATH,
            content: render_generated_rust(rows)?,
        },
        GeneratedSurface {
            path: GENERATED_SCENARIO_INDEX_PATH,
            content: render_generated_scenario_index(rows)?,
        },
    ];
    for surface in &surfaces {
        validate_generated_output_path(surface.path)?;
    }
    Ok(surfaces)
}

fn validate_generated_output_path(path: &str) -> Result<(), Vec<String>> {
    let unsafe_path = path.is_empty()
        || path.starts_with('/')
        || path
            .split('/')
            .any(|component| component.is_empty() || component == "." || component == "..");
    if unsafe_path {
        return Err(vec![format!("unsafe generated output path {path:?}")]);
    }
    Ok(())
}

fn render_generated_rust(rows: &[ScenarioRow]) -> Result<String, Vec<String>> {
    let mut output = String::new();
    output
        .push_str("// @generated by tools/check_scenario_manifest.rs --write-generated-surfaces\n");
    output.push_str(
        "// Do not edit by hand; edit config/mc-compat/scenario-manifest.ncl instead.\n\n",
    );
    output.push_str("pub(crate) const ONE_CLIENT: u8 = 1;\n");
    output.push_str("pub(crate) const TWO_CLIENTS: u8 = 2;\n");
    output.push_str("pub(crate) const ONE_SESSION: u8 = 1;\n");
    output.push_str("pub(crate) const TWO_SESSIONS: u8 = 2;\n\n");
    output.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n");
    output.push_str("pub(crate) struct GeneratedScenarioRow {\n");
    output.push_str("    pub(crate) name: &'static str,\n");
    output.push_str("    pub(crate) aliases: &'static [&'static str],\n");
    output.push_str("    pub(crate) client_milestones: &'static [&'static str],\n");
    output.push_str("    pub(crate) server_milestones: &'static [&'static str],\n");
    output.push_str("    pub(crate) forbidden_patterns: &'static [&'static str],\n");
    output.push_str("    pub(crate) client_count: u8,\n");
    output.push_str("    pub(crate) session_count: u8,\n");
    output.push_str("    pub(crate) dry_run_check: &'static str,\n");
    output.push_str("    pub(crate) dry_run_wrapper: &'static str,\n");
    output.push_str("    pub(crate) dry_run_exclusion_reason: &'static str,\n");
    output.push_str("    pub(crate) migration_state: &'static str,\n");
    output.push_str("}\n\n");
    output.push_str("pub(crate) const SCENARIO_MANIFEST_ROWS: &[GeneratedScenarioRow] = &[\n");
    let mut seen_names = BTreeSet::new();
    for row in rows {
        if !seen_names.insert(row.name.as_str()) {
            return Err(vec![format!(
                "duplicate generated scenario name {}",
                row.name
            )]);
        }
        output.push_str("    GeneratedScenarioRow {\n");
        output.push_str(&format!("        name: {},\n", rust_string(&row.name)));
        output.push_str(&format!(
            "        aliases: {},\n",
            rust_string_array(&row.aliases, "        ")
        ));
        output.push_str(&format!(
            "        client_milestones: {},\n",
            rust_string_array(&row.client_milestones, "        ")
        ));
        output.push_str(&format!(
            "        server_milestones: {},\n",
            rust_string_array(&row.server_milestones, "        ")
        ));
        output.push_str(&format!(
            "        forbidden_patterns: {},\n",
            rust_string_array(&row.forbidden_patterns, "        ")
        ));
        output.push_str(&format!(
            "        client_count: {},\n",
            rust_count_expr(row.client_count, "ONE_CLIENT", "TWO_CLIENTS")
        ));
        output.push_str(&format!(
            "        session_count: {},\n",
            rust_count_expr(row.session_count, "ONE_SESSION", "TWO_SESSIONS")
        ));
        output.push_str(&format!(
            "        dry_run_check: {},\n",
            rust_string(&row.dry_run.check)
        ));
        output.push_str(&format!(
            "        dry_run_wrapper: {},\n",
            rust_string(&row.dry_run.wrapper)
        ));
        output.push_str(&format!(
            "        dry_run_exclusion_reason: {},\n",
            rust_string(&row.dry_run.exclusion_reason)
        ));
        output.push_str(&format!(
            "        migration_state: {},\n",
            rust_string(&row.migration_state)
        ));
        output.push_str("    },\n");
    }
    output.push_str("];\n");
    Ok(output)
}

fn render_generated_scenario_index(rows: &[ScenarioRow]) -> Result<String, Vec<String>> {
    let mut output = String::new();
    output.push_str("<!-- BEGIN: mc-compat-generated-scenario-index -->\n");
    output.push_str("<!-- @generated by tools/check_scenario_manifest.rs --write-generated-surfaces; edit config/mc-compat/scenario-manifest.ncl instead. -->\n\n");
    output.push_str("# mc-compat generated scenario index\n\n");
    output.push_str("This bounded index is generated from `config/mc-compat/scenario-manifest.ncl`. It records harness wiring only and does not broaden compatibility claims.\n\n");
    output.push_str(
        "| Scenario | Aliases | Clients | Sessions | Dry-run check | Wrapper | Migration |\n",
    );
    output.push_str("| --- | --- | ---: | ---: | --- | --- | --- |\n");
    let mut seen_names = BTreeSet::new();
    for row in rows {
        if !seen_names.insert(row.name.as_str()) {
            return Err(vec![format!(
                "duplicate generated scenario name {}",
                row.name
            )]);
        }
        output.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} |\n",
            markdown_cell(&row.name),
            markdown_cell(&row.aliases.join(", ")),
            row.client_count,
            row.session_count,
            markdown_cell(&empty_as_dash(&row.dry_run.check)),
            markdown_cell(&empty_as_dash(&row.dry_run.wrapper)),
            markdown_cell(&row.migration_state)
        ));
    }
    output.push_str("\n<!-- END: mc-compat-generated-scenario-index -->\n");
    Ok(output)
}

fn rust_count_expr(count: u32, one: &str, two: &str) -> String {
    match count {
        1 => one.to_string(),
        2 => two.to_string(),
        other => other.to_string(),
    }
}

fn rust_string_array(values: &[String], indent: &str) -> String {
    if values.is_empty() {
        return "&[]".to_string();
    }
    if values.len() <= MINIMUM_POSITIVE_COUNT as usize {
        return format!("&[{}]", rust_string(&values[0]));
    }
    let child_indent = format!("{indent}    ");
    let mut output = String::from("&[\n");
    for value in values {
        output.push_str(&child_indent);
        output.push_str(&rust_string(value));
        output.push_str(",\n");
    }
    output.push_str(indent);
    output.push(']');
    output
}

fn rust_string(value: &str) -> String {
    let mut output = String::with_capacity(value.len() + 2);
    output.push('"');
    for ch in value.chars() {
        match ch {
            '"' => output.push_str("\\\""),
            '\\' => output.push_str("\\\\"),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            ch if ch.is_control() => output.push_str(&format!("\\u{{{:x}}}", ch as u32)),
            ch => output.push(ch),
        }
    }
    output.push('"');
    output
}

fn markdown_cell(value: &str) -> String {
    value.replace('|', "\\|").replace('\n', " ")
}

fn empty_as_dash(value: &str) -> String {
    if value.is_empty() {
        "-".to_string()
    } else {
        value.to_string()
    }
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
    live_capability_registry_diagnostics(&evaluate_live_capability_registry_surface(
        scenario_core,
        LIVE_CAPABILITY_REGISTRY_TOKENS,
    ))
}

fn evaluate_live_capability_registry_surface<'a>(
    scenario_core: &str,
    expected_tokens: &'a [&'a str],
) -> LiveCapabilityRegistryEvaluation<'a> {
    let missing = expected_tokens
        .iter()
        .copied()
        .filter(|token| !scenario_core.contains(token))
        .map(|token| MissingLiveCapability {
            path: RUNNER_SCENARIO_CORE_PATH,
            token,
        })
        .collect();
    LiveCapabilityRegistryEvaluation { missing }
}

fn live_capability_registry_diagnostics(
    evaluation: &LiveCapabilityRegistryEvaluation<'_>,
) -> Vec<String> {
    evaluation
        .missing
        .iter()
        .map(|missing| format!("{} missing {:?}", missing.path, missing.token))
        .collect()
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
        ("waiver_backed", waiver_fixture(), true),
        ("typed_event_ready", typed_event_ready_fixture(), true),
        ("duplicate", duplicate_fixture(), false),
        ("missing_alias", missing_alias_fixture(), false),
        ("missing_milestone", missing_milestone_fixture(), false),
        ("invalid_wrapper", invalid_wrapper_fixture(), false),
        (
            "missing_waiver_wrapper",
            missing_waiver_wrapper_fixture(),
            false,
        ),
        ("empty_waiver", empty_waiver_fixture(), false),
        ("incomplete_waiver", incomplete_waiver_fixture(), false),
        ("stale_waiver", stale_waiver_fixture(), false),
        (
            "missing_typed_event_client",
            missing_typed_event_client_fixture(),
            false,
        ),
        (
            "missing_typed_event_server",
            missing_typed_event_server_fixture(),
            false,
        ),
        (
            "missing_typed_event_forbidden",
            missing_typed_event_forbidden_fixture(),
            false,
        ),
        (
            "missing_typed_event_fallback_waiver",
            missing_typed_event_fallback_waiver_fixture(),
            false,
        ),
        (
            "covered_row_with_waiver",
            covered_row_with_waiver_fixture(),
            false,
        ),
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
    let dry_run_coverage = evaluate_dry_run_coverage(&manifest.rows);
    if !dry_run_coverage.is_complete() {
        errors.push("self-test case valid dry-run coverage expected pass=true".to_string());
    }
    let typed_event_readiness = evaluate_typed_event_readiness(&manifest);
    if !typed_event_readiness.is_complete() {
        errors.push("self-test case valid typed-event fallback expected pass=true".to_string());
    }
    let ready_manifest =
        parse_manifest(&typed_event_ready_fixture()).expect("ready fixture parses");
    let ready_evaluation = evaluate_typed_event_readiness(&ready_manifest);
    if !ready_evaluation.is_complete() {
        errors.push("self-test case typed-event readiness expected pass=true".to_string());
    }
    if let Err(generator_errors) = render_generated_surfaces(&manifest.rows) {
        errors.push(format!(
            "self-test case generated_surfaces expected pass=true: {generator_errors:?}"
        ));
    }
    if validate_generated_output_path("../escape.rs").is_ok() {
        errors.push("self-test case unsafe_generated_output_path expected pass=false".to_string());
    }
    let duplicate_manifest =
        parse_manifest(&duplicate_fixture()).expect("duplicate fixture parses");
    if render_generated_surfaces(&duplicate_manifest.rows).is_ok() {
        errors.push("self-test case duplicate_generated_surface expected pass=false".to_string());
    }
    let split_surface = combined_runner_surface("", "\"smoke\"\n\"protocol_detected\"\n\"panic\"");
    if !validate_runner_surfaces(&manifest.rows, &split_surface).is_empty() {
        errors.push("self-test case split_runner_surface expected pass=true".to_string());
    }
    if validate_runner_surfaces(&manifest.rows, "\"smoke\"").is_empty() {
        errors.push("self-test case missing_split_runner_surface expected pass=false".to_string());
    }
    let live_registry_surface = LIVE_CAPABILITY_REGISTRY_TOKENS.join("\n");
    let complete_registry = evaluate_live_capability_registry_surface(
        &live_registry_surface,
        LIVE_CAPABILITY_REGISTRY_TOKENS,
    );
    if !complete_registry.is_complete()
        || !live_capability_registry_diagnostics(&complete_registry).is_empty()
    {
        errors
            .push("self-test case live_capability_registry_surface expected pass=true".to_string());
    }
    let missing_registry = evaluate_live_capability_registry_surface(
        "ScenarioLiveCapability",
        LIVE_CAPABILITY_REGISTRY_TOKENS,
    );
    if missing_registry.is_complete()
        || live_capability_registry_diagnostics(&missing_registry).is_empty()
    {
        errors.push(
            "self-test case missing_live_capability_registry_surface expected pass=false"
                .to_string(),
        );
    }
    let malformed_registry = evaluate_live_capability_registry_surface(
        "UnknownLiveCapability\nresource-pack-status",
        LIVE_CAPABILITY_REGISTRY_TOKENS,
    );
    let malformed_errors = live_capability_registry_diagnostics(&malformed_registry);
    if malformed_registry.is_complete()
        || !malformed_errors
            .iter()
            .any(|error| error.contains("ResourcePackStatusLocalContract"))
    {
        errors.push(
            "self-test case malformed_live_capability_registry_surface expected fail-closed diagnostic"
                .to_string(),
        );
    }
    let stale_revision_registry = evaluate_live_capability_registry_surface(
        "ScenarioLiveCapability\nlive.revision.status = stale",
        LIVE_CAPABILITY_REGISTRY_TOKENS,
    );
    if stale_revision_registry.is_complete()
        || live_capability_registry_diagnostics(&stale_revision_registry).is_empty()
    {
        errors.push(
            "self-test case stale_revision_live_capability_registry_surface expected fail-closed diagnostic"
                .to_string(),
        );
    }
    let overclaim_registry = evaluate_live_capability_registry_surface(
        "ScenarioLiveCapability\nbroad_minecraft_compatibility = true",
        LIVE_CAPABILITY_REGISTRY_TOKENS,
    );
    if overclaim_registry.is_complete()
        || live_capability_registry_diagnostics(&overclaim_registry).is_empty()
    {
        errors.push(
            "self-test case overclaim_live_capability_registry_surface expected fail-closed diagnostic"
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
    fixture_with_row_and_typed_event_waiver(row, complete_typed_event_fallback_waiver_metadata())
}

fn fixture_with_row_and_typed_event_waiver(row: &str, waiver: &str) -> String {
    format!(
        "schema = \"{SUPPORTED_SCHEMA}\"\ntyped_event_fallback_waiver = \"{waiver}\"\nscenarios = [\n{{\n{row}\n}},\n],\n"
    )
}

fn complete_typed_event_fallback_waiver_metadata() -> &'static str {
    "owner=mc-compat; reason=legacy rows still rely on substring log evidence; non_claim=typed-event migration changes observability only; next_action=migrate rows when typed-event fixtures cover client server and forbidden surfaces"
}

fn valid_row() -> &'static str {
    "name = \"smoke\",\naliases = [\"smoke\"],\nclient_milestones = [\"protocol_detected\"],\nserver_milestones = [],\nforbidden_patterns = [\"panic\"],\nclient_count = 1,\nsession_count = 1,\nmaintained = true,\ndry_run = { check = \"mc-compat-dry-run\", wrapper = \"mc-compat-smoke\", receipt_shape_check = true, exclusion_reason = \"\" },\nreceipt_expectations = [\"schema\"],\nmigration_state = \"substring-fallback\",\ncurrent_bundle_row = \"\",\ncurrent_bundle_exclusion_reason = \"harness row\","
}

fn valid_fixture() -> String {
    fixture_with_row(valid_row())
}

fn duplicate_fixture() -> String {
    let row = valid_row();
    let waiver = complete_typed_event_fallback_waiver_metadata();
    format!(
        "schema = \"{SUPPORTED_SCHEMA}\"\ntyped_event_fallback_waiver = \"{waiver}\"\nscenarios = [\n{{\n{row}\n}},\n{{\n{row}\n}},\n],\n"
    )
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

fn complete_waiver_metadata() -> &'static str {
    "owner=mc-compat; reason=paired reference comparator remains the review source; non_claim=dry-run shape coverage would not promote live parity; next_action=add dedicated wrapper after comparator fixture review"
}

fn waiver_row() -> String {
    valid_row().replace(
        "dry_run = { check = \"mc-compat-dry-run\", wrapper = \"mc-compat-smoke\", receipt_shape_check = true, exclusion_reason = \"\" }",
        &format!(
            "dry_run = {{ check = \"\", wrapper = \"mc-compat-smoke\", receipt_shape_check = false, exclusion_reason = \"{}\" }}",
            complete_waiver_metadata()
        ),
    )
}

fn waiver_fixture() -> String {
    fixture_with_row(&waiver_row())
}

fn missing_waiver_wrapper_fixture() -> String {
    fixture_with_row(&waiver_row().replace("wrapper = \"mc-compat-smoke\"", "wrapper = \"\""))
}

fn empty_waiver_fixture() -> String {
    fixture_with_row(&waiver_row().replace(complete_waiver_metadata(), ""))
}

fn incomplete_waiver_fixture() -> String {
    fixture_with_row(&waiver_row().replace(
        "; next_action=add dedicated wrapper after comparator fixture review",
        "",
    ))
}

fn stale_waiver_fixture() -> String {
    fixture_with_row(&waiver_row().replace(
        complete_waiver_metadata(),
        "owner=mc-compat; reason=covered by historical live receipt and not yet by a dedicated flake dry-run wrapper; non_claim=dry-run shape coverage would not promote live parity; next_action=add dedicated wrapper after comparator fixture review",
    ))
}

fn covered_row_with_waiver_fixture() -> String {
    fixture_with_row(&valid_row().replace(
        "exclusion_reason = \"\"",
        &format!("exclusion_reason = \"{}\"", complete_waiver_metadata()),
    ))
}

fn typed_event_ready_row() -> String {
    valid_row().replace(
        "migration_state = \"substring-fallback\"",
        "migration_state = \"typed-event-ready\"",
    )
}

fn typed_event_ready_fixture() -> String {
    fixture_with_row(&typed_event_ready_row())
}

fn missing_typed_event_client_fixture() -> String {
    fixture_with_row(&typed_event_ready_row().replace(
        "client_milestones = [\"protocol_detected\"]",
        "client_milestones = [\"missing_client_event\"]",
    ))
}

fn missing_typed_event_server_fixture() -> String {
    fixture_with_row(&typed_event_ready_row().replace(
        "server_milestones = []",
        "server_milestones = [\"missing_server_event\"]",
    ))
}

fn missing_typed_event_forbidden_fixture() -> String {
    fixture_with_row(&typed_event_ready_row().replace(
        "forbidden_patterns = [\"panic\"]",
        "forbidden_patterns = [\"unmapped_forbidden_event\"]",
    ))
}

fn missing_typed_event_fallback_waiver_fixture() -> String {
    fixture_with_row_and_typed_event_waiver(valid_row(), "")
}

fn unsupported_migration_fixture() -> String {
    fixture_with_row(&valid_row().replace(
        "migration_state = \"substring-fallback\"",
        "migration_state = \"magic\"",
    ))
}
