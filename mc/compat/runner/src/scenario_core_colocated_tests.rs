#![allow(unused_imports)]

use super::*;
use crate::test_support::*;
use crate::*;
use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::time::Duration;

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
