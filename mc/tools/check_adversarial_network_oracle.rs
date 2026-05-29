#!/usr/bin/env -S nix shell "github:nix-community/fenix?rev=092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain" nixpkgs#gcc -c cargo -q -Zscript

use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

const SELF_TEST_FLAG: &str = "--self-test";
const CONTRACT_FLAG: &str = "--contract";
const RECORD_FLAG: &str = "--record";
const CONTRACT_DOC: &str =
    "docs/evidence/protocol-763-adversarial-network-oracle-contract-2026-05-29.md";
const EXPECTED_RAIL_NAME: &str = "adversarial-network-oracle";
const EXPECTED_THREAT_MODEL_ID: &str = "protocol763-custom-payload-truncated-varint-v1";
const EXPECTED_MUTATION_TYPE: &str = "custom_payload_truncated_varint";
const EXPECTED_TARGET_OWNERSHIP: &str = "owned-local-fixture";
const EXPECTED_AUTHORIZATION: &str = "fixture-only-approved";
const EXPECTED_ORACLE_DECISION: &str = "approved_for_deterministic_fixture_only";
const EXPECTED_CONTAINMENT: &str = "failed_closed_before_live_traffic";
const EXPECTED_MAX_MUTATED_PACKETS: u32 = 1;
const EXPECTED_MAX_PAYLOAD_BYTES: u32 = 64;
const TOO_MANY_MUTATED_PACKETS: u32 = EXPECTED_MAX_MUTATED_PACKETS + 1;
const TOO_MANY_PAYLOAD_BYTES: u32 = EXPECTED_MAX_PAYLOAD_BYTES + 1;
const KEY_VALUE_SEPARATOR: char = '=';
const SUCCESS: ExitCode = ExitCode::SUCCESS;
const FAILURE: ExitCode = ExitCode::FAILURE;

const REQUIRED_TELEMETRY: &[&str] = &[
    "threat_model_id",
    "mutation_types",
    "packet_bounds",
    "target_ownership",
    "authorization",
    "abort_criteria",
    "observed_containment",
    "oracle_decision",
];

const FORBIDDEN_TRUE_CLAIMS: &[&str] = &[
    "claims.adversarial_network_safety",
    "claims.malicious_client_resilience",
    "claims.hostile_internet_safety",
    "claims.production_readiness",
    "claims.public_server_safety",
    "claims.unbounded_adversarial_robustness",
    "claims.full_protocol_security",
];

const CONTRACT_TOKENS: &[&str] = &[
    EXPECTED_RAIL_NAME,
    EXPECTED_THREAT_MODEL_ID,
    EXPECTED_MUTATION_TYPE,
    EXPECTED_TARGET_OWNERSHIP,
    EXPECTED_AUTHORIZATION,
    EXPECTED_ORACLE_DECISION,
    "max_mutated_packets=1",
    "max_payload_bytes=64",
    "missing_oracle_approval",
    "missing_threat_model_id",
    "missing_target_ownership",
    "missing_authorization",
    "unbounded_mutation",
    "live_network_enabled",
    "missing_telemetry",
    "missing_abort_criteria",
    "security_overclaim",
    "full protocol security remain non-claims",
];

