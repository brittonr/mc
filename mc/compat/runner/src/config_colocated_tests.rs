#![allow(unused_imports)]

use super::*;
use crate::test_support::*;
use crate::*;
use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::time::Duration;

#[test]
fn defaults_to_valence_protocol_and_port() {
    let cfg = test_config(&[], &[]).expect("default config parses");

    assert_eq!(cfg.root, PathBuf::from("/workspace/mc"));
    assert_eq!(
        cfg.client_dir,
        PathBuf::from("/workspace/mc/clients/stevenarella")
    );
    assert_eq!(
        cfg.valence_repo,
        PathBuf::from("/workspace/mc/servers/valence")
    );
    assert_eq!(cfg.server_backend, ServerBackend::Valence);
    assert_eq!(cfg.server_protocol, DEFAULT_SERVER_PROTOCOL);
    assert_eq!(cfg.server_port, VALENCE_DEFAULT_SERVER_PORT);
    assert_eq!(cfg.valence_rev, DEFAULT_VALENCE_REV);
    assert_eq!(cfg.mode, Mode::DryRun);
}

#[test]
fn cli_overrides_backend_client_dir_valence_repo_and_revision() {
    let cfg = test_config(
        &[
            "--run",
            "--server-backend",
            "paper",
            "--client-dir",
            "/tmp/editable-stevenarella",
            "--receipt=/tmp/mc-smoke.json",
            "--valence-repo",
            "/tmp/editable-valence",
            "--valence-rev=local-debug-rev",
        ],
        &[],
    )
    .expect("cli override config parses");

    assert_eq!(cfg.mode, Mode::Run);
    assert_eq!(cfg.server_backend, ServerBackend::Paper);
    assert_eq!(cfg.server_port, PAPER_DEFAULT_SERVER_PORT);
    assert_eq!(cfg.client_dir, PathBuf::from("/tmp/editable-stevenarella"));
    assert_eq!(cfg.receipt_path, Some(PathBuf::from("/tmp/mc-smoke.json")));
    assert_eq!(cfg.valence_repo, PathBuf::from("/tmp/editable-valence"));
    assert_eq!(cfg.valence_rev, "local-debug-rev");
}

#[test]
fn failure_bundle_path_parses_from_env_and_cli_override() {
    let cfg = test_config(
        &["--failure-bundle", "docs/evidence/cli-failure-bundle.json"],
        &[(
            "MC_COMPAT_FAILURE_BUNDLE",
            "docs/evidence/env-failure-bundle.json",
        )],
    )
    .expect("failure bundle config parses");

    assert_eq!(
        cfg.failure_bundle_path,
        Some(PathBuf::from("docs/evidence/cli-failure-bundle.json"))
    );
}

#[test]
fn run_matrix_config_sets_receipt_dir_and_backend_defaults() {
    let cfg = test_config(
        &[
            "--run-matrix",
            "--receipt-dir",
            "/tmp/matrix-receipts",
            "--dry-run",
            "--client-dir",
            "/tmp/stevenarella",
        ],
        &[],
    )
    .expect("matrix config parses");

    assert_eq!(cfg.mode, Mode::RunMatrix);
    assert!(cfg.matrix_dry_run);
    assert_eq!(cfg.receipt_dir, Some(PathBuf::from("/tmp/matrix-receipts")));

    let paper = matrix_backend_config(&cfg, ServerBackend::Paper, PathBuf::from("paper.json"));
    let valence =
        matrix_backend_config(&cfg, ServerBackend::Valence, PathBuf::from("valence.json"));
    assert_eq!(paper.mode, Mode::DryRun);
    assert_eq!(paper.server_port, PAPER_DEFAULT_SERVER_PORT);
    assert_eq!(paper.receipt_path, Some(PathBuf::from("paper.json")));
    assert_eq!(valence.mode, Mode::DryRun);
    assert_eq!(valence.server_port, VALENCE_DEFAULT_SERVER_PORT);
    assert_eq!(valence.receipt_path, Some(PathBuf::from("valence.json")));
}

