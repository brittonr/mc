use std::env;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

const SELF_TEST_FLAG: &str = "--self-test";
const RECEIPT_FLAG: &str = "--receipt";
const BLAKE3_HEX_LENGTH: usize = 64;
const MCP_BLOCK: &str = "mcp_control";
const FRAME_BLOCK: &str = "frame_artifacts";
const TRUE_VALUE: &str = "true";
const FALSE_VALUE: &str = "false";
const NULL_VALUE: &str = "null";
const EMPTY_ARRAY: &str = "[]";
const JSON_FIELD_SEPARATOR: &str = ":";
const JSON_STRING_QUOTE: char = '"';
const JSON_OBJECT_START: char = '{';
const JSON_OBJECT_END: char = '}';
const JSON_ARRAY_START: char = '[';
const JSON_ARRAY_END: char = ']';
const PATH_ESCAPE_TOKEN: &str = "../";
const TARGET_PATH_TOKEN: &str = "target/";
const DOCS_EVIDENCE_PREFIX: &str = "docs/evidence/";
const STALE_REVISION_TOKEN: &str = "stale";
const DIRTY_REVISION_STATUS: &str = "dirty";
const UNAVAILABLE_REVISION_STATUS: &str = "unavailable";

const REQUIRED_NON_CLAIMS: &[&str] = &[
    "visual_regression_approval",
    "semantic_equivalence",
    "full_minecraft_compatibility",
    "production_readiness",
    "public_server_safety",
    "load_testing",
];

const REQUIRED_CALLS: &[&str] = &["initialize", "tools/list", "tools/call status"];

const REQUIRED_OUTCOMES: &[&str] = &[
    "status.applied",
    "look.applied",
    "key.applied",
    "chat.applied",
];
const OVERCLAIM_BOOL_FIELDS: &[&str] = &[
    "claims_semantic_equivalence",
    "claims_correctness",
    "visual_regression_approval",
    "public_server_safety",
    "production_readiness",
    "load_testing",
];

