use super::*;

pub(crate) fn scenario_behavior_metadata(scenario: Scenario) -> ScenarioBehaviorMetadata {
    let spec = scenario_spec(scenario);
    let kind_metadata = scenario_behavior_kind_metadata(&spec.behavior);
    ScenarioBehaviorMetadata {
        run_strategy: kind_metadata.run_strategy,
        env_intents: kind_metadata.env_intents,
        typed_event_edges: scenario_typed_event_edges(scenario),
        typed_event_known_events: scenario_typed_event_known_events(scenario),
        evidence_selectors: scenario_evidence_selectors(scenario, &spec.behavior),
        non_claims: SCENARIO_METADATA_NON_CLAIMS,
        handler: kind_metadata.handler,
    }
}

pub(super) fn scenario_behavior_kind_metadata(
    behavior: &ScenarioBehaviorKind,
) -> ScenarioBehaviorKindMetadata {
    match behavior {
        ScenarioBehaviorKind::Default => ScenarioBehaviorKindMetadata {
            run_strategy: Some(ScenarioRunStrategy::SingleClient),
            env_intents: EMPTY_ENV_INTENTS,
            handler: HANDLER_DEFAULT,
        },
        ScenarioBehaviorKind::CompatBotProbe => ScenarioBehaviorKindMetadata {
            run_strategy: Some(ScenarioRunStrategy::SingleClient),
            env_intents: COMPAT_BOT_ENV_INTENTS,
            handler: HANDLER_COMPAT_BOT_PROBE,
        },
        ScenarioBehaviorKind::FlagScore { reconnect, .. } => ScenarioBehaviorKindMetadata {
            run_strategy: Some(ScenarioRunStrategy::SingleClient),
            env_intents: if *reconnect {
                FLAG_SCORE_RECONNECT_ENV_INTENTS
            } else {
                FLAG_SCORE_ENV_INTENTS
            },
            handler: HANDLER_FLAG_SCORE,
        },
        ScenarioBehaviorKind::ReconnectFlagState { negative_probe } => {
            ScenarioBehaviorKindMetadata {
                run_strategy: Some(ScenarioRunStrategy::ReconnectSequence),
                env_intents: if negative_probe.is_some() {
                    NEGATIVE_RECONNECT_FLAG_ENV_INTENTS
                } else {
                    RECONNECT_FLAG_ENV_INTENTS
                },
                handler: HANDLER_RECONNECT_FLAG_STATE,
            }
        }
        ScenarioBehaviorKind::InventoryInteraction => ScenarioBehaviorKindMetadata {
            run_strategy: Some(ScenarioRunStrategy::SingleClient),
            env_intents: INVENTORY_ENV_INTENTS,
            handler: HANDLER_INVENTORY_INTERACTION,
        },
        ScenarioBehaviorKind::InventoryStackSplitMerge => ScenarioBehaviorKindMetadata {
            run_strategy: Some(ScenarioRunStrategy::SingleClient),
            env_intents: INVENTORY_ENV_INTENTS,
            handler: HANDLER_INVENTORY_STACK,
        },
        ScenarioBehaviorKind::InventoryDragTransactions => ScenarioBehaviorKindMetadata {
            run_strategy: Some(ScenarioRunStrategy::SingleClient),
            env_intents: INVENTORY_ENV_INTENTS,
            handler: HANDLER_INVENTORY_DRAG,
        },
        ScenarioBehaviorKind::NegativeInventory { .. } => ScenarioBehaviorKindMetadata {
            run_strategy: Some(ScenarioRunStrategy::SingleClient),
            env_intents: NEGATIVE_INVENTORY_ENV_INTENTS,
            handler: HANDLER_NEGATIVE_INVENTORY,
        },
        ScenarioBehaviorKind::NegativeCustomPayload => ScenarioBehaviorKindMetadata {
            run_strategy: Some(ScenarioRunStrategy::SingleClient),
            env_intents: NEGATIVE_CUSTOM_PAYLOAD_ENV_INTENTS,
            handler: HANDLER_NEGATIVE_CUSTOM_PAYLOAD,
        },
        ScenarioBehaviorKind::SurvivalChestPersistence
        | ScenarioBehaviorKind::SurvivalFurnacePersistence => ScenarioBehaviorKindMetadata {
            run_strategy: Some(ScenarioRunStrategy::ReconnectSequence),
            env_intents: SURVIVAL_RECONNECT_ENV_INTENTS,
            handler: HANDLER_SURVIVAL,
        },
        ScenarioBehaviorKind::WorldPersistenceRestart { .. }
        | ScenarioBehaviorKind::SurvivalWorldMultichunkDurability => ScenarioBehaviorKindMetadata {
            run_strategy: Some(ScenarioRunStrategy::ReconnectSequence),
            env_intents: SURVIVAL_PERSISTENCE_ENV_INTENTS,
            handler: HANDLER_WORLD_PERSISTENCE,
        },
        ScenarioBehaviorKind::SurvivalBreakPlacePickup
        | ScenarioBehaviorKind::SurvivalCraftingTable
        | ScenarioBehaviorKind::SurvivalCraftingRecipeBreadth
        | ScenarioBehaviorKind::SurvivalFurnaceSmeltingBreadth
        | ScenarioBehaviorKind::SurvivalHungerFood
        | ScenarioBehaviorKind::SurvivalHungerHealthCycle
        | ScenarioBehaviorKind::SurvivalMobDrop
        | ScenarioBehaviorKind::SurvivalMobAiLootBreadth
        | ScenarioBehaviorKind::SurvivalRedstoneToggle
        | ScenarioBehaviorKind::SurvivalRedstoneCircuitBreadth
        | ScenarioBehaviorKind::SurvivalContainerBlockEntityBreadth
        | ScenarioBehaviorKind::SurvivalBiomeDimensionState
        | ScenarioBehaviorKind::SurvivalBiomeDimensionTravel
        | ScenarioBehaviorKind::SurvivalSignEditingLive => ScenarioBehaviorKindMetadata {
            run_strategy: Some(ScenarioRunStrategy::SingleClient),
            env_intents: SURVIVAL_ENV_INTENTS,
            handler: HANDLER_SURVIVAL,
        },
        ScenarioBehaviorKind::McpControlledSmoke => ScenarioBehaviorKindMetadata {
            run_strategy: Some(ScenarioRunStrategy::SingleClient),
            env_intents: MCP_CONTROL_ENV_INTENTS,
            handler: HANDLER_MCP_CONTROLLED_SMOKE,
        },
        ScenarioBehaviorKind::Combat { .. } => ScenarioBehaviorKindMetadata {
            run_strategy: Some(ScenarioRunStrategy::MultiClient),
            env_intents: COMBAT_ENV_INTENTS,
            handler: HANDLER_COMBAT,
        },
        ScenarioBehaviorKind::EquipmentUpdate => ScenarioBehaviorKindMetadata {
            run_strategy: Some(ScenarioRunStrategy::MultiClient),
            env_intents: EQUIPMENT_ENV_INTENTS,
            handler: HANDLER_EQUIPMENT_UPDATE,
        },
        ScenarioBehaviorKind::Projectile { .. } => ScenarioBehaviorKindMetadata {
            run_strategy: Some(ScenarioRunStrategy::MultiClient),
            env_intents: PROJECTILE_ENV_INTENTS,
            handler: HANDLER_PROJECTILE,
        },
        ScenarioBehaviorKind::MultiClientLoadScore => ScenarioBehaviorKindMetadata {
            run_strategy: Some(ScenarioRunStrategy::MultiClient),
            env_intents: MULTI_CLIENT_LOAD_ENV_INTENTS,
            handler: HANDLER_MULTI_CLIENT_LOAD,
        },
        ScenarioBehaviorKind::NegativeCtfWrongScore => ScenarioBehaviorKindMetadata {
            run_strategy: Some(ScenarioRunStrategy::SingleClient),
            env_intents: NEGATIVE_CTF_ENV_INTENTS,
            handler: HANDLER_NEGATIVE_CTF,
        },
        ScenarioBehaviorKind::CtfScoreLimitWinCondition => ScenarioBehaviorKindMetadata {
            run_strategy: Some(ScenarioRunStrategy::SingleClient),
            env_intents: CTF_SCORE_LIMIT_ENV_INTENTS,
            handler: HANDLER_CTF_RULE,
        },
        ScenarioBehaviorKind::CtfSimultaneousPickupCaptureRace => ScenarioBehaviorKindMetadata {
            run_strategy: Some(ScenarioRunStrategy::MultiClient),
            env_intents: CTF_RACE_ENV_INTENTS,
            handler: HANDLER_CTF_RULE,
        },
        ScenarioBehaviorKind::CtfSpawnTeamBalanceReset => ScenarioBehaviorKindMetadata {
            run_strategy: Some(ScenarioRunStrategy::MultiClient),
            env_intents: CTF_SPAWN_ENV_INTENTS,
            handler: HANDLER_CTF_RULE,
        },
        ScenarioBehaviorKind::CtfInvalidPickupOwnership
        | ScenarioBehaviorKind::CtfInvalidReturnDrop
        | ScenarioBehaviorKind::CtfInvalidOpponentBaseReturnDrop => ScenarioBehaviorKindMetadata {
            run_strategy: Some(ScenarioRunStrategy::SingleClient),
            env_intents: NEGATIVE_CTF_ENV_INTENTS,
            handler: HANDLER_CTF_RULE,
        },
    }
}

