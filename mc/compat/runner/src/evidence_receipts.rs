//! Pure receipt evidence evaluation and rendering helpers.
//!
//! Config/environment collection, file writes, hashing, process control, and
//! diagnostics stay in `main.rs`. This module accepts typed in-memory inputs and
//! returns typed evidence or rendered JSON snippets.

use super::*;

pub(crate) fn evaluate_load_network_safety(
    input: LoadNetworkSafetyInputs,
) -> LoadNetworkSafetyEvidence {
    let authorized = input.owned_local_target || input.explicit_authorization;
    let mut missing_fields = Vec::new();
    push_missing_safety_field(
        &mut missing_fields,
        "target_scope",
        !input.target_scope.is_empty(),
    );
    push_missing_safety_field(
        &mut missing_fields,
        "latency_ms",
        !input.latency_ms.is_empty(),
    );
    push_missing_safety_field(
        &mut missing_fields,
        "jitter_ms",
        !input.jitter_ms.is_empty(),
    );
    push_missing_safety_field(
        &mut missing_fields,
        "loss_percent",
        !input.loss_percent.is_empty(),
    );

    let mut bound_violations = Vec::new();
    if input.public_target && !input.explicit_authorization {
        bound_violations.push("public_target_without_authorization");
    }
    if input.planned_clients == 0 {
        bound_violations.push("planned_clients_empty");
    }
    if input.planned_clients > input.max_clients {
        bound_violations.push("planned_clients_exceed_max");
    }
    if input.duration_secs == 0 {
        bound_violations.push("duration_empty");
    }
    if input.duration_secs > input.max_duration_secs {
        bound_violations.push("duration_exceeds_max");
    }

    let preflight_passed = authorized && missing_fields.is_empty() && bound_violations.is_empty();
    let promotion_ready = preflight_passed && input.telemetry_present && input.live_receipt;
    LoadNetworkSafetyEvidence {
        target_scope: input.target_scope,
        owned_local_target: input.owned_local_target,
        explicit_authorization: input.explicit_authorization,
        public_target: input.public_target,
        authorized,
        planned_clients: input.planned_clients,
        max_clients: input.max_clients,
        duration_secs: input.duration_secs,
        max_duration_secs: input.max_duration_secs,
        reconnect_sessions: input.reconnect_sessions,
        latency_ms: input.latency_ms,
        jitter_ms: input.jitter_ms,
        loss_percent: input.loss_percent,
        telemetry_present: input.telemetry_present,
        live_receipt: input.live_receipt,
        missing_fields,
        bound_violations,
        preflight_passed,
        promotion_ready,
    }
}

fn push_missing_safety_field(
    missing_fields: &mut Vec<&'static str>,
    field: &'static str,
    present: bool,
) {
    if !present {
        missing_fields.push(field);
    }
}

pub(crate) fn evaluate_negative_live_rail_safety_from_inputs(
    input: NegativeLiveRailInputs,
) -> NegativeLiveRailEvidence {
    let owned_local_target = !input.public_target;
    let mut missing_fields = Vec::new();
    if input.selected {
        push_missing_safety_field(
            &mut missing_fields,
            "invalid_action",
            input.invalid_action.is_some(),
        );
        push_missing_safety_field(
            &mut missing_fields,
            "expected_outcome",
            input.expected_outcome.is_some(),
        );
        push_missing_safety_field(
            &mut missing_fields,
            "target_scope",
            !input.target_scope.is_empty(),
        );
        push_missing_safety_field(
            &mut missing_fields,
            "postcondition_milestone",
            input.postcondition_milestone.is_some(),
        );
        if input.telemetry_required {
            push_missing_safety_field(
                &mut missing_fields,
                "telemetry",
                input.telemetry_present && input.observed_outcome.is_some(),
            );
        }
    }
    let mut bound_violations = Vec::new();
    if input.selected && input.public_target && !input.explicit_authorization {
        bound_violations.push("public_target_without_authorization");
    }
    if input.selected && input.planned_clients == 0 {
        bound_violations.push("planned_clients_empty");
    }
    if input.selected && input.planned_clients > input.max_clients {
        bound_violations.push("planned_clients_exceed_negative_max");
    }
    if input.selected && input.timeout_secs < NEGATIVE_LIVE_RAIL_MIN_TIMEOUT_SECS {
        bound_violations.push("timeout_empty");
    }
    let preflight_passed = !input.selected
        || ((owned_local_target || input.explicit_authorization)
            && missing_fields.is_empty()
            && bound_violations.is_empty());
    NegativeLiveRailEvidence {
        selected: input.selected,
        rail: input.rail,
        invalid_action: input.invalid_action,
        expected_outcome: input.expected_outcome,
        observed_outcome: input.observed_outcome,
        observed_outcome_source: input.observed_outcome_source,
        postcondition_milestone: input.postcondition_milestone,
        telemetry_present: input.telemetry_present,
        target_scope: input.target_scope,
        owned_local_target,
        explicit_authorization: input.explicit_authorization,
        public_target: input.public_target,
        planned_clients: input.planned_clients,
        max_clients: input.max_clients,
        timeout_secs: input.timeout_secs,
        missing_fields,
        bound_violations,
        preflight_passed,
    }
}