#[test]
fn run_matrix_rejects_single_receipt_path() {
    let err = test_config(&["--run-matrix", "--receipt", "/tmp/one.json"], &[]).unwrap_err();
    assert!(
        err.contains("--run-matrix writes backend receipts"),
        "{err}"
    );
}

#[test]
fn status_and_cleanup_modes_parse_without_server_probe_mode() {
    let status = test_config(&["--status"], &[]).expect("status config parses");
    assert_eq!(status.mode, Mode::HarnessStatus);
    assert!(!status.cleanup_apply);

    let cleanup_dry =
        test_config(&["--cleanup", "--dry-run"], &[]).expect("cleanup dry-run config parses");
    assert_eq!(cleanup_dry.mode, Mode::Cleanup);
    assert!(!cleanup_dry.cleanup_apply);

    let cleanup_apply =
        test_config(&["--cleanup", "--apply"], &[]).expect("cleanup apply config parses");
    assert_eq!(cleanup_apply.mode, Mode::Cleanup);
    assert!(cleanup_apply.cleanup_apply);
}

#[test]
fn nickel_exported_json_config_sets_defaults_and_allows_env_cli_precedence() {
    let config_json = r#"{
      "client_dir": "/config/stevenarella",
      "valence_repo": "/config/valence",
      "valence_rev": "config-rev",
      "server_backend": "paper",
      "server_protocol": 758,
      "server_port": 25566,
      "client_timeout_secs": 9,
      "client_success_patterns": ["Detected server protocol version", "Dimension type:"],
      "receipt_path": "/config/receipt.json"
    }"#;
    let mut cfg = Config::defaults(PathBuf::from("/workspace/mc"))
        .expect("default config resolves source layout");

    let server_port_was_set = apply_config_json(&mut cfg, config_json).expect("config applies");

    assert!(server_port_was_set);
    assert_eq!(cfg.client_dir, PathBuf::from("/config/stevenarella"));
    assert_eq!(cfg.valence_repo, PathBuf::from("/config/valence"));
    assert_eq!(cfg.valence_rev, "config-rev");
    assert_eq!(cfg.server_backend, ServerBackend::Paper);
    assert_eq!(cfg.server_port, 25566);
    assert_eq!(cfg.client_timeout, Duration::from_secs(9));
    assert_eq!(
        cfg.receipt_path,
        Some(PathBuf::from("/config/receipt.json"))
    );
    assert_eq!(
        cfg.client_success_needles,
        vec![
            "Detected server protocol version".to_string(),
            "Dimension type:".to_string()
        ]
    );

    let cfg = test_config(
        &[
            "--config",
            "/tmp/mc-compat-config.json",
            "--server-backend",
            "valence",
        ],
        &[("MC_COMPAT_CONFIG", "/tmp/mc-compat-config.json")],
    );
    assert!(
        cfg.unwrap_err()
            .contains("read config /tmp/mc-compat-config.json"),
        "missing config path should produce actionable read error"
    );
}

