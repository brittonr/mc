#![allow(unused_imports)]

use super::*;
use crate::test_support::*;
use crate::*;
use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::time::Duration;

const ALL_TEST_SCENARIOS: &[Scenario] = ALL_SCENARIOS;

fn passing_client_lines(scenario: Scenario) -> Vec<(&'static str, String)> {
    scenario_required_milestones(scenario)
        .iter()
        .map(|(name, needle)| (*name, (*needle).to_string()))
        .collect()
}

fn passing_client_output(scenario: Scenario) -> String {
    output_from_lines(&passing_client_lines(scenario))
}

fn passing_server_lines(scenario: Scenario) -> Vec<(&'static str, String)> {
    server_required_milestones(scenario)
        .iter()
        .map(|(name, needle)| (*name, server_fixture_line_for(name, needle)))
        .collect()
}

fn output_from_lines(lines: &[(&'static str, String)]) -> String {
    lines
        .iter()
        .map(|(_, line)| line.as_str())
        .collect::<Vec<_>>()
        .join("\n")
}

fn output_without_line(lines: &[(&'static str, String)], omitted: &'static str) -> String {
    lines
        .iter()
        .filter(|(name, _)| *name != omitted)
        .map(|(_, line)| line.as_str())
        .collect::<Vec<_>>()
        .join("\n")
}

fn server_fixture_line_for(name: &str, needle: &str) -> String {
    match name {
        "server_username_seen" => "compatbot".to_string(),
        "server_client_a_seen" => "compatbota".to_string(),
        "server_client_b_seen" => "compatbotb".to_string(),
        "server_flag_or_score" => "flag".to_string(),
        _ => needle.to_string(),
    }
}

#[test]
fn evidence_matchers_cover_supported_positive_cases() {
    let corpus = EvidenceCorpus::new(
        "Detected server protocol version\nCompatBotA joined\nSCOREBOARD flag update\nupdate_health health=17.0\n",
    );
    let context = EvidenceContext {
        username: "CompatBot",
    };

    assert!(MatcherKind::Literal("Detected server protocol version").is_match(&corpus, &context));
    assert!(MatcherKind::CaseInsensitive("scoreboard").is_match(&corpus, &context));
    assert!(MatcherKind::DynamicUsername.is_match(&corpus, &context));
    assert!(MatcherKind::DynamicClientSuffix(CLIENT_A_SUFFIX).is_match(&corpus, &context));
    assert!(MatcherKind::AnyOfCaseInsensitive(FLAG_OR_SCORE_NEEDLES).is_match(&corpus, &context));

    let client = evaluate_scenario_with_projectile_health(
        Scenario::ProjectileDamageAttribution,
        "mc_compat_projectile_damage_client_count=2\nDetected server protocol version\njoin_game\nrender_tick_with_player\nYou are on team RED!\nYou are on team BLUE!\nremote_player_spawn\nprojectile_probe_use_item_sent\nprojectile_probe_swing_sent\ncustom projectile health\n",
        "custom projectile health",
    );
    assert!(client.passed, "{client:?}");
    assert!(
        client
            .observed_milestones
            .contains(&PROJECTILE_DAMAGE_UPDATE_MILESTONE),
        "{client:?}"
    );
}

#[test]
fn evidence_matchers_fail_closed_for_missing_case_and_dynamic_context() {
    let corpus = EvidenceCorpus::new("compatbota joined\nred flag captured\n");
    let context = EvidenceContext {
        username: "compatbot",
    };

    assert!(!MatcherKind::Literal("COMPATBOTA").is_match(&corpus, &context));
    assert!(!MatcherKind::DynamicClientSuffix(CLIENT_B_SUFFIX).is_match(&corpus, &context));
    assert!(!MatcherKind::DynamicUsername.is_match(
        &corpus,
        &EvidenceContext {
            username: "otherbot"
        }
    ));
    assert!(!MatcherKind::AnyOfCaseInsensitive(&["capture"])
        .is_match(&EvidenceCorpus::new("no matching evidence"), &context));

    let server = evaluate_server_scenario(
        Scenario::MultiClientLoadScore,
        "compatbota joined\nred flag captured\n",
        "compatbot",
    );
    assert!(!server.passed, "{server:?}");
    assert!(server.missing_milestones.contains(&"server_client_b_seen"));

    let forbidden = evaluate_scenario(
        Scenario::NegativeCustomPayload,
        "Detected server protocol version\njoin_game\nrender_tick_with_player\nnegative_custom_payload_sent\nnegative_custom_payload_contained\npanicked\n",
    );
    assert!(!forbidden.passed, "{forbidden:?}");
    assert!(forbidden.forbidden_matches.contains(&"panic"));
}

#[test]
fn scenario_oracle_property_all_required_client_milestones_matter() {
    for scenario in ALL_TEST_SCENARIOS {
        let lines = passing_client_lines(*scenario);
        let full = evaluate_scenario(*scenario, &output_from_lines(&lines));
        assert!(
            full.passed,
            "{scenario:?} complete fixture failed: {full:?}"
        );

        for (milestone, _) in &lines {
            let mutated = evaluate_scenario(*scenario, &output_without_line(&lines, milestone));
            assert!(
                !mutated.passed,
                "{scenario:?} passed after removing client milestone {milestone}"
            );
            assert!(
                mutated.missing_milestones.contains(milestone),
                "{scenario:?} missing diagnostic for removed client milestone {milestone}: {mutated:?}"
            );
        }
    }
}

#[test]
fn scenario_oracle_property_all_required_server_milestones_matter() {
    for scenario in ALL_TEST_SCENARIOS {
        let lines = passing_server_lines(*scenario);
        let full = evaluate_server_scenario(*scenario, &output_from_lines(&lines), "compatbot");
        assert!(
            full.passed,
            "{scenario:?} complete server fixture failed: {full:?}"
        );

        for (milestone, _) in &lines {
            let full_output = output_from_lines(&lines);
            let mutated_output = match *milestone {
                "server_username_seen" => full_output.replace("compatbot", "otherbot"),
                "server_client_a_seen" => full_output.replace("compatbota", "otherbota"),
                "server_client_b_seen" => full_output.replace("compatbotb", "otherbotb"),
                _ => output_without_line(&lines, milestone),
            };
            let mutated = evaluate_server_scenario(*scenario, &mutated_output, "compatbot");
            assert!(
                !mutated.passed,
                "{scenario:?} passed after removing server milestone {milestone}"
            );
            assert!(
                mutated.missing_milestones.contains(milestone),
                "{scenario:?} missing diagnostic for removed server milestone {milestone}: {mutated:?}"
            );
        }
    }
}

#[test]
fn scenario_oracle_property_forbidden_markers_fail() {
    for scenario in ALL_TEST_SCENARIOS {
        let base = passing_client_output(*scenario);
        for (forbidden_name, forbidden_needle) in scenario_forbidden_patterns(*scenario) {
            let mutated = format!("{base}\n{forbidden_needle}\n");
            let evidence = evaluate_scenario(*scenario, &mutated);
            assert!(
                !evidence.passed,
                "{scenario:?} passed after forbidden marker {forbidden_name}"
            );
            assert!(
                evidence.forbidden_matches.contains(forbidden_name),
                "{scenario:?} missing forbidden diagnostic {forbidden_name}: {evidence:?}"
            );
        }
    }
}

#[test]
fn enriched_triage_core_bounds_and_redacts_context() {
    let scenario = evaluate_scenario(
        Scenario::FlagScoreRepeat,
        "Detected server protocol version 763",
    );
    let server = evaluate_server_scenario(Scenario::FlagScoreRepeat, "compatbot", "compatbot");
    let usernames = vec!["compatbot".to_string()];
    let triage = build_enriched_triage(EnrichedTriageInput {
        scenario: &scenario,
        server_scenario: &server,
        scenario_name: "flag-score-repeat",
        usernames: &usernames,
        error: Some("token=secret /tmp/private/server.log"),
        first_missing_client: scenario.missing_milestones.first().copied(),
        first_missing_server: server.missing_milestones.first().copied(),
        first_forbidden_source: None,
        first_forbidden_pattern: None,
        suggested_boundary: "client-probe",
    });

    assert_eq!(triage.boundary_confidence, TRIAGE_CONFIDENCE_MEDIUM);
    assert!(triage
        .correlation_ids
        .contains(&"scenario=flag-score-repeat".to_string()));
    assert!(triage
        .correlation_ids
        .contains(&"user=compatbot".to_string()));
    assert!(triage
        .timeline_excerpt
        .iter()
        .any(|line| line.contains(TRIAGE_REDACTED)));
    assert!(triage
        .timeline_excerpt
        .iter()
        .all(|line| line.chars().count() <= TRIAGE_MAX_EXCERPT_CHARS));
}

const TEST_SESSION_ID: &str = "s1";
const TEST_USERNAME: &str = "compatbot";
const TEST_CLIENT_DIR: &str = "/tmp/stevenarella";
const TEST_ATTACKER_USERNAME: &str = "compatbota";
const TEST_VICTIM_USERNAME: &str = "compatbotb";

type TypedEventFixtureStep = (&'static str, Option<&'static str>, &'static str);

fn typed_event_fixture_from_steps(
    scenario: Scenario,
    steps: &[TypedEventFixtureStep],
) -> Vec<TypedEvent> {
    let scenario_label = scenario_name(scenario);
    steps
        .iter()
        .enumerate()
        .map(|(index, step)| {
            let (source, username, kind) = *step;
            let sequence_index = index + TYPED_EVENT_SEQUENCE_INDEX_OFFSET;
            let sequence =
                u32::try_from(sequence_index).expect("fixture sequence fits in u32");
            let username_field = username
                .map(|name| format!(" username={name}"))
                .unwrap_or_default();
            let line = format!(
                "{TYPED_EVENT_PREFIX} schema={TYPED_EVENT_SCHEMA_VERSION} source={source} scenario={scenario_label} session={TEST_SESSION_ID}{username_field} seq={sequence} event={kind}"
            );
            parse_typed_event_line(&line).expect("typed event fixture parses")
        })
        .collect()
}

fn typed_event_fixture_required_events(steps: &[TypedEventFixtureStep]) -> Vec<&'static str> {
    steps.iter().map(|(_, _, kind)| *kind).collect()
}

fn assert_typed_event_fixture_passes(
    scenario: Scenario,
    username: Option<&str>,
    steps: &[TypedEventFixtureStep],
    ordered_edges: &[(&str, &str)],
) {
    let events = typed_event_fixture_from_steps(scenario, steps);
    let required_events = typed_event_fixture_required_events(steps);
    let result = evaluate_typed_event_graph(
        &events,
        scenario_name(scenario),
        TEST_SESSION_ID,
        username,
        &required_events,
        &["panic"],
        ordered_edges,
    );

    assert_eq!(events.len(), steps.len());
    assert!(events
        .iter()
        .all(|event| event.schema_version == TYPED_EVENT_SCHEMA_VERSION));
    assert!(events
        .iter()
        .all(|event| event.scenario == scenario_name(scenario)));
    assert!(events.iter().all(|event| event.session == TEST_SESSION_ID));
    assert!(events
        .iter()
        .all(|event| matches!(event.source.as_str(), "client" | "server" | "mcp")));
    assert!(events
        .windows(2)
        .all(|pair| pair[0].sequence < pair[1].sequence));
    assert!(result.passed, "{result:?}");
    assert_eq!(result.observed_events.len(), required_events.len());
    assert!(result.missing_events.is_empty(), "{result:?}");
    assert!(result.forbidden_events.is_empty(), "{result:?}");
    assert!(result.order_violations.is_empty(), "{result:?}");
}

fn assert_typed_event_fixture_rejects_missing_event(
    scenario: Scenario,
    username: Option<&str>,
    steps: &[TypedEventFixtureStep],
    missing_event: &'static str,
) {
    let events = typed_event_fixture_from_steps(scenario, steps);
    let mut required_events = typed_event_fixture_required_events(steps);
    required_events.push(missing_event);
    let result = evaluate_typed_event_graph(
        &events,
        scenario_name(scenario),
        TEST_SESSION_ID,
        username,
        &required_events,
        &["panic"],
        &[],
    );

    assert!(!result.passed, "{result:?}");
    assert!(
        result.missing_events.contains(&missing_event.to_string()),
        "{result:?}"
    );
}

fn assert_typed_event_fixture_rejects_order(
    scenario: Scenario,
    username: Option<&str>,
    steps: &[TypedEventFixtureStep],
    before: &'static str,
    after: &'static str,
) {
    let events = typed_event_fixture_from_steps(scenario, steps);
    let required_events = typed_event_fixture_required_events(steps);
    let result = evaluate_typed_event_graph(
        &events,
        scenario_name(scenario),
        TEST_SESSION_ID,
        username,
        &required_events,
        &["panic"],
        &[(before, after)],
    );
    let expected_violation = format!("{before}_before_{after}");

    assert!(!result.passed, "{result:?}");
    assert!(
        result.order_violations.contains(&expected_violation),
        "{result:?}"
    );
}

#[test]
fn typed_event_parser_accepts_versioned_event_lines() {
    let event = parse_typed_event_line(typed_event_fixture_lines()[0]).expect("event parses");

    assert_eq!(event.schema_version, TYPED_EVENT_SCHEMA_VERSION);
    assert_eq!(event.source, "client");
    assert_eq!(event.scenario, "smoke");
    assert_eq!(event.session, "s1");
    assert_eq!(event.username.as_deref(), Some("compatbot"));
    assert_eq!(event.sequence, 1);
    assert_eq!(event.kind, "protocol_detected");

    let wrong_schema = parse_typed_event_line(
        "MC-COMPAT-EVENT schema=2 source=client scenario=smoke session=s1 username=compatbot seq=1 event=protocol_detected",
    )
    .unwrap_err();
    assert!(
        wrong_schema.contains("unsupported typed event schema"),
        "{wrong_schema}"
    );
}

#[test]
fn typed_event_pass_fail_gate_includes_only_migrated_rows() {
    assert!(typed_event_oracle_contributes_to_pass_fail(Scenario::Smoke));
    assert!(typed_event_oracle_contributes_to_pass_fail(
        Scenario::McpControlledSmoke
    ));
    assert!(typed_event_oracle_contributes_to_pass_fail(
        Scenario::InventoryInteraction
    ));
    assert!(typed_event_oracle_contributes_to_pass_fail(
        Scenario::InventoryStackSplitMerge
    ));
    assert!(typed_event_oracle_contributes_to_pass_fail(
        Scenario::InventoryDragTransactions
    ));
    assert!(typed_event_oracle_contributes_to_pass_fail(
        Scenario::SurvivalBreakPlacePickup
    ));
    assert!(typed_event_oracle_contributes_to_pass_fail(
        Scenario::SurvivalCraftingTable
    ));
    assert!(typed_event_oracle_contributes_to_pass_fail(
        Scenario::SurvivalCraftingRecipeBreadth
    ));
    assert!(typed_event_oracle_contributes_to_pass_fail(
        Scenario::SurvivalChestPersistence
    ));
    assert!(typed_event_oracle_contributes_to_pass_fail(
        Scenario::SurvivalHungerHealthCycle
    ));
    assert!(typed_event_oracle_contributes_to_pass_fail(
        Scenario::SurvivalFurnacePersistence
    ));
    assert!(typed_event_oracle_contributes_to_pass_fail(
        Scenario::SurvivalFurnaceSmeltingBreadth
    ));
    assert!(typed_event_oracle_contributes_to_pass_fail(
        Scenario::SurvivalHungerFood
    ));
    assert!(typed_event_oracle_contributes_to_pass_fail(
        Scenario::SurvivalMobDrop
    ));
    assert!(typed_event_oracle_contributes_to_pass_fail(
        Scenario::SurvivalMobAiLootBreadth
    ));
    assert!(typed_event_oracle_contributes_to_pass_fail(
        Scenario::SurvivalRedstoneToggle
    ));
    assert!(typed_event_oracle_contributes_to_pass_fail(
        Scenario::SurvivalRedstoneCircuitBreadth
    ));
    assert!(typed_event_oracle_contributes_to_pass_fail(
        Scenario::SurvivalWorldPersistenceRestart
    ));
    assert!(typed_event_oracle_contributes_to_pass_fail(
        Scenario::SurvivalWorldMultichunkDurability
    ));
    assert!(typed_event_oracle_contributes_to_pass_fail(
        Scenario::SurvivalCrashRecoveryParity
    ));
    assert!(typed_event_oracle_contributes_to_pass_fail(
        Scenario::SurvivalBlockEntityPersistenceParity
    ));
    assert!(typed_event_oracle_contributes_to_pass_fail(
        Scenario::SurvivalContainerBlockEntityBreadth
    ));
    assert!(typed_event_oracle_contributes_to_pass_fail(
        Scenario::SurvivalBiomeDimensionState
    ));
    assert!(typed_event_oracle_contributes_to_pass_fail(
        Scenario::SurvivalBiomeDimensionTravel
    ));
    assert!(typed_event_oracle_contributes_to_pass_fail(
        Scenario::SurvivalSignEditingLive
    ));
    for scenario in [
        Scenario::FlagScoreRepeat,
        Scenario::BlueFlagScore,
        Scenario::CombatDamage,
        Scenario::CombatKnockback,
        Scenario::ArmorEquipmentMitigation,
        Scenario::EquipmentUpdateObservation,
        Scenario::ProjectileHit,
        Scenario::ProjectileDamageAttribution,
        Scenario::FlagCarrierDeathReturn,
        Scenario::ReconnectFlagState,
        Scenario::CtfInvalidPickupOwnership,
        Scenario::CtfInvalidReturnDrop,
        Scenario::CtfInvalidOpponentBaseReturnDrop,
        Scenario::CtfScoreLimitWinCondition,
        Scenario::CtfSimultaneousPickupCaptureRace,
        Scenario::CtfSpawnTeamBalanceReset,
        Scenario::ReconnectFlagScore,
        Scenario::MultiClientLoadScore,
    ] {
        assert!(
            typed_event_oracle_contributes_to_pass_fail(scenario),
            "{scenario:?} should use typed-event pass/fail"
        );
    }
    for scenario in [Scenario::CompatBotProbe] {
        assert!(
            !typed_event_oracle_contributes_to_pass_fail(scenario),
            "{scenario:?} should stay substring fallback"
        );
    }
}

fn biome_dimension_join_state_matching_record() -> BiomeDimensionJoinStateRecord {
    let client = evaluate_scenario(
        Scenario::SurvivalBiomeDimensionState,
        &format!(
            "Detected server protocol version {DEFAULT_SERVER_PROTOCOL}\njoin_game\nrender_tick_with_player\n{SURVIVAL_BIOME_DIMENSION_CLIENT_STATE_NEEDLE}\n"
        ),
    );
    let server = evaluate_server_scenario(
        Scenario::SurvivalBiomeDimensionState,
        SURVIVAL_BIOME_DIMENSION_SERVER_STATE_NEEDLE,
        TEST_USERNAME,
    );
    let evidence = evaluate_biome_dimension_join_state(
        Scenario::SurvivalBiomeDimensionState,
        DEFAULT_SERVER_PROTOCOL,
        &client,
        &server,
    );
    assert!(evidence.validation.passed, "{evidence:?}");
    evidence.record
}

#[test]
fn biome_dimension_join_state_validator_accepts_matching_fixture() {
    let record = biome_dimension_join_state_matching_record();
    let validation = validate_biome_dimension_join_state_record(&record);
    assert!(validation.passed, "{validation:?}");
    assert_eq!(record.protocol, Some(DEFAULT_SERVER_PROTOCOL));
    assert!(record
        .non_claims
        .iter()
        .any(|claim| claim == "not_dimension_travel"));
    let client = record
        .client_observed_state
        .as_ref()
        .expect("client join-state fields are recorded");
    let server = record
        .server_configured_state
        .as_ref()
        .expect("server join-state fields are recorded");
    assert_eq!(client.normalized_identifier, server.normalized_identifier);
    assert_eq!(client.environment_identifier, server.environment_identifier);
}

#[test]
fn biome_dimension_join_state_validator_rejects_weak_evidence() {
    let matching = biome_dimension_join_state_matching_record();

    let mut client_only = matching.clone();
    client_only.server_configured_state = None;
    let client_only_validation = validate_biome_dimension_join_state_record(&client_only);
    assert!(!client_only_validation.passed);
    assert!(client_only_validation
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic == "missing_server_configured_state"));

    let mut mismatch = matching.clone();
    mismatch
        .server_configured_state
        .as_mut()
        .expect("server fixture exists")
        .normalized_identifier = "minecraft:the_nether".to_string();
    let mismatch_validation = validate_biome_dimension_join_state_record(&mismatch);
    assert!(!mismatch_validation.passed);
    assert!(mismatch_validation
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic == "mismatched_normalized_identifier"));

    let mut missing_protocol = matching.clone();
    missing_protocol.protocol = None;
    let missing_protocol_validation = validate_biome_dimension_join_state_record(&missing_protocol);
    assert!(!missing_protocol_validation.passed);
    assert!(missing_protocol_validation
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic == "missing_protocol_context"));

    let mut overbroad = matching;
    overbroad.non_claims = biome_dimension_join_state_required_non_claims()
        .iter()
        .map(|claim| (*claim).to_string())
        .collect();
    overbroad.non_claims.push("dimension_travel".to_string());
    let overbroad_validation = validate_biome_dimension_join_state_record(&overbroad);
    assert!(!overbroad_validation.passed);
    assert!(overbroad_validation
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic == "overbroad_claim:dimension_travel"));
}

#[test]
fn typed_event_graph_checks_required_forbidden_and_ordered_events() {
    let events = typed_event_fixture();
    let pass = evaluate_typed_event_graph(
        &events,
        "smoke",
        "s1",
        Some("compatbot"),
        &["protocol_detected", "join_game", "render_tick"],
        &["panic"],
        &[("protocol_detected", "render_tick")],
    );
    assert!(pass.passed, "{pass:?}");

    let missing_required = evaluate_typed_event_graph(
        &events,
        "smoke",
        "s1",
        Some("compatbot"),
        &["protocol_detected", "missing_event"],
        &[],
        &[],
    );
    assert!(!missing_required.passed, "{missing_required:?}");
    assert!(missing_required
        .missing_events
        .contains(&"missing_event".to_string()));

    let wrong_username = evaluate_typed_event_graph(
        &events,
        "smoke",
        "s1",
        Some("otherbot"),
        &["protocol_detected"],
        &[],
        &[],
    );
    assert!(!wrong_username.passed, "{wrong_username:?}");
    assert!(wrong_username
        .missing_events
        .contains(&"protocol_detected".to_string()));

    let wrong_session = evaluate_typed_event_graph(
        &events,
        "smoke",
        "s2",
        Some("compatbot"),
        &["protocol_detected"],
        &[],
        &[],
    );
    assert!(!wrong_session.passed, "{wrong_session:?}");
    assert!(wrong_session
        .missing_events
        .contains(&"protocol_detected".to_string()));

    let mut forbidden_events = events.clone();
    forbidden_events.push(parse_typed_event_line(
        "MC-COMPAT-EVENT schema=1 source=client scenario=smoke session=s1 username=compatbot seq=4 event=panic",
    )
    .expect("forbidden event parses"));
    let forbidden = evaluate_typed_event_graph(
        &forbidden_events,
        "smoke",
        "s1",
        Some("compatbot"),
        &["protocol_detected"],
        &["panic"],
        &[],
    );
    assert!(!forbidden.passed, "{forbidden:?}");
    assert!(forbidden.forbidden_events.contains(&"panic".to_string()));

    let out_of_order = vec![
        parse_typed_event_line(
            "MC-COMPAT-EVENT schema=1 source=client scenario=smoke session=s1 username=compatbot seq=2 event=protocol_detected",
        )
        .expect("late event parses"),
        parse_typed_event_line(
            "MC-COMPAT-EVENT schema=1 source=client scenario=smoke session=s1 username=compatbot seq=1 event=render_tick",
        )
        .expect("early event parses"),
    ];
    let ordered = evaluate_typed_event_graph(
        &out_of_order,
        "smoke",
        "s1",
        Some("compatbot"),
        &["protocol_detected", "render_tick"],
        &[],
        &[("protocol_detected", "render_tick")],
    );
    assert!(!ordered.passed, "{ordered:?}");
    assert!(ordered
        .order_violations
        .contains(&"protocol_detected_before_render_tick".to_string()));

    let stale_sequence = vec![
        parse_typed_event_line(
            "MC-COMPAT-EVENT schema=1 source=client scenario=smoke session=s1 username=compatbot seq=7 event=protocol_detected",
        )
        .expect("first duplicate sequence event parses"),
        parse_typed_event_line(
            "MC-COMPAT-EVENT schema=1 source=client scenario=smoke session=s1 username=compatbot seq=7 event=render_tick",
        )
        .expect("stale sequence event parses"),
    ];
    let stale = evaluate_typed_event_graph(
        &stale_sequence,
        "smoke",
        "s1",
        Some("compatbot"),
        &["protocol_detected", "render_tick"],
        &[],
        &[("protocol_detected", "render_tick")],
    );
    assert!(!stale.passed, "{stale:?}");
    assert!(stale
        .order_violations
        .contains(&"protocol_detected_before_render_tick".to_string()));
}

