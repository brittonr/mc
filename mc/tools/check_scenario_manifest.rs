use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::io::ErrorKind;
use std::path::Path;
use std::process::ExitCode;

const MANIFEST_PATH: &str = "compat/config/scenario-manifest.ncl";
const FALLBACK_BUDGET_BASELINE_PATH: &str = "compat/config/scenario-fallback-budget-baseline.ncl";
const GENERATED_RUST_PATH: &str = "compat/runner/src/scenario_manifest_generated.rs";
const RUNNER_MAIN_PATH: &str = "compat/runner/src/main.rs";
const RUNNER_SCENARIO_CORE_PATH: &str = "compat/runner/src/scenario_core.rs";
const RUNNER_SURFACE_PATH: &str = "compat/runner/src/{main.rs,scenario_core.rs}";
const FLAKE_PATH: &str = "flake.nix";
const NIX_APPS_PATH: &str = "nix/apps.nix";
const NIX_PACKAGES_PATH: &str = "nix/packages.nix";
const NIX_CHECKS_PATH: &str = "nix/checks.nix";
const README_PATH: &str = "README.md";
const SCENARIO_COMMANDS_DOC_PATH: &str = "docs/scenario-commands.md";
const EVIDENCE_WORKFLOW_DOC_PATH: &str = "docs/evidence-workflow.md";
const CONFIGURATION_DOC_PATH: &str = "docs/configuration.md";
const VERIFICATION_DOC_PATH: &str = "docs/verification.md";
const SURFACE_INVENTORY_PATH: &str = "docs/scenario-derived-surfaces.md";
const CURRENT_BUNDLE_PATH: &str = "docs/evidence/protocol-763-current-evidence-bundle.md";
const GENERATED_SCENARIO_INDEX_PATH: &str = "docs/evidence/mc-compat-scenario-index.generated.md";
const GENERATED_SCENARIO_COMMANDS_PATH: &str = "docs/scenario-commands.generated.md";
const GENERATED_WRAPPER_METADATA_PATH: &str =
    "compat/config/generated/scenario-wrapper-metadata.nix";
