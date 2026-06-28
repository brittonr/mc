#![allow(unused_imports)]

use super::*;
use crate::test_support::*;
use crate::*;
use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::time::Duration;

#[test]
fn latency_jitter_receipt_renders_bounded_wan_telemetry_fields() {
    const TEST_TIMEOUT_SECS: u64 = 180;
    const TEST_CLIENT_COUNT: usize = 1;
    let receipt = LatencyJitterTelemetryReceipt {
        selected: true,
        mechanism: LATENCY_JITTER_DEFAULT_MECHANISM.to_string(),
        target_rail: "inventory-interaction".to_string(),
        delay_ms: "80".to_string(),
        jitter_ms: "30".to_string(),
        loss_percent: LATENCY_JITTER_DEFAULT_METRIC.to_string(),
        timeout_secs: TEST_TIMEOUT_SECS,
        duration_secs: TEST_TIMEOUT_SECS,
        client_count: TEST_CLIENT_COUNT,
        reconnect_count: NO_RECONNECT_SESSIONS,
        target_ownership: WAN_TARGET_OWNERSHIP_OWNED_LOCAL.to_string(),
        authorization: WAN_AUTHORIZATION_OWNED_LOCAL.to_string(),
        hygiene_status: LATENCY_JITTER_ENABLED_HYGIENE_STATUS,
    };

    let json = render_latency_jitter_receipt_json(&receipt);

    assert!(json.contains("\"selected\": true"), "{json}");
    assert!(
        json.contains("\"target_ownership\": \"owned-local-loopback\""),
        "{json}"
    );
    assert!(
        json.contains("\"authorization\": \"owned-local-fixture-approved\""),
        "{json}"
    );
    assert!(json.contains("\"duration_secs\": 180"), "{json}");
    assert!(json.contains("\"client_count\": 1"), "{json}");
    assert!(json.contains("\"reconnect_count\": 0"), "{json}");
    assert!(json.contains("\"telemetry_samples\""), "{json}");
    assert!(json.contains("\"scenario_observed_milestones\""), "{json}");
    assert!(
        json.contains("\"pass_fail_criteria\": \"inventory_interaction_client_server_milestones\""),
        "{json}"
    );
    assert!(json.contains("\"claims_wan_safety\": false"), "{json}");
    assert!(
        json.contains("\"claims_packet_loss_tolerance\": false"),
        "{json}"
    );
    assert!(
        json.contains("\"claims_public_server_safety\": false"),
        "{json}"
    );
    assert!(
        json.contains("\"claims_production_readiness\": false"),
        "{json}"
    );
}

#[test]
fn latency_jitter_receipt_disabled_path_stays_non_claim() {
    const TEST_TIMEOUT_SECS: u64 = 180;
    const TEST_CLIENT_COUNT: usize = 1;
    let receipt = LatencyJitterTelemetryReceipt {
        selected: false,
        mechanism: LATENCY_JITTER_DEFAULT_MECHANISM.to_string(),
        target_rail: "smoke".to_string(),
        delay_ms: LATENCY_JITTER_DEFAULT_METRIC.to_string(),
        jitter_ms: LATENCY_JITTER_DEFAULT_METRIC.to_string(),
        loss_percent: LATENCY_JITTER_DEFAULT_METRIC.to_string(),
        timeout_secs: TEST_TIMEOUT_SECS,
        duration_secs: TEST_TIMEOUT_SECS,
        client_count: TEST_CLIENT_COUNT,
        reconnect_count: NO_RECONNECT_SESSIONS,
        target_ownership: WAN_TARGET_OWNERSHIP_OWNED_LOCAL.to_string(),
        authorization: WAN_AUTHORIZATION_OWNED_LOCAL.to_string(),
        hygiene_status: LATENCY_JITTER_DISABLED_HYGIENE_STATUS,
    };

    let json = render_latency_jitter_receipt_json(&receipt);

    assert!(json.contains("\"selected\": false"), "{json}");
    assert!(
        json.contains("\"hygiene_status\": \"not-selected\""),
        "{json}"
    );
    assert!(
        json.contains("\"fail_closed_when_unavailable\": true"),
        "{json}"
    );
    assert!(json.contains("\"claims_wan_safety\": false"), "{json}");
    assert!(
        json.contains("\"claims_internet_path_safety\": false"),
        "{json}"
    );
}

#[test]
fn latency_jitter_reconnect_count_is_explicit() {
    assert_eq!(
        latency_jitter_reconnect_count(Scenario::InventoryInteraction),
        NO_RECONNECT_SESSIONS
    );
    assert_eq!(
        latency_jitter_reconnect_count(Scenario::ReconnectFlagState),
        SINGLE_RECONNECT_SESSION
    );
    assert_eq!(
        latency_jitter_reconnect_count(Scenario::NegativeReconnectRace),
        SINGLE_RECONNECT_SESSION
    );
}

