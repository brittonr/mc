mod backend_shell;
mod client_driver;
mod config_patches;
mod env_patch;
mod evidence_bundle;
mod evidence_core;
mod evidence_receipts;
mod evidence_types;
mod json_support;
mod layout;
mod planning;
mod receipt_validation;
mod receipts;
mod runtime_config;
mod scenario_catalog;
mod scenario_contracts_generated;
mod scenario_core;
#[allow(dead_code)]
mod scenario_manifest_generated;
mod wire;

use backend_shell::{
    build_git_revision_evidence, child_revision_evidence_for_receipt, cleanup_harness_state,
    cleanup_path, ensure_valence_repo_ready, force_stop_paper_server, force_stop_server,
    force_stop_valence_server, git_rev_parse, git_revision_evidence, is_mc_compat_client_log,
    print_harness_status, probe_status, start_paper_server, start_server, start_valence_server,
    stop_paper_server, stop_server, stop_valence_server, valence_source_dir,
};
use client_driver::{
    client_timeout_secs, derive_survival_crash_recovery_client_milestones,
    derive_survival_crash_recovery_server_milestones, mcp_control_dry_run_control_evidence,
    mcp_controlled_dry_run_evidence, mcp_controlled_success_output, planned_client_usernames,
    projectile_damage_dry_run_evidence, projectile_travel_collision_dry_run_evidence,
    read_paper_log, read_valence_log, requires_server_correlation, run_client, server_log_label,
    uses_isolated_restart_storage, world_persistence_phase_value,
    world_persistence_pre_restart_server_log_path, world_persistence_restart_phase_path,
    world_persistence_state_dir,
};
use env_patch::{EnvPatch, EnvPatchDiagnostic};
use evidence_bundle::*;
#[cfg(test)]
use evidence_core::{
    biome_dimension_join_state_required_non_claims, evaluate_scenario,
    evaluate_scenario_with_projectile_health, parse_typed_event_line,
    validate_biome_dimension_join_state_record,
};
use evidence_core::{
    evaluate_biome_dimension_join_state, evaluate_projectile_damage_causality,
    evaluate_projectile_damage_causality_for_damage, evaluate_projectile_travel_collision,
    evaluate_scenario_for_config, evaluate_server_scenario, evaluate_typed_event_graph,
    normalize_typed_event_timeline, typed_event_oracle_contributes_to_pass_fail,
    typed_event_oracle_receipt_json, typed_event_ordered_edges_for_scenario,
    typed_event_required_events_for_graph, typed_event_timeline_blake3,
    typed_events_from_receipt_evidence, validate_typed_event_oracle_for_migrated_scenario,
};
use evidence_receipts::*;
use evidence_types::*;
use json_support::*;
use planning::{
    format_plan_diagnostics, harness_plan_from_config, log_harness_plan, scenario_route_non_claims,
};
use receipts::smoke_receipt_json_with_typed_event_oracle;
#[cfg(test)]
use receipts::{build_enriched_triage, smoke_receipt_json, EnrichedTriageInput};
use wire::{McRead, McWrite};

use layout::{resolve_repository_layout, resolve_valence_source_dir, LayoutResolutionMode};
#[cfg(test)]
use receipt_validation::validate_receipt_summary;
use receipt_validation::{read_receipt_summary_from_text, validate_receipt_pair, ReceiptSummary};
use scenario_catalog::*;
use scenario_core::{
    parse_scenario, scenario_behavior_kind, scenario_behavior_metadata,
    scenario_forbidden_patterns, scenario_name, scenario_required_milestones,
    server_required_milestones, validate_static_scenario_specs,
};
use scenario_core::{
    NegativeLiveRailBehavior, ProbeTeam, COMBAT_ATTACKER_ROLE, COMBAT_TARGET_USERNAME,
    COMBAT_VICTIM_ROLE, EQUIPMENT_UPDATE_CLIENT_COUNT_NEEDLE, FIRST_CLIENT_INDEX,
    FLAG_CARRIER_DEATH_PICKUP_FIRST_TICK, MULTI_CLIENT_LOAD_COUNT_NEEDLE, MULTI_CLIENT_READY_COUNT,
    PROBE_ENABLED_VALUE, PROBE_REPEAT_DOUBLE, PROBE_REPEAT_SINGLE,
    PROJECTILE_DAMAGE_CLIENT_COUNT_NEEDLE, PROJECTILE_HIT_CLIENT_COUNT_NEEDLE,
    RECONNECT_SESSION_COUNT_NEEDLE, SECOND_CLIENT_INDEX, SESSION_INDEX_ENV_OFFSET, TEAM_BLUE_VALUE,
    TEAM_RED_VALUE,
};
use scenario_core::{Scenario, ScenarioBehaviorKind, ScenarioRunStrategy, SCENARIO_SPECS};
#[cfg(test)]
use scenario_core::{ScenarioMilestone, ALL_SCENARIOS};

use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitCode, Stdio};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const DEFAULT_VALENCE_REV: &str = "8ad9c85";
const DEFAULT_VALENCE_EXAMPLE: &str = "terrain";
const DEFAULT_SERVER_VERSION: &str = "1.18.2";
const CARGO_MANIFEST_FILE: &str = "Cargo.toml";
const GIT_HEAD_REV: &str = "HEAD";
const GIT_CURRENT_DIR_PATHSPEC: &str = ".";
const GIT_LOG_COMMIT_FORMAT: &str = "--format=%H";
const GIT_DIRTY_SKIPPED_LAYOUT_DIAGNOSTIC: &str =
    "skipped dirty-state check because source layout did not resolve";
const DEFAULT_SERVER_PROTOCOL: u32 = 758;
const DEFAULT_CLIENT_USERNAME: &str = "compatbot";
const DEFAULT_CLIENT_TIMEOUT_SECS: u64 = 20;
const STATUS_SOCKET_TIMEOUT_SECS: u64 = 2;
const STATUS_LOCALHOST_ADDRESS: &str = "127.0.0.1";
const STATUS_PACKET_ID: u32 = 0;
const STATUS_HANDSHAKE_NEXT_STATE: u32 = 1;
const VARINT_SEGMENT_BITS: u32 = 7;
const VARINT_SEGMENT_BITS_USIZE: usize = VARINT_SEGMENT_BITS as usize;
const VARINT_SEGMENT_MASK: u32 = 0x7f;
const VARINT_CONTINUATION_BIT: u8 = 0x80;
const VARINT_MAX_SHIFT_EXCLUSIVE: u32 = 35;
const MULTI_CLIENT_LOAD_PEER_TIMEOUT_SECS: u64 = 10;
const PAPER_CONNECTION_THROTTLE_CLEAR_SECS: u64 = 5;
const MULTI_CLIENT_START_STAGGER_SECS: u64 = PAPER_CONNECTION_THROTTLE_CLEAR_SECS;
const CTF_RACE_ACCEPT_CLIENT_FIRST_TICK: u32 = 760;
const CTF_RACE_REJECT_CLIENT_FIRST_TICK: u32 = 800;
const PAPER_PLUGIN_CONTAINER_DIR: &str = "/plugins";
const PAPER_EULA_ACCEPTED_VALUE: &str = "TRUE";
const PAPER_SERVER_TYPE: &str = "PAPER";
const PAPER_ONLINE_MODE_VALUE: &str = "FALSE";
const PAPER_MEMORY_LIMIT: &str = "1G";
const PAPER_GRACEFUL_STOP_TIMEOUT_SECS: u64 = 60;
const PAPER_VIEW_DISTANCE: u32 = 2;
const PAPER_SIMULATION_DISTANCE: u32 = 2;
const SAFETY_MAX_LOCAL_CLIENTS: usize = 2;
const SAFETY_MAX_DURATION_SECS: u64 = 600;
const SAFETY_SINGLE_SESSION_COUNT: usize = 1;
const SAFETY_RECONNECT_SESSION_COUNT: usize = 2;
const RECONNECT_SEQUENCE_SESSION_COUNT: usize = 2;
const RECONNECT_SEQUENCE_PAUSE_SECS: u64 = 4;
const SAFETY_ZERO_VALUE: &str = "0";
const SAFETY_OWNED_LOCAL_SCOPE: &str = "owned-local-loopback";
const PINNED_PROJECTILE_DAMAGE_VALENCE_REV: &str = "e5d18ad04010d92881267ac1ea43922ae91821f5";
const PROJECTILE_DAMAGE_ATTACKER_SUFFIX: &str = "a";
const PROJECTILE_DAMAGE_VICTIM_SUFFIX: &str = "b";
const PROJECTILE_DAMAGE_CLIENT_USE_NEEDLE: &str = "projectile_probe_use_item_sent";
const PROJECTILE_DAMAGE_CLIENT_SWING_NEEDLE: &str = "projectile_probe_swing_sent";
#[cfg(test)]
const PROJECTILE_DAMAGE_CLIENT_HEALTH_NEEDLE: &str = "update_health health=17.0";
const PROJECTILE_DAMAGE_SERVER_USE_NEEDLE: &str = "MC-COMPAT-MILESTONE projectile_use";
const PROJECTILE_DAMAGE_SERVER_HIT_NEEDLE: &str = "MC-COMPAT-MILESTONE projectile_hit";
const PROJECTILE_DAMAGE_SEQUENCE_NEEDLE: &str = "sequence=303";
const PROJECTILE_DAMAGE_AMOUNT_NEEDLE: &str = "damage=3.0";
const PROJECTILE_TRAVEL_COLLISION_ROW_ID: &str = "bow_arrow_synthetic_use_to_hit";
const PROJECTILE_TRAVEL_COLLISION_WEAPON: &str = "Bow";
const PROJECTILE_TRAVEL_COLLISION_WEAPON_REPRESENTATIVE: &str = "bow_like_projectile_probe";
const PROJECTILE_TRAVEL_COLLISION_PROJECTILE_REPRESENTATIVE: &str = "arrow_like_probe";
const PROJECTILE_TRAVEL_COLLISION_PROJECTILE_ID: &str = "arrow_probe_sequence_303";
const PROJECTILE_TRAVEL_COLLISION_CLIENT_SPAWN_NEEDLE: &str = "projectile_probe_spawn_visible";
const PROJECTILE_TRAVEL_COLLISION_CLIENT_TRAVEL_NEEDLE: &str = "projectile_probe_travel_observed";
const PROJECTILE_TRAVEL_COLLISION_SERVER_TRAVEL_NEEDLE: &str =
    "MC-COMPAT-MILESTONE projectile_travel_sample";
const PROJECTILE_TRAVEL_COLLISION_SERVER_COLLISION_NEEDLE: &str =
    "MC-COMPAT-MILESTONE projectile_collision";
const PROJECTILE_TRAVEL_COLLISION_SAMPLE_KIND: &str = "synthetic_midpoint";
const PROJECTILE_TRAVEL_COLLISION_SAMPLE_INDEX: u32 = 1;
const PROJECTILE_TRAVEL_COLLISION_COLLISION_KIND: &str = "synthetic_entity_hit";
const PROJECTILE_TRAVEL_COLLISION_PROOF_BASIS: &str = "bounded_fixture_not_entity_physics";
const PROJECTILE_TRAVEL_COLLISION_POLICY_ID: &str = "damage-linear";
const PROJECTILE_TRAVEL_COLLISION_POLICY_GENERATION: u64 = 0;
const PROJECTILE_TRAVEL_COLLISION_LOADOUT_ARROW_COUNT: u32 = 16;
const PROJECTILE_TRAVEL_COLLISION_VICTIM_END_HEALTH: f64 = 17.0;
const PROJECTILE_TRAVEL_COLLISION_FORBIDDEN_PARITY_CLAIMS: &[&str] = &[
    "claim=exact_vanilla_projectile_parity",
    "claim=full_projectile_physics",
    "claims_exact_vanilla_parity=true",
];
const PROJECTILE_TRAVEL_COLLISION_NON_CLAIMS: &[&str] = &[
    "not_full_projectile_physics",
    "not_entity_spawn_or_ballistics",
    "not_exact_vanilla_projectile_parity",
    "not_public_server_safety",
    "not_production_readiness",
];
const DEFAULT_ARROW_DAMAGE: f64 = 3.0;
const DEFAULT_ARROW_VELOCITY_MULTIPLIER: f64 = 1.0;
const DEFAULT_ARROW_MAX_DAMAGE: f64 = 10.0;
const GIT_REV_DRY_RUN_PLACEHOLDER: &str = "dry-run";
const GIT_STATUS_CLEAN: &str = "clean";
const GIT_STATUS_DIRTY: &str = "dirty";
const GIT_STATUS_DRY_RUN: &str = "dry-run";
const GIT_STATUS_UNAVAILABLE: &str = "unavailable";
const GIT_STATUS_PORCELAIN_FLAG: &str = "--porcelain";
const PROJECTILE_DAMAGE_CONTEXT_VELOCITY: f64 = 0.0;
const PROJECTILE_DAMAGE_CONTEXT_PULL_STRENGTH: f64 = 1.0;
const XVFB_SERVER_ARGS: &str = "-screen 0 1280x720x24 +extension GLX +render -noreset";
const ENV_SOURCE_BUILD: &str = "build-env";
const ENV_SOURCE_HEADLESS: &str = "headless-x11-env";
const ENV_SOURCE_CLIENT_SCENARIO: &str = "client-scenario-env";
const ENV_SOURCE_VALENCE_SCENARIO: &str = "valence-scenario-env";
const ENV_SOURCE_VALENCE_STEEL_CONFIG: &str = "valence-steel-config-env";
const ENV_SOURCE_PAPER_BASE: &str = "paper-base-env";
const ENV_SOURCE_PAPER_SCENARIO: &str = "paper-scenario-env";
const PROJECTILE_DAMAGE_VICTIM_START_HEALTH: f64 = 20.0;
const INVENTORY_STACK_SPLIT_MERGE_PROBE_ENV: &str =
    scenario_contracts_generated::MC_COMPAT_INVENTORY_STACK_SPLIT_MERGE_PROBE;
const INVENTORY_DRAG_TRANSACTIONS_PROBE_ENV: &str =
    scenario_contracts_generated::MC_COMPAT_INVENTORY_DRAG_TRANSACTIONS_PROBE;
const SURVIVAL_CHEST_FIXTURE_ENV: &str =
    scenario_contracts_generated::MC_COMPAT_SURVIVAL_CHEST_FIXTURE;
const SURVIVAL_CRAFTING_FIXTURE_ENV: &str = "MC_COMPAT_SURVIVAL_CRAFTING_FIXTURE";
const SURVIVAL_CRAFTING_BREADTH_PROBE_ENV: &str = "MC_COMPAT_SURVIVAL_CRAFTING_BREADTH_PROBE";
const SURVIVAL_CRAFTING_BREADTH_FIXTURE_ENV: &str = "MC_COMPAT_SURVIVAL_CRAFTING_BREADTH_FIXTURE";
const SURVIVAL_FURNACE_PROBE_ENV: &str = "MC_COMPAT_SURVIVAL_FURNACE_PROBE";
const SURVIVAL_FURNACE_SMELTING_BREADTH_PROBE_ENV: &str =
    "MC_COMPAT_SURVIVAL_FURNACE_SMELTING_BREADTH_PROBE";
const SURVIVAL_FURNACE_SESSION_ENV: &str = "MC_COMPAT_SURVIVAL_FURNACE_SESSION";
const SURVIVAL_FURNACE_FIXTURE_ENV: &str = "MC_COMPAT_SURVIVAL_FURNACE_FIXTURE";
const SURVIVAL_FURNACE_SMELTING_BREADTH_FIXTURE_ENV: &str =
    "MC_COMPAT_SURVIVAL_FURNACE_SMELTING_BREADTH_FIXTURE";
const SURVIVAL_HUNGER_FOOD_PROBE_ENV: &str = "MC_COMPAT_SURVIVAL_HUNGER_FOOD_PROBE";
const SURVIVAL_HUNGER_FOOD_FIXTURE_ENV: &str = "MC_COMPAT_SURVIVAL_HUNGER_FOOD_FIXTURE";
const SURVIVAL_HUNGER_HEALTH_PROBE_ENV: &str = "MC_COMPAT_SURVIVAL_HUNGER_HEALTH_PROBE";
const SURVIVAL_HUNGER_HEALTH_FIXTURE_ENV: &str = "MC_COMPAT_SURVIVAL_HUNGER_HEALTH_FIXTURE";
const SURVIVAL_MOB_DROP_PROBE_ENV: &str = "MC_COMPAT_SURVIVAL_MOB_DROP_PROBE";
const SURVIVAL_MOB_DROP_FIXTURE_ENV: &str = "MC_COMPAT_SURVIVAL_MOB_DROP_FIXTURE";
const SURVIVAL_MOB_AI_LOOT_PROBE_ENV: &str = "MC_COMPAT_SURVIVAL_MOB_AI_LOOT_PROBE";
const SURVIVAL_MOB_AI_LOOT_FIXTURE_ENV: &str = "MC_COMPAT_SURVIVAL_MOB_AI_LOOT_FIXTURE";
const SURVIVAL_REDSTONE_TOGGLE_PROBE_ENV: &str = "MC_COMPAT_SURVIVAL_REDSTONE_TOGGLE_PROBE";
const SURVIVAL_REDSTONE_TOGGLE_FIXTURE_ENV: &str = "MC_COMPAT_SURVIVAL_REDSTONE_TOGGLE_FIXTURE";
const SURVIVAL_REDSTONE_CIRCUIT_PROBE_ENV: &str = "MC_COMPAT_SURVIVAL_REDSTONE_CIRCUIT_PROBE";
const SURVIVAL_REDSTONE_CIRCUIT_FIXTURE_ENV: &str = "MC_COMPAT_SURVIVAL_REDSTONE_CIRCUIT_FIXTURE";
const SURVIVAL_WORLD_MULTICHUNK_PROBE_ENV: &str = "MC_COMPAT_SURVIVAL_WORLD_MULTICHUNK_PROBE";
const SURVIVAL_WORLD_MULTICHUNK_SESSION_ENV: &str = "MC_COMPAT_SURVIVAL_WORLD_MULTICHUNK_SESSION";
const SURVIVAL_WORLD_MULTICHUNK_FIXTURE_ENV: &str = "MC_COMPAT_SURVIVAL_WORLD_MULTICHUNK_FIXTURE";
const SURVIVAL_WORLD_MULTICHUNK_DIR_ENV: &str = "MC_COMPAT_SURVIVAL_WORLD_MULTICHUNK_DIR";
const SURVIVAL_WORLD_MULTICHUNK_PHASE_ENV: &str = "MC_COMPAT_SURVIVAL_WORLD_MULTICHUNK_PHASE";
const SURVIVAL_CONTAINER_BLOCK_ENTITY_PROBE_ENV: &str =
    "MC_COMPAT_SURVIVAL_CONTAINER_BLOCK_ENTITY_PROBE";
const SURVIVAL_CONTAINER_BLOCK_ENTITY_FIXTURE_ENV: &str =
    "MC_COMPAT_SURVIVAL_CONTAINER_BLOCK_ENTITY_FIXTURE";
const SURVIVAL_SIGN_EDITING_PROBE_ENV: &str = "MC_COMPAT_SURVIVAL_SIGN_EDITING_PROBE";
const SURVIVAL_SIGN_EDITING_FIXTURE_ENV: &str = "MC_COMPAT_SURVIVAL_SIGN_EDITING_FIXTURE";
const SURVIVAL_WORLD_PERSISTENCE_PROBE_ENV: &str = "MC_COMPAT_SURVIVAL_WORLD_PERSISTENCE_PROBE";
const SURVIVAL_WORLD_PERSISTENCE_SESSION_ENV: &str = "MC_COMPAT_SURVIVAL_WORLD_PERSISTENCE_SESSION";
const SURVIVAL_WORLD_PERSISTENCE_FIXTURE_ENV: &str = "MC_COMPAT_SURVIVAL_WORLD_PERSISTENCE_FIXTURE";
const SURVIVAL_WORLD_PERSISTENCE_DIR_ENV: &str = "MC_COMPAT_SURVIVAL_WORLD_PERSISTENCE_DIR";
const SURVIVAL_WORLD_PERSISTENCE_PHASE_ENV: &str = "MC_COMPAT_SURVIVAL_WORLD_PERSISTENCE_PHASE";
const SURVIVAL_WORLD_PERSISTENCE_INITIAL_PHASE: &str = "initial";
const SURVIVAL_WORLD_PERSISTENCE_POST_RESTART_PHASE: &str = "post_restart";
const SURVIVAL_BLOCK_ENTITY_PROBE_ENV: &str = "MC_COMPAT_SURVIVAL_BLOCK_ENTITY_PROBE";
const SURVIVAL_BLOCK_ENTITY_SESSION_ENV: &str = "MC_COMPAT_SURVIVAL_BLOCK_ENTITY_SESSION";
const SURVIVAL_BLOCK_ENTITY_FIXTURE_ENV: &str = "MC_COMPAT_SURVIVAL_BLOCK_ENTITY_FIXTURE";
const SURVIVAL_BLOCK_ENTITY_DIR_ENV: &str = "MC_COMPAT_SURVIVAL_BLOCK_ENTITY_DIR";
const SURVIVAL_BLOCK_ENTITY_PHASE_ENV: &str = "MC_COMPAT_SURVIVAL_BLOCK_ENTITY_PHASE";
const SURVIVAL_BIOME_DIMENSION_PROBE_ENV: &str = "MC_COMPAT_SURVIVAL_BIOME_DIMENSION_PROBE";
const SURVIVAL_BIOME_DIMENSION_FIXTURE_ENV: &str = "MC_COMPAT_SURVIVAL_BIOME_DIMENSION_FIXTURE";
const SURVIVAL_BIOME_DIMENSION_TRAVEL_PROBE_ENV: &str =
    "MC_COMPAT_SURVIVAL_BIOME_DIMENSION_TRAVEL_PROBE";
const SURVIVAL_BIOME_DIMENSION_TRAVEL_FIXTURE_ENV: &str =
    "MC_COMPAT_SURVIVAL_BIOME_DIMENSION_TRAVEL_FIXTURE";