fn scenario_evidence_selectors(
    scenario: Scenario,
    behavior: &ScenarioBehaviorKind,
) -> ScenarioEvidenceSelectors {
    ScenarioEvidenceSelectors {
        typed_event_pass_fail: TYPED_EVENT_PASS_FAIL_SCENARIOS.contains(&scenario),
        typed_event_required_events: if scenario == Scenario::McpControlledSmoke {
            MCP_TYPED_EVENT_REQUIRED_EVENTS
        } else {
            EMPTY_TYPED_EVENT_REQUIRED_EVENTS
        },
        dynamic_projectile_health: matches!(
            behavior,
            ScenarioBehaviorKind::Projectile { damage: true }
        ),
        mcp_control: matches!(behavior, ScenarioBehaviorKind::McpControlledSmoke),
        negative_live_rail: behavior.negative_live_rail().is_some(),
    }
}

pub(super) fn scenario_evidence_selectors_for_kind(
    behavior: &ScenarioBehaviorKind,
) -> ScenarioEvidenceSelectors {
    ScenarioEvidenceSelectors {
        typed_event_pass_fail: false,
        typed_event_required_events: EMPTY_TYPED_EVENT_REQUIRED_EVENTS,
        dynamic_projectile_health: matches!(
            behavior,
            ScenarioBehaviorKind::Projectile { damage: true }
        ),
        mcp_control: matches!(behavior, ScenarioBehaviorKind::McpControlledSmoke),
        negative_live_rail: behavior.negative_live_rail().is_some(),
    }
}

