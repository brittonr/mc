use std::env;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

const SELF_TEST_FLAG: &str = "--self-test";
const RECEIPT_FLAG: &str = "--receipt";
const ARGUMENT_START_INDEX: usize = 1;
const FLAG_VALUE_OFFSET: usize = 1;
const FLAG_VALUE_STRIDE: usize = 2;
const SUCCESS: ExitCode = ExitCode::SUCCESS;
const FAILURE: ExitCode = ExitCode::FAILURE;

const SHAPE_OBJECT: &str = "paired_reference_dry_run_shape";
const FIELD_SELECTED: &str = "selected";
const FIELD_SCENARIO: &str = "scenario";
const FIELD_REFERENCE_BACKEND: &str = "reference_backend";
const FIELD_VALENCE_BACKEND: &str = "valence_backend";
const FIELD_REFERENCE_REVISION: &str = "reference_revision";
const FIELD_VALENCE_REVISION: &str = "valence_revision";
const FIELD_METRIC_NAMES: &str = "metric_names";
const FIELD_TOLERANCE_FIELDS: &str = "tolerance_fields";
const FIELD_COMPARISON_STATUS: &str = "comparison_status";
const FIELD_LIVE_COMPARATOR_EVIDENCE: &str = "live_comparator_evidence";
const FIELD_CLAIMS_LIVE_PARITY: &str = "claims_live_parity";
const FIELD_CLAIMS_EXACT_VANILLA_PARITY: &str = "claims_exact_vanilla_parity";
const FIELD_NON_CLAIMS: &str = "non_claims";

const COMBAT_REFERENCE_SCENARIO: &str = "vanilla-combat-reference-parity";
const ARMOR_REFERENCE_SCENARIO: &str = "vanilla-combat-armor-reference-parity";
const REFERENCE_BACKEND_LABEL: &str = "paper-reference";
const VALENCE_BACKEND_LABEL: &str = "valence";
const DRY_RUN_REVISION_PLACEHOLDER: &str = "dry-run";
const COMPARISON_STATUS_PLACEHOLDER: &str = "dry-run-shape-not-compared";

const EXPECTED_SCENARIOS: &[&str] = &[COMBAT_REFERENCE_SCENARIO, ARMOR_REFERENCE_SCENARIO];
const REQUIRED_METRIC_NAMES: &[&str] = &[
    "attacker_identity",
    "victim_identity",
    "weapon",
    "armor_state",
    "pre_health",
    "post_health",
    "damage_delta",
    "knockback_metric",
];
const REQUIRED_TOLERANCE_FIELDS: &[&str] = &["damage_tolerance", "knockback_tolerance"];
const REQUIRED_NON_CLAIMS: &[&str] = &[
    "dry_run_shape_only",
    "not_live_paper_valence_evidence",
    "not_comparator_pass",
    "not_exact_mojang_vanilla_parity",
    "not_full_combat_parity",
    "not_public_server_safety",
    "not_production_readiness",
];