const MCP_CONTROL_ENDPOINT_STDIO: &str = "stdio";
const MCP_CONTROL_FAILURE_LIVE_EVIDENCE_MISSING: &str = "live-mcp-controlled-evidence-missing";
const MCP_CONTROL_FAILURE_REVISION_DIRTY: &str = "stevenarella-revision-dirty";
const MCP_CONTROL_FAILURE_REVISION_UNAVAILABLE: &str = "stevenarella-revision-unavailable";
const MCP_CONTROL_FAILURE_HANDSHAKE: &str = "mcp-handshake-failed";
const MCP_CONTROL_FAILURE_TOOLS_LIST: &str = "mcp-tools-list-failed";
const MCP_CONTROL_FAILURE_STATUS_TIMEOUT: &str = "mcp-status-connected-timeout";
const MCP_CONTROL_FAILURE_COMMAND: &str = "mcp-command-failed";
const MCP_CONTROL_FAILURE_FRAME_CAPTURE: &str = "mcp-frame-capture-failed";
const MCP_CONTROL_TOOL_LIST_DIGEST_SEPARATOR: &str = "\n";
const MCP_CONTROL_PREREQUISITES: &[&str] = &[
    "stevenarella_mcp_control_archived",
    "main_thread_command_queue",
    "stdout_clean_stdio",
];
const MCP_CONTROL_TOOL_NAMES: &[&str] = &[
    "stevenarella.enqueue_control",
    "stevenarella.capture_screenshot",
    "stevenarella.capture_latest_frame",
];
const MCP_CONTROL_REQUIRED_CALLS: &[&str] = &[
    "initialize",
    "tools/list",
    "tools/call status",
    "tools/call look",
    "tools/call key",
    "tools/call chat",
];
const MCP_CONTROL_LIVE_CALLS: &[&str] = &[
    "initialize",
    "tools/list",
    "tools/call status",
    "tools/call look",
    "tools/call key",
    "tools/call chat",
    "tools/call capture_latest_frame",
];
const MCP_CONTROL_REQUIRED_OUTCOME_IDS: &[&str] = &[
    "status.applied",
    "look.applied",
    "key.applied",
    "chat.applied",
];
#[cfg(test)]
const MCP_CONTROL_LIVE_OUTCOME_IDS: &[&str] = &[
    "status.applied",
    "look.applied",
    "key.applied",
    "chat.applied",
    "capture_latest_frame.captured",
];
const MCP_CONTROL_JSONRPC_VERSION_NEEDLE: &str = "\"jsonrpc\":\"2.0\"";
const MCP_CONTROL_RESULT_NEEDLE: &str = "\"result\"";
const MCP_CONTROL_TOOLS_ARRAY_NEEDLE: &str = "\"tools\"";
const MCP_CONTROL_CONNECTED_TOKEN: &str = "connected=true";
const MCP_CONTROL_OUTCOME_APPLIED_ESCAPED: &str = "\\\"outcome\\\":\\\"applied\\\"";
const MCP_CONTROL_LIVE_CAPTURE_RELATIVE_PATH: &str = "mcp-controlled-smoke/latest-frame.png";
const MCP_CONTROL_LIVE_CAPTURE_DIR_SUFFIX: &str = "frames";
const MCP_CONTROL_LIVE_STDERR_LOG_EXTENSION: &str = "stderr.log";
const MCP_CONTROL_LIVE_TRANSCRIPT_EXTENSION: &str = "mcp-transcript.log";
const MCP_CONTROL_MAX_STATUS_POLLS: usize = 40;
const MCP_CONTROL_STATUS_POLL_MILLIS: u64 = 250;
const MCP_CONTROL_TERMINATE_GRACE_MILLIS: u64 = 250;
const MCP_CONTROL_PROCESS_GROUP_COMMAND: &str = "setsid";
const MCP_CONTROL_TERMINATE_COMMAND: &str = "kill";
const MCP_CONTROL_TERMINATE_SIGNAL: &str = "-TERM";
const MCP_CONTROL_KILL_SIGNAL: &str = "-KILL";
const MCP_CONTROL_INITIALIZE_ID: &str = "mcp-initialize";
const MCP_CONTROL_TOOLS_LIST_ID: &str = "mcp-tools-list";
const MCP_CONTROL_STATUS_ID_PREFIX: &str = "mcp-status";
const MCP_CONTROL_LOOK_ID: &str = "mcp-look";
const MCP_CONTROL_KEY_ID: &str = "mcp-key";
const MCP_CONTROL_CHAT_ID: &str = "mcp-chat";
const MCP_CONTROL_CAPTURE_ID: &str = "mcp-capture-latest-frame";
const MCP_CONTROL_NON_CLAIMS: &[&str] = &[
    "screenshots_alone",
    "visual_regression_approval",
    "semantic_equivalence",
    "full_minecraft_compatibility",
    "production_readiness",
    "public_server_safety",
    "load_testing",
];
const FRAME_ARTIFACT_NON_CLAIMS: &[&str] = &[
    "frame_capture_not_selected",
    "visual_regression_approval",
    "semantic_equivalence",
];
const CTF_OPPONENT_RETURN_DROP_SERVER_REJECTION_NEEDLE: &str =
    "invalid_opponent_base_return_drop_rejected";
const DEFAULT_SUCCESS_PATTERN: &[&str] = &[
    "Detected server protocol version",
    "Dimension type:",
    "Received chat message",
];
const TRIAGE_MAX_TIMELINE_EVENTS: usize = 6;
const TRIAGE_MAX_EXCERPT_CHARS: usize = 160;
const TRIAGE_CONFIDENCE_HIGH: &str = "high";
const TRIAGE_CONFIDENCE_MEDIUM: &str = "medium";
const TRIAGE_CONFIDENCE_NONE: &str = "none";
const TRIAGE_REDACTED: &str = "[redacted]";
const TYPED_EVENT_PREFIX: &str = "MC-COMPAT-EVENT";
const TYPED_EVENT_SCHEMA_VERSION: u32 = 1;
const TYPED_EVENT_MIGRATION_FALLBACK: &str = "substring-fallback";
const TYPED_EVENT_MIGRATION_DERIVED_FROM_MILESTONES: &str = "derived-from-milestones";
const TYPED_EVENT_LOG_EXTENSION: &str = "typed-events.log";
const TYPED_EVENT_DEFAULT_SESSION_ID: &str = "mc_compat_session";
const TYPED_EVENT_MAX_FIELD_CHARS: usize = 128;
const TYPED_EVENT_SEQUENCE_INDEX_OFFSET: usize = 1;
const TYPED_EVENT_SINGLE_USERNAME_COUNT: usize = 1;
const NEGATIVE_LIVE_RAIL_MAX_CLIENTS: usize = 2;
const NEGATIVE_LIVE_RAIL_MIN_TIMEOUT_SECS: u64 = 1;
const NEGATIVE_LIVE_RAIL_EXPECTED_OUTCOME: &str = "containment_or_disconnect_without_promotion";
const NEGATIVE_LIVE_RAIL_OBSERVED_OUTCOME_CONTAINMENT: &str = "containment_observed";
const NEGATIVE_LIVE_RAIL_OUTCOME_SOURCE_PREFIX: &str = "client_milestone:";
const NEGATIVE_LIVE_RAIL_NON_CLAIMS: &[&str] = &[
    "broad_invalid_input_coverage",
    "adversarial_security",
    "public_server_safety",
    "production_readiness",
    "full_inventory_transaction_semantics",
    "broad_plugin_message_semantics",
    "full_ctf_correctness",
];
const NEGATIVE_LIVE_RAIL_EVIDENCE_FIELDS: &[&str] = &[
    "invalid_action",
    "expected_outcome",
    "target_scope",
    "planned_clients",
    "timeout_secs",
    "client_milestone",
    "server_forbidden_matches",
    "postcondition",
];
const VANILLA_COMBAT_REFERENCE_PROBE_ENV: &str = "MC_COMPAT_VANILLA_COMBAT_REFERENCE_PROBE";
const VANILLA_COMBAT_ARMOR_REFERENCE_PROBE_ENV: &str =
    "MC_COMPAT_VANILLA_COMBAT_ARMOR_REFERENCE_PROBE";
const ARMOR_MATRIX_ROW_ID: &str = "chest_diamond_none_none_melee";
const ARMOR_MATRIX_LOADOUT_ID: &str = "armor_loadout_chest_only";
const ARMOR_MATRIX_EQUIPMENT_SLOT: &str = "chest=DiamondChestplate";
const ARMOR_MATRIX_ENCHANTMENT_NONE: &str = "enchantment_none";
const ARMOR_MATRIX_STATUS_EFFECT_NONE: &str = "status_effect_none";
const ARMOR_MATRIX_ATTACK_TYPE_MELEE: &str = "melee";
const ARMOR_MATRIX_REFERENCE_RECEIPT_NONE: &str = "none";
const ARMOR_MATRIX_NON_CLAIMS: &[&str] = &[
    "all_armor_permutations",
    "all_enchantments",
    "all_status_effects",
    "exact_vanilla_balancing",
    "production_readiness",
    "full_combat_correctness",
];
const EQUIPMENT_MATRIX_ROW_ID: &str = "remote_main_hand_slot4_item829_count1_non_empty";
const EQUIPMENT_MATRIX_ACTOR: &str = "compatbotb";
const EQUIPMENT_MATRIX_OBSERVER: &str = "compatbota";
const EQUIPMENT_MATRIX_REMOTE_ENTITY_ID: &str = "4";
const EQUIPMENT_MATRIX_SEMANTIC_SLOT: &str = "main_hand_remote_entity";
const EQUIPMENT_MATRIX_WIRE_SLOT: &str = "slot4";
const EQUIPMENT_MATRIX_ITEM_ID: &str = "829";
const EQUIPMENT_MATRIX_ITEM_COUNT: &str = "1";
const EQUIPMENT_MATRIX_TRANSITION: &str = "non_empty_update";
const EQUIPMENT_MATRIX_UPDATE_ORDER: &str = "after_remote_spawn";
const EQUIPMENT_MATRIX_REFERENCE_RECEIPT_NONE: &str = "none";
const EQUIPMENT_MATRIX_NON_CLAIMS: &[&str] = &[
    "all_equipment_slots",
    "all_item_types",
    "all_transition_orders",
    "equipment_packet_permutations",
    "armor_mitigation",
    "enchantment_status_effects",
    "production_readiness",
    "full_equipment_semantics",
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Mode {
    DryRun,
    Run,
    RunMatrix,
    BuildClient,
    StatusOnly,
    HarnessStatus,
    Cleanup,
    Stop,
    CompareReceipts,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ServerBackend {
    Valence,
    Paper,
}

const SCENARIO_ROUTER_COMMAND: &str = "scenario";
const SCENARIO_ROUTER_RUN_SUBCOMMAND: &str = "run";
const SCENARIO_ROUTER_BACKEND_FLAG: &str = "--backend";
const SCENARIO_ROUTER_SERVER_BACKEND_FLAG: &str = "--server-backend";
const SCENARIO_ROUTER_RECEIPT_FLAG: &str = "--receipt";
const SCENARIO_ROUTER_FAILURE_BUNDLE_FLAG: &str = "--failure-bundle";
const SCENARIO_ROUTER_TIMEOUT_FLAG: &str = "--timeout";
const SCENARIO_ROUTER_PACKET_CAPTURE_FLAG: &str = "--packet-capture-summary";
const SCENARIO_ROUTER_PROXY_ROUTE_FLAG: &str = "--proxy-route";
const SCENARIO_ROUTER_PROXY_FORWARDING_MODE_FLAG: &str = "--proxy-forwarding-mode";
const SCENARIO_ROUTER_DRY_RUN_FLAG: &str = "--dry-run";
const SCENARIO_ROUTER_RUN_FLAG: &str = "--run";
const SCENARIO_ROUTER_LIVE_FLAG: &str = "--live";
const SCENARIO_ROUTER_RECEIPT_EQUALS_PREFIX: &str = "--receipt=";
const SCENARIO_ROUTER_FAILURE_BUNDLE_EQUALS_PREFIX: &str = "--failure-bundle=";
const SCENARIO_ROUTER_TIMEOUT_EQUALS_PREFIX: &str = "--timeout=";
const SCENARIO_ROUTER_BACKEND_EQUALS_PREFIX: &str = "--backend=";
const SCENARIO_ROUTER_SERVER_BACKEND_EQUALS_PREFIX: &str = "--server-backend=";
const SCENARIO_ROUTER_PROXY_ROUTE_EQUALS_PREFIX: &str = "--proxy-route=";
const SCENARIO_ROUTER_PROXY_FORWARDING_MODE_EQUALS_PREFIX: &str = "--proxy-forwarding-mode=";
const SCENARIO_ROUTER_LEGACY_SCENARIO_FLAG: &str = "--scenario";
const SCENARIO_ROUTER_LEGACY_SCENARIO_EQUALS_PREFIX: &str = "--scenario=";
const SCENARIO_ROUTER_SUBCOMMAND_INDEX: usize = 1;
const SCENARIO_ROUTER_SCENARIO_INDEX: usize = 2;
const SCENARIO_ROUTER_OPTION_START_INDEX: usize = 3;
const SCENARIO_ROUTER_MISSING_VALUE: &str = "missing value";
const SCENARIO_ROUTER_NON_CLAIM_BROAD_COMPATIBILITY: &str = "broad_minecraft_compatibility_false";
const SCENARIO_ROUTER_NON_CLAIM_PRODUCTION_READINESS: &str = "production_readiness_false";
const SCENARIO_ROUTER_NON_CLAIM_SEMANTIC_EQUIVALENCE: &str = "semantic_equivalence_false";
const SCENARIO_ROUTER_NON_CLAIMS: &[&str] = &[
    SCENARIO_ROUTER_NON_CLAIM_BROAD_COMPATIBILITY,
    SCENARIO_ROUTER_NON_CLAIM_PRODUCTION_READINESS,
    SCENARIO_ROUTER_NON_CLAIM_SEMANTIC_EQUIVALENCE,
];
const SCENARIO_ROUTER_BLOCKED_COMMAND_FLAGS: &[&str] = &[
    "--run-matrix",
    "--build-client",
    "--status-only",
    "--status",
    "--cleanup",
    "--stop",
    "--compare-receipts",
];
const SCENARIO_ROUTER_BLOCKED_OVERCLAIM_FLAGS: &[&str] = &[
    "--claim-full-compatibility",
    "--claim-production-readiness",
    "--claim-semantic-equivalence",
];

#[derive(Debug, Clone, PartialEq, Eq)]
struct ScenarioRouteRequest {
    scenario: Scenario,
    backend: ServerBackend,
    mode: Mode,
    receipt_path: Option<PathBuf>,
    timeout_secs: Option<u64>,
    packet_capture_summary: bool,
    proxy_route: Option<String>,
    proxy_forwarding_mode: Option<String>,
    failure_bundle_path: Option<PathBuf>,
    passthrough_args: Vec<String>,
}

const VALENCE_DEFAULT_SERVER_PORT: u16 = 25565;
const PAPER_DEFAULT_SERVER_PORT: u16 = 25566;

struct ValenceRuntime;
struct PaperRuntime;

static VALENCE_RUNTIME: ValenceRuntime = ValenceRuntime;
static PAPER_RUNTIME: PaperRuntime = PaperRuntime;

trait ServerRuntime {
    fn name(&self) -> &'static str;
    fn default_port(&self) -> u16;
    fn start(&self, cfg: &Config) -> Result<ManagedServer, String>;
    fn stop(&self, cfg: &Config) -> Result<(), String>;
    fn force_stop(&self, cfg: &Config) -> Result<(), String>;
    fn log_label(&self, cfg: &Config) -> String;
    fn read_log(&self, cfg: &Config) -> Result<String, String>;
}

impl ServerBackend {
    fn runtime(self) -> &'static dyn ServerRuntime {
        match self {
            ServerBackend::Valence => &VALENCE_RUNTIME,
            ServerBackend::Paper => &PAPER_RUNTIME,
        }
    }
}

impl ServerRuntime for ValenceRuntime {
    fn name(&self) -> &'static str {
        "valence"
    }

    fn default_port(&self) -> u16 {
        VALENCE_DEFAULT_SERVER_PORT
    }

    fn start(&self, cfg: &Config) -> Result<ManagedServer, String> {
        start_valence_server(cfg)
    }

    fn stop(&self, cfg: &Config) -> Result<(), String> {
        stop_valence_server(cfg)
    }

    fn force_stop(&self, cfg: &Config) -> Result<(), String> {
        force_stop_valence_server(cfg)
    }

    fn log_label(&self, cfg: &Config) -> String {
        cfg.valence_log.display().to_string()
    }

    fn read_log(&self, cfg: &Config) -> Result<String, String> {
        read_valence_log(cfg)
    }
}

impl ServerRuntime for PaperRuntime {
    fn name(&self) -> &'static str {
        "paper"
    }

    fn default_port(&self) -> u16 {
        PAPER_DEFAULT_SERVER_PORT
    }

    fn start(&self, cfg: &Config) -> Result<ManagedServer, String> {
        start_paper_server(cfg)?;
        Ok(ManagedServer {
            child: None,
            pid_file: cfg.valence_pid_file.clone(),
            paper_container: Some(cfg.server_name.clone()),
            keep: cfg.keep_server || cfg.mode == Mode::DryRun,
        })
    }

    fn stop(&self, cfg: &Config) -> Result<(), String> {
        stop_paper_server(cfg)
    }

    fn force_stop(&self, cfg: &Config) -> Result<(), String> {
        force_stop_paper_server(cfg)
    }

    fn log_label(&self, cfg: &Config) -> String {
        format!("docker logs {}", cfg.server_name)
    }

    fn read_log(&self, cfg: &Config) -> Result<String, String> {
        read_paper_log(cfg)
    }
}

#[derive(Debug, Clone)]
struct Config {
    root: PathBuf,
    client_dir: PathBuf,
    valence_repo: PathBuf,
    valence_rev: String,
    valence_worktree: PathBuf,
    valence_example: String,
    valence_log: PathBuf,
    valence_target_dir: PathBuf,
    valence_pid_file: PathBuf,
    server_backend: ServerBackend,
    target_dir: PathBuf,
    server_name: String,
    server_version: String,
    server_protocol: u32,
    server_port: u16,
    client_username: String,
    docker_image: String,
    paper_plugin_jar: Option<PathBuf>,
    mode: Mode,
    keep_server: bool,
    client_timeout: Duration,
    client_success_needles: Vec<String>,
    scenario: Scenario,
    expected_status_description: Option<String>,
    expected_status_version_name: Option<String>,
    expected_status_sample: Vec<String>,
    packet_capture_summary: bool,
    proxy_route: Option<String>,
    proxy_forwarding_mode: Option<String>,
    receipt_path: Option<PathBuf>,
    receipt_dir: Option<PathBuf>,
    failure_bundle_path: Option<PathBuf>,
    compare_receipts: Option<(PathBuf, PathBuf)>,
    config_path: Option<PathBuf>,
    steel_config_path: Option<PathBuf>,
    matrix_dry_run: bool,
    cleanup_apply: bool,
    negative_public_target: bool,
    negative_external_authorized: bool,
    arrow_damage_policy: runtime_config::ArrowDamagePolicy,
    scenario_route: Option<ScenarioRouteRequest>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ClientRunEvidence {
    log_path: Option<PathBuf>,
    log_paths: Vec<PathBuf>,
    usernames: Vec<String>,
    exit_code: Option<i32>,
    classification: &'static str,
    matched_success_pattern: Option<String>,
    scenario: Option<ScenarioEvidence>,
    server_scenario: Option<ServerScenarioEvidence>,
    projectile_damage_causality: Option<ProjectileDamageCausalityEvidence>,
    projectile_travel_collision: Option<ProjectileTravelCollisionEvidence>,
    mcp_control: Option<McpControlRunEvidence>,
    frame_artifacts: Option<FrameArtifactsReceiptEvidence>,
}

struct ManagedServer {
    child: Option<Child>,
    pid_file: PathBuf,
    paper_container: Option<String>,
    keep: bool,
}

impl Drop for ManagedServer {
    fn drop(&mut self) {
        if self.keep {
            return;
        }
        if let Some(mut child) = self.child.take() {
            eprintln!(
                "[mc-compat] stopping managed Valence server process {}",
                child.id()
            );
            let _ = child.kill();
            let _ = child.wait();
            let _ = fs::remove_file(&self.pid_file);
        }
        if let Some(container) = self.paper_container.take() {
            eprintln!("[mc-compat] stopping managed Paper container {container}");
            let _ = Command::new("docker")
                .arg("rm")
                .arg("-f")
                .arg(container)
                .status();
        }
    }
}

fn main() -> ExitCode {
    match real_main() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("[mc-compat] error: {err}");
            ExitCode::from(1)
        }
    }
}

fn real_main() -> Result<(), String> {
    let cfg = Config::from_env_and_args()?;
    let result = execute(&cfg);
    let mut follow_up_errors = Vec::new();
    if cfg.receipt_path.is_some() {
        if let Err(receipt_err) = write_smoke_receipt(&cfg, result.as_ref()) {
            follow_up_errors.push(format!("failed to write receipt: {receipt_err}"));
        }
    }
    if result.is_err() && cfg.failure_bundle_path.is_some() {
        if let Err(bundle_err) = write_failure_evidence_bundle(&cfg, result.as_ref()) {
            follow_up_errors.push(format!("failed to write failure bundle: {bundle_err}"));
        }
    }
    combine_runner_result(result, follow_up_errors)
}

fn combine_runner_result(
    result: Result<Option<ClientRunEvidence>, String>,
    follow_up_errors: Vec<String>,
) -> Result<(), String> {
    let follow_up = follow_up_errors.join("; ");
    match (result, follow_up.is_empty()) {
        (Ok(_), true) => Ok(()),
        (Ok(_), false) => Err(follow_up),
        (Err(err), true) => Err(err),
        (Err(err), false) => Err(format!("{err}; additionally: {follow_up}")),
    }
}

fn execute(cfg: &Config) -> Result<Option<ClientRunEvidence>, String> {
    validate_static_scenario_specs(SCENARIO_SPECS)?;
    let plan = harness_plan_from_config(cfg).map_err(format_plan_diagnostics)?;
    validate_projectile_damage_dependency(cfg)?;
    validate_mcp_controlled_live_preflight(cfg)?;
    validate_load_network_safety_preflight(cfg)?;
    validate_negative_live_rail_preflight(cfg)?;
    if matches!(cfg.mode, Mode::DryRun | Mode::Run | Mode::BuildClient) {
        ensure_client_dir_ready(cfg)?;
    }
    if cfg.server_backend == ServerBackend::Valence && matches!(cfg.mode, Mode::DryRun | Mode::Run)
    {
        ensure_valence_repo_ready(cfg)?;
    }
    match cfg.mode {
        Mode::DryRun => {
            log_harness_plan(&plan);
            build_client(cfg)?;
            if cfg.server_backend == ServerBackend::Paper {
                log(format_args!(
                    "server start will set EULA=TRUE using recorded user acceptance"
                ));
            }
            let _server = start_server(cfg)?;
            probe_status(cfg)?;
            let client = run_client(cfg)?;
            Ok(Some(client))
        }
        Mode::BuildClient => {
            build_client(cfg)?;
            Ok(None)
        }
        Mode::StatusOnly => {
            probe_status(cfg)?;
            Ok(None)
        }
        Mode::HarnessStatus => {
            print_harness_status(cfg)?;
            Ok(None)
        }
        Mode::Cleanup => {
            cleanup_harness_state(cfg, &plan.cleanup)?;
            Ok(None)
        }
        Mode::Stop => {
            stop_server(cfg)?;
            Ok(None)
        }
        Mode::CompareReceipts => {
            compare_receipts(cfg)?;
            Ok(None)
        }
        Mode::RunMatrix => {
            let matrix = plan
                .matrix
                .as_ref()
                .ok_or_else(|| "run-matrix mode missing matrix plan".to_string())?;
            run_matrix(cfg, matrix)?;
            Ok(None)
        }
        Mode::Run => {
            build_client(cfg)?;
            prepare_world_persistence_state_dir(cfg)?;
            let _server = start_server(cfg)?;
            probe_status(cfg)?;
            let client = run_client(cfg)?;
            Ok(Some(client))
        }
    }
}

