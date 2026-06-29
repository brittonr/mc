use super::*;
use std::collections::BTreeMap;
use std::ffi::OsStr;
use std::path::Path;
use std::process::Command;

const CLIENT_BASELINE_SOURCE: &str = "client";
const VALENCE_BASELINE_SOURCE: &str = "valence";
const PAPER_BASELINE_SOURCE: &str = "paper";
const TEST_SESSION_SOURCE: &str = "test-session-env";
const PAPER_ENV_ARG_FLAG: &str = "-e";
const PAPER_EULA_ARG: &str = "EULA=TRUE";
const PAPER_SURVIVAL_CHEST_ARG: &str = "MC_COMPAT_SURVIVAL_CHEST_FIXTURE=1";

#[test]
fn env_patch_outputs_match_captured_representative_scenario_families() {
    let client_samples = [
        (
            "inventory",
            Scenario::InventoryInteraction,
            FIRST_CLIENT_INDEX,
            map(&[
                ("MC_COMPAT_ACTIVE_PROBE", "1"),
                ("MC_COMPAT_INVENTORY_PROBE", "1"),
                ("MC_COMPAT_TEAM_PROBE", "1"),
                ("MC_COMPAT_TEAM_PROBE_TEAM", "red"),
            ]),
        ),
        (
            "survival",
            Scenario::SurvivalChestPersistence,
            SECOND_CLIENT_INDEX,
            map(&[
                ("MC_COMPAT_SURVIVAL_CHEST_PROBE", "1"),
                ("MC_COMPAT_SURVIVAL_CHEST_SESSION", "2"),
            ]),
        ),
        (
            "combat",
            Scenario::CombatDamage,
            FIRST_CLIENT_INDEX,
            map(&[
                ("MC_COMPAT_ACTIVE_PROBE", "1"),
                ("MC_COMPAT_COMBAT_PROBE", "1"),
                ("MC_COMPAT_COMBAT_PROBE_ROLE", "attacker"),
                ("MC_COMPAT_COMBAT_TARGET_USERNAME", "compatbotb"),
                ("MC_COMPAT_TEAM_PROBE", "1"),
                ("MC_COMPAT_TEAM_PROBE_TEAM", "red"),
            ]),
        ),
        (
            "projectile",
            Scenario::ProjectileHit,
            FIRST_CLIENT_INDEX,
            map(&[
                ("MC_COMPAT_ACTIVE_PROBE", "1"),
                ("MC_COMPAT_COMBAT_PROBE", "1"),
                ("MC_COMPAT_COMBAT_PROBE_ROLE", "attacker"),
                ("MC_COMPAT_COMBAT_TARGET_USERNAME", "compatbotb"),
                ("MC_COMPAT_PROJECTILE_PROBE", "1"),
                ("MC_COMPAT_TEAM_PROBE", "1"),
                ("MC_COMPAT_TEAM_PROBE_TEAM", "red"),
            ]),
        ),
        (
            "ctf",
            Scenario::FlagScoreRepeat,
            FIRST_CLIENT_INDEX,
            map(&[
                ("MC_COMPAT_ACTIVE_PROBE", "1"),
                ("MC_COMPAT_FLAG_PROBE", "1"),
                ("MC_COMPAT_FLAG_PROBE_REPEAT", "2"),
                ("MC_COMPAT_FLAG_PROBE_TEAM", "red"),
                ("MC_COMPAT_TEAM_PROBE", "1"),
                ("MC_COMPAT_TEAM_PROBE_TEAM", "red"),
            ]),
        ),
        (
            "reconnect",
            Scenario::ReconnectFlagState,
            FIRST_CLIENT_INDEX,
            map(&[
                ("MC_COMPAT_ACTIVE_PROBE", "1"),
                ("MC_COMPAT_FLAG_PROBE", "1"),
                ("MC_COMPAT_FLAG_PROBE_PICKUP_ONLY", "1"),
                ("MC_COMPAT_FLAG_PROBE_REPEAT", "1"),
                ("MC_COMPAT_FLAG_PROBE_TEAM", "red"),
                ("MC_COMPAT_TEAM_PROBE", "1"),
                ("MC_COMPAT_TEAM_PROBE_TEAM", "red"),
            ]),
        ),
        (
            "mcp",
            Scenario::McpControlledSmoke,
            FIRST_CLIENT_INDEX,
            map(&[("MC_COMPAT_MCP_CONTROLLED_SMOKE", "1")]),
        ),
    ];
    for (family, scenario, client_index, expected) in client_samples {
        let patch = scenario_behavior(scenario)
            .client_probe_env_patch(client_index, ServerBackend::Valence)
            .expect("client env patch derives");
        println!(
            "env_patch source={CLIENT_BASELINE_SOURCE} family={family} scenario={} client_index={client_index} env={:?}",
            scenario_name(scenario),
            patch.as_map()
        );
        assert_eq!(patch.as_map(), expected, "{family} client env changed");
        assert_sources(&patch, ENV_SOURCE_CLIENT_SCENARIO);
    }

    let valence_samples = [
        (
            "inventory",
            Scenario::InventoryStackSplitMerge,
            map(&[("MC_COMPAT_INVENTORY_STACK_SPLIT_MERGE_PROBE", "1")]),
        ),
        (
            "survival",
            Scenario::SurvivalChestPersistence,
            map(&[("MC_COMPAT_SURVIVAL_CHEST_FIXTURE", "1")]),
        ),
        (
            "combat",
            Scenario::VanillaCombatReferenceParity,
            map(&[("MC_COMPAT_VANILLA_COMBAT_REFERENCE_PROBE", "1")]),
        ),
        (
            "projectile",
            Scenario::ProjectileHit,
            map(&[("MC_COMPAT_PROJECTILE_PROBE", "1")]),
        ),
        (
            "ctf",
            Scenario::CtfInvalidReturnDrop,
            map(&[("MC_COMPAT_CTF_INVALID_RETURN_DROP_PROBE", "1")]),
        ),
        ("mcp", Scenario::McpControlledSmoke, map(&[])),
    ];
    for (family, scenario, expected) in valence_samples {
        let cfg = test_support::test_config(&["--scenario", scenario_name(scenario)], &[])
            .expect("Valence scenario config parses");
        let patch = scenario_behavior(scenario)
            .valence_server_env_patch(&cfg)
            .expect("Valence env patch derives");
        println!(
            "env_patch source={VALENCE_BASELINE_SOURCE} family={family} scenario={} env={:?}",
            scenario_name(scenario),
            patch.as_map()
        );
        assert_eq!(patch.as_map(), expected, "{family} Valence env changed");
        assert_sources(&patch, ENV_SOURCE_VALENCE_SCENARIO);
    }

    let paper_survival = paper_scenario_patch(Scenario::SurvivalChestPersistence);
    let paper_combat = paper_scenario_patch(Scenario::VanillaCombatReferenceParity);
    println!(
        "env_patch source={PAPER_BASELINE_SOURCE} family=survival scenario=survival-chest-persistence env={:?}",
        paper_survival.as_map()
    );
    println!(
        "env_patch source={PAPER_BASELINE_SOURCE} family=combat scenario=vanilla-combat-reference-parity env={:?}",
        paper_combat.as_map()
    );
    assert_eq!(
        paper_survival.as_map(),
        map(&[("MC_COMPAT_SURVIVAL_CHEST_FIXTURE", "1")])
    );
    assert_eq!(
        paper_combat.as_map(),
        map(&[("MC_COMPAT_VANILLA_COMBAT_REFERENCE_PROBE", "1")])
    );
    assert_sources(&paper_survival, ENV_SOURCE_PAPER_SCENARIO);
    assert_sources(&paper_combat, ENV_SOURCE_PAPER_SCENARIO);
}