#[test]
fn typed_event_graph_accepts_representative_scenario_fixtures() {
    assert_typed_event_fixture_passes(
        Scenario::Smoke,
        Some(TEST_USERNAME),
        &[
            ("client", Some(TEST_USERNAME), "protocol_detected"),
            ("client", Some(TEST_USERNAME), "join_game"),
            ("client", Some(TEST_USERNAME), "render_tick"),
        ],
        &[("protocol_detected", "render_tick")],
    );
    assert_typed_event_fixture_passes(
        Scenario::McpControlledSmoke,
        Some(TEST_USERNAME),
        &[
            ("client", Some(TEST_USERNAME), "mcp_initialize"),
            ("client", Some(TEST_USERNAME), "mcp_tools_list"),
            ("client", Some(TEST_USERNAME), "mcp_status_call"),
            ("mcp", Some(TEST_USERNAME), "mcp_stdout_clean"),
            ("mcp", Some(TEST_USERNAME), "mcp_look_call"),
            ("mcp", Some(TEST_USERNAME), "mcp_input_call"),
            ("mcp", Some(TEST_USERNAME), "mcp_capture_latest_frame"),
            ("mcp", Some(TEST_USERNAME), "mcp_frame_artifact_identity"),
            ("client", Some(TEST_USERNAME), "mcp_command_outcomes"),
        ],
        &[
            ("mcp_initialize", "mcp_tools_list"),
            ("mcp_tools_list", "mcp_status_call"),
            ("mcp_status_call", "mcp_look_call"),
            ("mcp_capture_latest_frame", "mcp_frame_artifact_identity"),
        ],
    );
    assert_typed_event_fixture_passes(
        Scenario::InventoryInteraction,
        Some(TEST_USERNAME),
        &[
            ("client", Some(TEST_USERNAME), "protocol_detected"),
            ("client", Some(TEST_USERNAME), "join_game"),
            ("client", Some(TEST_USERNAME), "render_tick"),
            ("client", Some(TEST_USERNAME), "team_red"),
            ("client", Some(TEST_USERNAME), "inventory_slot_update"),
            ("client", Some(TEST_USERNAME), "inventory_drop_sent"),
            ("client", Some(TEST_USERNAME), "inventory_pickup_seen"),
            ("client", Some(TEST_USERNAME), "inventory_click_sent"),
            ("client", Some(TEST_USERNAME), "inventory_block_place_sent"),
            (
                "server",
                Some(TEST_USERNAME),
                "server_inventory_hotbar_select",
            ),
            ("server", Some(TEST_USERNAME), "server_inventory_drop"),
            ("server", Some(TEST_USERNAME), "server_inventory_pickup"),
            ("server", Some(TEST_USERNAME), "server_inventory_click"),
            (
                "server",
                Some(TEST_USERNAME),
                "server_inventory_container_click",
            ),
            ("server", Some(TEST_USERNAME), "server_block_place"),
        ],
        &[
            ("protocol_detected", "inventory_drop_sent"),
            ("inventory_drop_sent", "inventory_pickup_seen"),
            ("server_inventory_drop", "server_inventory_pickup"),
            ("server_inventory_container_click", "server_block_place"),
        ],
    );
    assert_typed_event_fixture_passes(
        Scenario::InventoryDragTransactions,
        Some(TEST_USERNAME),
        &[
            ("client", Some(TEST_USERNAME), "protocol_detected"),
            ("client", Some(TEST_USERNAME), "join_game"),
            ("client", Some(TEST_USERNAME), "render_tick"),
            ("client", Some(TEST_USERNAME), "team_red"),
            ("client", Some(TEST_USERNAME), "inventory_drag_initial_slot"),
            ("client", Some(TEST_USERNAME), "inventory_drag_pickup_sent"),
            (
                "client",
                Some(TEST_USERNAME),
                "inventory_drag_source_empty_seen",
            ),
            ("client", Some(TEST_USERNAME), "inventory_drag_start_sent"),
            (
                "client",
                Some(TEST_USERNAME),
                "inventory_drag_target_a_sent",
            ),
            (
                "client",
                Some(TEST_USERNAME),
                "inventory_drag_target_b_sent",
            ),
            ("client", Some(TEST_USERNAME), "inventory_drag_end_sent"),
            (
                "client",
                Some(TEST_USERNAME),
                "inventory_drag_final_distribution_seen",
            ),
            ("server", Some(TEST_USERNAME), "server_username_seen"),
            (
                "server",
                Some(TEST_USERNAME),
                "server_inventory_drag_pickup",
            ),
            ("server", Some(TEST_USERNAME), "server_inventory_drag_start"),
            (
                "server",
                Some(TEST_USERNAME),
                "server_inventory_drag_target_a",
            ),
            (
                "server",
                Some(TEST_USERNAME),
                "server_inventory_drag_target_b",
            ),
            ("server", Some(TEST_USERNAME), "server_inventory_drag_end"),
        ],
        &[
            ("inventory_drag_initial_slot", "inventory_drag_pickup_sent"),
            ("inventory_drag_pickup_sent", "inventory_drag_start_sent"),
            (
                "inventory_drag_target_a_sent",
                "inventory_drag_target_b_sent",
            ),
            (
                "inventory_drag_end_sent",
                "inventory_drag_final_distribution_seen",
            ),
            (
                "server_inventory_drag_pickup",
                "server_inventory_drag_start",
            ),
            (
                "server_inventory_drag_target_b",
                "server_inventory_drag_end",
            ),
        ],
    );
    assert_typed_event_fixture_passes(
        Scenario::SurvivalBreakPlacePickup,
        Some(TEST_USERNAME),
        &[
            ("client", Some(TEST_USERNAME), "protocol_detected"),
            ("client", Some(TEST_USERNAME), "join_game"),
            ("client", Some(TEST_USERNAME), "render_tick"),
            ("client", Some(TEST_USERNAME), "survival_break_sent"),
            ("client", Some(TEST_USERNAME), "survival_break_update"),
            ("client", Some(TEST_USERNAME), "survival_pickup_seen"),
            ("client", Some(TEST_USERNAME), "survival_place_sent"),
            ("client", Some(TEST_USERNAME), "survival_place_update"),
            ("server", Some(TEST_USERNAME), "server_survival_join"),
            ("server", Some(TEST_USERNAME), "server_survival_break"),
            ("server", Some(TEST_USERNAME), "server_survival_pickup"),
            ("server", Some(TEST_USERNAME), "server_survival_place"),
        ],
        &[
            ("survival_break_sent", "survival_pickup_seen"),
            ("survival_pickup_seen", "survival_place_sent"),
            ("server_survival_break", "server_survival_place"),
        ],
    );
    assert_typed_event_fixture_passes(
        Scenario::SurvivalChestPersistence,
        Some(TEST_USERNAME),
        &[
            ("client", Some(TEST_USERNAME), "protocol_detected"),
            ("client", Some(TEST_USERNAME), "join_game"),
            ("client", Some(TEST_USERNAME), "render_tick"),
            ("client", Some(TEST_USERNAME), "survival_chest_open_seen"),
            ("client", Some(TEST_USERNAME), "survival_chest_store_sent"),
            ("client", Some(TEST_USERNAME), "survival_chest_close_sent"),
            (
                "client",
                Some(TEST_USERNAME),
                "survival_chest_reconnect_sent",
            ),
            ("client", Some(TEST_USERNAME), "survival_chest_reopen_seen"),
            (
                "client",
                Some(TEST_USERNAME),
                "survival_chest_persisted_seen",
            ),
            ("server", Some(TEST_USERNAME), "server_username_seen"),
            ("server", Some(TEST_USERNAME), "server_survival_chest_open"),
            ("server", Some(TEST_USERNAME), "server_survival_chest_store"),
            ("server", Some(TEST_USERNAME), "server_survival_chest_close"),
            (
                "server",
                Some(TEST_USERNAME),
                "server_survival_chest_reopen",
            ),
            (
                "server",
                Some(TEST_USERNAME),
                "server_survival_chest_persisted",
            ),
        ],
        &[
            ("survival_chest_open_seen", "survival_chest_store_sent"),
            ("survival_chest_close_sent", "survival_chest_reopen_seen"),
            (
                "server_survival_chest_reopen",
                "server_survival_chest_persisted",
            ),
        ],
    );
    assert_typed_event_fixture_passes(
        Scenario::SurvivalFurnacePersistence,
        Some(TEST_USERNAME),
        &[
            ("client", Some(TEST_USERNAME), "protocol_detected"),
            ("client", Some(TEST_USERNAME), "join_game"),
            ("client", Some(TEST_USERNAME), "render_tick"),
            ("client", Some(TEST_USERNAME), "survival_furnace_open_seen"),
            ("client", Some(TEST_USERNAME), "survival_furnace_input_sent"),
            ("client", Some(TEST_USERNAME), "survival_furnace_fuel_sent"),
            (
                "client",
                Some(TEST_USERNAME),
                "survival_furnace_burn_progress_seen",
            ),
            (
                "client",
                Some(TEST_USERNAME),
                "survival_furnace_output_seen",
            ),
            (
                "client",
                Some(TEST_USERNAME),
                "survival_furnace_output_collected",
            ),
            (
                "client",
                Some(TEST_USERNAME),
                "survival_furnace_inventory_updated",
            ),
            (
                "client",
                Some(TEST_USERNAME),
                "survival_furnace_reconnect_sent",
            ),
            (
                "client",
                Some(TEST_USERNAME),
                "survival_furnace_reopen_seen",
            ),
            ("server", Some(TEST_USERNAME), "server_username_seen"),
            (
                "server",
                Some(TEST_USERNAME),
                "server_survival_furnace_open",
            ),
            (
                "server",
                Some(TEST_USERNAME),
                "server_survival_furnace_input",
            ),
            (
                "server",
                Some(TEST_USERNAME),
                "server_survival_furnace_fuel",
            ),
            (
                "server",
                Some(TEST_USERNAME),
                "server_survival_furnace_burn_progress",
            ),
            (
                "server",
                Some(TEST_USERNAME),
                "server_survival_furnace_output_available",
            ),
            (
                "server",
                Some(TEST_USERNAME),
                "server_survival_furnace_output_collect",
            ),
            (
                "server",
                Some(TEST_USERNAME),
                "server_survival_furnace_reconnect_reopen",
            ),
            (
                "server",
                Some(TEST_USERNAME),
                "server_survival_furnace_state",
            ),
        ],
        &[
            ("survival_furnace_open_seen", "survival_furnace_input_sent"),
            (
                "survival_furnace_output_seen",
                "survival_furnace_output_collected",
            ),
            (
                "survival_furnace_output_collected",
                "survival_furnace_reconnect_sent",
            ),
            (
                "server_survival_furnace_output_collect",
                "server_survival_furnace_reconnect_reopen",
            ),
        ],
    );
    assert_typed_event_fixture_passes(
        Scenario::SurvivalFurnaceSmeltingBreadth,
        Some(TEST_USERNAME),
        &[
            ("client", Some(TEST_USERNAME), "protocol_detected"),
            ("client", Some(TEST_USERNAME), "join_game"),
            ("client", Some(TEST_USERNAME), "render_tick"),
            ("client", Some(TEST_USERNAME), "survival_furnace_open_seen"),
            ("client", Some(TEST_USERNAME), "survival_furnace_input_sent"),
            ("client", Some(TEST_USERNAME), "survival_furnace_fuel_sent"),
            (
                "client",
                Some(TEST_USERNAME),
                "survival_furnace_burn_progress_seen",
            ),
            (
                "client",
                Some(TEST_USERNAME),
                "survival_furnace_output_seen",
            ),
            (
                "client",
                Some(TEST_USERNAME),
                "survival_furnace_output_collected",
            ),
            (
                "client",
                Some(TEST_USERNAME),
                "survival_furnace_inventory_updated",
            ),
            (
                "client",
                Some(TEST_USERNAME),
                "survival_furnace_invalid_fuel_sent",
            ),
            ("server", Some(TEST_USERNAME), "server_username_seen"),
            (
                "server",
                Some(TEST_USERNAME),
                "server_survival_furnace_open",
            ),
            (
                "server",
                Some(TEST_USERNAME),
                "server_survival_furnace_input",
            ),
            (
                "server",
                Some(TEST_USERNAME),
                "server_survival_furnace_fuel",
            ),
            (
                "server",
                Some(TEST_USERNAME),
                "server_survival_furnace_burn_progress",
            ),
            (
                "server",
                Some(TEST_USERNAME),
                "server_survival_furnace_output_available",
            ),
            (
                "server",
                Some(TEST_USERNAME),
                "server_survival_furnace_output_collect",
            ),
            (
                "server",
                Some(TEST_USERNAME),
                "server_survival_furnace_invalid_fuel_rejected",
            ),
            (
                "server",
                Some(TEST_USERNAME),
                "server_survival_furnace_breadth_state",
            ),
        ],
        &[
            ("survival_furnace_open_seen", "survival_furnace_input_sent"),
            (
                "survival_furnace_output_seen",
                "survival_furnace_output_collected",
            ),
            (
                "survival_furnace_output_collected",
                "survival_furnace_invalid_fuel_sent",
            ),
            (
                "server_survival_furnace_invalid_fuel_rejected",
                "server_survival_furnace_breadth_state",
            ),
        ],
    );
    assert_typed_event_fixture_passes(
        Scenario::SurvivalMobAiLootBreadth,
        Some(TEST_USERNAME),
        &[
            ("client", Some(TEST_USERNAME), "protocol_detected"),
            ("client", Some(TEST_USERNAME), "join_game"),
            ("client", Some(TEST_USERNAME), "render_tick"),
            (
                "client",
                Some(TEST_USERNAME),
                "survival_mob_ai_loot_mob_seen",
            ),
            (
                "client",
                Some(TEST_USERNAME),
                "survival_mob_ai_loot_attack_sent",
            ),
            (
                "client",
                Some(TEST_USERNAME),
                "survival_mob_ai_loot_death_seen",
            ),
            (
                "client",
                Some(TEST_USERNAME),
                "survival_mob_ai_loot_drop_seen",
            ),
            (
                "client",
                Some(TEST_USERNAME),
                "survival_mob_ai_loot_pickup_seen",
            ),
            (
                "client",
                Some(TEST_USERNAME),
                "survival_mob_ai_loot_inventory_updated",
            ),
            ("server", Some(TEST_USERNAME), "server_username_seen"),
            (
                "server",
                Some(TEST_USERNAME),
                "server_survival_mob_ai_loot_spawn",
            ),
            (
                "server",
                Some(TEST_USERNAME),
                "server_survival_mob_ai_loot_ai_checkpoint",
            ),
            (
                "server",
                Some(TEST_USERNAME),
                "server_survival_mob_ai_loot_attack",
            ),
            (
                "server",
                Some(TEST_USERNAME),
                "server_survival_mob_ai_loot_death",
            ),
            (
                "server",
                Some(TEST_USERNAME),
                "server_survival_mob_ai_loot_drop_spawn",
            ),
            (
                "server",
                Some(TEST_USERNAME),
                "server_survival_mob_ai_loot_pickup",
            ),
            (
                "server",
                Some(TEST_USERNAME),
                "server_survival_mob_ai_loot_inventory",
            ),
            (
                "server",
                Some(TEST_USERNAME),
                "server_survival_mob_ai_loot_state",
            ),
        ],
        &[
            (
                "survival_mob_ai_loot_mob_seen",
                "survival_mob_ai_loot_attack_sent",
            ),
            (
                "survival_mob_ai_loot_drop_seen",
                "survival_mob_ai_loot_pickup_seen",
            ),
            (
                "server_survival_mob_ai_loot_spawn",
                "server_survival_mob_ai_loot_attack",
            ),
            (
                "server_survival_mob_ai_loot_inventory",
                "server_survival_mob_ai_loot_state",
            ),
        ],
    );
    assert_typed_event_fixture_passes(
        Scenario::SurvivalRedstoneCircuitBreadth,
        Some(TEST_USERNAME),
        &[
            ("client", Some(TEST_USERNAME), "protocol_detected"),
            ("client", Some(TEST_USERNAME), "join_game"),
            ("client", Some(TEST_USERNAME), "render_tick"),
            (
                "client",
                Some(TEST_USERNAME),
                "survival_redstone_circuit_initial_state",
            ),
            (
                "client",
                Some(TEST_USERNAME),
                "survival_redstone_circuit_input_sent",
            ),
            (
                "client",
                Some(TEST_USERNAME),
                "survival_redstone_circuit_output_update",
            ),
            (
                "client",
                Some(TEST_USERNAME),
                "survival_redstone_circuit_return_input_sent",
            ),
            (
                "client",
                Some(TEST_USERNAME),
                "survival_redstone_circuit_return_update",
            ),
            ("server", Some(TEST_USERNAME), "server_username_seen"),
            (
                "server",
                Some(TEST_USERNAME),
                "server_survival_redstone_circuit_initial",
            ),
            (
                "server",
                Some(TEST_USERNAME),
                "server_survival_redstone_circuit_input",
            ),
            (
                "server",
                Some(TEST_USERNAME),
                "server_survival_redstone_circuit_powered_on",
            ),
            (
                "server",
                Some(TEST_USERNAME),
                "server_survival_redstone_circuit_powered_off",
            ),
            (
                "server",
                Some(TEST_USERNAME),
                "server_survival_redstone_circuit_state",
            ),
        ],
        &[
            (
                "survival_redstone_circuit_initial_state",
                "survival_redstone_circuit_output_update",
            ),
            (
                "survival_redstone_circuit_output_update",
                "survival_redstone_circuit_return_update",
            ),
            (
                "server_survival_redstone_circuit_initial",
                "server_survival_redstone_circuit_powered_on",
            ),
            (
                "server_survival_redstone_circuit_powered_on",
                "server_survival_redstone_circuit_state",
            ),
        ],
    );
    assert_typed_event_fixture_passes(
        Scenario::SurvivalWorldMultichunkDurability,
        Some(TEST_USERNAME),
        &[
            ("client", Some(TEST_USERNAME), "protocol_detected"),
            ("client", Some(TEST_USERNAME), "join_game"),
            ("client", Some(TEST_USERNAME), "render_tick"),
            (
                "client",
                Some(TEST_USERNAME),
                "survival_world_multichunk_mutation_sent",
            ),
            (
                "client",
                Some(TEST_USERNAME),
                "survival_world_multichunk_pre_restart_update",
            ),
            (
                "client",
                Some(TEST_USERNAME),
                "survival_world_multichunk_reconnect_sent",
            ),
            (
                "client",
                Some(TEST_USERNAME),
                "survival_world_multichunk_post_restart_update",
            ),
            ("server", Some(TEST_USERNAME), "server_username_seen"),
            (
                "server",
                Some(TEST_USERNAME),
                "server_survival_world_multichunk_mutation",
            ),
            (
                "server",
                Some(TEST_USERNAME),
                "server_survival_world_multichunk_clean_shutdown",
            ),
            (
                "server",
                Some(TEST_USERNAME),
                "server_survival_world_multichunk_backend_restart",
            ),
            (
                "server",
                Some(TEST_USERNAME),
                "server_survival_world_multichunk_post_restart",
            ),
            (
                "server",
                Some(TEST_USERNAME),
                "server_survival_world_multichunk_state",
            ),
        ],
        &[
            (
                "survival_world_multichunk_mutation_sent",
                "survival_world_multichunk_reconnect_sent",
            ),
            (
                "survival_world_multichunk_reconnect_sent",
                "survival_world_multichunk_post_restart_update",
            ),
            (
                "server_survival_world_multichunk_mutation",
                "server_survival_world_multichunk_backend_restart",
            ),
            (
                "server_survival_world_multichunk_post_restart",
                "server_survival_world_multichunk_state",
            ),
        ],
    );
    assert_typed_event_fixture_passes(
        Scenario::SurvivalContainerBlockEntityBreadth,
        Some(TEST_USERNAME),
        &[
            ("client", Some(TEST_USERNAME), "protocol_detected"),
            ("client", Some(TEST_USERNAME), "join_game"),
            ("client", Some(TEST_USERNAME), "render_tick"),
            (
                "client",
                Some(TEST_USERNAME),
                "survival_container_block_entity_open_seen",
            ),
            (
                "client",
                Some(TEST_USERNAME),
                "survival_container_block_entity_transfer_sent",
            ),
            (
                "client",
                Some(TEST_USERNAME),
                "survival_container_block_entity_payload_seen",
            ),
            (
                "client",
                Some(TEST_USERNAME),
                "survival_container_block_entity_metadata_seen",
            ),
            (
                "client",
                Some(TEST_USERNAME),
                "survival_container_block_entity_reopen_seen",
            ),
            ("server", Some(TEST_USERNAME), "server_username_seen"),
            (
                "server",
                Some(TEST_USERNAME),
                "server_survival_container_block_entity_open",
            ),
            (
                "server",
                Some(TEST_USERNAME),
                "server_survival_container_block_entity_transfer",
            ),
            (
                "server",
                Some(TEST_USERNAME),
                "server_survival_container_block_entity_payload",
            ),
            (
                "server",
                Some(TEST_USERNAME),
                "server_survival_container_block_entity_metadata",
            ),
            (
                "server",
                Some(TEST_USERNAME),
                "server_survival_container_block_entity_state",
            ),
        ],
        &[
            (
                "survival_container_block_entity_open_seen",
                "survival_container_block_entity_transfer_sent",
            ),
            (
                "survival_container_block_entity_transfer_sent",
                "survival_container_block_entity_metadata_seen",
            ),
            (
                "server_survival_container_block_entity_open",
                "server_survival_container_block_entity_payload",
            ),
            (
                "server_survival_container_block_entity_metadata",
                "server_survival_container_block_entity_state",
            ),
        ],
    );
    assert_typed_event_fixture_passes(
        Scenario::SurvivalBiomeDimensionState,
        Some(TEST_USERNAME),
        &[
            ("client", Some(TEST_USERNAME), "protocol_detected"),
            ("client", Some(TEST_USERNAME), "join_game"),
            ("client", Some(TEST_USERNAME), "render_tick"),
            (
                "client",
                Some(TEST_USERNAME),
                "survival_biome_dimension_state",
            ),
            ("server", Some(TEST_USERNAME), "server_username_seen"),
            (
                "server",
                Some(TEST_USERNAME),
                "server_survival_biome_dimension_state",
            ),
        ],
        &[],
    );
    assert_typed_event_fixture_passes(
        Scenario::SurvivalBiomeDimensionTravel,
        Some(TEST_USERNAME),
        &[
            ("client", Some(TEST_USERNAME), "protocol_detected"),
            ("client", Some(TEST_USERNAME), "join_game"),
            ("client", Some(TEST_USERNAME), "render_tick"),
            (
                "client",
                Some(TEST_USERNAME),
                "survival_biome_dimension_travel_origin",
            ),
            (
                "client",
                Some(TEST_USERNAME),
                "survival_biome_dimension_travel_transition_sent",
            ),
            (
                "client",
                Some(TEST_USERNAME),
                "survival_biome_dimension_travel_destination_seen",
            ),
            ("server", Some(TEST_USERNAME), "server_username_seen"),
            (
                "server",
                Some(TEST_USERNAME),
                "server_survival_biome_dimension_travel_origin",
            ),
            (
                "server",
                Some(TEST_USERNAME),
                "server_survival_biome_dimension_travel_transition",
            ),
            (
                "server",
                Some(TEST_USERNAME),
                "server_survival_biome_dimension_travel_state",
            ),
        ],
        &[
            (
                "survival_biome_dimension_travel_origin",
                "survival_biome_dimension_travel_destination_seen",
            ),
            (
                "server_survival_biome_dimension_travel_origin",
                "server_survival_biome_dimension_travel_state",
            ),
        ],
    );
    assert_typed_event_fixture_passes(
        Scenario::SurvivalSignEditingLive,
        Some(TEST_USERNAME),
        &[
            ("client", Some(TEST_USERNAME), "protocol_detected"),
            ("client", Some(TEST_USERNAME), "join_game"),
            ("client", Some(TEST_USERNAME), "render_tick"),
            (
                "client",
                Some(TEST_USERNAME),
                "survival_sign_editing_open_seen",
            ),
            (
                "client",
                Some(TEST_USERNAME),
                "survival_sign_editing_update_sent",
            ),
            (
                "client",
                Some(TEST_USERNAME),
                "survival_sign_editing_post_update_seen",
            ),
            ("server", Some(TEST_USERNAME), "server_username_seen"),
            (
                "server",
                Some(TEST_USERNAME),
                "server_survival_sign_editing_open",
            ),
            (
                "server",
                Some(TEST_USERNAME),
                "server_survival_sign_editing_update_accepted",
            ),
            (
                "server",
                Some(TEST_USERNAME),
                "server_survival_sign_editing_state",
            ),
        ],
        &[
            (
                "survival_sign_editing_open_seen",
                "survival_sign_editing_update_sent",
            ),
            (
                "server_survival_sign_editing_open",
                "server_survival_sign_editing_state",
            ),
        ],
    );
    assert_typed_event_fixture_passes(
        Scenario::ReconnectFlagState,
        Some(TEST_USERNAME),
        &[
            ("client", Some(TEST_USERNAME), "protocol_detected"),
            ("client", Some(TEST_USERNAME), "join_game"),
            ("client", Some(TEST_USERNAME), "render_tick"),
            ("client", Some(TEST_USERNAME), "team_red"),
            ("client", Some(TEST_USERNAME), "flag_pickup"),
            ("client", Some(TEST_USERNAME), "reconnect_session"),
            ("server", Some(TEST_USERNAME), "server_flag_pickup"),
            (
                "server",
                Some(TEST_USERNAME),
                "server_flag_disconnect_return",
            ),
            (
                "server",
                Some(TEST_USERNAME),
                "server_reconnect_state_coherent",
            ),
        ],
        &[
            ("flag_pickup", "reconnect_session"),
            (
                "server_flag_disconnect_return",
                "server_reconnect_state_coherent",
            ),
        ],
    );
    assert_typed_event_fixture_passes(
        Scenario::CombatDamage,
        None,
        &[
            ("client", Some(TEST_ATTACKER_USERNAME), "protocol_detected"),
            ("client", Some(TEST_ATTACKER_USERNAME), "team_red"),
            ("client", Some(TEST_VICTIM_USERNAME), "team_blue"),
            (
                "client",
                Some(TEST_ATTACKER_USERNAME),
                "remote_player_spawn",
            ),
            ("client", Some(TEST_ATTACKER_USERNAME), "combat_attack_sent"),
            ("client", Some(TEST_VICTIM_USERNAME), "combat_health_update"),
            (
                "server",
                Some(TEST_ATTACKER_USERNAME),
                "server_client_a_seen",
            ),
            ("server", Some(TEST_VICTIM_USERNAME), "server_client_b_seen"),
            ("server", None, "server_combat_damage"),
        ],
        &[
            ("remote_player_spawn", "combat_attack_sent"),
            ("combat_attack_sent", "combat_health_update"),
            ("server_client_a_seen", "server_combat_damage"),
        ],
    );
    assert_typed_event_fixture_passes(
        Scenario::VanillaCombatReferenceParity,
        None,
        &[
            ("client", Some(TEST_ATTACKER_USERNAME), "protocol_detected"),
            (
                "client",
                Some(TEST_ATTACKER_USERNAME),
                "remote_player_spawn",
            ),
            ("client", Some(TEST_ATTACKER_USERNAME), "combat_attack_sent"),
            ("client", Some(TEST_VICTIM_USERNAME), "combat_health_update"),
            (
                "client",
                Some(TEST_VICTIM_USERNAME),
                "combat_velocity_update",
            ),
            (
                "server",
                Some(TEST_ATTACKER_USERNAME),
                "server_client_a_seen",
            ),
            ("server", Some(TEST_VICTIM_USERNAME), "server_client_b_seen"),
            ("server", None, "server_vanilla_combat_reference_damage"),
            ("server", None, "server_vanilla_combat_reference_knockback"),
        ],
        &[
            ("remote_player_spawn", "combat_attack_sent"),
            ("combat_attack_sent", "combat_health_update"),
            ("combat_health_update", "combat_velocity_update"),
            (
                "server_client_a_seen",
                "server_vanilla_combat_reference_damage",
            ),
            (
                "server_vanilla_combat_reference_damage",
                "server_vanilla_combat_reference_knockback",
            ),
        ],
    );
    assert_typed_event_fixture_passes(
        Scenario::ProjectileDamageAttribution,
        None,
        &[
            (
                "client",
                Some(TEST_ATTACKER_USERNAME),
                "attacker_client_projectile_use_sent",
            ),
            (
                "client",
                Some(TEST_ATTACKER_USERNAME),
                "attacker_client_projectile_swing_sent",
            ),
            ("server", None, "server_projectile_use_attacker_victim"),
            (
                "server",
                None,
                "server_projectile_hit_attacker_victim_health_delta",
            ),
            (
                "client",
                Some(TEST_VICTIM_USERNAME),
                "victim_client_damage_update",
            ),
        ],
        &[
            (
                "attacker_client_projectile_use_sent",
                "server_projectile_use_attacker_victim",
            ),
            (
                "server_projectile_use_attacker_victim",
                "server_projectile_hit_attacker_victim_health_delta",
            ),
            (
                "server_projectile_hit_attacker_victim_health_delta",
                "victim_client_damage_update",
            ),
        ],
    );
}

