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
    "docs/evidence/protocol-763-wan-tolerance-bounded-telemetry-contract-2026-05-29.md";
const ROW_DOC: &str = "docs/evidence/protocol-763-wan-tolerance-bounded-telemetry-2026-05-29.md";
const ROW_RECEIPT: &str =
    "docs/evidence/protocol-763-wan-tolerance-bounded-telemetry-2026-05-29.receipt.json";
const ROW_RECORD: &str =
    "docs/evidence/protocol-763-wan-tolerance-bounded-telemetry-2026-05-29.record";
const PRODUCTION_MATRIX_DOC: &str =
    "docs/evidence/protocol-763-production-network-safety-matrix-2026-05-28.md";
const CURRENT_BUNDLE_DOC: &str = "docs/evidence/protocol-763-current-evidence-bundle.md";
const ACCEPTANCE_MATRIX_DOC: &str = "docs/evidence/protocol-763-acceptance-matrix.md";

const EXPECTED_ROW_NAME: &str = "wan-tolerance-bounded-telemetry";
const EXPECTED_SCENARIO: &str = "inventory-interaction";
const EXPECTED_MECHANISM: &str = "bounded-client-cadence";
const EXPECTED_TARGET_OWNERSHIP: &str = "owned-local-loopback";
const EXPECTED_AUTHORIZATION: &str = "owned-local-fixture-approved";
const EXPECTED_DELAY_MS: &str = "80";
const EXPECTED_JITTER_MS: &str = "30";
const EXPECTED_LOSS_PERCENT: &str = "0";
const EXPECTED_TIMEOUT_SECS: &str = "180";
const EXPECTED_DURATION_SECS: &str = "180";
const EXPECTED_CLIENT_COUNT: &str = "1";
const EXPECTED_RECONNECT_COUNT: &str = "0";
const EXPECTED_STATUS: &str = "pass";
const EXPECTED_PASS_FAIL_CRITERIA: &str = "inventory_interaction_client_server_milestones";
const EXPECTED_MATRIX_STATUS: &str = "covered_owned_local_bounded_telemetry";
const PUBLIC_INTERNET_WAN_NON_CLAIM: &str = "Public/internet WAN safety remains a non-claim";
const MATRIX_PUBLIC_INTERNET_WAN_NON_CLAIM: &str = "No public/internet WAN safety";

const REQUIRED_TELEMETRY: &[&str] = &[
    "scenario_required_milestones",
    "scenario_observed_milestones",
    "server_required_milestones",
    "server_observed_milestones",
    "client_classification",
    "triage_boundary",
];

const FORBIDDEN_TRUE_CLAIMS: &[&str] = &[
    "claims.wan_safety",
    "claims.packet_loss_tolerance",
    "claims.internet_path_safety",
    "claims.public_server_safety",
    "claims.production_readiness",
    "claims.adversarial_network_safety",
    "claims.unbounded_soak",
    "claims.unbounded_reconnect",
];

const CONTRACT_TOKENS: &[&str] = &[
    EXPECTED_ROW_NAME,
    EXPECTED_SCENARIO,
    EXPECTED_MECHANISM,
    EXPECTED_TARGET_OWNERSHIP,
    EXPECTED_AUTHORIZATION,
    "delay_ms=80",
    "jitter_ms=30",
    "loss_percent=0",
    "timeout_secs=180",
    "duration_secs=180",
    "client_count=1",
    "reconnect_count=0",
    "scenario_observed_milestones",
    "server_observed_milestones",
    "missing_authorization",
    "missing_target_ownership",
    "missing_delay_ms",
    "missing_jitter_ms",
    "missing_loss_percent",
    "missing_telemetry",
    "wan_overclaim",
    "public_target_overclaim",
    "production_readiness_overclaim",
    PUBLIC_INTERNET_WAN_NON_CLAIM,
];