const SUPPORTED_SCHEMA: &str = "mc.compat.scenario-manifest.v1";
const FALLBACK_BUDGET_BASELINE_SCHEMA: &str = "mc.compat.scenario-fallback-budget-baseline.v1";
const SUBSTRING_FALLBACK_MIGRATION: &str = "substring-fallback";
const TYPED_EVENT_READY_MIGRATION: &str = "typed-event-ready";
const SELF_TEST_FLAG: &str = "--self-test";
const CHECK_GENERATED_SURFACES_FLAG: &str = "--check-generated-surfaces";
const WRITE_GENERATED_SURFACES_FLAG: &str = "--write-generated-surfaces";
const MINIMUM_POSITIVE_COUNT: u32 = 1;
const GENERATED_RUST_MAX_WIDTH: usize = 100;
const GENERATED_RUST_COLLECTION_WIDTH: usize = 60;
const GENERATED_RUST_TRAILING_COMMA_WIDTH: usize = 1;
const STRING_QUOTE_OVERHEAD: usize = 2;
const STRING_FIELD_DELIMITER: &str = " = \"";
const ARRAY_START: &str = "[";
const ARRAY_END: &str = "]";
const ROW_START: &str = "{";
const ROW_END: &str = "},";
const SCENARIOS_START: &str = "scenarios = [";
const SCENARIOS_END: &str = "],";
const FALLBACK_BUDGET_FALLBACK_ROWS_START: &str = "fallback_rows = [";
const FALLBACK_BUDGET_TYPED_READY_FIELD: &str = "typed_event_ready_rows";
const COMMON_WAIVER_REFERENCE_PREFIX: &str = "common_waiver.";
const EXIT_SUCCESS: ExitCode = ExitCode::SUCCESS;
const EXIT_FAILURE: ExitCode = ExitCode::FAILURE;
const LIVE_CAPABILITY_REGISTRY_TOKENS: &[&str] = &[
    "ScenarioLiveCapability",
    "CreativeInventoryLiveContract",
    "CREATIVE_INVENTORY_LIVE_CONTRACT",
    "validate_creative_inventory_live_contract",
    "ResourcePackStatusLocalContract",
    "RESOURCE_PACK_STATUS_LOCAL_CONTRACT",
    "validate_resource_pack_status_local_contract",
    "SignEditorLiveContract",
    "SIGN_EDITOR_LIVE_CONTRACT",
    "validate_sign_editor_live_contract",
    "SCENARIO_LIVE_CAPABILITIES",
    "validate_static_live_capabilities",
    "targeted-packet-live-blocker",
    "fixture-bounded-blocker",
    "creative-inventory-action",
    "creative_slot_mutation_accepted",
    "resource-pack-status",
    "resource_pack_status_declined_observed",
    "sign-editor-open-update",
    "sign_update_accepted_observed",
];
const WAIVER_OWNER_FIELD: &str = "owner=";
const WAIVER_REASON_FIELD: &str = "reason=";
const WAIVER_NON_CLAIM_FIELD: &str = "non_claim=";
const WAIVER_NEXT_ACTION_FIELD: &str = "next_action=";
const REQUIRED_WAIVER_FIELDS: &[&str] = &[
    WAIVER_OWNER_FIELD,
    WAIVER_REASON_FIELD,
    WAIVER_NON_CLAIM_FIELD,
    WAIVER_NEXT_ACTION_FIELD,
];
const STALE_DRY_RUN_EXCLUSION_MARKERS: &[&str] = &[
    "not yet by a dedicated",
    "instead of a dedicated dry-run wrapper",
    "instead of a dry-run wrapper",
];
const TYPED_EVENT_FALLBACK_WAIVER_FIELD: &str = "typed_event_fallback_waiver";
const TYPED_EVENT_COMMON_FORBIDDEN_EVENTS: &[&str] = &[
    "panic",
    "unexpected_eof",
    "protocol_mismatch",
    "decode_error",
];
const WRAPPER_METADATA_FIELDS: &[&str] = &[
    "scenario",
    "aliases",
    "appWrapper",
    "dryRunCheck",
    "receiptShapeCheck",
    "clientCount",
    "sessionCount",
    "migrationState",
];
const FLAKE_SURFACE_PATHS: &[&str] = &[
    FLAKE_PATH,
    NIX_APPS_PATH,
    NIX_PACKAGES_PATH,
    NIX_CHECKS_PATH,
];
const README_REQUIRED_DOC_LINKS: &[&str] = &[
    SCENARIO_COMMANDS_DOC_PATH,
    EVIDENCE_WORKFLOW_DOC_PATH,
    CONFIGURATION_DOC_PATH,
    VERIFICATION_DOC_PATH,
    "docs/check-tiers.md",
];
const REQUIRED_SURFACE_INVENTORY_TOKENS: &[&str] = &[
    MANIFEST_PATH,
    FALLBACK_BUDGET_BASELINE_PATH,
    GENERATED_RUST_PATH,
    GENERATED_WRAPPER_METADATA_PATH,
    GENERATED_SCENARIO_INDEX_PATH,
    GENERATED_SCENARIO_COMMANDS_PATH,
    RUNNER_MAIN_PATH,
    RUNNER_SCENARIO_CORE_PATH,
    FLAKE_PATH,
    NIX_CHECKS_PATH,
    README_PATH,
    SCENARIO_COMMANDS_DOC_PATH,
    CURRENT_BUNDLE_PATH,
    "Generated",
    "Human-authored",
    "Intentionally duplicated",
];
const TYPED_EVENT_EMPTY_EVENTS: &[&str] = &[];
const SMOKE_TYPED_EVENT_CLIENT_EVENTS: &[&str] = &["protocol_detected"];
const MCP_CONTROLLED_SMOKE_TYPED_EVENT_CLIENT_EVENTS: &[&str] = &[
    "mcp_initialize",
    "mcp_tools_list",
    "mcp_status_call",
    "mcp_command_outcomes",
    "mcp_stdout_clean",
    "mcp_look_call",
    "mcp_input_call",
    "mcp_capture_latest_frame",
    "mcp_frame_artifact_identity",
];
const INVENTORY_TYPED_EVENT_CLIENT_EVENTS: &[&str] = &[
    "protocol_detected",
    "join_game",
    "render_tick",
    "team_red",
    "inventory_slot_update",
    "inventory_sword_slot",
    "inventory_wool_slot",
    "inventory_drop_sent",
    "inventory_pickup_seen",
    "inventory_click_sent",
    "inventory_open_container_seen",
    "inventory_container_click_sent",
    "inventory_block_place_sent",
];
const INVENTORY_TYPED_EVENT_SERVER_EVENTS: &[&str] = &[
    "server_username_seen",
    "server_inventory_hotbar_select",
    "server_inventory_drop",
    "server_inventory_pickup",
    "server_inventory_click",
    "server_inventory_open_container",
    "server_inventory_container_click",
    "server_block_place",
];
const INVENTORY_STACK_TYPED_EVENT_CLIENT_EVENTS: &[&str] = &[
    "protocol_detected",
    "join_game",
    "render_tick",
    "team_red",
    "inventory_stack_initial_slot",
    "inventory_stack_split_pickup_sent",
    "inventory_stack_split_source_seen",
    "inventory_stack_split_place_sent",
    "inventory_stack_destination_seen",
    "inventory_stack_merge_pickup_sent",
    "inventory_stack_merge_destination_empty_seen",
    "inventory_stack_merge_place_sent",
    "inventory_stack_final_source_seen",
];
const INVENTORY_STACK_TYPED_EVENT_SERVER_EVENTS: &[&str] = &[
    "server_username_seen",
    "server_inventory_stack_split_pickup",
    "server_inventory_stack_split",
    "server_inventory_stack_merge_pickup",
    "server_inventory_stack_merge",
];
const INVENTORY_DRAG_TYPED_EVENT_CLIENT_EVENTS: &[&str] = &[
    "protocol_detected",
    "join_game",
    "render_tick",
    "team_red",
    "inventory_drag_initial_slot",
    "inventory_drag_pickup_sent",
    "inventory_drag_source_empty_seen",
    "inventory_drag_start_sent",
    "inventory_drag_target_a_sent",
    "inventory_drag_target_b_sent",
    "inventory_drag_end_sent",
    "inventory_drag_final_distribution_seen",
];
const INVENTORY_DRAG_TYPED_EVENT_SERVER_EVENTS: &[&str] = &[
    "server_username_seen",
    "server_inventory_drag_pickup",
    "server_inventory_drag_start",
    "server_inventory_drag_target_a",
    "server_inventory_drag_target_b",
    "server_inventory_drag_end",
];
const SURVIVAL_BREAK_PLACE_TYPED_EVENT_CLIENT_EVENTS: &[&str] = &[
    "protocol_detected",
    "join_game",
    "render_tick",
    "survival_break_sent",
    "survival_break_update",
    "survival_pickup_seen",
    "survival_place_sent",
    "survival_place_update",
];
const SURVIVAL_BREAK_PLACE_TYPED_EVENT_SERVER_EVENTS: &[&str] = &[
    "server_username_seen",
    "server_survival_join",
    "server_survival_break",
    "server_survival_pickup",
    "server_survival_place",
];
const SURVIVAL_CHEST_TYPED_EVENT_CLIENT_EVENTS: &[&str] = &[
    "protocol_detected",
    "join_game",
    "render_tick",
    "survival_chest_open_seen",
    "survival_chest_store_sent",
    "survival_chest_close_sent",
    "survival_chest_reconnect_sent",
    "survival_chest_reopen_seen",
    "survival_chest_persisted_seen",
];
const SURVIVAL_CHEST_TYPED_EVENT_SERVER_EVENTS: &[&str] = &[
    "server_username_seen",
    "server_survival_chest_open",
    "server_survival_chest_store",
    "server_survival_chest_close",
    "server_survival_chest_reopen",
    "server_survival_chest_persisted",
];
const SURVIVAL_FURNACE_PERSISTENCE_TYPED_EVENT_CLIENT_EVENTS: &[&str] = &[
    "protocol_detected",
    "join_game",
    "render_tick",
    "survival_furnace_open_seen",
    "survival_furnace_input_sent",
    "survival_furnace_fuel_sent",
    "survival_furnace_burn_progress_seen",
    "survival_furnace_output_seen",
    "survival_furnace_output_collected",
    "survival_furnace_inventory_updated",
    "survival_furnace_reconnect_sent",
    "survival_furnace_reopen_seen",
];
const SURVIVAL_FURNACE_PERSISTENCE_TYPED_EVENT_SERVER_EVENTS: &[&str] = &[
    "server_username_seen",
    "server_survival_furnace_open",
    "server_survival_furnace_input",
    "server_survival_furnace_fuel",
    "server_survival_furnace_burn_progress",
    "server_survival_furnace_output_available",
    "server_survival_furnace_output_collect",
    "server_survival_furnace_reconnect_reopen",
    "server_survival_furnace_state",
];
const SURVIVAL_FURNACE_SMELTING_BREADTH_TYPED_EVENT_CLIENT_EVENTS: &[&str] = &[
    "protocol_detected",
    "join_game",
    "render_tick",
    "survival_furnace_open_seen",
    "survival_furnace_input_sent",
    "survival_furnace_fuel_sent",
    "survival_furnace_burn_progress_seen",
    "survival_furnace_output_seen",
    "survival_furnace_output_collected",
    "survival_furnace_inventory_updated",
    "survival_furnace_invalid_fuel_sent",
];
const SURVIVAL_FURNACE_SMELTING_BREADTH_TYPED_EVENT_SERVER_EVENTS: &[&str] = &[
    "server_username_seen",
    "server_survival_furnace_open",
    "server_survival_furnace_input",
    "server_survival_furnace_fuel",
    "server_survival_furnace_burn_progress",
    "server_survival_furnace_output_available",
    "server_survival_furnace_output_collect",
    "server_survival_furnace_invalid_fuel_rejected",
    "server_survival_furnace_breadth_state",
];
const SURVIVAL_CRAFTING_TABLE_TYPED_EVENT_CLIENT_EVENTS: &[&str] = &[
    "protocol_detected",
    "join_game",
    "render_tick",
    "survival_crafting_table_open_seen",
    "survival_crafting_input_a_sent",
    "survival_crafting_input_b_sent",
    "survival_crafting_result_seen",
    "survival_crafting_result_collected",
    "survival_crafting_inventory_updated",
];
const SURVIVAL_CRAFTING_TABLE_TYPED_EVENT_SERVER_EVENTS: &[&str] = &[
    "server_username_seen",
    "server_survival_crafting_table_open",
    "server_survival_crafting_input_a",
    "server_survival_crafting_input_b",
    "server_survival_crafting_result",
    "server_survival_crafting_collect",
];
const SURVIVAL_CRAFTING_RECIPE_BREADTH_TYPED_EVENT_CLIENT_EVENTS: &[&str] = &[
    "protocol_detected",
    "join_game",
    "render_tick",
    "survival_crafting_breadth_shaped_seen",
    "survival_crafting_breadth_shapeless_seen",
    "survival_crafting_breadth_grid_clear_seen",
    "survival_crafting_breadth_invalid_seen",
    "survival_crafting_breadth_inventory_updated",
];
const SURVIVAL_CRAFTING_RECIPE_BREADTH_TYPED_EVENT_SERVER_EVENTS: &[&str] = &[
    "server_username_seen",
    "server_survival_crafting_breadth_shaped",
    "server_survival_crafting_breadth_shapeless",
    "server_survival_crafting_breadth_grid_clear",
    "server_survival_crafting_breadth_invalid_rejected",
    "server_survival_crafting_breadth_state",
];
const SURVIVAL_HUNGER_FOOD_TYPED_EVENT_CLIENT_EVENTS: &[&str] = &[
    "protocol_detected",
    "join_game",
    "render_tick",
    "survival_hunger_food_item_seen",
    "survival_hunger_food_pre_seen",
    "survival_hunger_food_use_sent",
    "survival_hunger_food_post_seen",
    "survival_hunger_food_inventory_updated",
];
const SURVIVAL_HUNGER_FOOD_TYPED_EVENT_SERVER_EVENTS: &[&str] = &[
    "server_username_seen",
    "server_survival_hunger_food_pre",
    "server_survival_hunger_food_consume_start",
    "server_survival_hunger_food_consume_finish",
    "server_survival_hunger_food_inventory",
    "server_survival_hunger_food_state",
];
const SURVIVAL_HUNGER_HEALTH_CYCLE_TYPED_EVENT_CLIENT_EVENTS: &[&str] = &[
    "protocol_detected",
    "join_game",
    "render_tick",
    "survival_hunger_health_item_seen",
    "survival_hunger_health_pre_seen",
    "survival_hunger_health_consume_sent",
    "survival_hunger_health_recovery_seen",
    "survival_hunger_health_inventory_updated",
];
const SURVIVAL_HUNGER_HEALTH_CYCLE_TYPED_EVENT_SERVER_EVENTS: &[&str] = &[
    "server_username_seen",
    "server_survival_hunger_health_pre",
    "server_survival_hunger_health_consume_start",
    "server_survival_hunger_health_consume_finish",
    "server_survival_hunger_health_inventory",
    "server_survival_hunger_health_state",
];
const SURVIVAL_MOB_DROP_TYPED_EVENT_CLIENT_EVENTS: &[&str] = &[
    "protocol_detected",
    "join_game",
    "render_tick",
    "survival_mob_drop_mob_seen",
    "survival_mob_drop_attack_sent",
    "survival_mob_drop_death_seen",
    "survival_mob_drop_drop_seen",
    "survival_mob_drop_pickup_seen",
    "survival_mob_drop_inventory_updated",
];
const SURVIVAL_MOB_DROP_TYPED_EVENT_SERVER_EVENTS: &[&str] = &[
    "server_username_seen",
    "server_survival_mob_drop_spawn",
    "server_survival_mob_drop_attack",
    "server_survival_mob_drop_death",
    "server_survival_mob_drop_drop_spawn",
    "server_survival_mob_drop_pickup",
    "server_survival_mob_drop_inventory",
    "server_survival_mob_drop_state",
];
const SURVIVAL_MOB_AI_LOOT_BREADTH_TYPED_EVENT_CLIENT_EVENTS: &[&str] = &[
    "protocol_detected",
    "join_game",
    "render_tick",
    "survival_mob_ai_loot_mob_seen",
    "survival_mob_ai_loot_attack_sent",
    "survival_mob_ai_loot_death_seen",
    "survival_mob_ai_loot_drop_seen",
    "survival_mob_ai_loot_pickup_seen",
    "survival_mob_ai_loot_inventory_updated",
];
const SURVIVAL_MOB_AI_LOOT_BREADTH_TYPED_EVENT_SERVER_EVENTS: &[&str] = &[
    "server_username_seen",
    "server_survival_mob_ai_loot_spawn",
    "server_survival_mob_ai_loot_ai_checkpoint",
    "server_survival_mob_ai_loot_attack",
    "server_survival_mob_ai_loot_death",
    "server_survival_mob_ai_loot_drop_spawn",
    "server_survival_mob_ai_loot_pickup",
    "server_survival_mob_ai_loot_inventory",
    "server_survival_mob_ai_loot_state",
];
const SURVIVAL_REDSTONE_TOGGLE_TYPED_EVENT_CLIENT_EVENTS: &[&str] = &[
    "protocol_detected",
    "join_game",
    "render_tick",
    "survival_redstone_toggle_input_sent",
    "survival_redstone_toggle_output_update",
    "survival_redstone_toggle_return_input_sent",
    "survival_redstone_toggle_return_update",
];
const SURVIVAL_REDSTONE_TOGGLE_TYPED_EVENT_SERVER_EVENTS: &[&str] = &[
    "server_username_seen",
    "server_survival_redstone_toggle_input",
    "server_survival_redstone_toggle_powered_on",
    "server_survival_redstone_toggle_powered_off",
    "server_survival_redstone_toggle_state",
];
const SURVIVAL_REDSTONE_CIRCUIT_BREADTH_TYPED_EVENT_CLIENT_EVENTS: &[&str] = &[
    "protocol_detected",
    "join_game",
    "render_tick",
    "survival_redstone_circuit_initial_state",
    "survival_redstone_circuit_input_sent",
    "survival_redstone_circuit_output_update",
    "survival_redstone_circuit_return_input_sent",
    "survival_redstone_circuit_return_update",
];
const SURVIVAL_REDSTONE_CIRCUIT_BREADTH_TYPED_EVENT_SERVER_EVENTS: &[&str] = &[
    "server_username_seen",
    "server_survival_redstone_circuit_initial",
    "server_survival_redstone_circuit_input",
    "server_survival_redstone_circuit_powered_on",
    "server_survival_redstone_circuit_powered_off",
    "server_survival_redstone_circuit_state",
];
const SURVIVAL_WORLD_MULTICHUNK_DURABILITY_TYPED_EVENT_CLIENT_EVENTS: &[&str] = &[
    "protocol_detected",
    "join_game",
    "render_tick",
    "survival_world_multichunk_mutation_sent",
    "survival_world_multichunk_pre_restart_update",
    "survival_world_multichunk_reconnect_sent",
    "survival_world_multichunk_post_restart_update",
];
const SURVIVAL_WORLD_MULTICHUNK_DURABILITY_TYPED_EVENT_SERVER_EVENTS: &[&str] = &[
    "server_username_seen",
    "server_survival_world_multichunk_mutation",
    "server_survival_world_multichunk_clean_shutdown",
    "server_survival_world_multichunk_backend_restart",
    "server_survival_world_multichunk_post_restart",
    "server_survival_world_multichunk_state",
];
const SURVIVAL_CONTAINER_BLOCK_ENTITY_BREADTH_TYPED_EVENT_CLIENT_EVENTS: &[&str] = &[
    "protocol_detected",
    "join_game",
    "render_tick",
    "survival_container_block_entity_open_seen",
    "survival_container_block_entity_transfer_sent",
    "survival_container_block_entity_payload_seen",
    "survival_container_block_entity_metadata_seen",
    "survival_container_block_entity_reopen_seen",
];
const SURVIVAL_CONTAINER_BLOCK_ENTITY_BREADTH_TYPED_EVENT_SERVER_EVENTS: &[&str] = &[
    "server_username_seen",
    "server_survival_container_block_entity_open",
    "server_survival_container_block_entity_transfer",
    "server_survival_container_block_entity_payload",
    "server_survival_container_block_entity_metadata",
    "server_survival_container_block_entity_state",
];
const SURVIVAL_BIOME_DIMENSION_STATE_TYPED_EVENT_CLIENT_EVENTS: &[&str] = &[
    "protocol_detected",
    "join_game",
    "render_tick",
    "survival_biome_dimension_state",
];
const SURVIVAL_BIOME_DIMENSION_STATE_TYPED_EVENT_SERVER_EVENTS: &[&str] = &[
    "server_username_seen",
    "server_survival_biome_dimension_state",
];
const SURVIVAL_BIOME_DIMENSION_TRAVEL_TYPED_EVENT_CLIENT_EVENTS: &[&str] = &[
    "protocol_detected",
    "join_game",
    "render_tick",
    "survival_biome_dimension_travel_origin",
    "survival_biome_dimension_travel_transition_sent",
    "survival_biome_dimension_travel_destination_seen",
];
const SURVIVAL_BIOME_DIMENSION_TRAVEL_TYPED_EVENT_SERVER_EVENTS: &[&str] = &[
    "server_username_seen",
    "server_survival_biome_dimension_travel_origin",
    "server_survival_biome_dimension_travel_transition",
    "server_survival_biome_dimension_travel_state",
];
const SURVIVAL_SIGN_EDITING_LIVE_TYPED_EVENT_CLIENT_EVENTS: &[&str] = &[
    "protocol_detected",
    "join_game",
    "render_tick",
    "survival_sign_editing_open_seen",
    "survival_sign_editing_update_sent",
    "survival_sign_editing_post_update_seen",
];
const SURVIVAL_SIGN_EDITING_LIVE_TYPED_EVENT_SERVER_EVENTS: &[&str] = &[
    "server_username_seen",
    "server_survival_sign_editing_open",
    "server_survival_sign_editing_update_accepted",
    "server_survival_sign_editing_state",
];
const CTF_RULE_TYPED_EVENT_CLIENT_EVENTS: &[&str] = &[
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
    "multi_client_count",
    "remote_player_spawn",
    "combat_attack_sent",
    "combat_health_update",
    "combat_velocity_update",
    "armor_inventory_slot",
    "entity_equipment_update",
    "projectile_use_sent",
    "projectile_spawn_visible",
    "projectile_swing_sent",
    "projectile_travel_observed",
    "projectile_damage_update",
    "combat_death_observed",
    "respawn_request_sent",
    "respawn_health_restored",
    "reconnect_session",
    "ctf_invalid_pickup_attempted",
    "ctf_invalid_pickup_contained",
    "ctf_invalid_return_drop_attempted",
    "ctf_invalid_return_drop_contained",
    "ctf_invalid_opponent_base_return_drop_attempted",
    "ctf_invalid_opponent_base_return_drop_contained",
    "ctf_score_limit_win_seen",
    "ctf_race_client_count",
    "ctf_spawn_team_reset_client_count",
];
const CTF_RULE_TYPED_EVENT_SERVER_EVENTS: &[&str] = &[
    "server_username_seen",
    "server_client_a_seen",
    "server_client_b_seen",
    "server_flag_or_score",
    "server_combat_damage",
    "server_combat_knockback",
    "server_equipment_state",
    "server_armor_mitigation",
    "server_equipment_update_state",
    "server_projectile_loadout",
    "server_projectile_use",
    "server_projectile_travel_sample",
    "server_projectile_collision",
    "server_projectile_hit",
    "server_flag_pickup",
    "server_flag_carrier_death",
    "server_flag_return",
    "server_flag_disconnect_return",
    "server_reconnect_state_coherent",
    "server_invalid_pickup_rejected",
    "server_invalid_return_drop_rejected",
    "server_invalid_opponent_base_return_drop_rejected",
    "server_score_limit_pre_state",
    "server_score_limit_final_capture",
    "server_score_limit_win_condition",
    "server_ctf_race_accepted_transition",
    "server_ctf_race_rejected_transition",
    "server_ctf_race_final_state",
    "server_ctf_spawn_red_assignment",
    "server_ctf_spawn_blue_assignment",
    "server_ctf_spawn_team_balance",
    "server_ctf_spawn_resource_reset",
];
const CTF_RULE_TYPED_EVENT_FORBIDDEN_EVENTS: &[&str] = &[
    "panic",
    "unexpected_eof",
    "protocol_mismatch",
    "decode_error",
    "unexpected_flag_capture",
    "unexpected_flag_capture_milestone",
    "unexpected_red_score",
    "unexpected_blue_score",
    "unexpected_flag_pickup_chat",
    "unexpected_flag_pickup_milestone",
    "unexpected_server_flag_pickup",
    "unexpected_flag_return",
    "unexpected_flag_disconnect_return",
    "score_limit_duplicate_win",
    "score_limit_post_win_score_mutation",
    "unexpected_red_score_3",
    "unexpected_blue_score_1",
    "ctf_race_double_accept",
    "unexpected_red_score_2",
    "spawn_team_imbalance",
    "spawn_resource_stale",
];
const TYPED_EVENT_READINESS_FIXTURES: &[TypedEventReadinessFixture<'static>] = &[
    TypedEventReadinessFixture {
        scenario: "smoke",
        client_events: SMOKE_TYPED_EVENT_CLIENT_EVENTS,
        server_events: TYPED_EVENT_EMPTY_EVENTS,
        forbidden_events: TYPED_EVENT_COMMON_FORBIDDEN_EVENTS,
        derivation_rules: TYPED_EVENT_EMPTY_EVENTS,
    },
    TypedEventReadinessFixture {
        scenario: "mcp-controlled-smoke",
        client_events: MCP_CONTROLLED_SMOKE_TYPED_EVENT_CLIENT_EVENTS,
        server_events: TYPED_EVENT_EMPTY_EVENTS,
        forbidden_events: TYPED_EVENT_COMMON_FORBIDDEN_EVENTS,
        derivation_rules: TYPED_EVENT_EMPTY_EVENTS,
    },
    TypedEventReadinessFixture {
        scenario: "inventory-interaction",
        client_events: INVENTORY_TYPED_EVENT_CLIENT_EVENTS,
        server_events: INVENTORY_TYPED_EVENT_SERVER_EVENTS,
        forbidden_events: TYPED_EVENT_COMMON_FORBIDDEN_EVENTS,
        derivation_rules: TYPED_EVENT_EMPTY_EVENTS,
    },
    TypedEventReadinessFixture {
        scenario: "inventory-stack-split-merge",
        client_events: INVENTORY_STACK_TYPED_EVENT_CLIENT_EVENTS,
        server_events: INVENTORY_STACK_TYPED_EVENT_SERVER_EVENTS,
        forbidden_events: TYPED_EVENT_COMMON_FORBIDDEN_EVENTS,
        derivation_rules: TYPED_EVENT_EMPTY_EVENTS,
    },
    TypedEventReadinessFixture {
        scenario: "inventory-drag-transactions",
        client_events: INVENTORY_DRAG_TYPED_EVENT_CLIENT_EVENTS,
        server_events: INVENTORY_DRAG_TYPED_EVENT_SERVER_EVENTS,
        forbidden_events: TYPED_EVENT_COMMON_FORBIDDEN_EVENTS,
        derivation_rules: TYPED_EVENT_EMPTY_EVENTS,
    },
    TypedEventReadinessFixture {
        scenario: "survival-break-place-pickup",
        client_events: SURVIVAL_BREAK_PLACE_TYPED_EVENT_CLIENT_EVENTS,
        server_events: SURVIVAL_BREAK_PLACE_TYPED_EVENT_SERVER_EVENTS,
        forbidden_events: TYPED_EVENT_COMMON_FORBIDDEN_EVENTS,
        derivation_rules: TYPED_EVENT_EMPTY_EVENTS,
    },
    TypedEventReadinessFixture {
        scenario: "survival-chest-persistence",
        client_events: SURVIVAL_CHEST_TYPED_EVENT_CLIENT_EVENTS,
        server_events: SURVIVAL_CHEST_TYPED_EVENT_SERVER_EVENTS,
        forbidden_events: TYPED_EVENT_COMMON_FORBIDDEN_EVENTS,
        derivation_rules: TYPED_EVENT_EMPTY_EVENTS,
    },
    TypedEventReadinessFixture {
        scenario: "survival-furnace-persistence",
        client_events: SURVIVAL_FURNACE_PERSISTENCE_TYPED_EVENT_CLIENT_EVENTS,
        server_events: SURVIVAL_FURNACE_PERSISTENCE_TYPED_EVENT_SERVER_EVENTS,
        forbidden_events: TYPED_EVENT_COMMON_FORBIDDEN_EVENTS,
        derivation_rules: TYPED_EVENT_EMPTY_EVENTS,
    },
    TypedEventReadinessFixture {
        scenario: "survival-furnace-smelting-breadth",
        client_events: SURVIVAL_FURNACE_SMELTING_BREADTH_TYPED_EVENT_CLIENT_EVENTS,
        server_events: SURVIVAL_FURNACE_SMELTING_BREADTH_TYPED_EVENT_SERVER_EVENTS,
        forbidden_events: TYPED_EVENT_COMMON_FORBIDDEN_EVENTS,
        derivation_rules: TYPED_EVENT_EMPTY_EVENTS,
    },
    TypedEventReadinessFixture {
        scenario: "survival-crafting-table",
        client_events: SURVIVAL_CRAFTING_TABLE_TYPED_EVENT_CLIENT_EVENTS,
        server_events: SURVIVAL_CRAFTING_TABLE_TYPED_EVENT_SERVER_EVENTS,
        forbidden_events: TYPED_EVENT_COMMON_FORBIDDEN_EVENTS,
        derivation_rules: TYPED_EVENT_EMPTY_EVENTS,
    },
    TypedEventReadinessFixture {
        scenario: "survival-crafting-recipe-breadth",
        client_events: SURVIVAL_CRAFTING_RECIPE_BREADTH_TYPED_EVENT_CLIENT_EVENTS,
        server_events: SURVIVAL_CRAFTING_RECIPE_BREADTH_TYPED_EVENT_SERVER_EVENTS,
        forbidden_events: TYPED_EVENT_COMMON_FORBIDDEN_EVENTS,
        derivation_rules: TYPED_EVENT_EMPTY_EVENTS,
    },
    TypedEventReadinessFixture {
        scenario: "survival-hunger-food",
        client_events: SURVIVAL_HUNGER_FOOD_TYPED_EVENT_CLIENT_EVENTS,
        server_events: SURVIVAL_HUNGER_FOOD_TYPED_EVENT_SERVER_EVENTS,
        forbidden_events: TYPED_EVENT_COMMON_FORBIDDEN_EVENTS,
        derivation_rules: TYPED_EVENT_EMPTY_EVENTS,
    },
    TypedEventReadinessFixture {
        scenario: "survival-hunger-health-cycle",
        client_events: SURVIVAL_HUNGER_HEALTH_CYCLE_TYPED_EVENT_CLIENT_EVENTS,
        server_events: SURVIVAL_HUNGER_HEALTH_CYCLE_TYPED_EVENT_SERVER_EVENTS,
        forbidden_events: TYPED_EVENT_COMMON_FORBIDDEN_EVENTS,
        derivation_rules: TYPED_EVENT_EMPTY_EVENTS,
    },
    TypedEventReadinessFixture {
        scenario: "survival-mob-drop",
        client_events: SURVIVAL_MOB_DROP_TYPED_EVENT_CLIENT_EVENTS,
        server_events: SURVIVAL_MOB_DROP_TYPED_EVENT_SERVER_EVENTS,
        forbidden_events: TYPED_EVENT_COMMON_FORBIDDEN_EVENTS,
        derivation_rules: TYPED_EVENT_EMPTY_EVENTS,
    },
    TypedEventReadinessFixture {
        scenario: "survival-mob-ai-loot-breadth",
        client_events: SURVIVAL_MOB_AI_LOOT_BREADTH_TYPED_EVENT_CLIENT_EVENTS,
        server_events: SURVIVAL_MOB_AI_LOOT_BREADTH_TYPED_EVENT_SERVER_EVENTS,
        forbidden_events: TYPED_EVENT_COMMON_FORBIDDEN_EVENTS,
        derivation_rules: TYPED_EVENT_EMPTY_EVENTS,
    },
    TypedEventReadinessFixture {
        scenario: "survival-redstone-toggle",
        client_events: SURVIVAL_REDSTONE_TOGGLE_TYPED_EVENT_CLIENT_EVENTS,
        server_events: SURVIVAL_REDSTONE_TOGGLE_TYPED_EVENT_SERVER_EVENTS,
        forbidden_events: TYPED_EVENT_COMMON_FORBIDDEN_EVENTS,
        derivation_rules: TYPED_EVENT_EMPTY_EVENTS,
    },
    TypedEventReadinessFixture {
        scenario: "survival-redstone-circuit-breadth",
        client_events: SURVIVAL_REDSTONE_CIRCUIT_BREADTH_TYPED_EVENT_CLIENT_EVENTS,
        server_events: SURVIVAL_REDSTONE_CIRCUIT_BREADTH_TYPED_EVENT_SERVER_EVENTS,
        forbidden_events: TYPED_EVENT_COMMON_FORBIDDEN_EVENTS,
        derivation_rules: TYPED_EVENT_EMPTY_EVENTS,
    },
    TypedEventReadinessFixture {
        scenario: "survival-world-multichunk-durability",
        client_events: SURVIVAL_WORLD_MULTICHUNK_DURABILITY_TYPED_EVENT_CLIENT_EVENTS,
        server_events: SURVIVAL_WORLD_MULTICHUNK_DURABILITY_TYPED_EVENT_SERVER_EVENTS,
        forbidden_events: TYPED_EVENT_COMMON_FORBIDDEN_EVENTS,
        derivation_rules: TYPED_EVENT_EMPTY_EVENTS,
    },
    TypedEventReadinessFixture {
        scenario: "survival-container-block-entity-breadth",
        client_events: SURVIVAL_CONTAINER_BLOCK_ENTITY_BREADTH_TYPED_EVENT_CLIENT_EVENTS,
        server_events: SURVIVAL_CONTAINER_BLOCK_ENTITY_BREADTH_TYPED_EVENT_SERVER_EVENTS,
        forbidden_events: TYPED_EVENT_COMMON_FORBIDDEN_EVENTS,
        derivation_rules: TYPED_EVENT_EMPTY_EVENTS,
    },
    TypedEventReadinessFixture {
        scenario: "survival-biome-dimension-state",
        client_events: SURVIVAL_BIOME_DIMENSION_STATE_TYPED_EVENT_CLIENT_EVENTS,
        server_events: SURVIVAL_BIOME_DIMENSION_STATE_TYPED_EVENT_SERVER_EVENTS,
        forbidden_events: TYPED_EVENT_COMMON_FORBIDDEN_EVENTS,
        derivation_rules: TYPED_EVENT_EMPTY_EVENTS,
    },
    TypedEventReadinessFixture {
        scenario: "survival-biome-dimension-travel",
        client_events: SURVIVAL_BIOME_DIMENSION_TRAVEL_TYPED_EVENT_CLIENT_EVENTS,
        server_events: SURVIVAL_BIOME_DIMENSION_TRAVEL_TYPED_EVENT_SERVER_EVENTS,
        forbidden_events: TYPED_EVENT_COMMON_FORBIDDEN_EVENTS,
        derivation_rules: TYPED_EVENT_EMPTY_EVENTS,
    },
    TypedEventReadinessFixture {
        scenario: "survival-sign-editing-live",
        client_events: SURVIVAL_SIGN_EDITING_LIVE_TYPED_EVENT_CLIENT_EVENTS,
        server_events: SURVIVAL_SIGN_EDITING_LIVE_TYPED_EVENT_SERVER_EVENTS,
        forbidden_events: TYPED_EVENT_COMMON_FORBIDDEN_EVENTS,
        derivation_rules: TYPED_EVENT_EMPTY_EVENTS,
    },
    TypedEventReadinessFixture {
        scenario: "flag-score-repeat",
        client_events: CTF_RULE_TYPED_EVENT_CLIENT_EVENTS,
        server_events: CTF_RULE_TYPED_EVENT_SERVER_EVENTS,
        forbidden_events: CTF_RULE_TYPED_EVENT_FORBIDDEN_EVENTS,
        derivation_rules: TYPED_EVENT_EMPTY_EVENTS,
    },
    TypedEventReadinessFixture {
        scenario: "blue-flag-score",
        client_events: CTF_RULE_TYPED_EVENT_CLIENT_EVENTS,
        server_events: CTF_RULE_TYPED_EVENT_SERVER_EVENTS,
        forbidden_events: CTF_RULE_TYPED_EVENT_FORBIDDEN_EVENTS,
        derivation_rules: TYPED_EVENT_EMPTY_EVENTS,
    },
    TypedEventReadinessFixture {
        scenario: "combat-damage",
        client_events: CTF_RULE_TYPED_EVENT_CLIENT_EVENTS,
        server_events: CTF_RULE_TYPED_EVENT_SERVER_EVENTS,
        forbidden_events: CTF_RULE_TYPED_EVENT_FORBIDDEN_EVENTS,
        derivation_rules: TYPED_EVENT_EMPTY_EVENTS,
    },
    TypedEventReadinessFixture {
        scenario: "combat-knockback",
        client_events: CTF_RULE_TYPED_EVENT_CLIENT_EVENTS,
        server_events: CTF_RULE_TYPED_EVENT_SERVER_EVENTS,
        forbidden_events: CTF_RULE_TYPED_EVENT_FORBIDDEN_EVENTS,
        derivation_rules: TYPED_EVENT_EMPTY_EVENTS,
    },
    TypedEventReadinessFixture {
        scenario: "armor-equipment-mitigation",
        client_events: CTF_RULE_TYPED_EVENT_CLIENT_EVENTS,
        server_events: CTF_RULE_TYPED_EVENT_SERVER_EVENTS,
        forbidden_events: CTF_RULE_TYPED_EVENT_FORBIDDEN_EVENTS,
        derivation_rules: TYPED_EVENT_EMPTY_EVENTS,
    },
    TypedEventReadinessFixture {
        scenario: "equipment-update-observation",
        client_events: CTF_RULE_TYPED_EVENT_CLIENT_EVENTS,
        server_events: CTF_RULE_TYPED_EVENT_SERVER_EVENTS,
        forbidden_events: CTF_RULE_TYPED_EVENT_FORBIDDEN_EVENTS,
        derivation_rules: TYPED_EVENT_EMPTY_EVENTS,
    },
    TypedEventReadinessFixture {
        scenario: "projectile-hit",
        client_events: CTF_RULE_TYPED_EVENT_CLIENT_EVENTS,
        server_events: CTF_RULE_TYPED_EVENT_SERVER_EVENTS,
        forbidden_events: CTF_RULE_TYPED_EVENT_FORBIDDEN_EVENTS,
        derivation_rules: TYPED_EVENT_EMPTY_EVENTS,
    },
    TypedEventReadinessFixture {
        scenario: "projectile-damage-attribution",
        client_events: CTF_RULE_TYPED_EVENT_CLIENT_EVENTS,
        server_events: CTF_RULE_TYPED_EVENT_SERVER_EVENTS,
        forbidden_events: CTF_RULE_TYPED_EVENT_FORBIDDEN_EVENTS,
        derivation_rules: TYPED_EVENT_EMPTY_EVENTS,
    },
    TypedEventReadinessFixture {
        scenario: "flag-carrier-death-return",
        client_events: CTF_RULE_TYPED_EVENT_CLIENT_EVENTS,
        server_events: CTF_RULE_TYPED_EVENT_SERVER_EVENTS,
        forbidden_events: CTF_RULE_TYPED_EVENT_FORBIDDEN_EVENTS,
        derivation_rules: TYPED_EVENT_EMPTY_EVENTS,
    },
    TypedEventReadinessFixture {
        scenario: "reconnect-flag-state",
        client_events: CTF_RULE_TYPED_EVENT_CLIENT_EVENTS,
        server_events: CTF_RULE_TYPED_EVENT_SERVER_EVENTS,
        forbidden_events: CTF_RULE_TYPED_EVENT_FORBIDDEN_EVENTS,
        derivation_rules: TYPED_EVENT_EMPTY_EVENTS,
    },
    TypedEventReadinessFixture {
        scenario: "ctf-invalid-pickup-ownership",
        client_events: CTF_RULE_TYPED_EVENT_CLIENT_EVENTS,
        server_events: CTF_RULE_TYPED_EVENT_SERVER_EVENTS,
        forbidden_events: CTF_RULE_TYPED_EVENT_FORBIDDEN_EVENTS,
        derivation_rules: TYPED_EVENT_EMPTY_EVENTS,
    },
    TypedEventReadinessFixture {
        scenario: "ctf-invalid-return-drop",
        client_events: CTF_RULE_TYPED_EVENT_CLIENT_EVENTS,
        server_events: CTF_RULE_TYPED_EVENT_SERVER_EVENTS,
        forbidden_events: CTF_RULE_TYPED_EVENT_FORBIDDEN_EVENTS,
        derivation_rules: TYPED_EVENT_EMPTY_EVENTS,
    },
    TypedEventReadinessFixture {
        scenario: "ctf-invalid-opponent-base-return-drop",
        client_events: CTF_RULE_TYPED_EVENT_CLIENT_EVENTS,
        server_events: CTF_RULE_TYPED_EVENT_SERVER_EVENTS,
        forbidden_events: CTF_RULE_TYPED_EVENT_FORBIDDEN_EVENTS,
        derivation_rules: TYPED_EVENT_EMPTY_EVENTS,
    },
    TypedEventReadinessFixture {
        scenario: "ctf-score-limit-win-condition",
        client_events: CTF_RULE_TYPED_EVENT_CLIENT_EVENTS,
        server_events: CTF_RULE_TYPED_EVENT_SERVER_EVENTS,
        forbidden_events: CTF_RULE_TYPED_EVENT_FORBIDDEN_EVENTS,
        derivation_rules: TYPED_EVENT_EMPTY_EVENTS,
    },
    TypedEventReadinessFixture {
        scenario: "ctf-simultaneous-pickup-capture-race",
        client_events: CTF_RULE_TYPED_EVENT_CLIENT_EVENTS,
        server_events: CTF_RULE_TYPED_EVENT_SERVER_EVENTS,
        forbidden_events: CTF_RULE_TYPED_EVENT_FORBIDDEN_EVENTS,
        derivation_rules: TYPED_EVENT_EMPTY_EVENTS,
    },
    TypedEventReadinessFixture {
        scenario: "ctf-spawn-team-balance-reset",
        client_events: CTF_RULE_TYPED_EVENT_CLIENT_EVENTS,
        server_events: CTF_RULE_TYPED_EVENT_SERVER_EVENTS,
        forbidden_events: CTF_RULE_TYPED_EVENT_FORBIDDEN_EVENTS,
        derivation_rules: TYPED_EVENT_EMPTY_EVENTS,
    },
    TypedEventReadinessFixture {
        scenario: "reconnect-flag-score",
        client_events: CTF_RULE_TYPED_EVENT_CLIENT_EVENTS,
        server_events: CTF_RULE_TYPED_EVENT_SERVER_EVENTS,
        forbidden_events: CTF_RULE_TYPED_EVENT_FORBIDDEN_EVENTS,
        derivation_rules: TYPED_EVENT_EMPTY_EVENTS,
    },
    TypedEventReadinessFixture {
        scenario: "multi-client-load-score",
        client_events: CTF_RULE_TYPED_EVENT_CLIENT_EVENTS,
        server_events: CTF_RULE_TYPED_EVENT_SERVER_EVENTS,
        forbidden_events: CTF_RULE_TYPED_EVENT_FORBIDDEN_EVENTS,
        derivation_rules: TYPED_EVENT_EMPTY_EVENTS,
    },
];