#[test]
fn env_patch_shell_applies_command_and_paper_envs() {
    let mut client_cmd = Command::new("client-env-shell");
    apply_build_env(&mut client_cmd, Path::new("target/client-shell"))
        .expect("build env shell applies");
    apply_headless_env(&mut client_cmd).expect("headless env shell applies");
    let client_env = command_env_snapshot(&client_cmd);

    assert_eq!(
        client_env.get("CARGO_TARGET_DIR"),
        Some(&Some("target/client-shell".to_string()))
    );
    assert_eq!(client_env.get("WAYLAND_DISPLAY"), Some(&None));
    assert_eq!(
        client_env.get("WINIT_UNIX_BACKEND"),
        Some(&Some("x11".to_string()))
    );

    let cfg = test_support::test_config(
        &[
            "--server-backend",
            "paper",
            "--scenario",
            "survival-chest-persistence",
        ],
        &[],
    )
    .expect("Paper config parses");
    let base_patch = paper_base_env_patch(&cfg).expect("base Paper patch derives");
    let scenario_patch = scenario_behavior(cfg.scenario)
        .paper_server_env_patch(&cfg)
        .expect("scenario Paper patch derives");
    let mut paper_cmd = Command::new("paper-env-shell");
    apply_env_patch_to_paper_args(&mut paper_cmd, &base_patch).expect("base Paper env applies");
    apply_env_patch_to_paper_args(&mut paper_cmd, &scenario_patch)
        .expect("scenario Paper env applies");
    let args = command_args(&paper_cmd);

    assert!(args.contains(&PAPER_ENV_ARG_FLAG.to_string()));
    assert!(args.contains(&PAPER_EULA_ARG.to_string()));
    assert!(args.contains(&PAPER_SURVIVAL_CHEST_ARG.to_string()));
}