fn prepare_world_persistence_state_dir(cfg: &Config) -> Result<(), String> {
    if !uses_isolated_restart_storage(cfg.scenario) || cfg.mode != Mode::Run {
        return Ok(());
    }
    let dir = world_persistence_state_dir(cfg, cfg.server_backend);
    if dir.exists() {
        fs::remove_dir_all(&dir).map_err(|err| format!("remove {}: {err}", dir.display()))?;
    }
    fs::create_dir_all(&dir).map_err(|err| format!("create {}: {err}", dir.display()))?;
    let phase_path = world_persistence_restart_phase_path(cfg);
    if phase_path.exists() {
        fs::remove_file(&phase_path)
            .map_err(|err| format!("remove {}: {err}", phase_path.display()))?;
    }
    let pre_restart_log = world_persistence_pre_restart_server_log_path(cfg);
    if pre_restart_log.exists() {
        fs::remove_file(&pre_restart_log)
            .map_err(|err| format!("remove {}: {err}", pre_restart_log.display()))?;
    }
    Ok(())
}

fn validate_mcp_controlled_live_preflight(cfg: &Config) -> Result<(), String> {
    if !scenario_behavior(cfg.scenario).is_mcp_controlled_smoke() || cfg.mode != Mode::Run {
        return Ok(());
    }
    if cfg.client_timeout.as_secs() > SAFETY_MAX_DURATION_SECS {
        return Err(format!(
            "{MCP_CONTROLLED_SMOKE_SCENARIO} client timeout exceeds bounded live rail max {SAFETY_MAX_DURATION_SECS}s"
        ));
    }
    Ok(())
}

fn validate_projectile_damage_dependency(cfg: &Config) -> Result<(), String> {
    if cfg.server_backend != ServerBackend::Valence
        || !scenario_behavior(cfg.scenario).uses_dynamic_projectile_health()
        || !matches!(cfg.mode, Mode::DryRun | Mode::Run)
    {
        return Ok(());
    }
    if cfg.valence_rev == PINNED_PROJECTILE_DAMAGE_VALENCE_REV {
        return Ok(());
    }
    Err(format!(
        "projectile-damage-attribution requires pinned Valence revision {PINNED_PROJECTILE_DAMAGE_VALENCE_REV}; got {}. Do not use VALENCE_REV=HEAD for promoted evidence.",
        cfg.valence_rev
    ))
}

fn validate_load_network_safety_preflight(cfg: &Config) -> Result<(), String> {
    if !matches!(cfg.mode, Mode::DryRun | Mode::Run | Mode::RunMatrix) {
        return Ok(());
    }
    let evidence = evaluate_load_network_safety(load_network_safety_inputs(cfg, false, false));
    if evidence.preflight_passed {
        return Ok(());
    }
    Err(format!(
        "load/network safety preflight failed: missing={:?} bound_violations={:?}",
        evidence.missing_fields, evidence.bound_violations
    ))
}

fn load_network_safety_inputs(
    cfg: &Config,
    telemetry_present: bool,
    live_receipt: bool,
) -> LoadNetworkSafetyInputs {
    let explicit_authorization = env::var("MC_COMPAT_EXTERNAL_LOAD_AUTHORIZED")
        .map(|value| value == "1")
        .unwrap_or(false);
    let public_target = env::var("MC_COMPAT_PUBLIC_TARGET")
        .map(|value| value == "1")
        .unwrap_or(false);
    LoadNetworkSafetyInputs {
        target_scope: SAFETY_OWNED_LOCAL_SCOPE,
        owned_local_target: !public_target,
        explicit_authorization,
        public_target,
        planned_clients: planned_client_usernames(cfg).len(),
        max_clients: SAFETY_MAX_LOCAL_CLIENTS,
        duration_secs: cfg.client_timeout.as_secs(),
        max_duration_secs: SAFETY_MAX_DURATION_SECS,
        reconnect_sessions: safety_reconnect_sessions(cfg.scenario),
        latency_ms: env::var("MC_COMPAT_LATENCY_MS")
            .unwrap_or_else(|_| SAFETY_ZERO_VALUE.to_string()),
        jitter_ms: env::var("MC_COMPAT_JITTER_MS")
            .unwrap_or_else(|_| SAFETY_ZERO_VALUE.to_string()),
        loss_percent: env::var("MC_COMPAT_LOSS_PERCENT")
            .unwrap_or_else(|_| SAFETY_ZERO_VALUE.to_string()),
        telemetry_present,
        live_receipt,
    }
}

fn safety_reconnect_sessions(scenario: Scenario) -> usize {
    scenario_behavior(scenario).safety_reconnect_sessions()
}

fn is_negative_live_rail(scenario: Scenario) -> bool {
    scenario_behavior(scenario).negative_live_rail().is_some()
}

fn negative_live_rail_invalid_action(scenario: Scenario) -> Option<&'static str> {
    scenario_behavior(scenario)
        .negative_live_rail()
        .map(|behavior| behavior.invalid_action)
}

fn negative_live_rail_postcondition_milestone(scenario: Scenario) -> Option<&'static str> {
    scenario_behavior(scenario)
        .negative_live_rail()
        .map(|behavior| behavior.postcondition)
}

fn observed_negative_live_rail_outcome(
    scenario: Scenario,
    scenario_evidence: &ScenarioEvidence,
) -> (Option<&'static str>, Option<String>, bool) {
    let Some(postcondition) = negative_live_rail_postcondition_milestone(scenario) else {
        return (None, None, false);
    };
    let observed = scenario_evidence
        .observed_milestones
        .contains(&postcondition);
    if !observed {
        return (None, None, false);
    }
    (
        Some(NEGATIVE_LIVE_RAIL_OBSERVED_OUTCOME_CONTAINMENT),
        Some(format!(
            "{NEGATIVE_LIVE_RAIL_OUTCOME_SOURCE_PREFIX}{postcondition}"
        )),
        true,
    )
}

fn negative_live_rail_inputs_from_config(
    cfg: &Config,
    scenario_evidence: Option<&ScenarioEvidence>,
    telemetry_required: bool,
) -> NegativeLiveRailInputs {
    let selected = is_negative_live_rail(cfg.scenario);
    let (observed_outcome, observed_outcome_source, telemetry_present) = scenario_evidence
        .map(|scenario| observed_negative_live_rail_outcome(cfg.scenario, scenario))
        .unwrap_or((None, None, false));
    NegativeLiveRailInputs {
        selected,
        rail: selected.then(|| scenario_name(cfg.scenario)),
        invalid_action: negative_live_rail_invalid_action(cfg.scenario),
        expected_outcome: selected.then_some(NEGATIVE_LIVE_RAIL_EXPECTED_OUTCOME),
        observed_outcome,
        observed_outcome_source,
        postcondition_milestone: negative_live_rail_postcondition_milestone(cfg.scenario),
        telemetry_required,
        telemetry_present,
        target_scope: SAFETY_OWNED_LOCAL_SCOPE,
        explicit_authorization: cfg.negative_external_authorized,
        public_target: cfg.negative_public_target,
        planned_clients: planned_client_usernames(cfg).len(),
        max_clients: NEGATIVE_LIVE_RAIL_MAX_CLIENTS,
        timeout_secs: cfg.client_timeout.as_secs(),
    }
}

fn evaluate_negative_live_rail_safety(cfg: &Config) -> NegativeLiveRailEvidence {
    evaluate_negative_live_rail_safety_from_inputs(negative_live_rail_inputs_from_config(
        cfg, None, false,
    ))
}

fn validate_negative_live_rail_preflight(cfg: &Config) -> Result<(), String> {
    let evidence = evaluate_negative_live_rail_safety(cfg);
    if evidence.preflight_passed {
        return Ok(());
    }
    Err(format!(
        "negative live rail preflight failed: missing={:?} bound_violations={:?}",
        evidence.missing_fields, evidence.bound_violations
    ))
}

fn parse_scenario_route_request(args: &[String]) -> Result<Option<ScenarioRouteRequest>, String> {
    let Some(command) = args.first() else {
        return Ok(None);
    };
    if command != SCENARIO_ROUTER_COMMAND {
        return Ok(None);
    }
    let subcommand = args
        .get(SCENARIO_ROUTER_SUBCOMMAND_INDEX)
        .ok_or_else(|| scenario_router_usage_error("missing subcommand"))?;
    if subcommand != SCENARIO_ROUTER_RUN_SUBCOMMAND {
        return Err(scenario_router_usage_error(&format!(
            "unknown subcommand: {subcommand}"
        )));
    }
    let scenario_value = args
        .get(SCENARIO_ROUTER_SCENARIO_INDEX)
        .ok_or_else(|| scenario_router_usage_error("missing scenario"))?;
    let scenario = parse_scenario(scenario_value)?;
    parse_scenario_route_options(scenario, &args[SCENARIO_ROUTER_OPTION_START_INDEX..])
}

fn parse_scenario_route_options(
    scenario: Scenario,
    args: &[String],
) -> Result<Option<ScenarioRouteRequest>, String> {
    let mut request = ScenarioRouteRequest {
        scenario,
        backend: ServerBackend::Valence,
        mode: Mode::DryRun,
        receipt_path: None,
        timeout_secs: None,
        packet_capture_summary: false,
        proxy_route: None,
        proxy_forwarding_mode: None,
        failure_bundle_path: None,
        passthrough_args: Vec::new(),
    };
    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        if let Some(overclaim) = blocked_scenario_route_overclaim_flag(arg) {
            return Err(format!(
                "scenario router rejects overclaiming option {overclaim}; broad compatibility, production readiness, and semantic equivalence claims remain false"
            ));
        }
        if SCENARIO_ROUTER_BLOCKED_COMMAND_FLAGS.contains(&arg.as_str()) {
            return Err(format!(
                "scenario router blocks non-scenario command option {arg}; use mc-compat-runner {SCENARIO_ROUTER_COMMAND} {SCENARIO_ROUTER_RUN_SUBCOMMAND} <scenario>"
            ));
        }
        match arg.as_str() {
            SCENARIO_ROUTER_DRY_RUN_FLAG => request.mode = Mode::DryRun,
            SCENARIO_ROUTER_RUN_FLAG | SCENARIO_ROUTER_LIVE_FLAG => request.mode = Mode::Run,
            SCENARIO_ROUTER_BACKEND_FLAG | SCENARIO_ROUTER_SERVER_BACKEND_FLAG => {
                let value = scenario_route_option_value(arg, iter.next())?;
                request.backend = parse_backend(value)?;
            }
            SCENARIO_ROUTER_RECEIPT_FLAG => {
                let value = scenario_route_option_value(arg, iter.next())?;
                request.receipt_path = Some(parse_scenario_route_receipt_path(value)?);
            }
            SCENARIO_ROUTER_FAILURE_BUNDLE_FLAG => {
                let value = scenario_route_option_value(arg, iter.next())?;
                request.failure_bundle_path =
                    Some(parse_scenario_route_failure_bundle_path(value)?);
            }
            SCENARIO_ROUTER_TIMEOUT_FLAG => {
                let value = scenario_route_option_value(arg, iter.next())?;
                request.timeout_secs = Some(parse_client_timeout_secs(
                    value,
                    SCENARIO_ROUTER_TIMEOUT_FLAG,
                )?);
            }
            SCENARIO_ROUTER_PACKET_CAPTURE_FLAG => request.packet_capture_summary = true,
            SCENARIO_ROUTER_PROXY_ROUTE_FLAG => {
                let value = scenario_route_option_value(arg, iter.next())?;
                request.proxy_route = Some(value.to_string());
            }
            SCENARIO_ROUTER_PROXY_FORWARDING_MODE_FLAG => {
                let value = scenario_route_option_value(arg, iter.next())?;
                request.proxy_forwarding_mode = Some(value.to_string());
            }
            SCENARIO_ROUTER_LEGACY_SCENARIO_FLAG => {
                return Err("scenario router takes the scenario as a positional argument; do not also pass --scenario".to_string());
            }
            _ if arg.starts_with(SCENARIO_ROUTER_LEGACY_SCENARIO_EQUALS_PREFIX) => {
                return Err("scenario router takes the scenario as a positional argument; do not also pass --scenario".to_string());
            }
            _ if arg.starts_with(SCENARIO_ROUTER_BACKEND_EQUALS_PREFIX) => {
                let value = &arg[SCENARIO_ROUTER_BACKEND_EQUALS_PREFIX.len()..];
                request.backend = parse_backend(value)?;
            }
            _ if arg.starts_with(SCENARIO_ROUTER_SERVER_BACKEND_EQUALS_PREFIX) => {
                let value = &arg[SCENARIO_ROUTER_SERVER_BACKEND_EQUALS_PREFIX.len()..];
                request.backend = parse_backend(value)?;
            }
            _ if arg.starts_with(SCENARIO_ROUTER_RECEIPT_EQUALS_PREFIX) => {
                let value = &arg[SCENARIO_ROUTER_RECEIPT_EQUALS_PREFIX.len()..];
                request.receipt_path = Some(parse_scenario_route_receipt_path(value)?);
            }
            _ if arg.starts_with(SCENARIO_ROUTER_FAILURE_BUNDLE_EQUALS_PREFIX) => {
                let value = &arg[SCENARIO_ROUTER_FAILURE_BUNDLE_EQUALS_PREFIX.len()..];
                request.failure_bundle_path =
                    Some(parse_scenario_route_failure_bundle_path(value)?);
            }
            _ if arg.starts_with(SCENARIO_ROUTER_TIMEOUT_EQUALS_PREFIX) => {
                let value = &arg[SCENARIO_ROUTER_TIMEOUT_EQUALS_PREFIX.len()..];
                request.timeout_secs = Some(parse_client_timeout_secs(
                    value,
                    SCENARIO_ROUTER_TIMEOUT_FLAG,
                )?);
            }
            _ if arg.starts_with(SCENARIO_ROUTER_PROXY_ROUTE_EQUALS_PREFIX) => {
                request.proxy_route =
                    Some(arg[SCENARIO_ROUTER_PROXY_ROUTE_EQUALS_PREFIX.len()..].to_string());
            }
            _ if arg.starts_with(SCENARIO_ROUTER_PROXY_FORWARDING_MODE_EQUALS_PREFIX) => {
                request.proxy_forwarding_mode = Some(
                    arg[SCENARIO_ROUTER_PROXY_FORWARDING_MODE_EQUALS_PREFIX.len()..].to_string(),
                );
            }
            _ => request.passthrough_args.push(arg.clone()),
        }
    }
    Ok(Some(request))
}

fn scenario_route_option_value<'a>(
    flag: &str,
    value: Option<&'a String>,
) -> Result<&'a str, String> {
    value
        .map(String::as_str)
        .ok_or_else(|| format!("{flag} requires a value; got {SCENARIO_ROUTER_MISSING_VALUE}"))
}

fn parse_scenario_route_receipt_path(value: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(value);
    validate_scenario_route_output_path(&path, "receipt path")?;
    Ok(path)
}

fn parse_scenario_route_failure_bundle_path(value: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(value);
    validate_scenario_route_output_path(&path, "failure bundle path")?;
    Ok(path)
}

fn validate_scenario_route_output_path(path: &Path, label: &str) -> Result<(), String> {
    if path.as_os_str().is_empty() {
        return Err(format!("scenario router {label} is empty"));
    }
    if path
        .components()
        .any(|component| matches!(component, std::path::Component::ParentDir))
    {
        return Err(format!(
            "scenario router {label} contains parent traversal: {}",
            path.display()
        ));
    }
    Ok(())
}

fn blocked_scenario_route_overclaim_flag(arg: &str) -> Option<&'static str> {
    SCENARIO_ROUTER_BLOCKED_OVERCLAIM_FLAGS
        .iter()
        .copied()
        .find(|flag| {
            arg == *flag
                || arg
                    .strip_prefix(*flag)
                    .is_some_and(|rest| rest.starts_with('='))
        })
}

fn scenario_router_usage_error(reason: &str) -> String {
    format!(
        "scenario router usage error: {reason}; expected mc-compat-runner {SCENARIO_ROUTER_COMMAND} {SCENARIO_ROUTER_RUN_SUBCOMMAND} <scenario> [{SCENARIO_ROUTER_DRY_RUN_FLAG}|{SCENARIO_ROUTER_RUN_FLAG}|{SCENARIO_ROUTER_LIVE_FLAG}] [{SCENARIO_ROUTER_BACKEND_FLAG} valence|paper] [{SCENARIO_ROUTER_RECEIPT_FLAG} PATH] [{SCENARIO_ROUTER_TIMEOUT_FLAG} SECS]"
    )
}

fn scenario_route_legacy_args(route: &ScenarioRouteRequest) -> Vec<String> {
    let mut args = Vec::new();
    args.push(format!("--{}", mode_name(route.mode)));
    args.push(SCENARIO_ROUTER_SERVER_BACKEND_FLAG.to_string());
    args.push(backend_name(route.backend).to_string());
    args.push(SCENARIO_ROUTER_LEGACY_SCENARIO_FLAG.to_string());
    args.push(scenario_name(route.scenario).to_string());
    if let Some(path) = &route.receipt_path {
        args.push(SCENARIO_ROUTER_RECEIPT_FLAG.to_string());
        args.push(path.display().to_string());
    }
    if let Some(path) = &route.failure_bundle_path {
        args.push(SCENARIO_ROUTER_FAILURE_BUNDLE_FLAG.to_string());
        args.push(path.display().to_string());
    }
    if let Some(timeout_secs) = route.timeout_secs {
        args.push(SCENARIO_ROUTER_TIMEOUT_FLAG.to_string());
        args.push(timeout_secs.to_string());
    }
    if route.packet_capture_summary {
        args.push(SCENARIO_ROUTER_PACKET_CAPTURE_FLAG.to_string());
    }
    if let Some(proxy_route) = &route.proxy_route {
        args.push(SCENARIO_ROUTER_PROXY_ROUTE_FLAG.to_string());
        args.push(proxy_route.clone());
    }
    if let Some(proxy_forwarding_mode) = &route.proxy_forwarding_mode {
        args.push(SCENARIO_ROUTER_PROXY_FORWARDING_MODE_FLAG.to_string());
        args.push(proxy_forwarding_mode.clone());
    }
    args.extend(route.passthrough_args.clone());
    args
}

fn parse_client_timeout_secs(value: &str, flag: &str) -> Result<u64, String> {
    let timeout_secs = value
        .parse::<u64>()
        .map_err(|err| format!("parse {flag}: {err}"))?;
    if timeout_secs == 0 {
        return Err(format!("{flag} must be greater than zero"));
    }
    Ok(timeout_secs)
}

impl Config {
    fn defaults(root: PathBuf) -> Result<Self, String> {
        let source_layout = resolve_repository_layout(&root, LayoutResolutionMode::AllowMissing)?;
        Ok(Config {
            client_dir: source_layout.client.path,
            valence_repo: source_layout.valence.path,
            valence_rev: DEFAULT_VALENCE_REV.to_string(),
            valence_worktree: PathBuf::from("/tmp/valence-compat-758"),
            valence_example: DEFAULT_VALENCE_EXAMPLE.to_string(),
            valence_log: PathBuf::from("/tmp/mc-compat-valence.log"),
            valence_target_dir: PathBuf::from("/tmp/valence-compat-758-target"),
            valence_pid_file: PathBuf::from("/tmp/mc-compat-valence.pid"),
            server_backend: ServerBackend::Valence,
            target_dir: PathBuf::from("/tmp/stevenarella-target2"),
            server_name: "mc-compat-1-18-2".to_string(),
            server_version: DEFAULT_SERVER_VERSION.to_string(),
            server_protocol: DEFAULT_SERVER_PROTOCOL,
            server_port: 25565,
            client_username: DEFAULT_CLIENT_USERNAME.to_string(),
            docker_image: "itzg/minecraft-server:java17".to_string(),
            paper_plugin_jar: None,
            mode: Mode::DryRun,
            keep_server: false,
            client_timeout: Duration::from_secs(DEFAULT_CLIENT_TIMEOUT_SECS),
            client_success_needles: DEFAULT_SUCCESS_PATTERN
                .iter()
                .map(|s| s.to_string())
                .collect(),
            scenario: Scenario::Smoke,
            expected_status_description: None,
            expected_status_version_name: None,
            expected_status_sample: Vec::new(),
            packet_capture_summary: false,
            proxy_route: None,
            proxy_forwarding_mode: None,
            receipt_path: None,
            receipt_dir: None,
            failure_bundle_path: None,
            compare_receipts: None,
            config_path: None,
            steel_config_path: None,
            matrix_dry_run: false,
            cleanup_apply: false,
            negative_public_target: false,
            negative_external_authorized: false,
            arrow_damage_policy: default_arrow_damage_policy(),
            scenario_route: None,
            root,
        })
    }

    fn from_env_and_args() -> Result<Self, String> {
        Self::from_sources(
            env::current_dir().map_err(|e| format!("current dir: {e}"))?,
            |name| env::var(name).ok().filter(|s| !s.is_empty()),
            env::args().skip(1),
        )
    }

    fn from_sources<I, F>(current_dir: PathBuf, mut get_env: F, args: I) -> Result<Self, String>
    where
        I: IntoIterator<Item = String>,
        F: FnMut(&str) -> Option<String>,
    {
        let args_vec: Vec<String> = args.into_iter().collect();
        let scenario_route = parse_scenario_route_request(&args_vec)?;
        let args_vec = scenario_route
            .as_ref()
            .map(scenario_route_legacy_args)
            .unwrap_or(args_vec);
        let root = get_env("MC_COMPAT_ROOT")
            .or_else(|| get_env("ROOT"))
            .map(PathBuf::from)
            .unwrap_or(current_dir);
        let base = Config::defaults(root)?;

        let mut patches = Vec::new();
        let config_path = find_config_path(get_env("MC_COMPAT_CONFIG"), &args_vec)?;
        let steel_config_path = find_named_config_path(
            "--steel-config",
            "MC_COMPAT_STEEL_CONFIG",
            get_env("MC_COMPAT_STEEL_CONFIG"),
            &args_vec,
        )?;
        if let Some(path) = config_path {
            patches.push(config_file_patch(
                &path,
                config_patches::ConfigSource::nickel_json(format!(
                    "selected config {}",
                    path.display()
                )),
            )?);
        }
        if let Some(path) = steel_config_path {
            patches.push(steel_config_file_patch(
                &path,
                config_patches::ConfigSource::steel(format!(
                    "selected Steel config {}",
                    path.display()
                )),
            )?);
        }

        let env_patch = env_config_patch(&mut get_env)?;
        if env_patch.has_updates() {
            patches.push(env_patch);
        }
        patches.extend(cli_config_patches(&args_vec, &base)?);

        let mut resolution = config_patches::resolve_config(base, &patches)
            .map_err(|diagnostics| config_patches::format_validation_diagnostics(&diagnostics))?;
        resolution.config.scenario_route = scenario_route;
        Ok(resolution.config)
    }
}

