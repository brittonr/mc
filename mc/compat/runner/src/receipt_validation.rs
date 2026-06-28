//! Pure receipt parsing and validation for matrix/comparison gates.
//!
//! This module evaluates in-memory receipt text and summaries only. File reads,
//! receipt writes, process exits, and user-facing output remain in `main.rs`.

use std::path::PathBuf;

const LEGACY_SMOKE_RECEIPT_SCHEMA: &str = "mc.compat.smoke.receipt.v1";
const SCENARIO_RECEIPT_SCHEMA: &str = "mc.compat.scenario.receipt.v2";
const PASS_STATUS: &str = "pass";
const PAPER_BACKEND: &str = "paper";
const VALENCE_BACKEND: &str = "valence";
const PAPER_DEFAULT_SERVER_PORT: u16 = 25566;
const VALENCE_DEFAULT_SERVER_PORT: u16 = 25565;
const TIMEOUT_SUCCESS_CLASSIFICATION: &str = "timeout-success-evidence";
const CLIENT_EXITED_SUCCESS_CLASSIFICATION: &str = "client-exited-success";
const MULTI_CLIENT_LOAD_CLASSIFICATION: &str = "multi-client-load-evidence";
const DRY_RUN_CLASSIFICATION: &str = "dry-run";
const CONTRACT_OBJECT: &str = "contract";
const SERVER_OBJECT: &str = "server";
const CLIENT_OBJECT: &str = "client";
const HEADLESS_ISOLATION_OBJECT: &str = "headless_isolation";
const SCHEMA_FIELD: &str = "schema";
const STATUS_FIELD: &str = "status";
const DRY_RUN_FIELD: &str = "dry_run";
const BACKEND_FIELD: &str = "backend";
const PROTOCOL_FIELD: &str = "protocol";
const PORT_FIELD: &str = "port";
const CLASSIFICATION_FIELD: &str = "classification";
const MATCHED_SUCCESS_PATTERN_FIELD: &str = "matched_success_pattern";
const XVFB_FIELD: &str = "xvfb";
const X11_BACKEND_FIELD: &str = "x11_backend";
const SOFTWARE_GL_FIELD: &str = "software_gl";
const WAYLAND_SOCKET_INHERITED_FIELD: &str = "wayland_socket_inherited";
const CLAIMS_CORRECTNESS_FIELD: &str = "claims_correctness";
const CLAIMS_SEMANTIC_EQUIVALENCE_FIELD: &str = "claims_semantic_equivalence";

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ReceiptSummary {
    pub(crate) path: PathBuf,
    pub(crate) schema: String,
    pub(crate) status: String,
    pub(crate) dry_run: bool,
    pub(crate) backend: String,
    pub(crate) protocol: u32,
    pub(crate) port: u16,
    pub(crate) classification: String,
    pub(crate) matched_success_pattern: Option<String>,
    pub(crate) xvfb: bool,
    pub(crate) x11_backend: bool,
    pub(crate) software_gl: bool,
    pub(crate) wayland_socket_inherited: bool,
    pub(crate) claims: ReceiptClaimBoundary,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ReceiptClaimBoundary {
    pub(crate) claims_correctness: bool,
    pub(crate) claims_semantic_equivalence: bool,
}

pub(crate) fn read_receipt_summary_from_text(
    path: PathBuf,
    text: &str,
) -> Result<ReceiptSummary, String> {
    let client = json_object_slice(text, CLIENT_OBJECT)?;
    let headless = json_object_slice(client, HEADLESS_ISOLATION_OBJECT)?;
    Ok(ReceiptSummary {
        path,
        schema: json_string_field(text, SCHEMA_FIELD)?,
        status: json_string_field(text, STATUS_FIELD)?,
        dry_run: json_bool_field(text, DRY_RUN_FIELD)?,
        backend: json_object_string_field(text, SERVER_OBJECT, BACKEND_FIELD)?,
        protocol: json_object_u32_field(text, SERVER_OBJECT, PROTOCOL_FIELD)?,
        port: json_object_u16_field(text, SERVER_OBJECT, PORT_FIELD)?,
        classification: json_object_string_field(text, CLIENT_OBJECT, CLASSIFICATION_FIELD)?,
        matched_success_pattern: json_object_optional_string_field(
            text,
            CLIENT_OBJECT,
            MATCHED_SUCCESS_PATTERN_FIELD,
        )?,
        xvfb: json_bool_field(headless, XVFB_FIELD)?,
        x11_backend: json_bool_field(headless, X11_BACKEND_FIELD)?,
        software_gl: json_bool_field(headless, SOFTWARE_GL_FIELD)?,
        wayland_socket_inherited: json_bool_field(headless, WAYLAND_SOCKET_INHERITED_FIELD)?,
        claims: read_receipt_claim_boundary(text)?,
    })
}

pub(crate) fn validate_receipt_pair(
    left: &ReceiptSummary,
    right: &ReceiptSummary,
    expected_protocol: u32,
) -> Result<(), String> {
    validate_receipt_summary(left)?;
    validate_receipt_summary(right)?;
    let backends = [left.backend.as_str(), right.backend.as_str()];
    if !(backends.contains(&PAPER_BACKEND) && backends.contains(&VALENCE_BACKEND)) {
        return Err(format!(
            "expected one paper receipt and one valence receipt, got {} and {}",
            left.backend, right.backend
        ));
    }
    if left.protocol != right.protocol {
        return Err(format!(
            "receipt protocol mismatch: {} has {}, {} has {}",
            left.path.display(),
            left.protocol,
            right.path.display(),
            right.protocol
        ));
    }
    if left.protocol != expected_protocol {
        return Err(format!(
            "expected protocol {}, got {}",
            expected_protocol, left.protocol
        ));
    }
    for receipt in [left, right] {
        match receipt.backend.as_str() {
            PAPER_BACKEND if receipt.port != PAPER_DEFAULT_SERVER_PORT => {
                return Err(format!(
                    "paper receipt port must be {}, got {}",
                    PAPER_DEFAULT_SERVER_PORT, receipt.port
                ));
            }
            VALENCE_BACKEND if receipt.port != VALENCE_DEFAULT_SERVER_PORT => {
                return Err(format!(
                    "valence receipt port must be {}, got {}",
                    VALENCE_DEFAULT_SERVER_PORT, receipt.port
                ));
            }
            _ => {}
        }
    }
    Ok(())
}

pub(crate) fn validate_receipt_summary(receipt: &ReceiptSummary) -> Result<(), String> {
    if !matches!(
        receipt.schema.as_str(),
        LEGACY_SMOKE_RECEIPT_SCHEMA | SCENARIO_RECEIPT_SCHEMA
    ) {
        return Err(format!(
            "{} has unexpected schema {}",
            receipt.path.display(),
            receipt.schema
        ));
    }
    if receipt.status != PASS_STATUS {
        return Err(format!(
            "{} did not pass; status={}",
            receipt.path.display(),
            receipt.status
        ));
    }
    let classification_supported = matches!(
        receipt.classification.as_str(),
        TIMEOUT_SUCCESS_CLASSIFICATION
            | CLIENT_EXITED_SUCCESS_CLASSIFICATION
            | MULTI_CLIENT_LOAD_CLASSIFICATION
    ) || (receipt.dry_run
        && receipt.classification == DRY_RUN_CLASSIFICATION);
    if !classification_supported {
        return Err(format!(
            "{} has unsupported client classification {}",
            receipt.path.display(),
            receipt.classification
        ));
    }
    if receipt.matched_success_pattern.is_none() && !receipt.dry_run {
        return Err(format!(
            "{} is missing matched client success pattern",
            receipt.path.display()
        ));
    }
    if !(receipt.xvfb && receipt.x11_backend && receipt.software_gl)
        || receipt.wayland_socket_inherited
    {
        return Err(format!(
            "{} does not prove niri-safe headless isolation",
            receipt.path.display()
        ));
    }
    validate_receipt_claim_boundary(&receipt.claims).map_err(|err| {
        format!(
            "{} violates receipt claim boundary: {err}",
            receipt.path.display()
        )
    })?;
    Ok(())
}

pub(crate) fn validate_receipt_claim_boundary(claims: &ReceiptClaimBoundary) -> Result<(), String> {
    if claims.claims_correctness {
        return Err("claims_correctness must remain false".to_string());
    }
    if claims.claims_semantic_equivalence {
        return Err("claims_semantic_equivalence must remain false".to_string());
    }
    Ok(())
}

fn read_receipt_claim_boundary(text: &str) -> Result<ReceiptClaimBoundary, String> {
    let contract = json_object_slice(text, CONTRACT_OBJECT)?;
    Ok(ReceiptClaimBoundary {
        claims_correctness: json_bool_field(contract, CLAIMS_CORRECTNESS_FIELD)?,
        claims_semantic_equivalence: json_bool_field(contract, CLAIMS_SEMANTIC_EQUIVALENCE_FIELD)?,
    })
}

fn json_object_string_field(text: &str, object: &str, key: &str) -> Result<String, String> {
    json_string_field(json_object_slice(text, object)?, key)
}

fn json_object_optional_string_field(
    text: &str,
    object: &str,
    key: &str,
) -> Result<Option<String>, String> {
    json_optional_string_field(json_object_slice(text, object)?, key)
}

fn json_object_u32_field(text: &str, object: &str, key: &str) -> Result<u32, String> {
    json_u32_field(json_object_slice(text, object)?, key)
}

fn json_object_u16_field(text: &str, object: &str, key: &str) -> Result<u16, String> {
    let value = json_object_u32_field(text, object, key)?;
    u16::try_from(value).map_err(|err| format!("parse field {key}: {err}"))
}

fn json_object_slice<'a>(text: &'a str, object: &str) -> Result<&'a str, String> {
    let key = format!("\"{object}\"");
    let mut search_start = 0usize;
    while let Some(relative_start) = text[search_start..].find(&key) {
        let start = search_start + relative_start;
        let after_key = &text[start + key.len()..];
        let after_colon = match after_key.trim_start().strip_prefix(':') {
            Some(value) => value,
            None => {
                search_start = start + key.len();
                continue;
            }
        };
        let brace_offset = after_colon
            .find('{')
            .ok_or_else(|| format!("missing object body for {object}"))?;
        let body_start = text.len() - after_colon.len() + brace_offset;
        let mut depth = 0usize;
        for (offset, ch) in text[body_start..].char_indices() {
            match ch {
                '{' => depth += 1,
                '}' => {
                    depth -= 1;
                    if depth == 0 {
                        return Ok(&text[body_start..=body_start + offset]);
                    }
                }
                _ => {}
            }
        }
        return Err(format!("unterminated object {object}"));
    }
    Err(format!("missing object {object}"))
}

