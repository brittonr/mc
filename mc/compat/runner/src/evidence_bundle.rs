//! Pure failure-bundle schema construction, validation, and rendering.
//!
//! Shell code in `main.rs` owns artifact discovery, file hashing, directory
//! creation, and writes. This module owns the in-memory compatibility contract
//! for failure evidence bundles.

use crate::json_support::{json_string, json_string_vec};
use crate::runner_config::Config;
use crate::scenario_core::scenario_name;
use crate::{backend_name, mode_name};
use std::path::Path;

pub(crate) const FAILURE_BUNDLE_SCHEMA: &str = "mc.compat.failure.bundle.v1";
pub(crate) const FAILURE_BUNDLE_OUTCOME_FAILED: &str = "failed";
const FAILURE_BUNDLE_OUTCOME_BLOCKED: &str = "blocked";
pub(crate) const FAILURE_BUNDLE_ARTIFACT_RECEIPT: &str = "receipt";
pub(crate) const FAILURE_BUNDLE_ARTIFACT_TYPED_EVENTS: &str = "typed_events";
pub(crate) const FAILURE_BUNDLE_ARTIFACT_MCP_TRANSCRIPT: &str = "mcp_transcript";
pub(crate) const FAILURE_BUNDLE_ARTIFACT_STDERR: &str = "stderr";
pub(crate) const FAILURE_BUNDLE_ARTIFACT_SERVER_LOG: &str = "server_log";
const FAILURE_BUNDLE_REVIEW_STORAGE_PREFIX: &str = "docs/evidence/";
const FAILURE_BUNDLE_TARGET_COMPONENT: &str = "target";
pub(crate) const FAILURE_BUNDLE_HASH_BUFFER_BYTES: usize = 8192;
const FAILURE_BUNDLE_BLAKE3_HEX_CHARS: usize = 64;
const FAILURE_BUNDLE_MAX_FIRST_FAILURE_CHARS: usize = 512;
const FAILURE_BUNDLE_MAX_COMMAND_SUMMARY_CHARS: usize = 256;
const FAILURE_BUNDLE_NON_CLAIM_SCENARIO_SUCCESS: &str = "scenario_success";
const FAILURE_BUNDLE_NON_CLAIM_GAMEPLAY_PARITY: &str = "gameplay_parity";
const FAILURE_BUNDLE_NON_CLAIM_FULL_PROTOCOL: &str = "full_protocol_compatibility";
const FAILURE_BUNDLE_NON_CLAIM_PUBLIC_SERVER_SAFETY: &str = "public_server_safety";
const FAILURE_BUNDLE_NON_CLAIM_PRODUCTION_READINESS: &str = "production_readiness";
pub(crate) const FAILURE_BUNDLE_NON_CLAIM_SEMANTIC_EQUIVALENCE: &str = "semantic_equivalence";
pub(crate) const FAILURE_BUNDLE_REQUIRED_NON_CLAIMS: &[&str] = &[
    FAILURE_BUNDLE_NON_CLAIM_SCENARIO_SUCCESS,
    FAILURE_BUNDLE_NON_CLAIM_GAMEPLAY_PARITY,
    FAILURE_BUNDLE_NON_CLAIM_FULL_PROTOCOL,
    FAILURE_BUNDLE_NON_CLAIM_PUBLIC_SERVER_SAFETY,
    FAILURE_BUNDLE_NON_CLAIM_PRODUCTION_READINESS,
    FAILURE_BUNDLE_NON_CLAIM_SEMANTIC_EQUIVALENCE,
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FailureBundleArtifact {
    pub(crate) kind: String,
    pub(crate) path: String,
    pub(crate) blake3: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FailureEvidenceBundle {
    pub(crate) schema: String,
    pub(crate) outcome: String,
    pub(crate) scenario: String,
    pub(crate) backend: String,
    pub(crate) mode: String,
    pub(crate) command_summary: String,
    pub(crate) first_failure: String,
    pub(crate) artifacts: Vec<FailureBundleArtifact>,
    pub(crate) non_claims: Vec<String>,
}

pub(crate) fn validate_failure_evidence_bundle(
    bundle: &FailureEvidenceBundle,
) -> Result<(), Vec<String>> {
    let mut diagnostics = Vec::new();
    if bundle.schema != FAILURE_BUNDLE_SCHEMA {
        diagnostics.push(format!(
            "unexpected failure bundle schema {}",
            bundle.schema
        ));
    }
    if !matches!(
        bundle.outcome.as_str(),
        FAILURE_BUNDLE_OUTCOME_FAILED | FAILURE_BUNDLE_OUTCOME_BLOCKED
    ) {
        diagnostics.push(format!(
            "failure bundle outcome must be failed or blocked, found {}",
            bundle.outcome
        ));
    }
    if bundle.scenario.is_empty() {
        diagnostics.push("failure bundle missing scenario".to_string());
    }
    if !matches!(bundle.backend.as_str(), "paper" | "valence") {
        diagnostics.push(format!(
            "failure bundle has unsupported backend {}",
            bundle.backend
        ));
    }
    if bundle.mode.is_empty() {
        diagnostics.push("failure bundle missing mode".to_string());
    }
    if bundle.command_summary.is_empty() {
        diagnostics.push("failure bundle missing command summary".to_string());
    }
    if bundle.first_failure.is_empty() {
        diagnostics.push("failure bundle missing first failure".to_string());
    }
    if bundle.artifacts.is_empty() {
        diagnostics.push("failure bundle missing artifacts".to_string());
    }
    for artifact in &bundle.artifacts {
        validate_failure_bundle_artifact(artifact, &mut diagnostics);
    }
    for required in FAILURE_BUNDLE_REQUIRED_NON_CLAIMS {
        if !bundle.non_claims.iter().any(|claim| claim == required) {
            diagnostics.push(format!("failure bundle missing non_claim {required}"));
        }
    }
    if diagnostics.is_empty() {
        Ok(())
    } else {
        Err(diagnostics)
    }
}

fn validate_failure_bundle_artifact(
    artifact: &FailureBundleArtifact,
    diagnostics: &mut Vec<String>,
) {
    if artifact.kind.is_empty() {
        diagnostics.push("failure bundle artifact missing kind".to_string());
    }
    if let Err(err) = validate_failure_bundle_artifact_path(&artifact.path) {
        diagnostics.push(err);
    }
    if !is_blake3_hex(&artifact.blake3) {
        diagnostics.push(format!(
            "failure bundle artifact {} has malformed BLAKE3 digest",
            artifact.kind
        ));
    }
}

pub(crate) fn validate_failure_bundle_artifact_path(path: &str) -> Result<(), String> {
    if path.is_empty() || path.contains('\0') {
        return Err("failure bundle artifact path is empty or contains NUL".to_string());
    }
    let path_value = Path::new(path);
    if path_value.is_absolute() {
        return Err(format!(
            "failure bundle artifact path must be repo-relative: {path}"
        ));
    }
    for component in path_value.components() {
        match component {
            std::path::Component::ParentDir => {
                return Err(format!("failure bundle artifact path escapes repo: {path}"));
            }
            std::path::Component::Normal(value) if value == FAILURE_BUNDLE_TARGET_COMPONENT => {
                return Err(format!(
                    "failure bundle artifact path is target-only evidence: {path}"
                ));
            }
            _ => {}
        }
    }
    if !path.starts_with(FAILURE_BUNDLE_REVIEW_STORAGE_PREFIX) {
        return Err(format!(
            "failure bundle artifact path must be copied under {FAILURE_BUNDLE_REVIEW_STORAGE_PREFIX}: {path}"
        ));
    }
    Ok(())
}

pub(crate) fn is_blake3_hex(value: &str) -> bool {
    value.len() == FAILURE_BUNDLE_BLAKE3_HEX_CHARS && value.chars().all(|ch| ch.is_ascii_hexdigit())
}

pub(crate) fn failure_bundle_from_config(
    cfg: &Config,
    first_failure: &str,
    artifacts: Vec<FailureBundleArtifact>,
) -> FailureEvidenceBundle {
    FailureEvidenceBundle {
        schema: FAILURE_BUNDLE_SCHEMA.to_string(),
        outcome: FAILURE_BUNDLE_OUTCOME_FAILED.to_string(),
        scenario: scenario_name(cfg.scenario).to_string(),
        backend: backend_name(cfg.server_backend).to_string(),
        mode: mode_name(cfg.mode).to_string(),
        command_summary: bounded_failure_bundle_text(
            &failure_bundle_command_summary(cfg),
            FAILURE_BUNDLE_MAX_COMMAND_SUMMARY_CHARS,
        ),
        first_failure: bounded_failure_bundle_text(
            first_failure,
            FAILURE_BUNDLE_MAX_FIRST_FAILURE_CHARS,
        ),
        artifacts,
        non_claims: FAILURE_BUNDLE_REQUIRED_NON_CLAIMS
            .iter()
            .map(|claim| (*claim).to_string())
            .collect(),
    }
}

fn failure_bundle_command_summary(cfg: &Config) -> String {
    format!(
        "mc-compat-runner --{} --scenario {} --server-backend {}",
        mode_name(cfg.mode),
        scenario_name(cfg.scenario),
        backend_name(cfg.server_backend)
    )
}

fn bounded_failure_bundle_text(value: &str, max_chars: usize) -> String {
    value.chars().take(max_chars).collect()
}

pub(crate) fn render_failure_evidence_bundle_json(bundle: &FailureEvidenceBundle) -> String {
    format!(
        r#"{{
  "schema": {schema},
  "outcome": {outcome},
  "scenario": {scenario},
  "backend": {backend},
  "mode": {mode},
  "command_summary": {command_summary},
  "first_failure": {first_failure},
  "artifacts": {artifacts},
  "non_claims": {non_claims},
  "diagnostic_only": true,
  "claims_scenario_success": false,
  "claims_gameplay_parity": false,
  "claims_full_protocol_compatibility": false,
  "claims_public_server_safety": false,
  "claims_production_readiness": false,
  "claims_semantic_equivalence": false
}}"#,
        schema = json_string(&bundle.schema),
        outcome = json_string(&bundle.outcome),
        scenario = json_string(&bundle.scenario),
        backend = json_string(&bundle.backend),
        mode = json_string(&bundle.mode),
        command_summary = json_string(&bundle.command_summary),
        first_failure = json_string(&bundle.first_failure),
        artifacts = render_failure_bundle_artifacts_json(&bundle.artifacts),
        non_claims = json_string_vec(&bundle.non_claims),
    )
}