#[derive(Debug, Clone, PartialEq, Eq)]
struct NormalizedPairedReferenceShape {
    selected: bool,
    scenario: String,
    reference_backend: String,
    valence_backend: String,
    reference_revision: String,
    valence_revision: String,
    metric_names: Vec<String>,
    tolerance_fields: Vec<String>,
    comparison_status: String,
    live_comparator_evidence: bool,
    claims_live_parity: bool,
    claims_exact_vanilla_parity: bool,
    non_claims: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ShapeDecision {
    passed: bool,
    diagnostics: Vec<String>,
}

fn main() -> ExitCode {
    let args = env::args().collect::<Vec<_>>();
    if args.iter().any(|arg| arg == SELF_TEST_FLAG) {
        return match run_self_tests() {
            Ok(summary) => {
                println!("paired reference dry-run shape self-test passed: {summary}");
                SUCCESS
            }
            Err(errors) => {
                print_errors(&errors);
                FAILURE
            }
        };
    }

    match parse_receipt_args(&args).and_then(run_receipt_checks) {
        Ok(summary) => {
            println!("paired reference dry-run shape check passed: {summary}");
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
        eprintln!("paired reference dry-run shape check failed: {error}");
    }
}

fn parse_receipt_args(args: &[String]) -> Result<Vec<String>, Vec<String>> {
    let mut receipts = Vec::new();
    let mut index = ARGUMENT_START_INDEX;
    while index < args.len() {
        let flag = args[index].as_str();
        let Some(value) = args.get(index + FLAG_VALUE_OFFSET) else {
            return Err(vec![usage()]);
        };
        match flag {
            RECEIPT_FLAG => receipts.push(value.clone()),
            _ => return Err(vec![format!("unknown argument: {flag}"), usage()]),
        }
        index += FLAG_VALUE_STRIDE;
    }

    if receipts.is_empty() {
        Err(vec![usage()])
    } else {
        Ok(receipts)
    }
}

fn usage() -> String {
    format!("usage: check_paired_reference_dry_run_shapes {RECEIPT_FLAG} <receipt> [{RECEIPT_FLAG} <receipt> ...]")
}

fn run_receipt_checks(receipt_paths: Vec<String>) -> Result<String, Vec<String>> {
    let mut errors = Vec::new();
    let mut checked = 0usize;
    for path in receipt_paths {
        let text = match fs::read_to_string(Path::new(&path)) {
            Ok(text) => text,
            Err(error) => {
                errors.push(format!("{path}: {error}"));
                continue;
            }
        };
        match normalize_receipt_shape(&text) {
            Ok(shape) => {
                let decision = validate_paired_reference_shape(&shape);
                if decision.passed {
                    checked += 1;
                } else {
                    errors.extend(
                        decision
                            .diagnostics
                            .into_iter()
                            .map(|diagnostic| format!("{path}: {diagnostic}")),
                    );
                }
            }
            Err(error) => errors.push(format!("{path}: {error}")),
        }
    }

    if errors.is_empty() {
        Ok(format!("{checked} receipt shapes validated"))
    } else {
        Err(errors)
    }
}

fn normalize_receipt_shape(text: &str) -> Result<NormalizedPairedReferenceShape, String> {
    let shape = json_object_slice(text, SHAPE_OBJECT)?;
    Ok(NormalizedPairedReferenceShape {
        selected: json_bool_field(shape, FIELD_SELECTED)?,
        scenario: json_string_field(shape, FIELD_SCENARIO)?,
        reference_backend: json_string_field(shape, FIELD_REFERENCE_BACKEND)?,
        valence_backend: json_string_field(shape, FIELD_VALENCE_BACKEND)?,
        reference_revision: json_string_field(shape, FIELD_REFERENCE_REVISION)?,
        valence_revision: json_string_field(shape, FIELD_VALENCE_REVISION)?,
        metric_names: json_string_array_field(shape, FIELD_METRIC_NAMES)?,
        tolerance_fields: json_string_array_field(shape, FIELD_TOLERANCE_FIELDS)?,
        comparison_status: json_string_field(shape, FIELD_COMPARISON_STATUS)?,
        live_comparator_evidence: json_bool_field(shape, FIELD_LIVE_COMPARATOR_EVIDENCE)?,
        claims_live_parity: json_bool_field(shape, FIELD_CLAIMS_LIVE_PARITY)?,
        claims_exact_vanilla_parity: json_bool_field(shape, FIELD_CLAIMS_EXACT_VANILLA_PARITY)?,
        non_claims: json_string_array_field(shape, FIELD_NON_CLAIMS)?,
    })
}

fn validate_paired_reference_shape(shape: &NormalizedPairedReferenceShape) -> ShapeDecision {
    let mut diagnostics = Vec::new();

    if !shape.selected {
        diagnostics.push("shape_not_selected".to_string());
    }
    if !EXPECTED_SCENARIOS.contains(&shape.scenario.as_str()) {
        diagnostics.push(format!("unexpected_scenario:{}", shape.scenario));
    }
    validate_string_field(
        FIELD_REFERENCE_BACKEND,
        &shape.reference_backend,
        REFERENCE_BACKEND_LABEL,
        &mut diagnostics,
    );
    validate_string_field(
        FIELD_VALENCE_BACKEND,
        &shape.valence_backend,
        VALENCE_BACKEND_LABEL,
        &mut diagnostics,
    );
    validate_string_field(
        FIELD_REFERENCE_REVISION,
        &shape.reference_revision,
        DRY_RUN_REVISION_PLACEHOLDER,
        &mut diagnostics,
    );
    validate_string_field(
        FIELD_VALENCE_REVISION,
        &shape.valence_revision,
        DRY_RUN_REVISION_PLACEHOLDER,
        &mut diagnostics,
    );
    validate_string_field(
        FIELD_COMPARISON_STATUS,
        &shape.comparison_status,
        COMPARISON_STATUS_PLACEHOLDER,
        &mut diagnostics,
    );
    validate_required_values(
        FIELD_METRIC_NAMES,
        &shape.metric_names,
        REQUIRED_METRIC_NAMES,
        &mut diagnostics,
    );
    validate_required_values(
        FIELD_TOLERANCE_FIELDS,
        &shape.tolerance_fields,
        REQUIRED_TOLERANCE_FIELDS,
        &mut diagnostics,
    );
    validate_required_values(
        FIELD_NON_CLAIMS,
        &shape.non_claims,
        REQUIRED_NON_CLAIMS,
        &mut diagnostics,
    );

    if shape.live_comparator_evidence {
        diagnostics.push("overclaim_live_comparator_evidence".to_string());
    }
    if shape.claims_live_parity {
        diagnostics.push("overclaim_live_parity".to_string());
    }
    if shape.claims_exact_vanilla_parity {
        diagnostics.push("overclaim_exact_vanilla_parity".to_string());
    }

    ShapeDecision {
        passed: diagnostics.is_empty(),
        diagnostics,
    }
}

fn validate_string_field(field: &str, actual: &str, expected: &str, diagnostics: &mut Vec<String>) {
    if actual != expected {
        diagnostics.push(format!("wrong_{field}:expected={expected}:actual={actual}"));
    }
}

fn validate_required_values(
    field: &str,
    actual: &[String],
    required: &[&str],
    diagnostics: &mut Vec<String>,
) {
    for value in required {
        if !actual.iter().any(|actual_value| actual_value == value) {
            diagnostics.push(format!("missing_{field}:{value}"));
        }
    }
}

fn json_object_slice<'a>(text: &'a str, key: &str) -> Result<&'a str, String> {
    let value = json_field_value(text, key)?;
    let trimmed = value.trim_start();
    if !trimmed.starts_with('{') {
        return Err(format!("field {key} is not an object"));
    }
    let object_start = text.len() - trimmed.len();
    let mut depth = 0usize;
    for (offset, character) in text[object_start..].char_indices() {
        match character {
            '{' => depth += 1,
            '}' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    let object_end = object_start + offset + character.len_utf8();
                    return Ok(&text[object_start..object_end]);
                }
            }
            _ => {}
        }
    }
    Err(format!("field {key} object is unterminated"))
}

