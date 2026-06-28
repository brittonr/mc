use super::*;

const PAIRED_REFERENCE_BACKEND_LABEL: &str = "paper-reference";
const PAIRED_REFERENCE_COMPARISON_STATUS_PLACEHOLDER: &str = "dry-run-shape-not-compared";
const PAIRED_REFERENCE_METRIC_NAMES: &[&str] = &[
    "attacker_identity",
    "victim_identity",
    "weapon",
    "armor_state",
    "pre_health",
    "post_health",
    "damage_delta",
    "knockback_metric",
];
const PAIRED_REFERENCE_TOLERANCE_FIELDS: &[&str] = &["damage_tolerance", "knockback_tolerance"];
const PAIRED_REFERENCE_NON_CLAIMS: &[&str] = &[
    "dry_run_shape_only",
    "not_live_paper_valence_evidence",
    "not_comparator_pass",
    "not_exact_mojang_vanilla_parity",
    "not_full_combat_parity",
    "not_public_server_safety",
    "not_production_readiness",
];
const EMPTY_SHAPE_STRINGS: &[&str] = &[];
const PAIRED_REFERENCE_NOT_SELECTED_STATUS: &str = "not-selected";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PairedReferenceDryRunShape {
    selected: bool,
    scenario: Option<&'static str>,
    reference_backend: Option<&'static str>,
    valence_backend: Option<&'static str>,
    reference_revision: Option<&'static str>,
    valence_revision: Option<&'static str>,
    metric_names: &'static [&'static str],
    tolerance_fields: &'static [&'static str],
    comparison_status: &'static str,
    live_comparator_evidence: bool,
    claims_live_parity: bool,
    claims_exact_vanilla_parity: bool,
    non_claims: &'static [&'static str],
}

#[cfg(test)]
pub(crate) fn smoke_receipt_json(
    cfg: &Config,
    result: Result<&Option<ClientRunEvidence>, &str>,
) -> String {
    smoke_receipt_json_with_typed_event_oracle(cfg, result, None)
}