fn main() -> ExitCode {
    let args = env::args().collect::<Vec<_>>();
    if args.iter().any(|arg| arg == SELF_TEST_FLAG) {
        return match run_self_tests() {
            Ok(summary) => {
                println!("mcp controlled rail self-test passed: {summary}");
                ExitCode::SUCCESS
            }
            Err(errors) => {
                print_errors(&errors);
                ExitCode::FAILURE
            }
        };
    }

    match receipt_path_arg(&args).and_then(validate_receipt_path) {
        Ok(summary) => {
            println!("mcp controlled rail receipt check passed: {summary}");
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
        eprintln!("mcp controlled rail check failed: {error}");
    }
}

fn receipt_path_arg(args: &[String]) -> Result<&Path, Vec<String>> {
    let Some(index) = args.iter().position(|arg| arg == RECEIPT_FLAG) else {
        return Err(vec![format!(
            "usage: check_mcp_controlled_compat_rail {RECEIPT_FLAG} PATH"
        )]);
    };
    let Some(path) = args.get(index + 1) else {
        return Err(vec![format!("{RECEIPT_FLAG} requires a path")]);
    };
    Ok(Path::new(path))
}

fn validate_receipt_path(path: &Path) -> Result<String, Vec<String>> {
    let text =
        fs::read_to_string(path).map_err(|err| vec![format!("{}: {err}", path.display())])?;
    let errors = validate_receipt_text(&text);
    if errors.is_empty() {
        Ok(path.display().to_string())
    } else {
        Err(errors)
    }
}

fn validate_receipt_text(text: &str) -> Vec<String> {
    let mut errors = Vec::new();
    let Some(mcp) = object_block(text, MCP_BLOCK) else {
        return vec![format!("missing {MCP_BLOCK} block")];
    };
    validate_mcp_block(&mcp, &mut errors);

    let Some(frame) = object_block(text, FRAME_BLOCK) else {
        errors.push(format!("missing {FRAME_BLOCK} block"));
        return errors;
    };
    validate_frame_block(&frame, &mut errors);
    errors
}

fn validate_mcp_block(block: &str, errors: &mut Vec<String>) {
    require_bool_field(block, "selected", true, errors);
    require_string_field(block, "endpoint_mode", "stdio", errors);
    require_bool_field(block, "handshake_success", true, errors);
    require_bool_field(block, "stdout_clean", true, errors);
    require_bool_field(block, "passed", true, errors);
    require_bool_field(block, "dry_run_fixture", true, errors);
    require_string_field(block, "revision_status", "dry-run", errors);

    match string_field(block, "tool_list_digest") {
        Some(value) if is_hex_digest(&value) => {}
        Some(value) => errors.push(format!(
            "tool_list_digest is not a BLAKE3 hex digest: {value}"
        )),
        None => errors.push("missing tool_list_digest".to_string()),
    }

    let child_revision = string_field(block, "stevenarella_child_revision");
    match child_revision.as_deref() {
        Some(value) if value != STALE_REVISION_TOKEN => {}
        Some(_) => errors.push("stale Stevenarella revision rejected".to_string()),
        None => errors.push("missing Stevenarella child revision".to_string()),
    }

    if string_field(block, "revision_status") == Some(DIRTY_REVISION_STATUS.to_string())
        || string_field(block, "revision_status") == Some(UNAVAILABLE_REVISION_STATUS.to_string())
    {
        errors.push("Stevenarella revision status is not promotable".to_string());
    }

    for required in REQUIRED_CALLS {
        if !array_field_contains(block, "calls_attempted", required) {
            errors.push(format!("calls_attempted missing {required}"));
        }
        if !array_field_contains(block, "calls_succeeded", required) {
            errors.push(format!("calls_succeeded missing {required}"));
        }
    }
    for outcome in REQUIRED_OUTCOMES {
        if !array_field_contains(block, "command_outcome_ids", outcome) {
            errors.push(format!("command_outcome_ids missing {outcome}"));
        }
    }
    if array_field_is_empty(block, "command_outcome_ids") {
        errors.push("command_outcome_ids must not be empty".to_string());
    }
    if field_value(block, "first_failure").is_some_and(|value| value != NULL_VALUE) {
        errors.push("first_failure must be null for valid dry-run receipt".to_string());
    }
    for non_claim in REQUIRED_NON_CLAIMS {
        if !array_field_contains(block, "non_claims", non_claim) {
            errors.push(format!("non_claims missing {non_claim}"));
        }
    }
    reject_overclaims(block, errors);
}

fn validate_frame_block(block: &str, errors: &mut Vec<String>) {
    if !bool_field(block, "selected").unwrap_or(false) {
        require_bool_field(block, "path_containment_checked", true, errors);
        require_bool_field(block, "promotion_ready", false, errors);
        return;
    }

    require_bool_field(block, "path_containment_checked", true, errors);
    if array_field_is_empty(block, "artifacts") {
        errors.push("frame_artifacts selected but artifacts are empty".to_string());
    }
    if !block.contains("\"blake3\"") || !contains_hex_digest(block) {
        errors.push("frame_artifacts selected but no BLAKE3 digest is recorded".to_string());
    }
    if block.contains(PATH_ESCAPE_TOKEN) || block.contains(TARGET_PATH_TOKEN) {
        errors.push("frame artifact path escapes reviewable evidence roots".to_string());
    }
    if !block.contains(DOCS_EVIDENCE_PREFIX) {
        errors.push("frame artifact path must be under docs/evidence/".to_string());
    }
}

fn require_bool_field(block: &str, field: &str, expected: bool, errors: &mut Vec<String>) {
    match bool_field(block, field) {
        Some(value) if value == expected => {}
        Some(value) => errors.push(format!("{field} expected {expected}, got {value}")),
        None => errors.push(format!("missing bool field {field}")),
    }
}

fn require_string_field(block: &str, field: &str, expected: &str, errors: &mut Vec<String>) {
    match string_field(block, field) {
        Some(value) if value == expected => {}
        Some(value) => errors.push(format!("{field} expected {expected:?}, got {value:?}")),
        None => errors.push(format!("missing string field {field}")),
    }
}

fn reject_overclaims(block: &str, errors: &mut Vec<String>) {
    for field in OVERCLAIM_BOOL_FIELDS {
        if bool_field(block, field) == Some(true) {
            errors.push(format!("overclaim rejected: {field}=true"));
        }
    }
}

fn object_block(text: &str, field: &str) -> Option<String> {
    let field_needle = format!("\"{field}\"");
    let field_start = text.find(&field_needle)?;
    let after_field = &text[field_start + field_needle.len()..];
    let colon = after_field.find(JSON_FIELD_SEPARATOR)?;
    let after_colon = &after_field[colon + JSON_FIELD_SEPARATOR.len()..];
    let object_relative_start = after_colon.find(JSON_OBJECT_START)?;
    let object_start = field_start
        + field_needle.len()
        + colon
        + JSON_FIELD_SEPARATOR.len()
        + object_relative_start;
    let mut depth = 0usize;
    for (offset, character) in text[object_start..].char_indices() {
        match character {
            JSON_OBJECT_START => depth += 1,
            JSON_OBJECT_END => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    let end = object_start + offset + character.len_utf8();
                    return Some(text[object_start..end].to_string());
                }
            }
            _ => {}
        }
    }
    None
}