fn json_field_value<'a>(text: &'a str, key: &str) -> Result<&'a str, String> {
    let token = format!("\"{key}\"");
    let Some(key_start) = text.find(&token) else {
        return Err(format!("missing field {key}"));
    };
    let after_key = &text[key_start + token.len()..];
    let Some(colon_offset) = after_key.find(':') else {
        return Err(format!("field {key} is missing ':'"));
    };
    Ok(&after_key[colon_offset + ':'.len_utf8()..])
}

fn json_string_field(text: &str, key: &str) -> Result<String, String> {
    let value = json_field_value(text, key)?.trim_start();
    parse_json_string(value)
        .map(|(parsed, _)| parsed)
        .ok_or_else(|| format!("field {key} is not a string"))
}

fn json_bool_field(text: &str, key: &str) -> Result<bool, String> {
    let value = json_field_value(text, key)?.trim_start();
    if value.starts_with("true") {
        Ok(true)
    } else if value.starts_with("false") {
        Ok(false)
    } else {
        Err(format!("field {key} is not a bool"))
    }
}

fn json_string_array_field(text: &str, key: &str) -> Result<Vec<String>, String> {
    let mut rest = json_field_value(text, key)?.trim_start();
    if !rest.starts_with('[') {
        return Err(format!("field {key} is not an array"));
    }
    rest = &rest['['.len_utf8()..];
    let mut values = Vec::new();
    loop {
        rest = rest.trim_start();
        if rest.starts_with(']') {
            return Ok(values);
        }
        let Some((value, after_value)) = parse_json_string(rest) else {
            return Err(format!("field {key} contains a non-string item"));
        };
        values.push(value);
        rest = after_value.trim_start();
        if rest.starts_with(',') {
            rest = &rest[','.len_utf8()..];
            continue;
        }
        if rest.starts_with(']') {
            return Ok(values);
        }
        return Err(format!(
            "field {key} array is missing comma or closing bracket"
        ));
    }
}

fn parse_json_string(text: &str) -> Option<(String, &str)> {
    let mut chars = text.char_indices();
    let (_, first) = chars.next()?;
    if first != '"' {
        return None;
    }
    let mut value = String::new();
    let mut escaped = false;
    for (offset, character) in chars {
        if escaped {
            value.push(match character {
                'n' => '\n',
                'r' => '\r',
                't' => '\t',
                other => other,
            });
            escaped = false;
            continue;
        }
        match character {
            '\\' => escaped = true,
            '"' => {
                let rest = &text[offset + character.len_utf8()..];
                return Some((value, rest));
            }
            other => value.push(other),
        }
    }
    None
}