fn typed_event_oracle_evidence_for_scenario(scenario: Scenario) -> ClientRunEvidence {
    let client_observed = scenario_required_milestones(scenario)
        .iter()
        .map(|(name, _)| *name)
        .collect::<Vec<_>>();
    let server_observed = server_required_milestones(scenario)
        .iter()
        .map(|(name, _)| *name)
        .collect::<Vec<_>>();

    ClientRunEvidence {
        log_path: None,
        log_paths: Vec::new(),
        usernames: vec![TEST_USERNAME.to_string()],
        exit_code: Some(0),
        classification: "client-exited-success",
        matched_success_pattern: Some("Detected server protocol version".to_string()),
        scenario: Some(ScenarioEvidence {
            observed_milestones: client_observed,
            missing_milestones: Vec::new(),
            forbidden_matches: Vec::new(),
            passed: true,
        }),
        server_scenario: Some(ServerScenarioEvidence {
            observed_milestones: server_observed,
            missing_milestones: Vec::new(),
            forbidden_matches: Vec::new(),
            passed: true,
        }),
        projectile_damage_causality: None,
        projectile_travel_collision: None,
        mcp_control: None,
        frame_artifacts: None,
    }
}

fn restart_persistence_test_config(scenario: Scenario) -> Config {
    test_config(
        &[
            "--run",
            "--scenario",
            scenario_name(scenario),
            "--client-dir",
            TEST_CLIENT_DIR,
        ],
        &[],
    )
    .expect("restart persistence scenario config parses")
}

#[test]
fn restart_persistence_typed_event_oracle_accepts_selected_rows() {
    for scenario in [
        Scenario::SurvivalWorldPersistenceRestart,
        Scenario::SurvivalCrashRecoveryParity,
        Scenario::SurvivalBlockEntityPersistenceParity,
    ] {
        let cfg = restart_persistence_test_config(scenario);
        let passing = typed_event_oracle_evidence_for_scenario(scenario);
        validate_typed_event_oracle_for_migrated_scenario(&cfg, &passing)
            .expect("complete restart persistence typed-event graph passes");
    }
}

#[test]
fn restart_persistence_typed_event_oracle_fails_closed() {
    let scenario = Scenario::SurvivalWorldPersistenceRestart;
    let cfg = restart_persistence_test_config(scenario);
    let passing = typed_event_oracle_evidence_for_scenario(scenario);

    let mut missing_boundary = passing.clone();
    missing_boundary
        .server_scenario
        .as_mut()
        .expect("server evidence")
        .observed_milestones
        .retain(|name| *name != "server_survival_world_persistence_clean_shutdown");
    let err = validate_typed_event_oracle_for_migrated_scenario(&cfg, &missing_boundary)
        .expect_err("missing restart boundary fails");
    assert!(
        err.contains("missing:server_survival_world_persistence_clean_shutdown"),
        "{err}"
    );

    let mut missing_reconnect = passing.clone();
    missing_reconnect
        .scenario
        .as_mut()
        .expect("client evidence")
        .observed_milestones
        .retain(|name| *name != "survival_world_persistence_reconnect_sent");
    let err = validate_typed_event_oracle_for_migrated_scenario(&cfg, &missing_reconnect)
        .expect_err("missing restart reconnect fails");
    assert!(
        err.contains("missing:survival_world_persistence_reconnect_sent"),
        "{err}"
    );

    let mut unordered = passing.clone();
    unordered
        .scenario
        .as_mut()
        .expect("client evidence")
        .observed_milestones = vec![
        "protocol_detected",
        "join_game",
        "render_tick",
        "survival_world_persistence_mutation_sent",
        "survival_world_persistence_reconnect_sent",
        "survival_world_persistence_pre_restart_update",
        "survival_world_persistence_post_restart_update",
    ];
    let err = validate_typed_event_oracle_for_migrated_scenario(&cfg, &unordered)
        .expect_err("unordered restart milestones fail");
    assert!(
        err.contains("unordered:survival_world_persistence_pre_restart_update_before_survival_world_persistence_reconnect_sent"),
        "{err}"
    );

    let mut duplicate = passing.clone();
    duplicate
        .server_scenario
        .as_mut()
        .expect("server evidence")
        .observed_milestones
        .push("server_survival_world_persistence_state");
    let err = validate_typed_event_oracle_for_migrated_scenario(&cfg, &duplicate)
        .expect_err("duplicate restored state fails");
    assert!(
        err.contains("duplicate:server_survival_world_persistence_state"),
        "{err}"
    );

    let mut mismatched = passing;
    let server_observed = &mut mismatched
        .server_scenario
        .as_mut()
        .expect("server evidence")
        .observed_milestones;
    server_observed.retain(|name| *name != "server_survival_world_persistence_state");
    server_observed.push("server_survival_crash_recovery_state");
    let err = validate_typed_event_oracle_for_migrated_scenario(&cfg, &mismatched)
        .expect_err("mismatched restored state fails");
    assert!(
        err.contains("mismatched_restored_state:server_survival_crash_recovery_state"),
        "{err}"
    );
}

#[test]
fn survival_breadth_typed_event_fixtures_fail_closed() {
    assert_typed_event_fixture_rejects_missing_event(
        Scenario::SurvivalMobAiLootBreadth,
        Some(TEST_USERNAME),
        &[(
            "client",
            Some(TEST_USERNAME),
            "survival_mob_ai_loot_mob_seen",
        )],
        "survival_mob_ai_loot_attack_sent",
    );
    assert_typed_event_fixture_rejects_order(
        Scenario::SurvivalMobAiLootBreadth,
        Some(TEST_USERNAME),
        &[
            (
                "client",
                Some(TEST_USERNAME),
                "survival_mob_ai_loot_attack_sent",
            ),
            (
                "client",
                Some(TEST_USERNAME),
                "survival_mob_ai_loot_mob_seen",
            ),
        ],
        "survival_mob_ai_loot_mob_seen",
        "survival_mob_ai_loot_attack_sent",
    );
    assert_typed_event_fixture_rejects_missing_event(
        Scenario::SurvivalRedstoneCircuitBreadth,
        Some(TEST_USERNAME),
        &[(
            "client",
            Some(TEST_USERNAME),
            "survival_redstone_circuit_initial_state",
        )],
        "survival_redstone_circuit_input_sent",
    );
    assert_typed_event_fixture_rejects_order(
        Scenario::SurvivalRedstoneCircuitBreadth,
        Some(TEST_USERNAME),
        &[
            (
                "client",
                Some(TEST_USERNAME),
                "survival_redstone_circuit_output_update",
            ),
            (
                "client",
                Some(TEST_USERNAME),
                "survival_redstone_circuit_input_sent",
            ),
        ],
        "survival_redstone_circuit_input_sent",
        "survival_redstone_circuit_output_update",
    );
    assert_typed_event_fixture_rejects_missing_event(
        Scenario::SurvivalWorldMultichunkDurability,
        Some(TEST_USERNAME),
        &[(
            "client",
            Some(TEST_USERNAME),
            "survival_world_multichunk_mutation_sent",
        )],
        "survival_world_multichunk_pre_restart_update",
    );
    assert_typed_event_fixture_rejects_order(
        Scenario::SurvivalWorldMultichunkDurability,
        Some(TEST_USERNAME),
        &[
            (
                "client",
                Some(TEST_USERNAME),
                "survival_world_multichunk_reconnect_sent",
            ),
            (
                "client",
                Some(TEST_USERNAME),
                "survival_world_multichunk_pre_restart_update",
            ),
        ],
        "survival_world_multichunk_pre_restart_update",
        "survival_world_multichunk_reconnect_sent",
    );
    assert_typed_event_fixture_rejects_missing_event(
        Scenario::SurvivalContainerBlockEntityBreadth,
        Some(TEST_USERNAME),
        &[(
            "client",
            Some(TEST_USERNAME),
            "survival_container_block_entity_open_seen",
        )],
        "survival_container_block_entity_transfer_sent",
    );
    assert_typed_event_fixture_rejects_order(
        Scenario::SurvivalContainerBlockEntityBreadth,
        Some(TEST_USERNAME),
        &[
            (
                "client",
                Some(TEST_USERNAME),
                "survival_container_block_entity_payload_seen",
            ),
            (
                "client",
                Some(TEST_USERNAME),
                "survival_container_block_entity_transfer_sent",
            ),
        ],
        "survival_container_block_entity_transfer_sent",
        "survival_container_block_entity_payload_seen",
    );
    assert_typed_event_fixture_rejects_missing_event(
        Scenario::SurvivalBiomeDimensionState,
        Some(TEST_USERNAME),
        &[
            ("client", Some(TEST_USERNAME), "protocol_detected"),
            ("client", Some(TEST_USERNAME), "join_game"),
            ("client", Some(TEST_USERNAME), "render_tick"),
            (
                "client",
                Some(TEST_USERNAME),
                "survival_biome_dimension_state",
            ),
            ("server", Some(TEST_USERNAME), "server_username_seen"),
        ],
        "server_survival_biome_dimension_state",
    );
    assert_typed_event_fixture_rejects_missing_event(
        Scenario::SurvivalBiomeDimensionTravel,
        Some(TEST_USERNAME),
        &[(
            "client",
            Some(TEST_USERNAME),
            "survival_biome_dimension_travel_origin",
        )],
        "survival_biome_dimension_travel_transition_sent",
    );
    assert_typed_event_fixture_rejects_order(
        Scenario::SurvivalBiomeDimensionTravel,
        Some(TEST_USERNAME),
        &[
            (
                "client",
                Some(TEST_USERNAME),
                "survival_biome_dimension_travel_destination_seen",
            ),
            (
                "client",
                Some(TEST_USERNAME),
                "survival_biome_dimension_travel_transition_sent",
            ),
        ],
        "survival_biome_dimension_travel_transition_sent",
        "survival_biome_dimension_travel_destination_seen",
    );
    assert_typed_event_fixture_rejects_missing_event(
        Scenario::SurvivalSignEditingLive,
        Some(TEST_USERNAME),
        &[(
            "client",
            Some(TEST_USERNAME),
            "survival_sign_editing_open_seen",
        )],
        "survival_sign_editing_update_sent",
    );
    assert_typed_event_fixture_rejects_order(
        Scenario::SurvivalSignEditingLive,
        Some(TEST_USERNAME),
        &[
            (
                "client",
                Some(TEST_USERNAME),
                "survival_sign_editing_post_update_seen",
            ),
            (
                "client",
                Some(TEST_USERNAME),
                "survival_sign_editing_update_sent",
            ),
        ],
        "survival_sign_editing_update_sent",
        "survival_sign_editing_post_update_seen",
    );
}

#[test]
fn typed_event_oracle_validates_biome_dimension_join_state_graph() {
    let scenario = Scenario::SurvivalBiomeDimensionState;
    let cfg = test_config(
        &[
            "--run",
            "--scenario",
            "survival-biome-dimension-state",
            "--client-dir",
            "/tmp/stevenarella",
        ],
        &[],
    )
    .expect("biome/dimension join-state config parses");
    let passing = typed_event_oracle_evidence_for_scenario(scenario);
    validate_typed_event_oracle_for_migrated_scenario(&cfg, &passing)
        .expect("complete biome/dimension join-state graph passes");

    let mut missing_server = passing.clone();
    missing_server
        .server_scenario
        .as_mut()
        .expect("server scenario exists")
        .observed_milestones
        .retain(|name| *name != "server_survival_biome_dimension_state");
    let err = validate_typed_event_oracle_for_migrated_scenario(&cfg, &missing_server)
        .expect_err("missing typed join-state server event fails");
    assert!(
        err.contains("server_survival_biome_dimension_state"),
        "{err}"
    );
}

#[test]
fn typed_event_oracle_validates_remaining_survival_breadth_graphs() {
    for (scenario, missing_client, misordered_before, misordered_after) in [
        (
            Scenario::SurvivalMobAiLootBreadth,
            "survival_mob_ai_loot_attack_sent",
            "server_survival_mob_ai_loot_ai_checkpoint",
            "server_survival_mob_ai_loot_attack",
        ),
        (
            Scenario::SurvivalRedstoneCircuitBreadth,
            "survival_redstone_circuit_input_sent",
            "server_survival_redstone_circuit_input",
            "server_survival_redstone_circuit_powered_on",
        ),
        (
            Scenario::SurvivalWorldMultichunkDurability,
            "survival_world_multichunk_pre_restart_update",
            "server_survival_world_multichunk_clean_shutdown",
            "server_survival_world_multichunk_backend_restart",
        ),
        (
            Scenario::SurvivalContainerBlockEntityBreadth,
            "survival_container_block_entity_transfer_sent",
            "server_survival_container_block_entity_transfer",
            "server_survival_container_block_entity_payload",
        ),
        (
            Scenario::SurvivalBiomeDimensionTravel,
            "survival_biome_dimension_travel_transition_sent",
            "server_survival_biome_dimension_travel_origin",
            "server_survival_biome_dimension_travel_transition",
        ),
        (
            Scenario::SurvivalSignEditingLive,
            "survival_sign_editing_update_sent",
            "server_survival_sign_editing_open",
            "server_survival_sign_editing_update_accepted",
        ),
    ] {
        let cfg = test_config(
            &[
                "--scenario",
                scenario_name(scenario),
                "--receipt",
                "/tmp/survival-breadth.receipt.json",
            ],
            &[],
        )
        .expect("remaining survival breadth config parses");
        let passing = typed_event_oracle_evidence_for_scenario(scenario);
        validate_typed_event_oracle_for_migrated_scenario(&cfg, &passing)
            .expect("complete remaining survival breadth graph passes");

        let mut missing_client_evidence = passing.clone();
        missing_client_evidence
            .scenario
            .as_mut()
            .expect("client evidence")
            .observed_milestones
            .retain(|name| *name != missing_client);
        let err = validate_typed_event_oracle_for_migrated_scenario(&cfg, &missing_client_evidence)
            .expect_err("missing typed remaining survival breadth client event fails");
        assert!(err.contains(missing_client), "{err}");

        let mut misordered_server_evidence = passing.clone();
        misordered_server_evidence
            .server_scenario
            .as_mut()
            .expect("server evidence")
            .observed_milestones
            .retain(|name| *name != misordered_before && *name != misordered_after);
        misordered_server_evidence
            .server_scenario
            .as_mut()
            .expect("server evidence")
            .observed_milestones
            .extend([misordered_after, misordered_before]);
        let err =
            validate_typed_event_oracle_for_migrated_scenario(&cfg, &misordered_server_evidence)
                .expect_err("misordered typed remaining survival breadth server phases fail");
        let expected_violation = format!("{misordered_before}_before_{misordered_after}");
        assert!(err.contains(&expected_violation), "{err}");
    }
}

#[test]
fn typed_event_oracle_validates_ctf_rule_family_graphs() {
    for (scenario, missing_client, misordered_before, misordered_after, reorder_server) in [
        (
            Scenario::FlagScoreRepeat,
            "flag_capture",
            "flag_pickup",
            "flag_capture",
            false,
        ),
        (
            Scenario::BlueFlagScore,
            "flag_capture",
            "flag_pickup",
            "flag_capture",
            false,
        ),
        (
            Scenario::CombatDamage,
            "combat_attack_sent",
            "server_client_a_seen",
            "server_combat_damage",
            true,
        ),
        (
            Scenario::CombatKnockback,
            "combat_velocity_update",
            "server_combat_damage",
            "server_combat_knockback",
            true,
        ),
        (
            Scenario::ArmorEquipmentMitigation,
            "armor_inventory_slot",
            "server_combat_damage",
            "server_armor_mitigation",
            true,
        ),
        (
            Scenario::EquipmentUpdateObservation,
            "entity_equipment_update",
            "remote_player_spawn",
            "entity_equipment_update",
            false,
        ),
        (
            Scenario::ProjectileHit,
            "projectile_travel_observed",
            "server_projectile_travel_sample",
            "server_projectile_collision",
            true,
        ),
        (
            Scenario::ProjectileDamageAttribution,
            "projectile_damage_update",
            "server_projectile_use",
            "server_projectile_hit",
            true,
        ),
        (
            Scenario::FlagCarrierDeathReturn,
            "respawn_health_restored",
            "server_flag_carrier_death",
            "server_flag_return",
            true,
        ),
        (
            Scenario::ReconnectFlagState,
            "reconnect_session",
            "server_flag_disconnect_return",
            "server_reconnect_state_coherent",
            true,
        ),
        (
            Scenario::CtfInvalidPickupOwnership,
            "ctf_invalid_pickup_contained",
            "ctf_invalid_pickup_attempted",
            "ctf_invalid_pickup_contained",
            false,
        ),
        (
            Scenario::CtfInvalidReturnDrop,
            "ctf_invalid_return_drop_contained",
            "ctf_invalid_return_drop_attempted",
            "ctf_invalid_return_drop_contained",
            false,
        ),
        (
            Scenario::CtfInvalidOpponentBaseReturnDrop,
            "ctf_invalid_opponent_base_return_drop_contained",
            "ctf_invalid_opponent_base_return_drop_attempted",
            "ctf_invalid_opponent_base_return_drop_contained",
            false,
        ),
        (
            Scenario::CtfScoreLimitWinCondition,
            "ctf_score_limit_win_seen",
            "server_score_limit_final_capture",
            "server_score_limit_win_condition",
            true,
        ),
        (
            Scenario::CtfSimultaneousPickupCaptureRace,
            "score_red_1",
            "server_ctf_race_rejected_transition",
            "server_ctf_race_final_state",
            true,
        ),
        (
            Scenario::CtfSpawnTeamBalanceReset,
            "score_red_1",
            "server_ctf_spawn_team_balance",
            "server_ctf_spawn_resource_reset",
            true,
        ),
        (
            Scenario::ReconnectFlagScore,
            "reconnect_session",
            "flag_capture",
            "score_red_1",
            false,
        ),
        (
            Scenario::MultiClientLoadScore,
            "score_red_1",
            "server_client_b_seen",
            "server_flag_or_score",
            true,
        ),
    ] {
        let cfg = test_config(
            &[
                "--scenario",
                scenario_name(scenario),
                "--receipt",
                "/tmp/ctf-rule.receipt.json",
            ],
            &[],
        )
        .expect("CTF rule config parses");
        let passing = typed_event_oracle_evidence_for_scenario(scenario);
        validate_typed_event_oracle_for_migrated_scenario(&cfg, &passing)
            .expect("complete CTF typed-event graph passes");

        let mut missing_client_evidence = passing.clone();
        missing_client_evidence
            .scenario
            .as_mut()
            .expect("client evidence")
            .observed_milestones
            .retain(|name| *name != missing_client);
        let err = validate_typed_event_oracle_for_migrated_scenario(&cfg, &missing_client_evidence)
            .expect_err("missing CTF typed event fails");
        assert!(err.contains(missing_client), "{err}");

        let mut misordered_evidence = passing.clone();
        let observed = if reorder_server {
            &mut misordered_evidence
                .server_scenario
                .as_mut()
                .expect("server evidence")
                .observed_milestones
        } else {
            &mut misordered_evidence
                .scenario
                .as_mut()
                .expect("client evidence")
                .observed_milestones
        };
        observed.retain(|name| *name != misordered_before && *name != misordered_after);
        observed.extend([misordered_after, misordered_before]);
        let err = validate_typed_event_oracle_for_migrated_scenario(&cfg, &misordered_evidence)
            .expect_err("misordered CTF typed-event phases fail");
        let expected_violation = format!("{misordered_before}_before_{misordered_after}");
        assert!(err.contains(&expected_violation), "{err}");
    }
}