pub(crate) fn evaluate_armor_loadout_enchantment_status_matrix(
    cfg: &Config,
    scenario: &ScenarioEvidence,
    server_scenario: &ServerScenarioEvidence,
) -> ArmorLoadoutEnchantmentStatusMatrixEvidence {
    let selected = cfg.scenario == Scenario::ArmorLoadoutEnchantmentStatusMatrix;
    let observed_live_evidence =
        selected && cfg.mode == Mode::Run && scenario.passed && server_scenario.passed;
    ArmorLoadoutEnchantmentStatusMatrixEvidence {
        selected,
        row_id: ARMOR_MATRIX_ROW_ID,
        loadout_id: ARMOR_MATRIX_LOADOUT_ID,
        equipment_slots: vec![ARMOR_MATRIX_EQUIPMENT_SLOT],
        enchantments: vec![ARMOR_MATRIX_ENCHANTMENT_NONE],
        status_effects: vec![ARMOR_MATRIX_STATUS_EFFECT_NONE],
        attack_type: ARMOR_MATRIX_ATTACK_TYPE_MELEE,
        reference_required: false,
        reference_receipt: ARMOR_MATRIX_REFERENCE_RECEIPT_NONE,
        live_receipt: observed_live_evidence,
        promotion_ready: observed_live_evidence,
        required_client_milestones: scenario_required_milestones(cfg.scenario)
            .iter()
            .map(|(milestone, _)| *milestone)
            .collect(),
        observed_client_milestones: scenario.observed_milestones.clone(),
        required_server_milestones: server_required_milestones(cfg.scenario)
            .iter()
            .map(|(milestone, _)| *milestone)
            .collect(),
        observed_server_milestones: server_scenario.observed_milestones.clone(),
        non_claims: ARMOR_MATRIX_NON_CLAIMS.to_vec(),
    }
}

pub(crate) fn evaluate_equipment_slot_item_matrix_expansion(
    cfg: &Config,
    scenario: &ScenarioEvidence,
    server_scenario: &ServerScenarioEvidence,
) -> EquipmentSlotItemMatrixExpansionEvidence {
    let selected = cfg.scenario == Scenario::EquipmentSlotItemMatrixExpansion;
    let observed_live_evidence =
        selected && cfg.mode == Mode::Run && scenario.passed && server_scenario.passed;
    EquipmentSlotItemMatrixExpansionEvidence {
        selected,
        row_id: EQUIPMENT_MATRIX_ROW_ID,
        actor_username: EQUIPMENT_MATRIX_ACTOR,
        observer_username: EQUIPMENT_MATRIX_OBSERVER,
        remote_entity_id: EQUIPMENT_MATRIX_REMOTE_ENTITY_ID,
        semantic_slot: EQUIPMENT_MATRIX_SEMANTIC_SLOT,
        wire_slot: EQUIPMENT_MATRIX_WIRE_SLOT,
        item_id: EQUIPMENT_MATRIX_ITEM_ID,
        item_count: EQUIPMENT_MATRIX_ITEM_COUNT,
        transition_kind: EQUIPMENT_MATRIX_TRANSITION,
        update_order: EQUIPMENT_MATRIX_UPDATE_ORDER,
        reference_required: false,
        reference_receipt: EQUIPMENT_MATRIX_REFERENCE_RECEIPT_NONE,
        live_receipt: observed_live_evidence,
        promotion_ready: observed_live_evidence,
        required_client_milestones: scenario_required_milestones(cfg.scenario)
            .iter()
            .map(|(milestone, _)| *milestone)
            .collect(),
        observed_client_milestones: scenario.observed_milestones.clone(),
        required_server_milestones: server_required_milestones(cfg.scenario)
            .iter()
            .map(|(milestone, _)| *milestone)
            .collect(),
        observed_server_milestones: server_scenario.observed_milestones.clone(),
        non_claims: EQUIPMENT_MATRIX_NON_CLAIMS.to_vec(),
    }
}