pub(crate) fn smoke_receipt_json_with_typed_event_oracle(
    cfg: &Config,
    result: Result<&Option<ClientRunEvidence>, &str>,
    typed_event_oracle: Option<&TypedEventOracleArtifact>,
) -> String {
    let status = if result.is_ok() { "pass" } else { "fail" };
    let error = result.err();
    let client = result.ok().and_then(|client| client.as_ref());
    let receipt_path = cfg
        .receipt_path
        .as_ref()
        .map(|path| path.display().to_string());
    let client_log_path = client
        .and_then(|evidence| evidence.log_path.as_ref())
        .map(|path| path.display().to_string());
    let client_log_paths = client
        .map(|evidence| {
            evidence
                .log_paths
                .iter()
                .map(|path| path.display().to_string())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let client_usernames = client
        .map(|evidence| evidence.usernames.clone())
        .unwrap_or_else(|| planned_client_usernames(cfg));
    let matched_pattern = client.and_then(|evidence| evidence.matched_success_pattern.as_deref());
    let classification = client.map(|evidence| evidence.classification);
    let exit_code = client.and_then(|evidence| evidence.exit_code);
    let scenario_evidence = client.and_then(|evidence| evidence.scenario.as_ref());
    let fallback_scenario = evaluate_scenario_for_config(cfg, "");
    let scenario = scenario_evidence.unwrap_or(&fallback_scenario);
    let server_evidence = client.and_then(|evidence| evidence.server_scenario.as_ref());
    let fallback_server = evaluate_server_scenario(cfg.scenario, "", &cfg.client_username);
    let server_scenario = server_evidence.unwrap_or(&fallback_server);
    let projectile_damage_causality =
        client.and_then(|evidence| evidence.projectile_damage_causality.as_ref());
    let fallback_projectile_damage_causality =
        evaluate_projectile_damage_causality(&[], "", &cfg.client_username);
    let selected_projectile_damage_causality =
        if scenario_behavior(cfg.scenario).uses_dynamic_projectile_health() {
            Some(projectile_damage_causality.unwrap_or(&fallback_projectile_damage_causality))
        } else {
            None
        };
    let projectile_damage_causality_passed = selected_projectile_damage_causality
        .map(|evidence| evidence.passed)
        .unwrap_or(true);
    let projectile_damage_causality_json = projectile_damage_causality_json(
        scenario_behavior(cfg.scenario).uses_dynamic_projectile_health(),
        selected_projectile_damage_causality,
    );
    let scenario_required: Vec<&str> = scenario_required_milestones(cfg.scenario)
        .iter()
        .map(|(name, _)| *name)
        .collect();
    let scenario_forbidden: Vec<&str> = scenario_forbidden_patterns(cfg.scenario)
        .iter()
        .map(|(name, _)| *name)
        .collect();
    let server_required: Vec<&str> = server_required_milestones(cfg.scenario)
        .iter()
        .map(|(name, _)| *name)
        .collect();
    let compat_bot_probe_selected = cfg.scenario == Scenario::CompatBotProbe;
    let compat_bot_target_address = format!("127.0.0.1:{}", cfg.server_port);
    let compat_bot_planned_clients = planned_client_usernames(cfg).len();
    let first_missing_client = scenario.missing_milestones.first().copied();
    let first_missing_server = server_scenario.missing_milestones.first().copied();
    let (first_forbidden_source, first_forbidden_pattern) =
        first_forbidden_match(scenario, server_scenario);
    let suggested_boundary = suggested_triage_boundary(
        error.is_some(),
        client.is_some(),
        first_missing_client,
        first_missing_server,
        first_forbidden_pattern,
        requires_server_correlation(cfg),
    );
    let enriched_triage = build_enriched_triage(EnrichedTriageInput {
        scenario,
        server_scenario,
        scenario_name: scenario_name(cfg.scenario),
        usernames: &client_usernames,
        error,
        first_missing_client,
        first_missing_server,
        first_forbidden_source,
        first_forbidden_pattern,
        suggested_boundary,
    });
    let enriched_triage_json = enriched_triage_json(&enriched_triage);
    let status_sample_json = json_string_vec(&cfg.expected_status_sample);
    let status_resource_configured = cfg.expected_status_description.is_some()
        || cfg.expected_status_version_name.is_some()
        || !cfg.expected_status_sample.is_empty();
    let packet_capture_selected = cfg.packet_capture_summary;
    let packet_capture_expected_packets: Vec<&str> = match cfg.scenario {
        Scenario::Smoke => vec!["status_response", "login_or_timeout"],
        Scenario::CompatBotProbe => vec!["status_response", "login_success", "play_join_game"],
        Scenario::FlagScoreRepeat | Scenario::BlueFlagScore => {
            vec!["login_success", "play_join_game", "chat_scoreboard"]
        }
        Scenario::InventoryInteraction => vec![
            "login_success",
            "play_join_game",
            "inventory_set_slot",
            "player_action_drop_item",
            "open_container",
            "player_window_click",
            "player_block_placement",
        ],
        Scenario::InventoryStackSplitMerge | Scenario::InventoryDragTransactions => vec![
            "login_success",
            "play_join_game",
            "inventory_set_slot",
            "player_window_click",
        ],
        Scenario::SurvivalBreakPlacePickup => vec![
            "login_success",
            "play_join_game",
            "player_action_break_block",
            "block_update",
            "inventory_pickup",
            "player_block_placement",
        ],
        Scenario::SurvivalChestPersistence => vec![
            "login_success",
            "play_join_game",
            "open_container",
            "player_window_click",
            "close_window",
            "disconnect_reconnect",
        ],
        Scenario::SurvivalCraftingTable => vec![
            "login_success",
            "play_join_game",
            "open_container",
            "crafting_grid_click",
            "crafting_result_collect",
            "inventory_update",
        ],
        Scenario::SurvivalCraftingRecipeBreadth => vec![
            "login_success",
            "play_join_game",
            "open_container",
            "shaped_recipe_result",
            "shapeless_recipe_result",
            "invalid_recipe_reject",
            "inventory_update",
        ],
        Scenario::SurvivalFurnacePersistence => vec![
            "login_success",
            "play_join_game",
            "open_container",
            "furnace_input_click",
            "furnace_fuel_click",
            "furnace_output_collect",
            "disconnect_reconnect",
        ],
        Scenario::SurvivalFurnaceSmeltingBreadth => vec![
            "login_success",
            "play_join_game",
            "open_container",
            "furnace_input",
            "fuel_inserted",
            "burn_progress",
            "output_available",
            "output_collected",
            "inventory_update",
            "invalid_fuel_attempt",
            "invalid_fuel_reject",
        ],
        Scenario::SurvivalHungerFood | Scenario::SurvivalHungerHealthCycle => vec![
            "login_success",
            "play_join_game",
            "inventory_set_slot",
            "use_item",
            "food_update",
            "inventory_update",
        ],
        Scenario::SurvivalMobDrop => vec![
            "login_success",
            "play_join_game",
            "spawn_mob",
            "use_entity_attack",
            "entity_destroy",
            "spawn_item",
            "collect_item",
            "inventory_update",
        ],
        Scenario::SurvivalMobAiLootBreadth => vec![
            "login_success",
            "play_join_game",
            "spawn_zombie",
            "ai_checkpoint",
            "use_entity_attack",
            "entity_destroy",
            "spawn_rotten_flesh",
            "collect_item",
            "inventory_update",
        ],
        Scenario::SurvivalRedstoneToggle => vec![
            "login_success",
            "play_join_game",
            "use_item_on_block",
            "redstone_powered_update",
            "redstone_return_update",
        ],
        Scenario::SurvivalRedstoneCircuitBreadth => vec![
            "login_success",
            "play_join_game",
            "use_item_on_block",
            "redstone_repeater_tick",
            "redstone_powered_update",
            "redstone_return_update",
        ],
        Scenario::SurvivalWorldPersistenceRestart => vec![
            "login_success",
            "play_join_game",
            "player_block_placement",
            "block_update",
            "controlled_reload",
            "disconnect_reconnect",
            "post_reload_block_update",
        ],
        Scenario::SurvivalWorldMultichunkDurability => vec![
            "login_success",
            "play_join_game",
            "two_chunk_block_mutation",
            "controlled_reload",
            "disconnect_reconnect",
            "post_reload_two_chunk_observation",
        ],
        Scenario::SurvivalCrashRecoveryParity => vec![
            "login_success",
            "play_join_game",
            "player_block_placement",
            "block_update",
            "forced_stop",
            "crash_recovery_restart",
            "disconnect_reconnect",
            "post_crash_block_update",
        ],
        Scenario::SurvivalBlockEntityPersistenceParity => vec![
            "login_success",
            "play_join_game",
            "sign_block_entity_payload",
            "controlled_reload",
            "disconnect_reconnect",
            "post_reload_sign_block_entity_payload",
        ],
        Scenario::SurvivalContainerBlockEntityBreadth => vec![
            "login_success",
            "play_join_game",
            "open_container",
            "container_transfer",
            "block_entity_payload",
            "block_entity_metadata",
            "container_reopen",
        ],
        Scenario::SurvivalBiomeDimensionState => vec![
            "login_success",
            "play_join_game",
            "dimension_world_identifier",
        ],
        Scenario::SurvivalBiomeDimensionTravel => vec![
            "login_success",
            "play_join_game",
            "origin_dimension_biome",
            "nether_portal_transition",
            "destination_dimension_biome",
        ],
        Scenario::SurvivalSignEditingLive => vec![
            "login_success",
            "play_join_game",
            "sign_editor_open",
            "update_sign",
            "post_update_sign_text",
        ],
        Scenario::McpControlledSmoke => vec![
            "mcp_initialize",
            "mcp_tools_list",
            "mcp_status_call",
            "mcp_command_outcomes",
        ],
        Scenario::CombatDamage => vec!["two_client_login", "play_join_game", "use_entity_attack"],
        Scenario::CombatKnockback => vec![
            "two_client_login",
            "play_join_game",
            "use_entity_attack",
            "entity_velocity",
        ],
        Scenario::VanillaCombatReferenceParity | Scenario::VanillaCombatArmorReferenceParity => {
            vec![
                "two_client_login",
                "play_join_game",
                "use_entity_attack",
                "health_update",
                "entity_velocity",
                "reference_comparator_inputs",
            ]
        }
        Scenario::ArmorEquipmentMitigation | Scenario::ArmorLoadoutEnchantmentStatusMatrix => vec![
            "two_client_login",
            "play_join_game",
            "inventory_set_slot",
            "use_entity_attack",
            "armor_mitigation",
        ],
        Scenario::EquipmentUpdateObservation | Scenario::EquipmentSlotItemMatrixExpansion => vec![
            "two_client_login",
            "play_join_game",
            "entity_equipment_update",
        ],
        Scenario::ProjectileHit => vec![
            "two_client_login",
            "play_join_game",
            "projectile_use_item",
            "projectile_hit_attribution",
        ],
        Scenario::ProjectileDamageAttribution => vec![
            "two_client_login",
            "play_join_game",
            "projectile_use_item",
            "projectile_hit_attribution",
            "health_update",
        ],
        Scenario::FlagCarrierDeathReturn => vec![
            "two_client_login",
            "play_join_game",
            "flag_pickup",
            "use_entity_attack",
            "health_death",
            "respawn_request",
        ],
        Scenario::ReconnectFlagState => vec![
            "login_success",
            "play_join_game",
            "flag_pickup",
            "disconnect_reconnect",
            "flag_state_reset",
        ],
        Scenario::ReconnectFlagScore => vec![
            "login_success",
            "play_join_game",
            "disconnect_reconnect",
            "chat_scoreboard",
        ],
        Scenario::MultiClientLoadScore => {
            vec!["two_client_login", "play_join_game", "chat_scoreboard"]
        }
        Scenario::NegativeInventoryStaleState => vec![
            "login_success",
            "play_join_game",
            "inventory_click_stale_state",
        ],
        Scenario::NegativeInventoryInvalidClick => vec![
            "login_success",
            "play_join_game",
            "inventory_click_invalid_slot",
        ],
        Scenario::NegativeCustomPayload => vec![
            "login_success",
            "play_join_game",
            "custom_payload_malformed",
        ],
        Scenario::NegativeReconnectRace => vec![
            "login_success",
            "play_join_game",
            "disconnect_reconnect",
            "flag_state_race",
        ],
        Scenario::NegativeCtfWrongScore => {
            vec!["login_success", "play_join_game", "wrong_score_path"]
        }
        Scenario::CtfInvalidPickupOwnership => vec![
            "login_success",
            "play_join_game",
            "own_flag_pickup_attempt",
            "invalid_flag_pickup_rejected",
        ],
        Scenario::CtfInvalidReturnDrop => vec![
            "login_success",
            "play_join_game",
            "own_flag_return_drop_attempt",
            "invalid_flag_return_drop_rejected",
        ],
        Scenario::CtfInvalidOpponentBaseReturnDrop => vec![
            "login_success",
            "play_join_game",
            "opponent_base_return_drop_attempt",
            "invalid_opponent_base_return_drop_rejected",
        ],
        Scenario::CtfScoreLimitWinCondition => vec![
            "login_success",
            "play_join_game",
            "flag_pickup",
            "flag_capture",
            "score_limit_win_condition",
        ],
        Scenario::CtfSimultaneousPickupCaptureRace => vec![
            "two_client_login",
            "play_join_game",
            "flag_pickup",
            "duplicate_flag_pickup_rejected",
            "flag_capture",
            "race_final_state",
        ],
        Scenario::CtfSpawnTeamBalanceReset => vec![
            "two_client_login",
            "play_join_game",
            "team_assignment",
            "balanced_team_counts",
            "flag_capture",
            "resource_reset_state",
        ],
    };
    let child_revisions = child_revision_evidence_for_receipt(cfg);
    let mcp_control = evaluate_mcp_control_receipt(cfg, &child_revisions.client, client);
    let mcp_control_json = render_mcp_control_receipt_json(&mcp_control);
    let frame_artifacts = evaluate_frame_artifacts_receipt(cfg, client);
    let frame_artifacts_json = render_frame_artifacts_receipt_json(&frame_artifacts);
    let typed_event_oracle_json = typed_event_oracle_receipt_json(typed_event_oracle);
    let latency_jitter_json = latency_jitter_receipt_json(cfg);
    let public_server_authorized_safety_json = public_server_authorized_safety_receipt_json(cfg);
    let load_network_safety = evaluate_load_network_safety(load_network_safety_inputs(
        cfg,
        client.is_some() && server_scenario.passed,
        matches!(cfg.mode, Mode::Run),
    ));
    let load_network_safety_json = render_load_network_safety_json(&load_network_safety);
    let negative_live_rail =
        evaluate_negative_live_rail_safety_from_inputs(negative_live_rail_inputs_from_config(
            cfg,
            Some(scenario),
            matches!(cfg.mode, Mode::Run) && is_negative_live_rail(cfg.scenario),
        ));
    let negative_live_rail_json = render_negative_live_rail_json(&negative_live_rail);
    let armor_loadout_enchantment_status_matrix =
        evaluate_armor_loadout_enchantment_status_matrix(cfg, scenario, server_scenario);
    let armor_loadout_enchantment_status_matrix_json =
        render_armor_loadout_enchantment_status_matrix_json(
            &armor_loadout_enchantment_status_matrix,
        );
    let equipment_slot_item_matrix_expansion =
        evaluate_equipment_slot_item_matrix_expansion(cfg, scenario, server_scenario);
    let equipment_slot_item_matrix_expansion_json =
        render_equipment_slot_item_matrix_expansion_json(&equipment_slot_item_matrix_expansion);
    let proxy_route = cfg.proxy_route.as_deref().unwrap_or("direct");
    let proxy_forwarding_mode = cfg.proxy_forwarding_mode.as_deref().unwrap_or("none");
    let proxy_selected = cfg.proxy_route.is_some();
    let gameplay_oracle_milestones: Vec<&str> = vec![
        "protocol_detected",
        "join_game",
        "render_tick",
        "team_red",
        "team_blue",
        "flag_pickup",
        "flag_capture",
        "score_red_1",
        "score_red_2",
        "score_blue_1",
        "inventory_slot_update",
        "inventory_sword_slot",
        "inventory_wool_slot",
        "inventory_drop_sent",
        "inventory_pickup_seen",
        "inventory_click_sent",
        "inventory_open_container_seen",
        "inventory_container_click_sent",
        "inventory_block_place_sent",
        "inventory_stack_initial_slot",
        "inventory_stack_split_pickup_sent",
        "inventory_stack_split_source_seen",
        "inventory_stack_split_place_sent",
        "inventory_stack_destination_seen",
        "inventory_stack_merge_pickup_sent",
        "inventory_stack_merge_destination_empty_seen",
        "inventory_stack_merge_place_sent",
        "inventory_stack_final_source_seen",
        "server_inventory_stack_split_pickup",
        "server_inventory_stack_split",
        "server_inventory_stack_merge_pickup",
        "server_inventory_stack_merge",
        "inventory_drag_initial_slot",
        "inventory_drag_pickup_sent",
        "inventory_drag_source_empty_seen",
        "inventory_drag_start_sent",
        "inventory_drag_target_a_sent",
        "inventory_drag_target_b_sent",
        "inventory_drag_end_sent",
        "inventory_drag_final_distribution_seen",
        "server_inventory_drag_pickup",
        "server_inventory_drag_start",
        "server_inventory_drag_target_a",
        "server_inventory_drag_target_b",
        "server_inventory_drag_end",
        "survival_break_sent",
        "survival_break_update",
        "survival_pickup_seen",
        "survival_place_sent",
        "survival_place_update",
        "server_survival_join",
        "server_survival_break",
        "server_survival_pickup",
        "server_survival_place",
        "survival_chest_open_seen",
        "survival_chest_store_sent",
        "survival_chest_close_sent",
        "survival_chest_reconnect_sent",
        "survival_chest_reopen_seen",
        "survival_chest_persisted_seen",
        "server_survival_chest_open",
        "server_survival_chest_store",
        "server_survival_chest_close",
        "server_survival_chest_reopen",
        "server_survival_chest_persisted",
        "survival_crafting_table_open_seen",
        "survival_crafting_input_a_sent",
        "survival_crafting_input_b_sent",
        "survival_crafting_result_seen",
        "survival_crafting_result_collected",
        "survival_crafting_inventory_updated",
        "server_survival_crafting_table_open",
        "server_survival_crafting_input_a",
        "server_survival_crafting_input_b",
        "server_survival_crafting_result",
        "server_survival_crafting_collect",
        "survival_crafting_breadth_shaped_seen",
        "survival_crafting_breadth_shapeless_seen",
        "survival_crafting_breadth_grid_clear_seen",
        "survival_crafting_breadth_invalid_seen",
        "survival_crafting_breadth_inventory_updated",
        "server_survival_crafting_breadth_shaped",
        "server_survival_crafting_breadth_shapeless",
        "server_survival_crafting_breadth_grid_clear",
        "server_survival_crafting_breadth_invalid_rejected",
        "server_survival_crafting_breadth_state",
        "survival_furnace_open_seen",
        "survival_furnace_input_sent",
        "survival_furnace_fuel_sent",
        "survival_furnace_burn_progress_seen",
        "survival_furnace_output_seen",
        "survival_furnace_output_collected",
        "survival_furnace_inventory_updated",
        "survival_furnace_reconnect_sent",
        "survival_furnace_reopen_seen",
        "server_survival_furnace_open",
        "server_survival_furnace_input",
        "server_survival_furnace_fuel",
        "server_survival_furnace_burn_progress",
        "server_survival_furnace_output_available",
        "server_survival_furnace_output_collect",
        "server_survival_furnace_reconnect_reopen",
        "server_survival_furnace_state",
        "survival_hunger_food_item_seen",
        "survival_hunger_food_pre_seen",
        "survival_hunger_food_use_sent",
        "survival_hunger_food_post_seen",
        "survival_hunger_food_inventory_updated",
        "server_survival_hunger_food_pre",
        "server_survival_hunger_food_consume_start",
        "server_survival_hunger_food_consume_finish",
        "server_survival_hunger_food_inventory",
        "server_survival_hunger_food_state",
        "survival_hunger_health_item_seen",
        "survival_hunger_health_pre_seen",
        "survival_hunger_health_consume_sent",
        "survival_hunger_health_recovery_seen",
        "survival_hunger_health_inventory_updated",
        "server_survival_hunger_health_pre",
        "server_survival_hunger_health_consume_start",
        "server_survival_hunger_health_consume_finish",
        "server_survival_hunger_health_inventory",
        "server_survival_hunger_health_state",
        "survival_mob_drop_mob_seen",
        "survival_mob_drop_attack_sent",
        "survival_mob_drop_death_seen",
        "survival_mob_drop_drop_seen",
        "survival_mob_drop_pickup_seen",
        "survival_mob_drop_inventory_updated",
        "server_survival_mob_drop_spawn",
        "server_survival_mob_drop_attack",
        "server_survival_mob_drop_death",
        "server_survival_mob_drop_drop_spawn",
        "server_survival_mob_drop_pickup",
        "server_survival_mob_drop_inventory",
        "server_survival_mob_drop_state",
        "survival_redstone_toggle_input_sent",
        "survival_redstone_toggle_output_update",
        "survival_redstone_toggle_return_input_sent",
        "survival_redstone_toggle_return_update",
        "server_survival_redstone_toggle_input",
        "server_survival_redstone_toggle_powered_on",
        "server_survival_redstone_toggle_powered_off",
        "server_survival_redstone_toggle_state",
        "survival_world_persistence_mutation_sent",
        "survival_world_persistence_pre_restart_update",
        "survival_world_persistence_reconnect_sent",
        "survival_world_persistence_post_restart_update",
        "server_survival_world_persistence_mutation",
        "server_survival_world_persistence_clean_shutdown",
        "server_survival_world_persistence_backend_restart",
        "server_survival_world_persistence_post_restart",
        "server_survival_world_persistence_state",
        "survival_crash_recovery_mutation_sent",
        "survival_crash_recovery_pre_crash_update",
        "survival_crash_recovery_reconnect_sent",
        "survival_crash_recovery_post_crash_update",
        "server_survival_crash_recovery_mutation",
        "server_survival_crash_recovery_forced_stop",
        "server_survival_crash_recovery_backend_restart",
        "server_survival_crash_recovery_post_crash",
        "server_survival_crash_recovery_state",
        "server_inventory_hotbar_select",
        "server_inventory_drop",
        "server_inventory_pickup",
        "server_inventory_click",
        "server_inventory_open_container",
        "server_inventory_container_click",
        "server_block_place",
        "reconnect_session",
        "multi_client_count",
        "remote_player_spawn",
        "combat_attack_sent",
        "combat_health_update",
        "combat_velocity_update",
        "server_vanilla_combat_reference_damage",
        "server_vanilla_combat_reference_knockback",
        "armor_inventory_slot",
        "entity_equipment_update",
        "server_equipment_state",
        "server_equipment_update_state",
        "server_armor_mitigation",
        "server_combat_damage",
        "server_combat_knockback",
        "projectile_use_sent",
        "projectile_swing_sent",
        "projectile_damage_update",
        "server_projectile_use",
        "server_projectile_hit",
        "flag_carrier_death",
        "flag_return",
        "flag_disconnect_return",
        "reconnect_state_coherent",
        "ctf_invalid_pickup_attempted",
        "ctf_invalid_pickup_contained",
        "server_invalid_pickup_rejected",
        "ctf_invalid_return_drop_attempted",
        "ctf_invalid_return_drop_contained",
        "server_invalid_return_drop_rejected",
        "ctf_invalid_opponent_base_return_drop_attempted",
        "ctf_invalid_opponent_base_return_drop_contained",
        "server_invalid_opponent_base_return_drop_rejected",
        "ctf_score_limit_win_seen",
        "server_score_limit_pre_state",
        "server_score_limit_final_capture",
        "server_score_limit_win_condition",
        "ctf_race_client_count",
        "server_ctf_race_accepted_transition",
        "server_ctf_race_rejected_transition",
        "server_ctf_race_final_state",
        "ctf_spawn_team_reset_client_count",
        "server_ctf_spawn_red_assignment",
        "server_ctf_spawn_blue_assignment",
        "server_ctf_spawn_team_balance",
        "server_ctf_spawn_resource_reset",
        "mcp_control_dry_run",
        "mcp_handshake_success",
        "mcp_stdout_clean",
        "mcp_command_outcome",
    ];
    let gameplay_non_claims: Vec<&str> = vec![
        "full_ctf_correctness",
        "full_survival_compatibility",
        "vanilla_parity",
        "broad_minecraft_compatibility",
        "unbounded_soak",
        "production_load",
        "full_projectile_physics",
        "all_projectile_weapons",
        "enchantments_or_status_effects",
    ];
    let paired_reference_dry_run_shape =
        build_paired_reference_dry_run_shape(cfg.scenario, cfg.mode == Mode::DryRun);
    let paired_reference_dry_run_shape_json =
        paired_reference_dry_run_shape_json(&paired_reference_dry_run_shape);
    let client_git_rev_json = json_optional_string(child_revisions.client.resolved_rev.as_deref());
    let client_git_status_json = json_string(child_revisions.client.status);
    let client_git_diagnostics_json = json_string_vec(&child_revisions.client.diagnostics);
    let valence_git_rev_requested_json =
        json_optional_string(child_revisions.valence.requested_rev.as_deref());
    let valence_git_rev_resolved_json =
        json_optional_string(child_revisions.valence.resolved_rev.as_deref());
    let valence_git_status_json = json_string(child_revisions.valence.status);
    let valence_git_diagnostics_json = json_string_vec(&child_revisions.valence.diagnostics);
    let error_json = error.map(json_string).unwrap_or_else(|| "null".to_string());
    let receipt_path_json = json_optional_string(receipt_path.as_deref());
    let client_log_json = json_optional_string(client_log_path.as_deref());
    let client_logs_json = json_string_vec(&client_log_paths);
    let client_usernames_json = json_string_vec(&client_usernames);
    let server_log_json = json_string(&server_log_label(cfg));
    let matched_pattern_json = json_optional_string(matched_pattern);
    let classification_json = json_optional_string(classification);
    let exit_code_json = exit_code
        .map(|code| code.to_string())
        .unwrap_or_else(|| "null".to_string());

    format!(
        r#"{{
  "schema": "mc.compat.scenario.receipt.v2",
  "legacy_schema": "mc.compat.smoke.receipt.v1",
  "status": {status_json},
  "mode": {mode_json},
  "dry_run": {dry_run},
  "contract": {{
    "cairn_contract": "mc.compat.scenario.receipt.v2",
    "legacy_cairn_contract": "mc.compat.smoke.receipt.v1",
    "octet_producer_surface": "compat/runner/src/main.rs",
    "claims_correctness": false,
    "claims_semantic_equivalence": false
  }},
  "scenario": {{
    "name": {scenario_name_json},
    "required_milestones": {scenario_required_json},
    "observed_milestones": {scenario_observed_json},
    "missing_milestones": {scenario_missing_json},
    "forbidden_patterns": {scenario_forbidden_json},
    "forbidden_matches": {scenario_forbidden_matches_json},
    "passed": {scenario_passed}
  }},
  "compat_bot_probe": {{
    "selected": {compat_bot_probe_selected},
    "safe_bounded_probe": true,
    "target_address": {compat_bot_target_address_json},
    "owned_local_target_required": true,
    "external_server_load_authorized": false,
    "public_stress_tool": false,
    "planned_clients": {compat_bot_planned_clients},
    "max_clients": 1,
    "duration_secs": {timeout_secs},
    "derived_from": "hyperion/tools/rust-mc-bot pattern"
  }},
  "status_response_resource": {{
    "resource_owned": true,
    "configured": {status_resource_configured},
    "expected_description": {expected_status_description_json},
    "expected_version_name": {expected_status_version_name_json},
    "expected_player_sample": {status_sample_json},
    "defaults_stable": true,
    "asserted_by_status_probe": {status_resource_configured}
  }},
  "packet_capture_oracle": {{
    "selected": {packet_capture_selected},
    "headless_cli": true,
    "redacted_receipt": true,
    "raw_payloads_recorded": false,
    "normalized_fields": ["direction", "state", "packet_id", "decode_status"],
    "expected_summary_packets": {packet_capture_expected_packets_json},
    "triage_correlation": true
  }},
  "typed_event_oracle": {typed_event_oracle_json},
  "mcp_control": {mcp_control_json},
  "frame_artifacts": {frame_artifacts_json},
  "latency_jitter_tolerance": {latency_jitter_json},
  "load_network_safety": {load_network_safety_json},
  "public_server_authorized_safety": {public_server_authorized_safety_json},
  "negative_live_rail": {negative_live_rail_json},
  "proxy_compat_seam": {{
    "selected": {proxy_selected},
    "route": {proxy_route_json},
    "forwarding_mode": {proxy_forwarding_mode_json},
    "direct_and_proxied_claims_separated": true,
    "mtls_ported": false,
    "credentials_recorded": false,
    "owned_local_proxy_required": true
  }},
  "gameplay_oracles": {{
    "derived_from": "hyperion gameplay milestone vocabulary",
    "selected_scenario": {scenario_name_json},
    "catalog": {gameplay_oracle_milestones_json},
    "requires_client_and_server_evidence_for_semantic_claims": true,
    "non_claims": {gameplay_non_claims_json}
  }},
  "paired_reference_dry_run_shape": {paired_reference_dry_run_shape_json},
  "armor_loadout_enchantment_status_matrix": {armor_loadout_enchantment_status_matrix_json},
  "equipment_slot_item_matrix_expansion": {equipment_slot_item_matrix_expansion_json},
  "server": {{
    "backend": {backend_json},
    "version": {version_json},
    "protocol": {protocol},
    "port": {port},
    "required_milestones": {server_required_json},
    "observed_milestones": {server_observed_json},
    "missing_milestones": {server_missing_json},
    "forbidden_matches": {server_forbidden_matches_json},
    "passed": {server_passed},
    "client_server_correlation": {{
      "scenario": {scenario_name_json},
      "usernames": {client_usernames_json},
      "passed": {correlation_passed}
    }}
  }},
  "projectile_damage_causality": {projectile_damage_causality_json},
  "client": {{
    "dir": {client_dir_json},
    "git_rev": {client_git_rev_json},
    "git_status": {client_git_status_json},
    "git_dirty": {client_git_dirty},
    "git_diagnostics": {client_git_diagnostics_json},
    "target_dir": {target_dir_json},
    "username": {username_json},
    "usernames": {client_usernames_json},
    "timeout_secs": {timeout_secs},
    "headless_isolation": {{
      "xvfb": true,
      "x11_backend": true,
      "software_gl": true,
      "wayland_socket_inherited": false
    }},
    "log_path": {client_log_json},
    "log_paths": {client_logs_json},
    "exit_code": {exit_code_json},
    "classification": {classification_json},
    "matched_success_pattern": {matched_pattern_json}
  }},
  "valence": {{
    "repo": {valence_repo_json},
    "rev": {valence_rev_json},
    "git_rev_requested": {valence_git_rev_requested_json},
    "git_rev_resolved": {valence_git_rev_resolved_json},
    "git_status": {valence_git_status_json},
    "git_dirty": {valence_git_dirty},
    "git_diagnostics": {valence_git_diagnostics_json},
    "worktree": {valence_worktree_json},
    "example": {valence_example_json},
    "log_path": {valence_log_json}
  }},
  "triage": {{
    "first_missing_client_milestone": {first_missing_client_json},
    "first_missing_server_milestone": {first_missing_server_json},
    "first_forbidden_pattern": {first_forbidden_pattern_json},
    "first_forbidden_source": {first_forbidden_source_json},
    "client_log_paths": {client_logs_json},
    "server_log_path": {server_log_json},
    "suggested_boundary": {suggested_boundary_json},
    "enriched": {enriched_triage_json}
  }},
  "receipt_path": {receipt_path_json},
  "error": {error_json}
}}
"#,
        status_json = json_string(status),
        mode_json = json_string(mode_name(cfg.mode)),
        dry_run = cfg.mode == Mode::DryRun,
        scenario_name_json = json_string(scenario_name(cfg.scenario)),
        scenario_required_json = json_string_array(&scenario_required),
        scenario_observed_json = json_string_array(&scenario.observed_milestones),
        scenario_missing_json = json_string_array(&scenario.missing_milestones),
        scenario_forbidden_json = json_string_array(&scenario_forbidden),
        scenario_forbidden_matches_json = json_string_array(&scenario.forbidden_matches),
        scenario_passed = scenario.passed,
        compat_bot_probe_selected = compat_bot_probe_selected,
        compat_bot_target_address_json = json_string(&compat_bot_target_address),
        compat_bot_planned_clients = compat_bot_planned_clients,
        status_resource_configured = status_resource_configured,
        expected_status_description_json =
            json_optional_string(cfg.expected_status_description.as_deref()),
        expected_status_version_name_json =
            json_optional_string(cfg.expected_status_version_name.as_deref()),
        status_sample_json = status_sample_json,
        packet_capture_selected = packet_capture_selected,
        packet_capture_expected_packets_json = json_string_array(&packet_capture_expected_packets),
        typed_event_oracle_json = typed_event_oracle_json,
        mcp_control_json = mcp_control_json,
        frame_artifacts_json = frame_artifacts_json,
        load_network_safety_json = load_network_safety_json,
        negative_live_rail_json = negative_live_rail_json,
        armor_loadout_enchantment_status_matrix_json = armor_loadout_enchantment_status_matrix_json,
        equipment_slot_item_matrix_expansion_json = equipment_slot_item_matrix_expansion_json,
        proxy_selected = proxy_selected,
        proxy_route_json = json_string(proxy_route),
        proxy_forwarding_mode_json = json_string(proxy_forwarding_mode),
        gameplay_oracle_milestones_json = json_string_array(&gameplay_oracle_milestones),
        gameplay_non_claims_json = json_string_array(&gameplay_non_claims),
        paired_reference_dry_run_shape_json = paired_reference_dry_run_shape_json,
        server_required_json = json_string_array(&server_required),
        server_observed_json = json_string_array(&server_scenario.observed_milestones),
        server_missing_json = json_string_array(&server_scenario.missing_milestones),
        server_forbidden_matches_json = json_string_array(&server_scenario.forbidden_matches),
        server_passed = server_scenario.passed,
        correlation_passed =
            scenario.passed && server_scenario.passed && projectile_damage_causality_passed,
        projectile_damage_causality_json = projectile_damage_causality_json,
        backend_json = json_string(backend_name(cfg.server_backend)),
        version_json = json_string(&cfg.server_version),
        protocol = cfg.server_protocol,
        port = cfg.server_port,
        client_dir_json = json_string(&cfg.client_dir.display().to_string()),
        client_git_rev_json = client_git_rev_json,
        client_git_status_json = client_git_status_json,
        client_git_dirty = child_revisions.client.dirty,
        client_git_diagnostics_json = client_git_diagnostics_json,
        target_dir_json = json_string(&cfg.target_dir.display().to_string()),
        username_json = json_string(&cfg.client_username),
        client_usernames_json = client_usernames_json,
        client_logs_json = client_logs_json,
        client_log_json = client_log_json,
        matched_pattern_json = matched_pattern_json,
        classification_json = classification_json,
        exit_code_json = exit_code_json,
        timeout_secs = cfg.client_timeout.as_secs(),
        valence_repo_json = json_string(&cfg.valence_repo.display().to_string()),
        valence_rev_json = json_string(&cfg.valence_rev),
        valence_git_rev_requested_json = valence_git_rev_requested_json,
        valence_git_rev_resolved_json = valence_git_rev_resolved_json,
        valence_git_status_json = valence_git_status_json,
        valence_git_dirty = child_revisions.valence.dirty,
        valence_git_diagnostics_json = valence_git_diagnostics_json,
        valence_worktree_json = json_string(&cfg.valence_worktree.display().to_string()),
        valence_example_json = json_string(&cfg.valence_example),
        valence_log_json = json_string(&cfg.valence_log.display().to_string()),
        server_log_json = server_log_json,
        receipt_path_json = receipt_path_json,
        error_json = error_json,
        first_missing_client_json = json_optional_string(first_missing_client),
        first_missing_server_json = json_optional_string(first_missing_server),
        first_forbidden_pattern_json = json_optional_string(first_forbidden_pattern),
        first_forbidden_source_json = json_optional_string(first_forbidden_source),
        suggested_boundary_json = json_string(suggested_boundary),
        enriched_triage_json = enriched_triage_json,
    )
}