#[test]
fn ctf_rule_typed_event_fixtures_fail_closed_on_wrong_actor() {
    let events = typed_event_fixture_from_steps(
        Scenario::CtfInvalidPickupOwnership,
        &[
            ("client", Some("otherbot"), "ctf_invalid_pickup_attempted"),
            ("client", Some("otherbot"), "ctf_invalid_pickup_contained"),
        ],
    );
    let result = evaluate_typed_event_graph(
        &events,
        scenario_name(Scenario::CtfInvalidPickupOwnership),
        TEST_SESSION_ID,
        Some(TEST_USERNAME),
        &[
            "ctf_invalid_pickup_attempted",
            "ctf_invalid_pickup_contained",
        ],
        &[],
        &[(
            "ctf_invalid_pickup_attempted",
            "ctf_invalid_pickup_contained",
        )],
    );
    assert!(!result.passed, "{result:?}");
    assert!(
        result
            .missing_events
            .contains(&"ctf_invalid_pickup_attempted".to_string()),
        "{result:?}"
    );
}

#[test]
fn typed_events_from_receipt_evidence_include_client_and_server_sources() {
    let cfg = test_config(
        &[
            "--scenario",
            "inventory-interaction",
            "--receipt",
            "/tmp/inventory.receipt.json",
        ],
        &[],
    )
    .expect("inventory config parses");
    let client = ClientRunEvidence {
        log_path: None,
        log_paths: Vec::new(),
        usernames: vec![TEST_USERNAME.to_string()],
        exit_code: Some(0),
        classification: "client-exited-success",
        matched_success_pattern: Some("Detected server protocol version".to_string()),
        scenario: Some(ScenarioEvidence {
            observed_milestones: vec!["protocol_detected", "inventory_drop_sent"],
            missing_milestones: Vec::new(),
            forbidden_matches: Vec::new(),
            passed: true,
        }),
        server_scenario: Some(ServerScenarioEvidence {
            observed_milestones: vec!["server_inventory_drop"],
            missing_milestones: Vec::new(),
            forbidden_matches: Vec::new(),
            passed: true,
        }),
        projectile_damage_causality: None,
        projectile_travel_collision: None,
        mcp_control: None,
        frame_artifacts: None,
    };

    let events =
        typed_events_from_receipt_evidence(&cfg, &client).expect("typed event evidence builds");
    let timeline = normalize_typed_event_timeline(&events);

    let expected_event_count = client
        .scenario
        .as_ref()
        .map(|scenario| scenario.observed_milestones.len())
        .unwrap_or_default()
        + client
            .server_scenario
            .as_ref()
            .map(|server| server.observed_milestones.len())
            .unwrap_or_default();
    assert_eq!(events.len(), expected_event_count);
    assert!(events.iter().any(|event| event.source == "client"));
    assert!(events.iter().any(|event| event.source == "server"));
    assert!(events
        .iter()
        .all(|event| event.username.as_deref() == Some(TEST_USERNAME)));
    assert!(timeline.contains("session=inventory.receipt"), "{timeline}");
    assert!(timeline.contains("event=inventory_drop_sent"), "{timeline}");
    assert!(
        timeline.contains("event=server_inventory_drop"),
        "{timeline}"
    );
}

#[test]
fn typed_event_oracle_validates_migrated_mcp_controlled_smoke_graph() {
    let cfg = test_config(
        &[
            "--scenario",
            MCP_CONTROLLED_SMOKE_SCENARIO,
            "--receipt",
            "/tmp/mcp-controlled-smoke.receipt.json",
        ],
        &[],
    )
    .expect("mcp-controlled smoke config parses");
    let mut passing = mcp_controlled_dry_run_evidence(&cfg);
    if let Some(control) = passing.mcp_control.as_mut() {
        control.calls_attempted = MCP_CONTROL_LIVE_CALLS.to_vec();
        control.calls_succeeded = MCP_CONTROL_LIVE_CALLS.to_vec();
        control.command_outcome_ids = MCP_CONTROL_LIVE_OUTCOME_IDS.to_vec();
    }
    passing.frame_artifacts = Some(FrameArtifactsReceiptEvidence {
        selected: true,
        capture_requested: true,
        artifact_count: 1,
        artifacts: vec![FrameArtifactReceiptItem {
            path: "docs/evidence/mcp-controlled-smoke-frames/latest-frame.png".to_string(),
            relative_path: MCP_CONTROL_LIVE_CAPTURE_RELATIVE_PATH.to_string(),
            format: "png".to_string(),
            width_px: 1280,
            height_px: 720,
            frame_id: 1,
            sequence_id: 1,
            byte_len: 16,
            blake3: "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef".to_string(),
            redaction: "not_reviewed".to_string(),
            includes_ui: true,
        }],
        missing_digests: Vec::new(),
        path_containment_checked: true,
        promotion_ready: true,
        non_claims: FRAME_ARTIFACT_NON_CLAIMS.to_vec(),
    });
    validate_typed_event_oracle_for_migrated_scenario(&cfg, &passing)
        .expect("complete typed MCP graph passes");

    let mut missing_artifact_identity = passing.clone();
    missing_artifact_identity.frame_artifacts = Some(FrameArtifactsReceiptEvidence {
        selected: true,
        capture_requested: true,
        artifact_count: 1,
        artifacts: Vec::new(),
        missing_digests: vec!["frame_blake3"],
        path_containment_checked: true,
        promotion_ready: false,
        non_claims: FRAME_ARTIFACT_NON_CLAIMS.to_vec(),
    });
    let err = validate_typed_event_oracle_for_migrated_scenario(&cfg, &missing_artifact_identity)
        .expect_err("missing MCP frame artifact identity fails");
    assert!(err.contains("mcp_frame_artifact_identity"), "{err}");

    let misordered = typed_event_fixture_from_steps(
        Scenario::McpControlledSmoke,
        &[
            ("client", Some(TEST_USERNAME), "mcp_initialize"),
            ("client", Some(TEST_USERNAME), "mcp_tools_list"),
            ("mcp", Some(TEST_USERNAME), "mcp_capture_latest_frame"),
            ("client", Some(TEST_USERNAME), "mcp_status_call"),
            ("mcp", Some(TEST_USERNAME), "mcp_look_call"),
            ("mcp", Some(TEST_USERNAME), "mcp_input_call"),
            ("mcp", Some(TEST_USERNAME), "mcp_frame_artifact_identity"),
            ("client", Some(TEST_USERNAME), "mcp_command_outcomes"),
        ],
    );
    let required = typed_event_required_events_for_graph(Scenario::McpControlledSmoke);
    let required_refs = required.iter().map(String::as_str).collect::<Vec<_>>();
    let result = evaluate_typed_event_graph(
        &misordered,
        MCP_CONTROLLED_SMOKE_SCENARIO,
        TEST_SESSION_ID,
        Some(TEST_USERNAME),
        &required_refs,
        &[],
        &typed_event_ordered_edges_for_scenario(Scenario::McpControlledSmoke),
    );
    assert!(!result.passed, "{result:?}");
    assert!(
        result
            .order_violations
            .contains(&"mcp_input_call_before_mcp_capture_latest_frame".to_string()),
        "{result:?}"
    );
}

#[test]
fn typed_event_oracle_validates_migrated_inventory_graph() {
    let cfg = test_config(
        &[
            "--scenario",
            "inventory-interaction",
            "--receipt",
            "/tmp/inventory.receipt.json",
        ],
        &[],
    )
    .expect("inventory config parses");
    let client_observed = scenario_required_milestones(Scenario::InventoryInteraction)
        .iter()
        .map(|(name, _)| *name)
        .collect::<Vec<_>>();
    let server_observed = server_required_milestones(Scenario::InventoryInteraction)
        .iter()
        .map(|(name, _)| *name)
        .collect::<Vec<_>>();
    let passing = ClientRunEvidence {
        log_path: None,
        log_paths: Vec::new(),
        usernames: vec![TEST_USERNAME.to_string()],
        exit_code: Some(0),
        classification: "client-exited-success",
        matched_success_pattern: Some("Detected server protocol version".to_string()),
        scenario: Some(ScenarioEvidence {
            observed_milestones: client_observed,
            missing_milestones: Vec::new(),
            forbidden_matches: Vec::new(),
            passed: true,
        }),
        server_scenario: Some(ServerScenarioEvidence {
            observed_milestones: server_observed.clone(),
            missing_milestones: Vec::new(),
            forbidden_matches: Vec::new(),
            passed: true,
        }),
        projectile_damage_causality: None,
        projectile_travel_collision: None,
        mcp_control: None,
        frame_artifacts: None,
    };
    validate_typed_event_oracle_for_migrated_scenario(&cfg, &passing)
        .expect("complete typed inventory graph passes");

    let mut missing_server = passing.clone();
    missing_server
        .server_scenario
        .as_mut()
        .expect("server evidence")
        .observed_milestones
        .retain(|name| *name != "server_block_place");
    let err = validate_typed_event_oracle_for_migrated_scenario(&cfg, &missing_server)
        .expect_err("missing typed server event fails");
    assert!(err.contains("server_block_place"), "{err}");
}

#[test]
fn typed_event_oracle_validates_migrated_inventory_drag_graph() {
    let cfg = test_config(
        &[
            "--scenario",
            "inventory-drag-transactions",
            "--receipt",
            "/tmp/inventory-drag.receipt.json",
        ],
        &[],
    )
    .expect("inventory drag config parses");
    let client_observed = scenario_required_milestones(Scenario::InventoryDragTransactions)
        .iter()
        .map(|(name, _)| *name)
        .collect::<Vec<_>>();
    let server_observed = server_required_milestones(Scenario::InventoryDragTransactions)
        .iter()
        .map(|(name, _)| *name)
        .collect::<Vec<_>>();
    let passing = ClientRunEvidence {
        log_path: None,
        log_paths: Vec::new(),
        usernames: vec![TEST_USERNAME.to_string()],
        exit_code: Some(0),
        classification: "client-exited-success",
        matched_success_pattern: Some("Detected server protocol version".to_string()),
        scenario: Some(ScenarioEvidence {
            observed_milestones: client_observed,
            missing_milestones: Vec::new(),
            forbidden_matches: Vec::new(),
            passed: true,
        }),
        server_scenario: Some(ServerScenarioEvidence {
            observed_milestones: server_observed,
            missing_milestones: Vec::new(),
            forbidden_matches: Vec::new(),
            passed: true,
        }),
        projectile_damage_causality: None,
        projectile_travel_collision: None,
        mcp_control: None,
        frame_artifacts: None,
    };
    validate_typed_event_oracle_for_migrated_scenario(&cfg, &passing)
        .expect("complete typed inventory drag graph passes");

    let mut missing_client_drag = passing.clone();
    missing_client_drag
        .scenario
        .as_mut()
        .expect("client evidence")
        .observed_milestones
        .retain(|name| *name != "inventory_drag_target_b_sent");
    let err = validate_typed_event_oracle_for_migrated_scenario(&cfg, &missing_client_drag)
        .expect_err("missing typed drag client event fails");
    assert!(err.contains("inventory_drag_target_b_sent"), "{err}");

    let mut misordered_server_drag = passing;
    misordered_server_drag
        .server_scenario
        .as_mut()
        .expect("server evidence")
        .observed_milestones = vec![
        "server_username_seen",
        "server_inventory_drag_pickup",
        "server_inventory_drag_start",
        "server_inventory_drag_target_a",
        "server_inventory_drag_end",
        "server_inventory_drag_target_b",
    ];
    let err = validate_typed_event_oracle_for_migrated_scenario(&cfg, &misordered_server_drag)
        .expect_err("misordered typed drag server phases fail");
    assert!(
        err.contains("server_inventory_drag_target_b_before_server_inventory_drag_end"),
        "{err}"
    );
}

#[test]
fn typed_event_oracle_validates_migrated_survival_break_place_graph() {
    let cfg = test_config(
        &[
            "--scenario",
            "survival-break-place-pickup",
            "--receipt",
            "/tmp/survival-break-place.receipt.json",
        ],
        &[],
    )
    .expect("survival break/place config parses");
    let client_observed = scenario_required_milestones(Scenario::SurvivalBreakPlacePickup)
        .iter()
        .map(|(name, _)| *name)
        .collect::<Vec<_>>();
    let server_observed = server_required_milestones(Scenario::SurvivalBreakPlacePickup)
        .iter()
        .map(|(name, _)| *name)
        .collect::<Vec<_>>();
    let passing = ClientRunEvidence {
        log_path: None,
        log_paths: Vec::new(),
        usernames: vec![TEST_USERNAME.to_string()],
        exit_code: Some(0),
        classification: "client-exited-success",
        matched_success_pattern: Some("Detected server protocol version".to_string()),
        scenario: Some(ScenarioEvidence {
            observed_milestones: client_observed,
            missing_milestones: Vec::new(),
            forbidden_matches: Vec::new(),
            passed: true,
        }),
        server_scenario: Some(ServerScenarioEvidence {
            observed_milestones: server_observed,
            missing_milestones: Vec::new(),
            forbidden_matches: Vec::new(),
            passed: true,
        }),
        projectile_damage_causality: None,
        projectile_travel_collision: None,
        mcp_control: None,
        frame_artifacts: None,
    };
    validate_typed_event_oracle_for_migrated_scenario(&cfg, &passing)
        .expect("complete typed survival break/place graph passes");

    let mut missing_client_survival = passing.clone();
    missing_client_survival
        .scenario
        .as_mut()
        .expect("client evidence")
        .observed_milestones
        .retain(|name| *name != "survival_place_update");
    let err = validate_typed_event_oracle_for_migrated_scenario(&cfg, &missing_client_survival)
        .expect_err("missing typed survival client event fails");
    assert!(err.contains("survival_place_update"), "{err}");

    let mut misordered_server_survival = passing;
    misordered_server_survival
        .server_scenario
        .as_mut()
        .expect("server evidence")
        .observed_milestones = vec![
        "server_username_seen",
        "server_survival_join",
        "server_survival_break",
        "server_survival_place",
        "server_survival_pickup",
    ];
    let err = validate_typed_event_oracle_for_migrated_scenario(&cfg, &misordered_server_survival)
        .expect_err("misordered typed survival server phases fail");
    assert!(
        err.contains("server_survival_pickup_before_server_survival_place"),
        "{err}"
    );
}

#[test]
fn typed_event_oracle_validates_migrated_survival_chest_persistence_graph() {
    let cfg = test_config(
        &[
            "--scenario",
            "survival-chest-persistence",
            "--receipt",
            "/tmp/survival-chest-persistence.receipt.json",
        ],
        &[],
    )
    .expect("survival chest persistence config parses");
    let client_observed = scenario_required_milestones(Scenario::SurvivalChestPersistence)
        .iter()
        .map(|(name, _)| *name)
        .collect::<Vec<_>>();
    let server_observed = server_required_milestones(Scenario::SurvivalChestPersistence)
        .iter()
        .map(|(name, _)| *name)
        .collect::<Vec<_>>();
    let passing = ClientRunEvidence {
        log_path: None,
        log_paths: Vec::new(),
        usernames: vec![TEST_USERNAME.to_string()],
        exit_code: Some(0),
        classification: "client-exited-success",
        matched_success_pattern: Some("Detected server protocol version".to_string()),
        scenario: Some(ScenarioEvidence {
            observed_milestones: client_observed,
            missing_milestones: Vec::new(),
            forbidden_matches: Vec::new(),
            passed: true,
        }),
        server_scenario: Some(ServerScenarioEvidence {
            observed_milestones: server_observed,
            missing_milestones: Vec::new(),
            forbidden_matches: Vec::new(),
            passed: true,
        }),
        projectile_damage_causality: None,
        projectile_travel_collision: None,
        mcp_control: None,
        frame_artifacts: None,
    };
    validate_typed_event_oracle_for_migrated_scenario(&cfg, &passing)
        .expect("complete typed survival chest graph passes");

    let mut missing_client_persisted = passing.clone();
    missing_client_persisted
        .scenario
        .as_mut()
        .expect("client evidence")
        .observed_milestones
        .retain(|name| *name != "survival_chest_persisted_seen");
    let err = validate_typed_event_oracle_for_migrated_scenario(&cfg, &missing_client_persisted)
        .expect_err("missing typed chest persisted client event fails");
    assert!(err.contains("survival_chest_persisted_seen"), "{err}");

    let mut misordered_client_reopen = passing;
    misordered_client_reopen
        .scenario
        .as_mut()
        .expect("client evidence")
        .observed_milestones = vec![
        "protocol_detected",
        "join_game",
        "render_tick",
        "survival_chest_open_seen",
        "survival_chest_store_sent",
        "survival_chest_reopen_seen",
        "survival_chest_close_sent",
        "survival_chest_reconnect_sent",
        "survival_chest_persisted_seen",
    ];
    let err = validate_typed_event_oracle_for_migrated_scenario(&cfg, &misordered_client_reopen)
        .expect_err("misordered typed chest reopen before close fails");
    assert!(
        err.contains("survival_chest_close_sent_before_survival_chest_reconnect_sent")
            || err.contains("survival_chest_reconnect_sent_before_survival_chest_reopen_seen"),
        "{err}"
    );
}

#[test]
fn typed_event_oracle_validates_migrated_survival_furnace_persistence_graph() {
    let cfg = test_config(
        &[
            "--scenario",
            "survival-furnace-persistence",
            "--receipt",
            "/tmp/survival-furnace-persistence.receipt.json",
        ],
        &[],
    )
    .expect("survival furnace persistence config parses");
    let client_observed = scenario_required_milestones(Scenario::SurvivalFurnacePersistence)
        .iter()
        .map(|(name, _)| *name)
        .collect::<Vec<_>>();
    let server_observed = server_required_milestones(Scenario::SurvivalFurnacePersistence)
        .iter()
        .map(|(name, _)| *name)
        .collect::<Vec<_>>();
    let passing = ClientRunEvidence {
        log_path: None,
        log_paths: Vec::new(),
        usernames: vec![TEST_USERNAME.to_string()],
        exit_code: Some(0),
        classification: "client-exited-success",
        matched_success_pattern: Some("Detected server protocol version".to_string()),
        scenario: Some(ScenarioEvidence {
            observed_milestones: client_observed,
            missing_milestones: Vec::new(),
            forbidden_matches: Vec::new(),
            passed: true,
        }),
        server_scenario: Some(ServerScenarioEvidence {
            observed_milestones: server_observed,
            missing_milestones: Vec::new(),
            forbidden_matches: Vec::new(),
            passed: true,
        }),
        projectile_damage_causality: None,
        projectile_travel_collision: None,
        mcp_control: None,
        frame_artifacts: None,
    };
    validate_typed_event_oracle_for_migrated_scenario(&cfg, &passing)
        .expect("complete typed survival furnace graph passes");

    let mut missing_client_output = passing.clone();
    missing_client_output
        .scenario
        .as_mut()
        .expect("client evidence")
        .observed_milestones
        .retain(|name| *name != "survival_furnace_output_seen");
    let err = validate_typed_event_oracle_for_migrated_scenario(&cfg, &missing_client_output)
        .expect_err("missing typed furnace output client event fails");
    assert!(err.contains("survival_furnace_output_seen"), "{err}");

    let mut misordered_client_reconnect = passing;
    misordered_client_reconnect
        .scenario
        .as_mut()
        .expect("client evidence")
        .observed_milestones = vec![
        "protocol_detected",
        "join_game",
        "render_tick",
        "survival_furnace_open_seen",
        "survival_furnace_input_sent",
        "survival_furnace_fuel_sent",
        "survival_furnace_burn_progress_seen",
        "survival_furnace_output_seen",
        "survival_furnace_reconnect_sent",
        "survival_furnace_output_collected",
        "survival_furnace_inventory_updated",
        "survival_furnace_reopen_seen",
    ];
    let err = validate_typed_event_oracle_for_migrated_scenario(&cfg, &misordered_client_reconnect)
        .expect_err("misordered typed furnace reconnect before collection fails");
    assert!(
        err.contains("survival_furnace_output_collected_before_survival_furnace_reconnect_sent"),
        "{err}"
    );
}

#[test]
fn typed_event_oracle_validates_migrated_survival_furnace_smelting_breadth_graph() {
    let cfg = test_config(
        &[
            "--scenario",
            "survival-furnace-smelting-breadth",
            "--receipt",
            "/tmp/survival-furnace-smelting-breadth.receipt.json",
        ],
        &[],
    )
    .expect("survival furnace smelting breadth config parses");
    let client_observed = scenario_required_milestones(Scenario::SurvivalFurnaceSmeltingBreadth)
        .iter()
        .map(|(name, _)| *name)
        .collect::<Vec<_>>();
    let server_observed = server_required_milestones(Scenario::SurvivalFurnaceSmeltingBreadth)
        .iter()
        .map(|(name, _)| *name)
        .collect::<Vec<_>>();
    let passing = ClientRunEvidence {
        log_path: None,
        log_paths: Vec::new(),
        usernames: vec![TEST_USERNAME.to_string()],
        exit_code: Some(0),
        classification: "client-exited-success",
        matched_success_pattern: Some("Detected server protocol version".to_string()),
        scenario: Some(ScenarioEvidence {
            observed_milestones: client_observed,
            missing_milestones: Vec::new(),
            forbidden_matches: Vec::new(),
            passed: true,
        }),
        server_scenario: Some(ServerScenarioEvidence {
            observed_milestones: server_observed,
            missing_milestones: Vec::new(),
            forbidden_matches: Vec::new(),
            passed: true,
        }),
        projectile_damage_causality: None,
        projectile_travel_collision: None,
        mcp_control: None,
        frame_artifacts: None,
    };
    validate_typed_event_oracle_for_migrated_scenario(&cfg, &passing)
        .expect("complete typed survival furnace smelting graph passes");

    let mut missing_server_invalid = passing.clone();
    missing_server_invalid
        .server_scenario
        .as_mut()
        .expect("server evidence")
        .observed_milestones
        .retain(|name| *name != "server_survival_furnace_invalid_fuel_rejected");
    let err = validate_typed_event_oracle_for_migrated_scenario(&cfg, &missing_server_invalid)
        .expect_err("missing typed furnace invalid-fuel server event fails");
    assert!(
        err.contains("server_survival_furnace_invalid_fuel_rejected"),
        "{err}"
    );

    let mut misordered_client_collect = passing;
    misordered_client_collect
        .scenario
        .as_mut()
        .expect("client evidence")
        .observed_milestones = vec![
        "protocol_detected",
        "join_game",
        "render_tick",
        "survival_furnace_open_seen",
        "survival_furnace_input_sent",
        "survival_furnace_fuel_sent",
        "survival_furnace_burn_progress_seen",
        "survival_furnace_output_collected",
        "survival_furnace_output_seen",
        "survival_furnace_inventory_updated",
        "survival_furnace_invalid_fuel_sent",
    ];
    let err = validate_typed_event_oracle_for_migrated_scenario(&cfg, &misordered_client_collect)
        .expect_err("misordered typed furnace collection before output fails");
    assert!(
        err.contains("survival_furnace_output_seen_before_survival_furnace_output_collected"),
        "{err}"
    );
}