#[test]
fn public_server_authorized_safety_receipt_renders_fixture_envelope() {
    const TEST_DURATION_SECS: u64 = 30;
    const TEST_CLIENT_COUNT: usize = 1;
    let receipt = PublicServerAuthorizedSafetyReceipt {
        selected: true,
        target_owner: PUBLIC_SERVER_DEFAULT_TARGET_OWNER.to_string(),
        authorization_artifact: PUBLIC_SERVER_DEFAULT_AUTHORIZATION_ARTIFACT.to_string(),
        target_scope: PUBLIC_SERVER_DEFAULT_TARGET_SCOPE.to_string(),
        client_count: TEST_CLIENT_COUNT,
        duration_secs: TEST_DURATION_SECS,
        checkpoint_decision: PUBLIC_SERVER_DEFAULT_CHECKPOINT_DECISION.to_string(),
        live_traffic_enabled: false,
    };

    let json = render_public_server_authorized_safety_receipt_json(&receipt);

    assert!(json.contains("\"selected\": true"), "{json}");
    assert!(
        json.contains("\"target_owner\": \"review-fixture-owner\""),
        "{json}"
    );
    assert!(
        json.contains("\"target_scope\": \"authorized-non-loopback-fixture\""),
        "{json}"
    );
    assert!(json.contains("\"client_count\": 1"), "{json}");
    assert!(json.contains("\"duration_secs\": 30"), "{json}");
    assert!(json.contains("\"status_probe_only\""), "{json}");
    assert!(json.contains("\"redaction_policy\""), "{json}");
    assert!(
        json.contains("\"checkpoint_decision\": \"approved_for_deterministic_fixture_only\""),
        "{json}"
    );
    assert!(json.contains("\"live_traffic_enabled\": false"), "{json}");
    assert!(
        json.contains("\"claims_authorized_public_envelope\": true"),
        "{json}"
    );
    assert!(
        json.contains("\"claims_live_public_server_safety\": false"),
        "{json}"
    );
    assert!(
        json.contains("\"claims_production_readiness\": false"),
        "{json}"
    );
    assert!(json.contains("\"claims_wan_tolerance\": false"), "{json}");
}

#[test]
fn public_server_authorized_safety_live_mode_fails_closed() {
    assert!(public_server_authorized_safety_selected(true, Mode::DryRun));
    assert!(!public_server_authorized_safety_selected(true, Mode::Run));
    assert!(!public_server_authorized_safety_selected(
        false,
        Mode::DryRun
    ));
}

#[test]
fn public_server_authorized_safety_disabled_path_stays_non_claim() {
    const TEST_DURATION_SECS: u64 = 30;
    const TEST_CLIENT_COUNT: usize = 1;
    let receipt = PublicServerAuthorizedSafetyReceipt {
        selected: false,
        target_owner: PUBLIC_SERVER_DEFAULT_TARGET_OWNER.to_string(),
        authorization_artifact: PUBLIC_SERVER_DEFAULT_AUTHORIZATION_ARTIFACT.to_string(),
        target_scope: PUBLIC_SERVER_DEFAULT_TARGET_SCOPE.to_string(),
        client_count: TEST_CLIENT_COUNT,
        duration_secs: TEST_DURATION_SECS,
        checkpoint_decision: PUBLIC_SERVER_DEFAULT_CHECKPOINT_DECISION.to_string(),
        live_traffic_enabled: false,
    };

    let json = render_public_server_authorized_safety_receipt_json(&receipt);

    assert!(json.contains("\"selected\": false"), "{json}");
    assert!(
        json.contains("\"claims_authorized_public_envelope\": false"),
        "{json}"
    );
    assert!(
        json.contains("\"claims_live_public_server_safety\": false"),
        "{json}"
    );
    assert!(
        json.contains("\"claims_third_party_target_safety_without_authorization\": false"),
        "{json}"
    );
}

fn baseline_negative_live_rail_inputs() -> NegativeLiveRailInputs {
    NegativeLiveRailInputs {
        selected: true,
        rail: Some("negative-custom-payload"),
        invalid_action: Some("malformed_custom_payload"),
        expected_outcome: Some(NEGATIVE_LIVE_RAIL_EXPECTED_OUTCOME),
        observed_outcome: Some(NEGATIVE_LIVE_RAIL_OBSERVED_OUTCOME_CONTAINMENT),
        observed_outcome_source: Some(
            "client_milestone:negative_custom_payload_contained".to_string(),
        ),
        postcondition_milestone: Some("negative_custom_payload_contained"),
        telemetry_required: true,
        telemetry_present: true,
        target_scope: SAFETY_OWNED_LOCAL_SCOPE,
        explicit_authorization: false,
        public_target: false,
        planned_clients: 1,
        max_clients: NEGATIVE_LIVE_RAIL_MAX_CLIENTS,
        timeout_secs: 20,
    }
}