fn render_failure_bundle_artifacts_json(artifacts: &[FailureBundleArtifact]) -> String {
    let rendered = artifacts
        .iter()
        .map(render_failure_bundle_artifact_json)
        .collect::<Vec<_>>();
    format!("[{}]", rendered.join(", "))
}

fn render_failure_bundle_artifact_json(artifact: &FailureBundleArtifact) -> String {
    format!(
        r#"{{"kind": {kind}, "path": {path}, "blake3": {blake3}}}"#,
        kind = json_string(&artifact.kind),
        path = json_string(&artifact.path),
        blake3 = json_string(&artifact.blake3),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_BLAKE3_HEX: &str =
        "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

    #[test]
    fn failure_bundle_validator_accepts_valid_reviewable_artifacts() {
        let bundle = FailureEvidenceBundle {
            schema: FAILURE_BUNDLE_SCHEMA.to_string(),
            outcome: FAILURE_BUNDLE_OUTCOME_FAILED.to_string(),
            scenario: "smoke".to_string(),
            backend: "valence".to_string(),
            mode: "run".to_string(),
            command_summary: "mc-compat-runner --run --scenario smoke --server-backend valence"
                .to_string(),
            first_failure: "client timeout".to_string(),
            artifacts: vec![FailureBundleArtifact {
                kind: FAILURE_BUNDLE_ARTIFACT_RECEIPT.to_string(),
                path: "docs/evidence/smoke.json".to_string(),
                blake3: VALID_BLAKE3_HEX.to_string(),
            }],
            non_claims: FAILURE_BUNDLE_REQUIRED_NON_CLAIMS
                .iter()
                .map(|claim| (*claim).to_string())
                .collect(),
        };

        validate_failure_evidence_bundle(&bundle).expect("bundle validates");
        let json = render_failure_evidence_bundle_json(&bundle);
        assert!(json.contains("\"diagnostic_only\": true"), "{json}");
        assert!(json.contains("\"claims_gameplay_parity\": false"), "{json}");
    }

    #[test]
    fn failure_bundle_validator_rejects_unreviewable_artifacts_and_overclaim_gaps() {
        let bundle = FailureEvidenceBundle {
            schema: FAILURE_BUNDLE_SCHEMA.to_string(),
            outcome: FAILURE_BUNDLE_OUTCOME_FAILED.to_string(),
            scenario: "smoke".to_string(),
            backend: "valence".to_string(),
            mode: "run".to_string(),
            command_summary: "mc-compat-runner".to_string(),
            first_failure: "client timeout".to_string(),
            artifacts: vec![FailureBundleArtifact {
                kind: FAILURE_BUNDLE_ARTIFACT_RECEIPT.to_string(),
                path: "target/smoke.json".to_string(),
                blake3: "not-a-digest".to_string(),
            }],
            non_claims: Vec::new(),
        };

        let diagnostics = validate_failure_evidence_bundle(&bundle).expect_err("bundle fails");
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.contains("target-only evidence")),
            "{diagnostics:?}"
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.contains("malformed BLAKE3")),
            "{diagnostics:?}"
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.contains("missing non_claim")),
            "{diagnostics:?}"
        );
    }
}

#[cfg(test)]
#[path = "evidence_bundle_colocated_tests.rs"]
mod root_colocated_tests;