#[derive(Debug, Clone, PartialEq, Eq)]
struct DryRun {
    check: String,
    wrapper: String,
    receipt_shape_check: bool,
    exclusion_reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ScenarioRow {
    name: String,
    aliases: Vec<String>,
    client_milestones: Vec<String>,
    server_milestones: Vec<String>,
    forbidden_patterns: Vec<String>,
    client_count: u32,
    session_count: u32,
    maintained: bool,
    dry_run: DryRun,
    receipt_expectations: Vec<String>,
    migration_state: String,
    current_bundle_row: String,
    current_bundle_exclusion_reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Manifest {
    schema: String,
    typed_event_fallback_waiver: String,
    rows: Vec<ScenarioRow>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FallbackBudgetEntry {
    name: String,
    owner: String,
    reason: String,
    non_claim: String,
    next_action: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FallbackBudgetBaseline {
    schema: String,
    fallback_rows: Vec<FallbackBudgetEntry>,
    typed_event_ready_rows: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FallbackBudgetReport {
    approved_fallback_rows: Vec<String>,
    removed_fallback_rows: Vec<String>,
    new_fallback_rows: Vec<String>,
    missing_waiver_metadata: Vec<String>,
    typed_event_regressions: Vec<String>,
    baseline_issues: Vec<String>,
}

impl FallbackBudgetReport {
    fn is_complete(&self) -> bool {
        self.new_fallback_rows.is_empty()
            && self.missing_waiver_metadata.is_empty()
            && self.typed_event_regressions.is_empty()
            && self.baseline_issues.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TypedEventReadinessFixture<'a> {
    scenario: &'a str,
    client_events: &'a [&'a str],
    server_events: &'a [&'a str],
    forbidden_events: &'a [&'a str],
    derivation_rules: &'a [&'a str],
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct GeneratedSurface {
    path: &'static str,
    content: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct MissingLiveCapability<'a> {
    path: &'static str,
    token: &'a str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LiveCapabilityRegistryEvaluation<'a> {
    missing: Vec<MissingLiveCapability<'a>>,
}

impl LiveCapabilityRegistryEvaluation<'_> {
    fn is_complete(&self) -> bool {
        self.missing.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DryRunCoverageEvaluation {
    covered: usize,
    waived: usize,
    unmaintained: usize,
    issues: Vec<String>,
}

impl DryRunCoverageEvaluation {
    fn is_complete(&self) -> bool {
        self.issues.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TypedEventReadinessEvaluation {
    ready: usize,
    fallback: usize,
    issues: Vec<String>,
}

impl TypedEventReadinessEvaluation {
    fn is_complete(&self) -> bool {
        self.issues.is_empty()
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.iter().any(|arg| arg == SELF_TEST_FLAG) {
        return match run_self_tests() {
            Ok(summary) => {
                println!("scenario manifest self-test passed: {summary}");
                EXIT_SUCCESS
            }
            Err(errors) => {
                print_errors(&errors);
                EXIT_FAILURE
            }
        };
    }
    if args.iter().any(|arg| arg == CHECK_GENERATED_SURFACES_FLAG) {
        return match run_generated_surfaces_check(Path::new(".")) {
            Ok(summary) => {
                println!("generated surface check passed: {summary}");
                EXIT_SUCCESS
            }
            Err(errors) => {
                print_errors(&errors);
                EXIT_FAILURE
            }
        };
    }
    if args.iter().any(|arg| arg == WRITE_GENERATED_SURFACES_FLAG) {
        return match run_generated_surfaces_write(Path::new(".")) {
            Ok(summary) => {
                println!("generated surface write passed: {summary}");
                EXIT_SUCCESS
            }
            Err(errors) => {
                print_errors(&errors);
                EXIT_FAILURE
            }
        };
    }

    match run_repo_check(Path::new(".")) {
        Ok(summary) => {
            println!("scenario manifest check passed: {summary}");
            EXIT_SUCCESS
        }
        Err(errors) => {
            print_errors(&errors);
            EXIT_FAILURE
        }
    }
}

fn print_errors(errors: &[String]) {
    for error in errors {
        eprintln!("scenario manifest check failed: {error}");
    }
}

fn run_repo_check(root: &Path) -> Result<String, Vec<String>> {
    let manifest_text = read_repo_file(root, MANIFEST_PATH)?;
    let manifest = parse_manifest(&manifest_text)?;
    validate_manifest(&manifest)?;
    let dry_run_coverage = evaluate_dry_run_coverage(&manifest.rows);
    let typed_event_readiness = evaluate_typed_event_readiness(&manifest);
    let fallback_budget = evaluate_repo_fallback_budget(root, &manifest)?;

    let generated = read_repo_file(root, GENERATED_RUST_PATH)?;
    let runner_main = read_repo_file(root, RUNNER_MAIN_PATH)?;
    let runner_scenario_core = read_repo_file(root, RUNNER_SCENARIO_CORE_PATH)?;
    let runner_surface = combined_runner_surface(&runner_main, &runner_scenario_core);
    let flake_surface = read_repo_files(root, FLAKE_SURFACE_PATHS)?;
    let readme = read_repo_file(root, README_PATH)?;
    let scenario_commands = read_repo_file(root, SCENARIO_COMMANDS_DOC_PATH)?;
    let _evidence_workflow = read_repo_file(root, EVIDENCE_WORKFLOW_DOC_PATH)?;
    let _configuration = read_repo_file(root, CONFIGURATION_DOC_PATH)?;
    let _verification = read_repo_file(root, VERIFICATION_DOC_PATH)?;
    let surface_inventory = read_repo_file(root, SURFACE_INVENTORY_PATH)?;
    let current_bundle = read_repo_file(root, CURRENT_BUNDLE_PATH)?;

    let mut errors = Vec::new();
    errors.extend(validate_generated_tables(&manifest.rows, &generated));
    errors.extend(validate_runner_surfaces(&manifest.rows, &runner_surface));
    errors.extend(validate_live_capability_registry_surface(
        &runner_scenario_core,
    ));
    errors.extend(validate_flake_surfaces(&manifest.rows, &flake_surface));
    errors.extend(validate_readme_doc_links(&readme));
    errors.extend(validate_scenario_command_docs(
        &manifest.rows,
        &scenario_commands,
    ));
    errors.extend(validate_surface_inventory(&surface_inventory));
    errors.extend(validate_current_bundle_surfaces(
        &manifest.rows,
        &current_bundle,
    ));

    if errors.is_empty() {
        Ok(format!(
            "{} rows validated; dry-run coverage: {} covered, {} waived, {} unmaintained; typed-event readiness: {} ready, {} fallback; {}",
            manifest.rows.len(),
            dry_run_coverage.covered,
            dry_run_coverage.waived,
            dry_run_coverage.unmaintained,
            typed_event_readiness.ready,
            typed_event_readiness.fallback,
            render_fallback_budget_report(&fallback_budget)
        ))
    } else {
        Err(errors)
    }
}

fn run_generated_surfaces_check(root: &Path) -> Result<String, Vec<String>> {
    let manifest_text = read_repo_file(root, MANIFEST_PATH)?;
    let manifest = parse_manifest(&manifest_text)?;
    validate_manifest(&manifest)?;
    let fallback_budget = evaluate_repo_fallback_budget(root, &manifest)?;
    let surfaces = render_generated_surfaces(&manifest.rows)?;
    let mut errors = Vec::new();
    for surface in &surfaces {
        let checked_in = read_repo_file(root, surface.path)?;
        if let Some(error) =
            generated_surface_stale_diagnostic(surface.path, &surface.content, &checked_in)
        {
            errors.push(error);
        }
    }
    if errors.is_empty() {
        Ok(format!(
            "{} generated surfaces current; {}",
            surfaces.len(),
            render_fallback_budget_report(&fallback_budget)
        ))
    } else {
        Err(errors)
    }
}

fn run_generated_surfaces_write(root: &Path) -> Result<String, Vec<String>> {
    let manifest_text = read_repo_file(root, MANIFEST_PATH)?;
    let manifest = parse_manifest(&manifest_text)?;
    validate_manifest(&manifest)?;
    let fallback_budget = evaluate_repo_fallback_budget(root, &manifest)?;
    let surfaces = render_generated_surfaces(&manifest.rows)?;
    let mut errors = Vec::new();
    for surface in &surfaces {
        let path = root.join(surface.path);
        if let Some(parent) = path.parent() {
            if let Err(err) = fs::create_dir_all(parent) {
                errors.push(format!("create {}: {err}", parent.display()));
                continue;
            }
        }
        if let Err(err) = fs::write(&path, &surface.content) {
            errors.push(format!("write {}: {err}", path.display()));
        }
    }
    if errors.is_empty() {
        Ok(format!(
            "{} generated surfaces written; {}",
            surfaces.len(),
            render_fallback_budget_report(&fallback_budget)
        ))
    } else {
        Err(errors)
    }
}

fn read_repo_file(root: &Path, relative: &str) -> Result<String, Vec<String>> {
    let path = root.join(relative);
    fs::read_to_string(&path).map_err(|err| vec![format!("{}: {err}", path.display())])
}

fn read_repo_files(root: &Path, relatives: &[&str]) -> Result<String, Vec<String>> {
    let mut combined = String::new();
    for relative in relatives {
        let path = root.join(relative);
        let text = match fs::read_to_string(&path) {
            Ok(text) => text,
            Err(err) if err.kind() == ErrorKind::NotFound => continue,
            Err(err) => return Err(vec![format!("{}: {err}", path.display())]),
        };
        combined.push_str("\n--- ");
        combined.push_str(relative);
        combined.push_str(" ---\n");
        combined.push_str(&text);
    }
    Ok(combined)
}

fn parse_manifest(text: &str) -> Result<Manifest, Vec<String>> {
    let schema = parse_top_level_string(text, "schema")?;
    let typed_event_fallback_waiver =
        parse_top_level_string(text, TYPED_EVENT_FALLBACK_WAIVER_FIELD)?;
    let rows = parse_scenario_rows(text)?;
    Ok(Manifest {
        schema,
        typed_event_fallback_waiver,
        rows,
    })
}

fn parse_fallback_budget_baseline(text: &str) -> Result<FallbackBudgetBaseline, Vec<String>> {
    let mut errors = Vec::new();
    let schema = parse_top_level_string(text, "schema").unwrap_or_else(|mut schema_errors| {
        errors.append(&mut schema_errors);
        String::new()
    });
    if !schema.is_empty() && schema != FALLBACK_BUDGET_BASELINE_SCHEMA {
        errors.push(format!(
            "unsupported fallback budget baseline schema {schema}"
        ));
    }
    let typed_event_ready_rows = parse_top_level_array(text, FALLBACK_BUDGET_TYPED_READY_FIELD)
        .unwrap_or_else(|mut array_errors| {
            errors.append(&mut array_errors);
            Vec::new()
        });
    let common_waiver =
        parse_common_fallback_budget_waiver(text).unwrap_or_else(|mut waiver_errors| {
            errors.append(&mut waiver_errors);
            empty_fallback_budget_entry()
        });
    let fallback_rows =
        parse_fallback_budget_rows(text, &common_waiver).unwrap_or_else(|mut row_errors| {
            errors.append(&mut row_errors);
            Vec::new()
        });
    if errors.is_empty() {
        Ok(FallbackBudgetBaseline {
            schema,
            fallback_rows,
            typed_event_ready_rows,
        })
    } else {
        Err(errors)
    }
}

fn parse_top_level_array(text: &str, field: &str) -> Result<Vec<String>, Vec<String>> {
    for raw_line in text.lines() {
        let line = raw_line.trim();
        let prefix = format!("{field} = ");
        if line.starts_with(&prefix) {
            return parse_string_array(line, field).map_err(|err| vec![err]);
        }
    }
    Err(vec![format!("missing top-level array field {field}")])
}

fn parse_common_fallback_budget_waiver(text: &str) -> Result<FallbackBudgetEntry, Vec<String>> {
    let mut errors = Vec::new();
    let owner = parse_top_level_string(text, "owner").unwrap_or_else(|mut field_errors| {
        errors.append(&mut field_errors);
        String::new()
    });
    let reason = parse_top_level_string(text, "reason").unwrap_or_else(|mut field_errors| {
        errors.append(&mut field_errors);
        String::new()
    });
    let non_claim = parse_top_level_string(text, "non_claim").unwrap_or_else(|mut field_errors| {
        errors.append(&mut field_errors);
        String::new()
    });
    let next_action =
        parse_top_level_string(text, "next_action").unwrap_or_else(|mut field_errors| {
            errors.append(&mut field_errors);
            String::new()
        });
    if errors.is_empty() {
        Ok(FallbackBudgetEntry {
            name: String::new(),
            owner,
            reason,
            non_claim,
            next_action,
        })
    } else {
        Err(errors)
    }
}

fn parse_fallback_budget_rows(
    text: &str,
    common_waiver: &FallbackBudgetEntry,
) -> Result<Vec<FallbackBudgetEntry>, Vec<String>> {
    let mut rows = Vec::new();
    let mut errors = Vec::new();
    let mut in_rows = false;

    for raw_line in text.lines() {
        let line = raw_line.trim();
        if !in_rows {
            if line == FALLBACK_BUDGET_FALLBACK_ROWS_START {
                in_rows = true;
            }
            continue;
        }
        if line == SCENARIOS_END {
            break;
        }
        if !line.starts_with(ROW_START) {
            continue;
        }
        match parse_fallback_budget_row(line, common_waiver) {
            Ok(row) => rows.push(row),
            Err(mut row_errors) => errors.append(&mut row_errors),
        }
    }

    if rows.is_empty() {
        errors.push("fallback budget baseline has no fallback rows".to_string());
    }
    if errors.is_empty() {
        Ok(rows)
    } else {
        Err(errors)
    }
}

fn parse_fallback_budget_row(
    line: &str,
    common_waiver: &FallbackBudgetEntry,
) -> Result<FallbackBudgetEntry, Vec<String>> {
    let mut errors = Vec::new();
    let name = parse_inline_record_string(line, "name").unwrap_or_else(|err| {
        errors.push(err);
        String::new()
    });
    let owner =
        parse_inline_record_waiver_value(line, "owner", common_waiver).unwrap_or_else(|err| {
            errors.push(err);
            String::new()
        });
    let reason =
        parse_inline_record_waiver_value(line, "reason", common_waiver).unwrap_or_else(|err| {
            errors.push(err);
            String::new()
        });
    let non_claim = parse_inline_record_waiver_value(line, "non_claim", common_waiver)
        .unwrap_or_else(|err| {
            errors.push(err);
            String::new()
        });
    let next_action = parse_inline_record_waiver_value(line, "next_action", common_waiver)
        .unwrap_or_else(|err| {
            errors.push(err);
            String::new()
        });

    if errors.is_empty() {
        Ok(FallbackBudgetEntry {
            name,
            owner,
            reason,
            non_claim,
            next_action,
        })
    } else {
        Err(errors)
    }
}

fn parse_top_level_string(text: &str, field: &str) -> Result<String, Vec<String>> {
    for raw_line in text.lines() {
        let line = raw_line.trim();
        if line.starts_with(&format!("{field}{STRING_FIELD_DELIMITER}")) {
            return parse_string_value(line, field).map_err(|err| vec![err]);
        }
    }
    Err(vec![format!("missing top-level string field {field}")])
}

fn parse_scenario_rows(text: &str) -> Result<Vec<ScenarioRow>, Vec<String>> {
    let mut rows = Vec::new();
    let mut errors = Vec::new();
    let mut in_scenarios = false;
    let mut in_row = false;
    let mut block = Vec::new();

    for raw_line in text.lines() {
        let line = raw_line.trim();
        if !in_scenarios {
            if line == SCENARIOS_START {
                in_scenarios = true;
            }
            continue;
        }
        if !in_row && line == SCENARIOS_END {
            break;
        }
        if line == ROW_START && !in_row {
            in_row = true;
            block.clear();
            continue;
        }
        if in_row {
            if line == ROW_END {
                match parse_scenario_row(&block) {
                    Ok(row) => rows.push(row),
                    Err(mut row_errors) => errors.append(&mut row_errors),
                }
                in_row = false;
                block.clear();
                continue;
            }
            block.push(line.to_string());
        }
    }

    if in_row {
        errors.push("unterminated scenario row".to_string());
    }
    if rows.is_empty() {
        errors.push("manifest has no scenario rows".to_string());
    }
    if errors.is_empty() {
        Ok(rows)
    } else {
        Err(errors)
    }
}

fn parse_scenario_row(lines: &[String]) -> Result<ScenarioRow, Vec<String>> {
    let mut errors = Vec::new();

    let name = collect_string(lines, "name", &mut errors);
    let aliases = collect_array(lines, "aliases", &mut errors);
    let client_milestones = collect_array(lines, "client_milestones", &mut errors);
    let server_milestones = collect_array(lines, "server_milestones", &mut errors);
    let forbidden_patterns = collect_array(lines, "forbidden_patterns", &mut errors);
    let client_count = collect_number(lines, "client_count", &mut errors);
    let session_count = collect_number(lines, "session_count", &mut errors);
    let maintained = collect_bool(lines, "maintained", &mut errors);
    let dry_run = collect_dry_run(lines, &mut errors);
    let receipt_expectations = collect_array(lines, "receipt_expectations", &mut errors);
    let migration_state = collect_string(lines, "migration_state", &mut errors);
    let current_bundle_row = collect_string(lines, "current_bundle_row", &mut errors);
    let current_bundle_exclusion_reason =
        collect_string(lines, "current_bundle_exclusion_reason", &mut errors);

    if errors.is_empty() {
        Ok(ScenarioRow {
            name,
            aliases,
            client_milestones,
            server_milestones,
            forbidden_patterns,
            client_count,
            session_count,
            maintained,
            dry_run,
            receipt_expectations,
            migration_state,
            current_bundle_row,
            current_bundle_exclusion_reason,
        })
    } else {
        Err(errors)
    }
}

fn collect_string(lines: &[String], field: &str, errors: &mut Vec<String>) -> String {
    match find_field_line(lines, field).and_then(|line| parse_string_value(line, field).ok()) {
        Some(value) => value,
        None => {
            errors.push(format!("missing or invalid string field {field}"));
            String::new()
        }
    }
}

fn collect_array(lines: &[String], field: &str, errors: &mut Vec<String>) -> Vec<String> {
    match find_field_line(lines, field).and_then(|line| parse_string_array(line, field).ok()) {
        Some(value) => value,
        None => {
            errors.push(format!("missing or invalid array field {field}"));
            Vec::new()
        }
    }
}

fn collect_number(lines: &[String], field: &str, errors: &mut Vec<String>) -> u32 {
    match find_field_line(lines, field).and_then(|line| parse_u32_value(line, field).ok()) {
        Some(value) => value,
        None => {
            errors.push(format!("missing or invalid number field {field}"));
            u32::MIN
        }
    }
}

fn collect_bool(lines: &[String], field: &str, errors: &mut Vec<String>) -> bool {
    match find_field_line(lines, field).and_then(|line| parse_bool_value(line, field).ok()) {
        Some(value) => value,
        None => {
            errors.push(format!("missing or invalid bool field {field}"));
            false
        }
    }
}

fn collect_dry_run(lines: &[String], errors: &mut Vec<String>) -> DryRun {
    let line = match find_field_line(lines, "dry_run") {
        Some(line) => line,
        None => {
            errors.push("missing dry_run record".to_string());
            return empty_dry_run();
        }
    };
    let check = parse_inline_record_string(line, "check").unwrap_or_else(|err| {
        errors.push(err);
        String::new()
    });
    let wrapper = parse_inline_record_string(line, "wrapper").unwrap_or_else(|err| {
        errors.push(err);
        String::new()
    });
    let receipt_shape_check =
        parse_inline_record_bool(line, "receipt_shape_check").unwrap_or_else(|err| {
            errors.push(err);
            false
        });
    let exclusion_reason =
        parse_inline_record_string(line, "exclusion_reason").unwrap_or_else(|err| {
            errors.push(err);
            String::new()
        });
    DryRun {
        check,
        wrapper,
        receipt_shape_check,
        exclusion_reason,
    }
}

fn empty_dry_run() -> DryRun {
    DryRun {
        check: String::new(),
        wrapper: String::new(),
        receipt_shape_check: false,
        exclusion_reason: String::new(),
    }
}

fn empty_fallback_budget_entry() -> FallbackBudgetEntry {
    FallbackBudgetEntry {
        name: String::new(),
        owner: String::new(),
        reason: String::new(),
        non_claim: String::new(),
        next_action: String::new(),
    }
}

fn find_field_line<'a>(lines: &'a [String], field: &str) -> Option<&'a str> {
    let prefix = format!("{field} =");
    lines
        .iter()
        .map(String::as_str)
        .find(|line| line.starts_with(&prefix))
}

fn parse_string_value(line: &str, field: &str) -> Result<String, String> {
    let prefix = format!("{field}{STRING_FIELD_DELIMITER}");
    let value = line
        .strip_prefix(&prefix)
        .ok_or_else(|| format!("{field}: expected string assignment"))?;
    let value = value
        .split('"')
        .next()
        .ok_or_else(|| format!("{field}: unterminated string"))?;
    Ok(value.to_string())
}

fn parse_string_array(line: &str, field: &str) -> Result<Vec<String>, String> {
    let prefix = format!("{field} = ");
    let rest = line
        .strip_prefix(&prefix)
        .ok_or_else(|| format!("{field}: expected array assignment"))?
        .trim_end_matches(',')
        .trim();
    let inner = rest
        .strip_prefix(ARRAY_START)
        .and_then(|value| value.strip_suffix(ARRAY_END))
        .ok_or_else(|| format!("{field}: expected one-line array"))?;
    if inner.trim().is_empty() {
        return Ok(Vec::new());
    }
    let mut values = Vec::new();
    for part in inner.split(',') {
        let value = part.trim();
        let value = value
            .strip_prefix('"')
            .and_then(|value| value.strip_suffix('"'))
            .ok_or_else(|| format!("{field}: invalid string array item {value}"))?;
        values.push(value.to_string());
    }
    Ok(values)
}

fn parse_u32_value(line: &str, field: &str) -> Result<u32, String> {
    let prefix = format!("{field} = ");
    let value = line
        .strip_prefix(&prefix)
        .ok_or_else(|| format!("{field}: expected number assignment"))?
        .trim_end_matches(',')
        .trim();
    value
        .parse::<u32>()
        .map_err(|err| format!("{field}: invalid number {value}: {err}"))
}

fn parse_bool_value(line: &str, field: &str) -> Result<bool, String> {
    let prefix = format!("{field} = ");
    let value = line
        .strip_prefix(&prefix)
        .ok_or_else(|| format!("{field}: expected bool assignment"))?
        .trim_end_matches(',')
        .trim();
    value
        .parse::<bool>()
        .map_err(|err| format!("{field}: invalid bool {value}: {err}"))
}

fn parse_inline_record_string(line: &str, field: &str) -> Result<String, String> {
    let needle = format!("{field}{STRING_FIELD_DELIMITER}");
    let rest = line
        .split(&needle)
        .nth(MINIMUM_POSITIVE_COUNT as usize)
        .ok_or_else(|| format!("dry_run.{field}: missing string field"))?;
    let value = rest
        .split('"')
        .next()
        .ok_or_else(|| format!("dry_run.{field}: unterminated string"))?;
    Ok(value.to_string())
}

fn parse_inline_record_bool(line: &str, field: &str) -> Result<bool, String> {
    let needle = format!("{field} = ");
    let rest = line
        .split(&needle)
        .nth(MINIMUM_POSITIVE_COUNT as usize)
        .ok_or_else(|| format!("dry_run.{field}: missing bool field"))?;
    let value = rest
        .split([',', '}'])
        .next()
        .ok_or_else(|| format!("dry_run.{field}: invalid bool field"))?
        .trim();
    value
        .parse::<bool>()
        .map_err(|err| format!("dry_run.{field}: invalid bool {value}: {err}"))
}

fn parse_inline_record_waiver_value(
    line: &str,
    field: &str,
    common_waiver: &FallbackBudgetEntry,
) -> Result<String, String> {
    let needle = format!("{field} = ");
    let rest = line
        .split(&needle)
        .nth(MINIMUM_POSITIVE_COUNT as usize)
        .ok_or_else(|| format!("fallback_budget.{field}: missing field"))?;
    let value = rest
        .split([',', '}'])
        .next()
        .ok_or_else(|| format!("fallback_budget.{field}: invalid field"))?
        .trim();
    if let Some(value) = value
        .strip_prefix('"')
        .and_then(|value| value.strip_suffix('"'))
    {
        return Ok(value.to_string());
    }
    let Some(reference) = value.strip_prefix(COMMON_WAIVER_REFERENCE_PREFIX) else {
        return Err(format!(
            "fallback_budget.{field}: unsupported waiver value {value:?}"
        ));
    };
    match reference {
        "owner" => Ok(common_waiver.owner.clone()),
        "reason" => Ok(common_waiver.reason.clone()),
        "non_claim" => Ok(common_waiver.non_claim.clone()),
        "next_action" => Ok(common_waiver.next_action.clone()),
        other => Err(format!(
            "fallback_budget.{field}: unsupported common waiver reference {other:?}"
        )),
    }
}

fn validate_manifest(manifest: &Manifest) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();
    if manifest.schema != SUPPORTED_SCHEMA {
        errors.push(format!("unsupported schema {}", manifest.schema));
    }
    let mut names = BTreeSet::new();
    let mut aliases = BTreeMap::<String, String>::new();
    for row in &manifest.rows {
        validate_row(row, &mut errors);
        if !names.insert(row.name.clone()) {
            errors.push(format!("duplicate scenario name {}", row.name));
        }
        for alias in &row.aliases {
            if let Some(existing) = aliases.insert(alias.clone(), row.name.clone()) {
                errors.push(format!(
                    "duplicate alias {alias} shared by {existing} and {}",
                    row.name
                ));
            }
        }
    }
    let dry_run_coverage = evaluate_dry_run_coverage(&manifest.rows);
    errors.extend(dry_run_coverage.issues);
    let typed_event_readiness = evaluate_typed_event_readiness(manifest);
    errors.extend(typed_event_readiness.issues);
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn validate_row(row: &ScenarioRow, errors: &mut Vec<String>) {
    if row.name.is_empty() {
        errors.push("scenario row has empty name".to_string());
    }
    if row.aliases.is_empty() || !row.aliases.iter().any(|alias| alias == &row.name) {
        errors.push(format!("{}: aliases must include canonical name", row.name));
    }
    if row.client_milestones.is_empty() {
        errors.push(format!("{}: client_milestones must be nonempty", row.name));
    }
    if row.forbidden_patterns.is_empty() {
        errors.push(format!("{}: forbidden_patterns must be nonempty", row.name));
    }
    if row.client_count < MINIMUM_POSITIVE_COUNT {
        errors.push(format!("{}: client_count must be positive", row.name));
    }
    if row.session_count < MINIMUM_POSITIVE_COUNT {
        errors.push(format!("{}: session_count must be positive", row.name));
    }
    if row.receipt_expectations.is_empty() {
        errors.push(format!(
            "{}: receipt_expectations must be nonempty",
            row.name
        ));
    }
    if !is_supported_migration_state(&row.migration_state) {
        errors.push(format!(
            "{}: unsupported migration_state {}",
            row.name, row.migration_state
        ));
    }
    if row.current_bundle_row.is_empty() && row.current_bundle_exclusion_reason.is_empty() {
        errors.push(format!(
            "{}: current bundle row or exclusion reason required",
            row.name
        ));
    }
}

fn is_supported_migration_state(value: &str) -> bool {
    value == SUBSTRING_FALLBACK_MIGRATION || value == TYPED_EVENT_READY_MIGRATION
}

fn evaluate_dry_run_coverage(rows: &[ScenarioRow]) -> DryRunCoverageEvaluation {
    let mut evaluation = DryRunCoverageEvaluation {
        covered: usize::MIN,
        waived: usize::MIN,
        unmaintained: usize::MIN,
        issues: Vec::new(),
    };
    for row in rows {
        evaluate_dry_run_coverage_row(row, &mut evaluation);
    }
    evaluation
}

fn evaluate_dry_run_coverage_row(row: &ScenarioRow, evaluation: &mut DryRunCoverageEvaluation) {
    if !row.maintained {
        evaluation.unmaintained += 1;
        return;
    }
    if row.dry_run.wrapper.is_empty() {
        evaluation
            .issues
            .push(format!("{}: dry-run wrapper metadata missing", row.name));
    }
    if !row.dry_run.check.is_empty() {
        evaluation.covered += 1;
        if !row.dry_run.receipt_shape_check {
            evaluation.issues.push(format!(
                "{}: dry-run check must set receipt_shape_check=true",
                row.name
            ));
        }
        if !row.dry_run.exclusion_reason.is_empty() {
            evaluation.issues.push(format!(
                "{}: covered row must not carry waiver metadata",
                row.name
            ));
        }
        return;
    }
    if !row.dry_run.exclusion_reason.is_empty() {
        evaluation.waived += 1;
        if row.dry_run.receipt_shape_check {
            evaluation.issues.push(format!(
                "{}: waived row must set receipt_shape_check=false",
                row.name
            ));
        }
        evaluation
            .issues
            .extend(validate_dry_run_waiver_metadata(row));
        return;
    }
    evaluation.issues.push(format!(
        "{}: maintained row needs dry-run check or waiver metadata",
        row.name
    ));
}

fn validate_dry_run_waiver_metadata(row: &ScenarioRow) -> Vec<String> {
    let mut errors = Vec::new();
    let metadata = row.dry_run.exclusion_reason.trim();
    for marker in STALE_DRY_RUN_EXCLUSION_MARKERS {
        if metadata.contains(marker) {
            errors.push(format!(
                "{}: waiver metadata contains stale exclusion marker {:?}",
                row.name, marker
            ));
        }
    }
    for field in REQUIRED_WAIVER_FIELDS {
        if waiver_field_value(metadata, field).is_none() {
            errors.push(format!(
                "{}: waiver metadata missing nonempty {field}",
                row.name
            ));
        }
    }
    errors
}

fn waiver_field_value<'a>(metadata: &'a str, field: &str) -> Option<&'a str> {
    metadata
        .split(';')
        .map(str::trim)
        .find_map(|part| part.strip_prefix(field).map(str::trim))
        .filter(|value| !value.is_empty())
}

fn evaluate_typed_event_readiness(manifest: &Manifest) -> TypedEventReadinessEvaluation {
    let mut evaluation = TypedEventReadinessEvaluation {
        ready: usize::MIN,
        fallback: usize::MIN,
        issues: Vec::new(),
    };
    for row in &manifest.rows {
        evaluate_typed_event_readiness_row(row, &mut evaluation);
    }
    if evaluation.fallback > usize::MIN {
        evaluation.issues.extend(validate_waiver_metadata_fields(
            TYPED_EVENT_FALLBACK_WAIVER_FIELD,
            &manifest.typed_event_fallback_waiver,
            &[],
        ));
    }
    evaluation
}

fn evaluate_typed_event_readiness_row(
    row: &ScenarioRow,
    evaluation: &mut TypedEventReadinessEvaluation,
) {
    if row.migration_state == TYPED_EVENT_READY_MIGRATION {
        evaluation.ready += 1;
        evaluation
            .issues
            .extend(validate_typed_event_ready_row(row));
        return;
    }
    if row.migration_state == SUBSTRING_FALLBACK_MIGRATION {
        evaluation.fallback += 1;
    }
}

fn validate_typed_event_ready_row(row: &ScenarioRow) -> Vec<String> {
    let Some(fixture) = typed_event_readiness_fixture(&row.name) else {
        return vec![format!(
            "{}: typed-event-ready row lacks readiness fixture",
            row.name
        )];
    };
    let mut errors = Vec::new();
    for milestone in &row.client_milestones {
        if !typed_event_surface_contains(fixture.client_events, fixture.derivation_rules, milestone)
        {
            errors.push(format!(
                "{}: missing client typed-event surface {milestone}",
                row.name
            ));
        }
    }
    for milestone in &row.server_milestones {
        if !typed_event_surface_contains(fixture.server_events, fixture.derivation_rules, milestone)
        {
            errors.push(format!(
                "{}: missing server typed-event surface {milestone}",
                row.name
            ));
        }
    }
    for forbidden in &row.forbidden_patterns {
        if !typed_event_surface_contains(
            fixture.forbidden_events,
            fixture.derivation_rules,
            forbidden,
        ) {
            errors.push(format!(
                "{}: missing forbidden typed-event surface {forbidden}",
                row.name
            ));
        }
    }
    errors
}

fn typed_event_surface_contains(events: &[&str], derivation_rules: &[&str], value: &str) -> bool {
    events.contains(&value) || derivation_rules.contains(&value)
}

fn typed_event_readiness_fixture(
    name: &str,
) -> Option<&'static TypedEventReadinessFixture<'static>> {
    TYPED_EVENT_READINESS_FIXTURES
        .iter()
        .find(|fixture| fixture.scenario == name)
}

fn validate_waiver_metadata_fields(
    label: &str,
    metadata: &str,
    stale_markers: &[&str],
) -> Vec<String> {
    let mut errors = Vec::new();
    let metadata = metadata.trim();
    for marker in stale_markers {
        if metadata.contains(marker) {
            errors.push(format!(
                "{label}: waiver metadata contains stale marker {:?}",
                marker
            ));
        }
    }
    for field in REQUIRED_WAIVER_FIELDS {
        if waiver_field_value(metadata, field).is_none() {
            errors.push(format!("{label}: waiver metadata missing nonempty {field}"));
        }
    }
    errors
}

fn evaluate_repo_fallback_budget(
    root: &Path,
    manifest: &Manifest,
) -> Result<FallbackBudgetReport, Vec<String>> {
    let baseline_text = read_repo_file(root, FALLBACK_BUDGET_BASELINE_PATH)?;
    let baseline = parse_fallback_budget_baseline(&baseline_text)?;
    let report = evaluate_fallback_budget(&manifest.rows, &baseline);
    let diagnostics = fallback_budget_diagnostics(&report);
    if diagnostics.is_empty() {
        Ok(report)
    } else {
        Err(diagnostics)
    }
}

fn evaluate_fallback_budget(
    rows: &[ScenarioRow],
    baseline: &FallbackBudgetBaseline,
) -> FallbackBudgetReport {
    let mut report = FallbackBudgetReport {
        approved_fallback_rows: Vec::new(),
        removed_fallback_rows: Vec::new(),
        new_fallback_rows: Vec::new(),
        missing_waiver_metadata: Vec::new(),
        typed_event_regressions: Vec::new(),
        baseline_issues: Vec::new(),
    };
    let current_rows = current_rows_by_name(rows, &mut report.baseline_issues);
    let baseline_fallback_rows = baseline_fallback_rows_by_name(baseline, &mut report);
    let typed_event_ready_rows = baseline_typed_event_ready_rows(baseline, &mut report);

    for row in rows {
        if row.migration_state != SUBSTRING_FALLBACK_MIGRATION {
            continue;
        }
        if baseline_fallback_rows.contains_key(row.name.as_str()) {
            report.approved_fallback_rows.push(row.name.clone());
        } else {
            report.new_fallback_rows.push(row.name.clone());
        }
        if typed_event_ready_rows.contains(row.name.as_str()) {
            report.typed_event_regressions.push(row.name.clone());
        }
    }

    for entry in &baseline.fallback_rows {
        if current_rows.get(entry.name.as_str()).map_or(true, |row| {
            row.migration_state != SUBSTRING_FALLBACK_MIGRATION
        }) {
            report.removed_fallback_rows.push(entry.name.clone());
        }
    }

    sort_fallback_budget_report(&mut report);
    report
}

fn current_rows_by_name<'a>(
    rows: &'a [ScenarioRow],
    baseline_issues: &mut Vec<String>,
) -> BTreeMap<&'a str, &'a ScenarioRow> {
    let mut current_rows = BTreeMap::new();
    for row in rows {
        if current_rows.insert(row.name.as_str(), row).is_some() {
            baseline_issues.push(format!("duplicate current scenario row {}", row.name));
        }
    }
    current_rows
}

fn baseline_fallback_rows_by_name<'a>(
    baseline: &'a FallbackBudgetBaseline,
    report: &mut FallbackBudgetReport,
) -> BTreeMap<&'a str, &'a FallbackBudgetEntry> {
    let mut rows = BTreeMap::new();
    if baseline.schema != FALLBACK_BUDGET_BASELINE_SCHEMA {
        report.baseline_issues.push(format!(
            "unsupported fallback budget baseline schema {}",
            baseline.schema
        ));
    }
    for entry in &baseline.fallback_rows {
        if entry.name.is_empty() {
            report
                .baseline_issues
                .push("fallback budget baseline contains empty row name".to_string());
            continue;
        }
        if rows.insert(entry.name.as_str(), entry).is_some() {
            report.baseline_issues.push(format!(
                "duplicate fallback budget baseline row {}",
                entry.name
            ));
        }
        report
            .missing_waiver_metadata
            .extend(fallback_budget_entry_metadata_diagnostics(entry));
    }
    rows
}

fn baseline_typed_event_ready_rows<'a>(
    baseline: &'a FallbackBudgetBaseline,
    report: &mut FallbackBudgetReport,
) -> BTreeSet<&'a str> {
    let mut rows = BTreeSet::new();
    let fallback_rows: BTreeSet<&str> = baseline
        .fallback_rows
        .iter()
        .map(|entry| entry.name.as_str())
        .collect();
    for row in &baseline.typed_event_ready_rows {
        if row.is_empty() {
            report
                .baseline_issues
                .push("fallback budget baseline contains empty typed-event-ready row".to_string());
            continue;
        }
        if !rows.insert(row.as_str()) {
            report
                .baseline_issues
                .push(format!("duplicate typed-event-ready baseline row {row}"));
        }
        if fallback_rows.contains(row.as_str()) {
            report.baseline_issues.push(format!(
                "{row}: baseline cannot be both fallback and typed-event-ready"
            ));
        }
    }
    rows
}