#[test]
fn typed_event_oracle_validates_migrated_survival_crafting_recipe_breadth_graph() {
    let cfg = test_config(
        &[
            "--scenario",
            "survival-crafting-recipe-breadth",
            "--receipt",
            "/tmp/survival-crafting-recipe-breadth.receipt.json",
        ],
        &[],
    )
    .expect("survival crafting recipe breadth config parses");
    let client_observed = scenario_required_milestones(Scenario::SurvivalCraftingRecipeBreadth)
        .iter()
        .map(|(name, _)| *name)
        .collect::<Vec<_>>();
    let server_observed = server_required_milestones(Scenario::SurvivalCraftingRecipeBreadth)
        .iter()
        .map(|(name, _)| *name)
        .collect::<Vec<_>>();
    let passing = ClientRunEvidence {
        log_path: None,
        log_paths: Vec::new(),
        usernames: vec![TEST_USERNAME.to_string()],
        exit_code: Some(0),
        classification: "client-exited-success",
        matched_success_pattern: Some("Detected server protocol version".to_string()),
        scenario: Some(ScenarioEvidence {
            observed_milestones: client_observed,
            missing_milestones: Vec::new(),
            forbidden_matches: Vec::new(),
            passed: true,
        }),
        server_scenario: Some(ServerScenarioEvidence {
            observed_milestones: server_observed,
            missing_milestones: Vec::new(),
            forbidden_matches: Vec::new(),
            passed: true,
        }),
        projectile_damage_causality: None,
        projectile_travel_collision: None,
        mcp_control: None,
        frame_artifacts: None,
    };
    validate_typed_event_oracle_for_migrated_scenario(&cfg, &passing)
        .expect("complete typed survival crafting recipe breadth graph passes");

    let mut missing_server_invalid = passing.clone();
    missing_server_invalid
        .server_scenario
        .as_mut()
        .expect("server evidence")
        .observed_milestones
        .retain(|name| *name != "server_survival_crafting_breadth_invalid_rejected");
    let err = validate_typed_event_oracle_for_migrated_scenario(&cfg, &missing_server_invalid)
        .expect_err("missing typed crafting breadth invalid rejection fails");
    assert!(
        err.contains("server_survival_crafting_breadth_invalid_rejected"),
        "{err}"
    );

    let mut misordered_client_crafting = passing;
    misordered_client_crafting
        .scenario
        .as_mut()
        .expect("client evidence")
        .observed_milestones = vec![
        "protocol_detected",
        "join_game",
        "render_tick",
        "survival_crafting_breadth_shaped_seen",
        "survival_crafting_breadth_grid_clear_seen",
        "survival_crafting_breadth_shapeless_seen",
        "survival_crafting_breadth_invalid_seen",
        "survival_crafting_breadth_inventory_updated",
    ];
    let err = validate_typed_event_oracle_for_migrated_scenario(&cfg, &misordered_client_crafting)
        .expect_err("misordered typed crafting breadth phases fail");
    assert!(
        err.contains(
            "survival_crafting_breadth_shapeless_seen_before_survival_crafting_breadth_grid_clear_seen"
        ),
        "{err}"
    );
}

#[test]
fn typed_event_oracle_validates_migrated_survival_crafting_table_graph() {
    let cfg = test_config(
        &[
            "--scenario",
            "survival-crafting-table",
            "--receipt",
            "/tmp/survival-crafting-table.receipt.json",
        ],
        &[],
    )
    .expect("survival crafting-table config parses");
    let client_observed = scenario_required_milestones(Scenario::SurvivalCraftingTable)
        .iter()
        .map(|(name, _)| *name)
        .collect::<Vec<_>>();
    let server_observed = server_required_milestones(Scenario::SurvivalCraftingTable)
        .iter()
        .map(|(name, _)| *name)
        .collect::<Vec<_>>();
    let passing = ClientRunEvidence {
        log_path: None,
        log_paths: Vec::new(),
        usernames: vec![TEST_USERNAME.to_string()],
        exit_code: Some(0),
        classification: "client-exited-success",
        matched_success_pattern: Some("Detected server protocol version".to_string()),
        scenario: Some(ScenarioEvidence {
            observed_milestones: client_observed,
            missing_milestones: Vec::new(),
            forbidden_matches: Vec::new(),
            passed: true,
        }),
        server_scenario: Some(ServerScenarioEvidence {
            observed_milestones: server_observed,
            missing_milestones: Vec::new(),
            forbidden_matches: Vec::new(),
            passed: true,
        }),
        projectile_damage_causality: None,
        projectile_travel_collision: None,
        mcp_control: None,
        frame_artifacts: None,
    };
    validate_typed_event_oracle_for_migrated_scenario(&cfg, &passing)
        .expect("complete typed survival crafting-table graph passes");

    let mut missing_client_crafting = passing.clone();
    missing_client_crafting
        .scenario
        .as_mut()
        .expect("client evidence")
        .observed_milestones
        .retain(|name| *name != "survival_crafting_result_collected");
    let err = validate_typed_event_oracle_for_migrated_scenario(&cfg, &missing_client_crafting)
        .expect_err("missing typed crafting client event fails");
    assert!(err.contains("survival_crafting_result_collected"), "{err}");

    let mut misordered_server_crafting = passing;
    misordered_server_crafting
        .server_scenario
        .as_mut()
        .expect("server evidence")
        .observed_milestones = vec![
        "server_username_seen",
        "server_survival_crafting_table_open",
        "server_survival_crafting_input_a",
        "server_survival_crafting_input_b",
        "server_survival_crafting_collect",
        "server_survival_crafting_result",
    ];
    let err = validate_typed_event_oracle_for_migrated_scenario(&cfg, &misordered_server_crafting)
        .expect_err("misordered typed crafting server phases fail");
    assert!(
        err.contains("server_survival_crafting_result_before_server_survival_crafting_collect"),
        "{err}"
    );
}

#[test]
fn typed_event_oracle_validates_migrated_survival_hunger_food_graph() {
    let scenario = Scenario::SurvivalHungerFood;
    let cfg = test_config(
        &[
            "--scenario",
            scenario_name(scenario),
            "--receipt",
            "/tmp/survival-hunger-food.receipt.json",
        ],
        &[],
    )
    .expect("survival hunger food config parses");
    let passing = typed_event_oracle_evidence_for_scenario(scenario);
    validate_typed_event_oracle_for_migrated_scenario(&cfg, &passing)
        .expect("complete typed survival hunger food graph passes");

    let mut missing_server_state = passing.clone();
    missing_server_state
        .server_scenario
        .as_mut()
        .expect("server evidence")
        .observed_milestones
        .retain(|name| *name != "server_survival_hunger_food_state");
    let err = validate_typed_event_oracle_for_migrated_scenario(&cfg, &missing_server_state)
        .expect_err("missing typed hunger food final state fails");
    assert!(err.contains("server_survival_hunger_food_state"), "{err}");

    let mut misordered_client_inventory = passing;
    misordered_client_inventory
        .scenario
        .as_mut()
        .expect("client evidence")
        .observed_milestones = vec![
        "protocol_detected",
        "join_game",
        "render_tick",
        "survival_hunger_food_item_seen",
        "survival_hunger_food_pre_seen",
        "survival_hunger_food_use_sent",
        "survival_hunger_food_inventory_updated",
        "survival_hunger_food_post_seen",
    ];
    let err = validate_typed_event_oracle_for_migrated_scenario(&cfg, &misordered_client_inventory)
        .expect_err("misordered typed hunger food inventory before post-state fails");
    assert!(
        err.contains(
            "survival_hunger_food_post_seen_before_survival_hunger_food_inventory_updated"
        ),
        "{err}"
    );
}

#[test]
fn typed_event_oracle_validates_migrated_survival_hunger_health_cycle_graph() {
    let cfg = test_config(
        &[
            "--scenario",
            "survival-hunger-health-cycle",
            "--receipt",
            "/tmp/survival-hunger-health-cycle.receipt.json",
        ],
        &[],
    )
    .expect("survival hunger health-cycle config parses");
    let client_observed = scenario_required_milestones(Scenario::SurvivalHungerHealthCycle)
        .iter()
        .map(|(name, _)| *name)
        .collect::<Vec<_>>();
    let server_observed = server_required_milestones(Scenario::SurvivalHungerHealthCycle)
        .iter()
        .map(|(name, _)| *name)
        .collect::<Vec<_>>();
    let passing = ClientRunEvidence {
        log_path: None,
        log_paths: Vec::new(),
        usernames: vec![TEST_USERNAME.to_string()],
        exit_code: Some(0),
        classification: "client-exited-success",
        matched_success_pattern: Some("Detected server protocol version".to_string()),
        scenario: Some(ScenarioEvidence {
            observed_milestones: client_observed,
            missing_milestones: Vec::new(),
            forbidden_matches: Vec::new(),
            passed: true,
        }),
        server_scenario: Some(ServerScenarioEvidence {
            observed_milestones: server_observed,
            missing_milestones: Vec::new(),
            forbidden_matches: Vec::new(),
            passed: true,
        }),
        projectile_damage_causality: None,
        projectile_travel_collision: None,
        mcp_control: None,
        frame_artifacts: None,
    };
    validate_typed_event_oracle_for_migrated_scenario(&cfg, &passing)
        .expect("complete typed survival hunger health-cycle graph passes");

    let mut missing_server_state = passing.clone();
    missing_server_state
        .server_scenario
        .as_mut()
        .expect("server evidence")
        .observed_milestones
        .retain(|name| *name != "server_survival_hunger_health_state");
    let err = validate_typed_event_oracle_for_migrated_scenario(&cfg, &missing_server_state)
        .expect_err("missing typed hunger health final state fails");
    assert!(err.contains("server_survival_hunger_health_state"), "{err}");

    let mut misordered_client_inventory = passing;
    misordered_client_inventory
        .scenario
        .as_mut()
        .expect("client evidence")
        .observed_milestones = vec![
        "protocol_detected",
        "join_game",
        "render_tick",
        "survival_hunger_health_item_seen",
        "survival_hunger_health_pre_seen",
        "survival_hunger_health_consume_sent",
        "survival_hunger_health_inventory_updated",
        "survival_hunger_health_recovery_seen",
    ];
    let err = validate_typed_event_oracle_for_migrated_scenario(&cfg, &misordered_client_inventory)
        .expect_err("misordered typed hunger inventory before recovery fails");
    assert!(
        err.contains(
            "survival_hunger_health_recovery_seen_before_survival_hunger_health_inventory_updated"
        ),
        "{err}"
    );
}

#[test]
fn typed_event_oracle_validates_migrated_survival_mob_drop_graph() {
    let scenario = Scenario::SurvivalMobDrop;
    let cfg = test_config(
        &[
            "--scenario",
            scenario_name(scenario),
            "--receipt",
            "/tmp/survival-mob-drop.receipt.json",
        ],
        &[],
    )
    .expect("survival mob drop config parses");
    let passing = typed_event_oracle_evidence_for_scenario(scenario);
    validate_typed_event_oracle_for_migrated_scenario(&cfg, &passing)
        .expect("complete typed survival mob drop graph passes");

    let mut missing_server_state = passing.clone();
    missing_server_state
        .server_scenario
        .as_mut()
        .expect("server evidence")
        .observed_milestones
        .retain(|name| *name != "server_survival_mob_drop_state");
    let err = validate_typed_event_oracle_for_migrated_scenario(&cfg, &missing_server_state)
        .expect_err("missing typed mob drop final state fails");
    assert!(err.contains("server_survival_mob_drop_state"), "{err}");

    let mut misordered_client_pickup = passing;
    misordered_client_pickup
        .scenario
        .as_mut()
        .expect("client evidence")
        .observed_milestones = vec![
        "protocol_detected",
        "join_game",
        "render_tick",
        "survival_mob_drop_mob_seen",
        "survival_mob_drop_attack_sent",
        "survival_mob_drop_death_seen",
        "survival_mob_drop_pickup_seen",
        "survival_mob_drop_drop_seen",
        "survival_mob_drop_inventory_updated",
    ];
    let err = validate_typed_event_oracle_for_migrated_scenario(&cfg, &misordered_client_pickup)
        .expect_err("misordered typed mob drop pickup before drop fails");
    assert!(
        err.contains("survival_mob_drop_drop_seen_before_survival_mob_drop_pickup_seen"),
        "{err}"
    );
}

#[test]
fn typed_event_oracle_validates_migrated_survival_redstone_toggle_graph() {
    let scenario = Scenario::SurvivalRedstoneToggle;
    let cfg = test_config(
        &[
            "--scenario",
            scenario_name(scenario),
            "--receipt",
            "/tmp/survival-redstone-toggle.receipt.json",
        ],
        &[],
    )
    .expect("survival redstone toggle config parses");
    let passing = typed_event_oracle_evidence_for_scenario(scenario);
    validate_typed_event_oracle_for_migrated_scenario(&cfg, &passing)
        .expect("complete typed survival redstone toggle graph passes");

    let mut missing_server_state = passing.clone();
    missing_server_state
        .server_scenario
        .as_mut()
        .expect("server evidence")
        .observed_milestones
        .retain(|name| *name != "server_survival_redstone_toggle_state");
    let err = validate_typed_event_oracle_for_migrated_scenario(&cfg, &missing_server_state)
        .expect_err("missing typed redstone toggle final state fails");
    assert!(
        err.contains("server_survival_redstone_toggle_state"),
        "{err}"
    );

    let mut misordered_client_return = passing;
    misordered_client_return
        .scenario
        .as_mut()
        .expect("client evidence")
        .observed_milestones = vec![
        "protocol_detected",
        "join_game",
        "render_tick",
        "survival_redstone_toggle_input_sent",
        "survival_redstone_toggle_output_update",
        "survival_redstone_toggle_return_update",
        "survival_redstone_toggle_return_input_sent",
    ];
    let err = validate_typed_event_oracle_for_migrated_scenario(&cfg, &misordered_client_return)
        .expect_err("misordered typed redstone return update before input fails");
    assert!(
        err.contains(
            "survival_redstone_toggle_return_input_sent_before_survival_redstone_toggle_return_update"
        ),
        "{err}"
    );
}

#[test]
fn ctf_simultaneous_race_tracks_one_accept_one_reject() {
    let client = evaluate_scenario(
        Scenario::CtfSimultaneousPickupCaptureRace,
        "mc_compat_ctf_race_client_count=2\nDetected server protocol version 763\njoin_game\nrender_tick_with_player\nYou are on team RED!\nYou have the flag!\nYou captured the flag!\nRED: 1\n",
    );
    assert!(client.passed, "{client:?}");
    assert!(client.forbidden_matches.is_empty(), "{client:?}");

    let double_accept = evaluate_scenario(
        Scenario::CtfSimultaneousPickupCaptureRace,
        "mc_compat_ctf_race_client_count=2\nDetected server protocol version 763\njoin_game\nrender_tick_with_player\nYou are on team RED!\nYou have the flag!\nYou captured the flag!\nRED: 1\nctf_race_double_accept\n",
    );
    assert!(!double_accept.passed, "{double_accept:?}");
    assert!(
        double_accept
            .forbidden_matches
            .contains(&"ctf_race_double_accept"),
        "{double_accept:?}"
    );

    let server = evaluate_server_scenario(
        Scenario::CtfSimultaneousPickupCaptureRace,
        "compatbota joined\ncompatbotb joined\nMC-COMPAT-MILESTONE ctf_race_accepted_transition username=compatbotb player_team=Red flag_team=Blue transition=pickup race_window_ticks=40\nMC-COMPAT-MILESTONE ctf_race_rejected_transition username=compatbota player_team=Red flag_team=Blue transition=duplicate_pickup reason=flag_already_held race_window_ticks=40\nMC-COMPAT-MILESTONE ctf_race_final_state capture_username=compatbotb accepted_username=compatbotb rejected_username=compatbota capture_team=Red carried_flag=Blue final_blue_flag_state=at_base red_score=1 blue_score=0 race_window_ticks=40 accepted_transition=pickup rejected_transition=duplicate_pickup\n",
        "compatbot",
    );
    assert!(server.passed, "{server:?}");
    assert!(server.forbidden_matches.is_empty(), "{server:?}");

    let missing_reject = evaluate_server_scenario(
        Scenario::CtfSimultaneousPickupCaptureRace,
        "compatbota joined\ncompatbotb joined\nMC-COMPAT-MILESTONE ctf_race_accepted_transition username=compatbotb player_team=Red flag_team=Blue transition=pickup race_window_ticks=40\nMC-COMPAT-MILESTONE ctf_race_final_state capture_username=compatbotb accepted_username=compatbotb rejected_username=compatbota capture_team=Red carried_flag=Blue final_blue_flag_state=at_base red_score=1 blue_score=0 race_window_ticks=40 accepted_transition=pickup rejected_transition=duplicate_pickup\n",
        "compatbot",
    );
    assert!(!missing_reject.passed, "{missing_reject:?}");
    assert!(
        missing_reject
            .missing_milestones
            .contains(&"server_ctf_race_rejected_transition"),
        "{missing_reject:?}"
    );
}

#[test]
fn ctf_spawn_team_balance_reset_tracks_client_server_and_guards() {
    let client = evaluate_scenario(
        Scenario::CtfSpawnTeamBalanceReset,
        "mc_compat_ctf_spawn_team_reset_client_count=2\nDetected server protocol version 763\njoin_game\nrender_tick_with_player\nYou are on team RED!\nYou are on team BLUE!\nYou have the flag!\nYou captured the flag!\nRED: 1\n",
    );
    assert!(client.passed, "{client:?}");
    assert!(client.forbidden_matches.is_empty(), "{client:?}");

    let stale_resource = evaluate_scenario(
        Scenario::CtfSpawnTeamBalanceReset,
        "mc_compat_ctf_spawn_team_reset_client_count=2\nDetected server protocol version 763\njoin_game\nrender_tick_with_player\nYou are on team RED!\nYou are on team BLUE!\nYou have the flag!\nYou captured the flag!\nRED: 1\nctf_spawn_resource_stale_after_reset\n",
    );
    assert!(!stale_resource.passed, "{stale_resource:?}");
    assert!(
        stale_resource
            .forbidden_matches
            .contains(&"spawn_resource_stale"),
        "{stale_resource:?}"
    );

    let server = evaluate_server_scenario(
        Scenario::CtfSpawnTeamBalanceReset,
        "compatbota joined\ncompatbotb joined\nMC-COMPAT-MILESTONE ctf_spawn_team_assignment username=compatbota team=Red red_count=1 blue_count=0 spawn_x=-40.0 spawn_y=65.0 spawn_z=0.0 slot36=WoodenSword:1 slot37=RedWool:64 correlation_id=team-select-compatbota\nMC-COMPAT-MILESTONE ctf_spawn_team_assignment username=compatbotb team=Blue red_count=1 blue_count=1 spawn_x=40.0 spawn_y=65.0 spawn_z=0.0 slot36=WoodenSword:1 slot37=BlueWool:64 correlation_id=team-select-compatbotb\nMC-COMPAT-MILESTONE ctf_spawn_team_balance red_count=1 blue_count=1 selected_teams=compatbota:Red,compatbotb:Blue outcome=balanced\nMC-COMPAT-MILESTONE ctf_spawn_resource_reset_state trigger=score capture_username=compatbota capture_team=Red carried_flag=Blue red_count=1 blue_count=1 red_score=1 blue_score=0 red_spawn=-40.0,65.0,0.0 blue_spawn=40.0,65.0,0.0 slot36=WoodenSword:1 slot37=TeamWool:64 reset_state=scoreboard_flags_and_resources_coherent correlation_id=score-reset-compatbota\n",
        "compatbot",
    );
    assert!(server.passed, "{server:?}");
    assert!(server.forbidden_matches.is_empty(), "{server:?}");

    let missing_reset = evaluate_server_scenario(
        Scenario::CtfSpawnTeamBalanceReset,
        "compatbota joined\ncompatbotb joined\nMC-COMPAT-MILESTONE ctf_spawn_team_assignment username=compatbota team=Red red_count=1 blue_count=0 spawn_x=-40.0 spawn_y=65.0 spawn_z=0.0 slot36=WoodenSword:1 slot37=RedWool:64 correlation_id=team-select-compatbota\nMC-COMPAT-MILESTONE ctf_spawn_team_assignment username=compatbotb team=Blue red_count=1 blue_count=1 spawn_x=40.0 spawn_y=65.0 spawn_z=0.0 slot36=WoodenSword:1 slot37=BlueWool:64 correlation_id=team-select-compatbotb\nMC-COMPAT-MILESTONE ctf_spawn_team_balance red_count=1 blue_count=1 selected_teams=compatbota:Red,compatbotb:Blue outcome=balanced\n",
        "compatbot",
    );
    assert!(!missing_reset.passed, "{missing_reset:?}");
    assert!(
        missing_reset
            .missing_milestones
            .contains(&"server_ctf_spawn_resource_reset"),
        "{missing_reset:?}"
    );
}

#[test]
fn ctf_invalid_pickup_ownership_tracks_client_server_and_envelope() {
    let client = evaluate_scenario(
        Scenario::CtfInvalidPickupOwnership,
        "Detected server protocol version 763\njoin_game\nrender_tick_with_player\nctf_invalid_pickup_attempted player_team=red flag_team=red pre_owner=none action=own_flag_pickup expected=no_owner_transfer_no_score\nctf_invalid_pickup_contained player_team=red flag_team=red post_owner=none red_score=0 blue_score=0 outcome=no_owner_transfer_no_score\n",
    );
    assert!(client.passed, "{client:?}");
    assert!(client.forbidden_matches.is_empty(), "{client:?}");

    let invalid_transfer = evaluate_scenario(
        Scenario::CtfInvalidPickupOwnership,
        "Detected server protocol version 763\njoin_game\nrender_tick_with_player\nctf_invalid_pickup_attempted\nctf_invalid_pickup_contained\nYou have the flag!\n",
    );
    assert!(!invalid_transfer.passed, "{invalid_transfer:?}");
    assert!(
        invalid_transfer
            .forbidden_matches
            .contains(&"unexpected_flag_pickup_chat"),
        "{invalid_transfer:?}"
    );

    let server = evaluate_server_scenario(
        Scenario::CtfInvalidPickupOwnership,
        "compatbot joined\nMC-COMPAT-MILESTONE invalid_flag_pickup_rejected username=compatbot player_team=Red flag_team=Red pre_owner=none post_owner=none red_score=0 blue_score=0 outcome=no_owner_transfer_no_score\n",
        "compatbot",
    );
    assert!(server.passed, "{server:?}");
    assert!(server.forbidden_matches.is_empty(), "{server:?}");

    let server_transfer = evaluate_server_scenario(
        Scenario::CtfInvalidPickupOwnership,
        "compatbot joined\nMC-COMPAT-MILESTONE invalid_flag_pickup_rejected username=compatbot player_team=Red flag_team=Red pre_owner=none post_owner=none red_score=0 blue_score=0 outcome=no_owner_transfer_no_score\nMC-COMPAT-MILESTONE flag_pickup username=compatbot carrier_team=Red flag_team=Red\n",
        "compatbot",
    );
    assert!(!server_transfer.passed, "{server_transfer:?}");
    assert!(
        server_transfer
            .forbidden_matches
            .contains(&"unexpected_server_flag_pickup"),
        "{server_transfer:?}"
    );

    let cfg = test_config(
        &["--dry-run", "--scenario", "ctf-invalid-pickup-ownership"],
        &[],
    )
    .expect("invalid pickup rail config parses");
    let evidence = evaluate_negative_live_rail_safety(&cfg);
    assert!(evidence.selected, "{evidence:?}");
    assert_eq!(evidence.rail, Some("ctf-invalid-pickup-ownership"));
    assert_eq!(
        evidence.invalid_action,
        Some("own_flag_pickup_without_ownership_transfer")
    );
    assert_eq!(
        evidence.postcondition_milestone,
        Some("ctf_invalid_pickup_contained")
    );
    assert!(evidence.preflight_passed, "{evidence:?}");
}

#[test]
fn ctf_invalid_return_drop_tracks_client_server_and_envelope() {
    let client = evaluate_scenario(
        Scenario::CtfInvalidReturnDrop,
        "Detected server protocol version 763\njoin_game\nrender_tick_with_player\nctf_invalid_return_drop_attempted player_team=red flag_team=red pre_state=at_base action=own_base_return expected=no_flag_state_mutation_no_score\nctf_invalid_return_drop_contained player_team=red flag_team=red post_state=at_base red_score=0 blue_score=0 outcome=no_flag_state_mutation_no_score\n",
    );
    assert!(client.passed, "{client:?}");
    assert!(client.forbidden_matches.is_empty(), "{client:?}");

    let invalid_return = evaluate_scenario(
        Scenario::CtfInvalidReturnDrop,
        "Detected server protocol version 763\njoin_game\nrender_tick_with_player\nctf_invalid_return_drop_attempted\nctf_invalid_return_drop_contained\nMC-COMPAT-MILESTONE flag_return carrier=compatbot flag_team=red\n",
    );
    assert!(!invalid_return.passed, "{invalid_return:?}");
    assert!(
        invalid_return
            .forbidden_matches
            .contains(&"unexpected_flag_return"),
        "{invalid_return:?}"
    );

    let server = evaluate_server_scenario(
        Scenario::CtfInvalidReturnDrop,
        "compatbot joined\nMC-COMPAT-MILESTONE invalid_flag_return_drop_rejected username=compatbot actor_team=Red flag_team=Red pre_state=at_base post_state=at_base red_score=0 blue_score=0 outcome=no_flag_state_mutation_no_score\n",
        "compatbot",
    );
    assert!(server.passed, "{server:?}");
    assert!(server.forbidden_matches.is_empty(), "{server:?}");

    let server_return = evaluate_server_scenario(
        Scenario::CtfInvalidReturnDrop,
        "compatbot joined\nMC-COMPAT-MILESTONE invalid_flag_return_drop_rejected username=compatbot actor_team=Red flag_team=Red pre_state=at_base post_state=at_base red_score=0 blue_score=0 outcome=no_flag_state_mutation_no_score\nMC-COMPAT-MILESTONE flag_return carrier=compatbot flag_team=red\n",
        "compatbot",
    );
    assert!(!server_return.passed, "{server_return:?}");
    assert!(
        server_return
            .forbidden_matches
            .contains(&"unexpected_flag_return"),
        "{server_return:?}"
    );

    let cfg = test_config(&["--dry-run", "--scenario", "ctf-invalid-return-drop"], &[])
        .expect("invalid return/drop rail config parses");
    let evidence = evaluate_negative_live_rail_safety(&cfg);
    assert!(evidence.selected, "{evidence:?}");
    assert_eq!(evidence.rail, Some("ctf-invalid-return-drop"));
    assert_eq!(
        evidence.invalid_action,
        Some("own_base_return_without_carrier")
    );
    assert_eq!(
        evidence.postcondition_milestone,
        Some("ctf_invalid_return_drop_contained")
    );
    assert!(evidence.preflight_passed, "{evidence:?}");
}

