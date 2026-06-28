#![allow(unused_imports)]

use super::*;
use crate::test_support::*;
use crate::*;
use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::time::Duration;

const DUPLICATE_SMOKE_ALIASES: &[&str] = &["smoke", "smoke"];
const UNKNOWN_ENV_INTENTS: &[&str] = &["client.unknown-env-intent"];
const UNSUPPORTED_HANDLER: &str = "unsupported-handler";
const INVALID_GRAPH_EDGE: ScenarioBehaviorEdge = ("protocol_detected", "missing_graph_event");
const REQUIRED_METADATA_NON_CLAIM: &str = "broad_minecraft_compatibility";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct RepresentativeMetadataCase {
    scenario: Scenario,
    run_strategy: ScenarioRunStrategy,
    env_intent: &'static str,
    edge: ScenarioBehaviorEdge,
    dynamic_projectile_health: bool,
    mcp_control: bool,
}

const REPRESENTATIVE_METADATA_CASES: &[RepresentativeMetadataCase] = &[
    RepresentativeMetadataCase {
        scenario: Scenario::FlagScoreRepeat,
        run_strategy: ScenarioRunStrategy::SingleClient,
        env_intent: "client.flag-probe",
        edge: ("flag_pickup", "flag_capture"),
        dynamic_projectile_health: false,
        mcp_control: false,
    },
    RepresentativeMetadataCase {
        scenario: Scenario::SurvivalChestPersistence,
        run_strategy: ScenarioRunStrategy::ReconnectSequence,
        env_intent: "client.survival-session",
        edge: ("survival_chest_close_sent", "survival_chest_reconnect_sent"),
        dynamic_projectile_health: false,
        mcp_control: false,
    },
    RepresentativeMetadataCase {
        scenario: Scenario::MultiClientLoadScore,
        run_strategy: ScenarioRunStrategy::MultiClient,
        env_intent: "client.flag-probe",
        edge: ("multi_client_count", "flag_pickup"),
        dynamic_projectile_health: false,
        mcp_control: false,
    },
    RepresentativeMetadataCase {
        scenario: Scenario::ProjectileDamageAttribution,
        run_strategy: ScenarioRunStrategy::MultiClient,
        env_intent: "client.projectile-probe",
        edge: ("projectile_swing_sent", "projectile_damage_update"),
        dynamic_projectile_health: true,
        mcp_control: false,
    },
    RepresentativeMetadataCase {
        scenario: Scenario::InventoryInteraction,
        run_strategy: ScenarioRunStrategy::SingleClient,
        env_intent: "client.inventory-probe",
        edge: ("inventory_drop_sent", "inventory_pickup_seen"),
        dynamic_projectile_health: false,
        mcp_control: false,
    },
    RepresentativeMetadataCase {
        scenario: Scenario::SurvivalBreakPlacePickup,
        run_strategy: ScenarioRunStrategy::SingleClient,
        env_intent: "client.survival-probe",
        edge: ("survival_break_sent", "survival_break_update"),
        dynamic_projectile_health: false,
        mcp_control: false,
    },
    RepresentativeMetadataCase {
        scenario: Scenario::CtfScoreLimitWinCondition,
        run_strategy: ScenarioRunStrategy::SingleClient,
        env_intent: "client.ctf-score-limit",
        edge: ("score_red_2", "ctf_score_limit_win_seen"),
        dynamic_projectile_health: false,
        mcp_control: false,
    },
    RepresentativeMetadataCase {
        scenario: Scenario::McpControlledSmoke,
        run_strategy: ScenarioRunStrategy::SingleClient,
        env_intent: "client.mcp-control",
        edge: ("mcp_input_call", "mcp_capture_latest_frame"),
        dynamic_projectile_health: false,
        mcp_control: true,
    },
];

#[test]
fn supported_scenario_usage_lists_all_supported_scenarios() {
    for row in scenario_manifest_generated::SCENARIO_MANIFEST_ROWS {
        assert!(
            SUPPORTED_SCENARIO_USAGE.contains(row.name),
            "usage omits supported scenario {}",
            row.name
        );
    }
}

