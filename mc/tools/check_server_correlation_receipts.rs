use std::env;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

const SELF_TEST_FLAG: &str = "--self-test";
const PROMOTABLE_FLAG: &str = "--promotable";
const SUCCESS: ExitCode = ExitCode::SUCCESS;
const FAILURE: ExitCode = ExitCode::FAILURE;
const SCHEMA_FIELD: &str = "schema";
const SCHEMA_VALUE: &str = "mc.compat.server_correlation_receipt.v1";
const RECEIPT_KIND_FIELD: &str = "receipt_kind";
const RECEIPT_KIND_CHECKER_FIXTURE: &str = "checker-fixture";
const RECEIPT_KIND_OWNED_LOCAL_LIVE: &str = "owned-local-live";
const ROW_FIELD: &str = "row";
const SCENARIO_FIELD: &str = "scenario";
const ACTOR_FIELD: &str = "actor";
const ACTOR_COMPATBOT: &str = "compatbot";
const SCOPE_FIELD: &str = "scope";
const SCOPE_OWNED_LOCAL: &str = "owned-local";
const REDACTION_POLICY_FIELD: &str = "redaction_policy";
const REDACTION_POLICY_NO_SECRETS: &str = "no-secrets-no-public-addresses";
const BACKEND_PATH_FIELD: &str = "backend_path";
const CLIENT_PATH_FIELD: &str = "client_path";
const BACKEND_REVISION_FIELD: &str = "backend_revision";
const CLIENT_REVISION_FIELD: &str = "client_revision";
const CORRELATION_STATUS_FIELD: &str = "correlation_status";
const CORRELATION_STATUS_OBSERVED: &str = "observed";
const PACKET_ROWS_FIELD: &str = "packet_rows";
const CLIENT_MILESTONES_FIELD: &str = "client_milestones";
const SERVER_EVENTS_FIELD: &str = "server_events";
const NONCLAIMS_FIELD: &str = "nonclaims";
const CLAIMS_PUBLIC_SERVER_SAFETY_FIELD: &str = "claims_public_server_safety";
const CLAIMS_PRODUCTION_READINESS_FIELD: &str = "claims_production_readiness";
const CLAIMS_FULL_PROTOCOL_763_FIELD: &str = "claims_full_protocol_763_compatibility";
const CLAIMS_BROAD_COMPAT_FIELD: &str = "claims_broad_minecraft_compatibility";
const CLAIMS_ARBITRARY_SIGN_SEMANTICS_FIELD: &str = "claims_arbitrary_sign_semantics";
const CLAIMS_RESOURCE_PACK_TRUST_FIELD: &str = "claims_resource_pack_asset_trust";
const STALE_REVISION: &str = "stale";
const DIRTY_REVISION: &str = "dirty";
const UNAVAILABLE_REVISION: &str = "unavailable";
const EMPTY_REVISION: &str = "";
const JSON_QUOTE: char = '"';
const JSON_ARRAY_START: char = '[';
const JSON_ARRAY_END: char = ']';
const JSON_FIELD_SEPARATOR: char = ':';
const JSON_COMMA: char = ',';
const TRUE_VALUE: &str = "true";
const FALSE_VALUE: &str = "false";
const RESOURCE_PACK_ROW: &str = "resource-pack-status";
const SIGN_EDITOR_ROW: &str = "sign-editor-open-update";
const RESOURCE_PACK_SCENARIO: &str = "mcp-controlled-smoke";
const SIGN_EDITOR_SCENARIO: &str = "survival-block-entity-persistence-parity";
const RESOURCE_PACK_BACKEND_PATH: &str = "owned-local-resource-pack-correlation-rail";
const SIGN_EDITOR_BACKEND_PATH: &str = "owned-local-sign-editor-correlation-rail";
const RESOURCE_PACK_CLIENT_PATH: &str = "stevenarella-resource-pack-status-driver";
const SIGN_EDITOR_CLIENT_PATH: &str = "stevenarella-sign-editor-driver";
const RESOURCE_PACK_OFFER_ID_FIELD: &str = "resource_pack_offer_id";
const RESOURCE_PACK_STATUS_FIELD: &str = "resource_pack_status";
const RESOURCE_PACK_NO_EXTERNAL_FETCH_FIELD: &str = "resource_pack_no_external_fetch";
const RESOURCE_PACK_OFFER_ID: &str = "mc-compat-local-resource-pack";
const RESOURCE_PACK_STATUS_DECLINED: &str = "declined";
const RESOURCE_PACK_PACKET_OFFER: &str = "play/clientbound/0x40 ResourcePackSendS2CPacket";
const RESOURCE_PACK_PACKET_STATUS: &str = "play/serverbound/0x24 ResourcePackStatusC2SPacket";
const RESOURCE_PACK_CLIENT_MILESTONE: &str = "resource_pack_status_sent";
const RESOURCE_PACK_SERVER_EVENT: &str = "resource_pack_status_declined_observed";
const SIGN_POSITION_FIELD: &str = "sign_position";
const SIGN_PAYLOAD_FIELD: &str = "sign_payload";
const SIGN_POSITION: &str = "28,64,0";
const SIGN_PAYLOAD: &str = "MC|Compat|Sign|Edit";
const SIGN_PACKET_OPEN: &str = "play/clientbound/0x31 SignEditorOpenS2CPacket";
const SIGN_PACKET_UPDATE: &str = "play/serverbound/0x2e UpdateSignC2SPacket";
const SIGN_OPEN_MILESTONE: &str = "sign_editor_open_observed";
const SIGN_UPDATE_MILESTONE: &str = "sign_update_sent";
const SIGN_SERVER_EVENT: &str = "sign_update_accepted_observed";
const COMMON_NONCLAIMS: &[&str] = &[
    "public_server_safety",
    "production_readiness",
    "full_protocol_763_compatibility",
    "broad_minecraft_compatibility",
];
const RESOURCE_PACK_NONCLAIMS: &[&str] = &["resource_pack_asset_trust"];
const SIGN_EDITOR_NONCLAIMS: &[&str] = &["arbitrary_sign_semantics", "all_block_entities"];
const OVERCLAIM_BOOL_FIELDS: &[&str] = &[
    CLAIMS_PUBLIC_SERVER_SAFETY_FIELD,
    CLAIMS_PRODUCTION_READINESS_FIELD,
    CLAIMS_FULL_PROTOCOL_763_FIELD,
    CLAIMS_BROAD_COMPAT_FIELD,
    CLAIMS_ARBITRARY_SIGN_SEMANTICS_FIELD,
    CLAIMS_RESOURCE_PACK_TRUST_FIELD,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ValidationMode {
    Fixture,
    Promotable,
}

fn main() -> ExitCode {
    let args = env::args().collect::<Vec<_>>();
    if args.iter().any(|arg| arg == SELF_TEST_FLAG) {
        return match run_self_tests() {
            Ok(summary) => {
                println!("server correlation receipt self-test passed: {summary}");
                SUCCESS
            }
            Err(errors) => {
                print_errors(&errors);
                FAILURE
            }
        };
    }

    let mode = if args.iter().any(|arg| arg == PROMOTABLE_FLAG) {
        ValidationMode::Promotable
    } else {
        ValidationMode::Fixture
    };
    let paths = receipt_paths(&args);
    if paths.is_empty() {
        eprintln!("usage: check_server_correlation_receipts [{PROMOTABLE_FLAG}] PATH [PATH ...]");
        return FAILURE;
    }

    let mut errors = Vec::new();
    for path in paths {
        if let Err(mut path_errors) = validate_receipt_path(Path::new(path), mode) {
            errors.append(&mut path_errors);
        }
    }
    if errors.is_empty() {
        println!(
            "server correlation receipt check passed: {} receipt(s)",
            receipt_paths(&args).len()
        );
        SUCCESS
    } else {
        print_errors(&errors);
        FAILURE
    }
}

fn print_errors(errors: &[String]) {
    for error in errors {
        eprintln!("server correlation receipt check failed: {error}");
    }
}

fn receipt_paths(args: &[String]) -> Vec<&str> {
    args.iter()
        .skip(1)
        .filter(|arg| arg.as_str() != PROMOTABLE_FLAG)
        .map(String::as_str)
        .collect()
}

fn validate_receipt_path(path: &Path, mode: ValidationMode) -> Result<(), Vec<String>> {
    let text =
        fs::read_to_string(path).map_err(|err| vec![format!("{}: {err}", path.display())])?;
    let errors = validate_receipt_text(&text, mode);
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors
            .into_iter()
            .map(|error| format!("{}: {error}", path.display()))
            .collect())
    }
}