#[test]
fn ctf_invalid_opponent_base_return_drop_tracks_client_server_and_envelope() {
    let client = evaluate_scenario(
        Scenario::CtfInvalidOpponentBaseReturnDrop,
        "Detected server protocol version 763\njoin_game\nrender_tick_with_player\nctf_invalid_opponent_base_return_drop_attempted actor_team=red flag_team=blue pre_state=at_base base=opponent_base action=opponent_base_return_drop_without_carrier expected=no_flag_state_mutation_no_score\nctf_invalid_opponent_base_return_drop_contained actor_team=red flag_team=blue post_state=at_base red_score=0 blue_score=0 outcome=no_flag_state_mutation_no_score\n",
    );
    assert!(client.passed, "{client:?}");
    assert!(client.forbidden_matches.is_empty(), "{client:?}");

    let pickup_mutation = evaluate_scenario(
        Scenario::CtfInvalidOpponentBaseReturnDrop,
        "Detected server protocol version 763\njoin_game\nrender_tick_with_player\nctf_invalid_opponent_base_return_drop_attempted\nctf_invalid_opponent_base_return_drop_contained\nYou have the flag!\n",
    );
    assert!(!pickup_mutation.passed, "{pickup_mutation:?}");
    assert!(
        pickup_mutation
            .forbidden_matches
            .contains(&"unexpected_flag_pickup_chat"),
        "{pickup_mutation:?}"
    );

    let server = evaluate_server_scenario(
        Scenario::CtfInvalidOpponentBaseReturnDrop,
        "compatbot joined\nMC-COMPAT-MILESTONE invalid_opponent_base_return_drop_rejected username=compatbot actor_team=Red flag_team=Blue pre_state=at_base post_state=at_base red_score=0 blue_score=0 outcome=no_flag_state_mutation_no_score\n",
        "compatbot",
    );
    assert!(server.passed, "{server:?}");
    assert!(server.forbidden_matches.is_empty(), "{server:?}");

    let server_pickup = evaluate_server_scenario(
        Scenario::CtfInvalidOpponentBaseReturnDrop,
        "compatbot joined\nMC-COMPAT-MILESTONE invalid_opponent_base_return_drop_rejected username=compatbot actor_team=Red flag_team=Blue pre_state=at_base post_state=at_base red_score=0 blue_score=0 outcome=no_flag_state_mutation_no_score\nMC-COMPAT-MILESTONE flag_pickup username=compatbot carrier_team=Red flag_team=Blue\n",
        "compatbot",
    );
    assert!(!server_pickup.passed, "{server_pickup:?}");
    assert!(
        server_pickup
            .forbidden_matches
            .contains(&"unexpected_server_flag_pickup"),
        "{server_pickup:?}"
    );

    let cfg = test_config(
        &[
            "--dry-run",
            "--scenario",
            "ctf-invalid-opponent-base-return-drop",
        ],
        &[],
    )
    .expect("invalid opponent-base return/drop rail config parses");
    let evidence = evaluate_negative_live_rail_safety(&cfg);
    assert!(evidence.selected, "{evidence:?}");
    assert_eq!(evidence.rail, Some("ctf-invalid-opponent-base-return-drop"));
    assert_eq!(
        evidence.invalid_action,
        Some("opponent_base_return_drop_without_carrier")
    );
    assert_eq!(
        evidence.postcondition_milestone,
        Some("ctf_invalid_opponent_base_return_drop_contained")
    );
    assert!(evidence.preflight_passed, "{evidence:?}");
}

#[test]
fn ctf_score_limit_win_condition_tracks_client_server_and_forbidden_guards() {
    let client = evaluate_scenario(
        Scenario::CtfScoreLimitWinCondition,
        "Detected server protocol version 763\njoin_game\nrender_tick_with_player\nYou are on team RED!\nYou have the flag!\nYou captured the flag!\nRED: 2\nctf_score_limit_win_seen score_limit=2 winning_team=red red_score=2 blue_score=0 end_state=winner_declared duplicate_win=false\n",
    );
    assert!(client.passed, "{client:?}");
    assert!(client.forbidden_matches.is_empty(), "{client:?}");

    let duplicate = evaluate_scenario(
        Scenario::CtfScoreLimitWinCondition,
        "Detected server protocol version 763\njoin_game\nrender_tick_with_player\nYou are on team RED!\nYou have the flag!\nYou captured the flag!\nRED: 2\nctf_score_limit_win_seen score_limit=2 winning_team=red red_score=2 blue_score=0 end_state=winner_declared duplicate_win=false\nscore_limit_duplicate_win\n",
    );
    assert!(!duplicate.passed, "{duplicate:?}");
    assert!(
        duplicate
            .forbidden_matches
            .contains(&"score_limit_duplicate_win"),
        "{duplicate:?}"
    );

    let server = evaluate_server_scenario(
        Scenario::CtfScoreLimitWinCondition,
        "compatbot joined\nMC-COMPAT-MILESTONE score_limit_pre_state score_limit=2 red_score=1 blue_score=0 next_capture_team=Red outcome=one_capture_before_win\nMC-COMPAT-MILESTONE score_limit_final_capture username=compatbot capture_team=Red carried_flag=Blue score_limit=2 red_score_before=1 blue_score_before=0 red_score_after=2 blue_score_after=0\nMC-COMPAT-MILESTONE score_limit_win_condition username=compatbot winning_team=Red score_limit=2 red_score=2 blue_score=0 end_state=winner_declared win_emissions=1 duplicate_win=false post_win_score_delta=0\n",
        "compatbot",
    );
    assert!(server.passed, "{server:?}");
    assert!(server.forbidden_matches.is_empty(), "{server:?}");

    let mutation = evaluate_server_scenario(
        Scenario::CtfScoreLimitWinCondition,
        "compatbot joined\nMC-COMPAT-MILESTONE score_limit_pre_state score_limit=2 red_score=1 blue_score=0 next_capture_team=Red outcome=one_capture_before_win\nMC-COMPAT-MILESTONE score_limit_final_capture username=compatbot capture_team=Red carried_flag=Blue score_limit=2 red_score_before=1 blue_score_before=0 red_score_after=2 blue_score_after=0\nMC-COMPAT-MILESTONE score_limit_win_condition username=compatbot winning_team=Red score_limit=2 red_score=2 blue_score=0 end_state=winner_declared win_emissions=1 duplicate_win=false post_win_score_delta=0\nMC-COMPAT-MILESTONE score_limit_post_win_score_mutation username=compatbot winning_team=Red score_limit=2 outcome=forbidden_score_after_win\n",
        "compatbot",
    );
    assert!(!mutation.passed, "{mutation:?}");
    assert!(
        mutation
            .forbidden_matches
            .contains(&"score_limit_post_win_score_mutation"),
        "{mutation:?}"
    );
}

#[test]
fn combat_damage_scenario_tracks_client_and_server_evidence() {
    let cfg = test_config(
        &["--scenario", "combat-damage"],
        &[("CLIENT_TIMEOUT", "150")],
    )
    .expect("combat config parses");
    assert_eq!(
        planned_client_usernames(&cfg),
        vec!["compatbota", "compatbotb"]
    );

    let client = evaluate_scenario(
        Scenario::CombatDamage,
        "mc_compat_combat_client_count=2\nDetected server protocol version 763\njoin_game\nrender_tick_with_player\nYou are on team RED!\nYou are on team BLUE!\nremote_player_spawn\ncombat_probe_attack_sent\nupdate_health health=16.0\n",
    );
    assert!(client.passed, "{client:?}");

    let server = evaluate_server_scenario(
        Scenario::CombatDamage,
        "compatbota joined\ncompatbotb joined\nMC-COMPAT-MILESTONE combat_damage attacker=compatbota victim=compatbotb damage=4.0 victim_health_before=20.0 victim_health_after=16.0 attacker_item=WoodenSword\n",
        "compatbot",
    );
    assert!(server.passed, "{server:?}");

    let missing_damage = evaluate_server_scenario(
        Scenario::CombatDamage,
        "compatbota joined\ncompatbotb joined\n",
        "compatbot",
    );
    assert!(!missing_damage.passed, "{missing_damage:?}");
    assert!(missing_damage
        .missing_milestones
        .contains(&"server_combat_damage"));
}

#[test]
fn armor_loadout_enchantment_status_matrix_tracks_isolated_row_evidence() {
    let cfg = test_config(
        &["--scenario", "armor-loadout-enchantment-status-matrix"],
        &[("CLIENT_TIMEOUT", "150")],
    )
    .expect("armor matrix config parses");
    assert_eq!(
        planned_client_usernames(&cfg),
        vec!["compatbota", "compatbotb"]
    );

    let client = evaluate_scenario(
        Scenario::ArmorLoadoutEnchantmentStatusMatrix,
        "mc_compat_combat_client_count=2\nDetected server protocol version 763\njoin_game\nrender_tick_with_player\nYou are on team RED!\nYou are on team BLUE!\nremote_player_spawn\ninventory_probe_set_slot\ncombat_probe_attack_sent\nupdate_health health=18.0\n",
    );
    assert!(client.passed, "{client:?}");
    assert!(client.missing_milestones.is_empty());

    let server = evaluate_server_scenario(
        Scenario::ArmorLoadoutEnchantmentStatusMatrix,
        "compatbota joined\ncompatbotb joined\nMC-COMPAT-MILESTONE armor_equipment_state username=compatbotb slot=chest item=DiamondChestplate source=team_inventory_setup\nMC-COMPAT-MILESTONE combat_damage attacker=compatbota victim=compatbotb damage=2.0 victim_health_before=20.0 victim_health_after=18.0 attacker_item=WoodenSword\nMC-COMPAT-MILESTONE combat_armor_mitigation attacker=compatbota victim=compatbotb base_damage=4.0 mitigation=2.0 final_damage=2.0 chest_item=DiamondChestplate victim_health_before=20.0 victim_health_after=18.0\n",
        "compatbot",
    );
    assert!(server.passed, "{server:?}");
    assert!(server.missing_milestones.is_empty());

    let missing_equipment = evaluate_server_scenario(
        Scenario::ArmorLoadoutEnchantmentStatusMatrix,
        "compatbota joined\ncompatbotb joined\nMC-COMPAT-MILESTONE combat_damage attacker=compatbota victim=compatbotb damage=2.0 victim_health_before=20.0 victim_health_after=18.0 attacker_item=WoodenSword\nMC-COMPAT-MILESTONE combat_armor_mitigation attacker=compatbota victim=compatbotb base_damage=4.0 mitigation=2.0 final_damage=2.0 chest_item=DiamondChestplate victim_health_before=20.0 victim_health_after=18.0\n",
        "compatbot",
    );
    assert!(!missing_equipment.passed, "{missing_equipment:?}");
    assert!(missing_equipment
        .missing_milestones
        .contains(&"server_equipment_state"));
}

#[test]
fn equipment_update_scenario_tracks_current_client_equipment_marker() {
    let client = evaluate_scenario(
        Scenario::EquipmentUpdateObservation,
        "mc_compat_equipment_update_client_count=2\nDetected server protocol version 763\njoin_game\nrender_tick_with_player\nYou are on team RED!\nYou are on team BLUE!\nremote_player_spawn\nequipment_probe_entity_equipment entity_id=4 entries=1 slots=slot4:id=829:count=1\n",
    );
    assert!(client.passed, "{client:?}");
    assert!(client.missing_milestones.is_empty());

    let missing_equipment = evaluate_scenario(
        Scenario::EquipmentUpdateObservation,
        "mc_compat_equipment_update_client_count=2\nDetected server protocol version 763\njoin_game\nrender_tick_with_player\nYou are on team RED!\nYou are on team BLUE!\nremote_player_spawn\n",
    );
    assert!(!missing_equipment.passed, "{missing_equipment:?}");
    assert!(missing_equipment
        .missing_milestones
        .contains(&"entity_equipment_update"));
}

#[test]
fn equipment_slot_item_matrix_expansion_tracks_isolated_row_evidence() {
    let cfg = test_config(
        &["--scenario", "equipment-slot-item-matrix-expansion"],
        &[("CLIENT_TIMEOUT", "150")],
    )
    .expect("equipment matrix config parses");
    assert_eq!(
        planned_client_usernames(&cfg),
        vec!["compatbota", "compatbotb"]
    );

    let client = evaluate_scenario(
        Scenario::EquipmentSlotItemMatrixExpansion,
        "mc_compat_equipment_update_client_count=2\nDetected server protocol version 763\njoin_game\nrender_tick_with_player\nYou are on team RED!\nYou are on team BLUE!\nremote_player_spawn entity_id=4\nequipment_probe_entity_equipment entity_id=4 entries=1 slots=slot4:id=829:count=1\n",
    );
    assert!(client.passed, "{client:?}");
    assert!(client.missing_milestones.is_empty());

    let server = evaluate_server_scenario(
        Scenario::EquipmentSlotItemMatrixExpansion,
        "compatbota joined\ncompatbotb joined\nMC-COMPAT-MILESTONE equipment_update_state username=compatbotb slot=main_hand item_id=829 count=1\n",
        "compatbot",
    );
    assert!(server.passed, "{server:?}");
    assert!(server.missing_milestones.is_empty());

    let missing_update = evaluate_scenario(
        Scenario::EquipmentSlotItemMatrixExpansion,
        "mc_compat_equipment_update_client_count=2\nDetected server protocol version 763\njoin_game\nrender_tick_with_player\nYou are on team RED!\nYou are on team BLUE!\nremote_player_spawn entity_id=4\n",
    );
    assert!(!missing_update.passed, "{missing_update:?}");
    assert!(missing_update
        .missing_milestones
        .contains(&"entity_equipment_update"));
}

#[test]
fn blue_flag_score_scenario_tracks_mirrored_team_evidence() {
    let pass = evaluate_scenario(
        Scenario::BlueFlagScore,
        "Detected server protocol version 763
join_game
render_tick_with_player
You are on team BLUE!
You have the flag!
You captured the flag!
BLUE: 1
",
    );
    assert!(pass.passed, "{pass:?}");
    assert!(pass.missing_milestones.is_empty());

    let fail = evaluate_scenario(
        Scenario::BlueFlagScore,
        "Detected server protocol version 763
join_game
render_tick_with_player
You are on team RED!
You have the flag!
You captured the flag!
RED: 1
",
    );
    assert!(!fail.passed);
    assert!(fail.missing_milestones.contains(&"team_blue"));
    assert!(fail.missing_milestones.contains(&"score_blue_1"));
}

#[test]
fn inventory_interaction_scenario_tracks_client_and_server_evidence() {
    let client = evaluate_scenario(
        Scenario::InventoryInteraction,
        "Detected server protocol version 763\njoin_game\nrender_tick_with_player\nYou are on team RED!\ninventory_probe_set_slot\ninventory_probe_slot36_nonempty\ninventory_probe_slot37_stack\ninventory_probe_drop_item_sent\ninventory_probe_collect_item\ninventory_probe_click_slot_sent\ninventory_probe_open_container\ninventory_probe_container_click_sent\ninventory_probe_place_block_sent\n",
    );
    assert!(client.passed, "{client:?}");

    let missing_drop = evaluate_scenario(
        Scenario::InventoryInteraction,
        "Detected server protocol version 763\njoin_game\nrender_tick_with_player\nYou are on team RED!\ninventory_probe_set_slot\ninventory_probe_slot36_nonempty\ninventory_probe_slot37_stack\n",
    );
    assert!(!missing_drop.passed);
    assert!(missing_drop
        .missing_milestones
        .contains(&"inventory_drop_sent"));

    let server = evaluate_server_scenario(
        Scenario::InventoryInteraction,
        "compatbot joined\nMC-COMPAT-MILESTONE inventory_hotbar_select username=compatbot slot=0\nMC-COMPAT-MILESTONE inventory_drop_item username=compatbot from_slot=36 item=WoodenSword count=1\nMC-COMPAT-MILESTONE inventory_pickup_item username=compatbot from_slot=36 item=WoodenSword count=1 collected_entity_id=7630036 collector_entity_id=1\nMC-COMPAT-MILESTONE inventory_click_slot username=compatbot window=0 slot=37 button=0 mode=click carried_item=RedWool count=63 slot_after=empty\nMC-COMPAT-MILESTONE inventory_open_container username=compatbot kind=Generic3x3 trigger=inventory_click_slot\nMC-COMPAT-MILESTONE inventory_container_click username=compatbot window=1 slot=0 button=0 mode=click carried_item=Air count=0 slot_changes=1\nMC-COMPAT-MILESTONE block_place_item username=compatbot item=RedWool from_slot=37 block=RedWool at=-40,65,0\n",
        "compatbot",
    );
    assert!(server.passed, "{server:?}");

    let missing_drop = evaluate_server_scenario(
        Scenario::InventoryInteraction,
        "compatbot joined\nMC-COMPAT-MILESTONE inventory_hotbar_select username=compatbot slot=0\n",
        "compatbot",
    );
    assert!(!missing_drop.passed, "{missing_drop:?}");
    assert!(missing_drop
        .missing_milestones
        .contains(&"server_inventory_drop"));
}

#[test]
fn inventory_stack_split_merge_tracks_client_and_server_evidence() {
    let client = evaluate_scenario(
        Scenario::InventoryStackSplitMerge,
        "Detected server protocol version 763\njoin_game\nrender_tick_with_player\nYou are on team RED!\ninventory_stack_initial_slot window=0 state_id=1\ninventory_stack_split_pickup_sent\ninventory_stack_split_source_seen\ninventory_stack_split_place_sent\ninventory_stack_split_destination_seen\ninventory_stack_merge_pickup_sent\ninventory_stack_merge_destination_empty_seen\ninventory_stack_merge_place_sent\ninventory_stack_final_source_seen\n",
    );
    assert!(client.passed, "{client:?}");

    let missing_final = evaluate_scenario(
        Scenario::InventoryStackSplitMerge,
        "Detected server protocol version 763\njoin_game\nrender_tick_with_player\nYou are on team RED!\ninventory_stack_initial_slot window=0 state_id=1\ninventory_stack_split_pickup_sent\ninventory_stack_split_source_seen\ninventory_stack_split_place_sent\ninventory_stack_split_destination_seen\ninventory_stack_merge_pickup_sent\ninventory_stack_merge_destination_empty_seen\ninventory_stack_merge_place_sent\n",
    );
    assert!(!missing_final.passed, "{missing_final:?}");
    assert!(missing_final
        .missing_milestones
        .contains(&"inventory_stack_final_source_seen"));

    let server = evaluate_server_scenario(
        Scenario::InventoryStackSplitMerge,
        "compatbot joined\nMC-COMPAT-MILESTONE inventory_stack_server_split_pickup username=compatbot window=0 state_id=1 source_slot=37 button=1 mode=Click item=RedWool source_count_after=32 carried_count=32\nMC-COMPAT-MILESTONE inventory_stack_server_split username=compatbot window=0 state_id_sequence=1->2 source_slot=37 destination_slot=38 button=0 mode=Click item=RedWool source_count_after=32 destination_count_after=32 carried_count=0\nMC-COMPAT-MILESTONE inventory_stack_server_merge_pickup username=compatbot window=0 state_id=3 destination_slot=38 button=0 mode=Click item=RedWool destination_count_after=0 carried_count=32\nMC-COMPAT-MILESTONE inventory_stack_server_merge username=compatbot window=0 state_id_sequence=2->3->4 source_slot=37 destination_slot=38 button=0 mode=Click item=RedWool source_count_after=64 destination_count_after=0 carried_count=0\n",
        "compatbot",
    );
    assert!(server.passed, "{server:?}");

    let missing_merge = evaluate_server_scenario(
        Scenario::InventoryStackSplitMerge,
        "compatbot joined\nMC-COMPAT-MILESTONE inventory_stack_server_split_pickup username=compatbot\nMC-COMPAT-MILESTONE inventory_stack_server_split username=compatbot\n",
        "compatbot",
    );
    assert!(!missing_merge.passed, "{missing_merge:?}");
    assert!(missing_merge
        .missing_milestones
        .contains(&"server_inventory_stack_merge"));
}

#[test]
fn inventory_drag_transactions_tracks_client_and_server_evidence() {
    let client = evaluate_scenario(
        Scenario::InventoryDragTransactions,
        "Detected server protocol version 763\njoin_game\nrender_tick_with_player\nYou are on team RED!\ninventory_drag_initial_slot window=0 state_id=1\ninventory_drag_pickup_sent\ninventory_drag_source_empty_seen\ninventory_drag_start_sent\ninventory_drag_target_a_sent\ninventory_drag_target_b_sent\ninventory_drag_end_sent\ninventory_drag_final_distribution_seen\n",
    );
    assert!(client.passed, "{client:?}");

    let missing_final = evaluate_scenario(
        Scenario::InventoryDragTransactions,
        "Detected server protocol version 763\njoin_game\nrender_tick_with_player\nYou are on team RED!\ninventory_drag_initial_slot window=0 state_id=1\ninventory_drag_pickup_sent\ninventory_drag_source_empty_seen\ninventory_drag_start_sent\ninventory_drag_target_a_sent\ninventory_drag_target_b_sent\ninventory_drag_end_sent\n",
    );
    assert!(!missing_final.passed, "{missing_final:?}");
    assert!(missing_final
        .missing_milestones
        .contains(&"inventory_drag_final_distribution_seen"));

    let server = evaluate_server_scenario(
        Scenario::InventoryDragTransactions,
        "compatbot joined\nMC-COMPAT-MILESTONE inventory_drag_server_pickup username=compatbot window=0 state_id=1 source_slot=37 button=0 mode=Click item=RedWool source_count_after=0 carried_count=64\nMC-COMPAT-MILESTONE inventory_drag_server_start username=compatbot window=0 state_id_sequence=1->2 slot=-999 button=0 mode=Drag item=RedWool carried_count=64\nMC-COMPAT-MILESTONE inventory_drag_server_target_a username=compatbot window=0 state_id=3 target_slot=38 button=1 mode=Drag item=RedWool carried_count=64\nMC-COMPAT-MILESTONE inventory_drag_server_target_b username=compatbot window=0 state_id_sequence=3->4 target_slots=38,39 button=1 mode=Drag item=RedWool carried_count=64\nMC-COMPAT-MILESTONE inventory_drag_server_end username=compatbot window=0 state_id_sequence=1->2->3->4->5 source_slot=37 target_slots=38,39 button=2 mode=Drag item=RedWool source_count_after=0 target_counts=32,32 carried_count=0\n",
        "compatbot",
    );
    assert!(server.passed, "{server:?}");

    let missing_end = evaluate_server_scenario(
        Scenario::InventoryDragTransactions,
        "compatbot joined\nMC-COMPAT-MILESTONE inventory_drag_server_pickup username=compatbot\nMC-COMPAT-MILESTONE inventory_drag_server_start username=compatbot\nMC-COMPAT-MILESTONE inventory_drag_server_target_a username=compatbot\nMC-COMPAT-MILESTONE inventory_drag_server_target_b username=compatbot\n",
        "compatbot",
    );
    assert!(!missing_end.passed, "{missing_end:?}");
    assert!(missing_end
        .missing_milestones
        .contains(&"server_inventory_drag_end"));
}