const CLI_CONFIG_FLAG: &str = "--config";
const CLI_CONFIG_EQUALS_PREFIX: &str = "--config=";
const CLI_STEEL_CONFIG_FLAG: &str = "--steel-config";
const CLI_STEEL_CONFIG_EQUALS_PREFIX: &str = "--steel-config=";
const CLI_COMPARE_RECEIPTS_FLAG: &str = "--compare-receipts";
const CLI_ACCEPT_EULA_FLAG: &str = "--accept-eula";
const CLI_KEEP_SERVER_FLAG: &str = "--keep-server";
const CLI_CLIENT_DIR_FLAG: &str = "--client-dir";
const CLI_CLIENT_DIR_EQUALS_PREFIX: &str = "--client-dir=";
const CLI_RECEIPT_DIR_FLAG: &str = "--receipt-dir";
const CLI_RECEIPT_DIR_EQUALS_PREFIX: &str = "--receipt-dir=";
const CLI_EXPECT_STATUS_DESCRIPTION_FLAG: &str = "--expect-status-description";
const CLI_EXPECT_STATUS_DESCRIPTION_EQUALS_PREFIX: &str = "--expect-status-description=";
const CLI_EXPECT_STATUS_VERSION_FLAG: &str = "--expect-status-version";
const CLI_EXPECT_STATUS_VERSION_EQUALS_PREFIX: &str = "--expect-status-version=";
const CLI_EXPECT_STATUS_SAMPLE_FLAG: &str = "--expect-status-sample";
const CLI_EXPECT_STATUS_SAMPLE_EQUALS_PREFIX: &str = "--expect-status-sample=";
const CLI_VALENCE_REPO_FLAG: &str = "--valence-repo";
const CLI_VALENCE_REPO_EQUALS_PREFIX: &str = "--valence-repo=";
const CLI_VALENCE_REV_FLAG: &str = "--valence-rev";
const CLI_VALENCE_REV_EQUALS_PREFIX: &str = "--valence-rev=";

fn find_config_path(env_path: Option<String>, args: &[String]) -> Result<Option<PathBuf>, String> {
    find_named_config_path("--config", "MC_COMPAT_CONFIG", env_path, args)
}

fn find_named_config_path(
    flag: &'static str,
    env_name: &'static str,
    env_path: Option<String>,
    args: &[String],
) -> Result<Option<PathBuf>, String> {
    let mut config_path = env_path.map(PathBuf::from);
    let equals_prefix = format!("{flag}=");
    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        if arg == flag {
            let value = iter
                .next()
                .ok_or_else(|| format!("{flag} requires a path; env alternative is {env_name}"))?;
            config_path = Some(PathBuf::from(value));
        } else if let Some(value) = arg.strip_prefix(&equals_prefix) {
            config_path = Some(PathBuf::from(value));
        }
    }
    Ok(config_path)
}

fn cli_config_patches(
    args: &[String],
    base: &Config,
) -> Result<Vec<config_patches::ConfigPatch>, String> {
    let mut patches = Vec::new();
    let mut current =
        config_patches::ConfigPatch::new(config_patches::ConfigSource::cli("CLI arguments"));
    let mut current_mode = base.mode;
    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            SCENARIO_ROUTER_DRY_RUN_FLAG => {
                if current_mode == Mode::RunMatrix {
                    current.matrix_dry_run = Some(true);
                } else if current_mode == Mode::Cleanup {
                    current.cleanup_apply = Some(false);
                } else {
                    current.mode = Some(Mode::DryRun);
                    current_mode = Mode::DryRun;
                }
            }
            SCENARIO_ROUTER_RUN_FLAG => {
                current.mode = Some(Mode::Run);
                current_mode = Mode::Run;
            }
            "--run-matrix" => {
                current.mode = Some(Mode::RunMatrix);
                current.matrix_dry_run = Some(false);
                current_mode = Mode::RunMatrix;
            }
            "--build-client" => {
                current.mode = Some(Mode::BuildClient);
                current_mode = Mode::BuildClient;
            }
            "--status-only" => {
                current.mode = Some(Mode::StatusOnly);
                current_mode = Mode::StatusOnly;
            }
            "--status" => {
                current.mode = Some(Mode::HarnessStatus);
                current_mode = Mode::HarnessStatus;
            }
            "--cleanup" => {
                current.mode = Some(Mode::Cleanup);
                current_mode = Mode::Cleanup;
            }
            "--apply" => current.cleanup_apply = Some(true),
            "--stop" => {
                current.mode = Some(Mode::Stop);
                current_mode = Mode::Stop;
            }
            CLI_CONFIG_FLAG => {
                let path = PathBuf::from(next_cli_value(
                    CLI_CONFIG_FLAG,
                    &mut iter,
                    "--config requires a Nickel-exported JSON path",
                )?);
                push_patch_if_nonempty(&mut patches, &mut current);
                patches.push(config_file_patch(
                    &path,
                    config_patches::ConfigSource::nickel_json(format!(
                        "CLI --config {}",
                        path.display()
                    )),
                )?);
            }
            CLI_STEEL_CONFIG_FLAG => {
                let path = PathBuf::from(next_cli_value(
                    CLI_STEEL_CONFIG_FLAG,
                    &mut iter,
                    "--steel-config requires a Steel module path",
                )?);
                push_patch_if_nonempty(&mut patches, &mut current);
                patches.push(steel_config_file_patch(
                    &path,
                    config_patches::ConfigSource::steel(format!(
                        "CLI --steel-config {}",
                        path.display()
                    )),
                )?);
            }
            CLI_COMPARE_RECEIPTS_FLAG => {
                let left = PathBuf::from(next_cli_value(
                    CLI_COMPARE_RECEIPTS_FLAG,
                    &mut iter,
                    "--compare-receipts requires PAPER_RECEIPT and VALENCE_RECEIPT",
                )?);
                let right = PathBuf::from(next_cli_value(
                    CLI_COMPARE_RECEIPTS_FLAG,
                    &mut iter,
                    "--compare-receipts requires PAPER_RECEIPT and VALENCE_RECEIPT",
                )?);
                current.mode = Some(Mode::CompareReceipts);
                current.compare_receipts = Some((left, right));
                current_mode = Mode::CompareReceipts;
            }
            CLI_ACCEPT_EULA_FLAG => {}
            CLI_KEEP_SERVER_FLAG => current.keep_server = Some(true),
            SCENARIO_ROUTER_SERVER_BACKEND_FLAG => {
                let value = next_cli_value(
                    SCENARIO_ROUTER_SERVER_BACKEND_FLAG,
                    &mut iter,
                    "--server-backend requires valence or paper",
                )?;
                current.server_backend = Some(parse_backend(&value)?);
            }
            CLI_CLIENT_DIR_FLAG => {
                current.client_dir = Some(PathBuf::from(next_cli_value(
                    CLI_CLIENT_DIR_FLAG,
                    &mut iter,
                    "--client-dir requires a path",
                )?));
            }
            SCENARIO_ROUTER_RECEIPT_FLAG => {
                current.receipt_path = Some(PathBuf::from(next_cli_value(
                    SCENARIO_ROUTER_RECEIPT_FLAG,
                    &mut iter,
                    "--receipt requires a path",
                )?));
            }
            CLI_RECEIPT_DIR_FLAG => {
                current.receipt_dir = Some(PathBuf::from(next_cli_value(
                    CLI_RECEIPT_DIR_FLAG,
                    &mut iter,
                    "--receipt-dir requires a path",
                )?));
            }
            SCENARIO_ROUTER_FAILURE_BUNDLE_FLAG => {
                current.failure_bundle_path = Some(PathBuf::from(next_cli_value(
                    SCENARIO_ROUTER_FAILURE_BUNDLE_FLAG,
                    &mut iter,
                    "--failure-bundle requires a path",
                )?));
            }
            SCENARIO_ROUTER_TIMEOUT_FLAG => {
                let value = next_cli_value(
                    SCENARIO_ROUTER_TIMEOUT_FLAG,
                    &mut iter,
                    "--timeout requires seconds",
                )?;
                current.client_timeout = Some(Duration::from_secs(parse_client_timeout_secs(
                    &value,
                    SCENARIO_ROUTER_TIMEOUT_FLAG,
                )?));
            }
            SCENARIO_ROUTER_LEGACY_SCENARIO_FLAG => {
                let value = next_cli_value(
                    SCENARIO_ROUTER_LEGACY_SCENARIO_FLAG,
                    &mut iter,
                    &format!("--scenario requires one of: {SUPPORTED_SCENARIO_USAGE}"),
                )?;
                current.scenario = Some(parse_scenario(&value)?);
            }
            CLI_EXPECT_STATUS_DESCRIPTION_FLAG => {
                current.expected_status_description = Some(next_cli_value(
                    CLI_EXPECT_STATUS_DESCRIPTION_FLAG,
                    &mut iter,
                    "--expect-status-description requires a string",
                )?);
            }
            CLI_EXPECT_STATUS_VERSION_FLAG => {
                current.expected_status_version_name = Some(next_cli_value(
                    CLI_EXPECT_STATUS_VERSION_FLAG,
                    &mut iter,
                    "--expect-status-version requires a string",
                )?);
            }
            CLI_EXPECT_STATUS_SAMPLE_FLAG => {
                current.expected_status_sample = Some(parse_comma_list(&next_cli_value(
                    CLI_EXPECT_STATUS_SAMPLE_FLAG,
                    &mut iter,
                    "--expect-status-sample requires comma-separated names",
                )?));
            }
            SCENARIO_ROUTER_PACKET_CAPTURE_FLAG => current.packet_capture_summary = Some(true),
            SCENARIO_ROUTER_PROXY_ROUTE_FLAG => {
                current.proxy_route = Some(next_cli_value(
                    SCENARIO_ROUTER_PROXY_ROUTE_FLAG,
                    &mut iter,
                    "--proxy-route requires a route label",
                )?);
            }
            SCENARIO_ROUTER_PROXY_FORWARDING_MODE_FLAG => {
                current.proxy_forwarding_mode = Some(next_cli_value(
                    SCENARIO_ROUTER_PROXY_FORWARDING_MODE_FLAG,
                    &mut iter,
                    "--proxy-forwarding-mode requires a mode label",
                )?);
            }
            CLI_VALENCE_REPO_FLAG => {
                current.valence_repo = Some(PathBuf::from(next_cli_value(
                    CLI_VALENCE_REPO_FLAG,
                    &mut iter,
                    "--valence-repo requires a path",
                )?));
            }
            CLI_VALENCE_REV_FLAG => {
                current.valence_rev = Some(next_cli_value(
                    CLI_VALENCE_REV_FLAG,
                    &mut iter,
                    "--valence-rev requires a git revision",
                )?);
            }
            "-h" | "--help" => {
                print_usage(base);
                std::process::exit(0);
            }
            _ if arg.starts_with(CLI_CONFIG_EQUALS_PREFIX) => {
                let path = PathBuf::from(
                    arg.strip_prefix(CLI_CONFIG_EQUALS_PREFIX)
                        .expect("prefix checked"),
                );
                push_patch_if_nonempty(&mut patches, &mut current);
                patches.push(config_file_patch(
                    &path,
                    config_patches::ConfigSource::nickel_json(format!(
                        "CLI --config {}",
                        path.display()
                    )),
                )?);
            }
            _ if arg.starts_with(CLI_STEEL_CONFIG_EQUALS_PREFIX) => {
                let path = PathBuf::from(
                    arg.strip_prefix(CLI_STEEL_CONFIG_EQUALS_PREFIX)
                        .expect("prefix checked"),
                );
                push_patch_if_nonempty(&mut patches, &mut current);
                patches.push(steel_config_file_patch(
                    &path,
                    config_patches::ConfigSource::steel(format!(
                        "CLI --steel-config {}",
                        path.display()
                    )),
                )?);
            }
            _ if arg.starts_with(SCENARIO_ROUTER_SERVER_BACKEND_EQUALS_PREFIX) => {
                let value = arg
                    .strip_prefix(SCENARIO_ROUTER_SERVER_BACKEND_EQUALS_PREFIX)
                    .expect("prefix checked");
                current.server_backend = Some(parse_backend(value)?);
            }
            _ if arg.starts_with(CLI_CLIENT_DIR_EQUALS_PREFIX) => {
                current.client_dir = Some(PathBuf::from(
                    arg.strip_prefix(CLI_CLIENT_DIR_EQUALS_PREFIX)
                        .expect("prefix checked"),
                ));
            }
            _ if arg.starts_with(SCENARIO_ROUTER_RECEIPT_EQUALS_PREFIX) => {
                current.receipt_path = Some(PathBuf::from(
                    arg.strip_prefix(SCENARIO_ROUTER_RECEIPT_EQUALS_PREFIX)
                        .expect("prefix checked"),
                ));
            }
            _ if arg.starts_with(CLI_RECEIPT_DIR_EQUALS_PREFIX) => {
                current.receipt_dir = Some(PathBuf::from(
                    arg.strip_prefix(CLI_RECEIPT_DIR_EQUALS_PREFIX)
                        .expect("prefix checked"),
                ));
            }
            _ if arg.starts_with(SCENARIO_ROUTER_FAILURE_BUNDLE_EQUALS_PREFIX) => {
                current.failure_bundle_path = Some(PathBuf::from(
                    arg.strip_prefix(SCENARIO_ROUTER_FAILURE_BUNDLE_EQUALS_PREFIX)
                        .expect("prefix checked"),
                ));
            }
            _ if arg.starts_with(SCENARIO_ROUTER_TIMEOUT_EQUALS_PREFIX) => {
                let value = arg
                    .strip_prefix(SCENARIO_ROUTER_TIMEOUT_EQUALS_PREFIX)
                    .expect("prefix checked");
                current.client_timeout = Some(Duration::from_secs(parse_client_timeout_secs(
                    value,
                    SCENARIO_ROUTER_TIMEOUT_FLAG,
                )?));
            }
            _ if arg.starts_with(SCENARIO_ROUTER_LEGACY_SCENARIO_EQUALS_PREFIX) => {
                let value = arg
                    .strip_prefix(SCENARIO_ROUTER_LEGACY_SCENARIO_EQUALS_PREFIX)
                    .expect("prefix checked");
                current.scenario = Some(parse_scenario(value)?);
            }
            _ if arg.starts_with(CLI_EXPECT_STATUS_DESCRIPTION_EQUALS_PREFIX) => {
                current.expected_status_description = Some(
                    arg.strip_prefix(CLI_EXPECT_STATUS_DESCRIPTION_EQUALS_PREFIX)
                        .expect("prefix checked")
                        .to_string(),
                );
            }
            _ if arg.starts_with(CLI_EXPECT_STATUS_VERSION_EQUALS_PREFIX) => {
                current.expected_status_version_name = Some(
                    arg.strip_prefix(CLI_EXPECT_STATUS_VERSION_EQUALS_PREFIX)
                        .expect("prefix checked")
                        .to_string(),
                );
            }
            _ if arg.starts_with(CLI_EXPECT_STATUS_SAMPLE_EQUALS_PREFIX) => {
                current.expected_status_sample = Some(parse_comma_list(
                    arg.strip_prefix(CLI_EXPECT_STATUS_SAMPLE_EQUALS_PREFIX)
                        .expect("prefix checked"),
                ));
            }
            _ if arg.starts_with(SCENARIO_ROUTER_PROXY_ROUTE_EQUALS_PREFIX) => {
                current.proxy_route = Some(
                    arg.strip_prefix(SCENARIO_ROUTER_PROXY_ROUTE_EQUALS_PREFIX)
                        .expect("prefix checked")
                        .to_string(),
                );
            }
            _ if arg.starts_with(SCENARIO_ROUTER_PROXY_FORWARDING_MODE_EQUALS_PREFIX) => {
                current.proxy_forwarding_mode = Some(
                    arg.strip_prefix(SCENARIO_ROUTER_PROXY_FORWARDING_MODE_EQUALS_PREFIX)
                        .expect("prefix checked")
                        .to_string(),
                );
            }
            _ if arg.starts_with(CLI_VALENCE_REPO_EQUALS_PREFIX) => {
                current.valence_repo = Some(PathBuf::from(
                    arg.strip_prefix(CLI_VALENCE_REPO_EQUALS_PREFIX)
                        .expect("prefix checked"),
                ));
            }
            _ if arg.starts_with(CLI_VALENCE_REV_EQUALS_PREFIX) => {
                current.valence_rev = Some(
                    arg.strip_prefix(CLI_VALENCE_REV_EQUALS_PREFIX)
                        .expect("prefix checked")
                        .to_string(),
                );
            }
            _ => return Err(format!("unknown arg: {arg}")),
        }
    }
    push_patch_if_nonempty(&mut patches, &mut current);
    Ok(patches)
}

fn push_patch_if_nonempty(
    patches: &mut Vec<config_patches::ConfigPatch>,
    patch: &mut config_patches::ConfigPatch,
) {
    let source = patch.source.clone();
    let next = config_patches::ConfigPatch::new(source);
    let candidate = std::mem::replace(patch, next);
    if candidate.has_updates() {
        patches.push(candidate);
    }
}

fn next_cli_value<'a, I>(_flag: &str, iter: &mut I, missing: &str) -> Result<String, String>
where
    I: Iterator<Item = &'a String>,
{
    iter.next().cloned().ok_or_else(|| missing.to_string())
}

fn parse_comma_list(value: &str) -> Vec<String> {
    value
        .split(',')
        .filter(|sample| !sample.is_empty())
        .map(str::to_string)
        .collect()
}

fn env_config_patch<F>(get_env: &mut F) -> Result<config_patches::ConfigPatch, String>
where
    F: FnMut(&str) -> Option<String>,
{
    let mut patch = config_patches::ConfigPatch::new(config_patches::ConfigSource::environment());
    if let Some(value) = get_env("CLIENT_DIR") {
        patch.client_dir = Some(PathBuf::from(value));
    }
    if let Some(value) = get_env("VALENCE_REPO") {
        patch.valence_repo = Some(PathBuf::from(value));
    }
    if let Some(value) = get_env("VALENCE_REV") {
        patch.valence_rev = Some(value);
    }
    if let Some(value) = get_env("VALENCE_WORKTREE") {
        patch.valence_worktree = Some(PathBuf::from(value));
    }
    if let Some(value) = get_env("VALENCE_EXAMPLE") {
        patch.valence_example = Some(value);
    }
    if let Some(value) = get_env("VALENCE_LOG") {
        patch.valence_log = Some(PathBuf::from(value));
    }
    if let Some(value) = get_env("VALENCE_TARGET_DIR") {
        patch.valence_target_dir = Some(PathBuf::from(value));
    }
    if let Some(value) = get_env("VALENCE_PID_FILE") {
        patch.valence_pid_file = Some(PathBuf::from(value));
    }
    if let Some(value) = get_env("SERVER_BACKEND") {
        patch.server_backend = Some(parse_backend(&value)?);
    }
    if let Some(value) = get_env("TARGET_DIR") {
        patch.target_dir = Some(PathBuf::from(value));
    }
    if let Some(value) = get_env("SERVER_NAME") {
        patch.server_name = Some(value);
    }
    if let Some(value) = get_env("SERVER_VERSION") {
        patch.server_version = Some(value);
    }
    if let Some(value) = get_env("SERVER_PROTOCOL") {
        patch.server_protocol = Some(
            value
                .parse()
                .map_err(|e| format!("parse SERVER_PROTOCOL: {e}"))?,
        );
    }
    if let Some(value) = get_env("SERVER_PORT") {
        patch.server_port = Some(
            value
                .parse()
                .map_err(|e| format!("parse SERVER_PORT: {e}"))?,
        );
    }
    if let Some(value) = get_env("CLIENT_USERNAME") {
        patch.client_username = Some(value);
    }
    if let Some(value) = get_env("DOCKER_IMAGE") {
        patch.docker_image = Some(value);
    }
    if let Some(value) = get_env("PAPER_PLUGIN_JAR") {
        patch.paper_plugin_jar = Some(PathBuf::from(value));
    }
    if let Some(value) = get_env("CLIENT_TIMEOUT") {
        patch.client_timeout = Some(Duration::from_secs(
            value
                .parse()
                .map_err(|e| format!("parse CLIENT_TIMEOUT: {e}"))?,
        ));
    }
    if let Some(value) = get_env("CLIENT_SUCCESS_PATTERN") {
        patch.client_success_needles = Some(value.split('|').map(str::to_string).collect());
    }
    if let Some(value) = get_env("MC_COMPAT_SCENARIO") {
        patch.scenario = Some(parse_scenario(&value)?);
    }
    if let Some(value) = get_env("MC_COMPAT_EXPECT_STATUS_DESCRIPTION") {
        patch.expected_status_description = Some(value);
    }
    if let Some(value) = get_env("MC_COMPAT_EXPECT_STATUS_VERSION") {
        patch.expected_status_version_name = Some(value);
    }
    if let Some(value) = get_env("MC_COMPAT_EXPECT_STATUS_SAMPLE") {
        patch.expected_status_sample = Some(
            value
                .split(',')
                .filter(|sample| !sample.is_empty())
                .map(str::to_string)
                .collect(),
        );
    }
    if get_env("MC_COMPAT_PACKET_CAPTURE_SUMMARY").is_some() {
        patch.packet_capture_summary = Some(true);
    }
    if let Some(value) = get_env("MC_COMPAT_PUBLIC_TARGET") {
        patch.negative_public_target = Some(value == "1");
    }
    if let Some(value) = get_env("MC_COMPAT_EXTERNAL_LOAD_AUTHORIZED") {
        patch.negative_external_authorized = Some(value == "1");
    }
    if let Some(value) = get_env("MC_COMPAT_PROXY_ROUTE") {
        patch.proxy_route = Some(value);
    }
    if let Some(value) = get_env("MC_COMPAT_PROXY_FORWARDING_MODE") {
        patch.proxy_forwarding_mode = Some(value);
    }
    if let Some(value) = get_env("SMOKE_RECEIPT") {
        patch.receipt_path = Some(PathBuf::from(value));
    }
    if let Some(value) = get_env("SMOKE_RECEIPT_DIR") {
        patch.receipt_dir = Some(PathBuf::from(value));
    }
    if let Some(value) = get_env("MC_COMPAT_FAILURE_BUNDLE") {
        patch.failure_bundle_path = Some(PathBuf::from(value));
    }
    Ok(patch)
}