#[test]
fn restricted_steel_config_sets_runtime_defaults_and_allows_env_cli_precedence() {
    let path =
        std::env::temp_dir().join(format!("mc-compat-steel-config-{}.scm", std::process::id()));
    fs::write(
        &path,
        r#"
(define config-version 1)
(define sandbox-profile "mc-compat/pure-v1")
(define server-backend "paper")
(define server-version "1.20.1")
(define server-protocol 763)
(define server-port 25566)
(define valence-rev "main")
(define valence-example "ctf")
(define valence-worktree "/tmp/valence-compat-763")
(define valence-target-dir "/tmp/valence-compat-763-target")
(define valence-log "/tmp/mc-compat-valence.log")
(define valence-pid-file "/tmp/mc-compat-valence.pid")
(define client-username "compatbot")
(define client-timeout-secs 77)
(define client-success-patterns (list "Detected server protocol version" "Dimension type:"))
(define receipt-dir "target/mc-compat-steel")
(define scenario "projectile-damage-attribution")
(define arrow-base-damage 3.0)
(define arrow-velocity-multiplier 1.0)
(define arrow-max-damage 10.0)
(define (arrow-damage ctx)
  (damage-linear ctx arrow-base-damage arrow-velocity-multiplier arrow-max-damage))
"#,
    )
    .expect("write Steel config fixture");

    let cfg = test_config(
        &[
            "--steel-config",
            path.to_str().expect("utf8 path"),
            "--server-backend",
            "valence",
        ],
        &[],
    )
    .expect("Steel config parses");

    assert_eq!(cfg.steel_config_path, Some(path.clone()));
    assert_eq!(cfg.server_backend, ServerBackend::Valence);
    assert_eq!(cfg.server_version, "1.20.1");
    assert_eq!(cfg.server_port, 25566);
    assert_eq!(cfg.server_protocol, 763);
    assert_eq!(cfg.valence_rev, "main");
    assert_eq!(cfg.valence_example, "ctf");
    assert_eq!(
        cfg.valence_worktree,
        PathBuf::from("/tmp/valence-compat-763")
    );
    assert_eq!(
        cfg.valence_target_dir,
        PathBuf::from("/tmp/valence-compat-763-target")
    );
    assert_eq!(cfg.valence_log, PathBuf::from("/tmp/mc-compat-valence.log"));
    assert_eq!(
        cfg.valence_pid_file,
        PathBuf::from("/tmp/mc-compat-valence.pid")
    );
    assert_eq!(cfg.client_username, "compatbot");
    assert_eq!(cfg.client_timeout, Duration::from_secs(77));
    assert_eq!(
        cfg.receipt_dir,
        Some(PathBuf::from("target/mc-compat-steel"))
    );
    assert_eq!(cfg.scenario, Scenario::ProjectileDamageAttribution);
    assert_eq!(
        cfg.client_success_needles,
        vec![
            "Detected server protocol version".to_string(),
            "Dimension type:".to_string()
        ]
    );
    let _ = fs::remove_file(path);
}

#[test]
fn restricted_steel_config_rejects_forbidden_capabilities() {
    let path = std::env::temp_dir().join(format!(
        "mc-compat-bad-steel-config-{}.scm",
        std::process::id()
    ));
    fs::write(
        &path,
        r#"
(define config-version 1)
(define sandbox-profile "mc-compat/pure-v1")
(define server-backend "valence")
(define server-protocol 763)
(define server-port 25565)
(define client-timeout-secs 20)
(define client-success-patterns (list "Detected server protocol version"))
(define scenario "smoke")
(define arrow-base-damage 3.0)
(define arrow-velocity-multiplier 1.0)
(define arrow-max-damage 10.0)
(define (arrow-damage ctx)
  (damage-linear ctx arrow-base-damage arrow-velocity-multiplier arrow-max-damage))
(open-input-file "/etc/passwd")
"#,
    )
    .expect("write bad Steel config fixture");

    let err = test_config(&["--steel-config", path.to_str().expect("utf8 path")], &[]).unwrap_err();
    assert!(err.contains("forbidden Steel capability"), "{err}");
    let _ = fs::remove_file(path);
}