pub(crate) fn render_latency_jitter_receipt_json(
    receipt: &LatencyJitterTelemetryReceipt,
) -> String {
    format!(
        r#"{{
    "selected": {selected},
    "mechanism": {mechanism},
    "target_rail": {target_rail},
    "delay_ms": {delay_ms},
    "jitter_ms": {jitter_ms},
    "loss_percent": {loss_percent},
    "timeout_secs": {timeout_secs},
    "duration_secs": {duration_secs},
    "client_count": {client_count},
    "reconnect_count": {reconnect_count},
    "target_ownership": {target_ownership},
    "authorization": {authorization},
    "telemetry_samples": {telemetry_samples},
    "pass_fail_criteria": {pass_fail_criteria},
    "abort_reason": {abort_reason},
    "hygiene_status": {hygiene_status},
    "privileged_network_mutation_required": false,
    "fail_closed_when_unavailable": true,
    "claims_wan_safety": false,
    "claims_packet_loss_tolerance": false,
    "claims_internet_path_safety": false,
    "claims_adversarial_network_safety": false,
    "claims_public_server_safety": false,
    "claims_production_readiness": false
  }}"#,
        selected = if receipt.selected { "true" } else { "false" },
        mechanism = json_string(&receipt.mechanism),
        target_rail = json_string(&receipt.target_rail),
        delay_ms = json_string(&receipt.delay_ms),
        jitter_ms = json_string(&receipt.jitter_ms),
        loss_percent = json_string(&receipt.loss_percent),
        timeout_secs = receipt.timeout_secs,
        duration_secs = receipt.duration_secs,
        client_count = receipt.client_count,
        reconnect_count = receipt.reconnect_count,
        target_ownership = json_string(&receipt.target_ownership),
        authorization = json_string(&receipt.authorization),
        telemetry_samples = json_string_array(WAN_TELEMETRY_SAMPLES),
        pass_fail_criteria = json_string(WAN_PASS_FAIL_CRITERIA),
        abort_reason = json_string(WAN_ABORT_REASON_NONE),
        hygiene_status = json_string(receipt.hygiene_status),
    )
}

pub(crate) fn render_public_server_authorized_safety_receipt_json(
    receipt: &PublicServerAuthorizedSafetyReceipt,
) -> String {
    format!(
        r#"{{
    "selected": {selected},
    "target_owner": {target_owner},
    "authorization_artifact": {authorization_artifact},
    "target_scope": {target_scope},
    "client_count": {client_count},
    "duration_secs": {duration_secs},
    "traffic_limits": {traffic_limits},
    "telemetry_fields": {telemetry_fields},
    "abort_criteria": {abort_criteria},
    "redaction_policy": {redaction_policy},
    "checkpoint_decision": {checkpoint_decision},
    "live_traffic_enabled": {live_traffic_enabled},
    "fixture_only": true,
    "claims_authorized_public_envelope": {claims_authorized_public_envelope},
    "claims_live_public_server_safety": false,
    "claims_third_party_target_safety_without_authorization": false,
    "claims_production_readiness": false,
    "claims_adversarial_safety": false,
    "claims_wan_tolerance": false,
    "claims_load_safety_beyond_configured_bounds": false,
    "claims_unbounded_public_testing": false
  }}"#,
        selected = receipt.selected,
        target_owner = json_string(&receipt.target_owner),
        authorization_artifact = json_string(&receipt.authorization_artifact),
        target_scope = json_string(&receipt.target_scope),
        client_count = receipt.client_count,
        duration_secs = receipt.duration_secs,
        traffic_limits = json_string_array(PUBLIC_SERVER_TRAFFIC_LIMITS),
        telemetry_fields = json_string_array(PUBLIC_SERVER_TELEMETRY_FIELDS),
        abort_criteria = json_string(PUBLIC_SERVER_ABORT_CRITERIA),
        redaction_policy = json_string(PUBLIC_SERVER_REDACTION_POLICY),
        checkpoint_decision = json_string(&receipt.checkpoint_decision),
        live_traffic_enabled = receipt.live_traffic_enabled,
        claims_authorized_public_envelope = receipt.selected,
    )
}