fn fallback_budget_entry_metadata_diagnostics(entry: &FallbackBudgetEntry) -> Vec<String> {
    let mut errors = Vec::new();
    if entry.owner.trim().is_empty() {
        errors.push(format!(
            "{}: fallback budget waiver missing owner",
            entry.name
        ));
    }
    if entry.reason.trim().is_empty() {
        errors.push(format!(
            "{}: fallback budget waiver missing reason",
            entry.name
        ));
    }
    if entry.non_claim.trim().is_empty() {
        errors.push(format!(
            "{}: fallback budget waiver missing non_claim",
            entry.name
        ));
    }
    if entry.next_action.trim().is_empty() {
        errors.push(format!(
            "{}: fallback budget waiver missing next_action",
            entry.name
        ));
    }
    errors
}

fn fallback_budget_diagnostics(report: &FallbackBudgetReport) -> Vec<String> {
    let mut errors = report.baseline_issues.clone();
    errors.extend(report.missing_waiver_metadata.clone());
    errors.extend(
        report
            .new_fallback_rows
            .iter()
            .map(|row| format!("{row}: unapproved substring fallback row")),
    );
    errors.extend(
        report.typed_event_regressions.iter().map(|row| {
            format!("{row}: typed-event-ready baseline regressed to substring fallback")
        }),
    );
    errors
}