fn config_file_patch(
    path: &Path,
    source: config_patches::ConfigSource,
) -> Result<config_patches::ConfigPatch, String> {
    let text =
        fs::read_to_string(path).map_err(|e| format!("read config {}: {e}", path.display()))?;
    let mut patch = config_patches::config_json_patch(source, &text)
        .map_err(|e| format!("config {}: {e}", path.display()))?;
    patch.config_path = Some(path.to_path_buf());
    Ok(patch)
}

fn steel_config_file_patch(
    path: &Path,
    source: config_patches::ConfigSource,
) -> Result<config_patches::ConfigPatch, String> {
    let text = fs::read_to_string(path)
        .map_err(|e| format!("read Steel config {}: {e}", path.display()))?;
    let steel_source = runtime_config::SteelSource {
        path: path.display().to_string(),
        module_blake3: "runtime-unverified".to_string(),
        sandbox_profile: "mc-compat/pure-v1".to_string(),
    };
    let snapshot =
        runtime_config::evaluate_steel_module(steel_source, &text).map_err(|diagnostics| {
            let details = diagnostics
                .into_iter()
                .map(|diagnostic| format!("{}: {}", diagnostic.path, diagnostic.message))
                .collect::<Vec<_>>()
                .join("; ");
            format!("Steel config {}: {details}", path.display())
        })?;
    let mut patch = config_patches::steel_snapshot_patch(source, snapshot)?;
    patch.steel_config_path = Some(path.to_path_buf());
    Ok(patch)
}

fn apply_config_json(cfg: &mut Config, text: &str) -> Result<bool, String> {
    let patch = config_patches::config_json_patch(
        config_patches::ConfigSource::nickel_json("in-memory JSON config"),
        text,
    )?;
    let server_port_was_set = patch.sets_server_port();
    config_patches::apply_patch_for_legacy_mutation(cfg, &patch);
    Ok(server_port_was_set)
}

trait EnvPatchValue {
    fn into_env_value(self) -> String;
}

impl EnvPatchValue for &str {
    fn into_env_value(self) -> String {
        self.to_string()
    }
}

impl EnvPatchValue for String {
    fn into_env_value(self) -> String {
        self
    }
}

impl EnvPatchValue for &String {
    fn into_env_value(self) -> String {
        self.clone()
    }
}

impl EnvPatchValue for &Path {
    fn into_env_value(self) -> String {
        self.display().to_string()
    }
}

impl EnvPatchValue for PathBuf {
    fn into_env_value(self) -> String {
        self.display().to_string()
    }
}

impl EnvPatchValue for &PathBuf {
    fn into_env_value(self) -> String {
        self.display().to_string()
    }
}

struct EnvPatchBuilder {
    source: String,
    patch: EnvPatch,
    diagnostics: Vec<EnvPatchDiagnostic>,
}

impl EnvPatchBuilder {
    fn new(source: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            patch: EnvPatch::new(),
            diagnostics: Vec::new(),
        }
    }

    fn env(&mut self, key: impl Into<String>, value: impl EnvPatchValue) -> &mut Self {
        if let Err(err) = self
            .patch
            .push_set(self.source.clone(), key, value.into_env_value())
        {
            self.diagnostics.push(err);
        }
        self
    }

    fn env_remove(&mut self, key: impl Into<String>) -> &mut Self {
        if let Err(err) = self.patch.push_remove(self.source.clone(), key) {
            self.diagnostics.push(err);
        }
        self
    }

    fn finish(self) -> Result<EnvPatch, String> {
        let Self {
            patch, diagnostics, ..
        } = self;
        if !diagnostics.is_empty() {
            return Err(diagnostics
                .into_iter()
                .map(|diagnostic| diagnostic.to_string())
                .collect::<Vec<_>>()
                .join("; "));
        }
        EnvPatch::compose(&[patch]).map_err(|diagnostic| diagnostic.to_string())
    }
}

fn apply_env_patch_to_command(cmd: &mut Command, patch: &EnvPatch) {
    for removal in patch.removals() {
        cmd.env_remove(&removal.key);
    }
    for entry in patch.entries() {
        cmd.env(&entry.key, &entry.value);
    }
}

fn apply_env_patch_to_paper_args(cmd: &mut Command, patch: &EnvPatch) -> Result<(), String> {
    if let Some(removal) = patch.removals().first() {
        return Err(format!(
            "Paper docker env cannot remove key {} from source {}",
            removal.key, removal.source
        ));
    }
    for entry in patch.entries() {
        cmd.arg("-e").arg(format!("{}={}", entry.key, entry.value));
    }
    Ok(())
}

fn scenario_behavior(scenario: Scenario) -> &'static dyn ScenarioBehavior {
    scenario_behavior_kind(scenario)
}

trait ScenarioBehavior {
    fn client_milestone_matcher<'a>(
        &self,
        id: &'static str,
        needle: &'static str,
        projectile_health_needle: &'a str,
    ) -> MatcherKind<'a>;
    fn run_strategy(&self) -> ScenarioRunStrategy;
    fn safety_reconnect_sessions(&self) -> usize;
    fn negative_live_rail(&self) -> Option<NegativeLiveRailBehavior>;
    fn requires_server_correlation(&self) -> bool;
    fn uses_armor_mitigation_probe(&self) -> bool;
    fn uses_dynamic_projectile_health(&self) -> bool;
    fn is_mcp_controlled_smoke(&self) -> bool;
    fn uses_isolated_restart_storage(&self) -> bool;
    fn uses_crash_recovery_restart(&self) -> bool;
    fn uses_block_entity_persistence_storage(&self) -> bool;
    fn uses_world_multichunk_storage(&self) -> bool;
    fn world_persistence_artifact_dir_name(&self) -> &'static str;
    fn uses_reconnect_session_marker(&self) -> bool;
    fn append_client_count_markers(&self, run_count: usize, output: &mut String);
    fn client_probe_env_patch(
        &self,
        client_index: usize,
        server_backend: ServerBackend,
    ) -> Result<EnvPatch, String>;
    fn valence_server_env_patch(&self, cfg: &Config) -> Result<EnvPatch, String>;
    fn paper_server_env_patch(&self, cfg: &Config) -> Result<EnvPatch, String>;
}