#[test]
fn survival_break_place_pickup_scenario_tracks_client_and_server_evidence() {
    let client = evaluate_scenario(
        Scenario::SurvivalBreakPlacePickup,
        "Detected server protocol version 763\njoin_game\nrender_tick_with_player\nsurvival_probe_break_block_sent\nsurvival_probe_block_update\nsurvival_probe_pickup_seen\nsurvival_probe_place_block_sent\nsurvival_probe_place_update\n",
    );
    assert!(client.passed, "{client:?}");
    assert!(client.missing_milestones.is_empty());

    let missing_pickup = evaluate_scenario(
        Scenario::SurvivalBreakPlacePickup,
        "Detected server protocol version 763\njoin_game\nrender_tick_with_player\nsurvival_probe_break_block_sent\nsurvival_probe_block_update\n",
    );
    assert!(!missing_pickup.passed, "{missing_pickup:?}");
    assert!(missing_pickup
        .missing_milestones
        .contains(&"survival_pickup_seen"));

    let server = evaluate_server_scenario(
        Scenario::SurvivalBreakPlacePickup,
        "compatbot joined\nMC-COMPAT-MILESTONE survival_join username=compatbot gamemode=Survival\nMC-COMPAT-MILESTONE survival_block_break username=compatbot item=Dirt at=0,64,1\nMC-COMPAT-MILESTONE survival_pickup_item username=compatbot slot=36 item=Dirt count=1\nMC-COMPAT-MILESTONE survival_block_place username=compatbot item=Dirt from_slot=36 at=0,65,1\n",
        "compatbot",
    );
    assert!(server.passed, "{server:?}");

    let missing_place = evaluate_server_scenario(
        Scenario::SurvivalBreakPlacePickup,
        "compatbot joined\nMC-COMPAT-MILESTONE survival_join username=compatbot gamemode=Survival\nMC-COMPAT-MILESTONE survival_block_break username=compatbot item=Dirt at=0,64,1\n",
        "compatbot",
    );
    assert!(!missing_place.passed, "{missing_place:?}");
    assert!(missing_place
        .missing_milestones
        .contains(&"server_survival_place"));
}

#[test]
fn survival_chest_persistence_scenario_tracks_client_and_server_evidence() {
    let client = evaluate_scenario(
        Scenario::SurvivalChestPersistence,
        "Detected server protocol version 763\njoin_game\nrender_tick_with_player\nsurvival_chest_open_seen window=1 position=8,64,0\nsurvival_chest_store_sent window=1 slot=0 item=Dirt count=1\nsurvival_chest_close_sent window=1\nsurvival_chest_reconnect_sent session=1\nsurvival_chest_reopen_seen window=1 position=8,64,0\nsurvival_chest_persisted_seen window=1 slot=0 item=Dirt count=1\n",
    );
    assert!(client.passed, "{client:?}");
    assert!(client.missing_milestones.is_empty());

    let missing_reopen = evaluate_scenario(
        Scenario::SurvivalChestPersistence,
        "Detected server protocol version 763\njoin_game\nrender_tick_with_player\nsurvival_chest_open_seen window=1 position=8,64,0\nsurvival_chest_store_sent window=1 slot=0 item=Dirt count=1\nsurvival_chest_close_sent window=1\nsurvival_chest_reconnect_sent session=1\n",
    );
    assert!(!missing_reopen.passed, "{missing_reopen:?}");
    assert!(missing_reopen
        .missing_milestones
        .contains(&"survival_chest_reopen_seen"));

    let wrong_client_values = evaluate_scenario(
        Scenario::SurvivalChestPersistence,
        "Detected server protocol version 763\njoin_game\nrender_tick_with_player\nsurvival_chest_open_seen window=1 position=9,64,0\nsurvival_chest_store_sent window=1 slot=1 item=Stone count=2\nsurvival_chest_close_sent window=1\nsurvival_chest_reconnect_sent session=2\nsurvival_chest_reopen_seen window=1 position=9,64,0\nsurvival_chest_persisted_seen window=1 slot=1 item=Stone count=2\n",
    );
    assert!(!wrong_client_values.passed, "{wrong_client_values:?}");
    assert!(wrong_client_values
        .missing_milestones
        .contains(&"survival_chest_open_seen"));
    assert!(wrong_client_values
        .missing_milestones
        .contains(&"survival_chest_store_sent"));
    assert!(wrong_client_values
        .missing_milestones
        .contains(&"survival_chest_reconnect_sent"));
    assert!(wrong_client_values
        .missing_milestones
        .contains(&"survival_chest_reopen_seen"));
    assert!(wrong_client_values
        .missing_milestones
        .contains(&"survival_chest_persisted_seen"));

    let wrong_reopen_window = evaluate_scenario(
        Scenario::SurvivalChestPersistence,
        "Detected server protocol version 763\njoin_game\nrender_tick_with_player\nsurvival_chest_open_seen window=1 position=8,64,0\nsurvival_chest_store_sent window=1 slot=0 item=Dirt count=1\nsurvival_chest_close_sent window=1\nsurvival_chest_reconnect_sent session=1\nsurvival_chest_reopen_seen window=3 position=8,64,0\nsurvival_chest_persisted_seen window=3 slot=0 item=Dirt count=1\n",
    );
    assert!(!wrong_reopen_window.passed, "{wrong_reopen_window:?}");
    assert!(wrong_reopen_window
        .missing_milestones
        .contains(&"survival_chest_reopen_seen"));
    assert!(wrong_reopen_window
        .missing_milestones
        .contains(&"survival_chest_persisted_seen"));

    let server = evaluate_server_scenario(
        Scenario::SurvivalChestPersistence,
        "compatbot joined\nMC-COMPAT-MILESTONE survival_chest_open username=compatbot position=8,64,0 window=1\nMC-COMPAT-MILESTONE survival_chest_store username=compatbot window=1 slot=0 item=Dirt count=1\nMC-COMPAT-MILESTONE survival_chest_close username=compatbot window=1\nMC-COMPAT-MILESTONE survival_chest_reopen username=compatbot position=8,64,0 window=1\nMC-COMPAT-MILESTONE survival_chest_persisted username=compatbot slot=0 item=Dirt count=1\n",
        "compatbot",
    );
    assert!(server.passed, "{server:?}");

    let missing_store = evaluate_server_scenario(
        Scenario::SurvivalChestPersistence,
        "compatbot joined\nMC-COMPAT-MILESTONE survival_chest_open username=compatbot position=8,64,0 window=1\n",
        "compatbot",
    );
    assert!(!missing_store.passed, "{missing_store:?}");
    assert!(missing_store
        .missing_milestones
        .contains(&"server_survival_chest_store"));

    let wrong_server_values = evaluate_server_scenario(
        Scenario::SurvivalChestPersistence,
        "compatbot joined\nMC-COMPAT-MILESTONE survival_chest_open username=compatbot position=9,64,0 window=1\nMC-COMPAT-MILESTONE survival_chest_store username=compatbot window=1 slot=1 item=Stone count=2\nMC-COMPAT-MILESTONE survival_chest_close username=compatbot window=1\nMC-COMPAT-MILESTONE survival_chest_reopen username=compatbot position=9,64,0 window=1\nMC-COMPAT-MILESTONE survival_chest_persisted username=compatbot slot=1 item=Stone count=2\n",
        "compatbot",
    );
    assert!(!wrong_server_values.passed, "{wrong_server_values:?}");
    assert!(wrong_server_values
        .missing_milestones
        .contains(&"server_survival_chest_open"));
    assert!(wrong_server_values
        .missing_milestones
        .contains(&"server_survival_chest_store"));
    assert!(wrong_server_values
        .missing_milestones
        .contains(&"server_survival_chest_reopen"));
    assert!(wrong_server_values
        .missing_milestones
        .contains(&"server_survival_chest_persisted"));

    let wrong_server_reopen_window = evaluate_server_scenario(
        Scenario::SurvivalChestPersistence,
        "compatbot joined\nMC-COMPAT-MILESTONE survival_chest_open username=compatbot position=8,64,0 window=1\nMC-COMPAT-MILESTONE survival_chest_store username=compatbot window=1 slot=0 item=Dirt count=1\nMC-COMPAT-MILESTONE survival_chest_close username=compatbot window=1\nMC-COMPAT-MILESTONE survival_chest_reopen username=compatbot position=8,64,0 window=3\nMC-COMPAT-MILESTONE survival_chest_persisted username=compatbot slot=0 item=Dirt count=1\n",
        "compatbot",
    );
    assert!(
        !wrong_server_reopen_window.passed,
        "{wrong_server_reopen_window:?}"
    );
    assert!(wrong_server_reopen_window
        .missing_milestones
        .contains(&"server_survival_chest_reopen"));
}

#[test]
fn survival_crafting_table_scenario_tracks_client_and_server_evidence() {
    let client = evaluate_scenario(
        Scenario::SurvivalCraftingTable,
        "Detected server protocol version 763\njoin_game\nrender_tick_with_player\nsurvival_crafting_table_open_seen window=1 position=4,64,0\nsurvival_crafting_input_a_sent window=1 slot=1 item=OakPlanks count=1\nsurvival_crafting_input_b_sent window=1 slot=4 item=OakPlanks count=1\nsurvival_crafting_result_seen window=1 slot=0 item=Stick count=4 recipe=minecraft:stick\nsurvival_crafting_result_collected window=1 slot=0 item=Stick count=4\nsurvival_crafting_inventory_updated slot=36 item=Stick count=4\n",
    );
    assert!(client.passed, "{client:?}");
    assert!(client.missing_milestones.is_empty());

    let missing_result = evaluate_scenario(
        Scenario::SurvivalCraftingTable,
        "Detected server protocol version 763\njoin_game\nrender_tick_with_player\nsurvival_crafting_table_open_seen window=1 position=4,64,0\nsurvival_crafting_input_a_sent window=1 slot=1 item=OakPlanks count=1\nsurvival_crafting_input_b_sent window=1 slot=4 item=OakPlanks count=1\n",
    );
    assert!(!missing_result.passed, "{missing_result:?}");
    assert!(missing_result
        .missing_milestones
        .contains(&"survival_crafting_result_seen"));

    let wrong_client_values = evaluate_scenario(
        Scenario::SurvivalCraftingTable,
        "Detected server protocol version 763\njoin_game\nrender_tick_with_player\nsurvival_crafting_table_open_seen window=1 position=5,64,0\nsurvival_crafting_input_a_sent window=1 slot=2 item=Stone count=2\nsurvival_crafting_input_b_sent window=1 slot=5 item=Stone count=2\nsurvival_crafting_result_seen window=1 slot=0 item=Stone count=2 recipe=minecraft:stone\nsurvival_crafting_result_collected window=1 slot=0 item=Stone count=2\nsurvival_crafting_inventory_updated slot=37 item=Stone count=2\n",
    );
    assert!(!wrong_client_values.passed, "{wrong_client_values:?}");
    assert!(wrong_client_values
        .missing_milestones
        .contains(&"survival_crafting_table_open_seen"));
    assert!(wrong_client_values
        .missing_milestones
        .contains(&"survival_crafting_input_a_sent"));
    assert!(wrong_client_values
        .missing_milestones
        .contains(&"survival_crafting_result_seen"));

    let server = evaluate_server_scenario(
        Scenario::SurvivalCraftingTable,
        "compatbot joined\nMC-COMPAT-MILESTONE survival_crafting_table_open username=compatbot position=4,64,0 window=1\nMC-COMPAT-MILESTONE survival_crafting_input_a username=compatbot window=1 slot=1 item=OakPlanks count=1\nMC-COMPAT-MILESTONE survival_crafting_input_b username=compatbot window=1 slot=4 item=OakPlanks count=1\nMC-COMPAT-MILESTONE survival_crafting_result username=compatbot window=1 slot=0 item=Stick count=4 recipe=minecraft:stick\nMC-COMPAT-MILESTONE survival_crafting_collect username=compatbot window=1 slot=0 item=Stick count=4 inventory_slot=36\n",
        "compatbot",
    );
    assert!(server.passed, "{server:?}");

    let missing_collect = evaluate_server_scenario(
        Scenario::SurvivalCraftingTable,
        "compatbot joined\nMC-COMPAT-MILESTONE survival_crafting_table_open username=compatbot position=4,64,0 window=1\nMC-COMPAT-MILESTONE survival_crafting_input_a username=compatbot window=1 slot=1 item=OakPlanks count=1\n",
        "compatbot",
    );
    assert!(!missing_collect.passed, "{missing_collect:?}");
    assert!(missing_collect
        .missing_milestones
        .contains(&"server_survival_crafting_collect"));
}

#[test]
fn survival_furnace_persistence_scenario_tracks_client_and_server_evidence() {
    let client = evaluate_scenario(
        Scenario::SurvivalFurnacePersistence,
        "Detected server protocol version 763\njoin_game\nrender_tick_with_player\nsurvival_furnace_open_seen window=1 position=12,64,0\nsurvival_furnace_input_sent window=1 slot=0 item=RawIron count=1\nsurvival_furnace_fuel_sent window=1 slot=1 item=Coal count=1\nsurvival_furnace_burn_progress_seen window=1 progress=started\nsurvival_furnace_output_seen window=1 slot=2 item=IronIngot count=1\nsurvival_furnace_output_collected window=1 slot=2 item=IronIngot count=1\nsurvival_furnace_inventory_updated slot=36 item=IronIngot count=1\nsurvival_furnace_reconnect_sent session=1\nsurvival_furnace_reopen_seen window=1 position=12,64,0\n",
    );
    assert!(client.passed, "{client:?}");
    assert!(client.missing_milestones.is_empty());

    let missing_output = evaluate_scenario(
        Scenario::SurvivalFurnacePersistence,
        "Detected server protocol version 763\njoin_game\nrender_tick_with_player\nsurvival_furnace_open_seen window=1 position=12,64,0\nsurvival_furnace_input_sent window=1 slot=0 item=RawIron count=1\nsurvival_furnace_fuel_sent window=1 slot=1 item=Coal count=1\n",
    );
    assert!(!missing_output.passed, "{missing_output:?}");
    assert!(missing_output
        .missing_milestones
        .contains(&"survival_furnace_output_seen"));

    let wrong_client_values = evaluate_scenario(
        Scenario::SurvivalFurnacePersistence,
        "Detected server protocol version 763\njoin_game\nrender_tick_with_player\nsurvival_furnace_open_seen window=2 position=13,64,0\nsurvival_furnace_input_sent window=2 slot=0 item=Sand count=1\nsurvival_furnace_fuel_sent window=2 slot=1 item=Charcoal count=1\nsurvival_furnace_burn_progress_seen window=2 progress=done\nsurvival_furnace_output_seen window=2 slot=2 item=Glass count=1\nsurvival_furnace_output_collected window=2 slot=2 item=Glass count=1\nsurvival_furnace_inventory_updated slot=37 item=Glass count=1\nsurvival_furnace_reconnect_sent session=2\nsurvival_furnace_reopen_seen window=2 position=13,64,0\n",
    );
    assert!(!wrong_client_values.passed, "{wrong_client_values:?}");
    assert!(wrong_client_values
        .missing_milestones
        .contains(&"survival_furnace_open_seen"));
    assert!(wrong_client_values
        .missing_milestones
        .contains(&"survival_furnace_input_sent"));
    assert!(wrong_client_values
        .missing_milestones
        .contains(&"survival_furnace_reopen_seen"));

    let server = evaluate_server_scenario(
        Scenario::SurvivalFurnacePersistence,
        "compatbot joined\nMC-COMPAT-MILESTONE survival_furnace_open username=compatbot position=12,64,0 window=1\nMC-COMPAT-MILESTONE survival_furnace_input_insert username=compatbot window=1 slot=0 item=RawIron count=1\nMC-COMPAT-MILESTONE survival_furnace_fuel_insert username=compatbot window=1 slot=1 item=Coal count=1\nMC-COMPAT-MILESTONE survival_furnace_burn_progress username=compatbot window=1 progress=started\nMC-COMPAT-MILESTONE survival_furnace_output_available username=compatbot window=1 slot=2 item=IronIngot count=1\nMC-COMPAT-MILESTONE survival_furnace_output_collect username=compatbot window=1 slot=2 item=IronIngot count=1 inventory_slot=36\nMC-COMPAT-MILESTONE survival_furnace_reconnect_reopen username=compatbot position=12,64,0 window=1\nMC-COMPAT-MILESTONE survival_furnace_server_state username=compatbot position=12,64,0 input=RawIron fuel=Coal output=empty collected=true session_persistent=true\n",
        "compatbot",
    );
    assert!(server.passed, "{server:?}");

    let missing_state = evaluate_server_scenario(
        Scenario::SurvivalFurnacePersistence,
        "compatbot joined\nMC-COMPAT-MILESTONE survival_furnace_open username=compatbot position=12,64,0 window=1\n",
        "compatbot",
    );
    assert!(!missing_state.passed, "{missing_state:?}");
    assert!(missing_state
        .missing_milestones
        .contains(&"server_survival_furnace_state"));
}

#[test]
fn survival_furnace_smelting_breadth_scenario_tracks_client_and_server_evidence() {
    let client = evaluate_scenario(
        Scenario::SurvivalFurnaceSmeltingBreadth,
        "Detected server protocol version 763\njoin_game\nrender_tick_with_player\nsurvival_furnace_open_seen window=1 position=12,64,0\nsurvival_furnace_input_sent window=1 slot=0 item=RawIron count=1\nsurvival_furnace_fuel_sent window=1 slot=1 item=Coal count=1\nsurvival_furnace_burn_progress_seen window=1 progress=started\nsurvival_furnace_output_seen window=1 slot=2 item=IronIngot count=1\nsurvival_furnace_output_collected window=1 slot=2 item=IronIngot count=1\nsurvival_furnace_inventory_updated slot=36 item=IronIngot count=1\nsurvival_furnace_invalid_fuel_sent window=1 slot=1 item=RawIron outcome=no_burn\n",
    );
    assert!(client.passed, "{client:?}");
    assert!(client.missing_milestones.is_empty());

    let missing_invalid = evaluate_scenario(
        Scenario::SurvivalFurnaceSmeltingBreadth,
        "Detected server protocol version 763\njoin_game\nrender_tick_with_player\nsurvival_furnace_open_seen window=1 position=12,64,0\nsurvival_furnace_input_sent window=1 slot=0 item=RawIron count=1\nsurvival_furnace_fuel_sent window=1 slot=1 item=Coal count=1\nsurvival_furnace_burn_progress_seen window=1 progress=started\nsurvival_furnace_output_seen window=1 slot=2 item=IronIngot count=1\nsurvival_furnace_output_collected window=1 slot=2 item=IronIngot count=1\nsurvival_furnace_inventory_updated slot=36 item=IronIngot count=1\n",
    );
    assert!(!missing_invalid.passed, "{missing_invalid:?}");
    assert!(missing_invalid
        .missing_milestones
        .contains(&"survival_furnace_invalid_fuel_sent"));

    let wrong_invalid_values = evaluate_scenario(
        Scenario::SurvivalFurnaceSmeltingBreadth,
        "Detected server protocol version 763\njoin_game\nrender_tick_with_player\nsurvival_furnace_open_seen window=1 position=12,64,0\nsurvival_furnace_input_sent window=1 slot=0 item=RawIron count=1\nsurvival_furnace_fuel_sent window=1 slot=1 item=Coal count=1\nsurvival_furnace_burn_progress_seen window=1 progress=started\nsurvival_furnace_output_seen window=1 slot=2 item=IronIngot count=1\nsurvival_furnace_output_collected window=1 slot=2 item=IronIngot count=1\nsurvival_furnace_inventory_updated slot=36 item=IronIngot count=1\nsurvival_furnace_invalid_fuel_sent window=1 slot=1 item=Coal outcome=burn\n",
    );
    assert!(!wrong_invalid_values.passed, "{wrong_invalid_values:?}");
    assert!(wrong_invalid_values
        .missing_milestones
        .contains(&"survival_furnace_invalid_fuel_sent"));

    let server = evaluate_server_scenario(
        Scenario::SurvivalFurnaceSmeltingBreadth,
        "compatbot joined\nMC-COMPAT-MILESTONE survival_furnace_open username=compatbot position=12,64,0 window=1\nMC-COMPAT-MILESTONE survival_furnace_input_insert username=compatbot window=1 slot=0 item=RawIron count=1\nMC-COMPAT-MILESTONE survival_furnace_fuel_insert username=compatbot window=1 slot=1 item=Coal count=1\nMC-COMPAT-MILESTONE survival_furnace_burn_progress username=compatbot window=1 progress=started\nMC-COMPAT-MILESTONE survival_furnace_output_available username=compatbot window=1 slot=2 item=IronIngot count=1\nMC-COMPAT-MILESTONE survival_furnace_output_collect username=compatbot window=1 slot=2 item=IronIngot count=1 inventory_slot=36\nMC-COMPAT-MILESTONE survival_furnace_invalid_fuel_rejected username=compatbot window=1 slot=1 item=RawIron outcome=no_burn\nMC-COMPAT-MILESTONE survival_furnace_breadth_state username=compatbot recipe=minecraft:iron_ingot input=RawIron fuel=Coal output=IronIngot count=1 invalid_fuel=RawIron invalid_fuel_outcome=no_burn broad_all_furnaces=false\n",
        "compatbot",
    );
    assert!(server.passed, "{server:?}");

    let missing_server_state = evaluate_server_scenario(
        Scenario::SurvivalFurnaceSmeltingBreadth,
        "compatbot joined\nMC-COMPAT-MILESTONE survival_furnace_open username=compatbot position=12,64,0 window=1\nMC-COMPAT-MILESTONE survival_furnace_input_insert username=compatbot window=1 slot=0 item=RawIron count=1\nMC-COMPAT-MILESTONE survival_furnace_fuel_insert username=compatbot window=1 slot=1 item=Coal count=1\nMC-COMPAT-MILESTONE survival_furnace_burn_progress username=compatbot window=1 progress=started\nMC-COMPAT-MILESTONE survival_furnace_output_available username=compatbot window=1 slot=2 item=IronIngot count=1\nMC-COMPAT-MILESTONE survival_furnace_output_collect username=compatbot window=1 slot=2 item=IronIngot count=1 inventory_slot=36\nMC-COMPAT-MILESTONE survival_furnace_invalid_fuel_rejected username=compatbot window=1 slot=1 item=RawIron outcome=no_burn\n",
        "compatbot",
    );
    assert!(!missing_server_state.passed, "{missing_server_state:?}");
    assert!(missing_server_state
        .missing_milestones
        .contains(&"server_survival_furnace_breadth_state"));
}