fn run_self_tests() -> Result<String, Vec<String>> {
    let mut errors = Vec::new();
    errors.extend(expect_ok(
        "valid combat reference dry-run shape",
        &valid_fixture(COMBAT_REFERENCE_SCENARIO),
    ));
    errors.extend(expect_ok(
        "valid armor reference dry-run shape",
        &valid_fixture(ARMOR_REFERENCE_SCENARIO),
    ));
    errors.extend(expect_error(
        "invalid reference backend field",
        &valid_fixture(COMBAT_REFERENCE_SCENARIO).replace(
            &format!("\"{FIELD_REFERENCE_BACKEND}\": \"{REFERENCE_BACKEND_LABEL}\""),
            &format!("\"{FIELD_REFERENCE_BACKEND}\": null"),
        ),
        FIELD_REFERENCE_BACKEND,
    ));
    errors.extend(expect_error(
        "invalid valence backend field",
        &valid_fixture(COMBAT_REFERENCE_SCENARIO).replace(
            &format!("\"{FIELD_VALENCE_BACKEND}\": \"{VALENCE_BACKEND_LABEL}\""),
            &format!("\"{FIELD_VALENCE_BACKEND}\": null"),
        ),
        FIELD_VALENCE_BACKEND,
    ));
    errors.extend(expect_error(
        "missing tolerance field",
        &valid_fixture(COMBAT_REFERENCE_SCENARIO).replace("\"damage_tolerance\", ", ""),
        "missing_tolerance_fields:damage_tolerance",
    ));
    errors.extend(expect_error(
        "wrong backend labels",
        &valid_fixture(COMBAT_REFERENCE_SCENARIO).replace(REFERENCE_BACKEND_LABEL, "paper"),
        "wrong_reference_backend",
    ));
    errors.extend(expect_error(
        "missing dry-run non-claim",
        &valid_fixture(COMBAT_REFERENCE_SCENARIO).replace("\"not_full_combat_parity\", ", ""),
        "missing_non_claims:not_full_combat_parity",
    ));
    errors.extend(expect_error(
        "overbroad live parity claim",
        &valid_fixture(COMBAT_REFERENCE_SCENARIO).replace(
            &format!("\"{FIELD_CLAIMS_LIVE_PARITY}\": false"),
            &format!("\"{FIELD_CLAIMS_LIVE_PARITY}\": true"),
        ),
        "overclaim_live_parity",
    ));

    if errors.is_empty() {
        Ok("positive and negative dry-run shape fixtures exercised".to_string())
    } else {
        Err(errors)
    }
}

fn expect_ok(name: &str, fixture: &str) -> Vec<String> {
    match normalize_receipt_shape(fixture).map(|shape| validate_paired_reference_shape(&shape)) {
        Ok(decision) if decision.passed => Vec::new(),
        Ok(decision) => vec![format!(
            "{name}: expected ok, got {:?}",
            decision.diagnostics
        )],
        Err(error) => vec![format!("{name}: expected ok, got parse error {error}")],
    }
}

fn expect_error(name: &str, fixture: &str, needle: &str) -> Vec<String> {
    match normalize_receipt_shape(fixture) {
        Ok(shape) => {
            let decision = validate_paired_reference_shape(&shape);
            if !decision.passed
                && decision
                    .diagnostics
                    .iter()
                    .any(|diagnostic| diagnostic.contains(needle))
            {
                Vec::new()
            } else {
                vec![format!(
                    "{name}: expected diagnostic containing {needle:?}, got {:?}",
                    decision.diagnostics
                )]
            }
        }
        Err(error) if error.contains(needle) => Vec::new(),
        Err(error) => vec![format!(
            "{name}: expected error containing {needle:?}, got parse error {error}"
        )],
    }
}

fn valid_fixture(scenario: &str) -> String {
    format!(
        "{{\n\
  \"schema\": \"mc.compat.scenario.receipt.v2\",\n\
  \"paired_reference_dry_run_shape\": {{\n\
  \"selected\": true,\n\
  \"scenario\": \"{scenario}\",\n\
  \"reference_backend\": \"{REFERENCE_BACKEND_LABEL}\",\n\
  \"valence_backend\": \"{VALENCE_BACKEND_LABEL}\",\n\
  \"reference_revision\": \"{DRY_RUN_REVISION_PLACEHOLDER}\",\n\
  \"valence_revision\": \"{DRY_RUN_REVISION_PLACEHOLDER}\",\n\
  \"metric_names\": [\"attacker_identity\", \"victim_identity\", \"weapon\", \"armor_state\", \"pre_health\", \"post_health\", \"damage_delta\", \"knockback_metric\"],\n\
  \"tolerance_fields\": [\"damage_tolerance\", \"knockback_tolerance\"],\n\
  \"comparison_status\": \"{COMPARISON_STATUS_PLACEHOLDER}\",\n\
  \"live_comparator_evidence\": false,\n\
  \"claims_live_parity\": false,\n\
  \"claims_exact_vanilla_parity\": false,\n\
  \"non_claims\": [\"dry_run_shape_only\", \"not_live_paper_valence_evidence\", \"not_comparator_pass\", \"not_exact_mojang_vanilla_parity\", \"not_full_combat_parity\", \"not_public_server_safety\", \"not_production_readiness\"]\n\
}}\n\
}}\n"
    )
}