fn validate_receipt_text(text: &str, mode: ValidationMode) -> Vec<String> {
    let mut errors = Vec::new();
    require_string_field(text, SCHEMA_FIELD, SCHEMA_VALUE, &mut errors);
    validate_receipt_kind(text, mode, &mut errors);
    require_string_field(text, ACTOR_FIELD, ACTOR_COMPATBOT, &mut errors);
    require_string_field(text, SCOPE_FIELD, SCOPE_OWNED_LOCAL, &mut errors);
    require_string_field(
        text,
        REDACTION_POLICY_FIELD,
        REDACTION_POLICY_NO_SECRETS,
        &mut errors,
    );
    require_string_field(
        text,
        CORRELATION_STATUS_FIELD,
        CORRELATION_STATUS_OBSERVED,
        &mut errors,
    );
    validate_revision_field(text, CLIENT_REVISION_FIELD, &mut errors);
    validate_revision_field(text, BACKEND_REVISION_FIELD, &mut errors);
    reject_overclaims(text, &mut errors);
    require_nonclaims(text, COMMON_NONCLAIMS, &mut errors);

    match string_field(text, ROW_FIELD).as_deref() {
        Some(RESOURCE_PACK_ROW) => validate_resource_pack_receipt(text, &mut errors),
        Some(SIGN_EDITOR_ROW) => validate_sign_editor_receipt(text, &mut errors),
        Some(row) => errors.push(format!("unsupported row {row}")),
        None => errors.push(format!("missing string field {ROW_FIELD}")),
    }

    errors
}