#[test]
fn env_patch_negative_paths_fail_closed_for_sessions_and_backends() {
    let missing_session = required_session_env_value(TEST_SESSION_SOURCE, None)
        .expect_err("missing session fails closed");
    assert!(missing_session.contains("missing required session value"));
    assert!(missing_session.contains(TEST_SESSION_SOURCE));

    let headless_patch = headless_env_patch().expect("headless patch derives");
    let mut paper_cmd = Command::new("paper-incompatible-env");
    let incompatible = apply_env_patch_to_paper_args(&mut paper_cmd, &headless_patch)
        .expect_err("Paper rejects removal-based patches");
    assert!(incompatible.contains("cannot remove key"), "{incompatible}");
    assert!(incompatible.contains("WAYLAND"), "{incompatible}");
}

fn paper_scenario_patch(scenario: Scenario) -> EnvPatch {
    let cfg = test_support::test_config(
        &[
            "--server-backend",
            "paper",
            "--scenario",
            scenario_name(scenario),
        ],
        &[],
    )
    .expect("Paper scenario config parses");
    scenario_behavior(scenario)
        .paper_server_env_patch(&cfg)
        .expect("Paper scenario env patch derives")
}

fn map(entries: &[(&str, &str)]) -> BTreeMap<String, String> {
    entries
        .iter()
        .map(|(key, value)| ((*key).to_string(), (*value).to_string()))
        .collect()
}

fn assert_sources(patch: &EnvPatch, source: &str) {
    for entry in patch.entries() {
        assert_eq!(entry.source, source, "entry source is explicit");
    }
    for removal in patch.removals() {
        assert_eq!(removal.source, source, "removal source is explicit");
    }
}

fn command_env_snapshot(cmd: &Command) -> BTreeMap<String, Option<String>> {
    cmd.get_envs()
        .map(|(key, value)| (os_str_to_string(key), value.map(os_str_to_string)))
        .collect()
}

fn command_args(cmd: &Command) -> Vec<String> {
    cmd.get_args().map(os_str_to_string).collect()
}

fn os_str_to_string(value: &OsStr) -> String {
    value.to_string_lossy().into_owned()
}