fn build_paired_reference_dry_run_shape(
    scenario: Scenario,
    dry_run: bool,
) -> PairedReferenceDryRunShape {
    if !dry_run || !is_paired_reference_scenario(scenario) {
        return PairedReferenceDryRunShape {
            selected: false,
            scenario: None,
            reference_backend: None,
            valence_backend: None,
            reference_revision: None,
            valence_revision: None,
            metric_names: EMPTY_SHAPE_STRINGS,
            tolerance_fields: EMPTY_SHAPE_STRINGS,
            comparison_status: PAIRED_REFERENCE_NOT_SELECTED_STATUS,
            live_comparator_evidence: false,
            claims_live_parity: false,
            claims_exact_vanilla_parity: false,
            non_claims: EMPTY_SHAPE_STRINGS,
        };
    }

    PairedReferenceDryRunShape {
        selected: true,
        scenario: Some(scenario_name(scenario)),
        reference_backend: Some(PAIRED_REFERENCE_BACKEND_LABEL),
        valence_backend: Some(backend_name(ServerBackend::Valence)),
        reference_revision: Some(GIT_REV_DRY_RUN_PLACEHOLDER),
        valence_revision: Some(GIT_REV_DRY_RUN_PLACEHOLDER),
        metric_names: PAIRED_REFERENCE_METRIC_NAMES,
        tolerance_fields: PAIRED_REFERENCE_TOLERANCE_FIELDS,
        comparison_status: PAIRED_REFERENCE_COMPARISON_STATUS_PLACEHOLDER,
        live_comparator_evidence: false,
        claims_live_parity: false,
        claims_exact_vanilla_parity: false,
        non_claims: PAIRED_REFERENCE_NON_CLAIMS,
    }
}

