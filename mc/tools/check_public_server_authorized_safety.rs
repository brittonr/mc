#!/usr/bin/env -S nix shell "github:nix-community/fenix?rev=092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain" nixpkgs#gcc -c cargo -q -Zscript

use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

const SELF_TEST_FLAG: &str = "--self-test";
const CONTRACT_FLAG: &str = "--contract";
const RECORD_FLAG: &str = "--record";
const KEY_VALUE_SEPARATOR: char = '=';
const SUCCESS: ExitCode = ExitCode::SUCCESS;
const FAILURE: ExitCode = ExitCode::FAILURE;

const CONTRACT_DOC: &str =
    "docs/evidence/protocol-763-public-server-authorized-safety-contract-2026-05-30.md";
const ROW_DOC: &str = "docs/evidence/protocol-763-public-server-authorized-safety-2026-05-30.md";
const ROW_RECEIPT: &str =
    "docs/evidence/protocol-763-public-server-authorized-safety-2026-05-30.receipt.json";
const ROW_RECORD: &str =
    "docs/evidence/protocol-763-public-server-authorized-safety-2026-05-30.record";
const CHECKPOINT_DOC: &str =
    "docs/evidence/protocol-763-public-server-authorized-safety-checkpoint-2026-05-30.md";
const PRODUCTION_MATRIX_DOC: &str =
    "docs/evidence/protocol-763-production-network-safety-matrix-2026-05-28.md";
const CURRENT_BUNDLE_DOC: &str = "docs/evidence/protocol-763-current-evidence-bundle.md";
const ACCEPTANCE_MATRIX_DOC: &str = "docs/evidence/protocol-763-acceptance-matrix.md";

const EXPECTED_ROW_NAME: &str = "public-server-authorized-safety";
const EXPECTED_STATUS: &str = "covered_authorized_fixture_only";
const EXPECTED_TARGET_OWNER: &str = "review-fixture-owner";
const EXPECTED_AUTHORIZATION_ARTIFACT: &str = CHECKPOINT_DOC;
const EXPECTED_TARGET_SCOPE: &str = "authorized-non-loopback-fixture";
const EXPECTED_CLIENT_COUNT: &str = "1";
const EXPECTED_DURATION_SECS: &str = "30";
const EXPECTED_TRAFFIC_LIMITS: &str =
    "client_count<=1,duration_secs<=30,status_probe_only,live_traffic_enabled=false";
const EXPECTED_ABORT_CRITERIA: &str = "missing_authorization_or_bound_violation";
const EXPECTED_REDACTION_POLICY: &str = "no_secrets_no_raw_public_address";
const EXPECTED_CHECKPOINT_DECISION: &str = "approved_for_deterministic_fixture_only";

const REQUIRED_TELEMETRY: &[&str] = &[
    "target_owner",
    "authorization_artifact",
    "target_scope",
    "client_count",
    "duration_secs",
    "traffic_limits",
    "abort_criteria",
    "redaction_policy",
    "checkpoint_decision",
];

const FORBIDDEN_TRUE_CLAIMS: &[&str] = &[
    "claims.live_public_server_safety",
    "claims.third_party_target_safety_without_authorization",
    "claims.production_readiness",
    "claims.adversarial_safety",
    "claims.wan_tolerance",
    "claims.load_safety_beyond_configured_bounds",
    "claims.unbounded_public_testing",
];

const CONTRACT_TOKENS: &[&str] = &[
    EXPECTED_ROW_NAME,
    EXPECTED_STATUS,
    EXPECTED_TARGET_OWNER,
    EXPECTED_TARGET_SCOPE,
    EXPECTED_AUTHORIZATION_ARTIFACT,
    "client_count=1",
    "duration_secs=30",
    "status_probe_only",
    "live_traffic_enabled=false",
    EXPECTED_ABORT_CRITERIA,
    EXPECTED_REDACTION_POLICY,
    EXPECTED_CHECKPOINT_DECISION,
    "missing_owner",
    "missing_written_authorization",
    "missing_bounds",
    "missing_telemetry",
    "missing_checkpoint",
    "secret_leak",
    "production_readiness_overclaim",
    "third-party target safety without authorization remains a non-claim",
];