#[test]
fn generated_scenario_manifest_matches_runner_parser() {
    for row in scenario_manifest_generated::SCENARIO_MANIFEST_ROWS {
        let canonical = parse_scenario(row.name).expect("canonical scenario parses");
        assert_eq!(scenario_name(canonical), row.name);
        for alias in row.aliases {
            let parsed = parse_scenario(alias).expect("alias scenario parses");
            assert_eq!(
                parsed, canonical,
                "alias {alias} parsed away from {}",
                row.name
            );
        }
        assert_eq!(
            scenario_required_milestones(canonical).len(),
            row.client_milestones.len()
        );
        for milestone in row.client_milestones {
            assert!(
                scenario_required_milestones(canonical)
                    .iter()
                    .any(|(name, _)| name == milestone),
                "generated manifest has client milestone {milestone} absent from runner"
            );
        }
        assert_eq!(
            server_required_milestones(canonical).len(),
            row.server_milestones.len()
        );
        for milestone in row.server_milestones {
            assert!(
                server_required_milestones(canonical)
                    .iter()
                    .any(|(name, _)| name == milestone),
                "generated manifest has server milestone {milestone} absent from runner"
            );
        }
        for forbidden in row.forbidden_patterns {
            assert!(
                scenario_forbidden_patterns(canonical)
                    .iter()
                    .any(|(name, _)| name == forbidden),
                "generated manifest has forbidden pattern {forbidden} absent from runner"
            );
        }
    }
}

#[test]
fn static_scenario_specs_validate_all_supported_behavior() {
    validate_static_scenario_specs(SCENARIO_SPECS).expect("static specs validate");
    assert_eq!(SCENARIO_SPECS.len(), ALL_SCENARIOS.len());

    for spec in SCENARIO_SPECS {
        assert_eq!(parse_scenario(spec.canonical_name), Ok(spec.scenario));
        assert_eq!(scenario_name(spec.scenario), spec.canonical_name);
        assert_eq!(
            scenario_required_milestones(spec.scenario),
            spec.client_milestones
        );
        assert_eq!(
            server_required_milestones(spec.scenario),
            spec.server_milestones
        );
        assert_eq!(
            scenario_forbidden_patterns(spec.scenario),
            spec.forbidden_patterns
        );
        for alias in spec.aliases {
            assert_eq!(parse_scenario(alias), Ok(spec.scenario));
        }
    }
}

#[test]
fn scenario_behavior_metadata_validates_representative_fixtures() {
    for case in REPRESENTATIVE_METADATA_CASES {
        let spec = scenario_spec(case.scenario);
        let metadata = scenario_behavior_metadata(case.scenario);
        validate_scenario_behavior_metadata(spec, &metadata)
            .expect("representative metadata validates");
        assert_eq!(metadata.run_strategy, Some(case.run_strategy));
        assert!(
            metadata.env_intents.contains(&case.env_intent),
            "{} missing env intent {}: {:?}",
            spec.canonical_name,
            case.env_intent,
            metadata.env_intents
        );
        assert!(
            metadata.typed_event_edges.contains(&case.edge),
            "{} missing typed edge {:?}: {:?}",
            spec.canonical_name,
            case.edge,
            metadata.typed_event_edges
        );
        assert!(metadata.evidence_selectors.typed_event_pass_fail);
        assert_eq!(
            metadata.evidence_selectors.dynamic_projectile_health,
            case.dynamic_projectile_health
        );
        assert_eq!(metadata.evidence_selectors.mcp_control, case.mcp_control);
        assert!(metadata.non_claims.contains(&REQUIRED_METADATA_NON_CLAIM));
        assert!(!metadata.handler.is_empty());
    }
}

