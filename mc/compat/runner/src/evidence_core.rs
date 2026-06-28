use super::*;

fn client_required_milestone_rules<'a>(
    scenario: Scenario,
    projectile_health_needle: &'a str,
) -> Vec<MilestoneRule<'a>> {
    let behavior = scenario_behavior(scenario);
    scenario_required_milestones(scenario)
        .iter()
        .map(|(id, needle)| MilestoneRule {
            id,
            matcher: behavior.client_milestone_matcher(id, needle, projectile_health_needle),
        })
        .collect()
}

fn server_required_milestone_rules(scenario: Scenario) -> Vec<MilestoneRule<'static>> {
    server_required_milestones(scenario)
        .iter()
        .map(|(id, needle)| MilestoneRule {
            id,
            matcher: server_required_matcher(id, needle),
        })
        .collect()
}

fn server_required_matcher(id: &str, needle: &'static str) -> MatcherKind<'static> {
    match id {
        "server_username_seen" => MatcherKind::DynamicUsername,
        "server_client_a_seen" => MatcherKind::DynamicClientSuffix(CLIENT_A_SUFFIX),
        "server_client_b_seen" => MatcherKind::DynamicClientSuffix(CLIENT_B_SUFFIX),
        "server_flag_or_score" => MatcherKind::AnyOfCaseInsensitive(FLAG_OR_SCORE_NEEDLES),
        _ => MatcherKind::CaseInsensitive(needle),
    }
}

fn forbidden_milestone_rules(
    scenario: Scenario,
    case_insensitive: bool,
) -> Vec<MilestoneRule<'static>> {
    scenario_forbidden_patterns(scenario)
        .iter()
        .map(|(id, needle)| MilestoneRule {
            id,
            matcher: if case_insensitive {
                MatcherKind::CaseInsensitive(needle)
            } else {
                MatcherKind::Literal(needle)
            },
        })
        .collect()
}

fn evaluate_required_rules(
    rules: &[MilestoneRule<'_>],
    corpus: &EvidenceCorpus<'_>,
    context: &EvidenceContext<'_>,
) -> (Vec<&'static str>, Vec<&'static str>) {
    let mut observed = Vec::new();
    let mut missing = Vec::new();
    for rule in rules {
        if rule.matcher.is_match(corpus, context) {
            observed.push(rule.id);
        } else {
            missing.push(rule.id);
        }
    }
    (observed, missing)
}

fn evaluate_forbidden_rules(
    rules: &[MilestoneRule<'_>],
    corpus: &EvidenceCorpus<'_>,
    context: &EvidenceContext<'_>,
) -> Vec<&'static str> {
    rules
        .iter()
        .filter_map(|rule| rule.matcher.is_match(corpus, context).then_some(rule.id))
        .collect()
}

#[cfg(test)]
pub(crate) fn evaluate_scenario(scenario: Scenario, output: &str) -> ScenarioEvidence {
    evaluate_scenario_with_projectile_health(
        scenario,
        output,
        PROJECTILE_DAMAGE_CLIENT_HEALTH_NEEDLE,
    )
}

pub(crate) fn evaluate_scenario_for_config(cfg: &Config, output: &str) -> ScenarioEvidence {
    let health_needle = projectile_damage_client_health_needle(cfg);
    evaluate_scenario_with_projectile_health(cfg.scenario, output, &health_needle)
}

pub(crate) fn evaluate_scenario_with_projectile_health(
    scenario: Scenario,
    output: &str,
    projectile_health_needle: &str,
) -> ScenarioEvidence {
    let corpus = EvidenceCorpus::new(output);
    let context = EvidenceContext { username: "" };
    let required_rules = client_required_milestone_rules(scenario, projectile_health_needle);
    let forbidden_rules = forbidden_milestone_rules(scenario, false);
    let (observed_milestones, missing_milestones) =
        evaluate_required_rules(&required_rules, &corpus, &context);
    let forbidden_matches = evaluate_forbidden_rules(&forbidden_rules, &corpus, &context);
    let passed = missing_milestones.is_empty() && forbidden_matches.is_empty();
    ScenarioEvidence {
        observed_milestones,
        missing_milestones,
        forbidden_matches,
        passed,
    }
}

pub(crate) fn evaluate_server_scenario(
    scenario: Scenario,
    server_log: &str,
    username: &str,
) -> ServerScenarioEvidence {
    let corpus = EvidenceCorpus::new(server_log);
    let context = EvidenceContext { username };
    let required_rules = server_required_milestone_rules(scenario);
    let forbidden_rules = forbidden_milestone_rules(scenario, true);
    let (observed_milestones, missing_milestones) =
        evaluate_required_rules(&required_rules, &corpus, &context);
    let forbidden_matches = evaluate_forbidden_rules(&forbidden_rules, &corpus, &context);
    let passed = missing_milestones.is_empty() && forbidden_matches.is_empty();
    ServerScenarioEvidence {
        observed_milestones,
        missing_milestones,
        forbidden_matches,
        passed,
    }
}

const BIOME_DIMENSION_JOIN_STATE_SCENARIO: &str = "survival-biome-dimension-state";
const BIOME_DIMENSION_JOIN_STATE_CLIENT_EVENT: &str = "survival_biome_dimension_state";
const BIOME_DIMENSION_JOIN_STATE_SERVER_EVENT: &str = "server_survival_biome_dimension_state";
const BIOME_DIMENSION_JOIN_STATE_MIN_PROTOCOL: u32 = 1;
const BIOME_DIMENSION_JOIN_STATE_NON_CLAIMS: &[&str] = &[
    "not_dimension_travel",
    "not_portal_behavior",
    "not_all_biome_semantics",
    "not_all_dimensions",
    "not_respawn_rules",
    "not_world_persistence",
    "not_full_survival_compatibility",
    "not_broad_vanilla_parity",
    "not_public_server_safety",
    "not_production_readiness",
];
const BIOME_DIMENSION_JOIN_STATE_OVERBROAD_CLAIMS: &[&str] = &[
    "dimension_travel",
    "portal_behavior",
    "all_biome_semantics",
    "all_dimensions",
    "respawn_rules",
    "world_persistence",
    "full_survival_compatibility",
    "broad_vanilla_parity",
    "public_server_safety",
    "production_readiness",
];
const BIOME_DIMENSION_JOIN_STATE_MISSING_PROTOCOL: &str = "missing_protocol_context";
const BIOME_DIMENSION_JOIN_STATE_MISSING_CLIENT: &str = "missing_client_observed_state";
const BIOME_DIMENSION_JOIN_STATE_MISSING_SERVER: &str = "missing_server_configured_state";
const BIOME_DIMENSION_JOIN_STATE_WRONG_SCENARIO: &str = "scenario_identity_mismatch";
const BIOME_DIMENSION_JOIN_STATE_SPAWN_MISMATCH: &str = "mismatched_spawn_environment";
const BIOME_DIMENSION_JOIN_STATE_ENVIRONMENT_MISMATCH: &str = "mismatched_environment_identifier";
const BIOME_DIMENSION_JOIN_STATE_UPDATE_MISMATCH: &str = "mismatched_environment_update";
const BIOME_DIMENSION_JOIN_STATE_NORMALIZED_MISMATCH: &str = "mismatched_normalized_identifier";
const BIOME_DIMENSION_JOIN_STATE_MISSING_NON_CLAIM_PREFIX: &str = "missing_non_claim:";
const BIOME_DIMENSION_JOIN_STATE_OVERBROAD_PREFIX: &str = "overbroad_claim:";

pub(crate) fn biome_dimension_join_state_required_non_claims() -> &'static [&'static str] {
    BIOME_DIMENSION_JOIN_STATE_NON_CLAIMS
}

pub(crate) fn evaluate_biome_dimension_join_state(
    scenario: Scenario,
    protocol: u32,
    client: &ScenarioEvidence,
    server: &ServerScenarioEvidence,
) -> BiomeDimensionJoinStateEvidence {
    let selected = scenario == Scenario::SurvivalBiomeDimensionState;
    let record = BiomeDimensionJoinStateRecord {
        selected,
        scenario: scenario_name(scenario).to_string(),
        protocol: selected.then_some(protocol),
        client_observed_state: selected
            .then(|| biome_dimension_client_state_from_milestones(client))
            .flatten(),
        server_configured_state: selected
            .then(|| biome_dimension_server_state_from_milestones(server))
            .flatten(),
        non_claims: selected
            .then(|| {
                BIOME_DIMENSION_JOIN_STATE_NON_CLAIMS
                    .iter()
                    .map(|claim| (*claim).to_string())
                    .collect()
            })
            .unwrap_or_default(),
    };
    let validation = validate_biome_dimension_join_state_record(&record);
    BiomeDimensionJoinStateEvidence { record, validation }
}

pub(crate) fn validate_biome_dimension_join_state_record(
    record: &BiomeDimensionJoinStateRecord,
) -> BiomeDimensionJoinStateValidation {
    if !record.selected {
        return BiomeDimensionJoinStateValidation {
            passed: true,
            diagnostics: Vec::new(),
        };
    }

    let mut diagnostics = Vec::new();
    if record.scenario != BIOME_DIMENSION_JOIN_STATE_SCENARIO {
        diagnostics.push(BIOME_DIMENSION_JOIN_STATE_WRONG_SCENARIO.to_string());
    }
    match record.protocol {
        Some(protocol) if protocol >= BIOME_DIMENSION_JOIN_STATE_MIN_PROTOCOL => {}
        _ => diagnostics.push(BIOME_DIMENSION_JOIN_STATE_MISSING_PROTOCOL.to_string()),
    }

    if record.client_observed_state.is_none() {
        diagnostics.push(BIOME_DIMENSION_JOIN_STATE_MISSING_CLIENT.to_string());
    }
    if record.server_configured_state.is_none() {
        diagnostics.push(BIOME_DIMENSION_JOIN_STATE_MISSING_SERVER.to_string());
    }
    if let (Some(client), Some(server)) = (
        record.client_observed_state.as_ref(),
        record.server_configured_state.as_ref(),
    ) {
        diagnostics.extend(biome_dimension_join_state_mismatch_diagnostics(
            client, server,
        ));
    }
    diagnostics.extend(biome_dimension_join_state_non_claim_diagnostics(
        &record.non_claims,
    ));

    BiomeDimensionJoinStateValidation {
        passed: diagnostics.is_empty(),
        diagnostics,
    }
}