fn json_string_field(text: &str, key: &str) -> Result<String, String> {
    let after_colon = json_field_value(text, key)?;
    parse_json_string(after_colon).map(|(value, _)| value)
}

fn json_optional_string_field(text: &str, key: &str) -> Result<Option<String>, String> {
    let Some(after_colon) = json_field_value_opt(text, key)? else {
        return Ok(None);
    };
    if after_colon.trim_start().starts_with("null") {
        Ok(None)
    } else {
        parse_json_string(after_colon).map(|(value, _)| Some(value))
    }
}

fn json_u32_field(text: &str, key: &str) -> Result<u32, String> {
    let value = json_field_value(text, key)?.trim_start();
    let digits: String = value.chars().take_while(|ch| ch.is_ascii_digit()).collect();
    if digits.is_empty() {
        return Err(format!("field {key} is not an unsigned integer"));
    }
    digits
        .parse()
        .map_err(|err| format!("parse field {key}: {err}"))
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

fn json_field_value<'a>(text: &'a str, key: &str) -> Result<&'a str, String> {
    json_field_value_opt(text, key)?.ok_or_else(|| format!("missing field {key}"))
}

fn json_field_value_opt<'a>(text: &'a str, key: &str) -> Result<Option<&'a str>, String> {
    let needle = format!("\"{key}\"");
    let Some(start) = text.find(&needle) else {
        return Ok(None);
    };
    let after_key = &text[start + needle.len()..];
    let colon = after_key
        .find(':')
        .ok_or_else(|| format!("missing colon for field {key}"))?;
    Ok(Some(&after_key[colon + 1..]))
}