impl ScenarioBehavior for ScenarioBehaviorKind {
    fn client_milestone_matcher<'a>(
        &self,
        id: &'static str,
        needle: &'static str,
        projectile_health_needle: &'a str,
    ) -> MatcherKind<'a> {
        if self.uses_dynamic_projectile_health() && id == PROJECTILE_DAMAGE_UPDATE_MILESTONE {
            MatcherKind::Literal(projectile_health_needle)
        } else {
            MatcherKind::Literal(needle)
        }
    }

    fn run_strategy(&self) -> ScenarioRunStrategy {
        ScenarioBehaviorKind::run_strategy(self)
    }

    fn safety_reconnect_sessions(&self) -> usize {
        match self {
            ScenarioBehaviorKind::FlagScore {
                reconnect: true, ..
            }
            | ScenarioBehaviorKind::ReconnectFlagState { .. }
            | ScenarioBehaviorKind::SurvivalChestPersistence
            | ScenarioBehaviorKind::SurvivalFurnacePersistence
            | ScenarioBehaviorKind::SurvivalWorldMultichunkDurability => {
                SAFETY_RECONNECT_SESSION_COUNT
            }
            _ => SAFETY_SINGLE_SESSION_COUNT,
        }
    }

    fn negative_live_rail(&self) -> Option<NegativeLiveRailBehavior> {
        ScenarioBehaviorKind::negative_live_rail(self)
    }

    fn requires_server_correlation(&self) -> bool {
        matches!(
            self,
            ScenarioBehaviorKind::FlagScore {
                team: ProbeTeam::Red,
                ..
            } | ScenarioBehaviorKind::InventoryInteraction
                | ScenarioBehaviorKind::InventoryStackSplitMerge
                | ScenarioBehaviorKind::InventoryDragTransactions
                | ScenarioBehaviorKind::SurvivalBreakPlacePickup
                | ScenarioBehaviorKind::SurvivalChestPersistence
                | ScenarioBehaviorKind::SurvivalCraftingTable
                | ScenarioBehaviorKind::SurvivalFurnacePersistence
                | ScenarioBehaviorKind::SurvivalFurnaceSmeltingBreadth
                | ScenarioBehaviorKind::SurvivalHungerFood
                | ScenarioBehaviorKind::SurvivalHungerHealthCycle
                | ScenarioBehaviorKind::SurvivalMobDrop
                | ScenarioBehaviorKind::SurvivalMobAiLootBreadth
                | ScenarioBehaviorKind::SurvivalRedstoneToggle
                | ScenarioBehaviorKind::SurvivalRedstoneCircuitBreadth
                | ScenarioBehaviorKind::WorldPersistenceRestart { .. }
                | ScenarioBehaviorKind::SurvivalWorldMultichunkDurability
                | ScenarioBehaviorKind::SurvivalContainerBlockEntityBreadth
                | ScenarioBehaviorKind::SurvivalBiomeDimensionState
                | ScenarioBehaviorKind::SurvivalBiomeDimensionTravel
                | ScenarioBehaviorKind::SurvivalSignEditingLive
                | ScenarioBehaviorKind::Combat { .. }
                | ScenarioBehaviorKind::EquipmentUpdate
                | ScenarioBehaviorKind::Projectile { .. }
                | ScenarioBehaviorKind::MultiClientLoadScore
                | ScenarioBehaviorKind::CtfInvalidPickupOwnership
                | ScenarioBehaviorKind::CtfInvalidReturnDrop
                | ScenarioBehaviorKind::CtfInvalidOpponentBaseReturnDrop
                | ScenarioBehaviorKind::CtfScoreLimitWinCondition
                | ScenarioBehaviorKind::CtfSimultaneousPickupCaptureRace
                | ScenarioBehaviorKind::CtfSpawnTeamBalanceReset
        )
    }

    fn uses_armor_mitigation_probe(&self) -> bool {
        matches!(
            self,
            ScenarioBehaviorKind::Combat {
                armor_mitigation: true,
                ..
            }
        )
    }

    fn uses_dynamic_projectile_health(&self) -> bool {
        ScenarioBehaviorKind::uses_dynamic_projectile_health(self)
    }

    fn is_mcp_controlled_smoke(&self) -> bool {
        ScenarioBehaviorKind::is_mcp_controlled_smoke(self)
    }

    fn uses_isolated_restart_storage(&self) -> bool {
        matches!(
            self,
            ScenarioBehaviorKind::WorldPersistenceRestart { .. }
                | ScenarioBehaviorKind::SurvivalWorldMultichunkDurability
        )
    }

    fn uses_crash_recovery_restart(&self) -> bool {
        matches!(
            self,
            ScenarioBehaviorKind::WorldPersistenceRestart {
                crash_recovery: true,
                ..
            }
        )
    }

    fn uses_block_entity_persistence_storage(&self) -> bool {
        matches!(
            self,
            ScenarioBehaviorKind::WorldPersistenceRestart {
                block_entity: true,
                ..
            }
        )
    }

    fn uses_world_multichunk_storage(&self) -> bool {
        matches!(
            self,
            ScenarioBehaviorKind::SurvivalWorldMultichunkDurability
        )
    }

    fn world_persistence_artifact_dir_name(&self) -> &'static str {
        match self {
            ScenarioBehaviorKind::WorldPersistenceRestart {
                crash_recovery: true,
                ..
            } => "mc-compat-survival-crash-recovery",
            ScenarioBehaviorKind::WorldPersistenceRestart {
                block_entity: true, ..
            } => "mc-compat-survival-block-entity-persistence",
            ScenarioBehaviorKind::SurvivalWorldMultichunkDurability => {
                "mc-compat-survival-world-multichunk-durability"
            }
            _ => "mc-compat-world-persistence",
        }
    }

    fn uses_reconnect_session_marker(&self) -> bool {
        matches!(
            self,
            ScenarioBehaviorKind::FlagScore {
                reconnect: true,
                ..
            } | ScenarioBehaviorKind::ReconnectFlagState { .. }
                | ScenarioBehaviorKind::SurvivalChestPersistence
                | ScenarioBehaviorKind::SurvivalFurnacePersistence
                | ScenarioBehaviorKind::WorldPersistenceRestart { .. }
                | ScenarioBehaviorKind::SurvivalWorldMultichunkDurability
        )
    }

    fn append_client_count_markers(&self, run_count: usize, output: &mut String) {
        if run_count < MULTI_CLIENT_READY_COUNT {
            return;
        }
        match self {
            ScenarioBehaviorKind::MultiClientLoadScore => {
                append_count_marker(output, MULTI_CLIENT_LOAD_COUNT_NEEDLE)
            }
            ScenarioBehaviorKind::Combat {
                reference_probe,
                count_needle,
                ..
            } => {
                if let Some(marker) = count_needle {
                    append_count_marker(output, marker);
                }
                if *reference_probe {
                    append_count_marker(output, VANILLA_COMBAT_REFERENCE_CLIENT_COUNT_NEEDLE);
                }
            }
            ScenarioBehaviorKind::EquipmentUpdate => {
                append_count_marker(output, EQUIPMENT_UPDATE_CLIENT_COUNT_NEEDLE)
            }
            ScenarioBehaviorKind::Projectile { damage: false } => {
                append_count_marker(output, PROJECTILE_HIT_CLIENT_COUNT_NEEDLE)
            }
            ScenarioBehaviorKind::Projectile { damage: true } => {
                append_count_marker(output, PROJECTILE_DAMAGE_CLIENT_COUNT_NEEDLE)
            }
            ScenarioBehaviorKind::CtfSimultaneousPickupCaptureRace => {
                append_count_marker(output, CTF_RACE_CLIENT_COUNT_NEEDLE)
            }
            ScenarioBehaviorKind::CtfSpawnTeamBalanceReset => {
                append_count_marker(output, CTF_SPAWN_TEAM_RESET_CLIENT_COUNT_NEEDLE)
            }
            _ => {}
        }
    }

    fn client_probe_env_patch(
        &self,
        client_index: usize,
        _server_backend: ServerBackend,
    ) -> Result<EnvPatch, String> {
        let mut cmd = EnvPatchBuilder::new(ENV_SOURCE_CLIENT_SCENARIO);
        match self {
            ScenarioBehaviorKind::Default => {}
            ScenarioBehaviorKind::CompatBotProbe => {
                cmd.env("MC_COMPAT_ACTIVE_PROBE", PROBE_ENABLED_VALUE);
            }
            ScenarioBehaviorKind::FlagScore { team, reconnect } => {
                let team = team.env_value();
                cmd.env("MC_COMPAT_ACTIVE_PROBE", PROBE_ENABLED_VALUE)
                    .env("MC_COMPAT_TEAM_PROBE", PROBE_ENABLED_VALUE)
                    .env("MC_COMPAT_TEAM_PROBE_TEAM", team)
                    .env("MC_COMPAT_FLAG_PROBE", PROBE_ENABLED_VALUE)
                    .env("MC_COMPAT_FLAG_PROBE_TEAM", team)
                    .env("MC_COMPAT_FLAG_PROBE_REPEAT", PROBE_REPEAT_DOUBLE);
                if *reconnect {
                    cmd.env("MC_COMPAT_RECONNECT_PROBE", PROBE_ENABLED_VALUE);
                }
            }
            ScenarioBehaviorKind::ReconnectFlagState { negative_probe } => {
                cmd.env("MC_COMPAT_ACTIVE_PROBE", PROBE_ENABLED_VALUE)
                    .env("MC_COMPAT_TEAM_PROBE", PROBE_ENABLED_VALUE)
                    .env("MC_COMPAT_TEAM_PROBE_TEAM", TEAM_RED_VALUE);
                if client_index == FIRST_CLIENT_INDEX {
                    cmd.env("MC_COMPAT_FLAG_PROBE", PROBE_ENABLED_VALUE)
                        .env("MC_COMPAT_FLAG_PROBE_TEAM", TEAM_RED_VALUE)
                        .env("MC_COMPAT_FLAG_PROBE_PICKUP_ONLY", PROBE_ENABLED_VALUE)
                        .env("MC_COMPAT_FLAG_PROBE_REPEAT", PROBE_REPEAT_SINGLE);
                }
                if let Some(probe) = negative_probe {
                    cmd.env("MC_COMPAT_NEGATIVE_PROBE", *probe);
                }
            }
            ScenarioBehaviorKind::InventoryInteraction => {
                cmd.env("MC_COMPAT_ACTIVE_PROBE", PROBE_ENABLED_VALUE)
                    .env("MC_COMPAT_TEAM_PROBE", PROBE_ENABLED_VALUE)
                    .env("MC_COMPAT_TEAM_PROBE_TEAM", TEAM_RED_VALUE)
                    .env("MC_COMPAT_INVENTORY_PROBE", PROBE_ENABLED_VALUE);
            }
            ScenarioBehaviorKind::InventoryStackSplitMerge => {
                cmd.env("MC_COMPAT_ACTIVE_PROBE", PROBE_ENABLED_VALUE)
                    .env("MC_COMPAT_TEAM_PROBE", PROBE_ENABLED_VALUE)
                    .env("MC_COMPAT_TEAM_PROBE_TEAM", TEAM_RED_VALUE)
                    .env("MC_COMPAT_INVENTORY_PROBE", PROBE_ENABLED_VALUE)
                    .env(INVENTORY_STACK_SPLIT_MERGE_PROBE_ENV, PROBE_ENABLED_VALUE);
            }
            ScenarioBehaviorKind::InventoryDragTransactions => {
                cmd.env("MC_COMPAT_ACTIVE_PROBE", PROBE_ENABLED_VALUE)
                    .env("MC_COMPAT_TEAM_PROBE", PROBE_ENABLED_VALUE)
                    .env("MC_COMPAT_TEAM_PROBE_TEAM", TEAM_RED_VALUE)
                    .env("MC_COMPAT_INVENTORY_PROBE", PROBE_ENABLED_VALUE)
                    .env(INVENTORY_DRAG_TRANSACTIONS_PROBE_ENV, PROBE_ENABLED_VALUE);
            }
            ScenarioBehaviorKind::NegativeInventory { probe, .. } => {
                cmd.env("MC_COMPAT_ACTIVE_PROBE", PROBE_ENABLED_VALUE)
                    .env("MC_COMPAT_TEAM_PROBE", PROBE_ENABLED_VALUE)
                    .env("MC_COMPAT_TEAM_PROBE_TEAM", TEAM_RED_VALUE)
                    .env("MC_COMPAT_INVENTORY_PROBE", PROBE_ENABLED_VALUE)
                    .env("MC_COMPAT_NEGATIVE_PROBE", *probe);
            }
            ScenarioBehaviorKind::NegativeCustomPayload => {
                cmd.env("MC_COMPAT_ACTIVE_PROBE", PROBE_ENABLED_VALUE)
                    .env("MC_COMPAT_NEGATIVE_PROBE", "custom_payload_malformed");
            }
            ScenarioBehaviorKind::SurvivalBreakPlacePickup => {
                cmd.env("MC_COMPAT_SURVIVAL_PROBE", PROBE_ENABLED_VALUE);
            }
            ScenarioBehaviorKind::SurvivalChestPersistence => {
                cmd.env("MC_COMPAT_SURVIVAL_CHEST_PROBE", PROBE_ENABLED_VALUE)
                    .env(
                        "MC_COMPAT_SURVIVAL_CHEST_SESSION",
                        required_session_env_value(ENV_SOURCE_CLIENT_SCENARIO, Some(client_index))?,
                    );
            }
            ScenarioBehaviorKind::SurvivalCraftingTable => {
                cmd.env("MC_COMPAT_SURVIVAL_CRAFTING_PROBE", PROBE_ENABLED_VALUE);
            }
            ScenarioBehaviorKind::SurvivalCraftingRecipeBreadth => {
                cmd.env(SURVIVAL_CRAFTING_BREADTH_PROBE_ENV, PROBE_ENABLED_VALUE);
            }
            ScenarioBehaviorKind::SurvivalFurnacePersistence => {
                cmd.env(SURVIVAL_FURNACE_PROBE_ENV, PROBE_ENABLED_VALUE)
                    .env(
                        SURVIVAL_FURNACE_SESSION_ENV,
                        required_session_env_value(ENV_SOURCE_CLIENT_SCENARIO, Some(client_index))?,
                    );
            }
            ScenarioBehaviorKind::SurvivalFurnaceSmeltingBreadth => {
                cmd.env(SURVIVAL_FURNACE_PROBE_ENV, PROBE_ENABLED_VALUE)
                    .env(
                        SURVIVAL_FURNACE_SMELTING_BREADTH_PROBE_ENV,
                        PROBE_ENABLED_VALUE,
                    )
                    .env(
                        SURVIVAL_FURNACE_SESSION_ENV,
                        required_session_env_value(
                            ENV_SOURCE_CLIENT_SCENARIO,
                            Some(FIRST_CLIENT_INDEX),
                        )?,
                    );
            }
            ScenarioBehaviorKind::SurvivalHungerFood => {
                cmd.env(SURVIVAL_HUNGER_FOOD_PROBE_ENV, PROBE_ENABLED_VALUE);
            }
            ScenarioBehaviorKind::SurvivalHungerHealthCycle => {
                cmd.env(SURVIVAL_HUNGER_HEALTH_PROBE_ENV, PROBE_ENABLED_VALUE);
            }
            ScenarioBehaviorKind::SurvivalMobDrop => {
                cmd.env(SURVIVAL_MOB_DROP_PROBE_ENV, PROBE_ENABLED_VALUE);
            }
            ScenarioBehaviorKind::SurvivalMobAiLootBreadth => {
                cmd.env(SURVIVAL_MOB_AI_LOOT_PROBE_ENV, PROBE_ENABLED_VALUE);
            }
            ScenarioBehaviorKind::SurvivalRedstoneToggle => {
                cmd.env(SURVIVAL_REDSTONE_TOGGLE_PROBE_ENV, PROBE_ENABLED_VALUE);
            }
            ScenarioBehaviorKind::SurvivalRedstoneCircuitBreadth => {
                cmd.env(SURVIVAL_REDSTONE_CIRCUIT_PROBE_ENV, PROBE_ENABLED_VALUE);
            }
            ScenarioBehaviorKind::WorldPersistenceRestart { block_entity, .. } => {
                if *block_entity {
                    cmd.env(SURVIVAL_BLOCK_ENTITY_PROBE_ENV, PROBE_ENABLED_VALUE)
                        .env(
                            SURVIVAL_BLOCK_ENTITY_SESSION_ENV,
                            required_session_env_value(
                                ENV_SOURCE_CLIENT_SCENARIO,
                                Some(client_index),
                            )?,
                        );
                } else {
                    cmd.env(SURVIVAL_WORLD_PERSISTENCE_PROBE_ENV, PROBE_ENABLED_VALUE)
                        .env(
                            SURVIVAL_WORLD_PERSISTENCE_SESSION_ENV,
                            required_session_env_value(
                                ENV_SOURCE_CLIENT_SCENARIO,
                                Some(client_index),
                            )?,
                        );
                }
            }
            ScenarioBehaviorKind::SurvivalWorldMultichunkDurability => {
                cmd.env(SURVIVAL_WORLD_MULTICHUNK_PROBE_ENV, PROBE_ENABLED_VALUE)
                    .env(
                        SURVIVAL_WORLD_MULTICHUNK_SESSION_ENV,
                        required_session_env_value(ENV_SOURCE_CLIENT_SCENARIO, Some(client_index))?,
                    );
            }
            ScenarioBehaviorKind::SurvivalContainerBlockEntityBreadth => {
                cmd.env(
                    SURVIVAL_CONTAINER_BLOCK_ENTITY_PROBE_ENV,
                    PROBE_ENABLED_VALUE,
                );
            }
            ScenarioBehaviorKind::SurvivalBiomeDimensionState => {
                cmd.env(SURVIVAL_BIOME_DIMENSION_PROBE_ENV, PROBE_ENABLED_VALUE);
            }
            ScenarioBehaviorKind::SurvivalBiomeDimensionTravel => {
                cmd.env(
                    SURVIVAL_BIOME_DIMENSION_TRAVEL_PROBE_ENV,
                    PROBE_ENABLED_VALUE,
                );
            }
            ScenarioBehaviorKind::SurvivalSignEditingLive => {
                cmd.env(SURVIVAL_SIGN_EDITING_PROBE_ENV, PROBE_ENABLED_VALUE);
            }
            ScenarioBehaviorKind::McpControlledSmoke => {
                cmd.env("MC_COMPAT_MCP_CONTROLLED_SMOKE", PROBE_ENABLED_VALUE);
            }
            ScenarioBehaviorKind::EquipmentUpdate => {
                let team = indexed_team(client_index);
                cmd.env("MC_COMPAT_ACTIVE_PROBE", PROBE_ENABLED_VALUE)
                    .env("MC_COMPAT_TEAM_PROBE", PROBE_ENABLED_VALUE)
                    .env("MC_COMPAT_TEAM_PROBE_TEAM", team)
                    .env("MC_COMPAT_EQUIPMENT_PROBE", PROBE_ENABLED_VALUE);
            }
            ScenarioBehaviorKind::Projectile { .. } => {
                let (team, role) = indexed_combat_team_role(client_index);
                cmd.env("MC_COMPAT_ACTIVE_PROBE", PROBE_ENABLED_VALUE)
                    .env("MC_COMPAT_TEAM_PROBE", PROBE_ENABLED_VALUE)
                    .env("MC_COMPAT_TEAM_PROBE_TEAM", team)
                    .env("MC_COMPAT_COMBAT_PROBE", PROBE_ENABLED_VALUE)
                    .env("MC_COMPAT_COMBAT_PROBE_ROLE", role)
                    .env("MC_COMPAT_PROJECTILE_PROBE", PROBE_ENABLED_VALUE);
                if role == COMBAT_ATTACKER_ROLE {
                    cmd.env("MC_COMPAT_COMBAT_TARGET_USERNAME", COMBAT_TARGET_USERNAME);
                }
            }
            ScenarioBehaviorKind::Combat {
                reference_probe,
                armor_reference,
                armor_mitigation,
                flag_carrier_death,
                ..
            } => {
                let (team, role) = indexed_combat_team_role(client_index);
                cmd.env("MC_COMPAT_ACTIVE_PROBE", PROBE_ENABLED_VALUE)
                    .env("MC_COMPAT_COMBAT_PROBE", PROBE_ENABLED_VALUE)
                    .env("MC_COMPAT_COMBAT_PROBE_ROLE", role);
                if !reference_probe {
                    cmd.env("MC_COMPAT_TEAM_PROBE", PROBE_ENABLED_VALUE)
                        .env("MC_COMPAT_TEAM_PROBE_TEAM", team);
                }
                if role == COMBAT_ATTACKER_ROLE {
                    cmd.env("MC_COMPAT_COMBAT_TARGET_USERNAME", COMBAT_TARGET_USERNAME);
                }
                if *reference_probe {
                    cmd.env(VANILLA_COMBAT_REFERENCE_PROBE_ENV, PROBE_ENABLED_VALUE)
                        .env("MC_COMPAT_STATIONARY_COMBAT_PROBE", PROBE_ENABLED_VALUE);
                }
                if *armor_reference {
                    cmd.env(
                        VANILLA_COMBAT_ARMOR_REFERENCE_PROBE_ENV,
                        PROBE_ENABLED_VALUE,
                    );
                }
                if *armor_mitigation {
                    cmd.env("MC_COMPAT_ARMOR_MITIGATION_PROBE", PROBE_ENABLED_VALUE);
                    if role == COMBAT_VICTIM_ROLE {
                        cmd.env("MC_COMPAT_INVENTORY_PROBE", PROBE_ENABLED_VALUE);
                    }
                }
                if *flag_carrier_death {
                    cmd.env("MC_COMPAT_FLAG_CARRIER_DEATH_PROBE", PROBE_ENABLED_VALUE)
                        .env("MC_COMPAT_RESPAWN_PROBE", PROBE_ENABLED_VALUE);
                    if client_index == SECOND_CLIENT_INDEX {
                        cmd.env("MC_COMPAT_FLAG_PROBE", PROBE_ENABLED_VALUE)
                            .env("MC_COMPAT_FLAG_PROBE_TEAM", TEAM_BLUE_VALUE)
                            .env("MC_COMPAT_FLAG_PROBE_PICKUP_ONLY", PROBE_ENABLED_VALUE)
                            .env(
                                "MC_COMPAT_FLAG_PROBE_FIRST_TICK",
                                FLAG_CARRIER_DEATH_PICKUP_FIRST_TICK.to_string(),
                            )
                            .env("MC_COMPAT_FLAG_PROBE_REPEAT", PROBE_REPEAT_SINGLE);
                    }
                }
            }
            ScenarioBehaviorKind::MultiClientLoadScore => {
                cmd.env("MC_COMPAT_ACTIVE_PROBE", PROBE_ENABLED_VALUE);
                if client_index == FIRST_CLIENT_INDEX {
                    cmd.env("MC_COMPAT_TEAM_PROBE", PROBE_ENABLED_VALUE)
                        .env("MC_COMPAT_TEAM_PROBE_TEAM", TEAM_RED_VALUE)
                        .env("MC_COMPAT_FLAG_PROBE", PROBE_ENABLED_VALUE)
                        .env("MC_COMPAT_FLAG_PROBE_REPEAT", PROBE_REPEAT_SINGLE);
                }
            }
            ScenarioBehaviorKind::NegativeCtfWrongScore => {
                cmd.env("MC_COMPAT_ACTIVE_PROBE", PROBE_ENABLED_VALUE)
                    .env("MC_COMPAT_TEAM_PROBE", PROBE_ENABLED_VALUE)
                    .env("MC_COMPAT_TEAM_PROBE_TEAM", TEAM_RED_VALUE)
                    .env("MC_COMPAT_FLAG_PROBE", PROBE_ENABLED_VALUE)
                    .env("MC_COMPAT_FLAG_PROBE_TEAM", TEAM_BLUE_VALUE)
                    .env("MC_COMPAT_FLAG_PROBE_PICKUP_ONLY", PROBE_ENABLED_VALUE)
                    .env("MC_COMPAT_NEGATIVE_PROBE", "ctf_wrong_score");
            }
            ScenarioBehaviorKind::CtfInvalidPickupOwnership => {
                cmd.env("MC_COMPAT_ACTIVE_PROBE", PROBE_ENABLED_VALUE)
                    .env("MC_COMPAT_TEAM_PROBE", PROBE_ENABLED_VALUE)
                    .env("MC_COMPAT_TEAM_PROBE_TEAM", TEAM_RED_VALUE)
                    .env("MC_COMPAT_FLAG_PROBE", PROBE_ENABLED_VALUE)
                    .env("MC_COMPAT_FLAG_PROBE_TEAM", TEAM_BLUE_VALUE)
                    .env("MC_COMPAT_FLAG_PROBE_PICKUP_ONLY", PROBE_ENABLED_VALUE)
                    .env("MC_COMPAT_NEGATIVE_PROBE", "ctf_invalid_pickup_ownership");
            }
            ScenarioBehaviorKind::CtfInvalidReturnDrop => {
                cmd.env("MC_COMPAT_ACTIVE_PROBE", PROBE_ENABLED_VALUE)
                    .env("MC_COMPAT_TEAM_PROBE", PROBE_ENABLED_VALUE)
                    .env("MC_COMPAT_TEAM_PROBE_TEAM", TEAM_RED_VALUE)
                    .env("MC_COMPAT_FLAG_PROBE", PROBE_ENABLED_VALUE)
                    .env("MC_COMPAT_FLAG_PROBE_TEAM", TEAM_BLUE_VALUE)
                    .env("MC_COMPAT_FLAG_PROBE_PICKUP_ONLY", PROBE_ENABLED_VALUE)
                    .env("MC_COMPAT_NEGATIVE_PROBE", "ctf_invalid_return_drop");
            }
            ScenarioBehaviorKind::CtfInvalidOpponentBaseReturnDrop => {
                cmd.env("MC_COMPAT_ACTIVE_PROBE", PROBE_ENABLED_VALUE)
                    .env("MC_COMPAT_TEAM_PROBE", PROBE_ENABLED_VALUE)
                    .env("MC_COMPAT_TEAM_PROBE_TEAM", TEAM_RED_VALUE)
                    .env("MC_COMPAT_FLAG_PROBE", PROBE_ENABLED_VALUE)
                    .env("MC_COMPAT_FLAG_PROBE_TEAM", TEAM_RED_VALUE)
                    .env("MC_COMPAT_FLAG_PROBE_PICKUP_ONLY", PROBE_ENABLED_VALUE)
                    .env(
                        "MC_COMPAT_NEGATIVE_PROBE",
                        "ctf_invalid_opponent_base_return_drop",
                    );
            }
            ScenarioBehaviorKind::CtfScoreLimitWinCondition => {
                cmd.env("MC_COMPAT_ACTIVE_PROBE", PROBE_ENABLED_VALUE)
                    .env("MC_COMPAT_TEAM_PROBE", PROBE_ENABLED_VALUE)
                    .env("MC_COMPAT_TEAM_PROBE_TEAM", TEAM_RED_VALUE)
                    .env("MC_COMPAT_FLAG_PROBE", PROBE_ENABLED_VALUE)
                    .env("MC_COMPAT_FLAG_PROBE_TEAM", TEAM_RED_VALUE)
                    .env("MC_COMPAT_FLAG_PROBE_REPEAT", PROBE_REPEAT_SINGLE)
                    .env("MC_COMPAT_SCORE_LIMIT_PROBE", PROBE_ENABLED_VALUE);
            }
            ScenarioBehaviorKind::CtfSimultaneousPickupCaptureRace => {
                let first_tick = if client_index == FIRST_CLIENT_INDEX {
                    CTF_RACE_REJECT_CLIENT_FIRST_TICK
                } else {
                    CTF_RACE_ACCEPT_CLIENT_FIRST_TICK
                };
                cmd.env("MC_COMPAT_ACTIVE_PROBE", PROBE_ENABLED_VALUE)
                    .env("MC_COMPAT_TEAM_PROBE", PROBE_ENABLED_VALUE)
                    .env("MC_COMPAT_TEAM_PROBE_TEAM", TEAM_RED_VALUE)
                    .env("MC_COMPAT_FLAG_PROBE", PROBE_ENABLED_VALUE)
                    .env("MC_COMPAT_FLAG_PROBE_TEAM", TEAM_RED_VALUE)
                    .env("MC_COMPAT_FLAG_PROBE_REPEAT", PROBE_REPEAT_SINGLE)
                    .env("MC_COMPAT_FLAG_PROBE_FIRST_TICK", first_tick.to_string());
            }
            ScenarioBehaviorKind::CtfSpawnTeamBalanceReset => {
                let team = indexed_team(client_index);
                cmd.env("MC_COMPAT_ACTIVE_PROBE", PROBE_ENABLED_VALUE)
                    .env("MC_COMPAT_TEAM_PROBE", PROBE_ENABLED_VALUE)
                    .env("MC_COMPAT_TEAM_PROBE_TEAM", team);
                if client_index == FIRST_CLIENT_INDEX {
                    cmd.env("MC_COMPAT_FLAG_PROBE", PROBE_ENABLED_VALUE)
                        .env("MC_COMPAT_FLAG_PROBE_TEAM", TEAM_RED_VALUE)
                        .env("MC_COMPAT_FLAG_PROBE_REPEAT", PROBE_REPEAT_SINGLE);
                }
            }
        }
        cmd.finish()
    }

    fn valence_server_env_patch(&self, cfg: &Config) -> Result<EnvPatch, String> {
        let mut cmd = EnvPatchBuilder::new(ENV_SOURCE_VALENCE_SCENARIO);
        if self.uses_armor_mitigation_probe() {
            cmd.env("MC_COMPAT_ARMOR_MITIGATION_PROBE", PROBE_ENABLED_VALUE);
        }
        match self {
            ScenarioBehaviorKind::EquipmentUpdate => {
                cmd.env("MC_COMPAT_EQUIPMENT_UPDATE_PROBE", PROBE_ENABLED_VALUE);
            }
            ScenarioBehaviorKind::Projectile { .. } => {
                cmd.env("MC_COMPAT_PROJECTILE_PROBE", PROBE_ENABLED_VALUE);
            }
            ScenarioBehaviorKind::Combat {
                reference_probe,
                armor_reference,
                ..
            } => {
                if *reference_probe {
                    cmd.env(VANILLA_COMBAT_REFERENCE_PROBE_ENV, PROBE_ENABLED_VALUE);
                }
                if *armor_reference {
                    cmd.env(
                        VANILLA_COMBAT_ARMOR_REFERENCE_PROBE_ENV,
                        PROBE_ENABLED_VALUE,
                    );
                }
            }
            ScenarioBehaviorKind::InventoryStackSplitMerge => {
                cmd.env(INVENTORY_STACK_SPLIT_MERGE_PROBE_ENV, PROBE_ENABLED_VALUE);
            }
            ScenarioBehaviorKind::InventoryDragTransactions => {
                cmd.env(INVENTORY_DRAG_TRANSACTIONS_PROBE_ENV, PROBE_ENABLED_VALUE);
            }
            ScenarioBehaviorKind::SurvivalChestPersistence => {
                cmd.env(SURVIVAL_CHEST_FIXTURE_ENV, PROBE_ENABLED_VALUE);
            }
            ScenarioBehaviorKind::SurvivalCraftingTable => {
                cmd.env(SURVIVAL_CRAFTING_FIXTURE_ENV, PROBE_ENABLED_VALUE);
            }
            ScenarioBehaviorKind::SurvivalCraftingRecipeBreadth => {
                cmd.env(SURVIVAL_CRAFTING_BREADTH_FIXTURE_ENV, PROBE_ENABLED_VALUE);
            }
            ScenarioBehaviorKind::SurvivalFurnacePersistence => {
                cmd.env(SURVIVAL_FURNACE_FIXTURE_ENV, PROBE_ENABLED_VALUE);
            }
            ScenarioBehaviorKind::SurvivalFurnaceSmeltingBreadth => {
                cmd.env(SURVIVAL_FURNACE_FIXTURE_ENV, PROBE_ENABLED_VALUE)
                    .env(
                        SURVIVAL_FURNACE_SMELTING_BREADTH_FIXTURE_ENV,
                        PROBE_ENABLED_VALUE,
                    );
            }
            ScenarioBehaviorKind::SurvivalHungerFood => {
                cmd.env(SURVIVAL_HUNGER_FOOD_FIXTURE_ENV, PROBE_ENABLED_VALUE);
            }
            ScenarioBehaviorKind::SurvivalHungerHealthCycle => {
                cmd.env(SURVIVAL_HUNGER_HEALTH_FIXTURE_ENV, PROBE_ENABLED_VALUE);
            }
            ScenarioBehaviorKind::SurvivalMobDrop => {
                cmd.env(SURVIVAL_MOB_DROP_FIXTURE_ENV, PROBE_ENABLED_VALUE);
            }
            ScenarioBehaviorKind::SurvivalMobAiLootBreadth => {
                cmd.env(SURVIVAL_MOB_AI_LOOT_FIXTURE_ENV, PROBE_ENABLED_VALUE);
            }
            ScenarioBehaviorKind::SurvivalRedstoneToggle => {
                cmd.env(SURVIVAL_REDSTONE_TOGGLE_FIXTURE_ENV, PROBE_ENABLED_VALUE);
            }
            ScenarioBehaviorKind::SurvivalRedstoneCircuitBreadth => {
                cmd.env(SURVIVAL_REDSTONE_CIRCUIT_FIXTURE_ENV, PROBE_ENABLED_VALUE);
            }
            ScenarioBehaviorKind::WorldPersistenceRestart { block_entity, .. } => {
                if *block_entity {
                    cmd.env(SURVIVAL_BLOCK_ENTITY_FIXTURE_ENV, PROBE_ENABLED_VALUE)
                        .env(
                            SURVIVAL_BLOCK_ENTITY_DIR_ENV,
                            world_persistence_state_dir(cfg, ServerBackend::Valence),
                        )
                        .env(
                            SURVIVAL_BLOCK_ENTITY_PHASE_ENV,
                            world_persistence_phase_value(cfg),
                        );
                } else {
                    cmd.env(SURVIVAL_WORLD_PERSISTENCE_FIXTURE_ENV, PROBE_ENABLED_VALUE)
                        .env(
                            SURVIVAL_WORLD_PERSISTENCE_DIR_ENV,
                            world_persistence_state_dir(cfg, ServerBackend::Valence),
                        )
                        .env(
                            SURVIVAL_WORLD_PERSISTENCE_PHASE_ENV,
                            world_persistence_phase_value(cfg),
                        );
                }
            }
            ScenarioBehaviorKind::SurvivalWorldMultichunkDurability => {
                cmd.env(SURVIVAL_WORLD_MULTICHUNK_FIXTURE_ENV, PROBE_ENABLED_VALUE)
                    .env(
                        SURVIVAL_WORLD_MULTICHUNK_DIR_ENV,
                        world_persistence_state_dir(cfg, ServerBackend::Valence),
                    )
                    .env(
                        SURVIVAL_WORLD_MULTICHUNK_PHASE_ENV,
                        world_persistence_phase_value(cfg),
                    );
            }
            ScenarioBehaviorKind::SurvivalContainerBlockEntityBreadth => {
                cmd.env(
                    SURVIVAL_CONTAINER_BLOCK_ENTITY_FIXTURE_ENV,
                    PROBE_ENABLED_VALUE,
                );
            }
            ScenarioBehaviorKind::SurvivalBiomeDimensionState => {
                cmd.env(SURVIVAL_BIOME_DIMENSION_FIXTURE_ENV, PROBE_ENABLED_VALUE);
            }
            ScenarioBehaviorKind::SurvivalBiomeDimensionTravel => {
                cmd.env(
                    SURVIVAL_BIOME_DIMENSION_TRAVEL_FIXTURE_ENV,
                    PROBE_ENABLED_VALUE,
                );
            }
            ScenarioBehaviorKind::SurvivalSignEditingLive => {
                cmd.env(SURVIVAL_SIGN_EDITING_FIXTURE_ENV, PROBE_ENABLED_VALUE);
            }
            ScenarioBehaviorKind::CtfInvalidReturnDrop => {
                cmd.env(
                    "MC_COMPAT_CTF_INVALID_RETURN_DROP_PROBE",
                    PROBE_ENABLED_VALUE,
                );
            }
            ScenarioBehaviorKind::CtfInvalidOpponentBaseReturnDrop => {
                cmd.env(
                    "MC_COMPAT_CTF_INVALID_OPPONENT_BASE_RETURN_DROP_PROBE",
                    PROBE_ENABLED_VALUE,
                );
            }
            ScenarioBehaviorKind::CtfScoreLimitWinCondition => {
                cmd.env("MC_COMPAT_CTF_SCORE_LIMIT_PROBE", PROBE_ENABLED_VALUE);
            }
            ScenarioBehaviorKind::CtfSimultaneousPickupCaptureRace => {
                cmd.env("MC_COMPAT_CTF_RACE_PROBE", PROBE_ENABLED_VALUE);
            }
            ScenarioBehaviorKind::CtfSpawnTeamBalanceReset => {
                cmd.env("MC_COMPAT_CTF_SPAWN_TEAM_RESET_PROBE", PROBE_ENABLED_VALUE);
            }
            _ => {}
        }
        cmd.finish()
    }

    fn paper_server_env_patch(&self, cfg: &Config) -> Result<EnvPatch, String> {
        let mut cmd = EnvPatchBuilder::new(ENV_SOURCE_PAPER_SCENARIO);
        match self {
            ScenarioBehaviorKind::SurvivalChestPersistence => {
                add_paper_env(&mut cmd, SURVIVAL_CHEST_FIXTURE_ENV, PROBE_ENABLED_VALUE);
            }
            ScenarioBehaviorKind::SurvivalCraftingTable => {
                add_paper_env(&mut cmd, SURVIVAL_CRAFTING_FIXTURE_ENV, PROBE_ENABLED_VALUE);
            }
            ScenarioBehaviorKind::SurvivalCraftingRecipeBreadth => {
                add_paper_env(
                    &mut cmd,
                    SURVIVAL_CRAFTING_BREADTH_FIXTURE_ENV,
                    PROBE_ENABLED_VALUE,
                );
            }
            ScenarioBehaviorKind::SurvivalFurnacePersistence => {
                add_paper_env(&mut cmd, SURVIVAL_FURNACE_FIXTURE_ENV, PROBE_ENABLED_VALUE);
            }
            ScenarioBehaviorKind::SurvivalFurnaceSmeltingBreadth => {
                add_paper_env(&mut cmd, SURVIVAL_FURNACE_FIXTURE_ENV, PROBE_ENABLED_VALUE);
                add_paper_env(
                    &mut cmd,
                    SURVIVAL_FURNACE_SMELTING_BREADTH_FIXTURE_ENV,
                    PROBE_ENABLED_VALUE,
                );
            }
            ScenarioBehaviorKind::SurvivalHungerFood => {
                add_paper_env(
                    &mut cmd,
                    SURVIVAL_HUNGER_FOOD_FIXTURE_ENV,
                    PROBE_ENABLED_VALUE,
                );
            }
            ScenarioBehaviorKind::SurvivalHungerHealthCycle => {
                add_paper_env(
                    &mut cmd,
                    SURVIVAL_HUNGER_HEALTH_FIXTURE_ENV,
                    PROBE_ENABLED_VALUE,
                );
            }
            ScenarioBehaviorKind::SurvivalMobDrop => {
                add_paper_env(&mut cmd, SURVIVAL_MOB_DROP_FIXTURE_ENV, PROBE_ENABLED_VALUE);
            }
            ScenarioBehaviorKind::SurvivalMobAiLootBreadth => {
                add_paper_env(
                    &mut cmd,
                    SURVIVAL_MOB_AI_LOOT_FIXTURE_ENV,
                    PROBE_ENABLED_VALUE,
                );
            }
            ScenarioBehaviorKind::SurvivalRedstoneToggle => {
                add_paper_env(
                    &mut cmd,
                    SURVIVAL_REDSTONE_TOGGLE_FIXTURE_ENV,
                    PROBE_ENABLED_VALUE,
                );
            }
            ScenarioBehaviorKind::SurvivalRedstoneCircuitBreadth => {
                add_paper_env(
                    &mut cmd,
                    SURVIVAL_REDSTONE_CIRCUIT_FIXTURE_ENV,
                    PROBE_ENABLED_VALUE,
                );
            }
            ScenarioBehaviorKind::WorldPersistenceRestart { block_entity, .. } => {
                if *block_entity {
                    add_paper_persistence_env_patch(
                        &mut cmd,
                        cfg,
                        SURVIVAL_BLOCK_ENTITY_FIXTURE_ENV,
                        SURVIVAL_BLOCK_ENTITY_PHASE_ENV,
                    );
                } else {
                    add_paper_persistence_env_patch(
                        &mut cmd,
                        cfg,
                        SURVIVAL_WORLD_PERSISTENCE_FIXTURE_ENV,
                        SURVIVAL_WORLD_PERSISTENCE_PHASE_ENV,
                    );
                }
            }
            ScenarioBehaviorKind::SurvivalWorldMultichunkDurability => {
                add_paper_persistence_env_patch(
                    &mut cmd,
                    cfg,
                    SURVIVAL_WORLD_MULTICHUNK_FIXTURE_ENV,
                    SURVIVAL_WORLD_MULTICHUNK_PHASE_ENV,
                );
            }
            ScenarioBehaviorKind::SurvivalContainerBlockEntityBreadth => {
                add_paper_env(
                    &mut cmd,
                    SURVIVAL_CONTAINER_BLOCK_ENTITY_FIXTURE_ENV,
                    PROBE_ENABLED_VALUE,
                );
            }
            ScenarioBehaviorKind::SurvivalBiomeDimensionState => {
                add_paper_env(
                    &mut cmd,
                    SURVIVAL_BIOME_DIMENSION_FIXTURE_ENV,
                    PROBE_ENABLED_VALUE,
                );
            }
            ScenarioBehaviorKind::SurvivalBiomeDimensionTravel => {
                add_paper_env(
                    &mut cmd,
                    SURVIVAL_BIOME_DIMENSION_TRAVEL_FIXTURE_ENV,
                    PROBE_ENABLED_VALUE,
                );
            }
            ScenarioBehaviorKind::SurvivalSignEditingLive => {
                add_paper_env(
                    &mut cmd,
                    SURVIVAL_SIGN_EDITING_FIXTURE_ENV,
                    PROBE_ENABLED_VALUE,
                );
            }
            ScenarioBehaviorKind::Combat {
                reference_probe,
                armor_reference,
                ..
            } => {
                if *reference_probe {
                    add_paper_env(
                        &mut cmd,
                        VANILLA_COMBAT_REFERENCE_PROBE_ENV,
                        PROBE_ENABLED_VALUE,
                    );
                }
                if *armor_reference {
                    add_paper_env(
                        &mut cmd,
                        VANILLA_COMBAT_ARMOR_REFERENCE_PROBE_ENV,
                        PROBE_ENABLED_VALUE,
                    );
                }
            }
            _ => {}
        }
        cmd.finish()
    }
}