pub(crate) fn render_negative_live_rail_json(evidence: &NegativeLiveRailEvidence) -> String {
    format!(
        r#"{{
    "selected": {selected},
    "rail": {rail},
    "invalid_action": {invalid_action},
    "expected_outcome": {expected_outcome},
    "observed_outcome": {observed_outcome},
    "observed_outcome_source": {observed_outcome_source},
    "postcondition_milestone": {postcondition_milestone},
    "telemetry_present": {telemetry_present},
    "target_scope": {target_scope},
    "owned_local_target": {owned_local_target},
    "explicit_authorization": {explicit_authorization},
    "public_target": {public_target},
    "planned_clients": {planned_clients},
    "max_clients": {max_clients},
    "timeout_secs": {timeout_secs},
    "evidence_fields": {evidence_fields},
    "missing_fields": {missing_fields},
    "bound_violations": {bound_violations},
    "preflight_passed": {preflight_passed},
    "non_claims": {non_claims}
  }}"#,
        selected = evidence.selected,
        rail = json_optional_string(evidence.rail),
        invalid_action = json_optional_string(evidence.invalid_action),
        expected_outcome = json_optional_string(evidence.expected_outcome),
        observed_outcome = json_optional_string(evidence.observed_outcome),
        observed_outcome_source = json_optional_string(evidence.observed_outcome_source.as_deref()),
        postcondition_milestone = json_optional_string(evidence.postcondition_milestone),
        telemetry_present = evidence.telemetry_present,
        target_scope = json_string(evidence.target_scope),
        owned_local_target = evidence.owned_local_target,
        explicit_authorization = evidence.explicit_authorization,
        public_target = evidence.public_target,
        planned_clients = evidence.planned_clients,
        max_clients = evidence.max_clients,
        timeout_secs = evidence.timeout_secs,
        evidence_fields = json_string_array(NEGATIVE_LIVE_RAIL_EVIDENCE_FIELDS),
        missing_fields = json_string_array(&evidence.missing_fields),
        bound_violations = json_string_array(&evidence.bound_violations),
        preflight_passed = evidence.preflight_passed,
        non_claims = json_string_array(NEGATIVE_LIVE_RAIL_NON_CLAIMS),
    )
}

pub(crate) fn render_armor_loadout_enchantment_status_matrix_json(
    evidence: &ArmorLoadoutEnchantmentStatusMatrixEvidence,
) -> String {
    format!(
        r#"{{
    "selected": {selected},
    "row_id": {row_id},
    "loadout_id": {loadout_id},
    "equipment_slots": {equipment_slots},
    "enchantments": {enchantments},
    "status_effects": {status_effects},
    "attack_type": {attack_type},
    "reference_required": {reference_required},
    "reference_receipt": {reference_receipt},
    "live_receipt": {live_receipt},
    "promotion_ready": {promotion_ready},
    "required_client_milestones": {required_client_milestones},
    "observed_client_milestones": {observed_client_milestones},
    "required_server_milestones": {required_server_milestones},
    "observed_server_milestones": {observed_server_milestones},
    "non_claims": {non_claims}
  }}"#,
        selected = evidence.selected,
        row_id = json_string(evidence.row_id),
        loadout_id = json_string(evidence.loadout_id),
        equipment_slots = json_string_array(&evidence.equipment_slots),
        enchantments = json_string_array(&evidence.enchantments),
        status_effects = json_string_array(&evidence.status_effects),
        attack_type = json_string(evidence.attack_type),
        reference_required = evidence.reference_required,
        reference_receipt = json_string(evidence.reference_receipt),
        live_receipt = evidence.live_receipt,
        promotion_ready = evidence.promotion_ready,
        required_client_milestones = json_string_array(&evidence.required_client_milestones),
        observed_client_milestones = json_string_array(&evidence.observed_client_milestones),
        required_server_milestones = json_string_array(&evidence.required_server_milestones),
        observed_server_milestones = json_string_array(&evidence.observed_server_milestones),
        non_claims = json_string_array(&evidence.non_claims),
    )
}