const ROW_RECEIPT_TOKENS: &[&str] = &[
    "mc.compat.scenario.receipt.v2",
    "\"mode\": \"dry-run\"",
    "\"dry_run\": true",
    "\"public_server_authorized_safety\"",
    "\"selected\": true",
    "\"target_owner\": \"review-fixture-owner\"",
    "\"authorization_artifact\": \"docs/evidence/protocol-763-public-server-authorized-safety-checkpoint-2026-05-30.md\"",
    "\"target_scope\": \"authorized-non-loopback-fixture\"",
    "\"client_count\": 1",
    "\"duration_secs\": 30",
    "\"status_probe_only\"",
    "\"live_traffic_enabled\": false",
    "\"checkpoint_decision\": \"approved_for_deterministic_fixture_only\"",
    "\"claims_authorized_public_envelope\": true",
    "\"claims_live_public_server_safety\": false",
    "\"claims_third_party_target_safety_without_authorization\": false",
    "\"claims_production_readiness\": false",
    "\"claims_wan_tolerance\": false",
];

const MATRIX_TOKENS: &[&str] = &[
    "| public-server safety | covered_authorized_fixture_only |",
    EXPECTED_TARGET_OWNER,
    EXPECTED_TARGET_SCOPE,
    EXPECTED_AUTHORIZATION_ARTIFACT,
    "client_count=1",
    "duration_secs=30",
    "status_probe_only",
    "live_traffic_enabled=false",
    ROW_RECEIPT,
    CHECKPOINT_DOC,
    "No live public-server safety",
    "No third-party target safety without authorization",
    "No production readiness",
    "No unbounded public testing",
];

const CURRENT_BUNDLE_TOKENS: &[&str] = &[
    "Public server authorized safety checkpoint",
    ROW_DOC,
    ROW_RECEIPT,
    CHECKPOINT_DOC,
    "tools/check_public_server_authorized_safety.rs",
    EXPECTED_STATUS,
    "live public-server safety remains a non-claim",
];

const ACCEPTANCE_MATRIX_TOKENS: &[&str] = &[
    "Production load / multiplayer scale",
    "public-server authorized fixture",
    "covered_authorized_fixture_only",
    "No live public-server safety",
];

const ROW_DOC_TOKENS: &[&str] = &[
    EXPECTED_ROW_NAME,
    ROW_RECEIPT,
    CHECKPOINT_DOC,
    "target_owner=review-fixture-owner",
    "target_scope=authorized-non-loopback-fixture",
    "client_count=1",
    "duration_secs=30",
    "traffic_limits=client_count<=1,duration_secs<=30,status_probe_only,live_traffic_enabled=false",
    "redaction_policy=no_secrets_no_raw_public_address",
    "promote only the authorized deterministic fixture envelope",
    "claims.live_public_server_safety=false",
    "claims.production_readiness=false",
];

const CHECKPOINT_TOKENS: &[&str] = &[
    "## Question",
    "## Inspected evidence",
    "## Decision",
    "## Owner",
    "## Next action",
    EXPECTED_CHECKPOINT_DECISION,
    "fixture only",
    "no live public-server safety claim",
];

#[derive(Debug, Clone, PartialEq, Eq)]
struct Args {
    self_test: bool,
    contract_path: String,
    record_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PublicServerRecord {
    values: BTreeMap<String, String>,
}

fn main() -> ExitCode {
    let args = match parse_args(env::args().skip(1)) {
        Ok(parsed) => parsed,
        Err(errors) => return exit_with_errors(&errors),
    };

    if args.self_test {
        return match run_self_tests() {
            Ok(summary) => {
                println!("public-server authorized safety self-test passed: {summary}");
                SUCCESS
            }
            Err(errors) => exit_with_errors(&errors),
        };
    }

    match run_repo_check(&args) {
        Ok(summary) => {
            println!("public-server authorized safety check passed: {summary}");
            SUCCESS
        }
        Err(errors) => exit_with_errors(&errors),
    }
}

fn parse_args<I>(args: I) -> Result<Args, Vec<String>>
where
    I: IntoIterator<Item = String>,
{
    let mut parsed = Args {
        self_test: false,
        contract_path: CONTRACT_DOC.to_string(),
        record_path: ROW_RECORD.to_string(),
    };
    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        if arg == SELF_TEST_FLAG {
            parsed.self_test = true;
        } else if arg == CONTRACT_FLAG {
            parsed.contract_path = next_value(&mut iter, CONTRACT_FLAG)?;
        } else if arg == RECORD_FLAG {
            parsed.record_path = next_value(&mut iter, RECORD_FLAG)?;
        } else {
            return Err(vec![format!("unknown argument: {arg}")]);
        }
    }
    Ok(parsed)
}