fn sort_fallback_budget_report(report: &mut FallbackBudgetReport) {
    report.approved_fallback_rows.sort();
    report.removed_fallback_rows.sort();
    report.new_fallback_rows.sort();
    report.missing_waiver_metadata.sort();
    report.typed_event_regressions.sort();
    report.baseline_issues.sort();
}

fn render_fallback_budget_report(report: &FallbackBudgetReport) -> String {
    format!(
        "fallback budget: approved=[{}]; removed=[{}]; new=[{}]; typed_event_regressions=[{}]; missing_waiver_metadata=[{}]",
        comma_list(&report.approved_fallback_rows),
        comma_list(&report.removed_fallback_rows),
        comma_list(&report.new_fallback_rows),
        comma_list(&report.typed_event_regressions),
        comma_list(&report.missing_waiver_metadata)
    )
}

fn comma_list(values: &[String]) -> String {
    if values.is_empty() {
        return String::new();
    }
    values.join(",")
}

fn render_generated_surfaces(rows: &[ScenarioRow]) -> Result<Vec<GeneratedSurface>, Vec<String>> {
    let surfaces = vec![
        GeneratedSurface {
            path: GENERATED_RUST_PATH,
            content: render_generated_rust(rows)?,
        },
        GeneratedSurface {
            path: GENERATED_SCENARIO_INDEX_PATH,
            content: render_generated_scenario_index(rows)?,
        },
        GeneratedSurface {
            path: GENERATED_SCENARIO_COMMANDS_PATH,
            content: render_generated_scenario_commands(rows)?,
        },
        GeneratedSurface {
            path: GENERATED_WRAPPER_METADATA_PATH,
            content: render_generated_wrapper_metadata(rows)?,
        },
    ];
    validate_generated_surfaces(&surfaces)?;
    Ok(surfaces)
}