pub(crate) fn render_equipment_slot_item_matrix_expansion_json(
    evidence: &EquipmentSlotItemMatrixExpansionEvidence,
) -> String {
    format!(
        r#"{{
    "selected": {selected},
    "row_id": {row_id},
    "actor_username": {actor_username},
    "observer_username": {observer_username},
    "remote_entity_id": {remote_entity_id},
    "semantic_slot": {semantic_slot},
    "wire_slot": {wire_slot},
    "item_id": {item_id},
    "item_count": {item_count},
    "transition_kind": {transition_kind},
    "update_order": {update_order},
    "reference_required": {reference_required},
    "reference_receipt": {reference_receipt},
    "live_receipt": {live_receipt},
    "promotion_ready": {promotion_ready},
    "required_client_milestones": {required_client_milestones},
    "observed_client_milestones": {observed_client_milestones},
    "required_server_milestones": {required_server_milestones},
    "observed_server_milestones": {observed_server_milestones},
    "non_claims": {non_claims}
  }}"#,
        selected = evidence.selected,
        row_id = json_string(evidence.row_id),
        actor_username = json_string(evidence.actor_username),
        observer_username = json_string(evidence.observer_username),
        remote_entity_id = json_string(evidence.remote_entity_id),
        semantic_slot = json_string(evidence.semantic_slot),
        wire_slot = json_string(evidence.wire_slot),
        item_id = json_string(evidence.item_id),
        item_count = json_string(evidence.item_count),
        transition_kind = json_string(evidence.transition_kind),
        update_order = json_string(evidence.update_order),
        reference_required = evidence.reference_required,
        reference_receipt = json_string(evidence.reference_receipt),
        live_receipt = evidence.live_receipt,
        promotion_ready = evidence.promotion_ready,
        required_client_milestones = json_string_array(&evidence.required_client_milestones),
        observed_client_milestones = json_string_array(&evidence.observed_client_milestones),
        required_server_milestones = json_string_array(&evidence.required_server_milestones),
        observed_server_milestones = json_string_array(&evidence.observed_server_milestones),
        non_claims = json_string_array(&evidence.non_claims),
    )
}

pub(crate) fn render_load_network_safety_json(evidence: &LoadNetworkSafetyEvidence) -> String {
    format!(
        r#"{{
    "target_scope": {target_scope},
    "owned_local_target": {owned_local_target},
    "explicit_authorization": {explicit_authorization},
    "public_target": {public_target},
    "authorized": {authorized},
    "planned_clients": {planned_clients},
    "max_clients": {max_clients},
    "duration_secs": {duration_secs},
    "max_duration_secs": {max_duration_secs},
    "reconnect_sessions": {reconnect_sessions},
    "latency_ms": {latency_ms},
    "jitter_ms": {jitter_ms},
    "loss_percent": {loss_percent},
    "telemetry_present": {telemetry_present},
    "live_receipt": {live_receipt},
    "missing_fields": {missing_fields},
    "bound_violations": {bound_violations},
    "preflight_passed": {preflight_passed},
    "promotion_ready": {promotion_ready},
    "claims_public_server_safety": false,
    "claims_production_readiness": false,
    "claims_unbounded_soak": false,
    "claims_unbounded_reconnect": false,
    "claims_wan_safety": false,
    "claims_adversarial_network_safety": false
  }}"#,
        target_scope = json_string(evidence.target_scope),
        owned_local_target = evidence.owned_local_target,
        explicit_authorization = evidence.explicit_authorization,
        public_target = evidence.public_target,
        authorized = evidence.authorized,
        planned_clients = evidence.planned_clients,
        max_clients = evidence.max_clients,
        duration_secs = evidence.duration_secs,
        max_duration_secs = evidence.max_duration_secs,
        reconnect_sessions = evidence.reconnect_sessions,
        latency_ms = json_string(&evidence.latency_ms),
        jitter_ms = json_string(&evidence.jitter_ms),
        loss_percent = json_string(&evidence.loss_percent),
        telemetry_present = evidence.telemetry_present,
        live_receipt = evidence.live_receipt,
        missing_fields = json_string_array(&evidence.missing_fields),
        bound_violations = json_string_array(&evidence.bound_violations),
        preflight_passed = evidence.preflight_passed,
        promotion_ready = evidence.promotion_ready,
    )
}