fn next_value<I>(iter: &mut I, flag: &str) -> Result<String, Vec<String>>
where
    I: Iterator<Item = String>,
{
    match iter.next() {
        Some(value) => Ok(value),
        None => Err(vec![format!("missing value for {flag}")]),
    }
}

fn run_repo_check(args: &Args) -> Result<String, Vec<String>> {
    let contract_text = read_text(&args.contract_path)?;
    let row_receipt_text = read_text(ROW_RECEIPT)?;
    let matrix_text = read_text(PRODUCTION_MATRIX_DOC)?;
    let bundle_text = read_text(CURRENT_BUNDLE_DOC)?;
    let acceptance_text = read_text(ACCEPTANCE_MATRIX_DOC)?;
    let row_doc_text = read_text(ROW_DOC)?;
    let checkpoint_text = read_text(CHECKPOINT_DOC)?;
    let record_text = read_text(&args.record_path)?;
    let record = parse_record(&record_text)?;

    let mut errors = validate_text_tokens("contract", &contract_text, CONTRACT_TOKENS);
    errors.extend(validate_text_tokens(
        "row receipt",
        &row_receipt_text,
        ROW_RECEIPT_TOKENS,
    ));
    errors.extend(validate_text_tokens(
        "production matrix",
        &matrix_text,
        MATRIX_TOKENS,
    ));
    errors.extend(validate_text_tokens(
        "current bundle",
        &bundle_text,
        CURRENT_BUNDLE_TOKENS,
    ));
    errors.extend(validate_text_tokens(
        "acceptance matrix",
        &acceptance_text,
        ACCEPTANCE_MATRIX_TOKENS,
    ));
    errors.extend(validate_text_tokens(
        "row evidence doc",
        &row_doc_text,
        ROW_DOC_TOKENS,
    ));
    errors.extend(validate_text_tokens(
        "checkpoint",
        &checkpoint_text,
        CHECKPOINT_TOKENS,
    ));
    errors.extend(validate_public_server_record(&record));

    if errors.is_empty() {
        Ok(format!(
            "contract={} record={} receipt={} matrix={} bundle={}",
            args.contract_path,
            args.record_path,
            ROW_RECEIPT,
            PRODUCTION_MATRIX_DOC,
            CURRENT_BUNDLE_DOC
        ))
    } else {
        Err(errors)
    }
}

fn read_text(path: &str) -> Result<String, Vec<String>> {
    fs::read_to_string(Path::new(path)).map_err(|err| vec![format!("{path}: {err}")])
}

fn parse_record(text: &str) -> Result<PublicServerRecord, Vec<String>> {
    let mut values = BTreeMap::new();
    let mut errors = Vec::new();
    for (line_index, raw_line) in text.lines().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        match line.split_once(KEY_VALUE_SEPARATOR) {
            Some((key, value)) => insert_record_value(&mut values, &mut errors, key, value),
            None => errors.push(format!(
                "line {} missing key=value separator",
                line_index + 1
            )),
        }
    }
    if errors.is_empty() {
        Ok(PublicServerRecord { values })
    } else {
        Err(errors)
    }
}

fn insert_record_value(
    values: &mut BTreeMap<String, String>,
    errors: &mut Vec<String>,
    key: &str,
    value: &str,
) {
    let normalized_key = key.trim().to_string();
    let normalized_value = value.trim().to_string();
    if normalized_key.is_empty() {
        errors.push("empty key".to_string());
        return;
    }
    if values
        .insert(normalized_key.clone(), normalized_value)
        .is_some()
    {
        errors.push(format!("duplicate key: {normalized_key}"));
    }
}

fn validate_text_tokens(label: &str, text: &str, tokens: &[&str]) -> Vec<String> {
    let mut errors = Vec::new();
    for token in tokens {
        if !text.contains(token) {
            errors.push(format!("{label} missing token: {token}"));
        }
    }
    errors
}