fn biome_dimension_join_state_mismatch_diagnostics(
    client: &BiomeDimensionJoinStateClientState,
    server: &BiomeDimensionJoinStateServerState,
) -> Vec<String> {
    let mut diagnostics = Vec::new();
    if client.spawn_environment != server.spawn_environment {
        diagnostics.push(BIOME_DIMENSION_JOIN_STATE_SPAWN_MISMATCH.to_string());
    }
    if client.environment_identifier != server.environment_identifier {
        diagnostics.push(BIOME_DIMENSION_JOIN_STATE_ENVIRONMENT_MISMATCH.to_string());
    }
    if client.client_environment_update != server.server_environment_state {
        diagnostics.push(BIOME_DIMENSION_JOIN_STATE_UPDATE_MISMATCH.to_string());
    }
    if client.normalized_identifier != server.normalized_identifier {
        diagnostics.push(BIOME_DIMENSION_JOIN_STATE_NORMALIZED_MISMATCH.to_string());
    }
    diagnostics
}

fn biome_dimension_join_state_non_claim_diagnostics(non_claims: &[String]) -> Vec<String> {
    let mut diagnostics = Vec::new();
    for required in BIOME_DIMENSION_JOIN_STATE_NON_CLAIMS {
        if !non_claims.iter().any(|claim| claim == required) {
            diagnostics.push(format!(
                "{BIOME_DIMENSION_JOIN_STATE_MISSING_NON_CLAIM_PREFIX}{required}"
            ));
        }
    }
    for overbroad in BIOME_DIMENSION_JOIN_STATE_OVERBROAD_CLAIMS {
        if non_claims.iter().any(|claim| claim == overbroad) {
            diagnostics.push(format!(
                "{BIOME_DIMENSION_JOIN_STATE_OVERBROAD_PREFIX}{overbroad}"
            ));
        }
    }
    diagnostics
}

fn biome_dimension_client_state_from_milestones(
    client: &ScenarioEvidence,
) -> Option<BiomeDimensionJoinStateClientState> {
    client
        .observed_milestones
        .iter()
        .any(|milestone| *milestone == BIOME_DIMENSION_JOIN_STATE_CLIENT_EVENT)
        .then(|| {
            biome_dimension_client_state_from_line(SURVIVAL_BIOME_DIMENSION_CLIENT_STATE_NEEDLE)
        })
        .flatten()
}

fn biome_dimension_server_state_from_milestones(
    server: &ServerScenarioEvidence,
) -> Option<BiomeDimensionJoinStateServerState> {
    server
        .observed_milestones
        .iter()
        .any(|milestone| *milestone == BIOME_DIMENSION_JOIN_STATE_SERVER_EVENT)
        .then(|| {
            biome_dimension_server_state_from_line(SURVIVAL_BIOME_DIMENSION_SERVER_STATE_NEEDLE)
        })
        .flatten()
}

fn biome_dimension_client_state_from_line(
    line: &str,
) -> Option<BiomeDimensionJoinStateClientState> {
    Some(BiomeDimensionJoinStateClientState {
        spawn_environment: key_value_field(line, "spawn_environment")?,
        environment_identifier: key_value_field(line, "environment_identifier")?,
        client_environment_update: key_value_field(line, "client_environment_update")?,
        normalized_identifier: key_value_field(line, "normalized_identifier")?,
    })
}

fn biome_dimension_server_state_from_line(
    line: &str,
) -> Option<BiomeDimensionJoinStateServerState> {
    Some(BiomeDimensionJoinStateServerState {
        username: key_value_field(line, "username")?,
        spawn_environment: key_value_field(line, "spawn_environment")?,
        environment_identifier: key_value_field(line, "environment_identifier")?,
        server_environment_state: key_value_field(line, "server_environment_state")?,
        normalized_identifier: key_value_field(line, "normalized_identifier")?,
    })
}

fn key_value_field(line: &str, key: &str) -> Option<String> {
    let prefix = format!("{key}=");
    line.split_whitespace()
        .find_map(|token| token.strip_prefix(&prefix))
        .map(str::to_string)
}

#[cfg(test)]
pub(crate) fn parse_typed_event_line(line: &str) -> Result<TypedEvent, String> {
    let line = line.trim();
    let Some(rest) = line.strip_prefix(TYPED_EVENT_PREFIX) else {
        return Err("typed event line missing prefix".to_string());
    };
    let fields = parse_typed_event_fields(rest.trim())?;
    let schema_version = typed_event_required_u32(&fields, "schema")?;
    if schema_version != TYPED_EVENT_SCHEMA_VERSION {
        return Err(format!(
            "unsupported typed event schema {schema_version}, expected {TYPED_EVENT_SCHEMA_VERSION}"
        ));
    }
    Ok(TypedEvent {
        schema_version,
        source: typed_event_required_string(&fields, "source")?,
        scenario: typed_event_required_string(&fields, "scenario")?,
        session: typed_event_required_string(&fields, "session")?,
        username: typed_event_optional_string(&fields, "username"),
        sequence: u64::from(typed_event_required_u32(&fields, "seq")?),
        kind: typed_event_required_string(&fields, "event")?,
    })
}

#[cfg(test)]
fn parse_typed_event_fields(text: &str) -> Result<Vec<(&str, &str)>, String> {
    let mut fields = Vec::new();
    for token in text.split_whitespace() {
        let Some((key, value)) = token.split_once('=') else {
            return Err(format!("typed event token missing '=': {token}"));
        };
        fields.push((key, value));
    }
    Ok(fields)
}

#[cfg(test)]
fn typed_event_required_string(fields: &[(&str, &str)], key: &str) -> Result<String, String> {
    typed_event_optional_string(fields, key)
        .ok_or_else(|| format!("missing typed event field {key}"))
}

#[cfg(test)]
fn typed_event_optional_string(fields: &[(&str, &str)], key: &str) -> Option<String> {
    fields
        .iter()
        .find_map(|(field_key, value)| (*field_key == key).then(|| (*value).to_string()))
}

#[cfg(test)]
fn typed_event_required_u32(fields: &[(&str, &str)], key: &str) -> Result<u32, String> {
    let value = typed_event_required_string(fields, key)?;
    value
        .parse::<u32>()
        .map_err(|err| format!("parse typed event field {key}: {err}"))
}

pub(crate) fn evaluate_typed_event_graph(
    events: &[TypedEvent],
    scenario: &str,
    session: &str,
    username: Option<&str>,
    required_events: &[&str],
    forbidden_events: &[&str],
    ordered_edges: &[(&str, &str)],
) -> TypedEventGraphEvaluation {
    let relevant: Vec<&TypedEvent> = events
        .iter()
        .filter(|event| {
            event.scenario == scenario
                && event.session == session
                && username.is_none_or(|name| event.username.as_deref() == Some(name))
        })
        .collect();
    let mut observed_events = Vec::new();
    let mut missing_events = Vec::new();
    for required in required_events {
        if relevant.iter().any(|event| event.kind == *required) {
            observed_events.push((*required).to_string());
        } else {
            missing_events.push((*required).to_string());
        }
    }
    let mut forbidden_matches = Vec::new();
    for forbidden in forbidden_events {
        if relevant.iter().any(|event| event.kind == *forbidden) {
            forbidden_matches.push((*forbidden).to_string());
        }
    }
    let mut order_violations = Vec::new();
    for (before, after) in ordered_edges {
        if let (Some(before_seq), Some(after_seq)) = (
            first_typed_event_sequence(&relevant, before),
            first_typed_event_sequence(&relevant, after),
        ) {
            if before_seq >= after_seq {
                order_violations.push(format!("{before}_before_{after}"));
            }
        }
    }
    let passed =
        missing_events.is_empty() && forbidden_matches.is_empty() && order_violations.is_empty();
    TypedEventGraphEvaluation {
        observed_events,
        missing_events,
        forbidden_events: forbidden_matches,
        order_violations,
        passed,
    }
}

fn first_typed_event_sequence(events: &[&TypedEvent], kind: &str) -> Option<u64> {
    events
        .iter()
        .filter(|event| event.kind == kind)
        .map(|event| event.sequence)
        .min()
}

pub(crate) fn typed_events_from_receipt_evidence(
    cfg: &Config,
    client: &ClientRunEvidence,
) -> Result<Vec<TypedEvent>, String> {
    let scenario_label = scenario_name(cfg.scenario).to_string();
    let session = typed_event_session_id(cfg);
    let default_username = single_typed_event_username(client);
    let mut events = Vec::new();
    if let Some(scenario) = &client.scenario {
        for milestone in &scenario.observed_milestones {
            push_typed_event(
                &mut events,
                "client",
                &scenario_label,
                &session,
                default_username,
                milestone,
            )?;
        }
    }
    if let Some(server) = &client.server_scenario {
        for milestone in &server.observed_milestones {
            push_typed_event(
                &mut events,
                "server",
                &scenario_label,
                &session,
                default_username,
                milestone,
            )?;
        }
    }
    append_mcp_control_typed_events(
        &mut events,
        cfg,
        client,
        &scenario_label,
        &session,
        default_username,
    )?;
    if let Some(causality) = &client.projectile_damage_causality {
        for step in &causality.observed_steps {
            let (source, username) = typed_event_projectile_step_source_username(
                step,
                &causality.attacker_username,
                &causality.victim_username,
            );
            push_typed_event(
                &mut events,
                source,
                &scenario_label,
                &session,
                username,
                step,
            )?;
        }
    }
    Ok(events)
}

fn append_mcp_control_typed_events(
    events: &mut Vec<TypedEvent>,
    cfg: &Config,
    client: &ClientRunEvidence,
    scenario_label: &str,
    session: &str,
    username: Option<&str>,
) -> Result<(), String> {
    if cfg.scenario != Scenario::McpControlledSmoke {
        return Ok(());
    }
    if let Some(control) = &client.mcp_control {
        if control.stdout_clean {
            push_typed_event(
                events,
                "mcp",
                scenario_label,
                session,
                username,
                "mcp_stdout_clean",
            )?;
        }
        if control
            .command_outcome_ids
            .iter()
            .any(|outcome| *outcome == "look.applied")
        {
            push_typed_event(
                events,
                "mcp",
                scenario_label,
                session,
                username,
                "mcp_look_call",
            )?;
        }
        if control
            .command_outcome_ids
            .iter()
            .any(|outcome| matches!(*outcome, "key.applied" | "chat.applied"))
        {
            push_typed_event(
                events,
                "mcp",
                scenario_label,
                session,
                username,
                "mcp_input_call",
            )?;
        }
        if control
            .command_outcome_ids
            .iter()
            .any(|outcome| *outcome == "capture_latest_frame.captured")
            || control
                .calls_succeeded
                .iter()
                .any(|call| *call == "tools/call capture_latest_frame")
        {
            push_typed_event(
                events,
                "mcp",
                scenario_label,
                session,
                username,
                "mcp_capture_latest_frame",
            )?;
        }
    }
    if client
        .frame_artifacts
        .as_ref()
        .is_some_and(frame_artifacts_have_reviewable_identity)
    {
        push_typed_event(
            events,
            "mcp",
            scenario_label,
            session,
            username,
            "mcp_frame_artifact_identity",
        )?;
    }
    Ok(())
}