fn is_paired_reference_scenario(scenario: Scenario) -> bool {
    matches!(
        scenario,
        Scenario::VanillaCombatReferenceParity | Scenario::VanillaCombatArmorReferenceParity
    )
}

fn paired_reference_dry_run_shape_json(shape: &PairedReferenceDryRunShape) -> String {
    format!(
        r#"{{
    "selected": {selected},
    "scenario": {scenario_json},
    "reference_backend": {reference_backend_json},
    "valence_backend": {valence_backend_json},
    "reference_revision": {reference_revision_json},
    "valence_revision": {valence_revision_json},
    "metric_names": {metric_names_json},
    "tolerance_fields": {tolerance_fields_json},
    "comparison_status": {comparison_status_json},
    "live_comparator_evidence": {live_comparator_evidence},
    "claims_live_parity": {claims_live_parity},
    "claims_exact_vanilla_parity": {claims_exact_vanilla_parity},
    "non_claims": {non_claims_json}
  }}"#,
        selected = shape.selected,
        scenario_json = json_optional_string(shape.scenario),
        reference_backend_json = json_optional_string(shape.reference_backend),
        valence_backend_json = json_optional_string(shape.valence_backend),
        reference_revision_json = json_optional_string(shape.reference_revision),
        valence_revision_json = json_optional_string(shape.valence_revision),
        metric_names_json = json_string_array(shape.metric_names),
        tolerance_fields_json = json_string_array(shape.tolerance_fields),
        comparison_status_json = json_string(shape.comparison_status),
        live_comparator_evidence = shape.live_comparator_evidence,
        claims_live_parity = shape.claims_live_parity,
        claims_exact_vanilla_parity = shape.claims_exact_vanilla_parity,
        non_claims_json = json_string_array(shape.non_claims),
    )
}