fn field_value<'a>(block: &'a str, field: &str) -> Option<&'a str> {
    let field_needle = format!("\"{field}\"");
    let mut search_start = 0usize;
    while let Some(relative_start) = block[search_start..].find(&field_needle) {
        let field_start = search_start + relative_start;
        let after_field = &block[field_start + field_needle.len()..];
        let after_whitespace = after_field.trim_start();
        if !after_whitespace.starts_with(JSON_FIELD_SEPARATOR) {
            search_start = field_start + field_needle.len();
            continue;
        }
        let after_colon = after_whitespace[JSON_FIELD_SEPARATOR.len()..].trim_start();
        let end = after_colon
            .find([',', JSON_OBJECT_END])
            .unwrap_or(after_colon.len());
        return Some(after_colon[..end].trim());
    }
    None
}

fn bool_field(block: &str, field: &str) -> Option<bool> {
    match field_value(block, field)? {
        TRUE_VALUE => Some(true),
        FALSE_VALUE => Some(false),
        _ => None,
    }
}

fn string_field(block: &str, field: &str) -> Option<String> {
    let value = field_value(block, field)?;
    let inner = value
        .strip_prefix(JSON_STRING_QUOTE)
        .and_then(|value| value.strip_suffix(JSON_STRING_QUOTE))?;
    Some(inner.to_string())
}

fn array_field_contains(block: &str, field: &str, expected: &str) -> bool {
    array_field(block, field).is_some_and(|array| array.contains(&format!("\"{expected}\"")))
}

fn array_field_is_empty(block: &str, field: &str) -> bool {
    array_field(block, field).is_some_and(|array| array.trim() == EMPTY_ARRAY)
}

fn array_field<'a>(block: &'a str, field: &str) -> Option<&'a str> {
    let field_needle = format!("\"{field}\"");
    let field_start = block.find(&field_needle)?;
    let after_field = &block[field_start + field_needle.len()..];
    let colon = after_field.find(JSON_FIELD_SEPARATOR)?;
    let after_colon = after_field[colon + JSON_FIELD_SEPARATOR.len()..].trim_start();
    let array_start = after_colon.find(JSON_ARRAY_START)?;
    let start = colon + JSON_FIELD_SEPARATOR.len() + after_colon[..array_start].len() + array_start;
    let absolute_start = field_start + field_needle.len() + start;
    let mut depth = 0usize;
    for (offset, character) in block[absolute_start..].char_indices() {
        match character {
            JSON_ARRAY_START => depth += 1,
            JSON_ARRAY_END => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    let end = absolute_start + offset + character.len_utf8();
                    return Some(&block[absolute_start..end]);
                }
            }
            _ => {}
        }
    }
    None
}

fn is_hex_digest(value: &str) -> bool {
    value.len() == BLAKE3_HEX_LENGTH && value.chars().all(|character| character.is_ascii_hexdigit())
}

fn contains_hex_digest(text: &str) -> bool {
    let mut length = 0usize;
    for character in text.chars() {
        if character.is_ascii_hexdigit() {
            length += 1;
            if length == BLAKE3_HEX_LENGTH {
                return true;
            }
        } else {
            length = 0;
        }
    }
    false
}