fn validate_receipt_kind(text: &str, mode: ValidationMode, errors: &mut Vec<String>) {
    match string_field(text, RECEIPT_KIND_FIELD).as_deref() {
        Some(RECEIPT_KIND_OWNED_LOCAL_LIVE) => {}
        Some(RECEIPT_KIND_CHECKER_FIXTURE) if mode == ValidationMode::Fixture => {}
        Some(RECEIPT_KIND_CHECKER_FIXTURE) => {
            errors.push("checker fixture receipts are not promotable live evidence".to_string())
        }
        Some(kind) => errors.push(format!("unsupported receipt_kind {kind}")),
        None => errors.push(format!("missing string field {RECEIPT_KIND_FIELD}")),
    }
}

fn validate_revision_field(text: &str, field: &str, errors: &mut Vec<String>) {
    match string_field(text, field).as_deref() {
        Some(EMPTY_REVISION) => errors.push(format!("{field} must not be empty")),
        Some(STALE_REVISION | DIRTY_REVISION | UNAVAILABLE_REVISION) => {
            errors.push(format!("{field} is not promotable"));
        }
        Some(_) => {}
        None => errors.push(format!("missing string field {field}")),
    }
}

fn validate_resource_pack_receipt(text: &str, errors: &mut Vec<String>) {
    require_string_field(text, SCENARIO_FIELD, RESOURCE_PACK_SCENARIO, errors);
    require_string_field(text, BACKEND_PATH_FIELD, RESOURCE_PACK_BACKEND_PATH, errors);
    require_string_field(text, CLIENT_PATH_FIELD, RESOURCE_PACK_CLIENT_PATH, errors);
    require_string_field(
        text,
        RESOURCE_PACK_OFFER_ID_FIELD,
        RESOURCE_PACK_OFFER_ID,
        errors,
    );
    require_string_field(
        text,
        RESOURCE_PACK_STATUS_FIELD,
        RESOURCE_PACK_STATUS_DECLINED,
        errors,
    );
    require_bool_field(text, RESOURCE_PACK_NO_EXTERNAL_FETCH_FIELD, true, errors);
    require_array_contains(text, PACKET_ROWS_FIELD, RESOURCE_PACK_PACKET_OFFER, errors);
    require_array_contains(text, PACKET_ROWS_FIELD, RESOURCE_PACK_PACKET_STATUS, errors);
    require_array_contains(
        text,
        CLIENT_MILESTONES_FIELD,
        RESOURCE_PACK_CLIENT_MILESTONE,
        errors,
    );
    require_array_contains(
        text,
        SERVER_EVENTS_FIELD,
        RESOURCE_PACK_SERVER_EVENT,
        errors,
    );
    require_nonclaims(text, RESOURCE_PACK_NONCLAIMS, errors);
}