const ROW_RECEIPT_TOKENS: &[&str] = &[
    "mc.compat.scenario.receipt.v2",
    "\"status\": \"pass\"",
    "\"mode\": \"run\"",
    "\"dry_run\": false",
    "\"name\": \"inventory-interaction\"",
    "\"latency_jitter_tolerance\"",
    "\"selected\": true",
    "\"mechanism\": \"bounded-client-cadence\"",
    "\"delay_ms\": \"80\"",
    "\"jitter_ms\": \"30\"",
    "\"loss_percent\": \"0\"",
    "\"timeout_secs\": 180",
    "\"duration_secs\": 180",
    "\"client_count\": 1",
    "\"reconnect_count\": 0",
    "\"target_ownership\": \"owned-local-loopback\"",
    "\"authorization\": \"owned-local-fixture-approved\"",
    "\"telemetry_samples\"",
    "\"pass_fail_criteria\": \"inventory_interaction_client_server_milestones\"",
    "\"abort_reason\": \"none\"",
    "\"claims_wan_safety\": false",
    "\"claims_packet_loss_tolerance\": false",
    "\"claims_internet_path_safety\": false",
    "\"claims_public_server_safety\": false",
    "\"claims_production_readiness\": false",
    "\"inventory_slot_update\"",
    "\"server_inventory_click\"",
];

const MATRIX_TOKENS: &[&str] = &[
    "| WAN tolerance | covered_owned_local_bounded_telemetry |",
    EXPECTED_TARGET_OWNERSHIP,
    EXPECTED_AUTHORIZATION,
    "delay_ms=80",
    "jitter_ms=30",
    "loss_percent=0",
    "timeout_secs=180",
    "duration_secs=180",
    "client_count=1",
    "reconnect_count=0",
    ROW_RECEIPT,
    ROW_DOC,
    "No public-server safety",
    "No internet-path safety",
    "No packet-loss tolerance beyond loss_percent=0",
    "No production readiness",
];

const CURRENT_BUNDLE_TOKENS: &[&str] = &[
    "WAN tolerance bounded telemetry checkpoint",
    ROW_DOC,
    ROW_RECEIPT,
    "tools/check_wan_tolerance_bounded_telemetry.rs",
    EXPECTED_MATRIX_STATUS,
    PUBLIC_INTERNET_WAN_NON_CLAIM,
];

const ACCEPTANCE_MATRIX_TOKENS: &[&str] = &[
    "Latency/jitter tolerance",
    "bounded owned-local WAN telemetry row",
    "target_ownership=owned-local-loopback",
    "authorization=owned-local-fixture-approved",
    MATRIX_PUBLIC_INTERNET_WAN_NON_CLAIM,
];

const ROW_DOC_TOKENS: &[&str] = &[
    EXPECTED_ROW_NAME,
    ROW_RECEIPT,
    "target_ownership=owned-local-loopback",
    "authorization=owned-local-fixture-approved",
    "delay_ms=80",
    "jitter_ms=30",
    "loss_percent=0",
    "telemetry_samples",
    "promote only the bounded owned-local WAN telemetry row",
    "claims.wan_safety=false",
    "claims.packet_loss_tolerance=false",
    MATRIX_PUBLIC_INTERNET_WAN_NON_CLAIM,
];