fn run_self_tests() -> Result<String, Vec<String>> {
    let mut errors = Vec::new();
    errors.extend(expect_ok("valid dry-run receipt", valid_receipt_fixture()));
    errors.extend(expect_error(
        "missing handshake",
        valid_receipt_fixture().replace(
            "\"handshake_success\": true",
            "\"handshake_success\": false",
        ),
        "handshake_success expected true",
    ));
    errors.extend(expect_error(
        "stdout contamination",
        valid_receipt_fixture().replace("\"stdout_clean\": true", "\"stdout_clean\": false"),
        "stdout_clean expected true",
    ));
    errors.extend(expect_error(
        "missing command outcome",
        valid_receipt_fixture().replace(
            "\"status.applied\", \"look.applied\", \"key.applied\", \"chat.applied\"",
            "",
        ),
        "command_outcome_ids missing status.applied",
    ));
    errors.extend(expect_error(
        "missing frame digest",
        frame_selected_without_digest_fixture(),
        "no BLAKE3 digest",
    ));
    errors.extend(expect_error(
        "path escape",
        frame_path_escape_fixture(),
        "path escapes",
    ));
    errors.extend(expect_error(
        "stale revision",
        valid_receipt_fixture().replace(
            "\"stevenarella_child_revision\": \"dry-run\"",
            "\"stevenarella_child_revision\": \"stale\"",
        ),
        "stale Stevenarella revision",
    ));
    errors.extend(expect_error(
        "overclaim wording",
        valid_receipt_fixture().replace(
            "\"passed\": true",
            "\"claims_semantic_equivalence\": true, \"passed\": true",
        ),
        "claims_semantic_equivalence=true",
    ));
    errors.extend(expect_error(
        "compact overclaim wording",
        valid_receipt_fixture().replace(
            "\"passed\": true",
            "\"claims_semantic_equivalence\":true, \"passed\": true",
        ),
        "claims_semantic_equivalence=true",
    ));

    if errors.is_empty() {
        Ok("positive and fail-closed fixtures exercised".to_string())
    } else {
        Err(errors)
    }
}

fn expect_ok(name: &str, text: String) -> Vec<String> {
    let errors = validate_receipt_text(&text);
    if errors.is_empty() {
        Vec::new()
    } else {
        vec![format!("{name}: expected ok, got {errors:?}")]
    }
}

fn expect_error(name: &str, text: String, needle: &str) -> Vec<String> {
    let errors = validate_receipt_text(&text);
    if errors.iter().any(|error| error.contains(needle)) {
        Vec::new()
    } else {
        vec![format!(
            "{name}: expected error containing {needle:?}, got {errors:?}"
        )]
    }
}

fn valid_receipt_fixture() -> String {
    let digest = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
    format!(
        r#"{{
  "mcp_control": {{
    "selected": true,
    "endpoint_mode": "stdio",
    "handshake_success": true,
    "tool_list_digest": "{digest}",
    "tool_names": ["status", "connect", "disconnect", "key", "look", "mouse", "use-item", "attack", "chat"],
    "calls_attempted": ["initialize", "tools/list", "tools/call status", "tools/call look", "tools/call key", "tools/call chat"],
    "calls_succeeded": ["initialize", "tools/list", "tools/call status", "tools/call look", "tools/call key", "tools/call chat"],
    "first_failure": null,
    "stdout_clean": true,
    "command_outcome_ids": ["status.applied", "look.applied", "key.applied", "chat.applied"],
    "stevenarella_child_revision": "dry-run",
    "revision_status": "dry-run",
    "dry_run_fixture": true,
    "live_receipt": false,
    "prerequisites": ["stevenarella_mcp_control_archived", "main_thread_command_queue", "stdout_clean_stdio"],
    "non_claims": ["screenshots_alone", "visual_regression_approval", "semantic_equivalence", "full_minecraft_compatibility", "production_readiness", "public_server_safety", "load_testing"],
    "passed": true
  }},
  "frame_artifacts": {{
    "selected": false,
    "capture_requested": true,
    "artifact_count": 0,
    "artifacts": [],
    "missing_digests": [],
    "path_containment_checked": true,
    "promotion_ready": false,
    "non_claims": ["frame_capture_not_selected", "visual_regression_approval", "semantic_equivalence"]
  }}
}}"#
    )
}

fn frame_selected_without_digest_fixture() -> String {
    valid_receipt_fixture().replace(
        "\"selected\": false,\n    \"capture_requested\": true,\n    \"artifact_count\": 0,\n    \"artifacts\": [],",
        "\"selected\": true,\n    \"capture_requested\": true,\n    \"artifact_count\": 1,\n    \"artifacts\": [{\"path\": \"docs/evidence/frame.png\"}],",
    )
}

fn frame_path_escape_fixture() -> String {
    valid_receipt_fixture().replace(
        "\"selected\": false,\n    \"capture_requested\": true,\n    \"artifact_count\": 0,\n    \"artifacts\": [],",
        "\"selected\": true,\n    \"capture_requested\": true,\n    \"artifact_count\": 1,\n    \"artifacts\": [{\"path\": \"../frame.png\", \"blake3\": \"0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef\"}],",
    )
}