fn validate_generated_surfaces(surfaces: &[GeneratedSurface]) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();
    let mut paths = BTreeSet::new();
    for surface in surfaces {
        if let Err(mut path_errors) = validate_generated_output_path(surface.path) {
            errors.append(&mut path_errors);
        }
        if !paths.insert(surface.path) {
            errors.push(format!("duplicate generated output path {}", surface.path));
        }
        if surface.content.is_empty() {
            errors.push(format!("{} generated content is empty", surface.path));
        }
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn validate_generated_output_path(path: &str) -> Result<(), Vec<String>> {
    let unsafe_path = path.is_empty()
        || path.starts_with('/')
        || path
            .split('/')
            .any(|component| component.is_empty() || component == "." || component == "..");
    if unsafe_path {
        return Err(vec![format!("unsafe generated output path {path:?}")]);
    }
    Ok(())
}

fn generated_surface_stale_diagnostic(
    path: &str,
    expected: &str,
    checked_in: &str,
) -> Option<String> {
    if checked_in == expected {
        return None;
    }
    Some(format!(
        "{path} is stale; run {WRITE_GENERATED_SURFACES_FLAG}"
    ))
}

fn render_generated_rust(rows: &[ScenarioRow]) -> Result<String, Vec<String>> {
    let mut output = String::new();
    output
        .push_str("// @generated by tools/check_scenario_manifest.rs --write-generated-surfaces\n");
    output
        .push_str("// Do not edit by hand; edit compat/config/scenario-manifest.ncl instead.\n\n");
    output.push_str("pub(crate) const ONE_CLIENT: u8 = 1;\n");
    output.push_str("pub(crate) const TWO_CLIENTS: u8 = 2;\n");
    output.push_str("pub(crate) const ONE_SESSION: u8 = 1;\n");
    output.push_str("pub(crate) const TWO_SESSIONS: u8 = 2;\n\n");
    output.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n");
    output.push_str("pub(crate) struct GeneratedScenarioRow {\n");
    output.push_str("    pub(crate) name: &'static str,\n");
    output.push_str("    pub(crate) aliases: &'static [&'static str],\n");
    output.push_str("    pub(crate) client_milestones: &'static [&'static str],\n");
    output.push_str("    pub(crate) server_milestones: &'static [&'static str],\n");
    output.push_str("    pub(crate) forbidden_patterns: &'static [&'static str],\n");
    output.push_str("    pub(crate) client_count: u8,\n");
    output.push_str("    pub(crate) session_count: u8,\n");
    output.push_str("    pub(crate) dry_run_check: &'static str,\n");
    output.push_str("    pub(crate) dry_run_wrapper: &'static str,\n");
    output.push_str("    pub(crate) dry_run_exclusion_reason: &'static str,\n");
    output.push_str("    pub(crate) migration_state: &'static str,\n");
    output.push_str("}\n\n");
    output.push_str("pub(crate) const SCENARIO_MANIFEST_ROWS: &[GeneratedScenarioRow] = &[\n");
    let mut seen_names = BTreeSet::new();
    for row in rows {
        if !seen_names.insert(row.name.as_str()) {
            return Err(vec![format!(
                "duplicate generated scenario name {}",
                row.name
            )]);
        }
        output.push_str("    GeneratedScenarioRow {\n");
        output.push_str(&format!("        name: {},\n", rust_string(&row.name)));
        output.push_str(&format!(
            "        aliases: {},\n",
            rust_string_array(&row.aliases, "        ", "        aliases: ")
        ));
        output.push_str(&format!(
            "        client_milestones: {},\n",
            rust_string_array(
                &row.client_milestones,
                "        ",
                "        client_milestones: ",
            )
        ));
        output.push_str(&format!(
            "        server_milestones: {},\n",
            rust_string_array(
                &row.server_milestones,
                "        ",
                "        server_milestones: ",
            )
        ));
        output.push_str(&format!(
            "        forbidden_patterns: {},\n",
            rust_string_array(
                &row.forbidden_patterns,
                "        ",
                "        forbidden_patterns: ",
            )
        ));
        output.push_str(&format!(
            "        client_count: {},\n",
            rust_count_expr(row.client_count, "ONE_CLIENT", "TWO_CLIENTS")
        ));
        output.push_str(&format!(
            "        session_count: {},\n",
            rust_count_expr(row.session_count, "ONE_SESSION", "TWO_SESSIONS")
        ));
        output.push_str(&format!(
            "        dry_run_check: {},\n",
            rust_string(&row.dry_run.check)
        ));
        output.push_str(&format!(
            "        dry_run_wrapper: {},\n",
            rust_string(&row.dry_run.wrapper)
        ));
        output.push_str(&format!(
            "        dry_run_exclusion_reason: {},\n",
            rust_string(&row.dry_run.exclusion_reason)
        ));
        output.push_str(&format!(
            "        migration_state: {},\n",
            rust_string(&row.migration_state)
        ));
        output.push_str("    },\n");
    }
    output.push_str("];\n");
    Ok(output)
}

fn render_generated_scenario_index(rows: &[ScenarioRow]) -> Result<String, Vec<String>> {
    let mut output = String::new();
    output.push_str("<!-- BEGIN: mc-compat-generated-scenario-index -->\n");
    output.push_str("<!-- @generated by tools/check_scenario_manifest.rs --write-generated-surfaces; edit compat/config/scenario-manifest.ncl instead. -->\n\n");
    output.push_str("# mc-compat generated scenario index\n\n");
    output.push_str("This bounded index is generated from `compat/config/scenario-manifest.ncl`. It records harness wiring and receipt expectation labels only and does not broaden compatibility claims.\n\n");
    output.push_str(
        "| Scenario | Aliases | Clients | Sessions | Dry-run check | Wrapper | Migration | Receipt expectations |\n",
    );
    output.push_str("| --- | --- | ---: | ---: | --- | --- | --- | --- |\n");
    let mut seen_names = BTreeSet::new();
    for row in rows {
        if !seen_names.insert(row.name.as_str()) {
            return Err(vec![format!(
                "duplicate generated scenario name {}",
                row.name
            )]);
        }
        output.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} | {} |\n",
            markdown_cell(&row.name),
            markdown_cell(&row.aliases.join(", ")),
            row.client_count,
            row.session_count,
            markdown_cell(&empty_as_dash(&row.dry_run.check)),
            markdown_cell(&empty_as_dash(&row.dry_run.wrapper)),
            markdown_cell(&row.migration_state),
            markdown_cell(&row.receipt_expectations.join(", "))
        ));
    }
    output.push_str("\n<!-- END: mc-compat-generated-scenario-index -->\n");
    Ok(output)
}