fn projectile_damage_causality_json(
    selected: bool,
    evidence: Option<&ProjectileDamageCausalityEvidence>,
) -> String {
    let Some(evidence) = evidence else {
        return format!(
            r#"{{
    "selected": {selected},
    "attacker": null,
    "victim": null,
    "required_steps": [],
    "observed_steps": [],
    "missing_steps": [],
    "order_violations": [],
    "proof_basis": "not-selected",
    "passed": {passed}
  }}"#,
            selected = selected,
            passed = !selected,
        );
    };
    format!(
        r#"{{
    "selected": {selected},
    "attacker": {attacker_json},
    "victim": {victim_json},
    "required_steps": {required_steps_json},
    "observed_steps": {observed_steps_json},
    "missing_steps": {missing_steps_json},
    "order_violations": {order_violations_json},
    "proof_basis": "attacker_client_packet_send_plus_valence_attacker_victim_health_delta_plus_victim_client_health_update",
    "passed": {passed}
  }}"#,
        selected = selected,
        attacker_json = json_string(&evidence.attacker_username),
        victim_json = json_string(&evidence.victim_username),
        required_steps_json = json_string_array(&evidence.required_steps),
        observed_steps_json = json_string_array(&evidence.observed_steps),
        missing_steps_json = json_string_array(&evidence.missing_steps),
        order_violations_json = json_string_array(&evidence.order_violations),
        passed = evidence.passed,
    )
}