fn validate_public_server_record(record: &PublicServerRecord) -> Vec<String> {
    let mut errors = Vec::new();
    require_text(
        &mut errors,
        record,
        "row.name",
        EXPECTED_ROW_NAME,
        "wrong_row",
    );
    require_text(
        &mut errors,
        record,
        "row.status",
        EXPECTED_STATUS,
        "wrong_status",
    );
    require_text(
        &mut errors,
        record,
        "target.owner",
        EXPECTED_TARGET_OWNER,
        "missing_owner",
    );
    require_text(
        &mut errors,
        record,
        "target.authorization_artifact",
        EXPECTED_AUTHORIZATION_ARTIFACT,
        "missing_written_authorization",
    );
    require_text(
        &mut errors,
        record,
        "target.scope",
        EXPECTED_TARGET_SCOPE,
        "missing_target_scope",
    );
    require_text(
        &mut errors,
        record,
        "bounds.client_count",
        EXPECTED_CLIENT_COUNT,
        "missing_bounds",
    );
    require_text(
        &mut errors,
        record,
        "bounds.duration_secs",
        EXPECTED_DURATION_SECS,
        "missing_bounds",
    );
    require_text(
        &mut errors,
        record,
        "bounds.traffic_limits",
        EXPECTED_TRAFFIC_LIMITS,
        "missing_bounds",
    );
    require_text(
        &mut errors,
        record,
        "abort.criteria",
        EXPECTED_ABORT_CRITERIA,
        "missing_abort_criteria",
    );
    require_text(
        &mut errors,
        record,
        "redaction.policy",
        EXPECTED_REDACTION_POLICY,
        "missing_redaction_policy",
    );
    require_text(
        &mut errors,
        record,
        "checkpoint.decision",
        EXPECTED_CHECKPOINT_DECISION,
        "missing_checkpoint",
    );
    require_true(
        &mut errors,
        record,
        "claims.authorized_public_envelope_fixture",
        "missing_fixture_claim",
    );
    require_telemetry(&mut errors, record);
    require_false(&mut errors, record, "secrets.recorded", "secret_leak");
    for claim in FORBIDDEN_TRUE_CLAIMS {
        require_false(&mut errors, record, claim, "public_server_overclaim");
    }
    errors
}

fn require_text(
    errors: &mut Vec<String>,
    record: &PublicServerRecord,
    key: &str,
    expected: &str,
    code: &str,
) {
    match record.values.get(key) {
        Some(actual) if actual == expected => {}
        Some(actual) => errors.push(format!("{code}: {key} expected {expected}, found {actual}")),
        None => errors.push(format!("{code}: {key} missing")),
    }
}

fn require_true(errors: &mut Vec<String>, record: &PublicServerRecord, key: &str, code: &str) {
    match record.values.get(key).map(String::as_str) {
        Some("true") => {}
        Some(actual) => errors.push(format!("{code}: {key} expected true, found {actual}")),
        None => errors.push(format!("{code}: {key} missing")),
    }
}

fn require_false(errors: &mut Vec<String>, record: &PublicServerRecord, key: &str, code: &str) {
    match record.values.get(key).map(String::as_str) {
        Some("false") => {}
        Some(actual) => errors.push(format!("{code}: {key} expected false, found {actual}")),
        None => errors.push(format!("{code}: {key} missing")),
    }
}

fn require_telemetry(errors: &mut Vec<String>, record: &PublicServerRecord) {
    let telemetry = match record.values.get("telemetry.fields") {
        Some(value) => split_csv(value),
        None => BTreeSet::new(),
    };
    for required in REQUIRED_TELEMETRY {
        if !telemetry.contains(*required) {
            errors.push(format!(
                "missing_telemetry: telemetry.fields missing {required}"
            ));
        }
    }
}

fn split_csv(value: &str) -> BTreeSet<String> {
    value
        .split(',')
        .map(str::trim)
        .filter(|item| !item.is_empty())
        .map(ToString::to_string)
        .collect()
}