fn frame_artifacts_have_reviewable_identity(frame: &FrameArtifactsReceiptEvidence) -> bool {
    frame.selected
        && frame.capture_requested
        && frame.artifact_count > 0
        && frame.path_containment_checked
        && frame.missing_digests.is_empty()
        && frame.artifacts.iter().any(|artifact| {
            !artifact.path.is_empty()
                && !artifact.relative_path.is_empty()
                && !artifact.blake3.is_empty()
                && artifact.byte_len > 0
        })
}

fn push_typed_event(
    events: &mut Vec<TypedEvent>,
    source: &str,
    scenario: &str,
    session: &str,
    username: Option<&str>,
    kind: &str,
) -> Result<(), String> {
    let sequence_index = events.len() + TYPED_EVENT_SEQUENCE_INDEX_OFFSET;
    let sequence = u64::try_from(sequence_index)
        .map_err(|err| format!("typed event sequence overflow at {sequence_index}: {err}"))?;
    events.push(TypedEvent {
        schema_version: TYPED_EVENT_SCHEMA_VERSION,
        source: source.to_string(),
        scenario: scenario.to_string(),
        session: session.to_string(),
        username: username.map(sanitize_typed_event_field),
        sequence,
        kind: kind.to_string(),
    });
    Ok(())
}

fn single_typed_event_username(client: &ClientRunEvidence) -> Option<&str> {
    if client.usernames.len() == TYPED_EVENT_SINGLE_USERNAME_COUNT {
        client.usernames.first().map(String::as_str)
    } else {
        None
    }
}

fn typed_event_projectile_step_source_username<'a>(
    step: &str,
    attacker_username: &'a str,
    victim_username: &'a str,
) -> (&'static str, Option<&'a str>) {
    if step.starts_with("attacker_client") {
        ("client", Some(attacker_username))
    } else if step.starts_with("victim_client") {
        ("client", Some(victim_username))
    } else {
        ("server", None)
    }
}

fn typed_event_session_id(cfg: &Config) -> String {
    cfg.receipt_path
        .as_ref()
        .and_then(|path| path.file_stem())
        .and_then(|stem| stem.to_str())
        .map(sanitize_typed_event_field)
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| TYPED_EVENT_DEFAULT_SESSION_ID.to_string())
}

fn sanitize_typed_event_field(value: &str) -> String {
    let mut sanitized = String::with_capacity(value.len().min(TYPED_EVENT_MAX_FIELD_CHARS));
    for ch in value.chars().take(TYPED_EVENT_MAX_FIELD_CHARS) {
        if ch.is_ascii_alphanumeric() || matches!(ch, '_' | '-' | '.') {
            sanitized.push(ch);
        } else {
            sanitized.push('_');
        }
    }
    sanitized
}

pub(crate) fn normalize_typed_event_timeline(events: &[TypedEvent]) -> String {
    let mut timeline = events.to_vec();
    timeline.sort_by(|left, right| {
        left.sequence
            .cmp(&right.sequence)
            .then_with(|| left.source.cmp(&right.source))
            .then_with(|| left.kind.cmp(&right.kind))
    });
    let mut output = String::new();
    for event in &timeline {
        output.push_str(&render_typed_event_line(event));
        output.push('\n');
    }
    output
}

fn render_typed_event_line(event: &TypedEvent) -> String {
    let username_field = event
        .username
        .as_ref()
        .map(|username| format!(" username={username}"))
        .unwrap_or_default();
    format!(
        "{TYPED_EVENT_PREFIX} schema={} source={} scenario={} session={}{} seq={} event={}",
        event.schema_version,
        event.source,
        event.scenario,
        event.session,
        username_field,
        event.sequence,
        event.kind
    )
}

pub(crate) fn typed_event_timeline_blake3(timeline: &str) -> String {
    blake3::hash(timeline.as_bytes()).to_hex().to_string()
}

pub(crate) fn typed_event_oracle_receipt_json(
    artifact: Option<&TypedEventOracleArtifact>,
) -> String {
    let selected = artifact.is_some();
    let migration_status = if selected {
        TYPED_EVENT_MIGRATION_DERIVED_FROM_MILESTONES
    } else {
        TYPED_EVENT_MIGRATION_FALLBACK
    };
    let event_log_path_json = artifact
        .map(|evidence| json_string(&evidence.event_log_path.display().to_string()))
        .unwrap_or_else(|| "null".to_string());
    let timeline_blake3_json = artifact
        .map(|evidence| json_string(&evidence.timeline_blake3))
        .unwrap_or_else(|| "null".to_string());
    let event_count = artifact
        .map(|evidence| evidence.event_count)
        .unwrap_or_default();
    let contributes_to_pass_fail = artifact
        .map(|evidence| evidence.contributes_to_pass_fail)
        .unwrap_or(false);
    format!(
        r#"{{
    "schema_version": {schema_version},
    "selected": {selected},
    "migration_status": {migration_status_json},
    "event_log_path": {event_log_path_json},
    "timeline_blake3": {timeline_blake3_json},
    "event_count": {event_count},
    "contributes_to_pass_fail": {contributes_to_pass_fail},
    "raw_payloads_recorded": false
  }}"#,
        schema_version = TYPED_EVENT_SCHEMA_VERSION,
        selected = selected,
        migration_status_json = json_string(migration_status),
        event_log_path_json = event_log_path_json,
        timeline_blake3_json = timeline_blake3_json,
        event_count = event_count,
        contributes_to_pass_fail = contributes_to_pass_fail,
    )
}

pub(crate) fn typed_event_oracle_contributes_to_pass_fail(scenario: Scenario) -> bool {
    matches!(
        scenario,
        Scenario::Smoke
            | Scenario::McpControlledSmoke
            | Scenario::InventoryInteraction
            | Scenario::InventoryStackSplitMerge
            | Scenario::InventoryDragTransactions
            | Scenario::SurvivalBreakPlacePickup
            | Scenario::SurvivalChestPersistence
            | Scenario::SurvivalCraftingTable
            | Scenario::SurvivalCraftingRecipeBreadth
            | Scenario::SurvivalFurnacePersistence
            | Scenario::SurvivalFurnaceSmeltingBreadth
            | Scenario::SurvivalHungerFood
            | Scenario::SurvivalHungerHealthCycle
            | Scenario::SurvivalMobDrop
            | Scenario::SurvivalMobAiLootBreadth
            | Scenario::SurvivalRedstoneToggle
            | Scenario::SurvivalRedstoneCircuitBreadth
            | Scenario::SurvivalWorldPersistenceRestart
            | Scenario::SurvivalWorldMultichunkDurability
            | Scenario::SurvivalCrashRecoveryParity
            | Scenario::SurvivalBlockEntityPersistenceParity
            | Scenario::SurvivalContainerBlockEntityBreadth
            | Scenario::SurvivalBiomeDimensionState
            | Scenario::SurvivalBiomeDimensionTravel
            | Scenario::SurvivalSignEditingLive
            | Scenario::FlagScoreRepeat
            | Scenario::BlueFlagScore
            | Scenario::CombatDamage
            | Scenario::CombatKnockback
            | Scenario::ArmorEquipmentMitigation
            | Scenario::EquipmentUpdateObservation
            | Scenario::ProjectileHit
            | Scenario::ProjectileDamageAttribution
            | Scenario::FlagCarrierDeathReturn
            | Scenario::ReconnectFlagState
            | Scenario::CtfInvalidPickupOwnership
            | Scenario::CtfInvalidReturnDrop
            | Scenario::CtfInvalidOpponentBaseReturnDrop
            | Scenario::CtfScoreLimitWinCondition
            | Scenario::CtfSimultaneousPickupCaptureRace
            | Scenario::CtfSpawnTeamBalanceReset
            | Scenario::ReconnectFlagScore
            | Scenario::MultiClientLoadScore
    )
}