pub(crate) fn mcp_control_tool_list_digest() -> String {
    blake3::hash(
        MCP_CONTROL_TOOL_NAMES
            .join(MCP_CONTROL_TOOL_LIST_DIGEST_SEPARATOR)
            .as_bytes(),
    )
    .to_hex()
    .to_string()
}

pub(crate) fn evaluate_mcp_control_receipt(
    cfg: &Config,
    child_revision: &GitRevisionEvidence,
    client: Option<&ClientRunEvidence>,
) -> McpControlReceiptEvidence {
    let selected = scenario_behavior(cfg.scenario).is_mcp_controlled_smoke();
    if !selected {
        return McpControlReceiptEvidence {
            selected,
            endpoint_mode: MCP_CONTROL_ENDPOINT_STDIO,
            handshake_success: false,
            tool_list_digest: String::new(),
            tool_names: Vec::new(),
            calls_attempted: Vec::new(),
            calls_succeeded: Vec::new(),
            first_failure: None,
            stdout_clean: false,
            command_outcome_ids: Vec::new(),
            stevenarella_child_revision: None,
            revision_status: child_revision.status,
            dry_run_fixture: false,
            live_receipt: false,
            prerequisites: Vec::new(),
            non_claims: MCP_CONTROL_NON_CLAIMS.to_vec(),
            passed: true,
        };
    }

    let dry_run_fixture = cfg.mode == Mode::DryRun;
    let live_receipt = cfg.mode == Mode::Run;
    let fallback;
    let run_evidence = if dry_run_fixture {
        fallback = mcp_control_dry_run_control_evidence();
        Some(&fallback)
    } else {
        client.and_then(|evidence| evidence.mcp_control.as_ref())
    };
    let revision_failure = mcp_control_revision_failure(child_revision, dry_run_fixture);
    let first_failure = match run_evidence {
        Some(evidence) => evidence.first_failure.or(revision_failure),
        None if live_receipt => Some(MCP_CONTROL_FAILURE_LIVE_EVIDENCE_MISSING),
        None => revision_failure,
    };
    let required_calls_present = run_evidence
        .map(|evidence| {
            MCP_CONTROL_REQUIRED_CALLS
                .iter()
                .all(|call| evidence.calls_succeeded.contains(call))
        })
        .unwrap_or(false);
    let required_outcomes_present = run_evidence
        .map(|evidence| {
            MCP_CONTROL_REQUIRED_OUTCOME_IDS
                .iter()
                .all(|outcome| evidence.command_outcome_ids.contains(outcome))
        })
        .unwrap_or(false);
    let revision_promotable = mcp_control_revision_promotable(child_revision, dry_run_fixture);
    let passed = run_evidence
        .map(|evidence| {
            evidence.handshake_success
                && evidence.stdout_clean
                && evidence.first_failure.is_none()
                && required_calls_present
                && required_outcomes_present
                && revision_promotable
        })
        .unwrap_or(false);
    McpControlReceiptEvidence {
        selected,
        endpoint_mode: MCP_CONTROL_ENDPOINT_STDIO,
        handshake_success: run_evidence
            .map(|evidence| evidence.handshake_success)
            .unwrap_or(false),
        tool_list_digest: run_evidence
            .map(|evidence| evidence.tool_list_digest.clone())
            .unwrap_or_else(mcp_control_tool_list_digest),
        tool_names: run_evidence
            .map(|evidence| evidence.tool_names.clone())
            .unwrap_or_else(|| MCP_CONTROL_TOOL_NAMES.to_vec()),
        calls_attempted: run_evidence
            .map(|evidence| evidence.calls_attempted.clone())
            .unwrap_or_else(|| MCP_CONTROL_LIVE_CALLS.to_vec()),
        calls_succeeded: run_evidence
            .map(|evidence| evidence.calls_succeeded.clone())
            .unwrap_or_default(),
        first_failure,
        stdout_clean: run_evidence
            .map(|evidence| evidence.stdout_clean)
            .unwrap_or(false),
        command_outcome_ids: run_evidence
            .map(|evidence| evidence.command_outcome_ids.clone())
            .unwrap_or_default(),
        stevenarella_child_revision: child_revision.resolved_rev.clone(),
        revision_status: child_revision.status,
        dry_run_fixture,
        live_receipt,
        prerequisites: MCP_CONTROL_PREREQUISITES.to_vec(),
        non_claims: MCP_CONTROL_NON_CLAIMS.to_vec(),
        passed,
    }
}

