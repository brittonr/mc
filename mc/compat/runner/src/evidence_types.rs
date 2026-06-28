//! Shared in-memory evidence data types for runner receipts and validators.
//!
//! These types describe compatibility evidence schemas. They are plain data
//! carriers: no file IO, process management, environment access, or logging.

use super::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ScenarioEvidence {
    pub(crate) observed_milestones: Vec<&'static str>,
    pub(crate) missing_milestones: Vec<&'static str>,
    pub(crate) forbidden_matches: Vec<&'static str>,
    pub(crate) passed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ServerScenarioEvidence {
    pub(crate) observed_milestones: Vec<&'static str>,
    pub(crate) missing_milestones: Vec<&'static str>,
    pub(crate) forbidden_matches: Vec<&'static str>,
    pub(crate) passed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ProjectileDamageCausalityEvidence {
    pub(crate) required_steps: Vec<&'static str>,
    pub(crate) observed_steps: Vec<&'static str>,
    pub(crate) missing_steps: Vec<&'static str>,
    pub(crate) order_violations: Vec<&'static str>,
    pub(crate) attacker_username: String,
    pub(crate) victim_username: String,
    pub(crate) passed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ProjectileTravelCollisionEvidence {
    pub(crate) selected: bool,
    pub(crate) row_id: &'static str,
    pub(crate) weapon_representative: &'static str,
    pub(crate) projectile_representative: &'static str,
    pub(crate) attacker_username: String,
    pub(crate) target_username: String,
    pub(crate) projectile_id: String,
    pub(crate) required_steps: Vec<&'static str>,
    pub(crate) observed_steps: Vec<&'static str>,
    pub(crate) missing_steps: Vec<&'static str>,
    pub(crate) order_violations: Vec<&'static str>,
    pub(crate) identity_violations: Vec<&'static str>,
    pub(crate) non_claims: Vec<&'static str>,
    pub(crate) passed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ArmorLoadoutEnchantmentStatusMatrixEvidence {
    pub(crate) selected: bool,
    pub(crate) row_id: &'static str,
    pub(crate) loadout_id: &'static str,
    pub(crate) equipment_slots: Vec<&'static str>,
    pub(crate) enchantments: Vec<&'static str>,
    pub(crate) status_effects: Vec<&'static str>,
    pub(crate) attack_type: &'static str,
    pub(crate) reference_required: bool,
    pub(crate) reference_receipt: &'static str,
    pub(crate) live_receipt: bool,
    pub(crate) promotion_ready: bool,
    pub(crate) required_client_milestones: Vec<&'static str>,
    pub(crate) observed_client_milestones: Vec<&'static str>,
    pub(crate) required_server_milestones: Vec<&'static str>,
    pub(crate) observed_server_milestones: Vec<&'static str>,
    pub(crate) non_claims: Vec<&'static str>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct EquipmentSlotItemMatrixExpansionEvidence {
    pub(crate) selected: bool,
    pub(crate) row_id: &'static str,
    pub(crate) actor_username: &'static str,
    pub(crate) observer_username: &'static str,
    pub(crate) remote_entity_id: &'static str,
    pub(crate) semantic_slot: &'static str,
    pub(crate) wire_slot: &'static str,
    pub(crate) item_id: &'static str,
    pub(crate) item_count: &'static str,
    pub(crate) transition_kind: &'static str,
    pub(crate) update_order: &'static str,
    pub(crate) reference_required: bool,
    pub(crate) reference_receipt: &'static str,
    pub(crate) live_receipt: bool,
    pub(crate) promotion_ready: bool,
    pub(crate) required_client_milestones: Vec<&'static str>,
    pub(crate) observed_client_milestones: Vec<&'static str>,
    pub(crate) required_server_milestones: Vec<&'static str>,
    pub(crate) observed_server_milestones: Vec<&'static str>,
    pub(crate) non_claims: Vec<&'static str>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct McpControlReceiptEvidence {
    pub(crate) selected: bool,
    pub(crate) endpoint_mode: &'static str,
    pub(crate) handshake_success: bool,
    pub(crate) tool_list_digest: String,
    pub(crate) tool_names: Vec<&'static str>,
    pub(crate) calls_attempted: Vec<&'static str>,
    pub(crate) calls_succeeded: Vec<&'static str>,
    pub(crate) first_failure: Option<&'static str>,
    pub(crate) stdout_clean: bool,
    pub(crate) command_outcome_ids: Vec<&'static str>,
    pub(crate) stevenarella_child_revision: Option<String>,
    pub(crate) revision_status: &'static str,
    pub(crate) dry_run_fixture: bool,
    pub(crate) live_receipt: bool,
    pub(crate) prerequisites: Vec<&'static str>,
    pub(crate) non_claims: Vec<&'static str>,
    pub(crate) passed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct McpControlRunEvidence {
    pub(crate) handshake_success: bool,
    pub(crate) tool_list_digest: String,
    pub(crate) tool_names: Vec<&'static str>,
    pub(crate) calls_attempted: Vec<&'static str>,
    pub(crate) calls_succeeded: Vec<&'static str>,
    pub(crate) first_failure: Option<&'static str>,
    pub(crate) stdout_clean: bool,
    pub(crate) command_outcome_ids: Vec<&'static str>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct FrameArtifactReceiptItem {
    pub(crate) path: String,
    pub(crate) relative_path: String,
    pub(crate) format: String,
    pub(crate) width_px: u32,
    pub(crate) height_px: u32,
    pub(crate) frame_id: u64,
    pub(crate) sequence_id: u64,
    pub(crate) byte_len: u64,
    pub(crate) blake3: String,
    pub(crate) redaction: String,
    pub(crate) includes_ui: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct FrameArtifactsReceiptEvidence {
    pub(crate) selected: bool,
    pub(crate) capture_requested: bool,
    pub(crate) artifact_count: usize,
    pub(crate) artifacts: Vec<FrameArtifactReceiptItem>,
    pub(crate) missing_digests: Vec<&'static str>,
    pub(crate) path_containment_checked: bool,
    pub(crate) promotion_ready: bool,
    pub(crate) non_claims: Vec<&'static str>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct NegativeLiveRailEvidence {
    pub(crate) selected: bool,
    pub(crate) rail: Option<&'static str>,
    pub(crate) invalid_action: Option<&'static str>,
    pub(crate) expected_outcome: Option<&'static str>,
    pub(crate) observed_outcome: Option<&'static str>,
    pub(crate) observed_outcome_source: Option<String>,
    pub(crate) postcondition_milestone: Option<&'static str>,
    pub(crate) telemetry_present: bool,
    pub(crate) target_scope: &'static str,
    pub(crate) owned_local_target: bool,
    pub(crate) explicit_authorization: bool,
    pub(crate) public_target: bool,
    pub(crate) planned_clients: usize,
    pub(crate) max_clients: usize,
    pub(crate) timeout_secs: u64,
    pub(crate) missing_fields: Vec<&'static str>,
    pub(crate) bound_violations: Vec<&'static str>,
    pub(crate) preflight_passed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct NegativeLiveRailInputs {
    pub(crate) selected: bool,
    pub(crate) rail: Option<&'static str>,
    pub(crate) invalid_action: Option<&'static str>,
    pub(crate) expected_outcome: Option<&'static str>,
    pub(crate) observed_outcome: Option<&'static str>,
    pub(crate) observed_outcome_source: Option<String>,
    pub(crate) postcondition_milestone: Option<&'static str>,
    pub(crate) telemetry_required: bool,
    pub(crate) telemetry_present: bool,
    pub(crate) target_scope: &'static str,
    pub(crate) explicit_authorization: bool,
    pub(crate) public_target: bool,
    pub(crate) planned_clients: usize,
    pub(crate) max_clients: usize,
    pub(crate) timeout_secs: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct GitRevisionEvidence {
    pub(crate) requested_rev: Option<String>,
    pub(crate) resolved_rev: Option<String>,
    pub(crate) status: &'static str,
    pub(crate) dirty: bool,
    pub(crate) diagnostics: Vec<String>,
}

impl GitRevisionEvidence {
    pub(crate) fn dry_run(requested_rev: Option<String>) -> Self {
        Self {
            requested_rev,
            resolved_rev: Some(GIT_REV_DRY_RUN_PLACEHOLDER.to_string()),
            status: GIT_STATUS_DRY_RUN,
            dirty: false,
            diagnostics: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ChildRevisionEvidence {
    pub(crate) client: GitRevisionEvidence,
    pub(crate) valence: GitRevisionEvidence,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct LoadNetworkSafetyInputs {
    pub(crate) target_scope: &'static str,
    pub(crate) owned_local_target: bool,
    pub(crate) explicit_authorization: bool,
    pub(crate) public_target: bool,
    pub(crate) planned_clients: usize,
    pub(crate) max_clients: usize,
    pub(crate) duration_secs: u64,
    pub(crate) max_duration_secs: u64,
    pub(crate) reconnect_sessions: usize,
    pub(crate) latency_ms: String,
    pub(crate) jitter_ms: String,
    pub(crate) loss_percent: String,
    pub(crate) telemetry_present: bool,
    pub(crate) live_receipt: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct LoadNetworkSafetyEvidence {
    pub(crate) target_scope: &'static str,
    pub(crate) owned_local_target: bool,
    pub(crate) explicit_authorization: bool,
    pub(crate) public_target: bool,
    pub(crate) authorized: bool,
    pub(crate) planned_clients: usize,
    pub(crate) max_clients: usize,
    pub(crate) duration_secs: u64,
    pub(crate) max_duration_secs: u64,
    pub(crate) reconnect_sessions: usize,
    pub(crate) latency_ms: String,
    pub(crate) jitter_ms: String,
    pub(crate) loss_percent: String,
    pub(crate) telemetry_present: bool,
    pub(crate) live_receipt: bool,
    pub(crate) missing_fields: Vec<&'static str>,
    pub(crate) bound_violations: Vec<&'static str>,
    pub(crate) preflight_passed: bool,
    pub(crate) promotion_ready: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ClientLogSlice<'a> {
    pub(crate) username: &'a str,
    pub(crate) output: &'a str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct EnrichedTriage {
    pub(crate) last_client_event: Option<String>,
    pub(crate) last_server_event: Option<String>,
    pub(crate) correlation_ids: Vec<String>,
    pub(crate) timeline_excerpt: Vec<String>,
    pub(crate) boundary_confidence: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct TypedEvent {
    pub(crate) schema_version: u32,
    pub(crate) source: String,
    pub(crate) scenario: String,
    pub(crate) session: String,
    pub(crate) username: Option<String>,
    pub(crate) sequence: u64,
    pub(crate) kind: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct TypedEventGraphEvaluation {
    pub(crate) observed_events: Vec<String>,
    pub(crate) missing_events: Vec<String>,
    pub(crate) forbidden_events: Vec<String>,
    pub(crate) order_violations: Vec<String>,
    pub(crate) passed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct TypedEventOracleArtifact {
    pub(crate) event_log_path: PathBuf,
    pub(crate) timeline_blake3: String,
    pub(crate) event_count: usize,
    pub(crate) contributes_to_pass_fail: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LatencyJitterTelemetryReceipt {
    pub(crate) selected: bool,
    pub(crate) mechanism: String,
    pub(crate) target_rail: String,
    pub(crate) delay_ms: String,
    pub(crate) jitter_ms: String,
    pub(crate) loss_percent: String,
    pub(crate) timeout_secs: u64,
    pub(crate) duration_secs: u64,
    pub(crate) client_count: usize,
    pub(crate) reconnect_count: u32,
    pub(crate) target_ownership: String,
    pub(crate) authorization: String,
    pub(crate) hygiene_status: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PublicServerAuthorizedSafetyReceipt {
    pub(crate) selected: bool,
    pub(crate) target_owner: String,
    pub(crate) authorization_artifact: String,
    pub(crate) target_scope: String,
    pub(crate) client_count: usize,
    pub(crate) duration_secs: u64,
    pub(crate) checkpoint_decision: String,
    pub(crate) live_traffic_enabled: bool,
}
