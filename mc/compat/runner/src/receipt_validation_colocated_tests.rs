#![allow(unused_imports)]

use super::*;
use crate::test_support::*;
use crate::*;
use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::time::Duration;

fn receipt_fixture(backend: &str, protocol: u32, port: u16) -> String {
    receipt_fixture_with_classification(backend, protocol, port, "timeout-success-evidence")
}

fn receipt_fixture_with_classification(
    backend: &str,
    protocol: u32,
    port: u16,
    classification: &str,
) -> String {
    format!(
        "{{\n  \"schema\": \"mc.compat.smoke.receipt.v1\",\n  \"status\": \"pass\",\n  \"mode\": \"run\",\n  \"dry_run\": false,\n  \"contract\": {{\n    \"claims_correctness\": false,\n    \"claims_semantic_equivalence\": false\n  }},\n  \"server\": {{\n    \"backend\": \"{backend}\",\n    \"version\": \"1.18.2\",\n    \"protocol\": {protocol},\n    \"port\": {port}\n  }},\n  \"client\": {{\n    \"headless_isolation\": {{\n      \"xvfb\": true,\n      \"x11_backend\": true,\n      \"software_gl\": true,\n      \"wayland_socket_inherited\": false\n    }},\n    \"classification\": \"{classification}\",\n    \"matched_success_pattern\": \"Detected server protocol version\"\n  }},\n  \"error\": null\n}}\n"
    )
}

#[test]
fn compares_paper_and_valence_receipts() {
    let paper = read_receipt_summary_from_text(
        PathBuf::from("paper.json"),
        &receipt_fixture("paper", 758, 25566),
    )
    .expect("paper fixture parses");
    let valence = read_receipt_summary_from_text(
        PathBuf::from("valence.json"),
        &receipt_fixture("valence", 758, 25565),
    )
    .expect("valence fixture parses");

    validate_receipt_pair(&paper, &valence, DEFAULT_SERVER_PROTOCOL)
        .expect("matching receipts compare");
}

#[test]
fn rejects_receipt_protocol_mismatch() {
    let paper = read_receipt_summary_from_text(
        PathBuf::from("paper.json"),
        &receipt_fixture("paper", 758, 25566),
    )
    .expect("paper fixture parses");
    let valence = read_receipt_summary_from_text(
        PathBuf::from("valence.json"),
        &receipt_fixture("valence", 759, 25565),
    )
    .expect("valence fixture parses");

    let err = validate_receipt_pair(&paper, &valence, DEFAULT_SERVER_PROTOCOL).unwrap_err();
    assert!(err.contains("receipt protocol mismatch"), "{err}");
}

#[test]
fn compares_protocol_763_matrix_receipts_when_configured() {
    const PROTOCOL_763: u32 = 763;
    let paper = read_receipt_summary_from_text(
        PathBuf::from("paper.json"),
        &receipt_fixture("paper", PROTOCOL_763, 25566),
    )
    .expect("paper fixture parses");
    let valence = read_receipt_summary_from_text(
        PathBuf::from("valence.json"),
        &receipt_fixture("valence", PROTOCOL_763, 25565),
    )
    .expect("valence fixture parses");

    validate_receipt_pair(&paper, &valence, PROTOCOL_763)
        .expect("configured protocol receipts compare");
}

#[test]
fn compares_reconnect_sequence_receipts_with_multi_client_classification() {
    const PROTOCOL_763: u32 = 763;
    let paper = read_receipt_summary_from_text(
        PathBuf::from("paper.json"),
        &receipt_fixture_with_classification(
            "paper",
            PROTOCOL_763,
            25566,
            "multi-client-load-evidence",
        ),
    )
    .expect("paper fixture parses");
    let valence = read_receipt_summary_from_text(
        PathBuf::from("valence.json"),
        &receipt_fixture_with_classification(
            "valence",
            PROTOCOL_763,
            25565,
            "multi-client-load-evidence",
        ),
    )
    .expect("valence fixture parses");

    validate_receipt_pair(&paper, &valence, PROTOCOL_763)
        .expect("reconnect sequence receipts compare");
}

#[test]
fn receipt_summary_mutations_fail_closed() {
    let missing_success = read_receipt_summary_from_text(
        PathBuf::from("missing-success.json"),
        &receipt_fixture("paper", DEFAULT_SERVER_PROTOCOL, 25566).replace(
            "\"matched_success_pattern\": \"Detected server protocol version\"",
            "\"matched_success_pattern\": null",
        ),
    )
    .expect("missing success fixture parses");
    let err = validate_receipt_summary(&missing_success).unwrap_err();
    assert!(
        err.contains("missing matched client success pattern"),
        "{err}"
    );

    let bad_headless = read_receipt_summary_from_text(
        PathBuf::from("bad-headless.json"),
        &receipt_fixture("paper", DEFAULT_SERVER_PROTOCOL, 25566).replace(
            "\"wayland_socket_inherited\": false",
            "\"wayland_socket_inherited\": true",
        ),
    )
    .expect("bad headless fixture parses");
    let err = validate_receipt_summary(&bad_headless).unwrap_err();
    assert!(err.contains("headless isolation"), "{err}");

    let failed_status = read_receipt_summary_from_text(
        PathBuf::from("failed-status.json"),
        &receipt_fixture("paper", DEFAULT_SERVER_PROTOCOL, 25566)
            .replace("\"status\": \"pass\"", "\"status\": \"fail\""),
    )
    .expect("failed status fixture parses");
    let err = validate_receipt_summary(&failed_status).unwrap_err();
    assert!(err.contains("did not pass"), "{err}");

    let unsupported_classification = read_receipt_summary_from_text(
        PathBuf::from("unsupported-classification.json"),
        &receipt_fixture_with_classification(
            "paper",
            DEFAULT_SERVER_PROTOCOL,
            25566,
            "unchecked-live-claim",
        ),
    )
    .expect("unsupported classification fixture parses");
    let err = validate_receipt_summary(&unsupported_classification).unwrap_err();
    assert!(err.contains("unsupported client classification"), "{err}");
}

#[test]
fn rejects_receipts_that_do_not_match_configured_protocol() {
    const PROTOCOL_763: u32 = 763;
    let paper = read_receipt_summary_from_text(
        PathBuf::from("paper.json"),
        &receipt_fixture("paper", PROTOCOL_763, 25566),
    )
    .expect("paper fixture parses");
    let valence = read_receipt_summary_from_text(
        PathBuf::from("valence.json"),
        &receipt_fixture("valence", PROTOCOL_763, 25565),
    )
    .expect("valence fixture parses");

    let err = validate_receipt_pair(&paper, &valence, DEFAULT_SERVER_PROTOCOL).unwrap_err();
    assert!(
        err.contains(&format!(
            "expected protocol {DEFAULT_SERVER_PROTOCOL}, got {PROTOCOL_763}"
        )),
        "{err}"
    );
}