#[test]
fn negative_live_rail_checker_rejects_unbounded_public_unauthenticated_inputs() {
    let mut inputs = baseline_negative_live_rail_inputs();
    inputs.public_target = true;
    inputs.planned_clients = NEGATIVE_LIVE_RAIL_MAX_CLIENTS + 1;
    let evidence = evaluate_negative_live_rail_safety_from_inputs(inputs);
    assert!(!evidence.preflight_passed, "{evidence:?}");
    assert!(evidence
        .bound_violations
        .contains(&"public_target_without_authorization"));
    assert!(evidence
        .bound_violations
        .contains(&"planned_clients_exceed_negative_max"));
}

#[test]
fn negative_live_rail_checker_rejects_missing_telemetry() {
    let mut inputs = baseline_negative_live_rail_inputs();
    inputs.telemetry_present = false;
    inputs.observed_outcome = None;
    inputs.observed_outcome_source = None;
    let evidence = evaluate_negative_live_rail_safety_from_inputs(inputs);
    assert!(!evidence.preflight_passed, "{evidence:?}");
    assert!(evidence.missing_fields.contains(&"telemetry"));
}

#[test]
fn negative_live_rail_checker_rejects_missing_expected_outcome() {
    let mut inputs = baseline_negative_live_rail_inputs();
    inputs.expected_outcome = None;
    let evidence = evaluate_negative_live_rail_safety_from_inputs(inputs);
    assert!(!evidence.preflight_passed, "{evidence:?}");
    assert!(evidence.missing_fields.contains(&"expected_outcome"));
}

#[test]
fn negative_live_rail_preflight_rejects_public_unowned_targets() {
    let cfg = test_config(
        &["--dry-run", "--scenario", "negative-custom-payload"],
        &[("MC_COMPAT_PUBLIC_TARGET", "1")],
    )
    .expect("negative rail config parses");
    let err = validate_negative_live_rail_preflight(&cfg)
        .expect_err("public negative rail without authorization fails");
    assert!(err.contains("public_target_without_authorization"), "{err}");
}

#[test]
fn load_network_safety_envelope_fails_closed_for_unsafe_inputs() {
    let safe = evaluate_load_network_safety(LoadNetworkSafetyInputs {
        target_scope: SAFETY_OWNED_LOCAL_SCOPE,
        owned_local_target: true,
        explicit_authorization: false,
        public_target: false,
        planned_clients: SAFETY_MAX_LOCAL_CLIENTS,
        max_clients: SAFETY_MAX_LOCAL_CLIENTS,
        duration_secs: SAFETY_MAX_DURATION_SECS,
        max_duration_secs: SAFETY_MAX_DURATION_SECS,
        reconnect_sessions: SAFETY_SINGLE_SESSION_COUNT,
        latency_ms: SAFETY_ZERO_VALUE.to_string(),
        jitter_ms: SAFETY_ZERO_VALUE.to_string(),
        loss_percent: SAFETY_ZERO_VALUE.to_string(),
        telemetry_present: true,
        live_receipt: true,
    });
    assert!(safe.preflight_passed, "{safe:?}");
    assert!(safe.promotion_ready, "{safe:?}");

    let unsafe_public = evaluate_load_network_safety(LoadNetworkSafetyInputs {
        target_scope: "public",
        owned_local_target: false,
        explicit_authorization: false,
        public_target: true,
        planned_clients: SAFETY_MAX_LOCAL_CLIENTS + 1,
        max_clients: SAFETY_MAX_LOCAL_CLIENTS,
        duration_secs: SAFETY_MAX_DURATION_SECS + 1,
        max_duration_secs: SAFETY_MAX_DURATION_SECS,
        reconnect_sessions: SAFETY_SINGLE_SESSION_COUNT,
        latency_ms: String::new(),
        jitter_ms: SAFETY_ZERO_VALUE.to_string(),
        loss_percent: SAFETY_ZERO_VALUE.to_string(),
        telemetry_present: false,
        live_receipt: false,
    });
    assert!(!unsafe_public.preflight_passed, "{unsafe_public:?}");
    assert!(!unsafe_public.promotion_ready, "{unsafe_public:?}");
    assert!(unsafe_public.missing_fields.contains(&"latency_ms"));
    assert!(unsafe_public
        .bound_violations
        .contains(&"public_target_without_authorization"));
    assert!(unsafe_public
        .bound_violations
        .contains(&"planned_clients_exceed_max"));
    assert!(unsafe_public
        .bound_violations
        .contains(&"duration_exceeds_max"));
}