#[test]
fn env_overrides_are_parsed_without_global_environment_mutation() {
    let cfg = test_config(
        &["--server-backend=paper"],
        &[
            ("MC_COMPAT_ROOT", "/repo/mc"),
            ("CLIENT_TIMEOUT", "8"),
            (
                "CLIENT_SUCCESS_PATTERN",
                "Detected server protocol version|Dimension type:",
            ),
            ("SERVER_PORT", "24444"),
            ("SMOKE_RECEIPT", "/repo/receipts/smoke.json"),
            ("CLIENT_DIR", "/repo/stevenarella-edit"),
            ("VALENCE_REPO", "/repo/valence-edit"),
            ("VALENCE_REV", "debug-rev"),
            ("PAPER_PLUGIN_JAR", "/repo/fixtures/paper-survival.jar"),
        ],
    )
    .expect("env override config parses");

    assert_eq!(cfg.root, PathBuf::from("/repo/mc"));
    assert_eq!(cfg.client_dir, PathBuf::from("/repo/stevenarella-edit"));
    assert_eq!(cfg.server_backend, ServerBackend::Paper);
    assert_eq!(cfg.server_port, 24444);
    assert_eq!(
        cfg.receipt_path,
        Some(PathBuf::from("/repo/receipts/smoke.json"))
    );
    assert_eq!(cfg.client_timeout, Duration::from_secs(8));
    assert_eq!(cfg.valence_repo, PathBuf::from("/repo/valence-edit"));
    assert_eq!(cfg.valence_rev, "debug-rev");
    assert_eq!(
        cfg.paper_plugin_jar,
        Some(PathBuf::from("/repo/fixtures/paper-survival.jar"))
    );
    assert_eq!(
        cfg.client_success_needles,
        vec![
            "Detected server protocol version".to_string(),
            "Dimension type:".to_string()
        ]
    );
}

#[test]
fn invalid_backend_is_rejected() {
    let err = test_config(&["--server-backend", "spigot"], &[]).unwrap_err();
    assert!(err.contains("unknown server backend: spigot"), "{err}");
}