#[derive(Debug, Clone, PartialEq, Eq)]
struct Args {
    self_test: bool,
    contract_path: String,
    record_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct WanRecord {
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
                println!("WAN tolerance bounded telemetry self-test passed: {summary}");
                SUCCESS
            }
            Err(errors) => exit_with_errors(&errors),
        };
    }

    match run_repo_check(&args) {
        Ok(summary) => {
            println!("WAN tolerance bounded telemetry check passed: {summary}");
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
    let record_text = read_text(&args.record_path)?;
    let record = parse_record(&record_text)?;

    let mut errors = validate_contract_text(&contract_text);
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
    errors.extend(validate_wan_record(&record));

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

fn parse_record(text: &str) -> Result<WanRecord, Vec<String>> {
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
        Ok(WanRecord { values })
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

fn validate_contract_text(text: &str) -> Vec<String> {
    validate_text_tokens("contract", text, CONTRACT_TOKENS)
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

fn validate_wan_record(record: &WanRecord) -> Vec<String> {
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
        "scenario.name",
        EXPECTED_SCENARIO,
        "wrong_scenario",
    );
    require_text(
        &mut errors,
        record,
        "perturbation.mechanism",
        EXPECTED_MECHANISM,
        "wrong_mechanism",
    );
    require_text(
        &mut errors,
        record,
        "target.ownership",
        EXPECTED_TARGET_OWNERSHIP,
        "missing_target_ownership",
    );
    require_text(
        &mut errors,
        record,
        "target.authorization",
        EXPECTED_AUTHORIZATION,
        "missing_authorization",
    );
    require_text(
        &mut errors,
        record,
        "metrics.delay_ms",
        EXPECTED_DELAY_MS,
        "missing_delay_ms",
    );
    require_text(
        &mut errors,
        record,
        "metrics.jitter_ms",
        EXPECTED_JITTER_MS,
        "missing_jitter_ms",
    );
    require_text(
        &mut errors,
        record,
        "metrics.loss_percent",
        EXPECTED_LOSS_PERCENT,
        "missing_loss_percent",
    );
    require_text(
        &mut errors,
        record,
        "metrics.timeout_secs",
        EXPECTED_TIMEOUT_SECS,
        "missing_timeout_secs",
    );
    require_text(
        &mut errors,
        record,
        "metrics.duration_secs",
        EXPECTED_DURATION_SECS,
        "missing_duration_secs",
    );
    require_text(
        &mut errors,
        record,
        "metrics.client_count",
        EXPECTED_CLIENT_COUNT,
        "missing_client_count",
    );
    require_text(
        &mut errors,
        record,
        "metrics.reconnect_count",
        EXPECTED_RECONNECT_COUNT,
        "missing_reconnect_count",
    );
    require_text(
        &mut errors,
        record,
        "pass_fail.criteria",
        EXPECTED_PASS_FAIL_CRITERIA,
        "missing_pass_fail_criteria",
    );
    require_text(
        &mut errors,
        record,
        "observed.status",
        EXPECTED_STATUS,
        "missing_status",
    );
    require_true(
        &mut errors,
        record,
        "preflight.fail_closed_when_unavailable",
        "missing_fail_closed_preflight",
    );
    require_true(
        &mut errors,
        record,
        "claims.bounded_wan_telemetry",
        "missing_bounded_claim",
    );
    require_telemetry(&mut errors, record);
    for claim in FORBIDDEN_TRUE_CLAIMS {
        require_false(&mut errors, record, claim, "wan_overclaim");
    }
    errors
}

fn require_text(
    errors: &mut Vec<String>,
    record: &WanRecord,
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

fn require_true(errors: &mut Vec<String>, record: &WanRecord, key: &str, code: &str) {
    match record.values.get(key).map(String::as_str) {
        Some("true") => {}
        Some(actual) => errors.push(format!("{code}: {key} expected true, found {actual}")),
        None => errors.push(format!("{code}: {key} missing")),
    }
}

fn require_false(errors: &mut Vec<String>, record: &WanRecord, key: &str, code: &str) {
    match record.values.get(key).map(String::as_str) {
        Some("false") => {}
        Some(actual) => errors.push(format!("{code}: {key} expected false, found {actual}")),
        None => errors.push(format!("{code}: {key} missing")),
    }
}

fn require_telemetry(errors: &mut Vec<String>, record: &WanRecord) {
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
         scenario.name={EXPECTED_SCENARIO}\n\
         perturbation.mechanism={EXPECTED_MECHANISM}\n\
         target.ownership={EXPECTED_TARGET_OWNERSHIP}\n\
         target.authorization={EXPECTED_AUTHORIZATION}\n\
         metrics.delay_ms={EXPECTED_DELAY_MS}\n\
         metrics.jitter_ms={EXPECTED_JITTER_MS}\n\
         metrics.loss_percent={EXPECTED_LOSS_PERCENT}\n\
         metrics.timeout_secs={EXPECTED_TIMEOUT_SECS}\n\
         metrics.duration_secs={EXPECTED_DURATION_SECS}\n\
         metrics.client_count={EXPECTED_CLIENT_COUNT}\n\
         metrics.reconnect_count={EXPECTED_RECONNECT_COUNT}\n\
         telemetry.fields={}\n\
         pass_fail.criteria={EXPECTED_PASS_FAIL_CRITERIA}\n\
         observed.status={EXPECTED_STATUS}\n\
         preflight.fail_closed_when_unavailable=true\n\
         claims.bounded_wan_telemetry=true\n\
         claims.wan_safety=false\n\
         claims.packet_loss_tolerance=false\n\
         claims.internet_path_safety=false\n\
         claims.public_server_safety=false\n\
         claims.production_readiness=false\n\
         claims.adversarial_network_safety=false\n\
         claims.unbounded_soak=false\n\
         claims.unbounded_reconnect=false\n",
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
        "missing authorization",
        EXPECTED_AUTHORIZATION,
        "",
        "missing_authorization",
    ));
    errors.extend(expect_record_error(
        "wrong ownership",
        EXPECTED_TARGET_OWNERSHIP,
        "public-internet",
        "missing_target_ownership",
    ));
    errors.extend(expect_record_error(
        "wrong delay",
        &format!("metrics.delay_ms={EXPECTED_DELAY_MS}"),
        "metrics.delay_ms=0",
        "missing_delay_ms",
    ));
    errors.extend(expect_record_error(
        "wrong jitter",
        &format!("metrics.jitter_ms={EXPECTED_JITTER_MS}"),
        "metrics.jitter_ms=0",
        "missing_jitter_ms",
    ));
    errors.extend(expect_record_error(
        "wrong loss",
        &format!("metrics.loss_percent={EXPECTED_LOSS_PERCENT}"),
        "metrics.loss_percent=5",
        "missing_loss_percent",
    ));
    errors.extend(expect_record_error(
        "missing telemetry",
        "telemetry.fields=",
        "telemetry.fields=scenario_observed_milestones",
        "missing_telemetry",
    ));
    errors.extend(expect_record_error(
        "missing fail closed preflight",
        "preflight.fail_closed_when_unavailable=true",
        "preflight.fail_closed_when_unavailable=false",
        "missing_fail_closed_preflight",
    ));
    errors.extend(expect_record_error(
        "WAN overclaim",
        "claims.wan_safety=false",
        "claims.wan_safety=true",
        "wan_overclaim",
    ));
    errors.extend(expect_record_error(
        "public target overclaim",
        "claims.public_server_safety=false",
        "claims.public_server_safety=true",
        "wan_overclaim",
    ));

    if errors.is_empty() {
        Ok("positive fixture and fail-closed mutations passed".to_string())
    } else {
        Err(errors)
    }
}

fn expect_record_ok(name: &str, text: &str) -> Vec<String> {
    match parse_record(text).map(|record| validate_wan_record(&record)) {
        Ok(errors) if errors.is_empty() => Vec::new(),
        Ok(errors) => vec![format!("{name}: expected ok, got {errors:?}")],
        Err(errors) => vec![format!("{name}: parse failed {errors:?}")],
    }
}

fn expect_record_error(name: &str, old_text: &str, new_text: &str, needle: &str) -> Vec<String> {
    let text = positive_record_text().replace(old_text, new_text);
    match parse_record(&text).map(|record| validate_wan_record(&record)) {
        Ok(errors) if errors.iter().any(|error| error.contains(needle)) => Vec::new(),
        Ok(errors) => vec![format!("{name}: expected {needle:?}, got {errors:?}")],
        Err(errors) => vec![format!("{name}: parse failed {errors:?}")],
    }
}

fn exit_with_errors(errors: &[String]) -> ExitCode {
    for error in errors {
        eprintln!("WAN tolerance bounded telemetry check failed: {error}");
    }
    FAILURE
}