fn first_forbidden_match<'a>(
    scenario: &'a ScenarioEvidence,
    server_scenario: &'a ServerScenarioEvidence,
) -> (Option<&'static str>, Option<&'a str>) {
    if let Some(pattern) = scenario.forbidden_matches.first() {
        (Some("client"), Some(*pattern))
    } else if let Some(pattern) = server_scenario.forbidden_matches.first() {
        (Some("server"), Some(*pattern))
    } else {
        (None, None)
    }
}

fn suggested_triage_boundary(
    has_error: bool,
    has_client_evidence: bool,
    first_missing_client: Option<&str>,
    first_missing_server: Option<&str>,
    first_forbidden_pattern: Option<&str>,
    requires_server_correlation: bool,
) -> &'static str {
    if has_error && !has_client_evidence {
        "preflight-or-server-startup"
    } else if first_forbidden_pattern.is_some() {
        "protocol-runtime"
    } else if first_missing_client.is_some() {
        "client-probe"
    } else if requires_server_correlation && first_missing_server.is_some() {
        "server-correlation"
    } else if has_error {
        "runner-error"
    } else {
        "none"
    }
}

pub(crate) struct EnrichedTriageInput<'a> {
    pub(crate) scenario: &'a ScenarioEvidence,
    pub(crate) server_scenario: &'a ServerScenarioEvidence,
    pub(crate) scenario_name: &'a str,
    pub(crate) usernames: &'a [String],
    pub(crate) error: Option<&'a str>,
    pub(crate) first_missing_client: Option<&'a str>,
    pub(crate) first_missing_server: Option<&'a str>,
    pub(crate) first_forbidden_source: Option<&'a str>,
    pub(crate) first_forbidden_pattern: Option<&'a str>,
    pub(crate) suggested_boundary: &'a str,
}