fn scenario_typed_event_known_events(scenario: Scenario) -> &'static [&'static str] {
    if scenario == Scenario::McpControlledSmoke {
        MCP_TYPED_EVENT_KNOWN_EVENTS
    } else {
        EMPTY_TYPED_EVENT_KNOWN_EVENTS
    }
}

pub(crate) fn validate_scenario_behavior_metadata(
    spec: &ScenarioSpec,
    metadata: &ScenarioBehaviorMetadata,
) -> Result<(), String> {
    if metadata.run_strategy.is_none() {
        return Err(format!(
            "scenario {} missing run strategy metadata",
            spec.canonical_name
        ));
    }
    if !SUPPORTED_SCENARIO_HANDLERS.contains(&metadata.handler) {
        return Err(format!(
            "scenario {} references unsupported handler {}",
            spec.canonical_name, metadata.handler
        ));
    }
    for intent in metadata.env_intents {
        if !SUPPORTED_SCENARIO_ENV_INTENTS.contains(intent) {
            return Err(format!(
                "scenario {} has unknown env intent {}",
                spec.canonical_name, intent
            ));
        }
    }
    if metadata.non_claims.is_empty() {
        return Err(format!(
            "scenario {} missing metadata non-claims",
            spec.canonical_name
        ));
    }
    validate_typed_event_edges(spec, metadata)
}

fn validate_typed_event_edges(
    spec: &ScenarioSpec,
    metadata: &ScenarioBehaviorMetadata,
) -> Result<(), String> {
    let valid_events = valid_typed_event_ids(spec, metadata);
    for (before, after) in &metadata.typed_event_edges {
        if before.is_empty() || after.is_empty() || before == after {
            return Err(format!(
                "scenario {} has invalid graph edge {before:?}->{after:?}",
                spec.canonical_name
            ));
        }
        if !valid_events.contains(before) || !valid_events.contains(after) {
            return Err(format!(
                "scenario {} has invalid graph edge {before}->{after}",
                spec.canonical_name
            ));
        }
    }
    Ok(())
}

fn valid_typed_event_ids(
    spec: &ScenarioSpec,
    metadata: &ScenarioBehaviorMetadata,
) -> Vec<&'static str> {
    let mut valid_events = scenario_milestone_ids(spec.client_milestones);
    valid_events.extend(scenario_milestone_ids(spec.server_milestones));
    valid_events.extend(
        metadata
            .evidence_selectors
            .typed_event_required_events
            .iter()
            .copied(),
    );
    valid_events.extend(metadata.typed_event_known_events.iter().copied());
    valid_events
}

fn scenario_typed_event_edges(scenario: Scenario) -> Vec<ScenarioBehaviorEdge> {
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