const RESTART_PERSISTENCE_REQUIRED_EVENT_COUNT: usize = 8;
const RESTART_PERSISTENCE_DIAGNOSTIC_MISSING_PREFIX: &str = "missing";
const RESTART_PERSISTENCE_DIAGNOSTIC_DUPLICATE_PREFIX: &str = "duplicate";
const RESTART_PERSISTENCE_DIAGNOSTIC_UNORDERED_PREFIX: &str = "unordered";
const RESTART_PERSISTENCE_DIAGNOSTIC_STALE_PREFIX: &str = "stale_restored_state";
const RESTART_PERSISTENCE_DIAGNOSTIC_MISMATCH_PREFIX: &str = "mismatched_restored_state";
const RESTART_PERSISTENCE_SERVER_STATE_EVENTS: &[&str] = &[
    "server_survival_world_persistence_state",
    "server_survival_crash_recovery_state",
    "server_survival_block_entity_state",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RestartPersistenceTypedContract {
    client_pre_boundary: &'static str,
    client_reconnect: &'static str,
    client_post_boundary: &'static str,
    server_mutation: &'static str,
    server_boundary: &'static str,
    server_restart: &'static str,
    server_post_boundary: &'static str,
    server_state: &'static str,
}

impl RestartPersistenceTypedContract {
    fn required_events(self) -> [&'static str; RESTART_PERSISTENCE_REQUIRED_EVENT_COUNT] {
        [
            self.client_pre_boundary,
            self.client_reconnect,
            self.client_post_boundary,
            self.server_mutation,
            self.server_boundary,
            self.server_restart,
            self.server_post_boundary,
            self.server_state,
        ]
    }

    fn ordered_edges(
        self,
    ) -> [(&'static str, &'static str); RESTART_PERSISTENCE_REQUIRED_EVENT_COUNT] {
        [
            (self.client_pre_boundary, self.client_reconnect),
            (self.client_reconnect, self.client_post_boundary),
            (self.server_mutation, self.server_boundary),
            (self.server_boundary, self.server_restart),
            (self.server_restart, self.server_post_boundary),
            (self.server_post_boundary, self.server_state),
            (self.server_boundary, self.server_state),
            (self.server_restart, self.server_state),
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RestartPersistenceTypedValidation {
    pub(crate) diagnostics: Vec<String>,
    pub(crate) passed: bool,
}

pub(crate) fn validate_restart_persistence_typed_events(
    scenario: Scenario,
    events: &[TypedEvent],
    scenario_label: &str,
    session: &str,
    username: Option<&str>,
) -> Option<RestartPersistenceTypedValidation> {
    let contract = restart_persistence_typed_contract(scenario)?;
    let relevant: Vec<&TypedEvent> = events
        .iter()
        .filter(|event| {
            event.scenario == scenario_label
                && event.session == session
                && username.is_none_or(|name| event.username.as_deref() == Some(name))
        })
        .collect();
    let mut diagnostics = Vec::new();
    for required in contract.required_events() {
        let observed_count = typed_event_kind_count(&relevant, required);
        if observed_count == 0 {
            diagnostics.push(format!(
                "{RESTART_PERSISTENCE_DIAGNOSTIC_MISSING_PREFIX}:{required}"
            ));
        }
        if observed_count > 1 {
            diagnostics.push(format!(
                "{RESTART_PERSISTENCE_DIAGNOSTIC_DUPLICATE_PREFIX}:{required}"
            ));
        }
    }
    for state_event in RESTART_PERSISTENCE_SERVER_STATE_EVENTS {
        if *state_event != contract.server_state
            && typed_event_kind_count(&relevant, state_event) > 0
        {
            diagnostics.push(format!(
                "{RESTART_PERSISTENCE_DIAGNOSTIC_MISMATCH_PREFIX}:{state_event}"
            ));
        }
    }
    for (before, after) in contract.ordered_edges() {
        push_typed_event_order_diagnostic(&relevant, before, after, &mut diagnostics);
    }
    push_stale_restart_state_diagnostic(&relevant, contract, &mut diagnostics);
    Some(RestartPersistenceTypedValidation {
        passed: diagnostics.is_empty(),
        diagnostics,
    })
}

fn restart_persistence_typed_contract(
    scenario: Scenario,
) -> Option<RestartPersistenceTypedContract> {
    match scenario {
        Scenario::SurvivalWorldPersistenceRestart => Some(RestartPersistenceTypedContract {
            client_pre_boundary: "survival_world_persistence_pre_restart_update",
            client_reconnect: "survival_world_persistence_reconnect_sent",
            client_post_boundary: "survival_world_persistence_post_restart_update",
            server_mutation: "server_survival_world_persistence_mutation",
            server_boundary: "server_survival_world_persistence_clean_shutdown",
            server_restart: "server_survival_world_persistence_backend_restart",
            server_post_boundary: "server_survival_world_persistence_post_restart",
            server_state: "server_survival_world_persistence_state",
        }),
        Scenario::SurvivalCrashRecoveryParity => Some(RestartPersistenceTypedContract {
            client_pre_boundary: "survival_crash_recovery_pre_crash_update",
            client_reconnect: "survival_crash_recovery_reconnect_sent",
            client_post_boundary: "survival_crash_recovery_post_crash_update",
            server_mutation: "server_survival_crash_recovery_mutation",
            server_boundary: "server_survival_crash_recovery_forced_stop",
            server_restart: "server_survival_crash_recovery_backend_restart",
            server_post_boundary: "server_survival_crash_recovery_post_crash",
            server_state: "server_survival_crash_recovery_state",
        }),
        Scenario::SurvivalBlockEntityPersistenceParity => Some(RestartPersistenceTypedContract {
            client_pre_boundary: "survival_block_entity_pre_restart_update",
            client_reconnect: "survival_block_entity_reconnect_sent",
            client_post_boundary: "survival_block_entity_post_restart_update",
            server_mutation: "server_survival_block_entity_mutation",
            server_boundary: "server_survival_block_entity_clean_shutdown",
            server_restart: "server_survival_block_entity_backend_restart",
            server_post_boundary: "server_survival_block_entity_post_restart",
            server_state: "server_survival_block_entity_state",
        }),
        _ => None,
    }
}

fn typed_event_kind_count(events: &[&TypedEvent], kind: &str) -> usize {
    events.iter().filter(|event| event.kind == kind).count()
}

fn push_typed_event_order_diagnostic(
    events: &[&TypedEvent],
    before: &str,
    after: &str,
    diagnostics: &mut Vec<String>,
) {
    if let (Some(before_seq), Some(after_seq)) = (
        first_typed_event_sequence(events, before),
        first_typed_event_sequence(events, after),
    ) {
        if before_seq >= after_seq {
            diagnostics.push(format!(
                "{RESTART_PERSISTENCE_DIAGNOSTIC_UNORDERED_PREFIX}:{before}_before_{after}"
            ));
        }
    }
}

fn push_stale_restart_state_diagnostic(
    events: &[&TypedEvent],
    contract: RestartPersistenceTypedContract,
    diagnostics: &mut Vec<String>,
) {
    if let (Some(state_seq), Some(post_seq)) = (
        first_typed_event_sequence(events, contract.server_state),
        first_typed_event_sequence(events, contract.server_post_boundary),
    ) {
        if state_seq <= post_seq {
            diagnostics.push(format!(
                "{RESTART_PERSISTENCE_DIAGNOSTIC_STALE_PREFIX}:{}",
                contract.server_state
            ));
        }
    }
}

pub(crate) fn validate_typed_event_oracle_for_migrated_scenario(
    cfg: &Config,
    client: &ClientRunEvidence,
) -> Result<(), String> {
    if !typed_event_oracle_contributes_to_pass_fail(cfg.scenario) {
        return Ok(());
    }
    let events = typed_events_from_receipt_evidence(cfg, client)?;
    let session = typed_event_session_id(cfg);
    let scenario_label = scenario_name(cfg.scenario);
    let username = single_typed_event_username(client);
    if let Some(restart_validation) = validate_restart_persistence_typed_events(
        cfg.scenario,
        &events,
        scenario_label,
        &session,
        username,
    ) {
        if !restart_validation.passed {
            return Err(format!(
                "restart persistence typed event oracle for scenario {} failed: diagnostics={:?}",
                scenario_label, restart_validation.diagnostics
            ));
        }
    }
    let required = typed_event_required_events_for_graph(cfg.scenario);
    let required_refs = required.iter().map(String::as_str).collect::<Vec<_>>();
    let forbidden = scenario_forbidden_patterns(cfg.scenario)
        .iter()
        .map(|(name, _)| *name)
        .collect::<Vec<_>>();
    let ordered_edges = typed_event_ordered_edges_for_scenario(cfg.scenario);
    let result = evaluate_typed_event_graph(
        &events,
        scenario_label,
        &session,
        username,
        &required_refs,
        &forbidden,
        &ordered_edges,
    );
    if result.passed {
        return Ok(());
    }
    Err(format!(
        "typed event oracle for scenario {} failed: missing={:?} forbidden={:?} order_violations={:?}",
        scenario_name(cfg.scenario),
        result.missing_events,
        result.forbidden_events,
        result.order_violations
    ))
}

pub(crate) fn typed_event_required_events_for_graph(scenario: Scenario) -> Vec<String> {
    let mut required = scenario_required_milestones(scenario)
        .iter()
        .map(|(name, _)| (*name).to_string())
        .collect::<Vec<_>>();
    required.extend(
        server_required_milestones(scenario)
            .iter()
            .map(|(name, _)| (*name).to_string()),
    );
    if scenario == Scenario::McpControlledSmoke {
        required.extend([
            "mcp_stdout_clean".to_string(),
            "mcp_look_call".to_string(),
            "mcp_input_call".to_string(),
            "mcp_capture_latest_frame".to_string(),
            "mcp_frame_artifact_identity".to_string(),
        ]);
    }
    required
}

pub(crate) fn typed_event_ordered_edges_for_scenario(
    scenario: Scenario,
) -> Vec<(&'static str, &'static str)> {
    match scenario {
        Scenario::Smoke => vec![],
        Scenario::McpControlledSmoke => vec![
            ("mcp_initialize", "mcp_tools_list"),
            ("mcp_tools_list", "mcp_status_call"),
            ("mcp_status_call", "mcp_look_call"),
            ("mcp_look_call", "mcp_input_call"),
            ("mcp_input_call", "mcp_capture_latest_frame"),
            ("mcp_capture_latest_frame", "mcp_frame_artifact_identity"),
        ],
        Scenario::InventoryInteraction => vec![
            ("protocol_detected", "inventory_drop_sent"),
            ("inventory_drop_sent", "inventory_pickup_seen"),
            ("inventory_pickup_seen", "inventory_click_sent"),
            ("inventory_click_sent", "inventory_container_click_sent"),
            (
                "inventory_container_click_sent",
                "inventory_block_place_sent",
            ),
            ("server_inventory_drop", "server_inventory_pickup"),
            ("server_inventory_pickup", "server_inventory_click"),
            ("server_inventory_container_click", "server_block_place"),
        ],
        Scenario::InventoryStackSplitMerge => vec![
            (
                "inventory_stack_initial_slot",
                "inventory_stack_split_pickup_sent",
            ),
            (
                "inventory_stack_split_pickup_sent",
                "inventory_stack_split_source_seen",
            ),
            (
                "inventory_stack_split_source_seen",
                "inventory_stack_split_place_sent",
            ),
            (
                "inventory_stack_split_place_sent",
                "inventory_stack_destination_seen",
            ),
            (
                "inventory_stack_destination_seen",
                "inventory_stack_merge_pickup_sent",
            ),
            (
                "inventory_stack_merge_pickup_sent",
                "inventory_stack_merge_destination_empty_seen",
            ),
            (
                "inventory_stack_merge_destination_empty_seen",
                "inventory_stack_merge_place_sent",
            ),
            (
                "inventory_stack_merge_place_sent",
                "inventory_stack_final_source_seen",
            ),
            (
                "server_inventory_stack_split_pickup",
                "server_inventory_stack_split",
            ),
            (
                "server_inventory_stack_split",
                "server_inventory_stack_merge_pickup",
            ),
            (
                "server_inventory_stack_merge_pickup",
                "server_inventory_stack_merge",
            ),
        ],
        Scenario::InventoryDragTransactions => vec![
            ("inventory_drag_initial_slot", "inventory_drag_pickup_sent"),
            (
                "inventory_drag_pickup_sent",
                "inventory_drag_source_empty_seen",
            ),
            (
                "inventory_drag_source_empty_seen",
                "inventory_drag_start_sent",
            ),
            ("inventory_drag_start_sent", "inventory_drag_target_a_sent"),
            (
                "inventory_drag_target_a_sent",
                "inventory_drag_target_b_sent",
            ),
            ("inventory_drag_target_b_sent", "inventory_drag_end_sent"),
            (
                "inventory_drag_end_sent",
                "inventory_drag_final_distribution_seen",
            ),
            (
                "server_inventory_drag_pickup",
                "server_inventory_drag_start",
            ),
            (
                "server_inventory_drag_start",
                "server_inventory_drag_target_a",
            ),
            (
                "server_inventory_drag_target_a",
                "server_inventory_drag_target_b",
            ),
            (
                "server_inventory_drag_target_b",
                "server_inventory_drag_end",
            ),
        ],
        Scenario::SurvivalBreakPlacePickup => vec![
            ("survival_break_sent", "survival_break_update"),
            ("survival_break_update", "survival_pickup_seen"),
            ("survival_pickup_seen", "survival_place_sent"),
            ("survival_place_sent", "survival_place_update"),
            ("server_survival_join", "server_survival_break"),
            ("server_survival_break", "server_survival_pickup"),
            ("server_survival_pickup", "server_survival_place"),
        ],
        Scenario::SurvivalChestPersistence => vec![
            ("survival_chest_open_seen", "survival_chest_store_sent"),
            ("survival_chest_store_sent", "survival_chest_close_sent"),
            ("survival_chest_close_sent", "survival_chest_reconnect_sent"),
            (
                "survival_chest_reconnect_sent",
                "survival_chest_reopen_seen",
            ),
            (
                "survival_chest_reopen_seen",
                "survival_chest_persisted_seen",
            ),
            ("server_survival_chest_open", "server_survival_chest_store"),
            ("server_survival_chest_store", "server_survival_chest_close"),
            (
                "server_survival_chest_close",
                "server_survival_chest_reopen",
            ),
            (
                "server_survival_chest_reopen",
                "server_survival_chest_persisted",
            ),
        ],
        Scenario::SurvivalFurnacePersistence => vec![
            ("survival_furnace_open_seen", "survival_furnace_input_sent"),
            ("survival_furnace_input_sent", "survival_furnace_fuel_sent"),
            (
                "survival_furnace_fuel_sent",
                "survival_furnace_burn_progress_seen",
            ),
            (
                "survival_furnace_burn_progress_seen",
                "survival_furnace_output_seen",
            ),
            (
                "survival_furnace_output_seen",
                "survival_furnace_output_collected",
            ),
            (
                "survival_furnace_output_collected",
                "survival_furnace_reconnect_sent",
            ),
            (
                "survival_furnace_reconnect_sent",
                "survival_furnace_reopen_seen",
            ),
            (
                "server_survival_furnace_open",
                "server_survival_furnace_input",
            ),
            (
                "server_survival_furnace_input",
                "server_survival_furnace_fuel",
            ),
            (
                "server_survival_furnace_fuel",
                "server_survival_furnace_burn_progress",
            ),
            (
                "server_survival_furnace_burn_progress",
                "server_survival_furnace_output_available",
            ),
            (
                "server_survival_furnace_output_available",
                "server_survival_furnace_output_collect",
            ),
            (
                "server_survival_furnace_output_collect",
                "server_survival_furnace_reconnect_reopen",
            ),
            (
                "server_survival_furnace_reconnect_reopen",
                "server_survival_furnace_state",
            ),
        ],
        Scenario::SurvivalFurnaceSmeltingBreadth => vec![
            ("survival_furnace_open_seen", "survival_furnace_input_sent"),
            ("survival_furnace_input_sent", "survival_furnace_fuel_sent"),
            (
                "survival_furnace_fuel_sent",
                "survival_furnace_burn_progress_seen",
            ),
            (
                "survival_furnace_burn_progress_seen",
                "survival_furnace_output_seen",
            ),
            (
                "survival_furnace_output_seen",
                "survival_furnace_output_collected",
            ),
            (
                "survival_furnace_output_collected",
                "survival_furnace_invalid_fuel_sent",
            ),
            (
                "server_survival_furnace_open",
                "server_survival_furnace_input",
            ),
            (
                "server_survival_furnace_input",
                "server_survival_furnace_fuel",
            ),
            (
                "server_survival_furnace_fuel",
                "server_survival_furnace_burn_progress",
            ),
            (
                "server_survival_furnace_burn_progress",
                "server_survival_furnace_output_available",
            ),
            (
                "server_survival_furnace_output_available",
                "server_survival_furnace_output_collect",
            ),
            (
                "server_survival_furnace_output_collect",
                "server_survival_furnace_invalid_fuel_rejected",
            ),
            (
                "server_survival_furnace_invalid_fuel_rejected",
                "server_survival_furnace_breadth_state",
            ),
        ],
        Scenario::SurvivalHungerFood => vec![
            (
                "survival_hunger_food_pre_seen",
                "survival_hunger_food_use_sent",
            ),
            (
                "survival_hunger_food_use_sent",
                "survival_hunger_food_post_seen",
            ),
            (
                "survival_hunger_food_post_seen",
                "survival_hunger_food_inventory_updated",
            ),
            (
                "server_survival_hunger_food_pre",
                "server_survival_hunger_food_consume_start",
            ),
            (
                "server_survival_hunger_food_consume_start",
                "server_survival_hunger_food_consume_finish",
            ),
            (
                "server_survival_hunger_food_consume_finish",
                "server_survival_hunger_food_inventory",
            ),
            (
                "server_survival_hunger_food_inventory",
                "server_survival_hunger_food_state",
            ),
        ],
        Scenario::SurvivalHungerHealthCycle => vec![
            (
                "survival_hunger_health_pre_seen",
                "survival_hunger_health_consume_sent",
            ),
            (
                "survival_hunger_health_consume_sent",
                "survival_hunger_health_recovery_seen",
            ),
            (
                "survival_hunger_health_recovery_seen",
                "survival_hunger_health_inventory_updated",
            ),
            (
                "server_survival_hunger_health_pre",
                "server_survival_hunger_health_consume_start",
            ),
            (
                "server_survival_hunger_health_consume_start",
                "server_survival_hunger_health_consume_finish",
            ),
            (
                "server_survival_hunger_health_consume_finish",
                "server_survival_hunger_health_inventory",
            ),
            (
                "server_survival_hunger_health_inventory",
                "server_survival_hunger_health_state",
            ),
        ],
        Scenario::SurvivalCraftingRecipeBreadth => vec![
            (
                "survival_crafting_breadth_shaped_seen",
                "survival_crafting_breadth_shapeless_seen",
            ),
            (
                "survival_crafting_breadth_shapeless_seen",
                "survival_crafting_breadth_grid_clear_seen",
            ),
            (
                "survival_crafting_breadth_shapeless_seen",
                "survival_crafting_breadth_invalid_seen",
            ),
            (
                "survival_crafting_breadth_invalid_seen",
                "survival_crafting_breadth_inventory_updated",
            ),
            (
                "server_survival_crafting_breadth_shaped",
                "server_survival_crafting_breadth_shapeless",
            ),
            (
                "server_survival_crafting_breadth_shapeless",
                "server_survival_crafting_breadth_grid_clear",
            ),
            (
                "server_survival_crafting_breadth_grid_clear",
                "server_survival_crafting_breadth_invalid_rejected",
            ),
            (
                "server_survival_crafting_breadth_invalid_rejected",
                "server_survival_crafting_breadth_state",
            ),
        ],
        Scenario::SurvivalCraftingTable => vec![
            (
                "survival_crafting_table_open_seen",
                "survival_crafting_input_a_sent",
            ),
            (
                "survival_crafting_input_a_sent",
                "survival_crafting_input_b_sent",
            ),
            (
                "survival_crafting_input_b_sent",
                "survival_crafting_result_seen",
            ),
            (
                "survival_crafting_result_seen",
                "survival_crafting_result_collected",
            ),
            (
                "survival_crafting_result_collected",
                "survival_crafting_inventory_updated",
            ),
            (
                "server_survival_crafting_table_open",
                "server_survival_crafting_input_a",
            ),
            (
                "server_survival_crafting_input_a",
                "server_survival_crafting_input_b",
            ),
            (
                "server_survival_crafting_input_b",
                "server_survival_crafting_result",
            ),
            (
                "server_survival_crafting_result",
                "server_survival_crafting_collect",
            ),
        ],
        Scenario::SurvivalMobDrop => vec![
            (
                "survival_mob_drop_mob_seen",
                "survival_mob_drop_attack_sent",
            ),
            (
                "survival_mob_drop_attack_sent",
                "survival_mob_drop_death_seen",
            ),
            (
                "survival_mob_drop_death_seen",
                "survival_mob_drop_drop_seen",
            ),
            (
                "survival_mob_drop_drop_seen",
                "survival_mob_drop_pickup_seen",
            ),
            (
                "survival_mob_drop_pickup_seen",
                "survival_mob_drop_inventory_updated",
            ),
            (
                "server_survival_mob_drop_spawn",
                "server_survival_mob_drop_attack",
            ),
            (
                "server_survival_mob_drop_attack",
                "server_survival_mob_drop_death",
            ),
            (
                "server_survival_mob_drop_death",
                "server_survival_mob_drop_drop_spawn",
            ),
            (
                "server_survival_mob_drop_drop_spawn",
                "server_survival_mob_drop_pickup",
            ),
            (
                "server_survival_mob_drop_pickup",
                "server_survival_mob_drop_inventory",
            ),
            (
                "server_survival_mob_drop_inventory",
                "server_survival_mob_drop_state",
            ),
        ],
        Scenario::SurvivalMobAiLootBreadth => vec![
            (
                "survival_mob_ai_loot_mob_seen",
                "survival_mob_ai_loot_attack_sent",
            ),
            (
                "survival_mob_ai_loot_attack_sent",
                "survival_mob_ai_loot_death_seen",
            ),
            (
                "survival_mob_ai_loot_death_seen",
                "survival_mob_ai_loot_drop_seen",
            ),
            (
                "survival_mob_ai_loot_drop_seen",
                "survival_mob_ai_loot_pickup_seen",
            ),
            (
                "survival_mob_ai_loot_pickup_seen",
                "survival_mob_ai_loot_inventory_updated",
            ),
            (
                "server_survival_mob_ai_loot_spawn",
                "server_survival_mob_ai_loot_ai_checkpoint",
            ),
            (
                "server_survival_mob_ai_loot_ai_checkpoint",
                "server_survival_mob_ai_loot_attack",
            ),
            (
                "server_survival_mob_ai_loot_attack",
                "server_survival_mob_ai_loot_death",
            ),
            (
                "server_survival_mob_ai_loot_death",
                "server_survival_mob_ai_loot_drop_spawn",
            ),
            (
                "server_survival_mob_ai_loot_drop_spawn",
                "server_survival_mob_ai_loot_pickup",
            ),
            (
                "server_survival_mob_ai_loot_pickup",
                "server_survival_mob_ai_loot_inventory",
            ),
            (
                "server_survival_mob_ai_loot_inventory",
                "server_survival_mob_ai_loot_state",
            ),
        ],
        Scenario::SurvivalRedstoneToggle => vec![
            (
                "survival_redstone_toggle_input_sent",
                "survival_redstone_toggle_output_update",
            ),
            (
                "survival_redstone_toggle_output_update",
                "survival_redstone_toggle_return_input_sent",
            ),
            (
                "survival_redstone_toggle_return_input_sent",
                "survival_redstone_toggle_return_update",
            ),
            (
                "server_survival_redstone_toggle_input",
                "server_survival_redstone_toggle_powered_on",
            ),
            (
                "server_survival_redstone_toggle_powered_on",
                "server_survival_redstone_toggle_powered_off",
            ),
            (
                "server_survival_redstone_toggle_powered_off",
                "server_survival_redstone_toggle_state",
            ),
        ],
        Scenario::SurvivalRedstoneCircuitBreadth => vec![
            (
                "survival_redstone_circuit_initial_state",
                "survival_redstone_circuit_input_sent",
            ),
            (
                "survival_redstone_circuit_input_sent",
                "survival_redstone_circuit_output_update",
            ),
            (
                "survival_redstone_circuit_output_update",
                "survival_redstone_circuit_return_input_sent",
            ),
            (
                "survival_redstone_circuit_return_input_sent",
                "survival_redstone_circuit_return_update",
            ),
            (
                "server_survival_redstone_circuit_initial",
                "server_survival_redstone_circuit_input",
            ),
            (
                "server_survival_redstone_circuit_input",
                "server_survival_redstone_circuit_powered_on",
            ),
            (
                "server_survival_redstone_circuit_powered_on",
                "server_survival_redstone_circuit_powered_off",
            ),
            (
                "server_survival_redstone_circuit_powered_off",
                "server_survival_redstone_circuit_state",
            ),
        ],
        Scenario::SurvivalWorldPersistenceRestart => vec![
            (
                "survival_world_persistence_mutation_sent",
                "survival_world_persistence_pre_restart_update",
            ),
            (
                "survival_world_persistence_pre_restart_update",
                "survival_world_persistence_reconnect_sent",
            ),
            (
                "survival_world_persistence_reconnect_sent",
                "survival_world_persistence_post_restart_update",
            ),
            (
                "server_survival_world_persistence_mutation",
                "server_survival_world_persistence_clean_shutdown",
            ),
            (
                "server_survival_world_persistence_clean_shutdown",
                "server_survival_world_persistence_backend_restart",
            ),
            (
                "server_survival_world_persistence_backend_restart",
                "server_survival_world_persistence_post_restart",
            ),
            (
                "server_survival_world_persistence_post_restart",
                "server_survival_world_persistence_state",
            ),
        ],
        Scenario::SurvivalWorldMultichunkDurability => vec![
            (
                "survival_world_multichunk_mutation_sent",
                "survival_world_multichunk_pre_restart_update",
            ),
            (
                "survival_world_multichunk_pre_restart_update",
                "survival_world_multichunk_reconnect_sent",
            ),
            (
                "survival_world_multichunk_reconnect_sent",
                "survival_world_multichunk_post_restart_update",
            ),
            (
                "server_survival_world_multichunk_mutation",
                "server_survival_world_multichunk_clean_shutdown",
            ),
            (
                "server_survival_world_multichunk_clean_shutdown",
                "server_survival_world_multichunk_backend_restart",
            ),
            (
                "server_survival_world_multichunk_backend_restart",
                "server_survival_world_multichunk_post_restart",
            ),
            (
                "server_survival_world_multichunk_post_restart",
                "server_survival_world_multichunk_state",
            ),
        ],
        Scenario::SurvivalCrashRecoveryParity => vec![
            (
                "survival_crash_recovery_mutation_sent",
                "survival_crash_recovery_pre_crash_update",
            ),
            (
                "survival_crash_recovery_pre_crash_update",
                "survival_crash_recovery_reconnect_sent",
            ),
            (
                "survival_crash_recovery_reconnect_sent",
                "survival_crash_recovery_post_crash_update",
            ),
            (
                "server_survival_crash_recovery_mutation",
                "server_survival_crash_recovery_forced_stop",
            ),
            (
                "server_survival_crash_recovery_forced_stop",
                "server_survival_crash_recovery_backend_restart",
            ),
            (
                "server_survival_crash_recovery_backend_restart",
                "server_survival_crash_recovery_post_crash",
            ),
            (
                "server_survival_crash_recovery_post_crash",
                "server_survival_crash_recovery_state",
            ),
        ],
        Scenario::SurvivalBlockEntityPersistenceParity => vec![
            (
                "survival_block_entity_pre_restart_update",
                "survival_block_entity_reconnect_sent",
            ),
            (
                "survival_block_entity_reconnect_sent",
                "survival_block_entity_post_restart_update",
            ),
            (
                "server_survival_block_entity_mutation",
                "server_survival_block_entity_clean_shutdown",
            ),
            (
                "server_survival_block_entity_clean_shutdown",
                "server_survival_block_entity_backend_restart",
            ),
            (
                "server_survival_block_entity_backend_restart",
                "server_survival_block_entity_post_restart",
            ),
            (
                "server_survival_block_entity_post_restart",
                "server_survival_block_entity_state",
            ),
        ],
        Scenario::SurvivalContainerBlockEntityBreadth => vec![
            (
                "survival_container_block_entity_open_seen",
                "survival_container_block_entity_transfer_sent",
            ),
            (
                "survival_container_block_entity_transfer_sent",
                "survival_container_block_entity_payload_seen",
            ),
            (
                "survival_container_block_entity_payload_seen",
                "survival_container_block_entity_metadata_seen",
            ),
            (
                "survival_container_block_entity_metadata_seen",
                "survival_container_block_entity_reopen_seen",
            ),
            (
                "server_survival_container_block_entity_open",
                "server_survival_container_block_entity_transfer",
            ),
            (
                "server_survival_container_block_entity_transfer",
                "server_survival_container_block_entity_payload",
            ),
            (
                "server_survival_container_block_entity_payload",
                "server_survival_container_block_entity_metadata",
            ),
            (
                "server_survival_container_block_entity_metadata",
                "server_survival_container_block_entity_state",
            ),
        ],
        Scenario::SurvivalBiomeDimensionTravel => vec![
            (
                "survival_biome_dimension_travel_origin",
                "survival_biome_dimension_travel_transition_sent",
            ),
            (
                "survival_biome_dimension_travel_transition_sent",
                "survival_biome_dimension_travel_destination_seen",
            ),
            (
                "server_survival_biome_dimension_travel_origin",
                "server_survival_biome_dimension_travel_transition",
            ),
            (
                "server_survival_biome_dimension_travel_transition",
                "server_survival_biome_dimension_travel_state",
            ),
        ],
        Scenario::SurvivalSignEditingLive => vec![
            (
                "survival_sign_editing_open_seen",
                "survival_sign_editing_update_sent",
            ),
            (
                "survival_sign_editing_update_sent",
                "survival_sign_editing_post_update_seen",
            ),
            (
                "server_survival_sign_editing_open",
                "server_survival_sign_editing_update_accepted",
            ),
            (
                "server_survival_sign_editing_update_accepted",
                "server_survival_sign_editing_state",
            ),
        ],
        Scenario::FlagScoreRepeat => vec![
            ("team_red", "flag_pickup"),
            ("flag_pickup", "flag_capture"),
            ("flag_capture", "score_red_1"),
            ("score_red_1", "score_red_2"),
        ],
        Scenario::BlueFlagScore => vec![
            ("team_blue", "flag_pickup"),
            ("flag_pickup", "flag_capture"),
            ("flag_capture", "score_blue_1"),
        ],
        Scenario::CombatDamage => vec![
            ("remote_player_spawn", "combat_attack_sent"),
            ("combat_attack_sent", "combat_health_update"),
            ("server_client_a_seen", "server_combat_damage"),
        ],
        Scenario::CombatKnockback => vec![
            ("remote_player_spawn", "combat_attack_sent"),
            ("combat_attack_sent", "combat_health_update"),
            ("combat_health_update", "combat_velocity_update"),
            ("server_combat_damage", "server_combat_knockback"),
        ],
        Scenario::ArmorEquipmentMitigation => vec![
            ("armor_inventory_slot", "combat_attack_sent"),
            ("combat_attack_sent", "combat_health_update"),
            ("server_equipment_state", "server_combat_damage"),
            ("server_combat_damage", "server_armor_mitigation"),
        ],
        Scenario::EquipmentUpdateObservation => vec![
            ("remote_player_spawn", "entity_equipment_update"),
            ("server_client_b_seen", "server_equipment_update_state"),
        ],
        Scenario::ProjectileHit => vec![
            ("remote_player_spawn", "projectile_use_sent"),
            ("projectile_use_sent", "projectile_spawn_visible"),
            ("projectile_spawn_visible", "projectile_swing_sent"),
            ("projectile_swing_sent", "projectile_travel_observed"),
            ("server_client_a_seen", "server_projectile_loadout"),
            ("server_projectile_loadout", "server_projectile_use"),
            ("server_projectile_use", "server_projectile_travel_sample"),
            (
                "server_projectile_travel_sample",
                "server_projectile_collision",
            ),
            ("server_projectile_collision", "server_projectile_hit"),
        ],
        Scenario::ProjectileDamageAttribution => vec![
            ("remote_player_spawn", "projectile_use_sent"),
            ("projectile_use_sent", "projectile_swing_sent"),
            ("projectile_swing_sent", "projectile_damage_update"),
            ("server_projectile_loadout", "server_projectile_use"),
            ("server_projectile_use", "server_projectile_hit"),
        ],
        Scenario::FlagCarrierDeathReturn => vec![
            ("flag_pickup", "combat_attack_sent"),
            ("combat_attack_sent", "combat_death_observed"),
            ("combat_death_observed", "respawn_request_sent"),
            ("respawn_request_sent", "respawn_health_restored"),
            ("server_flag_pickup", "server_flag_carrier_death"),
            ("server_flag_carrier_death", "server_flag_return"),
        ],
        Scenario::ReconnectFlagState => vec![
            ("flag_pickup", "reconnect_session"),
            ("server_flag_pickup", "server_flag_disconnect_return"),
            (
                "server_flag_disconnect_return",
                "server_reconnect_state_coherent",
            ),
        ],
        Scenario::CtfInvalidPickupOwnership => vec![
            (
                "ctf_invalid_pickup_attempted",
                "ctf_invalid_pickup_contained",
            ),
            ("server_username_seen", "server_invalid_pickup_rejected"),
        ],
        Scenario::CtfInvalidReturnDrop => vec![
            (
                "ctf_invalid_return_drop_attempted",
                "ctf_invalid_return_drop_contained",
            ),
            (
                "server_username_seen",
                "server_invalid_return_drop_rejected",
            ),
        ],
        Scenario::CtfInvalidOpponentBaseReturnDrop => vec![
            (
                "ctf_invalid_opponent_base_return_drop_attempted",
                "ctf_invalid_opponent_base_return_drop_contained",
            ),
            (
                "server_username_seen",
                "server_invalid_opponent_base_return_drop_rejected",
            ),
        ],
        Scenario::CtfScoreLimitWinCondition => vec![
            ("team_red", "flag_pickup"),
            ("flag_pickup", "flag_capture"),
            ("flag_capture", "score_red_2"),
            ("score_red_2", "ctf_score_limit_win_seen"),
            (
                "server_score_limit_pre_state",
                "server_score_limit_final_capture",
            ),
            (
                "server_score_limit_final_capture",
                "server_score_limit_win_condition",
            ),
        ],
        Scenario::CtfSimultaneousPickupCaptureRace => vec![
            ("ctf_race_client_count", "flag_pickup"),
            ("flag_pickup", "flag_capture"),
            (
                "server_ctf_race_accepted_transition",
                "server_ctf_race_rejected_transition",
            ),
            (
                "server_ctf_race_rejected_transition",
                "server_ctf_race_final_state",
            ),
        ],
        Scenario::CtfSpawnTeamBalanceReset => vec![
            ("ctf_spawn_team_reset_client_count", "team_red"),
            ("team_red", "team_blue"),
            ("flag_pickup", "flag_capture"),
            (
                "server_ctf_spawn_red_assignment",
                "server_ctf_spawn_blue_assignment",
            ),
            (
                "server_ctf_spawn_blue_assignment",
                "server_ctf_spawn_team_balance",
            ),
            (
                "server_ctf_spawn_team_balance",
                "server_ctf_spawn_resource_reset",
            ),
        ],
        Scenario::ReconnectFlagScore => vec![
            ("flag_pickup", "flag_capture"),
            ("flag_capture", "score_red_1"),
            ("score_red_1", "reconnect_session"),
        ],
        Scenario::MultiClientLoadScore => vec![
            ("multi_client_count", "flag_pickup"),
            ("flag_pickup", "flag_capture"),
            ("flag_capture", "score_red_1"),
            ("server_client_a_seen", "server_client_b_seen"),
            ("server_client_b_seen", "server_flag_or_score"),
        ],
        _ => vec![],
    }
}

pub(crate) fn projectile_damage_required_steps() -> Vec<&'static str> {
    vec![
        "attacker_client_projectile_use_sent",
        "attacker_client_projectile_swing_sent",
        "server_projectile_use_attacker_victim",
        "server_projectile_hit_attacker_victim_health_delta",
        "victim_client_damage_update",
    ]
}

pub(crate) fn evaluate_projectile_damage_causality(
    client_logs: &[ClientLogSlice<'_>],
    server_log: &str,
    base_username: &str,
) -> ProjectileDamageCausalityEvidence {
    evaluate_projectile_damage_causality_for_damage(
        client_logs,
        server_log,
        base_username,
        PROJECTILE_DAMAGE_AMOUNT_NEEDLE,
    )
}

pub(crate) fn evaluate_projectile_damage_causality_for_damage(
    client_logs: &[ClientLogSlice<'_>],
    server_log: &str,
    base_username: &str,
    expected_damage_needle: &str,
) -> ProjectileDamageCausalityEvidence {
    let fallback_attacker = format!("{base_username}{PROJECTILE_DAMAGE_ATTACKER_SUFFIX}");
    let fallback_victim = format!("{base_username}{PROJECTILE_DAMAGE_VICTIM_SUFFIX}");
    let server_use = first_projectile_server_use(server_log, expected_damage_needle);
    let (attacker_username, victim_username) = server_use
        .as_ref()
        .map(|event| (event.attacker.clone(), event.victim.clone()))
        .unwrap_or_else(|| (fallback_attacker, fallback_victim));
    let server_hit = first_projectile_server_hit(
        server_log,
        &attacker_username,
        &victim_username,
        server_use.as_ref().map(|event| event.line),
    );
    let attacker_log = client_log_for(client_logs, &attacker_username);
    let victim_log = client_log_for(client_logs, &victim_username);

    let attacker_use = first_line_index(attacker_log, PROJECTILE_DAMAGE_CLIENT_USE_NEEDLE);
    let attacker_swing = first_line_index(attacker_log, PROJECTILE_DAMAGE_CLIENT_SWING_NEEDLE);
    let victim_health = server_hit
        .as_ref()
        .and_then(|hit| first_line_index(victim_log, &client_health_needle(&hit.health_after)));

    let mut observed_steps = Vec::new();
    let mut missing_steps = Vec::new();
    push_step_presence(
        &mut observed_steps,
        &mut missing_steps,
        "attacker_client_projectile_use_sent",
        attacker_use,
    );
    push_step_presence(
        &mut observed_steps,
        &mut missing_steps,
        "attacker_client_projectile_swing_sent",
        attacker_swing,
    );
    push_step_presence(
        &mut observed_steps,
        &mut missing_steps,
        "server_projectile_use_attacker_victim",
        server_use.as_ref().map(|event| event.line),
    );
    push_step_presence(
        &mut observed_steps,
        &mut missing_steps,
        "server_projectile_hit_attacker_victim_health_delta",
        server_hit.as_ref().map(|event| event.line),
    );
    push_step_presence(
        &mut observed_steps,
        &mut missing_steps,
        "victim_client_damage_update",
        victim_health,
    );

    let mut order_violations = Vec::new();
    if let (Some(use_line), Some(swing_line)) = (attacker_use, attacker_swing) {
        if use_line >= swing_line {
            order_violations.push("attacker_client_use_before_swing");
        }
    }
    if let Some(use_event) = &server_use {
        if let Some(hit_event) = &server_hit {
            if use_event.line >= hit_event.line {
                order_violations.push("server_projectile_use_before_hit");
            }
        } else if first_projectile_server_hit(
            server_log,
            &attacker_username,
            &victim_username,
            None,
        )
        .is_some_and(|hit_event| hit_event.line < use_event.line)
        {
            order_violations.push("server_projectile_use_before_hit");
        }
    }

    let passed = missing_steps.is_empty() && order_violations.is_empty();
    ProjectileDamageCausalityEvidence {
        required_steps: projectile_damage_required_steps(),
        observed_steps,
        missing_steps,
        order_violations,
        attacker_username,
        victim_username,
        passed,
    }
}

pub(crate) fn projectile_travel_collision_required_steps() -> Vec<&'static str> {
    vec![
        "attacker_client_projectile_use_sent",
        "attacker_client_projectile_spawn_visible",
        "attacker_client_projectile_swing_sent",
        "attacker_client_projectile_travel_observed",
        "server_projectile_use_attacker_target",
        "server_projectile_travel_sample",
        "server_projectile_collision",
        "server_projectile_hit_result",
    ]
}

pub(crate) fn evaluate_projectile_travel_collision(
    client_logs: &[ClientLogSlice<'_>],
    server_log: &str,
    base_username: &str,
) -> ProjectileTravelCollisionEvidence {
    let fallback_attacker = format!("{base_username}{PROJECTILE_DAMAGE_ATTACKER_SUFFIX}");
    let fallback_target = format!("{base_username}{PROJECTILE_DAMAGE_VICTIM_SUFFIX}");
    let use_events =
        projectile_travel_server_events(server_log, PROJECTILE_DAMAGE_SERVER_USE_NEEDLE);
    let travel_events = projectile_travel_server_events(
        server_log,
        PROJECTILE_TRAVEL_COLLISION_SERVER_TRAVEL_NEEDLE,
    );
    let collision_events = projectile_travel_server_events(
        server_log,
        PROJECTILE_TRAVEL_COLLISION_SERVER_COLLISION_NEEDLE,
    );
    let hit_events =
        projectile_travel_server_events(server_log, PROJECTILE_DAMAGE_SERVER_HIT_NEEDLE);

    let mut identity_violations = Vec::new();
    note_ambiguous_projectile_identity(&use_events, &mut identity_violations);
    note_ambiguous_projectile_identity(&travel_events, &mut identity_violations);
    note_ambiguous_projectile_identity(&collision_events, &mut identity_violations);
    note_ambiguous_projectile_identity(&hit_events, &mut identity_violations);

    let server_use = first_projectile_travel_event(&use_events);
    let server_travel = first_projectile_travel_event(&travel_events);
    let server_collision = first_projectile_travel_event(&collision_events);
    let server_hit = first_projectile_travel_event(&hit_events);

    let attacker_username = server_use
        .map(|event| event.attacker.clone())
        .unwrap_or_else(|| fallback_attacker.clone());
    let target_username = server_use
        .map(|event| event.target.clone())
        .unwrap_or_else(|| fallback_target.clone());
    let projectile_id = server_use
        .map(|event| event.projectile_id.clone())
        .unwrap_or_else(|| PROJECTILE_TRAVEL_COLLISION_PROJECTILE_ID.to_string());
    let attacker_log = client_log_for(client_logs, &attacker_username);

    let attacker_use = first_line_index(attacker_log, PROJECTILE_DAMAGE_CLIENT_USE_NEEDLE);
    let attacker_spawn = first_line_index(
        attacker_log,
        PROJECTILE_TRAVEL_COLLISION_CLIENT_SPAWN_NEEDLE,
    );
    let attacker_swing = first_line_index(attacker_log, PROJECTILE_DAMAGE_CLIENT_SWING_NEEDLE);
    let attacker_travel = first_line_index(
        attacker_log,
        PROJECTILE_TRAVEL_COLLISION_CLIENT_TRAVEL_NEEDLE,
    );

    let mut observed_steps = Vec::new();
    let mut missing_steps = Vec::new();
    push_step_presence(
        &mut observed_steps,
        &mut missing_steps,
        "attacker_client_projectile_use_sent",
        attacker_use,
    );
    push_step_presence(
        &mut observed_steps,
        &mut missing_steps,
        "attacker_client_projectile_spawn_visible",
        attacker_spawn,
    );
    push_step_presence(
        &mut observed_steps,
        &mut missing_steps,
        "attacker_client_projectile_swing_sent",
        attacker_swing,
    );
    push_step_presence(
        &mut observed_steps,
        &mut missing_steps,
        "attacker_client_projectile_travel_observed",
        attacker_travel,
    );
    push_step_presence(
        &mut observed_steps,
        &mut missing_steps,
        "server_projectile_use_attacker_target",
        server_use.map(|event| event.line),
    );
    push_step_presence(
        &mut observed_steps,
        &mut missing_steps,
        "server_projectile_travel_sample",
        server_travel.map(|event| event.line),
    );
    push_step_presence(
        &mut observed_steps,
        &mut missing_steps,
        "server_projectile_collision",
        server_collision.map(|event| event.line),
    );
    push_step_presence(
        &mut observed_steps,
        &mut missing_steps,
        "server_projectile_hit_result",
        server_hit.map(|event| event.line),
    );

    validate_projectile_travel_event_identity(
        server_use,
        &fallback_attacker,
        &fallback_target,
        &mut identity_violations,
    );
    validate_projectile_travel_event_identity(
        server_travel,
        &fallback_attacker,
        &fallback_target,
        &mut identity_violations,
    );
    validate_projectile_travel_event_identity(
        server_collision,
        &fallback_attacker,
        &fallback_target,
        &mut identity_violations,
    );
    validate_projectile_travel_event_identity(
        server_hit,
        &fallback_attacker,
        &fallback_target,
        &mut identity_violations,
    );
    if projectile_travel_forbidden_claim_present(server_log)
        || projectile_travel_forbidden_claim_present(attacker_log)
    {
        push_unique_violation(&mut identity_violations, "overbroad_parity_claim");
    }

    let mut order_violations = Vec::new();
    push_line_order_violation(
        &mut order_violations,
        attacker_use,
        attacker_spawn,
        "attacker_client_use_before_spawn_visible",
    );
    push_line_order_violation(
        &mut order_violations,
        attacker_spawn,
        attacker_swing,
        "attacker_client_spawn_visible_before_swing",
    );
    push_line_order_violation(
        &mut order_violations,
        attacker_swing,
        attacker_travel,
        "attacker_client_swing_before_travel_observed",
    );
    push_event_order_violation(
        &mut order_violations,
        server_use,
        server_travel,
        "server_projectile_use_before_travel",
    );
    push_event_order_violation(
        &mut order_violations,
        server_travel,
        server_collision,
        "server_projectile_travel_before_collision",
    );
    push_event_order_violation(
        &mut order_violations,
        server_collision,
        server_hit,
        "server_projectile_collision_before_hit",
    );
    if server_travel.is_none() && (server_collision.is_some() || server_hit.is_some()) {
        push_unique_violation(
            &mut order_violations,
            "server_collision_or_hit_without_travel",
        );
    }

    let passed =
        missing_steps.is_empty() && order_violations.is_empty() && identity_violations.is_empty();
    ProjectileTravelCollisionEvidence {
        selected: true,
        row_id: PROJECTILE_TRAVEL_COLLISION_ROW_ID,
        weapon_representative: PROJECTILE_TRAVEL_COLLISION_WEAPON_REPRESENTATIVE,
        projectile_representative: PROJECTILE_TRAVEL_COLLISION_PROJECTILE_REPRESENTATIVE,
        attacker_username,
        target_username,
        projectile_id,
        required_steps: projectile_travel_collision_required_steps(),
        observed_steps,
        missing_steps,
        order_violations,
        identity_violations,
        non_claims: PROJECTILE_TRAVEL_COLLISION_NON_CLAIMS.to_vec(),
        passed,
    }
}

fn push_step_presence(
    observed_steps: &mut Vec<&'static str>,
    missing_steps: &mut Vec<&'static str>,
    step: &'static str,
    line: Option<usize>,
) {
    if line.is_some() {
        observed_steps.push(step);
    } else {
        missing_steps.push(step);
    }
}

fn first_line_index(output: &str, needle: &str) -> Option<usize> {
    output.lines().position(|line| line.contains(needle))
}

fn client_log_for<'a>(client_logs: &'a [ClientLogSlice<'a>], username: &str) -> &'a str {
    client_logs
        .iter()
        .find(|log| log.username == username)
        .map(|log| log.output)
        .unwrap_or("")
}

fn push_line_order_violation(
    order_violations: &mut Vec<&'static str>,
    before: Option<usize>,
    after: Option<usize>,
    violation: &'static str,
) {
    if let (Some(before_line), Some(after_line)) = (before, after) {
        if before_line >= after_line {
            push_unique_violation(order_violations, violation);
        }
    }
}

fn push_event_order_violation(
    order_violations: &mut Vec<&'static str>,
    before: Option<&ProjectileTravelServerEvent>,
    after: Option<&ProjectileTravelServerEvent>,
    violation: &'static str,
) {
    push_line_order_violation(
        order_violations,
        before.map(|event| event.line),
        after.map(|event| event.line),
        violation,
    );
}

fn push_unique_violation(violations: &mut Vec<&'static str>, violation: &'static str) {
    if !violations.contains(&violation) {
        violations.push(violation);
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ProjectileTravelServerEvent {
    line: usize,
    attacker: String,
    target: String,
    projectile_id: String,
    weapon: String,
    sequence: String,
}

fn projectile_travel_server_events(
    server_log: &str,
    needle: &str,
) -> Vec<ProjectileTravelServerEvent> {
    let mut events = Vec::new();
    for (line, text) in server_log.lines().enumerate() {
        if !text.contains(needle) {
            continue;
        }
        let Some(projectile_id) = field_value(text, "projectile_id=") else {
            continue;
        };
        let Some(attacker) = field_value(text, "attacker=") else {
            continue;
        };
        let Some(target) = field_value(text, "target=").or_else(|| field_value(text, "victim="))
        else {
            continue;
        };
        let Some(sequence) = field_value(text, "sequence=") else {
            continue;
        };
        let weapon = field_value(text, "weapon=")
            .or_else(|| field_value(text, "item="))
            .unwrap_or("");
        let candidate = ProjectileTravelServerEvent {
            line,
            attacker: attacker.to_string(),
            target: target.to_string(),
            projectile_id: projectile_id.to_string(),
            weapon: weapon.to_string(),
            sequence: sequence.to_string(),
        };
        if !events
            .iter()
            .any(|event| projectile_travel_event_key_matches(event, &candidate))
        {
            events.push(candidate);
        }
    }
    events
}

fn projectile_travel_event_key_matches(
    left: &ProjectileTravelServerEvent,
    right: &ProjectileTravelServerEvent,
) -> bool {
    left.attacker == right.attacker
        && left.target == right.target
        && left.projectile_id == right.projectile_id
        && left.weapon == right.weapon
        && left.sequence == right.sequence
}

fn first_projectile_travel_event(
    events: &[ProjectileTravelServerEvent],
) -> Option<&ProjectileTravelServerEvent> {
    events
        .iter()
        .find(|event| event.projectile_id == PROJECTILE_TRAVEL_COLLISION_PROJECTILE_ID)
}

fn note_ambiguous_projectile_identity(
    events: &[ProjectileTravelServerEvent],
    identity_violations: &mut Vec<&'static str>,
) {
    let matching_events = events
        .iter()
        .filter(|event| event.projectile_id == PROJECTILE_TRAVEL_COLLISION_PROJECTILE_ID)
        .count();
    if matching_events > 1 {
        push_unique_violation(identity_violations, "ambiguous_projectile_identity");
    }
}

fn projectile_travel_forbidden_claim_present(text: &str) -> bool {
    PROJECTILE_TRAVEL_COLLISION_FORBIDDEN_PARITY_CLAIMS
        .iter()
        .any(|claim| text.contains(claim))
}

fn validate_projectile_travel_event_identity(
    event: Option<&ProjectileTravelServerEvent>,
    expected_attacker: &str,
    expected_target: &str,
    identity_violations: &mut Vec<&'static str>,
) {
    let Some(event) = event else {
        return;
    };
    if event.projectile_id != PROJECTILE_TRAVEL_COLLISION_PROJECTILE_ID {
        push_unique_violation(identity_violations, "wrong_projectile_identity");
    }
    if event.attacker != expected_attacker {
        push_unique_violation(identity_violations, "wrong_attacker");
    }
    if event.target != expected_target {
        push_unique_violation(identity_violations, "wrong_target");
    }
    if event.weapon != PROJECTILE_TRAVEL_COLLISION_WEAPON {
        push_unique_violation(identity_violations, "wrong_weapon");
    }
    if event.sequence != projectile_travel_collision_sequence_value() {
        push_unique_violation(identity_violations, "wrong_sequence");
    }
}

fn projectile_travel_collision_sequence_value() -> &'static str {
    PROJECTILE_DAMAGE_SEQUENCE_NEEDLE
        .strip_prefix("sequence=")
        .expect("projectile sequence needle carries field prefix")
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ProjectileServerUse {
    line: usize,
    attacker: String,
    victim: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ProjectileServerHit {
    line: usize,
    health_after: String,
}

fn first_projectile_server_use(
    server_log: &str,
    expected_damage_needle: &str,
) -> Option<ProjectileServerUse> {
    server_log.lines().enumerate().find_map(|(line, text)| {
        if !text.contains(PROJECTILE_DAMAGE_SERVER_USE_NEEDLE)
            || !text.contains(PROJECTILE_DAMAGE_SEQUENCE_NEEDLE)
            || !text.contains(expected_damage_needle)
        {
            return None;
        }
        Some(ProjectileServerUse {
            line,
            attacker: field_value(text, "attacker=")?.to_string(),
            victim: field_value(text, "victim=")?.to_string(),
        })
    })
}

fn first_projectile_server_hit(
    server_log: &str,
    attacker_username: &str,
    victim_username: &str,
    after_line: Option<usize>,
) -> Option<ProjectileServerHit> {
    let attacker_needle = format!("attacker={attacker_username}");
    let victim_needle = format!("victim={victim_username}");
    server_log.lines().enumerate().find_map(|(line, text)| {
        if after_line.is_some_and(|minimum_line| line <= minimum_line)
            || !text.contains(PROJECTILE_DAMAGE_SERVER_HIT_NEEDLE)
            || !text.contains(&attacker_needle)
            || !text.contains(&victim_needle)
        {
            return None;
        }
        Some(ProjectileServerHit {
            line,
            health_after: field_value(text, "victim_health_after=")?.to_string(),
        })
    })
}

fn field_value<'a>(line: &'a str, field: &str) -> Option<&'a str> {
    let value_start = line.find(field)? + field.len();
    let value = &line[value_start..];
    let value_end = value.find(char::is_whitespace).unwrap_or(value.len());
    Some(&value[..value_end])
}

fn client_health_needle(health_after: &str) -> String {
    format!("update_health health={health_after}")
}

#[cfg(test)]
#[path = "evidence_core_colocated_tests.rs"]
mod root_colocated_tests;