fn validate_sign_editor_receipt(text: &str, errors: &mut Vec<String>) {
    require_string_field(text, SCENARIO_FIELD, SIGN_EDITOR_SCENARIO, errors);
    require_string_field(text, BACKEND_PATH_FIELD, SIGN_EDITOR_BACKEND_PATH, errors);
    require_string_field(text, CLIENT_PATH_FIELD, SIGN_EDITOR_CLIENT_PATH, errors);
    require_string_field(text, SIGN_POSITION_FIELD, SIGN_POSITION, errors);
    require_string_field(text, SIGN_PAYLOAD_FIELD, SIGN_PAYLOAD, errors);
    require_array_contains(text, PACKET_ROWS_FIELD, SIGN_PACKET_OPEN, errors);
    require_array_contains(text, PACKET_ROWS_FIELD, SIGN_PACKET_UPDATE, errors);
    require_array_contains(text, CLIENT_MILESTONES_FIELD, SIGN_OPEN_MILESTONE, errors);
    require_array_contains(text, CLIENT_MILESTONES_FIELD, SIGN_UPDATE_MILESTONE, errors);
    require_array_contains(text, SERVER_EVENTS_FIELD, SIGN_SERVER_EVENT, errors);
    require_nonclaims(text, SIGN_EDITOR_NONCLAIMS, errors);
}

fn require_string_field(text: &str, field: &str, expected: &str, errors: &mut Vec<String>) {
    match string_field(text, field) {
        Some(value) if value == expected => {}
        Some(value) => errors.push(format!("{field} expected {expected:?}, got {value:?}")),
        None => errors.push(format!("missing string field {field}")),
    }
}

fn require_bool_field(text: &str, field: &str, expected: bool, errors: &mut Vec<String>) {
    match bool_field(text, field) {
        Some(value) if value == expected => {}
        Some(value) => errors.push(format!("{field} expected {expected}, got {value}")),
        None => errors.push(format!("missing bool field {field}")),
    }
}

fn require_array_contains(text: &str, field: &str, expected: &str, errors: &mut Vec<String>) {
    if !array_field_contains(text, field, expected) {
        errors.push(format!("{field} missing {expected}"));
    }
}

fn require_nonclaims(text: &str, required: &[&str], errors: &mut Vec<String>) {
    for nonclaim in required {
        require_array_contains(text, NONCLAIMS_FIELD, nonclaim, errors);
    }
}

fn reject_overclaims(text: &str, errors: &mut Vec<String>) {
    for field in OVERCLAIM_BOOL_FIELDS {
        if bool_field(text, field) == Some(true) {
            errors.push(format!("overclaim rejected: {field}=true"));
        }
    }
}

fn string_field(text: &str, field: &str) -> Option<String> {
    let value = field_value(text, field)?;
    let trimmed = value.trim_start();
    if !trimmed.starts_with(JSON_QUOTE) {
        return None;
    }
    let content = &trimmed[JSON_QUOTE.len_utf8()..];
    let end = content.find(JSON_QUOTE)?;
    Some(content[..end].to_string())
}

fn bool_field(text: &str, field: &str) -> Option<bool> {
    let value = field_value(text, field)?.trim_start();
    if value.starts_with(TRUE_VALUE) {
        Some(true)
    } else if value.starts_with(FALSE_VALUE) {
        Some(false)
    } else {
        None
    }
}

fn array_field_contains(text: &str, field: &str, expected: &str) -> bool {
    string_array_field(text, field)
        .is_some_and(|values| values.iter().any(|value| value == expected))
}