fn positive_record_text() -> String {
    format!(
        "row.name={EXPECTED_ROW_NAME}\n\
         row.status={EXPECTED_STATUS}\n\
         target.owner={EXPECTED_TARGET_OWNER}\n\
         target.authorization_artifact={EXPECTED_AUTHORIZATION_ARTIFACT}\n\
         target.scope={EXPECTED_TARGET_SCOPE}\n\
         bounds.client_count={EXPECTED_CLIENT_COUNT}\n\
         bounds.duration_secs={EXPECTED_DURATION_SECS}\n\
         bounds.traffic_limits={EXPECTED_TRAFFIC_LIMITS}\n\
         telemetry.fields={}\n\
         abort.criteria={EXPECTED_ABORT_CRITERIA}\n\
         redaction.policy={EXPECTED_REDACTION_POLICY}\n\
         checkpoint.decision={EXPECTED_CHECKPOINT_DECISION}\n\
         claims.authorized_public_envelope_fixture=true\n\
         claims.live_public_server_safety=false\n\
         claims.third_party_target_safety_without_authorization=false\n\
         claims.production_readiness=false\n\
         claims.adversarial_safety=false\n\
         claims.wan_tolerance=false\n\
         claims.load_safety_beyond_configured_bounds=false\n\
         claims.unbounded_public_testing=false\n\
         secrets.recorded=false\n",
        REQUIRED_TELEMETRY.join(",")
    )
}

fn run_self_tests() -> Result<String, Vec<String>> {
    let mut errors = Vec::new();
    errors.extend(expect_record_ok(
        "positive fixture",
        &positive_record_text(),
    ));
    errors.extend(expect_record_error(
        "missing owner",
        EXPECTED_TARGET_OWNER,
        "",
        "missing_owner",
    ));
    errors.extend(expect_record_error(
        "missing authorization",
        EXPECTED_AUTHORIZATION_ARTIFACT,
        "",
        "missing_written_authorization",
    ));
    errors.extend(expect_record_error(
        "wrong target scope",
        EXPECTED_TARGET_SCOPE,
        "public-internet-live",
        "missing_target_scope",
    ));
    errors.extend(expect_record_error(
        "missing bounds",
        &format!("bounds.duration_secs={EXPECTED_DURATION_SECS}"),
        "bounds.duration_secs=0",
        "missing_bounds",
    ));
    errors.extend(expect_record_error(
        "missing telemetry",
        "telemetry.fields=",
        "telemetry.fields=target_owner",
        "missing_telemetry",
    ));
    errors.extend(expect_record_error(
        "missing checkpoint",
        EXPECTED_CHECKPOINT_DECISION,
        "missing",
        "missing_checkpoint",
    ));
    errors.extend(expect_record_error(
        "secret leak",
        "secrets.recorded=false",
        "secrets.recorded=true",
        "secret_leak",
    ));
    errors.extend(expect_record_error(
        "live public overclaim",
        "claims.live_public_server_safety=false",
        "claims.live_public_server_safety=true",
        "public_server_overclaim",
    ));
    errors.extend(expect_record_error(
        "production overclaim",
        "claims.production_readiness=false",
        "claims.production_readiness=true",
        "public_server_overclaim",
    ));

    if errors.is_empty() {
        Ok("positive fixture and fail-closed mutations passed".to_string())
    } else {
        Err(errors)
    }
}

fn expect_record_ok(name: &str, text: &str) -> Vec<String> {
    match parse_record(text).map(|record| validate_public_server_record(&record)) {
        Ok(errors) if errors.is_empty() => Vec::new(),
        Ok(errors) => vec![format!("{name}: expected ok, got {errors:?}")],
        Err(errors) => vec![format!("{name}: parse failed {errors:?}")],
    }
}

fn expect_record_error(name: &str, old_text: &str, new_text: &str, needle: &str) -> Vec<String> {
    let text = positive_record_text().replace(old_text, new_text);
    match parse_record(&text).map(|record| validate_public_server_record(&record)) {
        Ok(errors) if errors.iter().any(|error| error.contains(needle)) => Vec::new(),
        Ok(errors) => vec![format!("{name}: expected {needle:?}, got {errors:?}")],
        Err(errors) => vec![format!("{name}: parse failed {errors:?}")],
    }
}

fn exit_with_errors(errors: &[String]) -> ExitCode {
    for error in errors {
        eprintln!("public-server authorized safety check failed: {error}");
    }
    FAILURE
}