#[test]
fn scenario_behavior_metadata_fails_closed_for_malformed_rows() {
    let smoke_spec = scenario_spec(Scenario::Smoke);
    let mut missing_run_strategy = scenario_behavior_metadata(Scenario::Smoke);
    missing_run_strategy.run_strategy = None;
    let err = validate_scenario_behavior_metadata(smoke_spec, &missing_run_strategy).unwrap_err();
    assert!(err.contains("missing run strategy"), "{err}");

    let mut unknown_env = scenario_behavior_metadata(Scenario::Smoke);
    unknown_env.env_intents = UNKNOWN_ENV_INTENTS;
    let err = validate_scenario_behavior_metadata(smoke_spec, &unknown_env).unwrap_err();
    assert!(err.contains("unknown env intent"), "{err}");

    let mut invalid_graph = scenario_behavior_metadata(Scenario::Smoke);
    invalid_graph.typed_event_edges = vec![INVALID_GRAPH_EDGE];
    let err = validate_scenario_behavior_metadata(smoke_spec, &invalid_graph).unwrap_err();
    assert!(err.contains("invalid graph edge"), "{err}");

    let mut unsupported_handler = scenario_behavior_metadata(Scenario::Smoke);
    unsupported_handler.handler = UNSUPPORTED_HANDLER;
    let err = validate_scenario_behavior_metadata(smoke_spec, &unsupported_handler).unwrap_err();
    assert!(err.contains("unsupported handler"), "{err}");

    let smoke_index = scenario_index(Scenario::Smoke);
    let mut duplicate_alias = SCENARIO_SPECS.to_vec();
    duplicate_alias[smoke_index].aliases = DUPLICATE_SMOKE_ALIASES;
    let err = validate_static_scenario_specs(&duplicate_alias).unwrap_err();
    assert!(err.contains("duplicate alias"), "{err}");
}

#[test]
fn static_scenario_specs_fail_closed_for_invalid_definitions() {
    assert!(parse_scenario("missing-scenario")
        .unwrap_err()
        .contains("unknown scenario: missing-scenario"));

    const COMPAT_ALIAS_MISSING_LEGACY: &[&str] = &["valence-compat-bot-probe"];
    const EMPTY_MILESTONES: &[ScenarioMilestone] = &[];
    const EMPTY_FORBIDDEN_PATTERNS: &[ScenarioMilestone] = &[];

    let compat_index = scenario_index(Scenario::CompatBotProbe);
    let projectile_index = scenario_index(Scenario::ProjectileDamageAttribution);
    let smoke_index = scenario_index(Scenario::Smoke);

    let mut missing_alias = SCENARIO_SPECS.to_vec();
    missing_alias[compat_index].aliases = COMPAT_ALIAS_MISSING_LEGACY;
    let err = validate_static_scenario_specs(&missing_alias).unwrap_err();
    assert!(err.contains("aliases drift"), "{err}");

    let mut duplicated_name = SCENARIO_SPECS.to_vec();
    duplicated_name[compat_index].canonical_name = "smoke";
    let err = validate_static_scenario_specs(&duplicated_name).unwrap_err();
    assert!(err.contains("duplicated canonical name smoke"), "{err}");

    let mut missing_milestones = SCENARIO_SPECS.to_vec();
    missing_milestones[smoke_index].client_milestones = EMPTY_MILESTONES;
    let err = validate_static_scenario_specs(&missing_milestones).unwrap_err();
    assert!(err.contains("missing client milestones"), "{err}");

    let mut missing_forbidden = SCENARIO_SPECS.to_vec();
    missing_forbidden[smoke_index].forbidden_patterns = EMPTY_FORBIDDEN_PATTERNS;
    let err = validate_static_scenario_specs(&missing_forbidden).unwrap_err();
    assert!(err.contains("missing forbidden patterns"), "{err}");

    let mut missing_hook = SCENARIO_SPECS.to_vec();
    missing_hook[projectile_index].behavior = ScenarioBehaviorKind::Default;
    let err = validate_static_scenario_specs(&missing_hook).unwrap_err();
    assert!(err.contains("missing projectile damage hook"), "{err}");
}

fn scenario_index(scenario: Scenario) -> usize {
    SCENARIO_SPECS
        .iter()
        .position(|spec| spec.scenario == scenario)
        .expect("scenario index exists")
}