pub(crate) fn build_enriched_triage(input: EnrichedTriageInput<'_>) -> EnrichedTriage {
    let last_client_event = input
        .scenario
        .observed_milestones
        .last()
        .map(|name| (*name).to_string());
    let last_server_event = input
        .server_scenario
        .observed_milestones
        .last()
        .map(|name| (*name).to_string());
    let mut correlation_ids = vec![format!("scenario={}", input.scenario_name)];
    correlation_ids.extend(
        input
            .usernames
            .iter()
            .map(|username| format!("user={username}")),
    );

    let mut timeline_excerpt = Vec::new();
    push_triage_excerpt(
        &mut timeline_excerpt,
        format!("boundary={}", input.suggested_boundary),
    );
    if let Some(error) = input.error {
        push_triage_excerpt(&mut timeline_excerpt, format!("error={error}"));
    }
    if let Some(milestone) = input.first_missing_client {
        push_triage_excerpt(&mut timeline_excerpt, format!("missing_client={milestone}"));
    }
    if let Some(milestone) = input.first_missing_server {
        push_triage_excerpt(&mut timeline_excerpt, format!("missing_server={milestone}"));
    }
    if let Some(pattern) = input.first_forbidden_pattern {
        let source = input.first_forbidden_source.unwrap_or("unknown");
        push_triage_excerpt(
            &mut timeline_excerpt,
            format!("forbidden_{source}={pattern}"),
        );
    }
    if timeline_excerpt.is_empty() {
        push_triage_excerpt(&mut timeline_excerpt, "boundary=none".to_string());
    }

    EnrichedTriage {
        last_client_event,
        last_server_event,
        correlation_ids,
        timeline_excerpt,
        boundary_confidence: triage_boundary_confidence(input.suggested_boundary),
    }
}