#[test]
fn scenario_cli_and_env_parse() {
    let cli = test_config(&["--scenario", "flag-score-repeat"], &[]).expect("scenario CLI parses");
    assert_eq!(cli.scenario, Scenario::FlagScoreRepeat);

    let env = test_config(&[], &[("MC_COMPAT_SCENARIO", "flag-score-repeat")])
        .expect("scenario env parses");
    assert_eq!(env.scenario, Scenario::FlagScoreRepeat);

    let compat = test_config(&["--scenario", "valence-compat-bot-probe"], &[])
        .expect("compat-bot scenario parses");
    assert_eq!(compat.scenario, Scenario::CompatBotProbe);

    let compat_alias =
        test_config(&["--scenario", "compat-bot-probe"], &[]).expect("compat-bot alias parses");
    assert_eq!(compat_alias.scenario, Scenario::CompatBotProbe);

    let reconnect = test_config(&["--scenario", "reconnect-flag-score"], &[])
        .expect("reconnect scenario parses");
    assert_eq!(reconnect.scenario, Scenario::ReconnectFlagScore);

    let reconnect_state = test_config(&["--scenario", "reconnect-flag-state"], &[])
        .expect("reconnect flag-state scenario parses");
    assert_eq!(reconnect_state.scenario, Scenario::ReconnectFlagState);

    let blue = test_config(&["--scenario", "blue-flag-score"], &[]).expect("blue scenario parses");
    assert_eq!(blue.scenario, Scenario::BlueFlagScore);

    let multi = test_config(&["--scenario", "multi-client-load-score"], &[])
        .expect("multi-client scenario parses");
    assert_eq!(multi.scenario, Scenario::MultiClientLoadScore);

    let inventory = test_config(&["--scenario", "inventory-interaction"], &[])
        .expect("inventory scenario parses");
    assert_eq!(inventory.scenario, Scenario::InventoryInteraction);

    let inventory_stack = test_config(&["--scenario", "inventory-stack-split-merge"], &[])
        .expect("inventory stack split/merge scenario parses");
    assert_eq!(inventory_stack.scenario, Scenario::InventoryStackSplitMerge);

    let inventory_drag = test_config(&["--scenario", "inventory-drag-transactions"], &[])
        .expect("inventory drag transactions scenario parses");
    assert_eq!(inventory_drag.scenario, Scenario::InventoryDragTransactions);

    let survival = test_config(&["--scenario", "survival-break-place-pickup"], &[])
        .expect("survival scenario parses");
    assert_eq!(survival.scenario, Scenario::SurvivalBreakPlacePickup);

    let chest = test_config(&["--scenario", "survival-chest-persistence"], &[])
        .expect("survival chest scenario parses");
    assert_eq!(chest.scenario, Scenario::SurvivalChestPersistence);

    let crafting = test_config(&["--scenario", "survival-crafting-table"], &[])
        .expect("survival crafting-table scenario parses");
    assert_eq!(crafting.scenario, Scenario::SurvivalCraftingTable);

    let crafting_breadth = test_config(&["--scenario", "survival-crafting-recipe-breadth"], &[])
        .expect("survival crafting recipe breadth scenario parses");
    assert_eq!(
        crafting_breadth.scenario,
        Scenario::SurvivalCraftingRecipeBreadth
    );

    let furnace = test_config(&["--scenario", "survival-furnace-persistence"], &[])
        .expect("survival furnace scenario parses");
    assert_eq!(furnace.scenario, Scenario::SurvivalFurnacePersistence);

    let furnace_breadth = test_config(&["--scenario", "survival-furnace-smelting-breadth"], &[])
        .expect("survival furnace smelting breadth scenario parses");
    assert_eq!(
        furnace_breadth.scenario,
        Scenario::SurvivalFurnaceSmeltingBreadth
    );

    let hunger_food = test_config(&["--scenario", "survival-hunger-food"], &[])
        .expect("survival hunger-food scenario parses");
    assert_eq!(hunger_food.scenario, Scenario::SurvivalHungerFood);

    let hunger_health_cycle = test_config(&["--scenario", "survival-hunger-health-cycle"], &[])
        .expect("survival hunger-health-cycle scenario parses");
    assert_eq!(
        hunger_health_cycle.scenario,
        Scenario::SurvivalHungerHealthCycle
    );

    let mob_drop = test_config(&["--scenario", "survival-mob-drop"], &[])
        .expect("survival mob-drop scenario parses");
    assert_eq!(mob_drop.scenario, Scenario::SurvivalMobDrop);

    let redstone = test_config(&["--scenario", "survival-redstone-toggle"], &[])
        .expect("survival redstone toggle scenario parses");
    assert_eq!(redstone.scenario, Scenario::SurvivalRedstoneToggle);

    let biome_dimension = test_config(&["--scenario", "survival-biome-dimension-state"], &[])
        .expect("survival biome/dimension scenario parses");
    assert_eq!(
        biome_dimension.scenario,
        Scenario::SurvivalBiomeDimensionState
    );

    let mcp_controlled = test_config(&["--scenario", MCP_CONTROLLED_SMOKE_SCENARIO], &[])
        .expect("mcp-controlled scenario parses");
    assert_eq!(mcp_controlled.scenario, Scenario::McpControlledSmoke);

    let knockback = test_config(&["--scenario", "combat-knockback"], &[])
        .expect("combat-knockback scenario parses");
    assert_eq!(knockback.scenario, Scenario::CombatKnockback);

    let vanilla_combat = test_config(&["--scenario", "vanilla-combat-reference-parity"], &[])
        .expect("vanilla combat reference parity scenario parses");
    assert_eq!(
        vanilla_combat.scenario,
        Scenario::VanillaCombatReferenceParity
    );

    let vanilla_armor_combat = test_config(
        &["--scenario", "vanilla-combat-armor-reference-parity"],
        &[],
    )
    .expect("vanilla combat armor reference parity scenario parses");
    assert_eq!(
        vanilla_armor_combat.scenario,
        Scenario::VanillaCombatArmorReferenceParity
    );

    let armor_matrix = test_config(
        &["--scenario", "armor-loadout-enchantment-status-matrix"],
        &[],
    )
    .expect("armor matrix scenario parses");
    assert_eq!(
        armor_matrix.scenario,
        Scenario::ArmorLoadoutEnchantmentStatusMatrix
    );

    let equipment_matrix =
        test_config(&["--scenario", "equipment-slot-item-matrix-expansion"], &[])
            .expect("equipment matrix scenario parses");
    assert_eq!(
        equipment_matrix.scenario,
        Scenario::EquipmentSlotItemMatrixExpansion
    );

    let projectile_damage = test_config(&["--scenario", "projectile-damage-attribution"], &[])
        .expect("projectile damage scenario parses");
    assert_eq!(
        projectile_damage.scenario,
        Scenario::ProjectileDamageAttribution
    );

    let negative = test_config(&["--scenario", "negative-inventory-stale-state"], &[])
        .expect("negative scenario parses");
    assert_eq!(negative.scenario, Scenario::NegativeInventoryStaleState);

    let invalid_pickup = test_config(&["--scenario", "ctf-invalid-pickup-ownership"], &[])
        .expect("invalid pickup scenario parses");
    assert_eq!(invalid_pickup.scenario, Scenario::CtfInvalidPickupOwnership);

    let invalid_return_drop = test_config(&["--scenario", "ctf-invalid-return-drop"], &[])
        .expect("invalid return/drop scenario parses");
    assert_eq!(invalid_return_drop.scenario, Scenario::CtfInvalidReturnDrop);

    let invalid_opponent_base_return_drop = test_config(
        &["--scenario", "ctf-invalid-opponent-base-return-drop"],
        &[],
    )
    .expect("invalid opponent-base return/drop scenario parses");
    assert_eq!(
        invalid_opponent_base_return_drop.scenario,
        Scenario::CtfInvalidOpponentBaseReturnDrop
    );

    let score_limit = test_config(&["--scenario", "ctf-score-limit-win-condition"], &[])
        .expect("score limit win scenario parses");
    assert_eq!(score_limit.scenario, Scenario::CtfScoreLimitWinCondition);

    let ctf_race = test_config(&["--scenario", "ctf-simultaneous-pickup-capture-race"], &[])
        .expect("ctf race scenario parses");
    assert_eq!(
        ctf_race.scenario,
        Scenario::CtfSimultaneousPickupCaptureRace
    );

    let spawn_reset = test_config(&["--scenario", "ctf-spawn-team-balance-reset"], &[])
        .expect("ctf spawn team reset scenario parses");
    assert_eq!(spawn_reset.scenario, Scenario::CtfSpawnTeamBalanceReset);
}