fn append_count_marker(output: &mut String, marker: &'static str) {
    output.push_str(marker);
    output.push('\n');
}

fn required_session_env_value(source: &str, client_index: Option<usize>) -> Result<String, String> {
    client_index
        .map(session_env_value)
        .ok_or_else(|| format!("missing required session value for {source}"))
}

fn session_env_value(client_index: usize) -> String {
    (client_index + SESSION_INDEX_ENV_OFFSET).to_string()
}

fn indexed_team(client_index: usize) -> &'static str {
    if client_index == FIRST_CLIENT_INDEX {
        TEAM_RED_VALUE
    } else {
        TEAM_BLUE_VALUE
    }
}

fn indexed_combat_team_role(client_index: usize) -> (&'static str, &'static str) {
    if client_index == FIRST_CLIENT_INDEX {
        (TEAM_RED_VALUE, COMBAT_ATTACKER_ROLE)
    } else {
        (TEAM_BLUE_VALUE, COMBAT_VICTIM_ROLE)
    }
}

fn add_paper_env(cmd: &mut EnvPatchBuilder, key: &'static str, value: impl EnvPatchValue) {
    cmd.env(key, value);
}

fn add_paper_persistence_env_patch(
    cmd: &mut EnvPatchBuilder,
    cfg: &Config,
    fixture_env: &'static str,
    phase_env: &'static str,
) {
    add_paper_env(cmd, fixture_env, PROBE_ENABLED_VALUE);
    add_paper_env(cmd, phase_env, world_persistence_phase_value(cfg));
}

fn add_paper_persistence_mount_if_needed(cfg: &Config, cmd: &mut Command) -> Result<(), String> {
    if !scenario_behavior(cfg.scenario).uses_isolated_restart_storage() {
        return Ok(());
    }
    let state_dir = world_persistence_state_dir(cfg, ServerBackend::Paper);
    fs::create_dir_all(&state_dir).map_err(|e| format!("create {}: {e}", state_dir.display()))?;
    let absolute_state_dir = fs::canonicalize(&state_dir)
        .map_err(|e| format!("canonicalize {}: {e}", state_dir.display()))?;
    cmd.arg("-v")
        .arg(format!("{}:/data", absolute_state_dir.display()));
    Ok(())
}

const PROJECTILE_DAMAGE_UPDATE_MILESTONE: &str = "projectile_damage_update";
const CLIENT_A_SUFFIX: &str = "a";
const CLIENT_B_SUFFIX: &str = "b";
const FLAG_OR_SCORE_NEEDLES: &[&str] = &["flag", "score"];

struct EvidenceCorpus<'a> {
    text: &'a str,
    normalized: String,
}

impl<'a> EvidenceCorpus<'a> {
    fn new(text: &'a str) -> Self {
        Self {
            text,
            normalized: text.to_lowercase(),
        }
    }
}

struct EvidenceContext<'a> {
    username: &'a str,
}

#[derive(Clone, Copy)]
struct MilestoneRule<'a> {
    id: &'static str,
    matcher: MatcherKind<'a>,
}

#[derive(Clone, Copy)]
enum MatcherKind<'a> {
    Literal(&'a str),
    CaseInsensitive(&'a str),
    DynamicUsername,
    DynamicClientSuffix(&'static str),
    AnyOfCaseInsensitive(&'static [&'static str]),
}

trait EvidenceMatcher {
    fn is_match(&self, corpus: &EvidenceCorpus<'_>, context: &EvidenceContext<'_>) -> bool;
}

impl EvidenceMatcher for MatcherKind<'_> {
    fn is_match(&self, corpus: &EvidenceCorpus<'_>, context: &EvidenceContext<'_>) -> bool {
        match self {
            MatcherKind::Literal(needle) => corpus.text.contains(needle),
            MatcherKind::CaseInsensitive(needle) => {
                corpus.normalized.contains(&needle.to_lowercase())
            }
            MatcherKind::DynamicUsername => {
                corpus.normalized.contains(&context.username.to_lowercase())
            }
            MatcherKind::DynamicClientSuffix(suffix) => corpus.normalized.contains(&format!(
                "{}{}",
                context.username.to_lowercase(),
                suffix
            )),
            MatcherKind::AnyOfCaseInsensitive(needles) => needles
                .iter()
                .any(|needle| corpus.normalized.contains(&needle.to_lowercase())),
        }
    }
}

fn default_port(backend: ServerBackend) -> u16 {
    backend.runtime().default_port()
}

fn default_arrow_damage_policy() -> runtime_config::ArrowDamagePolicy {
    runtime_config::ArrowDamagePolicy {
        base_damage: DEFAULT_ARROW_DAMAGE,
        velocity_multiplier: DEFAULT_ARROW_VELOCITY_MULTIPLIER,
        max_damage: DEFAULT_ARROW_MAX_DAMAGE,
    }
}

fn projectile_damage_decision(cfg: &Config) -> runtime_config::ArrowDamageDecision {
    runtime_config::evaluate_arrow_damage(
        &cfg.arrow_damage_policy,
        &runtime_config::ProjectileDamageContext {
            projectile_velocity: PROJECTILE_DAMAGE_CONTEXT_VELOCITY,
            pull_strength: PROJECTILE_DAMAGE_CONTEXT_PULL_STRENGTH,
        },
    )
}

fn projectile_damage_amount_text(cfg: &Config) -> String {
    format_one_decimal(projectile_damage_decision(cfg).damage)
}

fn projectile_damage_amount_needle(cfg: &Config) -> String {
    format!("damage={}", projectile_damage_amount_text(cfg))
}

fn projectile_damage_client_health_needle(cfg: &Config) -> String {
    format!(
        "update_health health={}",
        projectile_damage_victim_health_after_text(cfg)
    )
}

fn projectile_damage_server_health_after_needle(cfg: &Config) -> String {
    format!(
        "victim_health_after={}",
        projectile_damage_victim_health_after_text(cfg)
    )
}

fn projectile_damage_victim_health_after_text(cfg: &Config) -> String {
    let after = PROJECTILE_DAMAGE_VICTIM_START_HEALTH - projectile_damage_decision(cfg).damage;
    format_one_decimal(after.max(0.0))
}

fn format_one_decimal(value: f64) -> String {
    format!("{value:.1}")
}

fn print_usage(cfg: &Config) {
    println!(
        "Usage: mc-compat-runner [--config PATH] [--steel-config PATH] [--dry-run|--run|--run-matrix] [--build-client] [--status-only] [--status] [--cleanup [--dry-run|--apply]] [--stop] [--compare-receipts PAPER_RECEIPT VALENCE_RECEIPT] [--scenario {}] [--keep-server] [--server-backend valence|paper] [--client-dir PATH] [--receipt PATH] [--receipt-dir DIR] [--failure-bundle PATH] [--timeout SECS] [--valence-repo PATH] [--valence-rev REV]\n  mc-compat-runner scenario run <scenario> [--dry-run|--run|--live] [--backend valence|paper] [--receipt PATH] [--timeout SECS] [--packet-capture-summary] [--proxy-route ROUTE] [--proxy-forwarding-mode MODE] [runner options...]\n\n\
Automates a local Stevenarella compatibility smoke against a Minecraft {} / protocol {} server.\n\
Default client source is the core Stevenarella client tree resolved from ./clients/stevenarella; pass --client-dir/CLIENT_DIR to use another source tree.\n\
Pass --config/MC_COMPAT_CONFIG a JSON file exported from legacy Nickel config, or --steel-config/MC_COMPAT_STEEL_CONFIG a restricted Steel module; env vars and later CLI flags override either config source.\n\
Pass --receipt/SMOKE_RECEIPT to write a machine-readable mc.compat.scenario.receipt.v2 JSON receipt for Cairn/Octet evidence flows. Pass --failure-bundle/MC_COMPAT_FAILURE_BUNDLE with a docs/evidence path to write a fail-only diagnostic bundle after failed runs.
Use `scenario run <scenario>` for the typed router form; it normalizes scenario, backend, dry-run/live mode, receipt, timeout, and evidence options before launching processes or writing artifacts, and it rejects broad compatibility, production, or semantic-equivalence claim flags.
Use --scenario valence-compat-bot-probe for a bounded one-client Valence probe with status/login/render milestones and safe non-load receipt fields. Use --scenario flag-score-repeat to require explicit protocol/login/render/team/flag/two-score milestones and forbidden-pattern checks. Use --scenario blue-flag-score to exercise the mirrored BLUE-team flag path. Use --scenario survival-break-place-pickup for the bounded survival fixture. Use --scenario survival-chest-persistence for the two-session chest open/store/close/reconnect/reopen probe. Use --scenario survival-crafting-table for one crafting-table open/input/result/collect rail. Use --scenario survival-crafting-recipe-breadth for one bounded shaped/shapeless/invalid recipe breadth rail. Use --scenario survival-furnace-persistence for one furnace input/fuel/output/reconnect rail. Use --scenario survival-furnace-smelting-breadth for one bounded raw-iron/coal smelt plus invalid-fuel rejection rail. Use --scenario survival-hunger-food for one hunger deficit, food consume, and inventory decrement rail. Use --scenario survival-hunger-health-cycle for the isolated bounded health-cycle row using explicit food, saturation, health recovery, and inventory checkpoints. Use --scenario survival-mob-drop for one configured mob kill, drop, pickup, and inventory increment rail. Use --scenario survival-redstone-toggle for one configured control on/off output update rail. Use --scenario survival-world-persistence-restart for one configured block mutation, controlled reload, reconnect, and post-reload observation rail. Use --scenario survival-crash-recovery-parity for one configured block mutation, forced backend stop, crash-recovery restart, reconnect, and post-crash observation rail. Use --scenario survival-block-entity-persistence-parity for one configured sign block entity, controlled reload, reconnect, and post-reload sign text observation rail. Use --scenario survival-biome-dimension-state for one client-observed dimension/world identifier rail. Use --scenario mcp-controlled-smoke for deterministic MCP receipt/checker dry-run evidence before live client driving. Use --scenario vanilla-combat-armor-reference-parity for one Paper/Valence diamond-chestplate combat reference row. Use --scenario reconnect-flag-state to require disconnect/return state coherence while holding a flag. Use --scenario ctf-invalid-pickup-ownership for one contained own-flag pickup attempt with server rejection evidence. Use --scenario ctf-invalid-return-drop for one contained own-base return/drop attempt with server rejection evidence. Use --scenario ctf-invalid-opponent-base-return-drop for one contained opponent-base return/drop attempt with server rejection evidence. Use --scenario ctf-score-limit-win-condition for one near-limit capture that emits exactly one win/end milestone. Use --scenario ctf-simultaneous-pickup-capture-race for one bounded two-client same-flag race with one accepted transition and one rejected duplicate pickup. Use --scenario ctf-spawn-team-balance-reset for one bounded two-client team assignment, spawn/resource, and post-score reset row. Use --scenario reconnect-flag-score to add reconnect evidence; use --scenario multi-client-load-score for two concurrent clients plus server-side correlation.\n\
Use --expect-status-description/--expect-status-version/--expect-status-sample to assert status response fixture data, --packet-capture-summary for redacted capture summary metadata, and --proxy-route/--proxy-forwarding-mode for proxied-route receipt fields.\n\
Use --compare-receipts PAPER_RECEIPT VALENCE_RECEIPT to check the fallback/control and default-backend receipts agree on protocol and headless isolation.\n\
Use --run-matrix --receipt-dir DIR to run Paper and Valence receipts then compare them; add --dry-run after --run-matrix for a non-side-effecting matrix fixture.\n\
Use --status to inspect harness-owned Paper/Valence/tmp state; use --cleanup --dry-run to preview cleanup and --cleanup --apply to remove it.\n\
Default server backend is Valence, using the resolved core Valence server tree plus an isolated worktree when a pinned revision is requested so the current source tree is untouched.\n\
If the Stevenarella or Valence source tree is missing, restore the core component tree or pass --client-dir/CLIENT_DIR and --valence-repo/VALENCE_REPO to alternate source trees.\n\
Client runs are forced through Xvfb/X11 with software GL and no inherited Wayland socket.\n\
Paper fallback runs set EULA=TRUE based on recorded user acceptance.\n\n\
Env: MC_COMPAT_ROOT={} MC_COMPAT_CONFIG={} MC_COMPAT_STEEL_CONFIG={} MC_COMPAT_SCENARIO={} CLIENT_DIR={} TARGET_DIR={} SMOKE_RECEIPT={} SMOKE_RECEIPT_DIR={} MC_COMPAT_FAILURE_BUNDLE={} VALENCE_REPO={} VALENCE_REV={} VALENCE_WORKTREE={} VALENCE_TARGET_DIR={} CLIENT_TIMEOUT={} PAPER_PLUGIN_JAR={}\n",
        SUPPORTED_SCENARIO_USAGE,
        cfg.server_version,
        cfg.server_protocol,
        cfg.root.display(),
        cfg.config_path
            .as_ref()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|| "<unset>".to_string()),
        cfg.steel_config_path
            .as_ref()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|| "<unset>".to_string()),
        scenario_name(cfg.scenario),
        cfg.client_dir.display(),
        cfg.target_dir.display(),
        cfg.receipt_path
            .as_ref()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|| "<unset>".to_string()),
        cfg.receipt_dir
            .as_ref()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|| "<unset>".to_string()),
        cfg.failure_bundle_path
            .as_ref()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|| "<unset>".to_string()),
        cfg.valence_repo.display(),
        cfg.valence_rev,
        cfg.valence_worktree.display(),
        cfg.valence_target_dir.display(),
        cfg.client_timeout.as_secs(),
        cfg.paper_plugin_jar
            .as_ref()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|| "<unset>".to_string())
    );
}

fn build_client(cfg: &Config) -> Result<(), String> {
    ensure_client_dir_ready(cfg)?;
    log(format_args!("building Stevenarella client"));
    let mut cmd = Command::new("cargo");
    cmd.current_dir(&cfg.client_dir)
        .arg("build")
        .arg("--bin")
        .arg("stevenarella");
    apply_build_env(&mut cmd, &cfg.target_dir)?;
    run_cmd(cfg, &mut cmd)
}

fn ensure_client_dir_ready(cfg: &Config) -> Result<(), String> {
    if !cfg.client_dir.exists() {
        return Err(format!(
            "Stevenarella source tree not found at {}. Keep the core client tree present at clients/stevenarella or pass --client-dir/CLIENT_DIR to another checkout.",
            cfg.client_dir.display()
        ));
    }

    let manifest = cfg.client_dir.join(CARGO_MANIFEST_FILE);
    if !manifest.exists() {
        return Err(format!(
            "Stevenarella source tree {} is missing Cargo.toml. Point --client-dir/CLIENT_DIR at the Stevenarella source root.",
            cfg.client_dir.display()
        ));
    }

    Ok(())
}

const SCENARIO_RECEIPT_SCHEMA: &str = "mc.compat.scenario.receipt.v2";
const DEFAULT_MATRIX_RECEIPT_DIR: &str = "target/mc-compat-matrix";
const PLAN_CLIENT_LOG_ENV_OR_TEMP: &str = "CLIENT_LOG-or-temp-mc-compat-client-log";
const PLAN_CLIENT_LOG_TEMP: &str = "temp-mc-compat-client-log";
const PLAN_CLIENT_LOG_RECONNECT_TEMP: &str = "temp-mc-compat-reconnect-session-log";
const PLAN_CLEANUP_CLIENT_LOG_DISCOVERY: &str = "discover-/tmp-mc-compat-client-logs";
const PLAN_NON_CLAIM_ARCHITECTURE_ONLY: &str = "architecture_only_no_new_compatibility_claim";
const HARNESS_TEMP_ROOT: &str = "/tmp";
const CLEANUP_ROOT_PATH: &str = "/";
const CLEANUP_MIN_SAFE_COMPONENTS: usize = 2;