fn mcp_control_revision_failure(
    child_revision: &GitRevisionEvidence,
    dry_run_fixture: bool,
) -> Option<&'static str> {
    if dry_run_fixture {
        return None;
    }
    match child_revision.status {
        GIT_STATUS_CLEAN => None,
        GIT_STATUS_DIRTY => Some(MCP_CONTROL_FAILURE_REVISION_DIRTY),
        _ => Some(MCP_CONTROL_FAILURE_REVISION_UNAVAILABLE),
    }
}

fn mcp_control_revision_promotable(
    child_revision: &GitRevisionEvidence,
    dry_run_fixture: bool,
) -> bool {
    if dry_run_fixture {
        return child_revision.resolved_rev.is_some();
    }
    child_revision.resolved_rev.is_some() && child_revision.status == GIT_STATUS_CLEAN
}

pub(crate) fn render_mcp_control_receipt_json(evidence: &McpControlReceiptEvidence) -> String {
    let first_failure_json = json_optional_string(evidence.first_failure);
    let child_revision_json = json_optional_string(evidence.stevenarella_child_revision.as_deref());
    format!(
        r#"{{
    "selected": {selected},
    "endpoint_mode": {endpoint_mode_json},
    "handshake_success": {handshake_success},
    "tool_list_digest": {tool_list_digest_json},
    "tool_names": {tool_names_json},
    "calls_attempted": {calls_attempted_json},
    "calls_succeeded": {calls_succeeded_json},
    "first_failure": {first_failure_json},
    "stdout_clean": {stdout_clean},
    "command_outcome_ids": {command_outcome_ids_json},
    "stevenarella_child_revision": {child_revision_json},
    "revision_status": {revision_status_json},
    "dry_run_fixture": {dry_run_fixture},
    "live_receipt": {live_receipt},
    "prerequisites": {prerequisites_json},
    "non_claims": {non_claims_json},
    "passed": {passed}
  }}"#,
        selected = evidence.selected,
        endpoint_mode_json = json_string(evidence.endpoint_mode),
        handshake_success = evidence.handshake_success,
        tool_list_digest_json = json_string(&evidence.tool_list_digest),
        tool_names_json = json_string_array(&evidence.tool_names),
        calls_attempted_json = json_string_array(&evidence.calls_attempted),
        calls_succeeded_json = json_string_array(&evidence.calls_succeeded),
        first_failure_json = first_failure_json,
        stdout_clean = evidence.stdout_clean,
        command_outcome_ids_json = json_string_array(&evidence.command_outcome_ids),
        child_revision_json = child_revision_json,
        revision_status_json = json_string(evidence.revision_status),
        dry_run_fixture = evidence.dry_run_fixture,
        live_receipt = evidence.live_receipt,
        prerequisites_json = json_string_array(&evidence.prerequisites),
        non_claims_json = json_string_array(&evidence.non_claims),
        passed = evidence.passed,
    )
}