#[derive(Debug, Clone, PartialEq, Eq)]
struct Args {
    self_test: bool,
    contract_path: String,
    record_path: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct OracleRecord {
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
                println!("adversarial-network oracle self-test passed: {summary}");
                SUCCESS
            }
            Err(errors) => exit_with_errors(&errors),
        };
    }

    match run_repo_check(&args) {
        Ok(summary) => {
            println!("adversarial-network oracle check passed: {summary}");
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
        record_path: None,
    };
    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        if arg == SELF_TEST_FLAG {
            parsed.self_test = true;
        } else if arg == CONTRACT_FLAG {
            parsed.contract_path = next_value(&mut iter, CONTRACT_FLAG)?;
        } else if arg == RECORD_FLAG {
            parsed.record_path = Some(next_value(&mut iter, RECORD_FLAG)?);
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
    let mut errors = validate_contract_text(&contract_text);
    if let Some(record_path) = &args.record_path {
        let record_text = read_text(record_path)?;
        let record = parse_record(&record_text)?;
        errors.extend(validate_oracle_record(&record));
    }
    if errors.is_empty() {
        Ok(format!(
            "contract={} record={}",
            args.contract_path,
            args.record_path.is_some()
        ))
    } else {
        Err(errors)
    }
}

fn read_text(path: &str) -> Result<String, Vec<String>> {
    fs::read_to_string(Path::new(path)).map_err(|err| vec![format!("{path}: {err}")])
}

fn parse_record(text: &str) -> Result<OracleRecord, Vec<String>> {
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
        Ok(OracleRecord { values })
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
    let mut errors = Vec::new();
    for token in CONTRACT_TOKENS {
        if !text.contains(token) {
            errors.push(format!("contract missing token: {token}"));
        }
    }
    errors
}

fn validate_oracle_record(record: &OracleRecord) -> Vec<String> {
    let mut errors = Vec::new();
    require_text(
        &mut errors,
        record,
        "rail.name",
        EXPECTED_RAIL_NAME,
        "wrong_rail",
    );
    require_text(
        &mut errors,
        record,
        "threat_model.id",
        EXPECTED_THREAT_MODEL_ID,
        "missing_threat_model_id",
    );
    require_true(
        &mut errors,
        record,
        "threat_model.approved",
        "missing_oracle_approval",
    );
    require_text(
        &mut errors,
        record,
        "threat_model.oracle_decision",
        EXPECTED_ORACLE_DECISION,
        "missing_oracle_approval",
    );
    require_text(
        &mut errors,
        record,
        "mutation.types",
        EXPECTED_MUTATION_TYPE,
        "missing_mutation_type",
    );
    require_u32_at_most(
        &mut errors,
        record,
        "mutation.max_packets",
        EXPECTED_MAX_MUTATED_PACKETS,
        "unbounded_mutation",
    );
    require_u32_at_most(
        &mut errors,
        record,
        "mutation.max_payload_bytes",
        EXPECTED_MAX_PAYLOAD_BYTES,
        "unbounded_mutation",
    );
    require_false(
        &mut errors,
        record,
        "mutation.live_network_enabled",
        "live_network_enabled",
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
    require_telemetry(&mut errors, record);
    require_text(
        &mut errors,
        record,
        "abort.criteria",
        "fail_closed_before_live_traffic",
        "missing_abort_criteria",
    );
    require_text(
        &mut errors,
        record,
        "observed.containment",
        EXPECTED_CONTAINMENT,
        "missing_telemetry",
    );
    require_true(
        &mut errors,
        record,
        "claims.adversarial_network_oracle_fixture",
        "missing_fixture_claim",
    );
    for claim in FORBIDDEN_TRUE_CLAIMS {
        require_false(&mut errors, record, claim, "security_overclaim");
    }
    errors
}

fn require_text(
    errors: &mut Vec<String>,
    record: &OracleRecord,
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

fn require_true(errors: &mut Vec<String>, record: &OracleRecord, key: &str, code: &str) {
    match record.values.get(key).map(String::as_str) {
        Some("true") => {}
        Some(actual) => errors.push(format!("{code}: {key} expected true, found {actual}")),
        None => errors.push(format!("{code}: {key} missing")),
    }
}

fn require_false(errors: &mut Vec<String>, record: &OracleRecord, key: &str, code: &str) {
    match record.values.get(key).map(String::as_str) {
        Some("false") => {}
        Some(actual) => errors.push(format!("{code}: {key} expected false, found {actual}")),
        None => errors.push(format!("{code}: {key} missing")),
    }
}

fn require_u32_at_most(
    errors: &mut Vec<String>,
    record: &OracleRecord,
    key: &str,
    maximum: u32,
    code: &str,
) {
    let Some(actual) = record.values.get(key) else {
        errors.push(format!("{code}: {key} missing"));
        return;
    };
    match actual.parse::<u32>() {
        Ok(number) if number <= maximum => {}
        Ok(number) => errors.push(format!("{code}: {key} {number} exceeds {maximum}")),
        Err(_) => errors.push(format!("{code}: {key} not an integer: {actual}")),
    }
}

fn require_telemetry(errors: &mut Vec<String>, record: &OracleRecord) {
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
        "rail.name={EXPECTED_RAIL_NAME}\n\
         threat_model.id={EXPECTED_THREAT_MODEL_ID}\n\
         threat_model.approved=true\n\
         threat_model.oracle_decision={EXPECTED_ORACLE_DECISION}\n\
         mutation.types={EXPECTED_MUTATION_TYPE}\n\
         mutation.max_packets={EXPECTED_MAX_MUTATED_PACKETS}\n\
         mutation.max_payload_bytes={EXPECTED_MAX_PAYLOAD_BYTES}\n\
         mutation.live_network_enabled=false\n\
         target.ownership={EXPECTED_TARGET_OWNERSHIP}\n\
         target.authorization={EXPECTED_AUTHORIZATION}\n\
         telemetry.fields={}\n\
         abort.criteria=fail_closed_before_live_traffic\n\
         observed.containment={EXPECTED_CONTAINMENT}\n\
         claims.adversarial_network_oracle_fixture=true\n\
         claims.adversarial_network_safety=false\n\
         claims.malicious_client_resilience=false\n\
         claims.hostile_internet_safety=false\n\
         claims.production_readiness=false\n\
         claims.public_server_safety=false\n\
         claims.unbounded_adversarial_robustness=false\n\
         claims.full_protocol_security=false\n",
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
        "missing approval",
        "threat_model.approved=true\n",
        "",
        "missing_oracle_approval",
    ));
    errors.extend(expect_record_error(
        "missing threat model",
        EXPECTED_THREAT_MODEL_ID,
        "wrong-model",
        "missing_threat_model_id",
    ));
    errors.extend(expect_record_error(
        "missing ownership",
        EXPECTED_TARGET_OWNERSHIP,
        "public-internet",
        "missing_target_ownership",
    ));
    errors.extend(expect_record_error(
        "missing authorization",
        EXPECTED_AUTHORIZATION,
        "",
        "missing_authorization",
    ));
    errors.extend(expect_record_error(
        "unbounded packets",
        &format!("mutation.max_packets={EXPECTED_MAX_MUTATED_PACKETS}"),
        &format!("mutation.max_packets={TOO_MANY_MUTATED_PACKETS}"),
        "unbounded_mutation",
    ));
    errors.extend(expect_record_error(
        "unbounded payload",
        &format!("mutation.max_payload_bytes={EXPECTED_MAX_PAYLOAD_BYTES}"),
        &format!("mutation.max_payload_bytes={TOO_MANY_PAYLOAD_BYTES}"),
        "unbounded_mutation",
    ));
    errors.extend(expect_record_error(
        "live network",
        "mutation.live_network_enabled=false",
        "mutation.live_network_enabled=true",
        "live_network_enabled",
    ));
    errors.extend(expect_record_error(
        "missing telemetry",
        "telemetry.fields=",
        "telemetry.fields=threat_model_id",
        "missing_telemetry",
    ));
    errors.extend(expect_record_error(
        "missing abort",
        "abort.criteria=fail_closed_before_live_traffic",
        "abort.criteria=",
        "missing_abort_criteria",
    ));
    errors.extend(expect_record_error(
        "security overclaim",
        "claims.full_protocol_security=false",
        "claims.full_protocol_security=true",
        "security_overclaim",
    ));

    if errors.is_empty() {
        Ok("positive fixture and fail-closed mutations passed".to_string())
    } else {
        Err(errors)
    }
}

fn expect_record_ok(name: &str, text: &str) -> Vec<String> {
    match parse_record(text).map(|record| validate_oracle_record(&record)) {
        Ok(errors) if errors.is_empty() => Vec::new(),
        Ok(errors) => vec![format!("{name}: expected ok, got {errors:?}")],
        Err(errors) => vec![format!("{name}: parse failed {errors:?}")],
    }
}

fn expect_record_error(name: &str, old_text: &str, new_text: &str, needle: &str) -> Vec<String> {
    let text = positive_record_text().replace(old_text, new_text);
    match parse_record(&text).map(|record| validate_oracle_record(&record)) {
        Ok(errors) if errors.iter().any(|error| error.contains(needle)) => Vec::new(),
        Ok(errors) => vec![format!("{name}: expected {needle:?}, got {errors:?}")],
        Err(errors) => vec![format!("{name}: parse failed {errors:?}")],
    }
}

fn exit_with_errors(errors: &[String]) -> ExitCode {
    for error in errors {
        eprintln!("adversarial-network oracle check failed: {error}");
    }
    FAILURE
}