#[derive(Debug, Clone, PartialEq, Eq)]
struct PlanningDiagnostic {
    field: String,
    message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct HarnessPlan {
    server: ServerStartupPlan,
    client_sessions: Vec<ClientSessionPlan>,
    receipt: ReceiptOutputPlan,
    artifacts: ArtifactCollectionPlan,
    cleanup: CleanupPlan,
    matrix: Option<MatrixPlan>,
    scenario_route: Option<ScenarioRoutePlan>,
    non_claims: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ServerStartupPlan {
    backend: String,
    protocol: u32,
    port: u16,
    server_name: String,
    keep_server: bool,
    eula_acceptance_required: bool,
    valence_worktree: Option<String>,
    valence_log: Option<String>,
    docker_image: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ClientSessionPlan {
    index: usize,
    username: String,
    timeout_secs: u64,
    scenario: String,
    session_count: usize,
    log_path_strategy: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ReceiptOutputPlan {
    receipt_path: Option<String>,
    receipt_dir: Option<String>,
    failure_bundle_path: Option<String>,
    schema: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ArtifactCollectionPlan {
    typed_event_log_path: Option<String>,
    failure_bundle_path: Option<String>,
    failure_artifact_candidates: Vec<ArtifactCandidatePlan>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ArtifactCandidatePlan {
    kind: String,
    path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CleanupPlan {
    apply: bool,
    paper_container: String,
    valence_pid_file: String,
    path_actions: Vec<CleanupPathPlan>,
    client_log_discovery: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CleanupPathPlan {
    label: String,
    path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct MatrixPlan {
    dry_run: bool,
    matrix_mode: String,
    receipt_dir: String,
    paper_receipt: String,
    valence_receipt: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ScenarioRoutePlan {
    scenario: String,
    backend: String,
    mode: String,
    receipt_path: Option<String>,
    timeout_secs: u64,
    packet_capture_summary: bool,
    proxy_route: Option<String>,
    proxy_forwarding_mode: Option<String>,
    failure_bundle_path: Option<String>,
    non_claims: Vec<String>,
}

fn write_failure_evidence_bundle(
    cfg: &Config,
    result: Result<&Option<ClientRunEvidence>, &String>,
) -> Result<(), String> {
    let Some(path) = &cfg.failure_bundle_path else {
        return Ok(());
    };
    let Err(first_failure) = result else {
        return Ok(());
    };
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        fs::create_dir_all(parent)
            .map_err(|err| format!("create failure bundle dir {}: {err}", parent.display()))?;
    }
    let artifacts = collect_failure_bundle_artifacts(cfg)?;
    let bundle = failure_bundle_from_config(cfg, first_failure, artifacts);
    validate_failure_evidence_bundle(&bundle).map_err(|diagnostics| {
        format!(
            "failure bundle validation failed: {}",
            diagnostics.join("; ")
        )
    })?;
    fs::write(path, render_failure_evidence_bundle_json(&bundle))
        .map_err(|err| format!("write failure bundle {}: {err}", path.display()))?;
    log(format_args!(
        "wrote failure evidence bundle {}",
        path.display()
    ));
    Ok(())
}

fn collect_failure_bundle_artifacts(cfg: &Config) -> Result<Vec<FailureBundleArtifact>, String> {
    let mut artifacts = Vec::new();
    for (kind, path) in failure_bundle_artifact_candidates(cfg) {
        push_failure_bundle_artifact(cfg, &mut artifacts, kind, path)?;
    }
    Ok(artifacts)
}

fn failure_bundle_artifact_candidates(cfg: &Config) -> Vec<(&'static str, PathBuf)> {
    let mut candidates = Vec::new();
    if let Some(receipt_path) = &cfg.receipt_path {
        candidates.push((FAILURE_BUNDLE_ARTIFACT_RECEIPT, receipt_path.clone()));
        candidates.push((
            FAILURE_BUNDLE_ARTIFACT_TYPED_EVENTS,
            typed_event_log_path_for_receipt(receipt_path),
        ));
        candidates.push((
            FAILURE_BUNDLE_ARTIFACT_MCP_TRANSCRIPT,
            receipt_path.with_extension(MCP_CONTROL_LIVE_TRANSCRIPT_EXTENSION),
        ));
        candidates.push((
            FAILURE_BUNDLE_ARTIFACT_STDERR,
            receipt_path.with_extension(MCP_CONTROL_LIVE_STDERR_LOG_EXTENSION),
        ));
    }
    if cfg.server_backend == ServerBackend::Valence {
        candidates.push((FAILURE_BUNDLE_ARTIFACT_SERVER_LOG, cfg.valence_log.clone()));
    }
    candidates
}

fn push_failure_bundle_artifact(
    cfg: &Config,
    artifacts: &mut Vec<FailureBundleArtifact>,
    kind: &'static str,
    path: PathBuf,
) -> Result<(), String> {
    let source_path = failure_bundle_source_path(&cfg.root, &path);
    if !source_path.exists() {
        return Ok(());
    }
    let Some(relative_path) = reviewable_failure_bundle_artifact_path(&cfg.root, &path) else {
        return Ok(());
    };
    if validate_failure_bundle_artifact_path(&relative_path).is_err() {
        return Ok(());
    }
    if artifacts
        .iter()
        .any(|artifact| artifact.path == relative_path)
    {
        return Ok(());
    }
    artifacts.push(FailureBundleArtifact {
        kind: kind.to_string(),
        blake3: blake3_file_hex(&source_path)?,
        path: relative_path,
    });
    Ok(())
}

fn failure_bundle_source_path(root: &Path, path: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        root.join(path)
    }
}

fn reviewable_failure_bundle_artifact_path(root: &Path, path: &Path) -> Option<String> {
    if path.is_absolute() {
        let canonical_root = root.canonicalize().ok()?;
        let canonical_path = path.canonicalize().ok()?;
        let relative = canonical_path.strip_prefix(canonical_root).ok()?;
        return path_to_forward_slashes(relative);
    }
    path_to_forward_slashes(path)
}

fn path_to_forward_slashes(path: &Path) -> Option<String> {
    let path = path.to_str()?;
    Some(path.replace(std::path::MAIN_SEPARATOR, "/"))
}

fn blake3_file_hex(path: &Path) -> Result<String, String> {
    let mut file = File::open(path).map_err(|err| format!("open {}: {err}", path.display()))?;
    let mut hasher = blake3::Hasher::new();
    let mut buffer = [0u8; FAILURE_BUNDLE_HASH_BUFFER_BYTES];
    loop {
        let bytes_read = file
            .read(&mut buffer)
            .map_err(|err| format!("read {}: {err}", path.display()))?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }
    Ok(hasher.finalize().to_hex().to_string())
}

fn write_smoke_receipt(
    cfg: &Config,
    result: Result<&Option<ClientRunEvidence>, &String>,
) -> Result<(), String> {
    let Some(path) = &cfg.receipt_path else {
        return Ok(());
    };
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        fs::create_dir_all(parent)
            .map_err(|e| format!("create receipt dir {}: {e}", parent.display()))?;
    }
    let client = match result {
        Ok(client) => client.as_ref(),
        Err(_) => None,
    };
    let typed_event_oracle = write_typed_event_oracle_artifact(cfg, client, path)?;
    let json = smoke_receipt_json_with_typed_event_oracle(
        cfg,
        result.map_err(|err| err.as_str()),
        typed_event_oracle.as_ref(),
    );
    fs::write(path, json).map_err(|e| format!("write receipt {}: {e}", path.display()))?;
    log(format_args!("wrote smoke receipt {}", path.display()));
    Ok(())
}

fn write_typed_event_oracle_artifact(
    cfg: &Config,
    client: Option<&ClientRunEvidence>,
    receipt_path: &Path,
) -> Result<Option<TypedEventOracleArtifact>, String> {
    if cfg.mode != Mode::Run {
        return Ok(None);
    }
    let Some(client) = client else {
        return Ok(None);
    };
    let events = typed_events_from_receipt_evidence(cfg, client)?;
    if events.is_empty() {
        return Ok(None);
    }
    let timeline = normalize_typed_event_timeline(&events);
    let timeline_blake3 = typed_event_timeline_blake3(&timeline);
    let event_log_path = typed_event_log_path_for_receipt(receipt_path);
    fs::write(&event_log_path, timeline)
        .map_err(|err| format!("write typed event log {}: {err}", event_log_path.display()))?;
    log(format_args!(
        "wrote typed event log {}",
        event_log_path.display()
    ));
    Ok(Some(TypedEventOracleArtifact {
        event_log_path,
        timeline_blake3,
        event_count: events.len(),
        contributes_to_pass_fail: typed_event_oracle_contributes_to_pass_fail(cfg.scenario),
    }))
}

fn typed_event_log_path_for_receipt(receipt_path: &Path) -> PathBuf {
    receipt_path.with_extension(TYPED_EVENT_LOG_EXTENSION)
}

const LATENCY_JITTER_ENABLED_ENV: &str = "MC_COMPAT_LATENCY_JITTER_ENABLED";
const LATENCY_JITTER_TARGET_RAIL_ENV: &str = "MC_COMPAT_LATENCY_JITTER_TARGET_RAIL";
const LATENCY_JITTER_MECHANISM_ENV: &str = "MC_COMPAT_LATENCY_JITTER_MECHANISM";
const LATENCY_MS_ENV: &str = "MC_COMPAT_LATENCY_MS";
const JITTER_MS_ENV: &str = "MC_COMPAT_JITTER_MS";
const LOSS_PERCENT_ENV: &str = "MC_COMPAT_LOSS_PERCENT";
const WAN_TARGET_OWNERSHIP_ENV: &str = "MC_COMPAT_WAN_TARGET_OWNERSHIP";
const WAN_AUTHORIZATION_ENV: &str = "MC_COMPAT_WAN_AUTHORIZATION";
const LATENCY_JITTER_ENV_ENABLED_VALUE: &str = "1";
const LATENCY_JITTER_DEFAULT_METRIC: &str = "0";
const LATENCY_JITTER_DEFAULT_MECHANISM: &str = "bounded-client-cadence";
const LATENCY_JITTER_ENABLED_HYGIENE_STATUS: &str = "bounded-local-fixture";
const LATENCY_JITTER_DISABLED_HYGIENE_STATUS: &str = "not-selected";
const WAN_TARGET_OWNERSHIP_OWNED_LOCAL: &str = "owned-local-loopback";
const WAN_AUTHORIZATION_OWNED_LOCAL: &str = "owned-local-fixture-approved";
const WAN_PASS_FAIL_CRITERIA: &str = "inventory_interaction_client_server_milestones";
const WAN_ABORT_REASON_NONE: &str = "none";
const NO_RECONNECT_SESSIONS: u32 = 0;
const SINGLE_RECONNECT_SESSION: u32 = 1;
const WAN_TELEMETRY_SAMPLES: &[&str] = &[
    "scenario_required_milestones",
    "scenario_observed_milestones",
    "server_required_milestones",
    "server_observed_milestones",
    "client_classification",
    "triage_boundary",
];

fn latency_jitter_receipt_json(cfg: &Config) -> String {
    let receipt = latency_jitter_receipt_from_config(cfg);
    render_latency_jitter_receipt_json(&receipt)
}

fn latency_jitter_receipt_from_config(cfg: &Config) -> LatencyJitterTelemetryReceipt {
    let selected = std::env::var(LATENCY_JITTER_ENABLED_ENV).unwrap_or_default()
        == LATENCY_JITTER_ENV_ENABLED_VALUE;
    let target_rail = std::env::var(LATENCY_JITTER_TARGET_RAIL_ENV)
        .unwrap_or_else(|_| scenario_name(cfg.scenario).to_string());
    let delay_ms =
        std::env::var(LATENCY_MS_ENV).unwrap_or_else(|_| LATENCY_JITTER_DEFAULT_METRIC.to_string());
    let jitter_ms =
        std::env::var(JITTER_MS_ENV).unwrap_or_else(|_| LATENCY_JITTER_DEFAULT_METRIC.to_string());
    let loss_percent = std::env::var(LOSS_PERCENT_ENV)
        .unwrap_or_else(|_| LATENCY_JITTER_DEFAULT_METRIC.to_string());
    let mechanism = std::env::var(LATENCY_JITTER_MECHANISM_ENV)
        .unwrap_or_else(|_| LATENCY_JITTER_DEFAULT_MECHANISM.to_string());
    let target_ownership = std::env::var(WAN_TARGET_OWNERSHIP_ENV)
        .unwrap_or_else(|_| WAN_TARGET_OWNERSHIP_OWNED_LOCAL.to_string());
    let authorization = std::env::var(WAN_AUTHORIZATION_ENV)
        .unwrap_or_else(|_| WAN_AUTHORIZATION_OWNED_LOCAL.to_string());
    let hygiene_status = if selected {
        LATENCY_JITTER_ENABLED_HYGIENE_STATUS
    } else {
        LATENCY_JITTER_DISABLED_HYGIENE_STATUS
    };
    LatencyJitterTelemetryReceipt {
        selected,
        mechanism,
        target_rail,
        delay_ms,
        jitter_ms,
        loss_percent,
        timeout_secs: cfg.client_timeout.as_secs(),
        duration_secs: cfg.client_timeout.as_secs(),
        client_count: planned_client_usernames(cfg).len(),
        reconnect_count: latency_jitter_reconnect_count(cfg.scenario),
        target_ownership,
        authorization,
        hygiene_status,
    }
}

fn latency_jitter_reconnect_count(scenario: Scenario) -> u32 {
    if matches!(
        scenario,
        Scenario::ReconnectFlagState
            | Scenario::ReconnectFlagScore
            | Scenario::NegativeReconnectRace
    ) {
        SINGLE_RECONNECT_SESSION
    } else {
        NO_RECONNECT_SESSIONS
    }
}

const PUBLIC_SERVER_AUTHORIZED_SAFETY_ENV: &str = "MC_COMPAT_PUBLIC_SERVER_AUTHORIZED_SAFETY";
const PUBLIC_SERVER_TARGET_OWNER_ENV: &str = "MC_COMPAT_PUBLIC_SERVER_TARGET_OWNER";
const PUBLIC_SERVER_AUTHORIZATION_ARTIFACT_ENV: &str =
    "MC_COMPAT_PUBLIC_SERVER_AUTHORIZATION_ARTIFACT";
const PUBLIC_SERVER_TARGET_SCOPE_ENV: &str = "MC_COMPAT_PUBLIC_SERVER_TARGET_SCOPE";
const PUBLIC_SERVER_CHECKPOINT_DECISION_ENV: &str = "MC_COMPAT_PUBLIC_SERVER_CHECKPOINT_DECISION";
const PUBLIC_SERVER_AUTHORIZED_ENV_VALUE: &str = "1";
const PUBLIC_SERVER_DEFAULT_TARGET_OWNER: &str = "review-fixture-owner";
const PUBLIC_SERVER_DEFAULT_TARGET_SCOPE: &str = "authorized-non-loopback-fixture";
const PUBLIC_SERVER_DEFAULT_AUTHORIZATION_ARTIFACT: &str =
    "docs/evidence/protocol-763-public-server-authorized-safety-checkpoint-2026-05-30.md";
const PUBLIC_SERVER_DEFAULT_CHECKPOINT_DECISION: &str = "approved_for_deterministic_fixture_only";
const PUBLIC_SERVER_ABORT_CRITERIA: &str = "missing_authorization_or_bound_violation";
const PUBLIC_SERVER_REDACTION_POLICY: &str = "no_secrets_no_raw_public_address";
const PUBLIC_SERVER_FIXTURE_LIVE_TRAFFIC_ENABLED: bool = false;
const PUBLIC_SERVER_TRAFFIC_LIMITS: &[&str] = &[
    "client_count<=1",
    "duration_secs<=30",
    "status_probe_only",
    "live_traffic_enabled=false",
];
const PUBLIC_SERVER_TELEMETRY_FIELDS: &[&str] = &[
    "target_owner",
    "authorization_artifact",
    "target_scope",
    "client_count",
    "duration_secs",
    "traffic_limits",
    "abort_criteria",
    "redaction_policy",
    "checkpoint_decision",
];

fn public_server_authorized_safety_receipt_json(cfg: &Config) -> String {
    let receipt = public_server_authorized_safety_from_config(cfg);
    render_public_server_authorized_safety_receipt_json(&receipt)
}

fn public_server_authorized_safety_from_config(
    cfg: &Config,
) -> PublicServerAuthorizedSafetyReceipt {
    let requested = std::env::var(PUBLIC_SERVER_AUTHORIZED_SAFETY_ENV).unwrap_or_default()
        == PUBLIC_SERVER_AUTHORIZED_ENV_VALUE;
    let selected = public_server_authorized_safety_selected(requested, cfg.mode);
    let target_owner = std::env::var(PUBLIC_SERVER_TARGET_OWNER_ENV)
        .unwrap_or_else(|_| PUBLIC_SERVER_DEFAULT_TARGET_OWNER.to_string());
    let authorization_artifact = std::env::var(PUBLIC_SERVER_AUTHORIZATION_ARTIFACT_ENV)
        .unwrap_or_else(|_| PUBLIC_SERVER_DEFAULT_AUTHORIZATION_ARTIFACT.to_string());
    let target_scope = std::env::var(PUBLIC_SERVER_TARGET_SCOPE_ENV)
        .unwrap_or_else(|_| PUBLIC_SERVER_DEFAULT_TARGET_SCOPE.to_string());
    let checkpoint_decision = std::env::var(PUBLIC_SERVER_CHECKPOINT_DECISION_ENV)
        .unwrap_or_else(|_| PUBLIC_SERVER_DEFAULT_CHECKPOINT_DECISION.to_string());
    PublicServerAuthorizedSafetyReceipt {
        selected,
        target_owner,
        authorization_artifact,
        target_scope,
        client_count: planned_client_usernames(cfg).len(),
        duration_secs: cfg.client_timeout.as_secs(),
        checkpoint_decision,
        live_traffic_enabled: PUBLIC_SERVER_FIXTURE_LIVE_TRAFFIC_ENABLED,
    }
}

fn public_server_authorized_safety_selected(requested: bool, mode: Mode) -> bool {
    requested && matches!(mode, Mode::DryRun)
}

fn run_matrix(cfg: &Config, plan: &MatrixPlan) -> Result<(), String> {
    let receipt_dir = PathBuf::from(&plan.receipt_dir);
    fs::create_dir_all(&receipt_dir)
        .map_err(|e| format!("create receipt dir {}: {e}", receipt_dir.display()))?;

    let paper_receipt = PathBuf::from(&plan.paper_receipt);
    let valence_receipt = PathBuf::from(&plan.valence_receipt);
    let matrix_mode = plan.matrix_mode.as_str();
    log(format_args!(
        "starting {matrix_mode} matrix: paper receipt={} valence receipt={}",
        paper_receipt.display(),
        valence_receipt.display()
    ));

    let paper_cfg = matrix_backend_config(cfg, ServerBackend::Paper, paper_receipt.clone());
    run_matrix_backend(&paper_cfg)?;

    let valence_cfg = matrix_backend_config(cfg, ServerBackend::Valence, valence_receipt.clone());
    run_matrix_backend(&valence_cfg)?;

    let paper = read_receipt_summary(&paper_receipt)?;
    let valence = read_receipt_summary(&valence_receipt)?;
    validate_receipt_pair(&paper, &valence, cfg.server_protocol)?;
    println!(
        "[mc-compat] matrix passed: paper={} valence={} protocol={} mode={matrix_mode}",
        paper_receipt.display(),
        valence_receipt.display(),
        paper.protocol
    );
    Ok(())
}

fn matrix_backend_config(cfg: &Config, backend: ServerBackend, receipt_path: PathBuf) -> Config {
    let mut backend_cfg = cfg.clone();
    backend_cfg.mode = if cfg.matrix_dry_run {
        Mode::DryRun
    } else {
        Mode::Run
    };
    backend_cfg.server_backend = backend;
    backend_cfg.server_port = default_port(backend);
    backend_cfg.receipt_path = Some(receipt_path);
    backend_cfg.receipt_dir = None;
    backend_cfg.compare_receipts = None;
    backend_cfg.keep_server = false;
    backend_cfg
}

fn run_matrix_backend(cfg: &Config) -> Result<(), String> {
    log(format_args!(
        "matrix backend {} -> {}",
        backend_name(cfg.server_backend),
        cfg.receipt_path
            .as_ref()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|| "<missing-receipt>".to_string())
    ));
    let result = execute(cfg);
    if let Err(receipt_err) = write_smoke_receipt(cfg, result.as_ref()) {
        return match result {
            Ok(_) => Err(receipt_err),
            Err(err) => Err(format!(
                "{err}; additionally failed to write receipt: {receipt_err}"
            )),
        };
    }
    result.map(|_| ())
}

fn compare_receipts(cfg: &Config) -> Result<(), String> {
    let (left, right) = cfg
        .compare_receipts
        .as_ref()
        .ok_or_else(|| "compare-receipts mode requires two receipt paths".to_string())?;
    let left = read_receipt_summary(left)?;
    let right = read_receipt_summary(right)?;
    validate_receipt_pair(&left, &right, cfg.server_protocol)?;
    let paper = if left.backend == "paper" {
        &left
    } else {
        &right
    };
    let valence = if left.backend == "valence" {
        &left
    } else {
        &right
    };
    println!(
        "[mc-compat] receipt comparison passed: paper={} valence={} protocol={} headless=xvfb/x11/software-gl/no-wayland",
        paper.path.display(),
        valence.path.display(),
        paper.protocol
    );
    Ok(())
}

fn read_receipt_summary(path: &Path) -> Result<ReceiptSummary, String> {
    let text =
        fs::read_to_string(path).map_err(|e| format!("read receipt {}: {e}", path.display()))?;
    read_receipt_summary_from_text(path.to_path_buf(), &text)
}

fn mode_name(mode: Mode) -> &'static str {
    match mode {
        Mode::DryRun => "dry-run",
        Mode::Run => "run",
        Mode::RunMatrix => "run-matrix",
        Mode::BuildClient => "build-client",
        Mode::StatusOnly => "status-only",
        Mode::HarnessStatus => "status",
        Mode::Cleanup => "cleanup",
        Mode::Stop => "stop",
        Mode::CompareReceipts => "compare-receipts",
    }
}

fn backend_name(backend: ServerBackend) -> &'static str {
    backend.runtime().name()
}

fn build_env_patch(target_dir: &Path) -> Result<EnvPatch, String> {
    let mut patch = EnvPatchBuilder::new(ENV_SOURCE_BUILD);
    patch
        .env("RUSTC_WRAPPER", "")
        .env("CARGO_TARGET_DIR", target_dir)
        .env("CMAKE_POLICY_VERSION_MINIMUM", "3.5");
    patch.finish()
}

fn valence_build_env_patch(target_dir: &Path) -> Result<EnvPatch, String> {
    let mut patch = EnvPatchBuilder::new(ENV_SOURCE_BUILD);
    patch
        .env("RUSTC_WRAPPER", "")
        .env("CARGO_TARGET_DIR", target_dir);
    patch.finish()
}

fn valence_steel_config_env_patch(path: &Path) -> Result<EnvPatch, String> {
    let mut patch = EnvPatchBuilder::new(ENV_SOURCE_VALENCE_STEEL_CONFIG);
    patch.env("MC_COMPAT_STEEL_CONFIG", path);
    patch.finish()
}

fn paper_base_env_patch(cfg: &Config) -> Result<EnvPatch, String> {
    let mut patch = EnvPatchBuilder::new(ENV_SOURCE_PAPER_BASE);
    patch
        .env("EULA", PAPER_EULA_ACCEPTED_VALUE)
        .env("TYPE", PAPER_SERVER_TYPE)
        .env("VERSION", &cfg.server_version)
        .env("ONLINE_MODE", PAPER_ONLINE_MODE_VALUE)
        .env("MEMORY", PAPER_MEMORY_LIMIT)
        .env("VIEW_DISTANCE", PAPER_VIEW_DISTANCE.to_string())
        .env("SIMULATION_DISTANCE", PAPER_SIMULATION_DISTANCE.to_string());
    patch.finish()
}

fn apply_build_env(cmd: &mut Command, target_dir: &Path) -> Result<(), String> {
    let patch = build_env_patch(target_dir)?;
    apply_env_patch_to_command(cmd, &patch);
    Ok(())
}

fn headless_env_patch() -> Result<EnvPatch, String> {
    let mut patch = EnvPatchBuilder::new(ENV_SOURCE_HEADLESS);
    patch
        .env_remove("WAYLAND_DISPLAY")
        .env_remove("WAYLAND_SOCKET")
        .env_remove("XDG_CURRENT_DESKTOP")
        .env("XDG_SESSION_TYPE", "x11")
        .env("WINIT_UNIX_BACKEND", "x11")
        .env("GDK_BACKEND", "x11")
        .env("SDL_VIDEODRIVER", "x11")
        .env("LIBGL_ALWAYS_SOFTWARE", "1")
        .env("MESA_LOADER_DRIVER_OVERRIDE", "llvmpipe");
    patch.finish()
}

fn apply_headless_env(cmd: &mut Command) -> Result<(), String> {
    let patch = headless_env_patch()?;
    apply_env_patch_to_command(cmd, &patch);
    Ok(())
}

fn run_cmd(cfg: &Config, cmd: &mut Command) -> Result<(), String> {
    if cfg.mode == Mode::DryRun {
        println!("+ {cmd:?}");
        return Ok(());
    }
    let status = cmd.status().map_err(|e| format!("spawn {cmd:?}: {e}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("command {cmd:?} failed with {status}"))
    }
}

fn parse_backend(value: &str) -> Result<ServerBackend, String> {
    match value {
        "valence" => Ok(ServerBackend::Valence),
        "paper" => Ok(ServerBackend::Paper),
        other => Err(format!("unknown server backend: {other}")),
    }
}

fn env_string(name: &str) -> Option<String> {
    env::var(name).ok().filter(|s| !s.is_empty())
}

fn env_path(name: &str) -> Option<PathBuf> {
    env_string(name).map(PathBuf::from)
}

fn temp_client_log_for(label: &str) -> PathBuf {
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    let safe_label: String = label
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '-' {
                ch
            } else {
                '-'
            }
        })
        .collect();
    PathBuf::from(format!("/tmp/mc-compat-client.{safe_label}.{millis}.log"))
}

fn log(args: std::fmt::Arguments<'_>) {
    println!("[mc-compat] {args}");
}

#[cfg(test)]
mod config_colocated_tests;
#[cfg(test)]
mod env_patch_baseline_colocated_tests;
#[cfg(test)]
mod runner_integration_tests;
#[cfg(test)]
mod test_support;
