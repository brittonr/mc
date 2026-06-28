#![allow(unused_imports)]

use super::*;
use crate::test_support::*;
use crate::*;
use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::time::Duration;

const TEST_FAILURE_BUNDLE_ARTIFACT_PATH: &str = "docs/evidence/failure-receipt.json";
const TEST_FAILURE_BUNDLE_KIND: &str = "receipt";
const TEST_FAILURE_BUNDLE_FIRST_FAILURE: &str = "scenario missing required milestone";
const TEST_FAILURE_BUNDLE_PATH_ESCAPE: &str = "docs/evidence/../secret.log";
const TEST_FAILURE_BUNDLE_TARGET_PATH: &str = "target/failure-receipt.json";
const TEST_FAILURE_BUNDLE_MALFORMED_DIGEST: &str = "not-a-blake3-digest";
const TEST_FAILURE_BUNDLE_SUCCESS_OUTCOME: &str = "pass";

fn failure_bundle_digest_fixture() -> String {
    blake3::hash(b"failure bundle artifact")
        .to_hex()
        .to_string()
}

fn failure_bundle_artifact_fixture() -> FailureBundleArtifact {
    FailureBundleArtifact {
        kind: TEST_FAILURE_BUNDLE_KIND.to_string(),
        path: TEST_FAILURE_BUNDLE_ARTIFACT_PATH.to_string(),
        blake3: failure_bundle_digest_fixture(),
    }
}

fn failure_bundle_fixture() -> FailureEvidenceBundle {
    FailureEvidenceBundle {
        schema: FAILURE_BUNDLE_SCHEMA.to_string(),
        outcome: FAILURE_BUNDLE_OUTCOME_FAILED.to_string(),
        scenario: "smoke".to_string(),
        backend: "valence".to_string(),
        mode: "run".to_string(),
        command_summary: "mc-compat-runner --run --scenario smoke --server-backend valence"
            .to_string(),
        first_failure: TEST_FAILURE_BUNDLE_FIRST_FAILURE.to_string(),
        artifacts: vec![failure_bundle_artifact_fixture()],
        non_claims: FAILURE_BUNDLE_REQUIRED_NON_CLAIMS
            .iter()
            .map(|claim| (*claim).to_string())
            .collect(),
    }
}

fn failure_bundle_diagnostics(bundle: &FailureEvidenceBundle) -> String {
    validate_failure_evidence_bundle(bundle)
        .expect_err("failure bundle fixture should fail")
        .join("; ")
}

#[test]
fn failure_bundle_validator_accepts_complete_fail_only_bundle() {
    let bundle = failure_bundle_fixture();
    validate_failure_evidence_bundle(&bundle).expect("valid failure bundle passes");
    let json = render_failure_evidence_bundle_json(&bundle);

    assert!(json.contains(FAILURE_BUNDLE_SCHEMA));
    assert!(json.contains("\"diagnostic_only\": true"));
    assert!(json.contains("\"claims_scenario_success\": false"));
    assert!(json.contains(TEST_FAILURE_BUNDLE_ARTIFACT_PATH));
    assert!(is_blake3_hex(&bundle.artifacts[0].blake3));
}

#[test]
fn failure_bundle_validator_rejects_negative_fixtures() {
    let mut missing_artifacts = failure_bundle_fixture();
    missing_artifacts.artifacts.clear();
    assert!(failure_bundle_diagnostics(&missing_artifacts).contains("missing artifacts"));

    let mut path_escape = failure_bundle_fixture();
    path_escape.artifacts[0].path = TEST_FAILURE_BUNDLE_PATH_ESCAPE.to_string();
    assert!(failure_bundle_diagnostics(&path_escape).contains("escapes repo"));

    let mut target_only = failure_bundle_fixture();
    target_only.artifacts[0].path = TEST_FAILURE_BUNDLE_TARGET_PATH.to_string();
    assert!(failure_bundle_diagnostics(&target_only).contains("target-only"));

    let mut malformed_digest = failure_bundle_fixture();
    malformed_digest.artifacts[0].blake3 = TEST_FAILURE_BUNDLE_MALFORMED_DIGEST.to_string();
    assert!(failure_bundle_diagnostics(&malformed_digest).contains("malformed BLAKE3"));

    let mut missing_nonclaim = failure_bundle_fixture();
    missing_nonclaim
        .non_claims
        .retain(|claim| claim != FAILURE_BUNDLE_NON_CLAIM_SEMANTIC_EQUIVALENCE);
    assert!(failure_bundle_diagnostics(&missing_nonclaim).contains("missing non_claim"));

    let mut success_labeled = failure_bundle_fixture();
    success_labeled.outcome = TEST_FAILURE_BUNDLE_SUCCESS_OUTCOME.to_string();
    assert!(failure_bundle_diagnostics(&success_labeled).contains("failed or blocked"));
}