fn render_generated_scenario_commands(rows: &[ScenarioRow]) -> Result<String, Vec<String>> {
    let mut output = String::new();
    output.push_str("<!-- BEGIN: mc-compat-generated-scenario-commands -->\n");
    output.push_str("<!-- @generated by tools/check_scenario_manifest.rs --write-generated-surfaces; edit compat/config/scenario-manifest.ncl instead. -->\n\n");
    output.push_str("# mc-compat generated scenario commands\n\n");
    output.push_str("This bounded command index is generated from `compat/config/scenario-manifest.ncl`. Commands are harness-shape references only; they do not claim live success, semantic equivalence, public-server safety, production readiness, or broad Minecraft compatibility.\n\n");
    output.push_str(
        "| Scenario | Router dry-run command | Router run command | Wrapper | Dry-run check | Receipt expectations |\n",
    );
    output.push_str("| --- | --- | --- | --- | --- | --- |\n");
    let mut seen_names = BTreeSet::new();
    for row in rows {
        if !seen_names.insert(row.name.as_str()) {
            return Err(vec![format!(
                "duplicate generated scenario name {}",
                row.name
            )]);
        }
        output.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} |\n",
            markdown_cell(&row.name),
            markdown_cell(&format!(
                "`nix run .#mc-compat-smoke -- scenario run {} --dry-run`",
                row.name
            )),
            markdown_cell(&format!(
                "`nix run .#mc-compat-smoke -- scenario run {} --run`",
                row.name
            )),
            markdown_cell(&empty_as_dash(&row.dry_run.wrapper)),
            markdown_cell(&empty_as_dash(&row.dry_run.check)),
            markdown_cell(&row.receipt_expectations.join(", "))
        ));
    }
    output.push_str("\n<!-- END: mc-compat-generated-scenario-commands -->\n");
    Ok(output)
}

fn render_generated_wrapper_metadata(rows: &[ScenarioRow]) -> Result<String, Vec<String>> {
    validate_wrapper_metadata_fields(WRAPPER_METADATA_FIELDS)?;
    let mut output = String::new();
    output
        .push_str("# @generated by tools/check_scenario_manifest.rs --write-generated-surfaces\n");
    output.push_str("# Do not edit by hand; edit compat/config/scenario-manifest.ncl instead.\n\n");
    output.push_str("{\n");
    output.push_str("  schema = \"mc.compat.generated-scenario-wrapper-metadata.v1\";\n");
    output.push_str("  source = \"compat/config/scenario-manifest.ncl\";\n");
    output.push_str("  rows = [\n");
    let mut seen_names = BTreeSet::new();
    for row in rows {
        if !seen_names.insert(row.name.as_str()) {
            return Err(vec![format!(
                "duplicate generated scenario name {}",
                row.name
            )]);
        }
        output.push_str("    {\n");
        output.push_str(&format!("      scenario = {};\n", nix_string(&row.name)));
        output.push_str(&format!(
            "      aliases = {};\n",
            nix_string_array(&row.aliases, "      ")
        ));
        output.push_str(&format!(
            "      appWrapper = {};\n",
            nix_string(&row.dry_run.wrapper)
        ));
        output.push_str(&format!(
            "      dryRunCheck = {};\n",
            nix_string(&row.dry_run.check)
        ));
        output.push_str(&format!(
            "      receiptShapeCheck = {};\n",
            nix_bool(row.dry_run.receipt_shape_check)
        ));
        output.push_str(&format!("      clientCount = {};\n", row.client_count));
        output.push_str(&format!("      sessionCount = {};\n", row.session_count));
        output.push_str(&format!(
            "      migrationState = {};\n",
            nix_string(&row.migration_state)
        ));
        output.push_str("    }\n");
    }
    output.push_str("  ];\n");
    output.push_str(&format!(
        "  appWrappers = {};\n",
        nix_string_array(
            &unique_nonempty_values(rows.iter().map(|row| row.dry_run.wrapper.as_str())),
            "  "
        )
    ));
    output.push_str(&format!(
        "  dryRunChecks = {};\n",
        nix_string_array(
            &unique_nonempty_values(rows.iter().map(|row| row.dry_run.check.as_str())),
            "  "
        )
    ));
    output.push_str("}\n");
    Ok(output)
}

fn validate_wrapper_metadata_fields(fields: &[&str]) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();
    let supported: BTreeSet<&str> = WRAPPER_METADATA_FIELDS.iter().copied().collect();
    let mut seen = BTreeSet::new();
    if fields.is_empty() {
        errors.push("wrapper metadata field list is empty".to_string());
    }
    for field in fields {
        if field.is_empty() {
            errors.push("wrapper metadata field name is empty".to_string());
            continue;
        }
        if !supported.contains(field) {
            errors.push(format!("unknown generated wrapper metadata field {field}"));
        }
        if !seen.insert(*field) {
            errors.push(format!(
                "duplicate generated wrapper metadata field {field}"
            ));
        }
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn unique_nonempty_values<'a>(values: impl Iterator<Item = &'a str>) -> Vec<String> {
    values
        .filter(|value| !value.is_empty())
        .collect::<BTreeSet<&str>>()
        .into_iter()
        .map(str::to_string)
        .collect()
}

fn nix_bool(value: bool) -> &'static str {
    if value {
        "true"
    } else {
        "false"
    }
}

fn nix_string_array(values: &[String], indent: &str) -> String {
    if values.is_empty() {
        return "[ ]".to_string();
    }
    if values.len() <= MINIMUM_POSITIVE_COUNT as usize {
        return format!("[ {} ]", nix_string(&values[0]));
    }
    let child_indent = format!("{indent}  ");
    let mut output = String::from("[\n");
    for value in values {
        output.push_str(&child_indent);
        output.push_str(&nix_string(value));
        output.push('\n');
    }
    output.push_str(indent);
    output.push(']');
    output
}

fn nix_string(value: &str) -> String {
    let mut output = String::with_capacity(value.len() + STRING_QUOTE_OVERHEAD);
    let mut chars = value.chars().peekable();
    output.push('"');
    while let Some(ch) = chars.next() {
        match ch {
            '"' => output.push_str("\\\""),
            '\\' => output.push_str("\\\\"),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            '$' if chars.peek() == Some(&'{') => output.push_str("\\$"),
            ch => output.push(ch),
        }
    }
    output.push('"');
    output
}

fn rust_count_expr(count: u32, one: &str, two: &str) -> String {
    match count {
        1 => one.to_string(),
        2 => two.to_string(),
        other => other.to_string(),
    }
}

fn rust_string_array(values: &[String], indent: &str, line_prefix: &str) -> String {
    if values.is_empty() {
        return "&[]".to_string();
    }
    let rendered = values
        .iter()
        .map(|value| rust_string(value))
        .collect::<Vec<_>>()
        .join(", ");
    let single_line = format!("&[{rendered}]");
    let fits_max_width = line_prefix.len() + single_line.len() + GENERATED_RUST_TRAILING_COMMA_WIDTH
        <= GENERATED_RUST_MAX_WIDTH;
    let fits_collection_width = single_line.len() <= GENERATED_RUST_COLLECTION_WIDTH;
    if fits_max_width && fits_collection_width {
        return single_line;
    }
    let child_indent = format!("{indent}    ");
    let mut output = String::from("&[\n");
    for value in values {
        output.push_str(&child_indent);
        output.push_str(&rust_string(value));
        output.push_str(",\n");
    }
    output.push_str(indent);
    output.push(']');
    output
}

fn rust_string(value: &str) -> String {
    let mut output = String::with_capacity(value.len() + STRING_QUOTE_OVERHEAD);
    output.push('"');
    for ch in value.chars() {
        match ch {
            '"' => output.push_str("\\\""),
            '\\' => output.push_str("\\\\"),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            ch if ch.is_control() => output.push_str(&format!("\\u{{{:x}}}", ch as u32)),
            ch => output.push(ch),
        }
    }
    output.push('"');
    output
}

fn markdown_cell(value: &str) -> String {
    value.replace('|', "\\|").replace('\n', " ")
}

fn empty_as_dash(value: &str) -> String {
    if value.is_empty() {
        "-".to_string()
    } else {
        value.to_string()
    }
}

fn validate_generated_tables(rows: &[ScenarioRow], generated: &str) -> Vec<String> {
    let mut errors = Vec::new();
    for row in rows {
        require_contains(
            &mut errors,
            GENERATED_RUST_PATH,
            generated,
            &format!("name: \"{}\"", row.name),
        );
        require_contains(
            &mut errors,
            GENERATED_RUST_PATH,
            generated,
            &format!("migration_state: \"{}\"", row.migration_state),
        );
        for alias in &row.aliases {
            require_contains(
                &mut errors,
                GENERATED_RUST_PATH,
                generated,
                &format!("\"{alias}\""),
            );
        }
        for milestone in row
            .client_milestones
            .iter()
            .chain(row.server_milestones.iter())
        {
            require_contains(
                &mut errors,
                GENERATED_RUST_PATH,
                generated,
                &format!("\"{milestone}\""),
            );
        }
    }
    errors
}

fn combined_runner_surface(main: &str, scenario_core: &str) -> String {
    format!("{main}\n{scenario_core}")
}

fn validate_runner_surfaces(rows: &[ScenarioRow], runner: &str) -> Vec<String> {
    let mut errors = Vec::new();
    for row in rows {
        require_contains(
            &mut errors,
            RUNNER_SURFACE_PATH,
            runner,
            &format!("\"{}\"", row.name),
        );
        for alias in &row.aliases {
            require_contains(
                &mut errors,
                RUNNER_SURFACE_PATH,
                runner,
                &format!("\"{alias}\""),
            );
        }
        for milestone in row
            .client_milestones
            .iter()
            .chain(row.server_milestones.iter())
        {
            require_contains(
                &mut errors,
                RUNNER_SURFACE_PATH,
                runner,
                &format!("\"{milestone}\""),
            );
        }
        for forbidden in &row.forbidden_patterns {
            require_contains(
                &mut errors,
                RUNNER_SURFACE_PATH,
                runner,
                &format!("\"{forbidden}\""),
            );
        }
    }
    errors
}

fn validate_live_capability_registry_surface(scenario_core: &str) -> Vec<String> {
    live_capability_registry_diagnostics(&evaluate_live_capability_registry_surface(
        scenario_core,
        LIVE_CAPABILITY_REGISTRY_TOKENS,
    ))
}

fn evaluate_live_capability_registry_surface<'a>(
    scenario_core: &str,
    expected_tokens: &'a [&'a str],
) -> LiveCapabilityRegistryEvaluation<'a> {
    let missing = expected_tokens
        .iter()
        .copied()
        .filter(|token| !scenario_core.contains(token))
        .map(|token| MissingLiveCapability {
            path: RUNNER_SCENARIO_CORE_PATH,
            token,
        })
        .collect();
    LiveCapabilityRegistryEvaluation { missing }
}

fn live_capability_registry_diagnostics(
    evaluation: &LiveCapabilityRegistryEvaluation<'_>,
) -> Vec<String> {
    evaluation
        .missing
        .iter()
        .map(|missing| format!("{} missing {:?}", missing.path, missing.token))
        .collect()
}

fn validate_flake_surfaces(rows: &[ScenarioRow], flake: &str) -> Vec<String> {
    let mut errors = Vec::new();
    for row in rows {
        if row.dry_run.check.is_empty() {
            continue;
        }
        require_contains(&mut errors, FLAKE_PATH, flake, &row.dry_run.check);
        require_contains(&mut errors, FLAKE_PATH, flake, &row.name);
    }
    errors
}

fn validate_readme_doc_links(readme: &str) -> Vec<String> {
    README_REQUIRED_DOC_LINKS
        .iter()
        .copied()
        .filter(|link| !readme.contains(link))
        .map(|link| format!("{README_PATH} missing moved-doc link {link:?}"))
        .collect()
}

fn validate_scenario_command_docs(rows: &[ScenarioRow], scenario_commands: &str) -> Vec<String> {
    let mut errors = Vec::new();
    for row in rows {
        let wrapper_documented =
            !row.dry_run.wrapper.is_empty() && scenario_commands.contains(&row.dry_run.wrapper);
        if scenario_commands.contains(&row.name) || wrapper_documented {
            continue;
        }
        if row.dry_run.exclusion_reason.is_empty() {
            errors.push(format!(
                "{} missing from {} command listings without exclusion",
                row.name, SCENARIO_COMMANDS_DOC_PATH
            ));
        }
    }
    errors
}

fn validate_surface_inventory(inventory: &str) -> Vec<String> {
    REQUIRED_SURFACE_INVENTORY_TOKENS
        .iter()
        .copied()
        .filter(|token| !inventory.contains(token))
        .map(|token| format!("{SURFACE_INVENTORY_PATH} missing {token:?}"))
        .collect()
}

fn validate_current_bundle_surfaces(rows: &[ScenarioRow], current_bundle: &str) -> Vec<String> {
    let mut errors = Vec::new();
    let lower_bundle = current_bundle.to_ascii_lowercase();
    for row in rows {
        if row.current_bundle_row.is_empty() {
            continue;
        }
        let needle = row.current_bundle_row.to_ascii_lowercase();
        if !lower_bundle.contains(&needle) {
            errors.push(format!(
                "{} current bundle row marker {:?} missing",
                row.name, row.current_bundle_row
            ));
        }
    }
    errors
}

fn require_contains(errors: &mut Vec<String>, path: &str, haystack: &str, needle: &str) {
    if !haystack.contains(needle) {
        errors.push(format!("{path} missing {needle:?}"));
    }
}