fn string_array_field(text: &str, field: &str) -> Option<Vec<String>> {
    let value = field_value(text, field)?;
    let array_start = value.find(JSON_ARRAY_START)?;
    let array_end = value[array_start..].find(JSON_ARRAY_END)? + array_start;
    let body = &value[array_start + JSON_ARRAY_START.len_utf8()..array_end];
    let mut values = Vec::new();
    let mut remaining = body.trim_start();
    while !remaining.is_empty() {
        if remaining.starts_with(JSON_COMMA) {
            remaining = remaining[JSON_COMMA.len_utf8()..].trim_start();
            continue;
        }
        if !remaining.starts_with(JSON_QUOTE) {
            return None;
        }
        let content = &remaining[JSON_QUOTE.len_utf8()..];
        let end = content.find(JSON_QUOTE)?;
        values.push(content[..end].to_string());
        remaining = content[end + JSON_QUOTE.len_utf8()..].trim_start();
    }
    Some(values)
}

fn field_value<'a>(text: &'a str, field: &str) -> Option<&'a str> {
    let needle = format!("\"{field}\"");
    let start = text.find(&needle)?;
    let after_field = &text[start + needle.len()..];
    let separator = after_field.find(JSON_FIELD_SEPARATOR)?;
    Some(&after_field[separator + JSON_FIELD_SEPARATOR.len_utf8()..])
}

fn run_self_tests() -> Result<String, Vec<String>> {
    let tests = [
        positive_test(
            "resource-pack fixture validates",
            resource_pack_fixture(),
            ValidationMode::Fixture,
        ),
        positive_test(
            "sign-editor fixture validates",
            sign_editor_fixture(),
            ValidationMode::Fixture,
        ),
        positive_test(
            "owned-local live receipt is promotable",
            resource_pack_fixture()
                .replace(RECEIPT_KIND_CHECKER_FIXTURE, RECEIPT_KIND_OWNED_LOCAL_LIVE),
            ValidationMode::Promotable,
        ),
        negative_test(
            "fixture is not promotable",
            resource_pack_fixture(),
            ValidationMode::Promotable,
            "checker fixture receipts are not promotable",
        ),
        negative_test(
            "blocked correlation fails",
            resource_pack_fixture().replace(
                "\"correlation_status\": \"observed\"",
                "\"correlation_status\": \"missing\"",
            ),
            ValidationMode::Fixture,
            "correlation_status expected",
        ),
        negative_test(
            "missing server event fails",
            resource_pack_fixture().replace(RESOURCE_PACK_SERVER_EVENT, "wrong_server_event"),
            ValidationMode::Fixture,
            "server_events missing",
        ),
        negative_test(
            "wrong resource pack status fails",
            resource_pack_fixture().replace(
                "\"resource_pack_status\": \"declined\"",
                "\"resource_pack_status\": \"accepted\"",
            ),
            ValidationMode::Fixture,
            "resource_pack_status expected",
        ),
        negative_test(
            "wrong sign position fails",
            sign_editor_fixture().replace(
                "\"sign_position\": \"28,64,0\"",
                "\"sign_position\": \"29,64,0\"",
            ),
            ValidationMode::Fixture,
            "sign_position expected",
        ),
        negative_test(
            "malformed packet row fails",
            sign_editor_fixture().replace(SIGN_PACKET_UPDATE, "play/serverbound/0xff WrongPacket"),
            ValidationMode::Fixture,
            "packet_rows missing",
        ),
        negative_test(
            "overclaim fails",
            sign_editor_fixture().replace(
                "\"claims_public_server_safety\": false",
                "\"claims_public_server_safety\": true",
            ),
            ValidationMode::Fixture,
            "overclaim rejected",
        ),
    ];

    let test_count = tests.len();
    let mut errors = Vec::new();
    for test in tests {
        if let Err(error) = test {
            errors.push(error);
        }
    }
    if errors.is_empty() {
        Ok(format!("{test_count} fixtures"))
    } else {
        Err(errors)
    }
}

type SelfTestResult = Result<(), String>;

fn positive_test(name: &str, text: String, mode: ValidationMode) -> SelfTestResult {
    let errors = validate_receipt_text(&text, mode);
    if errors.is_empty() {
        Ok(())
    } else {
        Err(format!("{name}: unexpected errors: {}", errors.join("; ")))
    }
}