fn parse_json_string(text: &str) -> Result<(String, &str), String> {
    let text = text.trim_start();
    let mut chars = text.char_indices();
    match chars.next() {
        Some((_, '"')) => {}
        _ => return Err("expected JSON string".to_string()),
    }
    let mut out = String::new();
    let mut escape = false;
    for (idx, ch) in chars {
        if escape {
            match ch {
                '"' => out.push('"'),
                '\\' => out.push('\\'),
                '/' => out.push('/'),
                'n' => out.push('\n'),
                'r' => out.push('\r'),
                't' => out.push('\t'),
                other => out.push(other),
            }
            escape = false;
        } else if ch == '\\' {
            escape = true;
        } else if ch == '"' {
            return Ok((out, &text[idx + 1..]));
        } else {
            out.push(ch);
        }
    }
    Err("unterminated JSON string".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const DEFAULT_PROTOCOL: u32 = 758;
    const PROTOCOL_763: u32 = 763;
    const INVALID_PORT: u16 = 25564;

    fn receipt_fixture(backend: &str, protocol: u32, port: u16) -> String {
        receipt_fixture_with_classification(backend, protocol, port, TIMEOUT_SUCCESS_CLASSIFICATION)
    }

    fn receipt_fixture_with_classification(
        backend: &str,
        protocol: u32,
        port: u16,
        classification: &str,
    ) -> String {
        format!(
            "{{\n  \"schema\": \"{LEGACY_SMOKE_RECEIPT_SCHEMA}\",\n  \"status\": \"{PASS_STATUS}\",\n  \"mode\": \"run\",\n  \"dry_run\": false,\n  \"contract\": {{\n    \"claims_correctness\": false,\n    \"claims_semantic_equivalence\": false\n  }},\n  \"server\": {{\n    \"backend\": \"{backend}\",\n    \"version\": \"1.18.2\",\n    \"protocol\": {protocol},\n    \"port\": {port}\n  }},\n  \"client\": {{\n    \"headless_isolation\": {{\n      \"xvfb\": true,\n      \"x11_backend\": true,\n      \"software_gl\": true,\n      \"wayland_socket_inherited\": false\n    }},\n    \"classification\": \"{classification}\",\n    \"matched_success_pattern\": \"Detected server protocol version\"\n  }},\n  \"error\": null\n}}\n"
        )
    }

    fn parse_fixture(name: &str, text: &str) -> ReceiptSummary {
        read_receipt_summary_from_text(PathBuf::from(name), text).expect("receipt fixture parses")
    }

    #[test]
    fn receipt_pair_validation_accepts_matching_paper_and_valence_receipts() {
        let paper = parse_fixture(
            "paper.json",
            &receipt_fixture(PAPER_BACKEND, DEFAULT_PROTOCOL, PAPER_DEFAULT_SERVER_PORT),
        );
        let valence = parse_fixture(
            "valence.json",
            &receipt_fixture(
                VALENCE_BACKEND,
                DEFAULT_PROTOCOL,
                VALENCE_DEFAULT_SERVER_PORT,
            ),
        );

        validate_receipt_pair(&paper, &valence, DEFAULT_PROTOCOL)
            .expect("matching paper/valence pair validates");
    }

    #[test]
    fn receipt_summary_validation_accepts_protocol_763_pair() {
        let paper = parse_fixture(
            "paper.json",
            &receipt_fixture(PAPER_BACKEND, PROTOCOL_763, PAPER_DEFAULT_SERVER_PORT),
        );
        let valence = parse_fixture(
            "valence.json",
            &receipt_fixture(VALENCE_BACKEND, PROTOCOL_763, VALENCE_DEFAULT_SERVER_PORT),
        );

        validate_receipt_pair(&paper, &valence, PROTOCOL_763)
            .expect("configured protocol pair validates");
    }

    #[test]
    fn receipt_summary_rejects_missing_required_fields() {
        let missing_status =
            receipt_fixture(PAPER_BACKEND, DEFAULT_PROTOCOL, PAPER_DEFAULT_SERVER_PORT)
                .replace("  \"status\": \"pass\",\n", "");

        let err =
            read_receipt_summary_from_text(PathBuf::from("missing-status.json"), &missing_status)
                .expect_err("missing status is rejected while parsing");

        assert!(err.contains("missing field status"), "{err}");
    }

    #[test]
    fn receipt_summary_rejects_wrong_typed_fields() {
        let wrong_type =
            receipt_fixture(PAPER_BACKEND, DEFAULT_PROTOCOL, PAPER_DEFAULT_SERVER_PORT)
                .replace("\"dry_run\": false", "\"dry_run\": \"false\"");

        let err = read_receipt_summary_from_text(PathBuf::from("wrong-type.json"), &wrong_type)
            .expect_err("wrong typed dry_run is rejected");

        assert!(err.contains("field dry_run is not a bool"), "{err}");
    }

    #[test]
    fn receipt_summary_rejects_malformed_evidence() {
        let bad_headless =
            receipt_fixture(PAPER_BACKEND, DEFAULT_PROTOCOL, PAPER_DEFAULT_SERVER_PORT).replace(
                "\"wayland_socket_inherited\": false",
                "\"wayland_socket_inherited\": true",
            );
        let summary = parse_fixture("bad-headless.json", &bad_headless);

        let err = validate_receipt_summary(&summary).expect_err("bad headless evidence fails");

        assert!(err.contains("headless isolation"), "{err}");
    }

    #[test]
    fn receipt_summary_rejects_broad_overclaims() {
        let overclaim = receipt_fixture(PAPER_BACKEND, DEFAULT_PROTOCOL, PAPER_DEFAULT_SERVER_PORT)
            .replace(
                "\"claims_correctness\": false",
                "\"claims_correctness\": true",
            );
        let summary = parse_fixture("overclaim.json", &overclaim);

        let err = validate_receipt_summary(&summary).expect_err("overclaiming receipt fails");

        assert!(
            err.contains("claims_correctness must remain false"),
            "{err}"
        );
    }

    #[test]
    fn receipt_pair_validation_rejects_wrong_backend_port() {
        let paper = parse_fixture(
            "paper.json",
            &receipt_fixture(PAPER_BACKEND, DEFAULT_PROTOCOL, INVALID_PORT),
        );
        let valence = parse_fixture(
            "valence.json",
            &receipt_fixture(
                VALENCE_BACKEND,
                DEFAULT_PROTOCOL,
                VALENCE_DEFAULT_SERVER_PORT,
            ),
        );

        let err = validate_receipt_pair(&paper, &valence, DEFAULT_PROTOCOL)
            .expect_err("paper port mismatch fails");

        assert!(err.contains("paper receipt port"), "{err}");
    }
}

#[cfg(test)]
#[path = "receipt_validation_colocated_tests.rs"]
mod root_colocated_tests;