#[test]
fn survival_mob_drop_scenario_tracks_client_and_server_evidence() {
    let client = evaluate_scenario(
        Scenario::SurvivalMobDrop,
        "Detected server protocol version 763\njoin_game\nrender_tick_with_player\nsurvival_mob_drop_mob_seen mob=IronGolem position=16.5,65.0,2.5 target_id=42 entity_type=118\nsurvival_mob_drop_attack_sent mob=IronGolem target_id=42\nsurvival_mob_drop_death_seen mob=IronGolem target_id=42\nsurvival_mob_drop_drop_seen item=IronIngot count=1 entity_id=43 position=16.5,65.0,2.5\nsurvival_mob_drop_pickup_seen item=IronIngot count=1 collected_entity_id=43\nsurvival_mob_drop_inventory_updated slot=36 item=IronIngot count=1\n",
    );
    assert!(client.passed, "{client:?}");
    assert!(client.missing_milestones.is_empty());

    let missing_drop = evaluate_scenario(
        Scenario::SurvivalMobDrop,
        "Detected server protocol version 763\njoin_game\nrender_tick_with_player\nsurvival_mob_drop_mob_seen mob=IronGolem position=16.5,65.0,2.5 target_id=42 entity_type=118\nsurvival_mob_drop_attack_sent mob=IronGolem target_id=42\nsurvival_mob_drop_death_seen mob=IronGolem target_id=42\n",
    );
    assert!(!missing_drop.passed, "{missing_drop:?}");
    assert!(missing_drop
        .missing_milestones
        .contains(&"survival_mob_drop_drop_seen"));

    let wrong_client_values = evaluate_scenario(
        Scenario::SurvivalMobDrop,
        "Detected server protocol version 763\njoin_game\nrender_tick_with_player\nsurvival_mob_drop_mob_seen mob=Zombie position=16.5,65.0,2.5 target_id=42 entity_type=118\nsurvival_mob_drop_attack_sent mob=Zombie target_id=42\nsurvival_mob_drop_death_seen mob=Zombie target_id=42\nsurvival_mob_drop_drop_seen item=RottenFlesh count=2 entity_id=43 position=16.5,65.0,2.5\nsurvival_mob_drop_pickup_seen item=RottenFlesh count=2 collected_entity_id=43\nsurvival_mob_drop_inventory_updated slot=36 item=RottenFlesh count=2\n",
    );
    assert!(!wrong_client_values.passed, "{wrong_client_values:?}");
    assert!(wrong_client_values
        .missing_milestones
        .contains(&"survival_mob_drop_mob_seen"));
    assert!(wrong_client_values
        .missing_milestones
        .contains(&"survival_mob_drop_inventory_updated"));

    let server = evaluate_server_scenario(
        Scenario::SurvivalMobDrop,
        "compatbot joined\nMC-COMPAT-MILESTONE survival_mob_drop_spawn username=compatbot mob=IronGolem position=16.5,65.0,2.5 health=20.0 ai=false\nMC-COMPAT-MILESTONE survival_mob_drop_attack username=compatbot mob=IronGolem damage=20.0\nMC-COMPAT-MILESTONE survival_mob_drop_death username=compatbot mob=IronGolem cause=client_attack\nMC-COMPAT-MILESTONE survival_mob_drop_drop_spawn username=compatbot item=IronIngot count=1 extra_drops=false\nMC-COMPAT-MILESTONE survival_mob_drop_pickup username=compatbot item=IronIngot count=1\nMC-COMPAT-MILESTONE survival_mob_drop_inventory username=compatbot slot=36 item=IronIngot count=1\nMC-COMPAT-MILESTONE survival_mob_drop_state username=compatbot mob=IronGolem drop=IronIngot count=1 extra_drops=false\n",
        "compatbot",
    );
    assert!(server.passed, "{server:?}");

    let missing_state = evaluate_server_scenario(
        Scenario::SurvivalMobDrop,
        "compatbot joined\nMC-COMPAT-MILESTONE survival_mob_drop_spawn username=compatbot mob=IronGolem position=16.5,65.0,2.5 health=20.0 ai=false\n",
        "compatbot",
    );
    assert!(!missing_state.passed, "{missing_state:?}");
    assert!(missing_state
        .missing_milestones
        .contains(&"server_survival_mob_drop_state"));
}

#[test]
fn survival_redstone_toggle_scenario_tracks_client_and_server_evidence() {
    let client = evaluate_scenario(
        Scenario::SurvivalRedstoneToggle,
        "Detected server protocol version 763\njoin_game\nrender_tick_with_player\nsurvival_redstone_toggle_input_sent control=Lever position=20,64,0 powered_before=false powered_after=true\nsurvival_redstone_toggle_output_update output=RedstoneLamp position=21,64,0 powered=true raw_id=123\nsurvival_redstone_toggle_return_input_sent control=Lever position=20,64,0 powered_before=true powered_after=false\nsurvival_redstone_toggle_return_update output=RedstoneLamp position=21,64,0 powered=false raw_id=122\n",
    );
    assert!(client.passed, "{client:?}");
    assert!(client.missing_milestones.is_empty());

    let missing_return = evaluate_scenario(
        Scenario::SurvivalRedstoneToggle,
        "Detected server protocol version 763\njoin_game\nrender_tick_with_player\nsurvival_redstone_toggle_input_sent control=Lever position=20,64,0 powered_before=false powered_after=true\nsurvival_redstone_toggle_output_update output=RedstoneLamp position=21,64,0 powered=true raw_id=123\n",
    );
    assert!(!missing_return.passed, "{missing_return:?}");
    assert!(missing_return
        .missing_milestones
        .contains(&"survival_redstone_toggle_return_update"));

    let wrong_client_values = evaluate_scenario(
        Scenario::SurvivalRedstoneToggle,
        "Detected server protocol version 763\njoin_game\nrender_tick_with_player\nsurvival_redstone_toggle_input_sent control=Button position=20,64,0 powered_before=false powered_after=true\nsurvival_redstone_toggle_output_update output=RedstoneTorch position=21,64,0 powered=true raw_id=123\nsurvival_redstone_toggle_return_input_sent control=Button position=20,64,0 powered_before=true powered_after=false\nsurvival_redstone_toggle_return_update output=RedstoneTorch position=21,64,0 powered=false raw_id=122\n",
    );
    assert!(!wrong_client_values.passed, "{wrong_client_values:?}");
    assert!(wrong_client_values
        .missing_milestones
        .contains(&"survival_redstone_toggle_input_sent"));
    assert!(wrong_client_values
        .missing_milestones
        .contains(&"survival_redstone_toggle_output_update"));

    let server = evaluate_server_scenario(
        Scenario::SurvivalRedstoneToggle,
        "compatbot joined\nMC-COMPAT-MILESTONE survival_redstone_toggle_input username=compatbot control=Lever position=20,64,0 powered_before=false powered_after=true\nMC-COMPAT-MILESTONE survival_redstone_toggle_powered_on username=compatbot output=RedstoneLamp position=21,64,0 powered=true\nMC-COMPAT-MILESTONE survival_redstone_toggle_powered_off username=compatbot output=RedstoneLamp position=21,64,0 powered=false\nMC-COMPAT-MILESTONE survival_redstone_toggle_state username=compatbot control=Lever output=RedstoneLamp on_seen=true off_seen=true unintended_outputs=false\n",
        "compatbot",
    );
    assert!(server.passed, "{server:?}");

    let missing_state = evaluate_server_scenario(
        Scenario::SurvivalRedstoneToggle,
        "compatbot joined\nMC-COMPAT-MILESTONE survival_redstone_toggle_input username=compatbot control=Lever position=20,64,0 powered_before=false powered_after=true\n",
        "compatbot",
    );
    assert!(!missing_state.passed, "{missing_state:?}");
    assert!(missing_state
        .missing_milestones
        .contains(&"server_survival_redstone_toggle_state"));
}

#[test]
fn survival_world_persistence_scenario_tracks_client_and_server_evidence() {
    let client = evaluate_scenario(
        Scenario::SurvivalWorldPersistenceRestart,
        "Detected server protocol version 763\njoin_game\nrender_tick_with_player\nsurvival_world_persistence_mutation_sent block=Dirt position=24,64,0 slot=36 hand=main sequence=933\nsurvival_world_persistence_pre_restart_update block=Dirt position=24,64,0 raw_id=10\nsurvival_world_persistence_reconnect_sent session=restart\nsurvival_world_persistence_post_restart_update block=Dirt position=24,64,0 raw_id=10\n",
    );
    assert!(client.passed, "{client:?}");
    assert!(client.missing_milestones.is_empty());

    let missing_post = evaluate_scenario(
        Scenario::SurvivalWorldPersistenceRestart,
        "Detected server protocol version 763\njoin_game\nrender_tick_with_player\nsurvival_world_persistence_mutation_sent block=Dirt position=24,64,0 slot=36 hand=main sequence=933\nsurvival_world_persistence_pre_restart_update block=Dirt position=24,64,0 raw_id=10\nsurvival_world_persistence_reconnect_sent session=restart\n",
    );
    assert!(!missing_post.passed, "{missing_post:?}");
    assert!(missing_post
        .missing_milestones
        .contains(&"survival_world_persistence_post_restart_update"));

    let wrong_client_values = evaluate_scenario(
        Scenario::SurvivalWorldPersistenceRestart,
        "Detected server protocol version 763\njoin_game\nrender_tick_with_player\nsurvival_world_persistence_mutation_sent block=Stone position=25,64,0 slot=37 hand=main sequence=934\nsurvival_world_persistence_pre_restart_update block=Stone position=25,64,0 raw_id=10\nsurvival_world_persistence_reconnect_sent session=restart\nsurvival_world_persistence_post_restart_update block=Stone position=25,64,0 raw_id=10\n",
    );
    assert!(!wrong_client_values.passed, "{wrong_client_values:?}");
    assert!(wrong_client_values
        .missing_milestones
        .contains(&"survival_world_persistence_mutation_sent"));

    let server = evaluate_server_scenario(
        Scenario::SurvivalWorldPersistenceRestart,
        "compatbot joined\nMC-COMPAT-MILESTONE survival_world_persistence_mutation username=compatbot block=Dirt position=24,64,0 persisted_before=false persisted_after=true\nMC-COMPAT-MILESTONE survival_world_persistence_clean_shutdown username=compatbot storage=isolated shutdown=graceful\nMC-COMPAT-MILESTONE survival_world_persistence_backend_restart username=compatbot method=controlled_reload storage=isolated restart_confirmed=true\nMC-COMPAT-MILESTONE survival_world_persistence_post_restart_observe username=compatbot block=Dirt position=24,64,0 persisted=true\nMC-COMPAT-MILESTONE survival_world_persistence_state username=compatbot block=Dirt position=24,64,0 pre_mutation=true clean_shutdown=true backend_restart=true post_observed=true dirty_reuse=false\n",
        "compatbot",
    );
    assert!(server.passed, "{server:?}");

    let missing_state = evaluate_server_scenario(
        Scenario::SurvivalWorldPersistenceRestart,
        "compatbot joined\nMC-COMPAT-MILESTONE survival_world_persistence_mutation username=compatbot block=Dirt position=24,64,0 persisted_before=false persisted_after=true\n",
        "compatbot",
    );
    assert!(!missing_state.passed, "{missing_state:?}");
    assert!(missing_state
        .missing_milestones
        .contains(&"server_survival_world_persistence_state"));
}

#[test]
fn survival_block_entity_persistence_scenario_tracks_client_and_server_evidence() {
    let client = evaluate_scenario(
        Scenario::SurvivalBlockEntityPersistenceParity,
        "Detected server protocol version 763\njoin_game\nrender_tick_with_player\nsurvival_block_entity_pre_restart_update kind=Sign position=28,64,0 text=MC|Compat|Sign|Persist source=chunk\nsurvival_block_entity_reconnect_sent session=restart\nsurvival_block_entity_post_restart_update kind=Sign position=28,64,0 text=MC|Compat|Sign|Persist source=chunk\n",
    );
    assert!(client.passed, "{client:?}");
    assert!(client.missing_milestones.is_empty());

    let missing_post = evaluate_scenario(
        Scenario::SurvivalBlockEntityPersistenceParity,
        "Detected server protocol version 763\njoin_game\nrender_tick_with_player\nsurvival_block_entity_pre_restart_update kind=Sign position=28,64,0 text=MC|Compat|Sign|Persist source=chunk\nsurvival_block_entity_reconnect_sent session=restart\n",
    );
    assert!(!missing_post.passed, "{missing_post:?}");
    assert!(missing_post
        .missing_milestones
        .contains(&"survival_block_entity_post_restart_update"));

    let wrong_client_values = evaluate_scenario(
        Scenario::SurvivalBlockEntityPersistenceParity,
        "Detected server protocol version 763\njoin_game\nrender_tick_with_player\nsurvival_block_entity_pre_restart_update kind=Chest position=29,64,0 text=Wrong source=chunk\nsurvival_block_entity_reconnect_sent session=restart\nsurvival_block_entity_post_restart_update kind=Chest position=29,64,0 text=Wrong source=chunk\n",
    );
    assert!(!wrong_client_values.passed, "{wrong_client_values:?}");
    assert!(wrong_client_values
        .missing_milestones
        .contains(&"survival_block_entity_pre_restart_update"));

    let server = evaluate_server_scenario(
        Scenario::SurvivalBlockEntityPersistenceParity,
        "compatbot joined\nMC-COMPAT-MILESTONE survival_block_entity_persistence_mutation username=compatbot kind=Sign position=28,64,0 text=MC|Compat|Sign|Persist persisted_before=false persisted_after=true\nMC-COMPAT-MILESTONE survival_block_entity_persistence_clean_shutdown username=compatbot storage=isolated shutdown=graceful\nMC-COMPAT-MILESTONE survival_block_entity_persistence_backend_restart username=compatbot method=controlled_reload storage=isolated restart_confirmed=true\nMC-COMPAT-MILESTONE survival_block_entity_persistence_post_restart_observe username=compatbot kind=Sign position=28,64,0 text=MC|Compat|Sign|Persist persisted=true\nMC-COMPAT-MILESTONE survival_block_entity_persistence_state username=compatbot kind=Sign position=28,64,0 text=MC|Compat|Sign|Persist pre_mutation=true clean_shutdown=true backend_restart=true post_observed=true dirty_reuse=false\n",
        "compatbot",
    );
    assert!(server.passed, "{server:?}");

    let missing_state = evaluate_server_scenario(
        Scenario::SurvivalBlockEntityPersistenceParity,
        "compatbot joined\nMC-COMPAT-MILESTONE survival_block_entity_persistence_mutation username=compatbot kind=Sign position=28,64,0 text=MC|Compat|Sign|Persist persisted_before=false persisted_after=true\n",
        "compatbot",
    );
    assert!(!missing_state.passed, "{missing_state:?}");
    assert!(missing_state
        .missing_milestones
        .contains(&"server_survival_block_entity_state"));
}

#[test]
fn survival_crash_recovery_scenario_tracks_derived_client_and_server_evidence() {
    let raw_client = "Detected server protocol version 763\njoin_game\nrender_tick_with_player\nsurvival_world_persistence_mutation_sent block=Dirt position=24,64,0 slot=36 hand=main sequence=933\nsurvival_world_persistence_pre_restart_update block=Dirt position=24,64,0 raw_id=10\nsurvival_world_persistence_reconnect_sent session=restart\nsurvival_world_persistence_post_restart_update block=Dirt position=24,64,0 raw_id=10\n";
    let raw_client_result = evaluate_scenario(Scenario::SurvivalCrashRecoveryParity, raw_client);
    assert!(!raw_client_result.passed, "{raw_client_result:?}");
    assert!(raw_client_result
        .missing_milestones
        .contains(&"survival_crash_recovery_mutation_sent"));

    let mut derived_client = raw_client.to_string();
    derived_client.push_str(&derive_survival_crash_recovery_client_milestones(
        raw_client,
    ));
    let client = evaluate_scenario(Scenario::SurvivalCrashRecoveryParity, &derived_client);
    assert!(client.passed, "{client:?}");
    assert!(client.missing_milestones.is_empty());

    let raw_server = "compatbot joined\nMC-COMPAT-MILESTONE survival_world_persistence_mutation username=compatbot block=Dirt position=24,64,0 persisted_before=false persisted_after=true\nMC-COMPAT-MILESTONE survival_crash_recovery_forced_stop username=compatbot method=forced_stop storage=isolated graceful=false\nMC-COMPAT-MILESTONE survival_crash_recovery_backend_restart username=compatbot method=crash_recovery storage=isolated restart_confirmed=true\nMC-COMPAT-MILESTONE survival_world_persistence_post_restart_observe username=compatbot block=Dirt position=24,64,0 persisted=true\nMC-COMPAT-MILESTONE survival_world_persistence_state username=compatbot block=Dirt position=24,64,0 pre_mutation=true clean_shutdown=true backend_restart=true post_observed=true dirty_reuse=false\n";
    let mut derived_server = raw_server.to_string();
    derived_server.push_str(&derive_survival_crash_recovery_server_milestones(
        raw_server,
    ));
    let server = evaluate_server_scenario(
        Scenario::SurvivalCrashRecoveryParity,
        &derived_server,
        "compatbot",
    );
    assert!(server.passed, "{server:?}");

    let missing_forced_stop = raw_server.replace(
        "MC-COMPAT-MILESTONE survival_crash_recovery_forced_stop username=compatbot method=forced_stop storage=isolated graceful=false\n",
        "",
    );
    let mut derived_missing = missing_forced_stop.clone();
    derived_missing.push_str(&derive_survival_crash_recovery_server_milestones(
        &missing_forced_stop,
    ));
    let missing_server = evaluate_server_scenario(
        Scenario::SurvivalCrashRecoveryParity,
        &derived_missing,
        "compatbot",
    );
    assert!(!missing_server.passed, "{missing_server:?}");
    assert!(missing_server
        .missing_milestones
        .contains(&"server_survival_crash_recovery_forced_stop"));
    assert!(missing_server
        .missing_milestones
        .contains(&"server_survival_crash_recovery_state"));
}

#[test]
fn survival_hunger_food_scenario_tracks_client_and_server_evidence() {
    let client = evaluate_scenario(
        Scenario::SurvivalHungerFood,
        "Detected server protocol version 763\njoin_game\nrender_tick_with_player\nsurvival_hunger_food_item_seen slot=36 item=Bread count=1\nsurvival_hunger_food_pre_seen health=20.0 food=15 saturation=0.0\nsurvival_hunger_food_use_sent slot=36 item=Bread count=1 hand=main sequence=810\nsurvival_hunger_food_post_seen health=20.0 food=20 saturation=6.0\nsurvival_hunger_food_inventory_updated slot=36 item=Bread count=0\n",
    );
    assert!(client.passed, "{client:?}");
    assert!(client.missing_milestones.is_empty());

    let missing_post = evaluate_scenario(
        Scenario::SurvivalHungerFood,
        "Detected server protocol version 763\njoin_game\nrender_tick_with_player\nsurvival_hunger_food_item_seen slot=36 item=Bread count=1\nsurvival_hunger_food_pre_seen health=20.0 food=15 saturation=0.0\nsurvival_hunger_food_use_sent slot=36 item=Bread count=1 hand=main sequence=810\n",
    );
    assert!(!missing_post.passed, "{missing_post:?}");
    assert!(missing_post
        .missing_milestones
        .contains(&"survival_hunger_food_post_seen"));

    let wrong_client_values = evaluate_scenario(
        Scenario::SurvivalHungerFood,
        "Detected server protocol version 763\njoin_game\nrender_tick_with_player\nsurvival_hunger_food_item_seen slot=37 item=Apple count=2\nsurvival_hunger_food_pre_seen health=20.0 food=16 saturation=1.0\nsurvival_hunger_food_use_sent slot=37 item=Apple count=2 hand=main sequence=811\nsurvival_hunger_food_post_seen health=19.0 food=20 saturation=4.0\nsurvival_hunger_food_inventory_updated slot=37 item=Apple count=1\n",
    );
    assert!(!wrong_client_values.passed, "{wrong_client_values:?}");
    assert!(wrong_client_values
        .missing_milestones
        .contains(&"survival_hunger_food_item_seen"));
    assert!(wrong_client_values
        .missing_milestones
        .contains(&"survival_hunger_food_pre_seen"));
    assert!(wrong_client_values
        .missing_milestones
        .contains(&"survival_hunger_food_inventory_updated"));

    let server = evaluate_server_scenario(
        Scenario::SurvivalHungerFood,
        "compatbot joined\nMC-COMPAT-MILESTONE survival_hunger_food_pre username=compatbot health=20.0 food=15 saturation=0.0 item=Bread count=1 slot=36\nMC-COMPAT-MILESTONE survival_hunger_food_consume_start username=compatbot item=Bread slot=36 food_before=15 saturation_before=0.0\nMC-COMPAT-MILESTONE survival_hunger_food_consume_finish username=compatbot item=Bread slot=36 food_after=20 saturation_after=6.0\nMC-COMPAT-MILESTONE survival_hunger_food_inventory username=compatbot slot=36 item=Bread count_before=1 count_after=0\nMC-COMPAT-MILESTONE survival_hunger_food_state username=compatbot health=20.0 food_before=15 food_after=20 saturation_before=0.0 saturation_after=6.0 unexpected_damage=false death=false\n",
        "compatbot",
    );
    assert!(server.passed, "{server:?}");

    let missing_state = evaluate_server_scenario(
        Scenario::SurvivalHungerFood,
        "compatbot joined\nMC-COMPAT-MILESTONE survival_hunger_food_pre username=compatbot health=20.0 food=15 saturation=0.0 item=Bread count=1 slot=36\n",
        "compatbot",
    );
    assert!(!missing_state.passed, "{missing_state:?}");
    assert!(missing_state
        .missing_milestones
        .contains(&"server_survival_hunger_food_state"));
}

#[test]
fn survival_hunger_health_cycle_scenario_tracks_client_and_server_evidence() {
    let client = evaluate_scenario(
        Scenario::SurvivalHungerHealthCycle,
        "Detected server protocol version 763\njoin_game\nrender_tick_with_player\nsurvival_hunger_health_item_seen slot=36 item=Bread count=1\nsurvival_hunger_health_pre_seen health=18.0 food=15 saturation=0.0\nsurvival_hunger_health_consume_sent slot=36 item=Bread count=1 hand=main sequence=810\nsurvival_hunger_health_recovery_seen health=20.0 food=20 saturation=6.0\nsurvival_hunger_health_inventory_updated slot=36 item=Bread count=0\n",
    );
    assert!(client.passed, "{client:?}");
    assert!(client.missing_milestones.is_empty());

    let missing_recovery = evaluate_scenario(
        Scenario::SurvivalHungerHealthCycle,
        "Detected server protocol version 763\njoin_game\nrender_tick_with_player\nsurvival_hunger_health_item_seen slot=36 item=Bread count=1\nsurvival_hunger_health_pre_seen health=18.0 food=15 saturation=0.0\nsurvival_hunger_health_consume_sent slot=36 item=Bread count=1 hand=main sequence=810\n",
    );
    assert!(!missing_recovery.passed, "{missing_recovery:?}");
    assert!(missing_recovery
        .missing_milestones
        .contains(&"survival_hunger_health_recovery_seen"));

    let wrong_client_values = evaluate_scenario(
        Scenario::SurvivalHungerHealthCycle,
        "Detected server protocol version 763\njoin_game\nrender_tick_with_player\nsurvival_hunger_health_item_seen slot=36 item=Bread count=1\nsurvival_hunger_health_pre_seen health=20.0 food=15 saturation=0.0\nsurvival_hunger_health_consume_sent slot=36 item=Bread count=1 hand=main sequence=810\nsurvival_hunger_health_recovery_seen health=20.0 food=20 saturation=6.0\nsurvival_hunger_health_inventory_updated slot=36 item=Bread count=0\n",
    );
    assert!(!wrong_client_values.passed, "{wrong_client_values:?}");
    assert!(wrong_client_values
        .missing_milestones
        .contains(&"survival_hunger_health_pre_seen"));

    let server = evaluate_server_scenario(
        Scenario::SurvivalHungerHealthCycle,
        "compatbot joined\nMC-COMPAT-MILESTONE survival_hunger_health_pre username=compatbot health=18.0 food=15 saturation=0.0 item=Bread count=1 slot=36\nMC-COMPAT-MILESTONE survival_hunger_health_consume_start username=compatbot item=Bread slot=36 food_before=15 saturation_before=0.0\nMC-COMPAT-MILESTONE survival_hunger_health_consume_finish username=compatbot item=Bread slot=36 food_after=20 saturation_after=6.0\nMC-COMPAT-MILESTONE survival_hunger_health_inventory username=compatbot slot=36 item=Bread count_before=1 count_after=0\nMC-COMPAT-MILESTONE survival_hunger_health_state username=compatbot pre_health=18.0 post_health=20.0 food_before=15 food_after=20 saturation_before=0.0 saturation_after=6.0 unexpected_damage=false death=false\n",
        "compatbot",
    );
    assert!(server.passed, "{server:?}");

    let missing_state = evaluate_server_scenario(
        Scenario::SurvivalHungerHealthCycle,
        "compatbot joined\nMC-COMPAT-MILESTONE survival_hunger_health_pre username=compatbot health=18.0 food=15 saturation=0.0 item=Bread count=1 slot=36\n",
        "compatbot",
    );
    assert!(!missing_state.passed, "{missing_state:?}");
    assert!(missing_state
        .missing_milestones
        .contains(&"server_survival_hunger_health_state"));
}

#[test]
fn flag_score_repeat_scenario_tracks_missing_and_forbidden_evidence() {
    let pass = evaluate_scenario(
        Scenario::FlagScoreRepeat,
        "Detected server protocol version 763\njoin_game\nrender_tick_with_player\nYou are on team RED!\nYou have the flag!\nYou captured the flag!\nRED: 1\nRED: 2\n",
    );
    assert!(pass.passed, "{pass:?}");
    assert_eq!(pass.missing_milestones, Vec::<&str>::new());
    assert_eq!(pass.forbidden_matches, Vec::<&str>::new());

    let fail = evaluate_scenario(
        Scenario::FlagScoreRepeat,
        "Detected server protocol version 763\njoin_game\nUnexpectedEof\n",
    );
    assert!(!fail.passed, "{fail:?}");
    assert!(fail.missing_milestones.contains(&"render_tick"));
    assert!(fail.missing_milestones.contains(&"score_red_2"));
    assert_eq!(fail.forbidden_matches, vec!["unexpected_eof"]);
}