fn negative_test(name: &str, text: String, mode: ValidationMode, expected: &str) -> SelfTestResult {
    let errors = validate_receipt_text(&text, mode);
    if errors.iter().any(|error| error.contains(expected)) {
        Ok(())
    } else {
        Err(format!(
            "{name}: expected diagnostic containing {expected:?}, got {}",
            errors.join("; ")
        ))
    }
}

fn resource_pack_fixture() -> String {
    format!(
        r#"{{
  "schema": "{SCHEMA_VALUE}",
  "receipt_kind": "{RECEIPT_KIND_CHECKER_FIXTURE}",
  "row": "{RESOURCE_PACK_ROW}",
  "scenario": "{RESOURCE_PACK_SCENARIO}",
  "actor": "{ACTOR_COMPATBOT}",
  "scope": "{SCOPE_OWNED_LOCAL}",
  "redaction_policy": "{REDACTION_POLICY_NO_SECRETS}",
  "backend_path": "{RESOURCE_PACK_BACKEND_PATH}",
  "client_path": "{RESOURCE_PACK_CLIENT_PATH}",
  "backend_revision": "fixture-backend-rev",
  "client_revision": "fixture-client-rev",
  "resource_pack_offer_id": "{RESOURCE_PACK_OFFER_ID}",
  "resource_pack_status": "{RESOURCE_PACK_STATUS_DECLINED}",
  "resource_pack_no_external_fetch": true,
  "packet_rows": ["{RESOURCE_PACK_PACKET_OFFER}", "{RESOURCE_PACK_PACKET_STATUS}"],
  "client_milestones": ["{RESOURCE_PACK_CLIENT_MILESTONE}"],
  "server_events": ["{RESOURCE_PACK_SERVER_EVENT}"],
  "correlation_status": "{CORRELATION_STATUS_OBSERVED}",
  "nonclaims": ["public_server_safety", "production_readiness", "full_protocol_763_compatibility", "broad_minecraft_compatibility", "resource_pack_asset_trust"],
  "claims_public_server_safety": false,
  "claims_production_readiness": false,
  "claims_full_protocol_763_compatibility": false,
  "claims_broad_minecraft_compatibility": false,
  "claims_resource_pack_asset_trust": false
}}"#
    )
}

fn sign_editor_fixture() -> String {
    format!(
        r#"{{
  "schema": "{SCHEMA_VALUE}",
  "receipt_kind": "{RECEIPT_KIND_CHECKER_FIXTURE}",
  "row": "{SIGN_EDITOR_ROW}",
  "scenario": "{SIGN_EDITOR_SCENARIO}",
  "actor": "{ACTOR_COMPATBOT}",
  "scope": "{SCOPE_OWNED_LOCAL}",
  "redaction_policy": "{REDACTION_POLICY_NO_SECRETS}",
  "backend_path": "{SIGN_EDITOR_BACKEND_PATH}",
  "client_path": "{SIGN_EDITOR_CLIENT_PATH}",
  "backend_revision": "fixture-backend-rev",
  "client_revision": "fixture-client-rev",
  "sign_position": "{SIGN_POSITION}",
  "sign_payload": "{SIGN_PAYLOAD}",
  "packet_rows": ["{SIGN_PACKET_OPEN}", "{SIGN_PACKET_UPDATE}"],
  "client_milestones": ["{SIGN_OPEN_MILESTONE}", "{SIGN_UPDATE_MILESTONE}"],
  "server_events": ["{SIGN_SERVER_EVENT}"],
  "correlation_status": "{CORRELATION_STATUS_OBSERVED}",
  "nonclaims": ["public_server_safety", "production_readiness", "full_protocol_763_compatibility", "broad_minecraft_compatibility", "arbitrary_sign_semantics", "all_block_entities"],
  "claims_public_server_safety": false,
  "claims_production_readiness": false,
  "claims_full_protocol_763_compatibility": false,
  "claims_broad_minecraft_compatibility": false,
  "claims_arbitrary_sign_semantics": false
}}"#
    )
}