#[test]
fn missing_client_checkout_has_actionable_diagnostic() {
    let missing = std::env::temp_dir().join(format!(
        "mc-compat-missing-stevenarella-{}",
        std::process::id()
    ));
    let cfg = test_config(&["--client-dir", missing.to_str().unwrap()], &[])
        .expect("config with missing Stevenarella checkout parses");

    let err = ensure_client_dir_ready(&cfg).unwrap_err();

    assert!(err.contains("Stevenarella source tree not found"), "{err}");
    assert!(err.contains("core client tree"), "{err}");
    assert!(err.contains("--client-dir/CLIENT_DIR"), "{err}");
    assert!(!err.contains("transition"), "{err}");
}

#[test]
fn client_checkout_must_point_at_manifest_root() {
    let dir =
        std::env::temp_dir().join(format!("mc-compat-bad-stevenarella-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("create bad Stevenarella checkout");
    let cfg = test_config(&["--client-dir", dir.to_str().unwrap()], &[])
        .expect("config with bad Stevenarella checkout parses");

    let err = ensure_client_dir_ready(&cfg).unwrap_err();

    assert!(err.contains("missing Cargo.toml"), "{err}");
    assert!(err.contains("Stevenarella source root"), "{err}");
}

#[test]
fn valid_client_checkout_preflight_passes() {
    let dir = fake_stevenarella_checkout("valid");
    let cfg = test_config(&["--client-dir", dir.to_str().unwrap()], &[])
        .expect("config with fake Stevenarella checkout parses");

    ensure_client_dir_ready(&cfg).expect("fake checkout has a manifest");
}