fn push_triage_excerpt(lines: &mut Vec<String>, line: String) {
    if lines.len() >= TRIAGE_MAX_TIMELINE_EVENTS {
        return;
    }
    lines.push(bound_redacted_excerpt(&line));
}

fn triage_boundary_confidence(boundary: &str) -> &'static str {
    match boundary {
        "none" => TRIAGE_CONFIDENCE_NONE,
        "client-probe" | "runner-error" => TRIAGE_CONFIDENCE_MEDIUM,
        _ => TRIAGE_CONFIDENCE_HIGH,
    }
}

fn bound_redacted_excerpt(line: &str) -> String {
    let redacted = redact_sensitive_excerpt(line);
    if redacted.chars().count() <= TRIAGE_MAX_EXCERPT_CHARS {
        return redacted;
    }
    redacted.chars().take(TRIAGE_MAX_EXCERPT_CHARS).collect()
}

fn redact_sensitive_excerpt(line: &str) -> String {
    line.split_whitespace()
        .map(redact_sensitive_token)
        .collect::<Vec<_>>()
        .join(" ")
}

fn redact_sensitive_token(token: &str) -> &str {
    let lower = token.to_ascii_lowercase();
    if lower.contains("token=")
        || lower.contains("secret=")
        || lower.contains("password=")
        || token.starts_with('/')
    {
        TRIAGE_REDACTED
    } else {
        token
    }
}

fn enriched_triage_json(triage: &EnrichedTriage) -> String {
    format!(
        r#"{{
    "last_client_event": {last_client_event_json},
    "last_server_event": {last_server_event_json},
    "correlation_ids": {correlation_ids_json},
    "timeline_excerpt": {timeline_excerpt_json},
    "boundary_confidence": {boundary_confidence_json}
  }}"#,
        last_client_event_json = json_optional_string(triage.last_client_event.as_deref()),
        last_server_event_json = json_optional_string(triage.last_server_event.as_deref()),
        correlation_ids_json = json_string_vec(&triage.correlation_ids),
        timeline_excerpt_json = json_string_vec(&triage.timeline_excerpt),
        boundary_confidence_json = json_string(triage.boundary_confidence),
    )
}