fn run_self_tests() -> Result<String, Vec<String>> {
    let cases = [
        ("valid", valid_fixture(), true),
        ("waiver_backed", waiver_fixture(), true),
        ("typed_event_ready", typed_event_ready_fixture(), true),
        ("duplicate", duplicate_fixture(), false),
        ("missing_alias", missing_alias_fixture(), false),
        ("missing_milestone", missing_milestone_fixture(), false),
        ("invalid_wrapper", invalid_wrapper_fixture(), false),
        (
            "missing_waiver_wrapper",
            missing_waiver_wrapper_fixture(),
            false,
        ),
        ("empty_waiver", empty_waiver_fixture(), false),
        ("incomplete_waiver", incomplete_waiver_fixture(), false),
        ("stale_waiver", stale_waiver_fixture(), false),
        (
            "missing_typed_event_client",
            missing_typed_event_client_fixture(),
            false,
        ),
        (
            "missing_typed_event_server",
            missing_typed_event_server_fixture(),
            false,
        ),
        (
            "missing_typed_event_forbidden",
            missing_typed_event_forbidden_fixture(),
            false,
        ),
        (
            "missing_typed_event_fallback_waiver",
            missing_typed_event_fallback_waiver_fixture(),
            false,
        ),
        (
            "covered_row_with_waiver",
            covered_row_with_waiver_fixture(),
            false,
        ),
        (
            "unsupported_migration",
            unsupported_migration_fixture(),
            false,
        ),
    ];
    let mut errors = Vec::new();
    for (name, fixture, should_pass) in cases {
        let result = parse_manifest(&fixture).and_then(|manifest| validate_manifest(&manifest));
        if result.is_ok() != should_pass {
            errors.push(format!("self-test case {name} expected pass={should_pass}"));
        }
    }
    let manifest = parse_manifest(&valid_fixture()).expect("valid fixture parses");
    let dry_run_coverage = evaluate_dry_run_coverage(&manifest.rows);
    if !dry_run_coverage.is_complete() {
        errors.push("self-test case valid dry-run coverage expected pass=true".to_string());
    }
    let typed_event_readiness = evaluate_typed_event_readiness(&manifest);
    if !typed_event_readiness.is_complete() {
        errors.push("self-test case valid typed-event fallback expected pass=true".to_string());
    }
    let ready_manifest =
        parse_manifest(&typed_event_ready_fixture()).expect("ready fixture parses");
    let ready_evaluation = evaluate_typed_event_readiness(&ready_manifest);
    if !ready_evaluation.is_complete() {
        errors.push("self-test case typed-event readiness expected pass=true".to_string());
    }
    let fallback_budget =
        evaluate_fallback_budget(&manifest.rows, &fallback_budget_baseline_fixture());
    if !fallback_budget.is_complete()
        || fallback_budget.approved_fallback_rows != vec!["smoke".to_string()]
    {
        errors.push(
            "self-test case fallback budget unchanged baseline expected pass=true".to_string(),
        );
    }
    let fallback_removed =
        evaluate_fallback_budget(&ready_manifest.rows, &fallback_budget_baseline_fixture());
    if !fallback_removed.is_complete()
        || fallback_removed.removed_fallback_rows != vec!["smoke".to_string()]
    {
        errors.push("self-test case fallback budget removal expected pass=true".to_string());
    }
    let new_fallback =
        evaluate_fallback_budget(&manifest.rows, &empty_fallback_budget_baseline_fixture());
    if new_fallback.is_complete()
        || !new_fallback
            .new_fallback_rows
            .iter()
            .any(|row| row == "smoke")
    {
        errors.push("self-test case fallback budget new fallback expected pass=false".to_string());
    }
    let missing_waiver = evaluate_fallback_budget(
        &manifest.rows,
        &missing_waiver_fallback_budget_baseline_fixture(),
    );
    if missing_waiver.is_complete() || missing_waiver.missing_waiver_metadata.is_empty() {
        errors
            .push("self-test case fallback budget missing waiver expected pass=false".to_string());
    }
    let typed_event_regression = evaluate_fallback_budget(
        &manifest.rows,
        &typed_event_ready_fallback_budget_baseline_fixture(),
    );
    if typed_event_regression.is_complete()
        || !typed_event_regression
            .typed_event_regressions
            .iter()
            .any(|row| row == "smoke")
    {
        errors.push(
            "self-test case fallback budget typed-event regression expected pass=false".to_string(),
        );
    }
    let generated_surfaces = render_generated_surfaces(&manifest.rows);
    if let Err(generator_errors) = &generated_surfaces {
        errors.push(format!(
            "self-test case generated_surfaces expected pass=true: {generator_errors:?}"
        ));
    }
    if let Ok(surfaces) = &generated_surfaces {
        let wrapper_metadata = surfaces
            .iter()
            .find(|surface| surface.path == GENERATED_WRAPPER_METADATA_PATH);
        match wrapper_metadata {
            Some(surface)
                if surface
                    .content
                    .contains("appWrapper = \"mc-compat-smoke\";")
                    && surface.content.contains("dryRunChecks") => {}
            Some(_) => errors.push(
                "self-test case generated_wrapper_metadata expected wrapper/check fields"
                    .to_string(),
            ),
            None => errors
                .push("self-test case generated_wrapper_metadata expected surface".to_string()),
        }
        let scenario_commands = surfaces
            .iter()
            .find(|surface| surface.path == GENERATED_SCENARIO_COMMANDS_PATH);
        match scenario_commands {
            Some(surface)
                if surface.content.contains("scenario run smoke --dry-run")
                    && surface.content.contains("Receipt expectations") => {}
            Some(_) => errors.push(
                "self-test case generated_scenario_commands expected router command fields"
                    .to_string(),
            ),
            None => errors
                .push("self-test case generated_scenario_commands expected surface".to_string()),
        }
    }
    let complete_readme_links = README_REQUIRED_DOC_LINKS.join("\n");
    if !validate_readme_doc_links(&complete_readme_links).is_empty() {
        errors.push("self-test case readme_doc_links expected pass=true".to_string());
    }
    if validate_readme_doc_links(SCENARIO_COMMANDS_DOC_PATH).is_empty() {
        errors.push("self-test case missing_readme_doc_link expected pass=false".to_string());
    }
    let complete_scenario_commands = render_generated_scenario_commands(&manifest.rows)
        .expect("valid fixture scenario commands render");
    if !validate_scenario_command_docs(&manifest.rows, &complete_scenario_commands).is_empty() {
        errors.push("self-test case scenario_command_docs expected pass=true".to_string());
    }
    if validate_scenario_command_docs(&manifest.rows, "unrelated docs").is_empty() {
        errors.push("self-test case missing_scenario_command_docs expected pass=false".to_string());
    }
    if validate_generated_output_path("../escape.rs").is_ok() {
        errors.push("self-test case unsafe_generated_output_path expected pass=false".to_string());
    }
    if validate_wrapper_metadata_fields(&["scenario", "unknownField"]).is_ok() {
        errors.push(
            "self-test case unknown_generated_wrapper_metadata_field expected pass=false"
                .to_string(),
        );
    }
    if generated_surface_stale_diagnostic(GENERATED_WRAPPER_METADATA_PATH, "expected", "stale")
        .is_none()
    {
        errors.push("self-test case stale_generated_surface expected pass=false".to_string());
    }
    let duplicate_manifest =
        parse_manifest(&duplicate_fixture()).expect("duplicate fixture parses");
    if render_generated_surfaces(&duplicate_manifest.rows).is_ok() {
        errors.push("self-test case duplicate_generated_surface expected pass=false".to_string());
    }
    let split_surface = combined_runner_surface("", "\"smoke\"\n\"protocol_detected\"\n\"panic\"");
    if !validate_runner_surfaces(&manifest.rows, &split_surface).is_empty() {
        errors.push("self-test case split_runner_surface expected pass=true".to_string());
    }
    if validate_runner_surfaces(&manifest.rows, "\"smoke\"").is_empty() {
        errors.push("self-test case missing_split_runner_surface expected pass=false".to_string());
    }
    let live_registry_surface = LIVE_CAPABILITY_REGISTRY_TOKENS.join("\n");
    let complete_registry = evaluate_live_capability_registry_surface(
        &live_registry_surface,
        LIVE_CAPABILITY_REGISTRY_TOKENS,
    );
    if !complete_registry.is_complete()
        || !live_capability_registry_diagnostics(&complete_registry).is_empty()
    {
        errors
            .push("self-test case live_capability_registry_surface expected pass=true".to_string());
    }
    let missing_registry = evaluate_live_capability_registry_surface(
        "ScenarioLiveCapability",
        LIVE_CAPABILITY_REGISTRY_TOKENS,
    );
    if missing_registry.is_complete()
        || live_capability_registry_diagnostics(&missing_registry).is_empty()
    {
        errors.push(
            "self-test case missing_live_capability_registry_surface expected pass=false"
                .to_string(),
        );
    }
    let malformed_registry = evaluate_live_capability_registry_surface(
        "UnknownLiveCapability\nresource-pack-status",
        LIVE_CAPABILITY_REGISTRY_TOKENS,
    );
    let malformed_errors = live_capability_registry_diagnostics(&malformed_registry);
    if malformed_registry.is_complete()
        || !malformed_errors
            .iter()
            .any(|error| error.contains("ResourcePackStatusLocalContract"))
    {
        errors.push(
            "self-test case malformed_live_capability_registry_surface expected fail-closed diagnostic"
                .to_string(),
        );
    }
    let complete_inventory = REQUIRED_SURFACE_INVENTORY_TOKENS.join("\n");
    if !validate_surface_inventory(&complete_inventory).is_empty() {
        errors.push("self-test case surface_inventory expected pass=true".to_string());
    }
    if validate_surface_inventory(MANIFEST_PATH).is_empty() {
        errors.push("self-test case missing_surface_inventory expected pass=false".to_string());
    }

    let stale_revision_registry = evaluate_live_capability_registry_surface(
        "ScenarioLiveCapability\nlive.revision.status = stale",
        LIVE_CAPABILITY_REGISTRY_TOKENS,
    );
    if stale_revision_registry.is_complete()
        || live_capability_registry_diagnostics(&stale_revision_registry).is_empty()
    {
        errors.push(
            "self-test case stale_revision_live_capability_registry_surface expected fail-closed diagnostic"
                .to_string(),
        );
    }
    let overclaim_registry = evaluate_live_capability_registry_surface(
        "ScenarioLiveCapability\nbroad_minecraft_compatibility = true",
        LIVE_CAPABILITY_REGISTRY_TOKENS,
    );
    if overclaim_registry.is_complete()
        || live_capability_registry_diagnostics(&overclaim_registry).is_empty()
    {
        errors.push(
            "self-test case overclaim_live_capability_registry_surface expected fail-closed diagnostic"
                .to_string(),
        );
    }

    if errors.is_empty() {
        Ok("positive and negative fixtures exercised, including wrapper metadata, README links, generated command docs, and stale-output rejection".to_string())
    } else {
        Err(errors)
    }
}

fn fixture_with_row(row: &str) -> String {
    fixture_with_row_and_typed_event_waiver(row, complete_typed_event_fallback_waiver_metadata())
}

fn fixture_with_row_and_typed_event_waiver(row: &str, waiver: &str) -> String {
    format!(
        "schema = \"{SUPPORTED_SCHEMA}\"\ntyped_event_fallback_waiver = \"{waiver}\"\nscenarios = [\n{{\n{row}\n}},\n],\n"
    )
}

fn complete_typed_event_fallback_waiver_metadata() -> &'static str {
    "owner=mc-compat; reason=legacy rows still rely on substring log evidence; non_claim=typed-event migration changes observability only; next_action=migrate rows when typed-event fixtures cover client server and forbidden surfaces"
}

fn valid_row() -> &'static str {
    "name = \"smoke\",\naliases = [\"smoke\"],\nclient_milestones = [\"protocol_detected\"],\nserver_milestones = [],\nforbidden_patterns = [\"panic\"],\nclient_count = 1,\nsession_count = 1,\nmaintained = true,\ndry_run = { check = \"mc-compat-dry-run\", wrapper = \"mc-compat-smoke\", receipt_shape_check = true, exclusion_reason = \"\" },\nreceipt_expectations = [\"schema\"],\nmigration_state = \"substring-fallback\",\ncurrent_bundle_row = \"\",\ncurrent_bundle_exclusion_reason = \"harness row\","
}

fn valid_fixture() -> String {
    fixture_with_row(valid_row())
}

fn duplicate_fixture() -> String {
    let row = valid_row();
    let waiver = complete_typed_event_fallback_waiver_metadata();
    format!(
        "schema = \"{SUPPORTED_SCHEMA}\"\ntyped_event_fallback_waiver = \"{waiver}\"\nscenarios = [\n{{\n{row}\n}},\n{{\n{row}\n}},\n],\n"
    )
}

fn missing_alias_fixture() -> String {
    fixture_with_row(&valid_row().replace("aliases = [\"smoke\"]", "aliases = []"))
}

fn missing_milestone_fixture() -> String {
    fixture_with_row(&valid_row().replace(
        "client_milestones = [\"protocol_detected\"]",
        "client_milestones = []",
    ))
}

fn invalid_wrapper_fixture() -> String {
    fixture_with_row(&valid_row().replace(
        "dry_run = { check = \"mc-compat-dry-run\", wrapper = \"mc-compat-smoke\", receipt_shape_check = true, exclusion_reason = \"\" }",
        "dry_run = { check = \"\", wrapper = \"\", receipt_shape_check = false, exclusion_reason = \"\" }",
    ))
}

fn complete_waiver_metadata() -> &'static str {
    "owner=mc-compat; reason=paired reference comparator remains the review source; non_claim=dry-run shape coverage would not promote live parity; next_action=add dedicated wrapper after comparator fixture review"
}

fn waiver_row() -> String {
    valid_row().replace(
        "dry_run = { check = \"mc-compat-dry-run\", wrapper = \"mc-compat-smoke\", receipt_shape_check = true, exclusion_reason = \"\" }",
        &format!(
            "dry_run = {{ check = \"\", wrapper = \"mc-compat-smoke\", receipt_shape_check = false, exclusion_reason = \"{}\" }}",
            complete_waiver_metadata()
        ),
    )
}

fn waiver_fixture() -> String {
    fixture_with_row(&waiver_row())
}

fn missing_waiver_wrapper_fixture() -> String {
    fixture_with_row(&waiver_row().replace("wrapper = \"mc-compat-smoke\"", "wrapper = \"\""))
}

fn empty_waiver_fixture() -> String {
    fixture_with_row(&waiver_row().replace(complete_waiver_metadata(), ""))
}

fn incomplete_waiver_fixture() -> String {
    fixture_with_row(&waiver_row().replace(
        "; next_action=add dedicated wrapper after comparator fixture review",
        "",
    ))
}

fn stale_waiver_fixture() -> String {
    fixture_with_row(&waiver_row().replace(
        complete_waiver_metadata(),
        "owner=mc-compat; reason=covered by historical live receipt and not yet by a dedicated flake dry-run wrapper; non_claim=dry-run shape coverage would not promote live parity; next_action=add dedicated wrapper after comparator fixture review",
    ))
}

fn covered_row_with_waiver_fixture() -> String {
    fixture_with_row(&valid_row().replace(
        "exclusion_reason = \"\"",
        &format!("exclusion_reason = \"{}\"", complete_waiver_metadata()),
    ))
}

fn typed_event_ready_row() -> String {
    valid_row().replace(
        "migration_state = \"substring-fallback\"",
        "migration_state = \"typed-event-ready\"",
    )
}

fn typed_event_ready_fixture() -> String {
    fixture_with_row(&typed_event_ready_row())
}

fn missing_typed_event_client_fixture() -> String {
    fixture_with_row(&typed_event_ready_row().replace(
        "client_milestones = [\"protocol_detected\"]",
        "client_milestones = [\"missing_client_event\"]",
    ))
}

fn missing_typed_event_server_fixture() -> String {
    fixture_with_row(&typed_event_ready_row().replace(
        "server_milestones = []",
        "server_milestones = [\"missing_server_event\"]",
    ))
}

fn missing_typed_event_forbidden_fixture() -> String {
    fixture_with_row(&typed_event_ready_row().replace(
        "forbidden_patterns = [\"panic\"]",
        "forbidden_patterns = [\"unmapped_forbidden_event\"]",
    ))
}

fn missing_typed_event_fallback_waiver_fixture() -> String {
    fixture_with_row_and_typed_event_waiver(valid_row(), "")
}

fn unsupported_migration_fixture() -> String {
    fixture_with_row(&valid_row().replace(
        "migration_state = \"substring-fallback\"",
        "migration_state = \"magic\"",
    ))
}

fn fallback_budget_baseline_fixture() -> FallbackBudgetBaseline {
    FallbackBudgetBaseline {
        schema: FALLBACK_BUDGET_BASELINE_SCHEMA.to_string(),
        fallback_rows: vec![fallback_budget_entry("smoke")],
        typed_event_ready_rows: Vec::new(),
    }
}

fn empty_fallback_budget_baseline_fixture() -> FallbackBudgetBaseline {
    FallbackBudgetBaseline {
        schema: FALLBACK_BUDGET_BASELINE_SCHEMA.to_string(),
        fallback_rows: Vec::new(),
        typed_event_ready_rows: Vec::new(),
    }
}

fn missing_waiver_fallback_budget_baseline_fixture() -> FallbackBudgetBaseline {
    FallbackBudgetBaseline {
        schema: FALLBACK_BUDGET_BASELINE_SCHEMA.to_string(),
        fallback_rows: vec![FallbackBudgetEntry {
            name: "smoke".to_string(),
            owner: String::new(),
            reason: "legacy row".to_string(),
            non_claim: "observability only".to_string(),
            next_action: "migrate typed-event fixture".to_string(),
        }],
        typed_event_ready_rows: Vec::new(),
    }
}

fn typed_event_ready_fallback_budget_baseline_fixture() -> FallbackBudgetBaseline {
    FallbackBudgetBaseline {
        schema: FALLBACK_BUDGET_BASELINE_SCHEMA.to_string(),
        fallback_rows: Vec::new(),
        typed_event_ready_rows: vec!["smoke".to_string()],
    }
}

fn fallback_budget_entry(name: &str) -> FallbackBudgetEntry {
    FallbackBudgetEntry {
        name: name.to_string(),
        owner: "mc-compat".to_string(),
        reason: "legacy rows still rely on substring log evidence".to_string(),
        non_claim: "typed-event migration changes observability only".to_string(),
        next_action: "migrate rows when typed-event fixtures cover surfaces".to_string(),
    }
}