pub(crate) fn evaluate_frame_artifacts_receipt(
    cfg: &Config,
    client: Option<&ClientRunEvidence>,
) -> FrameArtifactsReceiptEvidence {
    if let Some(frame_artifacts) = client.and_then(|evidence| evidence.frame_artifacts.as_ref()) {
        return frame_artifacts.clone();
    }
    FrameArtifactsReceiptEvidence {
        selected: false,
        capture_requested: scenario_behavior(cfg.scenario).is_mcp_controlled_smoke(),
        artifact_count: 0,
        artifacts: Vec::new(),
        missing_digests: Vec::new(),
        path_containment_checked: true,
        promotion_ready: false,
        non_claims: FRAME_ARTIFACT_NON_CLAIMS.to_vec(),
    }
}

pub(crate) fn render_frame_artifacts_receipt_json(
    evidence: &FrameArtifactsReceiptEvidence,
) -> String {
    format!(
        r#"{{
    "selected": {selected},
    "capture_requested": {capture_requested},
    "artifact_count": {artifact_count},
    "artifacts": {artifacts_json},
    "missing_digests": {missing_digests_json},
    "path_containment_checked": {path_containment_checked},
    "promotion_ready": {promotion_ready},
    "non_claims": {non_claims_json}
  }}"#,
        selected = evidence.selected,
        capture_requested = evidence.capture_requested,
        artifact_count = evidence.artifact_count,
        artifacts_json = frame_artifact_items_json(&evidence.artifacts),
        missing_digests_json = json_string_array(&evidence.missing_digests),
        path_containment_checked = evidence.path_containment_checked,
        promotion_ready = evidence.promotion_ready,
        non_claims_json = json_string_array(&evidence.non_claims),
    )
}

fn frame_artifact_items_json(items: &[FrameArtifactReceiptItem]) -> String {
    let mut out = String::from("[");
    for (index, item) in items.iter().enumerate() {
        if index > 0 {
            out.push_str(", ");
        }
        out.push_str(&format!(
            r#"{{"path": {path}, "relative_path": {relative_path}, "format": {format}, "width_px": {width_px}, "height_px": {height_px}, "frame_id": {frame_id}, "sequence_id": {sequence_id}, "byte_len": {byte_len}, "blake3": {blake3}, "redaction": {redaction}, "includes_ui": {includes_ui}}}"#,
            path = json_string(&item.path),
            relative_path = json_string(&item.relative_path),
            format = json_string(&item.format),
            width_px = item.width_px,
            height_px = item.height_px,
            frame_id = item.frame_id,
            sequence_id = item.sequence_id,
            byte_len = item.byte_len,
            blake3 = json_string(&item.blake3),
            redaction = json_string(&item.redaction),
            includes_ui = item.includes_ui,
        ));
    }
    out.push(']');
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_network_safety_core_accepts_bounded_local_inputs() {
        let evidence = evaluate_load_network_safety(LoadNetworkSafetyInputs {
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

        assert!(evidence.preflight_passed, "{evidence:?}");
        assert!(evidence.promotion_ready, "{evidence:?}");
    }

    #[test]
    fn negative_live_rail_core_rejects_public_unauthorized_inputs() {
        let evidence = evaluate_negative_live_rail_safety_from_inputs(NegativeLiveRailInputs {
            selected: true,
            rail: Some("negative-custom-payload"),
            invalid_action: Some("custom_payload_malformed"),
            expected_outcome: Some(NEGATIVE_LIVE_RAIL_EXPECTED_OUTCOME),
            observed_outcome: None,
            observed_outcome_source: None,
            postcondition_milestone: Some("negative_custom_payload_contained"),
            telemetry_required: true,
            telemetry_present: false,
            target_scope: SAFETY_OWNED_LOCAL_SCOPE,
            explicit_authorization: false,
            public_target: true,
            planned_clients: SAFETY_MAX_LOCAL_CLIENTS + 1,
            max_clients: NEGATIVE_LIVE_RAIL_MAX_CLIENTS,
            timeout_secs: NEGATIVE_LIVE_RAIL_MIN_TIMEOUT_SECS,
        });

        assert!(!evidence.preflight_passed, "{evidence:?}");
        assert!(
            evidence
                .bound_violations
                .contains(&"public_target_without_authorization"),
            "{evidence:?}"
        );
        assert!(
            evidence.missing_fields.contains(&"telemetry"),
            "{evidence:?}"
        );
    }
}

#[cfg(test)]
#[path = "evidence_receipts_colocated_tests.rs"]
mod root_colocated_tests;
