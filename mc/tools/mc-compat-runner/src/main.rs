mod runtime_config;
mod scenario_core;
#[allow(dead_code)]
mod scenario_manifest_generated;

use scenario_core::{
    parse_scenario, scenario_behavior_kind, scenario_forbidden_patterns, scenario_name,
    scenario_required_milestones, server_required_milestones, validate_static_scenario_specs,
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
const VALENCE_MONOREPO_SUBTREE_DIR: &str = "mc/valence";
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
const PROJECTILE_DAMAGE_VICTIM_START_HEALTH: f64 = 20.0;
const INVENTORY_STACK_SPLIT_MERGE_PROBE_ENV: &str = "MC_COMPAT_INVENTORY_STACK_SPLIT_MERGE_PROBE";
const INVENTORY_STACK_CLIENT_INITIAL_NEEDLE: &str =
    "inventory_stack_initial_slot window=0 state_id=";
const INVENTORY_STACK_CLIENT_SPLIT_PICKUP_NEEDLE: &str = "inventory_stack_split_pickup_sent";
const INVENTORY_STACK_CLIENT_SPLIT_SOURCE_NEEDLE: &str = "inventory_stack_split_source_seen";
const INVENTORY_STACK_CLIENT_SPLIT_PLACE_NEEDLE: &str = "inventory_stack_split_place_sent";
const INVENTORY_STACK_CLIENT_DESTINATION_NEEDLE: &str = "inventory_stack_split_destination_seen";
const INVENTORY_STACK_CLIENT_MERGE_PICKUP_NEEDLE: &str = "inventory_stack_merge_pickup_sent";
const INVENTORY_STACK_CLIENT_MERGE_EMPTY_NEEDLE: &str =
    "inventory_stack_merge_destination_empty_seen";
const INVENTORY_STACK_CLIENT_MERGE_PLACE_NEEDLE: &str = "inventory_stack_merge_place_sent";
const INVENTORY_STACK_CLIENT_FINAL_NEEDLE: &str = "inventory_stack_final_source_seen";
const INVENTORY_STACK_SERVER_SPLIT_PICKUP_NEEDLE: &str =
    "inventory_stack_server_split_pickup username=compatbot";
const INVENTORY_STACK_SERVER_SPLIT_NEEDLE: &str = "inventory_stack_server_split username=compatbot";
const INVENTORY_STACK_SERVER_MERGE_PICKUP_NEEDLE: &str =
    "inventory_stack_server_merge_pickup username=compatbot";
const INVENTORY_STACK_SERVER_MERGE_NEEDLE: &str = "inventory_stack_server_merge username=compatbot";
const INVENTORY_DRAG_TRANSACTIONS_PROBE_ENV: &str = "MC_COMPAT_INVENTORY_DRAG_TRANSACTIONS_PROBE";
const INVENTORY_DRAG_CLIENT_INITIAL_NEEDLE: &str = "inventory_drag_initial_slot window=0 state_id=";
const INVENTORY_DRAG_CLIENT_PICKUP_NEEDLE: &str = "inventory_drag_pickup_sent";
const INVENTORY_DRAG_CLIENT_SOURCE_EMPTY_NEEDLE: &str = "inventory_drag_source_empty_seen";
const INVENTORY_DRAG_CLIENT_START_NEEDLE: &str = "inventory_drag_start_sent";
const INVENTORY_DRAG_CLIENT_TARGET_A_NEEDLE: &str = "inventory_drag_target_a_sent";
const INVENTORY_DRAG_CLIENT_TARGET_B_NEEDLE: &str = "inventory_drag_target_b_sent";
const INVENTORY_DRAG_CLIENT_END_NEEDLE: &str = "inventory_drag_end_sent";
const INVENTORY_DRAG_CLIENT_FINAL_NEEDLE: &str = "inventory_drag_final_distribution_seen";
const INVENTORY_DRAG_SERVER_PICKUP_NEEDLE: &str = "inventory_drag_server_pickup username=compatbot";
const INVENTORY_DRAG_SERVER_START_NEEDLE: &str = "inventory_drag_server_start username=compatbot";
const INVENTORY_DRAG_SERVER_TARGET_A_NEEDLE: &str =
    "inventory_drag_server_target_a username=compatbot";
const INVENTORY_DRAG_SERVER_TARGET_B_NEEDLE: &str =
    "inventory_drag_server_target_b username=compatbot";
const INVENTORY_DRAG_SERVER_END_NEEDLE: &str = "inventory_drag_server_end username=compatbot";
const SURVIVAL_CHEST_CLIENT_OPEN_NEEDLE: &str = "survival_chest_open_seen window=1 position=8,64,0";
const SURVIVAL_CHEST_CLIENT_STORE_NEEDLE: &str =
    "survival_chest_store_sent window=1 slot=0 item=Dirt count=1";
const SURVIVAL_CHEST_CLIENT_CLOSE_NEEDLE: &str = "survival_chest_close_sent window=1";
const SURVIVAL_CHEST_CLIENT_RECONNECT_NEEDLE: &str = "survival_chest_reconnect_sent session=1";
const SURVIVAL_CHEST_CLIENT_REOPEN_NEEDLE: &str =
    "survival_chest_reopen_seen window=1 position=8,64,0";
const SURVIVAL_CHEST_CLIENT_PERSISTED_NEEDLE: &str =
    "survival_chest_persisted_seen window=1 slot=0 item=Dirt count=1";
const SURVIVAL_CHEST_SERVER_OPEN_NEEDLE: &str =
    "survival_chest_open username=compatbot position=8,64,0 window=1";
const SURVIVAL_CHEST_SERVER_STORE_NEEDLE: &str =
    "survival_chest_store username=compatbot window=1 slot=0 item=Dirt count=1";
const SURVIVAL_CHEST_SERVER_CLOSE_NEEDLE: &str = "survival_chest_close username=compatbot window=1";
const SURVIVAL_CHEST_SERVER_REOPEN_NEEDLE: &str =
    "survival_chest_reopen username=compatbot position=8,64,0 window=1";
const SURVIVAL_CHEST_SERVER_PERSISTED_NEEDLE: &str =
    "survival_chest_persisted username=compatbot slot=0 item=Dirt count=1";
const SURVIVAL_CHEST_FIXTURE_ENV: &str = "MC_COMPAT_SURVIVAL_CHEST_FIXTURE";
const SURVIVAL_CRAFTING_CLIENT_OPEN_NEEDLE: &str =
    "survival_crafting_table_open_seen window=1 position=4,64,0";
const SURVIVAL_CRAFTING_CLIENT_INPUT_A_NEEDLE: &str =
    "survival_crafting_input_a_sent window=1 slot=1 item=OakPlanks count=1";
const SURVIVAL_CRAFTING_CLIENT_INPUT_B_NEEDLE: &str =
    "survival_crafting_input_b_sent window=1 slot=4 item=OakPlanks count=1";
const SURVIVAL_CRAFTING_CLIENT_RESULT_NEEDLE: &str =
    "survival_crafting_result_seen window=1 slot=0 item=Stick count=4 recipe=minecraft:stick";
const SURVIVAL_CRAFTING_CLIENT_COLLECT_NEEDLE: &str =
    "survival_crafting_result_collected window=1 slot=0 item=Stick count=4";
const SURVIVAL_CRAFTING_CLIENT_INVENTORY_NEEDLE: &str =
    "survival_crafting_inventory_updated slot=36 item=Stick count=4";
const SURVIVAL_CRAFTING_SERVER_OPEN_NEEDLE: &str =
    "survival_crafting_table_open username=compatbot position=4,64,0 window=1";
const SURVIVAL_CRAFTING_SERVER_INPUT_A_NEEDLE: &str =
    "survival_crafting_input_a username=compatbot window=1 slot=1 item=OakPlanks count=1";
const SURVIVAL_CRAFTING_SERVER_INPUT_B_NEEDLE: &str =
    "survival_crafting_input_b username=compatbot window=1 slot=4 item=OakPlanks count=1";
const SURVIVAL_CRAFTING_SERVER_RESULT_NEEDLE: &str =
    "survival_crafting_result username=compatbot window=1 slot=0 item=Stick count=4 recipe=minecraft:stick";
const SURVIVAL_CRAFTING_SERVER_COLLECT_NEEDLE: &str =
    "survival_crafting_collect username=compatbot window=1 slot=0 item=Stick count=4 inventory_slot=36";
const SURVIVAL_CRAFTING_FIXTURE_ENV: &str = "MC_COMPAT_SURVIVAL_CRAFTING_FIXTURE";
const SURVIVAL_CRAFTING_BREADTH_PROBE_ENV: &str = "MC_COMPAT_SURVIVAL_CRAFTING_BREADTH_PROBE";
const SURVIVAL_CRAFTING_BREADTH_FIXTURE_ENV: &str = "MC_COMPAT_SURVIVAL_CRAFTING_BREADTH_FIXTURE";
const SURVIVAL_CRAFTING_BREADTH_CLIENT_SHAPED_NEEDLE: &str =
    "survival_crafting_breadth_shaped_seen window=1 recipe=minecraft:chest input=oak_planksx8 result=Chest count=1";
const SURVIVAL_CRAFTING_BREADTH_CLIENT_SHAPELESS_NEEDLE: &str =
    "survival_crafting_breadth_shapeless_seen window=1 recipe=minecraft:oak_planks input=oak_logx1 result=OakPlanks count=4";
const SURVIVAL_CRAFTING_BREADTH_CLIENT_CLEAR_NEEDLE: &str =
    "survival_crafting_breadth_grid_clear_seen window=1 occupied_slots=0";
const SURVIVAL_CRAFTING_BREADTH_CLIENT_INVALID_NEEDLE: &str =
    "survival_crafting_breadth_invalid_seen window=1 recipe=minecraft:stick_insufficient_input_rejection input=single_oak_plank outcome=no_result";
const SURVIVAL_CRAFTING_BREADTH_CLIENT_INVENTORY_NEEDLE: &str =
    "survival_crafting_breadth_inventory_updated slot=36 item=Chest count=1 slot=37 item=OakPlanks count=4";
const SURVIVAL_CRAFTING_BREADTH_SERVER_SHAPED_NEEDLE: &str =
    "survival_crafting_breadth_shaped username=compatbot recipe=minecraft:chest input=oak_planksx8 result=Chest count=1";
const SURVIVAL_CRAFTING_BREADTH_SERVER_SHAPELESS_NEEDLE: &str =
    "survival_crafting_breadth_shapeless username=compatbot recipe=minecraft:oak_planks input=oak_logx1 result=OakPlanks count=4";
const SURVIVAL_CRAFTING_BREADTH_SERVER_CLEAR_NEEDLE: &str =
    "survival_crafting_breadth_grid_clear username=compatbot window=1 occupied_slots=0";
const SURVIVAL_CRAFTING_BREADTH_SERVER_INVALID_NEEDLE: &str =
    "survival_crafting_breadth_invalid_rejected username=compatbot recipe=minecraft:stick_insufficient_input_rejection input=single_oak_plank outcome=no_result";
const SURVIVAL_CRAFTING_BREADTH_SERVER_STATE_NEEDLE: &str =
    "survival_crafting_breadth_state username=compatbot shaped=true shapeless=true invalid_rejected=true extra_outputs=false";
const SURVIVAL_FURNACE_CLIENT_OPEN_NEEDLE: &str =
    "survival_furnace_open_seen window=1 position=12,64,0";
const SURVIVAL_FURNACE_CLIENT_INPUT_NEEDLE: &str =
    "survival_furnace_input_sent window=1 slot=0 item=RawIron count=1";
const SURVIVAL_FURNACE_CLIENT_FUEL_NEEDLE: &str =
    "survival_furnace_fuel_sent window=1 slot=1 item=Coal count=1";
const SURVIVAL_FURNACE_CLIENT_BURN_NEEDLE: &str =
    "survival_furnace_burn_progress_seen window=1 progress=started";
const SURVIVAL_FURNACE_CLIENT_OUTPUT_NEEDLE: &str =
    "survival_furnace_output_seen window=1 slot=2 item=IronIngot count=1";
const SURVIVAL_FURNACE_CLIENT_COLLECT_NEEDLE: &str =
    "survival_furnace_output_collected window=1 slot=2 item=IronIngot count=1";
const SURVIVAL_FURNACE_CLIENT_INVENTORY_NEEDLE: &str =
    "survival_furnace_inventory_updated slot=36 item=IronIngot count=1";
const SURVIVAL_FURNACE_CLIENT_INVALID_FUEL_NEEDLE: &str =
    "survival_furnace_invalid_fuel_sent window=1 slot=1 item=RawIron outcome=no_burn";
const SURVIVAL_FURNACE_CLIENT_RECONNECT_NEEDLE: &str = "survival_furnace_reconnect_sent session=1";
const SURVIVAL_FURNACE_CLIENT_REOPEN_NEEDLE: &str =
    "survival_furnace_reopen_seen window=1 position=12,64,0";
const SURVIVAL_FURNACE_SERVER_OPEN_NEEDLE: &str =
    "survival_furnace_open username=compatbot position=12,64,0 window=1";
const SURVIVAL_FURNACE_SERVER_INPUT_NEEDLE: &str =
    "survival_furnace_input_insert username=compatbot window=1 slot=0 item=RawIron count=1";
const SURVIVAL_FURNACE_SERVER_FUEL_NEEDLE: &str =
    "survival_furnace_fuel_insert username=compatbot window=1 slot=1 item=Coal count=1";
const SURVIVAL_FURNACE_SERVER_BURN_NEEDLE: &str =
    "survival_furnace_burn_progress username=compatbot window=1 progress=started";
const SURVIVAL_FURNACE_SERVER_OUTPUT_NEEDLE: &str =
    "survival_furnace_output_available username=compatbot window=1 slot=2 item=IronIngot count=1";
const SURVIVAL_FURNACE_SERVER_COLLECT_NEEDLE: &str =
    "survival_furnace_output_collect username=compatbot window=1 slot=2 item=IronIngot count=1 inventory_slot=36";
const SURVIVAL_FURNACE_SERVER_INVALID_FUEL_NEEDLE: &str =
    "survival_furnace_invalid_fuel_rejected username=compatbot window=1 slot=1 item=RawIron outcome=no_burn";
const SURVIVAL_FURNACE_SERVER_BREADTH_STATE_NEEDLE: &str =
    "survival_furnace_breadth_state username=compatbot recipe=minecraft:iron_ingot input=RawIron fuel=Coal output=IronIngot count=1 invalid_fuel=RawIron invalid_fuel_outcome=no_burn broad_all_furnaces=false";
const SURVIVAL_FURNACE_SERVER_REOPEN_NEEDLE: &str =
    "survival_furnace_reconnect_reopen username=compatbot position=12,64,0 window=1";
const SURVIVAL_FURNACE_SERVER_STATE_NEEDLE: &str =
    "survival_furnace_server_state username=compatbot position=12,64,0 input=RawIron fuel=Coal output=empty collected=true session_persistent=true";
const SURVIVAL_FURNACE_PROBE_ENV: &str = "MC_COMPAT_SURVIVAL_FURNACE_PROBE";
const SURVIVAL_FURNACE_SMELTING_BREADTH_PROBE_ENV: &str =
    "MC_COMPAT_SURVIVAL_FURNACE_SMELTING_BREADTH_PROBE";
const SURVIVAL_FURNACE_SESSION_ENV: &str = "MC_COMPAT_SURVIVAL_FURNACE_SESSION";
const SURVIVAL_FURNACE_FIXTURE_ENV: &str = "MC_COMPAT_SURVIVAL_FURNACE_FIXTURE";
const SURVIVAL_FURNACE_SMELTING_BREADTH_FIXTURE_ENV: &str =
    "MC_COMPAT_SURVIVAL_FURNACE_SMELTING_BREADTH_FIXTURE";
const SURVIVAL_HUNGER_FOOD_CLIENT_ITEM_NEEDLE: &str =
    "survival_hunger_food_item_seen slot=36 item=Bread count=1";
const SURVIVAL_HUNGER_FOOD_CLIENT_PRE_NEEDLE: &str =
    "survival_hunger_food_pre_seen health=20.0 food=15 saturation=0.0";
const SURVIVAL_HUNGER_FOOD_CLIENT_USE_NEEDLE: &str =
    "survival_hunger_food_use_sent slot=36 item=Bread count=1 hand=main sequence=810";
const SURVIVAL_HUNGER_FOOD_CLIENT_POST_NEEDLE: &str =
    "survival_hunger_food_post_seen health=20.0 food=20 saturation=6.0";
const SURVIVAL_HUNGER_FOOD_CLIENT_INVENTORY_NEEDLE: &str =
    "survival_hunger_food_inventory_updated slot=36 item=Bread count=0";
const SURVIVAL_HUNGER_FOOD_SERVER_PRE_NEEDLE: &str =
    "survival_hunger_food_pre username=compatbot health=20.0 food=15 saturation=0.0 item=Bread count=1 slot=36";
const SURVIVAL_HUNGER_FOOD_SERVER_CONSUME_START_NEEDLE: &str =
    "survival_hunger_food_consume_start username=compatbot item=Bread slot=36 food_before=15 saturation_before=0.0";
const SURVIVAL_HUNGER_FOOD_SERVER_CONSUME_FINISH_NEEDLE: &str =
    "survival_hunger_food_consume_finish username=compatbot item=Bread slot=36 food_after=20 saturation_after=6.0";
const SURVIVAL_HUNGER_FOOD_SERVER_INVENTORY_NEEDLE: &str =
    "survival_hunger_food_inventory username=compatbot slot=36 item=Bread count_before=1 count_after=0";
const SURVIVAL_HUNGER_FOOD_SERVER_STATE_NEEDLE: &str =
    "survival_hunger_food_state username=compatbot health=20.0 food_before=15 food_after=20 saturation_before=0.0 saturation_after=6.0 unexpected_damage=false death=false";
const SURVIVAL_HUNGER_FOOD_PROBE_ENV: &str = "MC_COMPAT_SURVIVAL_HUNGER_FOOD_PROBE";
const SURVIVAL_HUNGER_FOOD_FIXTURE_ENV: &str = "MC_COMPAT_SURVIVAL_HUNGER_FOOD_FIXTURE";
const SURVIVAL_HUNGER_HEALTH_CLIENT_ITEM_NEEDLE: &str =
    "survival_hunger_health_item_seen slot=36 item=Bread count=1";
const SURVIVAL_HUNGER_HEALTH_CLIENT_PRE_NEEDLE: &str =
    "survival_hunger_health_pre_seen health=18.0 food=15 saturation=0.0";
const SURVIVAL_HUNGER_HEALTH_CLIENT_USE_NEEDLE: &str =
    "survival_hunger_health_consume_sent slot=36 item=Bread count=1 hand=main sequence=810";
const SURVIVAL_HUNGER_HEALTH_CLIENT_POST_NEEDLE: &str =
    "survival_hunger_health_recovery_seen health=20.0 food=20 saturation=6.0";
const SURVIVAL_HUNGER_HEALTH_CLIENT_INVENTORY_NEEDLE: &str =
    "survival_hunger_health_inventory_updated slot=36 item=Bread count=0";
const SURVIVAL_HUNGER_HEALTH_SERVER_PRE_NEEDLE: &str =
    "survival_hunger_health_pre username=compatbot health=18.0 food=15 saturation=0.0 item=Bread count=1 slot=36";
const SURVIVAL_HUNGER_HEALTH_SERVER_CONSUME_START_NEEDLE: &str =
    "survival_hunger_health_consume_start username=compatbot item=Bread slot=36 food_before=15 saturation_before=0.0";
const SURVIVAL_HUNGER_HEALTH_SERVER_CONSUME_FINISH_NEEDLE: &str =
    "survival_hunger_health_consume_finish username=compatbot item=Bread slot=36 food_after=20 saturation_after=6.0";
const SURVIVAL_HUNGER_HEALTH_SERVER_INVENTORY_NEEDLE: &str =
    "survival_hunger_health_inventory username=compatbot slot=36 item=Bread count_before=1 count_after=0";
const SURVIVAL_HUNGER_HEALTH_SERVER_STATE_NEEDLE: &str =
    "survival_hunger_health_state username=compatbot pre_health=18.0 post_health=20.0 food_before=15 food_after=20 saturation_before=0.0 saturation_after=6.0 unexpected_damage=false death=false";
const SURVIVAL_HUNGER_HEALTH_PROBE_ENV: &str = "MC_COMPAT_SURVIVAL_HUNGER_HEALTH_PROBE";
const SURVIVAL_HUNGER_HEALTH_FIXTURE_ENV: &str = "MC_COMPAT_SURVIVAL_HUNGER_HEALTH_FIXTURE";
const SURVIVAL_MOB_DROP_CLIENT_MOB_NEEDLE: &str =
    "survival_mob_drop_mob_seen mob=IronGolem position=16.5,65.0,2.5";
const SURVIVAL_MOB_DROP_CLIENT_ATTACK_NEEDLE: &str =
    "survival_mob_drop_attack_sent mob=IronGolem target_id=";
const SURVIVAL_MOB_DROP_CLIENT_DEATH_NEEDLE: &str =
    "survival_mob_drop_death_seen mob=IronGolem target_id=";
const SURVIVAL_MOB_DROP_CLIENT_DROP_NEEDLE: &str =
    "survival_mob_drop_drop_seen item=IronIngot count=1";
const SURVIVAL_MOB_DROP_CLIENT_PICKUP_NEEDLE: &str =
    "survival_mob_drop_pickup_seen item=IronIngot count=1";
const SURVIVAL_MOB_DROP_CLIENT_INVENTORY_NEEDLE: &str =
    "survival_mob_drop_inventory_updated slot=36 item=IronIngot count=1";
const SURVIVAL_MOB_DROP_SERVER_SPAWN_NEEDLE: &str =
    "survival_mob_drop_spawn username=compatbot mob=IronGolem position=16.5,65.0,2.5";
const SURVIVAL_MOB_DROP_SERVER_ATTACK_NEEDLE: &str =
    "survival_mob_drop_attack username=compatbot mob=IronGolem damage=20.0";
const SURVIVAL_MOB_DROP_SERVER_DEATH_NEEDLE: &str =
    "survival_mob_drop_death username=compatbot mob=IronGolem";
const SURVIVAL_MOB_DROP_SERVER_DROP_NEEDLE: &str =
    "survival_mob_drop_drop_spawn username=compatbot item=IronIngot count=1";
const SURVIVAL_MOB_DROP_SERVER_PICKUP_NEEDLE: &str =
    "survival_mob_drop_pickup username=compatbot item=IronIngot count=1";
const SURVIVAL_MOB_DROP_SERVER_INVENTORY_NEEDLE: &str =
    "survival_mob_drop_inventory username=compatbot slot=36 item=IronIngot count=1";
const SURVIVAL_MOB_DROP_SERVER_STATE_NEEDLE: &str =
    "survival_mob_drop_state username=compatbot mob=IronGolem drop=IronIngot count=1 extra_drops=false";
const SURVIVAL_MOB_DROP_PROBE_ENV: &str = "MC_COMPAT_SURVIVAL_MOB_DROP_PROBE";
const SURVIVAL_MOB_DROP_FIXTURE_ENV: &str = "MC_COMPAT_SURVIVAL_MOB_DROP_FIXTURE";
const SURVIVAL_MOB_AI_LOOT_CLIENT_MOB_NEEDLE: &str =
    "survival_mob_ai_loot_mob_seen mob=Zombie position=16.5,65.0,4.5 ai_checkpoint=approach_player";
const SURVIVAL_MOB_AI_LOOT_CLIENT_ATTACK_NEEDLE: &str =
    "survival_mob_ai_loot_attack_sent mob=Zombie kill_method=player_attack";
const SURVIVAL_MOB_AI_LOOT_CLIENT_DEATH_NEEDLE: &str = "survival_mob_ai_loot_death_seen mob=Zombie";
const SURVIVAL_MOB_AI_LOOT_CLIENT_DROP_NEEDLE: &str =
    "survival_mob_ai_loot_drop_seen item=RottenFlesh count=1";
const SURVIVAL_MOB_AI_LOOT_CLIENT_PICKUP_NEEDLE: &str =
    "survival_mob_ai_loot_pickup_seen item=RottenFlesh count=1";
const SURVIVAL_MOB_AI_LOOT_CLIENT_INVENTORY_NEEDLE: &str =
    "survival_mob_ai_loot_inventory_updated slot=36 item=RottenFlesh count=1";
const SURVIVAL_MOB_AI_LOOT_SERVER_SPAWN_NEEDLE: &str =
    "survival_mob_ai_loot_spawn username=compatbot mob=Zombie position=16.5,65.0,4.5";
const SURVIVAL_MOB_AI_LOOT_SERVER_AI_NEEDLE: &str =
    "survival_mob_ai_loot_ai_checkpoint username=compatbot mob=Zombie checkpoint=approach_player target=compatbot";
const SURVIVAL_MOB_AI_LOOT_SERVER_ATTACK_NEEDLE: &str =
    "survival_mob_ai_loot_attack username=compatbot mob=Zombie kill_method=player_attack";
const SURVIVAL_MOB_AI_LOOT_SERVER_DEATH_NEEDLE: &str =
    "survival_mob_ai_loot_death username=compatbot mob=Zombie";
const SURVIVAL_MOB_AI_LOOT_SERVER_DROP_NEEDLE: &str =
    "survival_mob_ai_loot_drop_spawn username=compatbot item=RottenFlesh count=1";
const SURVIVAL_MOB_AI_LOOT_SERVER_PICKUP_NEEDLE: &str =
    "survival_mob_ai_loot_pickup username=compatbot item=RottenFlesh count=1";
const SURVIVAL_MOB_AI_LOOT_SERVER_INVENTORY_NEEDLE: &str =
    "survival_mob_ai_loot_inventory username=compatbot slot=36 item=RottenFlesh count=1";
const SURVIVAL_MOB_AI_LOOT_SERVER_STATE_NEEDLE: &str =
    "survival_mob_ai_loot_state username=compatbot mob=Zombie ai_checkpoint=approach_player kill_method=player_attack drop=RottenFlesh count=1 pickup=observed inventory_increment=1 extra_mobs=false";
const SURVIVAL_MOB_AI_LOOT_PROBE_ENV: &str = "MC_COMPAT_SURVIVAL_MOB_AI_LOOT_PROBE";
const SURVIVAL_MOB_AI_LOOT_FIXTURE_ENV: &str = "MC_COMPAT_SURVIVAL_MOB_AI_LOOT_FIXTURE";
const SURVIVAL_REDSTONE_TOGGLE_CLIENT_INPUT_ON_NEEDLE: &str =
    "survival_redstone_toggle_input_sent control=Lever position=20,64,0 powered_before=false powered_after=true";
const SURVIVAL_REDSTONE_TOGGLE_CLIENT_OUTPUT_ON_NEEDLE: &str =
    "survival_redstone_toggle_output_update output=RedstoneLamp position=21,64,0 powered=true";
const SURVIVAL_REDSTONE_TOGGLE_CLIENT_INPUT_OFF_NEEDLE: &str =
    "survival_redstone_toggle_return_input_sent control=Lever position=20,64,0 powered_before=true powered_after=false";
const SURVIVAL_REDSTONE_TOGGLE_CLIENT_OUTPUT_OFF_NEEDLE: &str =
    "survival_redstone_toggle_return_update output=RedstoneLamp position=21,64,0 powered=false";
const SURVIVAL_REDSTONE_TOGGLE_SERVER_INPUT_NEEDLE: &str =
    "survival_redstone_toggle_input username=compatbot control=Lever position=20,64,0 powered_before=false powered_after=true";
const SURVIVAL_REDSTONE_TOGGLE_SERVER_ON_NEEDLE: &str =
    "survival_redstone_toggle_powered_on username=compatbot output=RedstoneLamp position=21,64,0 powered=true";
const SURVIVAL_REDSTONE_TOGGLE_SERVER_OFF_NEEDLE: &str =
    "survival_redstone_toggle_powered_off username=compatbot output=RedstoneLamp position=21,64,0 powered=false";
const SURVIVAL_REDSTONE_TOGGLE_SERVER_STATE_NEEDLE: &str =
    "survival_redstone_toggle_state username=compatbot control=Lever output=RedstoneLamp on_seen=true off_seen=true unintended_outputs=false";
const SURVIVAL_REDSTONE_TOGGLE_PROBE_ENV: &str = "MC_COMPAT_SURVIVAL_REDSTONE_TOGGLE_PROBE";
const SURVIVAL_REDSTONE_TOGGLE_FIXTURE_ENV: &str = "MC_COMPAT_SURVIVAL_REDSTONE_TOGGLE_FIXTURE";
const SURVIVAL_REDSTONE_CIRCUIT_CLIENT_INITIAL_NEEDLE: &str =
    "survival_redstone_circuit_initial_state circuit=lever_lamp_repeater tick=0 powered=false";
const SURVIVAL_REDSTONE_CIRCUIT_CLIENT_INPUT_NEEDLE: &str =
    "survival_redstone_circuit_input_sent control=Lever position=20,64,0 tick=2 powered_after=true";
const SURVIVAL_REDSTONE_CIRCUIT_CLIENT_OUTPUT_ON_NEEDLE: &str =
    "survival_redstone_circuit_output_update output=RedstoneLamp repeater=Repeater position=21,64,0 tick=2 powered=true";
const SURVIVAL_REDSTONE_CIRCUIT_CLIENT_RETURN_NEEDLE: &str =
    "survival_redstone_circuit_return_input_sent control=Lever position=20,64,0 tick=4 powered_after=false";
const SURVIVAL_REDSTONE_CIRCUIT_CLIENT_OUTPUT_OFF_NEEDLE: &str =
    "survival_redstone_circuit_return_update output=RedstoneLamp repeater=Repeater position=21,64,0 tick=4 powered=false";
const SURVIVAL_REDSTONE_CIRCUIT_SERVER_INITIAL_NEEDLE: &str =
    "survival_redstone_circuit_initial username=compatbot circuit=lever_lamp_repeater powered=false tick=0";
const SURVIVAL_REDSTONE_CIRCUIT_SERVER_INPUT_NEEDLE: &str =
    "survival_redstone_circuit_input username=compatbot control=Lever position=20,64,0 tick=2 powered_after=true";
const SURVIVAL_REDSTONE_CIRCUIT_SERVER_ON_NEEDLE: &str =
    "survival_redstone_circuit_powered_on username=compatbot output=RedstoneLamp repeater=Repeater tick=2 powered=true";
const SURVIVAL_REDSTONE_CIRCUIT_SERVER_OFF_NEEDLE: &str =
    "survival_redstone_circuit_powered_off username=compatbot output=RedstoneLamp repeater=Repeater tick=4 powered=false";
const SURVIVAL_REDSTONE_CIRCUIT_SERVER_STATE_NEEDLE: &str =
    "survival_redstone_circuit_state username=compatbot circuit=lever_lamp_repeater initial=false after_input=true after_return=false tick_sequence=0:false,2:true,4:false unintended_outputs=false";
const SURVIVAL_REDSTONE_CIRCUIT_PROBE_ENV: &str = "MC_COMPAT_SURVIVAL_REDSTONE_CIRCUIT_PROBE";
const SURVIVAL_REDSTONE_CIRCUIT_FIXTURE_ENV: &str = "MC_COMPAT_SURVIVAL_REDSTONE_CIRCUIT_FIXTURE";
const SURVIVAL_WORLD_PERSISTENCE_CLIENT_MUTATION_NEEDLE: &str =
    "survival_world_persistence_mutation_sent block=Dirt position=24,64,0 slot=36";
const SURVIVAL_WORLD_PERSISTENCE_CLIENT_PRE_RESTART_NEEDLE: &str =
    "survival_world_persistence_pre_restart_update block=Dirt position=24,64,0";
const SURVIVAL_WORLD_PERSISTENCE_CLIENT_RECONNECT_NEEDLE: &str =
    "survival_world_persistence_reconnect_sent session=restart";
const SURVIVAL_WORLD_PERSISTENCE_CLIENT_POST_RESTART_NEEDLE: &str =
    "survival_world_persistence_post_restart_update block=Dirt position=24,64,0";
const SURVIVAL_WORLD_PERSISTENCE_SERVER_MUTATION_NEEDLE: &str =
    "survival_world_persistence_mutation username=compatbot block=Dirt position=24,64,0 persisted_before=false persisted_after=true";
const SURVIVAL_WORLD_PERSISTENCE_SERVER_CLEAN_NEEDLE: &str =
    "survival_world_persistence_clean_shutdown username=compatbot storage=isolated shutdown=graceful";
const SURVIVAL_WORLD_PERSISTENCE_SERVER_RESTART_NEEDLE: &str =
    "survival_world_persistence_backend_restart username=compatbot method=controlled_reload storage=isolated restart_confirmed=true";
const SURVIVAL_WORLD_PERSISTENCE_SERVER_POST_NEEDLE: &str =
    "survival_world_persistence_post_restart_observe username=compatbot block=Dirt position=24,64,0 persisted=true";
const SURVIVAL_WORLD_PERSISTENCE_SERVER_STATE_NEEDLE: &str =
    "survival_world_persistence_state username=compatbot block=Dirt position=24,64,0 pre_mutation=true clean_shutdown=true backend_restart=true post_observed=true dirty_reuse=false";
const SURVIVAL_BLOCK_ENTITY_CLIENT_PRE_RESTART_NEEDLE: &str =
    "survival_block_entity_pre_restart_update kind=Sign position=28,64,0 text=MC|Compat|Sign|Persist";
const SURVIVAL_BLOCK_ENTITY_CLIENT_RECONNECT_NEEDLE: &str =
    "survival_block_entity_reconnect_sent session=restart";
const SURVIVAL_BLOCK_ENTITY_CLIENT_POST_RESTART_NEEDLE: &str =
    "survival_block_entity_post_restart_update kind=Sign position=28,64,0 text=MC|Compat|Sign|Persist";
const SURVIVAL_BLOCK_ENTITY_SERVER_MUTATION_NEEDLE: &str =
    "survival_block_entity_persistence_mutation username=compatbot kind=Sign position=28,64,0 text=MC|Compat|Sign|Persist persisted_before=false persisted_after=true";
const SURVIVAL_BLOCK_ENTITY_SERVER_CLEAN_NEEDLE: &str =
    "survival_block_entity_persistence_clean_shutdown username=compatbot storage=isolated shutdown=graceful";
const SURVIVAL_BLOCK_ENTITY_SERVER_RESTART_NEEDLE: &str =
    "survival_block_entity_persistence_backend_restart username=compatbot method=controlled_reload storage=isolated restart_confirmed=true";
const SURVIVAL_BLOCK_ENTITY_SERVER_POST_NEEDLE: &str =
    "survival_block_entity_persistence_post_restart_observe username=compatbot kind=Sign position=28,64,0 text=MC|Compat|Sign|Persist persisted=true";
const SURVIVAL_BLOCK_ENTITY_SERVER_STATE_NEEDLE: &str =
    "survival_block_entity_persistence_state username=compatbot kind=Sign position=28,64,0 text=MC|Compat|Sign|Persist pre_mutation=true clean_shutdown=true backend_restart=true post_observed=true dirty_reuse=false";
const SURVIVAL_WORLD_MULTICHUNK_CLIENT_MUTATION_NEEDLE: &str =
    "survival_world_multichunk_mutation_sent primary=0,64,0:Dirt secondary=32,64,0:OakPlanks chunks=0,0;2,0";
const SURVIVAL_WORLD_MULTICHUNK_CLIENT_PRE_RESTART_NEEDLE: &str =
    "survival_world_multichunk_pre_restart_update primary=present secondary=present auxiliary_marker_only=false";
const SURVIVAL_WORLD_MULTICHUNK_CLIENT_RECONNECT_NEEDLE: &str =
    "survival_world_multichunk_reconnect_sent session=restart";
const SURVIVAL_WORLD_MULTICHUNK_CLIENT_POST_RESTART_NEEDLE: &str =
    "survival_world_multichunk_post_restart_update primary=present secondary=present";
const SURVIVAL_WORLD_MULTICHUNK_SERVER_MUTATION_NEEDLE: &str =
    "survival_world_multichunk_mutation username=compatbot chunks=0,0;2,0 primary=0,64,0:Dirt secondary=32,64,0:OakPlanks persisted_before=false persisted_after=true";
const SURVIVAL_WORLD_MULTICHUNK_SERVER_CLEAN_NEEDLE: &str =
    "survival_world_multichunk_clean_shutdown username=compatbot storage=isolated shutdown=graceful";
const SURVIVAL_WORLD_MULTICHUNK_SERVER_RESTART_NEEDLE: &str =
    "survival_world_multichunk_backend_restart username=compatbot method=controlled_reload storage=isolated restart_confirmed=true";
const SURVIVAL_WORLD_MULTICHUNK_SERVER_POST_NEEDLE: &str =
    "survival_world_multichunk_post_restart_observe username=compatbot primary=present secondary=present auxiliary_marker_only=false";
const SURVIVAL_WORLD_MULTICHUNK_SERVER_STATE_NEEDLE: &str =
    "survival_world_multichunk_state username=compatbot chunks=0,0;2,0 primary=present secondary=present controlled_reload=true post_observed=true auxiliary_marker_only=false dirty_reuse=false";
const SURVIVAL_CONTAINER_BLOCK_ENTITY_CLIENT_OPEN_NEEDLE: &str =
    "survival_container_block_entity_open_seen window=1 kind=Barrel position=34,64,0";
const SURVIVAL_CONTAINER_BLOCK_ENTITY_CLIENT_TRANSFER_NEEDLE: &str =
    "survival_container_block_entity_transfer_sent window=1 slot=0 item=Dirt count=1";
const SURVIVAL_CONTAINER_BLOCK_ENTITY_CLIENT_PAYLOAD_NEEDLE: &str =
    "survival_container_block_entity_payload_seen summary=slot0:Dirt:1";
const SURVIVAL_CONTAINER_BLOCK_ENTITY_CLIENT_METADATA_NEEDLE: &str =
    "survival_container_block_entity_metadata_seen summary=custom_name:MC Compat Barrel";
const SURVIVAL_CONTAINER_BLOCK_ENTITY_CLIENT_REOPEN_NEEDLE: &str =
    "survival_container_block_entity_reopen_seen window=1 kind=Barrel position=34,64,0 payload=slot0:Dirt:1";
const SURVIVAL_CONTAINER_BLOCK_ENTITY_SERVER_OPEN_NEEDLE: &str =
    "survival_container_block_entity_open username=compatbot window=1 kind=Barrel position=34,64,0";
const SURVIVAL_CONTAINER_BLOCK_ENTITY_SERVER_TRANSFER_NEEDLE: &str =
    "survival_container_block_entity_transfer username=compatbot window=1 slot=0 item=Dirt count=1";
const SURVIVAL_CONTAINER_BLOCK_ENTITY_SERVER_PAYLOAD_NEEDLE: &str =
    "survival_container_block_entity_payload username=compatbot summary=slot0:Dirt:1";
const SURVIVAL_CONTAINER_BLOCK_ENTITY_SERVER_METADATA_NEEDLE: &str =
    "survival_container_block_entity_metadata username=compatbot summary=custom_name:MC Compat Barrel";
const SURVIVAL_CONTAINER_BLOCK_ENTITY_SERVER_STATE_NEEDLE: &str =
    "survival_container_block_entity_state username=compatbot kind=Barrel position=34,64,0 transfer=Dirt:1 payload=slot0:Dirt:1 metadata=custom_name:MC Compat Barrel reopen=payload_present arbitrary_nbt=false";
const SURVIVAL_SIGN_EDITING_CLIENT_OPEN_NEEDLE: &str =
    "survival_sign_editing_open_seen position=28,64,0 side=front milestone=sign_editor_open_observed";
const SURVIVAL_SIGN_EDITING_CLIENT_UPDATE_NEEDLE: &str =
    "survival_sign_editing_update_sent position=28,64,0 side=front payload=MC|Compat|Sign|Edit milestone=sign_update_sent";
const SURVIVAL_SIGN_EDITING_CLIENT_POST_NEEDLE: &str =
    "survival_sign_editing_post_update_seen position=28,64,0 side=front text=MC|Compat|Sign|Edit observation=text_visible";
const SURVIVAL_SIGN_EDITING_SERVER_OPEN_NEEDLE: &str =
    "survival_sign_editing_open username=compatbot position=28,64,0 side=front milestone=sign_editor_open_observed";
const SURVIVAL_SIGN_EDITING_SERVER_UPDATE_NEEDLE: &str =
    "survival_sign_editing_update_accepted username=compatbot position=28,64,0 side=front payload=MC|Compat|Sign|Edit milestone=sign_update_accepted_observed";
const SURVIVAL_SIGN_EDITING_SERVER_STATE_NEEDLE: &str =
    "survival_sign_editing_state username=compatbot position=28,64,0 side=front payload=MC|Compat|Sign|Edit post_update=text_visible arbitrary_sign_ui=false";
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
const SURVIVAL_CRASH_RECOVERY_CLIENT_MUTATION_NEEDLE: &str =
    "survival_crash_recovery_mutation_sent block=Dirt position=24,64,0 slot=36";
const SURVIVAL_CRASH_RECOVERY_CLIENT_PRE_CRASH_NEEDLE: &str =
    "survival_crash_recovery_pre_crash_update block=Dirt position=24,64,0";
const SURVIVAL_CRASH_RECOVERY_CLIENT_RECONNECT_NEEDLE: &str =
    "survival_crash_recovery_reconnect_sent session=crash_recovery";
const SURVIVAL_CRASH_RECOVERY_CLIENT_POST_CRASH_NEEDLE: &str =
    "survival_crash_recovery_post_crash_update block=Dirt position=24,64,0";
const SURVIVAL_CRASH_RECOVERY_SERVER_MUTATION_NEEDLE: &str =
    "survival_crash_recovery_mutation username=compatbot block=Dirt position=24,64,0 persisted_before=false persisted_after=true";
const SURVIVAL_CRASH_RECOVERY_SERVER_FORCED_STOP_NEEDLE: &str =
    "survival_crash_recovery_forced_stop username=compatbot method=forced_stop storage=isolated graceful=false";
const SURVIVAL_CRASH_RECOVERY_SERVER_RESTART_NEEDLE: &str =
    "survival_crash_recovery_backend_restart username=compatbot method=crash_recovery storage=isolated restart_confirmed=true";
const SURVIVAL_CRASH_RECOVERY_SERVER_POST_NEEDLE: &str =
    "survival_crash_recovery_post_crash_observe username=compatbot block=Dirt position=24,64,0 persisted=true";
const SURVIVAL_CRASH_RECOVERY_SERVER_STATE_NEEDLE: &str =
    "survival_crash_recovery_state username=compatbot block=Dirt position=24,64,0 pre_mutation=true crash_stop=true backend_restart=true post_observed=true dirty_reuse=false";
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
const SURVIVAL_BIOME_DIMENSION_CLIENT_STATE_NEEDLE: &str =
    "survival_biome_dimension_state spawn_environment=minecraft:overworld environment_identifier=minecraft:overworld client_environment_update=minecraft:overworld normalized_identifier=minecraft:overworld";
const SURVIVAL_BIOME_DIMENSION_SERVER_STATE_NEEDLE: &str =
    "survival_biome_dimension_state username=compatbot spawn_environment=minecraft:overworld environment_identifier=minecraft:overworld server_environment_state=minecraft:overworld normalized_identifier=minecraft:overworld";
const SURVIVAL_BIOME_DIMENSION_TRAVEL_CLIENT_ORIGIN_NEEDLE: &str =
    "survival_biome_dimension_travel_origin dimension=minecraft:overworld biome=minecraft:plains";
const SURVIVAL_BIOME_DIMENSION_TRAVEL_CLIENT_TRANSITION_NEEDLE: &str =
    "survival_biome_dimension_travel_transition_sent kind=nether_portal destination=minecraft:the_nether";
const SURVIVAL_BIOME_DIMENSION_TRAVEL_CLIENT_DESTINATION_NEEDLE: &str =
    "survival_biome_dimension_travel_destination_seen dimension=minecraft:the_nether biome=minecraft:nether_wastes checkpoint=dimension_changed";
const SURVIVAL_BIOME_DIMENSION_TRAVEL_SERVER_ORIGIN_NEEDLE: &str =
    "survival_biome_dimension_travel_origin username=compatbot dimension=minecraft:overworld biome=minecraft:plains";
const SURVIVAL_BIOME_DIMENSION_TRAVEL_SERVER_TRANSITION_NEEDLE: &str =
    "survival_biome_dimension_travel_transition username=compatbot kind=nether_portal from=minecraft:overworld to=minecraft:the_nether";
const SURVIVAL_BIOME_DIMENSION_TRAVEL_SERVER_STATE_NEEDLE: &str =
    "survival_biome_dimension_travel_state username=compatbot origin_dimension=minecraft:overworld origin_biome=minecraft:plains destination_dimension=minecraft:the_nether destination_biome=minecraft:nether_wastes transition=nether_portal server_checkpoint=environment_changed";
const SURVIVAL_BIOME_DIMENSION_PROBE_ENV: &str = "MC_COMPAT_SURVIVAL_BIOME_DIMENSION_PROBE";
const SURVIVAL_BIOME_DIMENSION_FIXTURE_ENV: &str = "MC_COMPAT_SURVIVAL_BIOME_DIMENSION_FIXTURE";
const SURVIVAL_BIOME_DIMENSION_TRAVEL_PROBE_ENV: &str =
    "MC_COMPAT_SURVIVAL_BIOME_DIMENSION_TRAVEL_PROBE";
const SURVIVAL_BIOME_DIMENSION_TRAVEL_FIXTURE_ENV: &str =
    "MC_COMPAT_SURVIVAL_BIOME_DIMENSION_TRAVEL_FIXTURE";
const MCP_CONTROLLED_SMOKE_SCENARIO: &str = "mcp-controlled-smoke";
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
const SUPPORTED_SCENARIO_USAGE: &str = "smoke|valence-compat-bot-probe|flag-score-repeat|blue-flag-score|inventory-interaction|inventory-stack-split-merge|inventory-drag-transactions|survival-break-place-pickup|survival-chest-persistence|survival-crafting-table|survival-crafting-recipe-breadth|survival-furnace-persistence|survival-furnace-smelting-breadth|survival-hunger-food|survival-hunger-health-cycle|survival-mob-drop|survival-mob-ai-loot-breadth|survival-redstone-toggle|survival-redstone-circuit-breadth|survival-world-persistence-restart|survival-world-multichunk-durability|survival-crash-recovery-parity|survival-block-entity-persistence-parity|survival-container-block-entity-breadth|survival-biome-dimension-state|survival-biome-dimension-travel|survival-sign-editing-live|mcp-controlled-smoke|combat-damage|combat-knockback|vanilla-combat-reference-parity|vanilla-combat-armor-reference-parity|armor-equipment-mitigation|armor-loadout-enchantment-status-matrix|equipment-update-observation|equipment-slot-item-matrix-expansion|projectile-hit|projectile-damage-attribution|flag-carrier-death-return|reconnect-flag-state|reconnect-flag-score|multi-client-load-score|negative-inventory-stale-state|negative-inventory-invalid-click|negative-custom-payload|negative-reconnect-race|negative-ctf-wrong-score|ctf-invalid-pickup-ownership|ctf-invalid-return-drop|ctf-score-limit-win-condition|ctf-simultaneous-pickup-capture-race|ctf-spawn-team-balance-reset";
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
const VANILLA_COMBAT_REFERENCE_CLIENT_COUNT_NEEDLE: &str =
    "mc_compat_vanilla_combat_reference_client_count=2";
const VANILLA_COMBAT_REFERENCE_DAMAGE_NEEDLE: &str = "vanilla_combat_reference_damage";
const VANILLA_COMBAT_REFERENCE_KNOCKBACK_NEEDLE: &str = "vanilla_combat_reference_knockback";
const VANILLA_COMBAT_REFERENCE_PROBE_ENV: &str = "MC_COMPAT_VANILLA_COMBAT_REFERENCE_PROBE";
const VANILLA_COMBAT_ARMOR_REFERENCE_PROBE_ENV: &str =
    "MC_COMPAT_VANILLA_COMBAT_ARMOR_REFERENCE_PROBE";
const VANILLA_COMBAT_ARMOR_REFERENCE_HEALTH_NEEDLE: &str = "update_health health=15.3";
const CTF_SCORE_LIMIT_CLIENT_WIN_NEEDLE: &str = "ctf_score_limit_win_seen score_limit=2 winning_team=red red_score=2 blue_score=0 end_state=winner_declared duplicate_win=false";
const CTF_SCORE_LIMIT_SERVER_PRE_STATE_NEEDLE: &str = "score_limit_pre_state score_limit=2 red_score=1 blue_score=0 next_capture_team=Red outcome=one_capture_before_win";
const CTF_SCORE_LIMIT_SERVER_FINAL_CAPTURE_NEEDLE: &str = "score_limit_final_capture username=compatbot capture_team=Red carried_flag=Blue score_limit=2 red_score_before=1 blue_score_before=0 red_score_after=2 blue_score_after=0";
const CTF_SCORE_LIMIT_SERVER_WIN_NEEDLE: &str = "score_limit_win_condition username=compatbot winning_team=Red score_limit=2 red_score=2 blue_score=0 end_state=winner_declared win_emissions=1 duplicate_win=false post_win_score_delta=0";
const CTF_RACE_CLIENT_COUNT_NEEDLE: &str = "mc_compat_ctf_race_client_count=2";
const CTF_RACE_ACCEPTED_SERVER_NEEDLE: &str = "ctf_race_accepted_transition username=compatbotb player_team=Red flag_team=Blue transition=pickup";
const CTF_RACE_REJECTED_SERVER_NEEDLE: &str = "ctf_race_rejected_transition username=compatbota player_team=Red flag_team=Blue transition=duplicate_pickup";
const CTF_RACE_FINAL_SERVER_NEEDLE: &str = "ctf_race_final_state capture_username=compatbotb accepted_username=compatbotb rejected_username=compatbota capture_team=Red carried_flag=Blue final_blue_flag_state=at_base red_score=1 blue_score=0";
const CTF_SPAWN_TEAM_RESET_CLIENT_COUNT_NEEDLE: &str =
    "mc_compat_ctf_spawn_team_reset_client_count=2";
const CTF_SPAWN_TEAM_RED_ASSIGNMENT_NEEDLE: &str = "ctf_spawn_team_assignment username=compatbota team=Red red_count=1 blue_count=0 spawn_x=-40.0 spawn_y=65.0 spawn_z=0.0 slot36=WoodenSword:1 slot37=RedWool:64";
const CTF_SPAWN_TEAM_BLUE_ASSIGNMENT_NEEDLE: &str = "ctf_spawn_team_assignment username=compatbotb team=Blue red_count=1 blue_count=1 spawn_x=40.0 spawn_y=65.0 spawn_z=0.0 slot36=WoodenSword:1 slot37=BlueWool:64";
const CTF_SPAWN_TEAM_BALANCE_NEEDLE: &str = "ctf_spawn_team_balance red_count=1 blue_count=1 selected_teams=compatbota:Red,compatbotb:Blue outcome=balanced";
const CTF_SPAWN_RESOURCE_RESET_NEEDLE: &str = "ctf_spawn_resource_reset_state trigger=score capture_username=compatbota capture_team=Red carried_flag=Blue red_count=1 blue_count=1 red_score=1 blue_score=0 red_spawn=-40.0,65.0,0.0 blue_spawn=40.0,65.0,0.0 slot36=WoodenSword:1 slot37=TeamWool:64 reset_state=scoreboard_flags_and_resources_coherent";
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

#[derive(Clone, Debug, PartialEq, Eq)]
struct ScenarioEvidence {
    observed_milestones: Vec<&'static str>,
    missing_milestones: Vec<&'static str>,
    forbidden_matches: Vec<&'static str>,
    passed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ServerScenarioEvidence {
    observed_milestones: Vec<&'static str>,
    missing_milestones: Vec<&'static str>,
    forbidden_matches: Vec<&'static str>,
    passed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ProjectileDamageCausalityEvidence {
    required_steps: Vec<&'static str>,
    observed_steps: Vec<&'static str>,
    missing_steps: Vec<&'static str>,
    order_violations: Vec<&'static str>,
    attacker_username: String,
    victim_username: String,
    passed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ArmorLoadoutEnchantmentStatusMatrixEvidence {
    selected: bool,
    row_id: &'static str,
    loadout_id: &'static str,
    equipment_slots: Vec<&'static str>,
    enchantments: Vec<&'static str>,
    status_effects: Vec<&'static str>,
    attack_type: &'static str,
    reference_required: bool,
    reference_receipt: &'static str,
    live_receipt: bool,
    promotion_ready: bool,
    required_client_milestones: Vec<&'static str>,
    observed_client_milestones: Vec<&'static str>,
    required_server_milestones: Vec<&'static str>,
    observed_server_milestones: Vec<&'static str>,
    non_claims: Vec<&'static str>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct EquipmentSlotItemMatrixExpansionEvidence {
    selected: bool,
    row_id: &'static str,
    actor_username: &'static str,
    observer_username: &'static str,
    remote_entity_id: &'static str,
    semantic_slot: &'static str,
    wire_slot: &'static str,
    item_id: &'static str,
    item_count: &'static str,
    transition_kind: &'static str,
    update_order: &'static str,
    reference_required: bool,
    reference_receipt: &'static str,
    live_receipt: bool,
    promotion_ready: bool,
    required_client_milestones: Vec<&'static str>,
    observed_client_milestones: Vec<&'static str>,
    required_server_milestones: Vec<&'static str>,
    observed_server_milestones: Vec<&'static str>,
    non_claims: Vec<&'static str>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct McpControlReceiptEvidence {
    selected: bool,
    endpoint_mode: &'static str,
    handshake_success: bool,
    tool_list_digest: String,
    tool_names: Vec<&'static str>,
    calls_attempted: Vec<&'static str>,
    calls_succeeded: Vec<&'static str>,
    first_failure: Option<&'static str>,
    stdout_clean: bool,
    command_outcome_ids: Vec<&'static str>,
    stevenarella_child_revision: Option<String>,
    revision_status: &'static str,
    dry_run_fixture: bool,
    live_receipt: bool,
    prerequisites: Vec<&'static str>,
    non_claims: Vec<&'static str>,
    passed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct McpControlRunEvidence {
    handshake_success: bool,
    tool_list_digest: String,
    tool_names: Vec<&'static str>,
    calls_attempted: Vec<&'static str>,
    calls_succeeded: Vec<&'static str>,
    first_failure: Option<&'static str>,
    stdout_clean: bool,
    command_outcome_ids: Vec<&'static str>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct FrameArtifactReceiptItem {
    path: String,
    relative_path: String,
    format: String,
    width_px: u32,
    height_px: u32,
    frame_id: u64,
    sequence_id: u64,
    byte_len: u64,
    blake3: String,
    redaction: String,
    includes_ui: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct FrameArtifactsReceiptEvidence {
    selected: bool,
    capture_requested: bool,
    artifact_count: usize,
    artifacts: Vec<FrameArtifactReceiptItem>,
    missing_digests: Vec<&'static str>,
    path_containment_checked: bool,
    promotion_ready: bool,
    non_claims: Vec<&'static str>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct NegativeLiveRailEvidence {
    selected: bool,
    rail: Option<&'static str>,
    invalid_action: Option<&'static str>,
    expected_outcome: Option<&'static str>,
    observed_outcome: Option<&'static str>,
    observed_outcome_source: Option<String>,
    postcondition_milestone: Option<&'static str>,
    telemetry_present: bool,
    target_scope: &'static str,
    owned_local_target: bool,
    explicit_authorization: bool,
    public_target: bool,
    planned_clients: usize,
    max_clients: usize,
    timeout_secs: u64,
    missing_fields: Vec<&'static str>,
    bound_violations: Vec<&'static str>,
    preflight_passed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct NegativeLiveRailInputs {
    selected: bool,
    rail: Option<&'static str>,
    invalid_action: Option<&'static str>,
    expected_outcome: Option<&'static str>,
    observed_outcome: Option<&'static str>,
    observed_outcome_source: Option<String>,
    postcondition_milestone: Option<&'static str>,
    telemetry_required: bool,
    telemetry_present: bool,
    target_scope: &'static str,
    explicit_authorization: bool,
    public_target: bool,
    planned_clients: usize,
    max_clients: usize,
    timeout_secs: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct GitRevisionEvidence {
    requested_rev: Option<String>,
    resolved_rev: Option<String>,
    status: &'static str,
    dirty: bool,
    diagnostics: Vec<String>,
}

impl GitRevisionEvidence {
    fn dry_run(requested_rev: Option<String>) -> Self {
        Self {
            requested_rev,
            resolved_rev: Some(GIT_REV_DRY_RUN_PLACEHOLDER.to_string()),
            status: GIT_STATUS_DRY_RUN,
            dirty: false,
            diagnostics: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ChildRevisionEvidence {
    client: GitRevisionEvidence,
    valence: GitRevisionEvidence,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct LoadNetworkSafetyInputs {
    target_scope: &'static str,
    owned_local_target: bool,
    explicit_authorization: bool,
    public_target: bool,
    planned_clients: usize,
    max_clients: usize,
    duration_secs: u64,
    max_duration_secs: u64,
    reconnect_sessions: usize,
    latency_ms: String,
    jitter_ms: String,
    loss_percent: String,
    telemetry_present: bool,
    live_receipt: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct LoadNetworkSafetyEvidence {
    target_scope: &'static str,
    owned_local_target: bool,
    explicit_authorization: bool,
    public_target: bool,
    authorized: bool,
    planned_clients: usize,
    max_clients: usize,
    duration_secs: u64,
    max_duration_secs: u64,
    reconnect_sessions: usize,
    latency_ms: String,
    jitter_ms: String,
    loss_percent: String,
    telemetry_present: bool,
    live_receipt: bool,
    missing_fields: Vec<&'static str>,
    bound_violations: Vec<&'static str>,
    preflight_passed: bool,
    promotion_ready: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ClientLogSlice<'a> {
    username: &'a str,
    output: &'a str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct EnrichedTriage {
    last_client_event: Option<String>,
    last_server_event: Option<String>,
    correlation_ids: Vec<String>,
    timeline_excerpt: Vec<String>,
    boundary_confidence: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct TypedEvent {
    schema_version: u32,
    source: String,
    scenario: String,
    session: String,
    username: Option<String>,
    sequence: u64,
    kind: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct TypedEventGraphEvaluation {
    observed_events: Vec<String>,
    missing_events: Vec<String>,
    forbidden_events: Vec<String>,
    order_violations: Vec<String>,
    passed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct TypedEventOracleArtifact {
    event_log_path: PathBuf,
    timeline_blake3: String,
    event_count: usize,
    contributes_to_pass_fail: bool,
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

fn evaluate_load_network_safety(input: LoadNetworkSafetyInputs) -> LoadNetworkSafetyEvidence {
    let authorized = input.owned_local_target || input.explicit_authorization;
    let mut missing_fields = Vec::new();
    push_missing_safety_field(
        &mut missing_fields,
        "target_scope",
        !input.target_scope.is_empty(),
    );
    push_missing_safety_field(
        &mut missing_fields,
        "latency_ms",
        !input.latency_ms.is_empty(),
    );
    push_missing_safety_field(
        &mut missing_fields,
        "jitter_ms",
        !input.jitter_ms.is_empty(),
    );
    push_missing_safety_field(
        &mut missing_fields,
        "loss_percent",
        !input.loss_percent.is_empty(),
    );

    let mut bound_violations = Vec::new();
    if input.public_target && !input.explicit_authorization {
        bound_violations.push("public_target_without_authorization");
    }
    if input.planned_clients == 0 {
        bound_violations.push("planned_clients_empty");
    }
    if input.planned_clients > input.max_clients {
        bound_violations.push("planned_clients_exceed_max");
    }
    if input.duration_secs == 0 {
        bound_violations.push("duration_empty");
    }
    if input.duration_secs > input.max_duration_secs {
        bound_violations.push("duration_exceeds_max");
    }

    let preflight_passed = authorized && missing_fields.is_empty() && bound_violations.is_empty();
    let promotion_ready = preflight_passed && input.telemetry_present && input.live_receipt;
    LoadNetworkSafetyEvidence {
        target_scope: input.target_scope,
        owned_local_target: input.owned_local_target,
        explicit_authorization: input.explicit_authorization,
        public_target: input.public_target,
        authorized,
        planned_clients: input.planned_clients,
        max_clients: input.max_clients,
        duration_secs: input.duration_secs,
        max_duration_secs: input.max_duration_secs,
        reconnect_sessions: input.reconnect_sessions,
        latency_ms: input.latency_ms,
        jitter_ms: input.jitter_ms,
        loss_percent: input.loss_percent,
        telemetry_present: input.telemetry_present,
        live_receipt: input.live_receipt,
        missing_fields,
        bound_violations,
        preflight_passed,
        promotion_ready,
    }
}

fn push_missing_safety_field(
    missing_fields: &mut Vec<&'static str>,
    field: &'static str,
    present: bool,
) {
    if !present {
        missing_fields.push(field);
    }
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

fn evaluate_negative_live_rail_safety_from_inputs(
    input: NegativeLiveRailInputs,
) -> NegativeLiveRailEvidence {
    let owned_local_target = !input.public_target;
    let mut missing_fields = Vec::new();
    if input.selected {
        push_missing_safety_field(
            &mut missing_fields,
            "invalid_action",
            input.invalid_action.is_some(),
        );
        push_missing_safety_field(
            &mut missing_fields,
            "expected_outcome",
            input.expected_outcome.is_some(),
        );
        push_missing_safety_field(
            &mut missing_fields,
            "target_scope",
            !input.target_scope.is_empty(),
        );
        push_missing_safety_field(
            &mut missing_fields,
            "postcondition_milestone",
            input.postcondition_milestone.is_some(),
        );
        if input.telemetry_required {
            push_missing_safety_field(
                &mut missing_fields,
                "telemetry",
                input.telemetry_present && input.observed_outcome.is_some(),
            );
        }
    }
    let mut bound_violations = Vec::new();
    if input.selected && input.public_target && !input.explicit_authorization {
        bound_violations.push("public_target_without_authorization");
    }
    if input.selected && input.planned_clients == 0 {
        bound_violations.push("planned_clients_empty");
    }
    if input.selected && input.planned_clients > input.max_clients {
        bound_violations.push("planned_clients_exceed_negative_max");
    }
    if input.selected && input.timeout_secs < NEGATIVE_LIVE_RAIL_MIN_TIMEOUT_SECS {
        bound_violations.push("timeout_empty");
    }
    let preflight_passed = !input.selected
        || ((owned_local_target || input.explicit_authorization)
            && missing_fields.is_empty()
            && bound_violations.is_empty());
    NegativeLiveRailEvidence {
        selected: input.selected,
        rail: input.rail,
        invalid_action: input.invalid_action,
        expected_outcome: input.expected_outcome,
        observed_outcome: input.observed_outcome,
        observed_outcome_source: input.observed_outcome_source,
        postcondition_milestone: input.postcondition_milestone,
        telemetry_present: input.telemetry_present,
        target_scope: input.target_scope,
        owned_local_target,
        explicit_authorization: input.explicit_authorization,
        public_target: input.public_target,
        planned_clients: input.planned_clients,
        max_clients: input.max_clients,
        timeout_secs: input.timeout_secs,
        missing_fields,
        bound_violations,
        preflight_passed,
    }
}

fn evaluate_negative_live_rail_safety(cfg: &Config) -> NegativeLiveRailEvidence {
    evaluate_negative_live_rail_safety_from_inputs(negative_live_rail_inputs_from_config(
        cfg, None, false,
    ))
}

fn evaluate_armor_loadout_enchantment_status_matrix(
    cfg: &Config,
    scenario: &ScenarioEvidence,
    server_scenario: &ServerScenarioEvidence,
) -> ArmorLoadoutEnchantmentStatusMatrixEvidence {
    let selected = cfg.scenario == Scenario::ArmorLoadoutEnchantmentStatusMatrix;
    let observed_live_evidence =
        selected && cfg.mode == Mode::Run && scenario.passed && server_scenario.passed;
    ArmorLoadoutEnchantmentStatusMatrixEvidence {
        selected,
        row_id: ARMOR_MATRIX_ROW_ID,
        loadout_id: ARMOR_MATRIX_LOADOUT_ID,
        equipment_slots: vec![ARMOR_MATRIX_EQUIPMENT_SLOT],
        enchantments: vec![ARMOR_MATRIX_ENCHANTMENT_NONE],
        status_effects: vec![ARMOR_MATRIX_STATUS_EFFECT_NONE],
        attack_type: ARMOR_MATRIX_ATTACK_TYPE_MELEE,
        reference_required: false,
        reference_receipt: ARMOR_MATRIX_REFERENCE_RECEIPT_NONE,
        live_receipt: observed_live_evidence,
        promotion_ready: observed_live_evidence,
        required_client_milestones: scenario_required_milestones(
            Scenario::ArmorLoadoutEnchantmentStatusMatrix,
        )
        .iter()
        .map(|(name, _)| *name)
        .collect(),
        observed_client_milestones: scenario.observed_milestones.clone(),
        required_server_milestones: server_required_milestones(
            Scenario::ArmorLoadoutEnchantmentStatusMatrix,
        )
        .iter()
        .map(|(name, _)| *name)
        .collect(),
        observed_server_milestones: server_scenario.observed_milestones.clone(),
        non_claims: ARMOR_MATRIX_NON_CLAIMS.to_vec(),
    }
}

fn evaluate_equipment_slot_item_matrix_expansion(
    cfg: &Config,
    scenario: &ScenarioEvidence,
    server_scenario: &ServerScenarioEvidence,
) -> EquipmentSlotItemMatrixExpansionEvidence {
    let selected = cfg.scenario == Scenario::EquipmentSlotItemMatrixExpansion;
    let observed_live_evidence =
        selected && cfg.mode == Mode::Run && scenario.passed && server_scenario.passed;
    EquipmentSlotItemMatrixExpansionEvidence {
        selected,
        row_id: EQUIPMENT_MATRIX_ROW_ID,
        actor_username: EQUIPMENT_MATRIX_ACTOR,
        observer_username: EQUIPMENT_MATRIX_OBSERVER,
        remote_entity_id: EQUIPMENT_MATRIX_REMOTE_ENTITY_ID,
        semantic_slot: EQUIPMENT_MATRIX_SEMANTIC_SLOT,
        wire_slot: EQUIPMENT_MATRIX_WIRE_SLOT,
        item_id: EQUIPMENT_MATRIX_ITEM_ID,
        item_count: EQUIPMENT_MATRIX_ITEM_COUNT,
        transition_kind: EQUIPMENT_MATRIX_TRANSITION,
        update_order: EQUIPMENT_MATRIX_UPDATE_ORDER,
        reference_required: false,
        reference_receipt: EQUIPMENT_MATRIX_REFERENCE_RECEIPT_NONE,
        live_receipt: observed_live_evidence,
        promotion_ready: observed_live_evidence,
        required_client_milestones: scenario_required_milestones(
            Scenario::EquipmentSlotItemMatrixExpansion,
        )
        .iter()
        .map(|(name, _)| *name)
        .collect(),
        observed_client_milestones: scenario.observed_milestones.clone(),
        required_server_milestones: server_required_milestones(
            Scenario::EquipmentSlotItemMatrixExpansion,
        )
        .iter()
        .map(|(name, _)| *name)
        .collect(),
        observed_server_milestones: server_scenario.observed_milestones.clone(),
        non_claims: EQUIPMENT_MATRIX_NON_CLAIMS.to_vec(),
    }
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

impl Config {
    fn defaults(root: PathBuf) -> Self {
        Config {
            client_dir: root.join("stevenarella"),
            valence_repo: root.join("valence"),
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
            root,
        }
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
        let root = get_env("MC_COMPAT_ROOT")
            .or_else(|| get_env("ROOT"))
            .map(PathBuf::from)
            .unwrap_or(current_dir);
        let mut cfg = Config::defaults(root);

        let config_path = find_config_path(get_env("MC_COMPAT_CONFIG"), &args_vec)?;
        let steel_config_path = find_named_config_path(
            "--steel-config",
            "MC_COMPAT_STEEL_CONFIG",
            get_env("MC_COMPAT_STEEL_CONFIG"),
            &args_vec,
        )?;
        let mut server_port_was_set = false;
        if let Some(path) = config_path {
            server_port_was_set |= apply_config_file(&mut cfg, &path)?;
            cfg.config_path = Some(path);
        }
        if let Some(path) = steel_config_path {
            server_port_was_set |= apply_steel_config_file(&mut cfg, &path)?;
            cfg.steel_config_path = Some(path);
        }

        apply_env_overrides(&mut cfg, &mut get_env, &mut server_port_was_set)?;

        let mut args = args_vec.into_iter();
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--dry-run" => {
                    if cfg.mode == Mode::RunMatrix {
                        cfg.matrix_dry_run = true;
                    } else if cfg.mode == Mode::Cleanup {
                        cfg.cleanup_apply = false;
                    } else {
                        cfg.mode = Mode::DryRun;
                    }
                }
                "--run" => cfg.mode = Mode::Run,
                "--run-matrix" => {
                    cfg.mode = Mode::RunMatrix;
                    cfg.matrix_dry_run = false;
                }
                "--build-client" => cfg.mode = Mode::BuildClient,
                "--status-only" => cfg.mode = Mode::StatusOnly,
                "--status" => cfg.mode = Mode::HarnessStatus,
                "--cleanup" => cfg.mode = Mode::Cleanup,
                "--apply" => cfg.cleanup_apply = true,
                "--stop" => cfg.mode = Mode::Stop,
                "--config" => {
                    let path = PathBuf::from(args.next().ok_or_else(|| {
                        "--config requires a Nickel-exported JSON path".to_string()
                    })?);
                    server_port_was_set |= apply_config_file(&mut cfg, &path)?;
                    cfg.config_path = Some(path);
                }
                "--steel-config" => {
                    let path = PathBuf::from(args.next().ok_or_else(|| {
                        "--steel-config requires a Steel module path".to_string()
                    })?);
                    server_port_was_set |= apply_steel_config_file(&mut cfg, &path)?;
                    cfg.steel_config_path = Some(path);
                }
                "--compare-receipts" => {
                    let left = PathBuf::from(args.next().ok_or_else(|| {
                        "--compare-receipts requires PAPER_RECEIPT and VALENCE_RECEIPT".to_string()
                    })?);
                    let right = PathBuf::from(args.next().ok_or_else(|| {
                        "--compare-receipts requires PAPER_RECEIPT and VALENCE_RECEIPT".to_string()
                    })?);
                    cfg.mode = Mode::CompareReceipts;
                    cfg.compare_receipts = Some((left, right));
                }
                "--accept-eula" => {}
                "--keep-server" => cfg.keep_server = true,
                "--server-backend" => {
                    let value = args
                        .next()
                        .ok_or_else(|| "--server-backend requires valence or paper".to_string())?;
                    cfg.server_backend = parse_backend(&value)?;
                }
                "--client-dir" => {
                    cfg.client_dir = PathBuf::from(
                        args.next()
                            .ok_or_else(|| "--client-dir requires a path".to_string())?,
                    );
                }
                "--receipt" => {
                    cfg.receipt_path = Some(PathBuf::from(
                        args.next()
                            .ok_or_else(|| "--receipt requires a path".to_string())?,
                    ));
                }
                "--receipt-dir" => {
                    cfg.receipt_dir =
                        Some(PathBuf::from(args.next().ok_or_else(|| {
                            "--receipt-dir requires a path".to_string()
                        })?));
                }
                "--failure-bundle" => {
                    cfg.failure_bundle_path =
                        Some(PathBuf::from(args.next().ok_or_else(|| {
                            "--failure-bundle requires a path".to_string()
                        })?));
                }
                "--scenario" => {
                    let value = args.next().ok_or_else(|| {
                        format!("--scenario requires one of: {SUPPORTED_SCENARIO_USAGE}")
                    })?;
                    cfg.scenario = parse_scenario(&value)?;
                }
                "--expect-status-description" => {
                    cfg.expected_status_description = Some(args.next().ok_or_else(|| {
                        "--expect-status-description requires a string".to_string()
                    })?);
                }
                "--expect-status-version" => {
                    cfg.expected_status_version_name =
                        Some(args.next().ok_or_else(|| {
                            "--expect-status-version requires a string".to_string()
                        })?);
                }
                "--expect-status-sample" => {
                    cfg.expected_status_sample = args
                        .next()
                        .ok_or_else(|| {
                            "--expect-status-sample requires comma-separated names".to_string()
                        })?
                        .split(',')
                        .filter(|value| !value.is_empty())
                        .map(str::to_string)
                        .collect();
                }
                "--packet-capture-summary" => cfg.packet_capture_summary = true,
                "--proxy-route" => {
                    cfg.proxy_route = Some(
                        args.next()
                            .ok_or_else(|| "--proxy-route requires a route label".to_string())?,
                    );
                }
                "--proxy-forwarding-mode" => {
                    cfg.proxy_forwarding_mode = Some(args.next().ok_or_else(|| {
                        "--proxy-forwarding-mode requires a mode label".to_string()
                    })?);
                }
                "--valence-repo" => {
                    cfg.valence_repo = PathBuf::from(
                        args.next()
                            .ok_or_else(|| "--valence-repo requires a path".to_string())?,
                    );
                }
                "--valence-rev" => {
                    cfg.valence_rev = args
                        .next()
                        .ok_or_else(|| "--valence-rev requires a git revision".to_string())?;
                }
                "-h" | "--help" => {
                    print_usage(&cfg);
                    std::process::exit(0);
                }
                _ if arg.starts_with("--config=") => {
                    let path = PathBuf::from(&arg[9..]);
                    server_port_was_set |= apply_config_file(&mut cfg, &path)?;
                    cfg.config_path = Some(path);
                }
                _ if arg.starts_with("--steel-config=") => {
                    let path = PathBuf::from(&arg[15..]);
                    server_port_was_set |= apply_steel_config_file(&mut cfg, &path)?;
                    cfg.steel_config_path = Some(path);
                }
                _ if arg.starts_with("--server-backend=") => {
                    cfg.server_backend = parse_backend(&arg[17..])?;
                }
                _ if arg.starts_with("--client-dir=") => {
                    cfg.client_dir = PathBuf::from(&arg[13..]);
                }
                _ if arg.starts_with("--receipt=") => {
                    cfg.receipt_path = Some(PathBuf::from(&arg[10..]));
                }
                _ if arg.starts_with("--receipt-dir=") => {
                    cfg.receipt_dir = Some(PathBuf::from(&arg[14..]));
                }
                _ if arg.starts_with("--failure-bundle=") => {
                    cfg.failure_bundle_path = Some(PathBuf::from(&arg[17..]));
                }
                _ if arg.starts_with("--scenario=") => {
                    cfg.scenario = parse_scenario(&arg[11..])?;
                }
                _ if arg.starts_with("--expect-status-description=") => {
                    cfg.expected_status_description = Some(arg[28..].to_string());
                }
                _ if arg.starts_with("--expect-status-version=") => {
                    cfg.expected_status_version_name = Some(arg[24..].to_string());
                }
                _ if arg.starts_with("--expect-status-sample=") => {
                    cfg.expected_status_sample = arg[23..]
                        .split(',')
                        .filter(|value| !value.is_empty())
                        .map(str::to_string)
                        .collect();
                }
                _ if arg == "--packet-capture-summary" => {
                    cfg.packet_capture_summary = true;
                }
                _ if arg.starts_with("--proxy-route=") => {
                    cfg.proxy_route = Some(arg[14..].to_string());
                }
                _ if arg.starts_with("--proxy-forwarding-mode=") => {
                    cfg.proxy_forwarding_mode = Some(arg[24..].to_string());
                }
                _ if arg.starts_with("--valence-repo=") => {
                    cfg.valence_repo = PathBuf::from(&arg[15..]);
                }
                _ if arg.starts_with("--valence-rev=") => {
                    cfg.valence_rev = arg[14..].to_string();
                }
                _ => return Err(format!("unknown arg: {arg}")),
            }
        }

        if !server_port_was_set {
            cfg.server_port = default_port(cfg.server_backend);
        }
        if cfg.mode == Mode::RunMatrix && cfg.receipt_path.is_some() {
            return Err("--run-matrix writes backend receipts under --receipt-dir; do not combine it with --receipt/SMOKE_RECEIPT".to_string());
        }
        Ok(cfg)
    }
}

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

fn apply_env_overrides<F>(
    cfg: &mut Config,
    get_env: &mut F,
    server_port_was_set: &mut bool,
) -> Result<(), String>
where
    F: FnMut(&str) -> Option<String>,
{
    if let Some(value) = get_env("CLIENT_DIR") {
        cfg.client_dir = PathBuf::from(value);
    }
    if let Some(value) = get_env("VALENCE_REPO") {
        cfg.valence_repo = PathBuf::from(value);
    }
    if let Some(value) = get_env("VALENCE_REV") {
        cfg.valence_rev = value;
    }
    if let Some(value) = get_env("VALENCE_WORKTREE") {
        cfg.valence_worktree = PathBuf::from(value);
    }
    if let Some(value) = get_env("VALENCE_EXAMPLE") {
        cfg.valence_example = value;
    }
    if let Some(value) = get_env("VALENCE_LOG") {
        cfg.valence_log = PathBuf::from(value);
    }
    if let Some(value) = get_env("VALENCE_TARGET_DIR") {
        cfg.valence_target_dir = PathBuf::from(value);
    }
    if let Some(value) = get_env("VALENCE_PID_FILE") {
        cfg.valence_pid_file = PathBuf::from(value);
    }
    if let Some(value) = get_env("SERVER_BACKEND") {
        cfg.server_backend = parse_backend(&value)?;
    }
    if let Some(value) = get_env("TARGET_DIR") {
        cfg.target_dir = PathBuf::from(value);
    }
    if let Some(value) = get_env("SERVER_NAME") {
        cfg.server_name = value;
    }
    if let Some(value) = get_env("SERVER_VERSION") {
        cfg.server_version = value;
    }
    if let Some(value) = get_env("SERVER_PROTOCOL") {
        cfg.server_protocol = value
            .parse()
            .map_err(|e| format!("parse SERVER_PROTOCOL: {e}"))?;
    }
    if let Some(value) = get_env("SERVER_PORT") {
        cfg.server_port = value
            .parse()
            .map_err(|e| format!("parse SERVER_PORT: {e}"))?;
        *server_port_was_set = true;
    }
    if let Some(value) = get_env("CLIENT_USERNAME") {
        cfg.client_username = value;
    }
    if let Some(value) = get_env("DOCKER_IMAGE") {
        cfg.docker_image = value;
    }
    if let Some(value) = get_env("PAPER_PLUGIN_JAR") {
        cfg.paper_plugin_jar = Some(PathBuf::from(value));
    }
    if let Some(value) = get_env("CLIENT_TIMEOUT") {
        cfg.client_timeout = Duration::from_secs(
            value
                .parse()
                .map_err(|e| format!("parse CLIENT_TIMEOUT: {e}"))?,
        );
    }
    if let Some(value) = get_env("CLIENT_SUCCESS_PATTERN") {
        cfg.client_success_needles = value.split('|').map(str::to_string).collect();
    }
    if let Some(value) = get_env("MC_COMPAT_SCENARIO") {
        cfg.scenario = parse_scenario(&value)?;
    }
    if let Some(value) = get_env("MC_COMPAT_EXPECT_STATUS_DESCRIPTION") {
        cfg.expected_status_description = Some(value);
    }
    if let Some(value) = get_env("MC_COMPAT_EXPECT_STATUS_VERSION") {
        cfg.expected_status_version_name = Some(value);
    }
    if let Some(value) = get_env("MC_COMPAT_EXPECT_STATUS_SAMPLE") {
        cfg.expected_status_sample = value
            .split(',')
            .filter(|sample| !sample.is_empty())
            .map(str::to_string)
            .collect();
    }
    if get_env("MC_COMPAT_PACKET_CAPTURE_SUMMARY").is_some() {
        cfg.packet_capture_summary = true;
    }
    if let Some(value) = get_env("MC_COMPAT_PUBLIC_TARGET") {
        cfg.negative_public_target = value == "1";
    }
    if let Some(value) = get_env("MC_COMPAT_EXTERNAL_LOAD_AUTHORIZED") {
        cfg.negative_external_authorized = value == "1";
    }
    if let Some(value) = get_env("MC_COMPAT_PROXY_ROUTE") {
        cfg.proxy_route = Some(value);
    }
    if let Some(value) = get_env("MC_COMPAT_PROXY_FORWARDING_MODE") {
        cfg.proxy_forwarding_mode = Some(value);
    }
    if let Some(value) = get_env("SMOKE_RECEIPT") {
        cfg.receipt_path = Some(PathBuf::from(value));
    }
    if let Some(value) = get_env("SMOKE_RECEIPT_DIR") {
        cfg.receipt_dir = Some(PathBuf::from(value));
    }
    if let Some(value) = get_env("MC_COMPAT_FAILURE_BUNDLE") {
        cfg.failure_bundle_path = Some(PathBuf::from(value));
    }
    Ok(())
}

fn apply_config_file(cfg: &mut Config, path: &Path) -> Result<bool, String> {
    let text =
        fs::read_to_string(path).map_err(|e| format!("read config {}: {e}", path.display()))?;
    apply_config_json(cfg, &text).map_err(|e| format!("config {}: {e}", path.display()))
}

fn apply_steel_config_file(cfg: &mut Config, path: &Path) -> Result<bool, String> {
    let text = fs::read_to_string(path)
        .map_err(|e| format!("read Steel config {}: {e}", path.display()))?;
    let source = runtime_config::SteelSource {
        path: path.display().to_string(),
        module_blake3: "runtime-unverified".to_string(),
        sandbox_profile: "mc-compat/pure-v1".to_string(),
    };
    let snapshot = runtime_config::evaluate_steel_module(source, &text).map_err(|diagnostics| {
        let details = diagnostics
            .into_iter()
            .map(|diagnostic| format!("{}: {}", diagnostic.path, diagnostic.message))
            .collect::<Vec<_>>()
            .join("; ");
        format!("Steel config {}: {details}", path.display())
    })?;
    cfg.server_backend = parse_backend(&snapshot.server_backend)?;
    cfg.server_version = snapshot.server_version;
    cfg.server_protocol = snapshot.server_protocol;
    cfg.server_port = snapshot.server_port;
    cfg.valence_rev = snapshot.valence_rev;
    cfg.valence_example = snapshot.valence_example;
    cfg.valence_worktree = PathBuf::from(snapshot.valence_worktree);
    cfg.valence_target_dir = PathBuf::from(snapshot.valence_target_dir);
    cfg.valence_log = PathBuf::from(snapshot.valence_log);
    cfg.valence_pid_file = PathBuf::from(snapshot.valence_pid_file);
    cfg.client_username = snapshot.client_username;
    cfg.client_timeout = Duration::from_secs(u64::from(snapshot.client_timeout_secs));
    cfg.client_success_needles = snapshot.client_success_patterns;
    cfg.receipt_dir = Some(PathBuf::from(snapshot.receipt_dir));
    cfg.scenario = parse_scenario(&snapshot.scenario)?;
    cfg.arrow_damage_policy = snapshot.arrow_damage;
    Ok(true)
}

fn apply_config_json(cfg: &mut Config, text: &str) -> Result<bool, String> {
    let mut server_port_was_set = false;
    if let Some(value) = json_optional_string_field(text, "client_dir")? {
        cfg.client_dir = PathBuf::from(value);
    }
    if let Some(value) = json_optional_string_field(text, "valence_repo")? {
        cfg.valence_repo = PathBuf::from(value);
    }
    if let Some(value) = json_optional_string_field(text, "valence_rev")? {
        cfg.valence_rev = value;
    }
    if let Some(value) = json_optional_string_field(text, "valence_worktree")? {
        cfg.valence_worktree = PathBuf::from(value);
    }
    if let Some(value) = json_optional_string_field(text, "valence_example")? {
        cfg.valence_example = value;
    }
    if let Some(value) = json_optional_string_field(text, "valence_log")? {
        cfg.valence_log = PathBuf::from(value);
    }
    if let Some(value) = json_optional_string_field(text, "valence_target_dir")? {
        cfg.valence_target_dir = PathBuf::from(value);
    }
    if let Some(value) = json_optional_string_field(text, "valence_pid_file")? {
        cfg.valence_pid_file = PathBuf::from(value);
    }
    if let Some(value) = json_optional_string_field(text, "server_backend")? {
        cfg.server_backend = parse_backend(&value)?;
        cfg.server_port = default_port(cfg.server_backend);
    }
    if let Some(value) = json_optional_string_field(text, "target_dir")? {
        cfg.target_dir = PathBuf::from(value);
    }
    if let Some(value) = json_optional_string_field(text, "server_name")? {
        cfg.server_name = value;
    }
    if let Some(value) = json_optional_string_field(text, "server_version")? {
        cfg.server_version = value;
    }
    if let Some(value) = json_optional_u32_field(text, "server_protocol")? {
        cfg.server_protocol = value;
    }
    if let Some(value) = json_optional_u32_field(text, "server_port")? {
        cfg.server_port =
            u16::try_from(value).map_err(|_| format!("server_port {value} exceeds u16"))?;
        server_port_was_set = true;
    }
    if let Some(value) = json_optional_string_field(text, "client_username")? {
        cfg.client_username = value;
    }
    if let Some(value) = json_optional_string_field(text, "docker_image")? {
        cfg.docker_image = value;
    }
    if let Some(value) = json_optional_string_field(text, "paper_plugin_jar")? {
        cfg.paper_plugin_jar = Some(PathBuf::from(value));
    }
    if let Some(value) = json_optional_u32_field(text, "client_timeout_secs")? {
        cfg.client_timeout = Duration::from_secs(u64::from(value));
    }
    if let Some(value) = json_optional_string_array_field(text, "client_success_patterns")? {
        cfg.client_success_needles = value;
    }
    if let Some(value) = json_optional_string_field(text, "scenario")? {
        cfg.scenario = parse_scenario(&value)?;
    }
    if let Some(value) = json_optional_string_field(text, "expected_status_description")? {
        cfg.expected_status_description = Some(value);
    }
    if let Some(value) = json_optional_string_field(text, "expected_status_version_name")? {
        cfg.expected_status_version_name = Some(value);
    }
    if let Some(value) = json_optional_string_array_field(text, "expected_status_sample")? {
        cfg.expected_status_sample = value;
    }
    if let Some(value) = json_optional_bool_field(text, "packet_capture_summary")? {
        cfg.packet_capture_summary = value;
    }
    if let Some(value) = json_optional_string_field(text, "proxy_route")? {
        cfg.proxy_route = Some(value);
    }
    if let Some(value) = json_optional_string_field(text, "proxy_forwarding_mode")? {
        cfg.proxy_forwarding_mode = Some(value);
    }
    if let Some(value) = json_optional_string_field(text, "receipt_path")? {
        cfg.receipt_path = Some(PathBuf::from(value));
    }
    if let Some(value) = json_optional_string_field(text, "receipt_dir")? {
        cfg.receipt_dir = Some(PathBuf::from(value));
    }
    if let Some(value) = json_optional_string_field(text, "failure_bundle_path")? {
        cfg.failure_bundle_path = Some(PathBuf::from(value));
    }
    Ok(server_port_was_set)
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
    fn apply_client_probe_env(
        &self,
        cmd: &mut Command,
        client_index: usize,
        server_backend: ServerBackend,
    );
    fn apply_valence_server_env(&self, cmd: &mut Command, cfg: &Config);
    fn apply_paper_server_env(&self, cmd: &mut Command, cfg: &Config) -> Result<(), String>;
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
        match self {
            ScenarioBehaviorKind::ReconnectFlagState { .. }
            | ScenarioBehaviorKind::SurvivalChestPersistence
            | ScenarioBehaviorKind::SurvivalFurnacePersistence
            | ScenarioBehaviorKind::WorldPersistenceRestart { .. }
            | ScenarioBehaviorKind::SurvivalWorldMultichunkDurability => {
                ScenarioRunStrategy::ReconnectSequence
            }
            ScenarioBehaviorKind::Combat { .. }
            | ScenarioBehaviorKind::EquipmentUpdate
            | ScenarioBehaviorKind::Projectile { .. }
            | ScenarioBehaviorKind::MultiClientLoadScore
            | ScenarioBehaviorKind::CtfSimultaneousPickupCaptureRace
            | ScenarioBehaviorKind::CtfSpawnTeamBalanceReset => ScenarioRunStrategy::MultiClient,
            _ => ScenarioRunStrategy::SingleClient,
        }
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
        match self {
            ScenarioBehaviorKind::NegativeInventory {
                invalid_action,
                postcondition,
                ..
            } => Some(NegativeLiveRailBehavior {
                invalid_action,
                postcondition,
            }),
            ScenarioBehaviorKind::NegativeCustomPayload => Some(NegativeLiveRailBehavior {
                invalid_action: "malformed_custom_payload",
                postcondition: "negative_custom_payload_contained",
            }),
            ScenarioBehaviorKind::ReconnectFlagState {
                negative_probe: Some(_),
            } => Some(NegativeLiveRailBehavior {
                invalid_action: "duplicate_reconnect_flag_transition",
                postcondition: "negative_reconnect_race_contained",
            }),
            ScenarioBehaviorKind::NegativeCtfWrongScore => Some(NegativeLiveRailBehavior {
                invalid_action: "wrong_team_or_wrong_portal_score_attempt",
                postcondition: "negative_wrong_score_contained",
            }),
            ScenarioBehaviorKind::CtfInvalidPickupOwnership => Some(NegativeLiveRailBehavior {
                invalid_action: "own_flag_pickup_without_ownership_transfer",
                postcondition: "ctf_invalid_pickup_contained",
            }),
            ScenarioBehaviorKind::CtfInvalidReturnDrop => Some(NegativeLiveRailBehavior {
                invalid_action: "own_base_return_without_carrier",
                postcondition: "ctf_invalid_return_drop_contained",
            }),
            _ => None,
        }
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
                | ScenarioBehaviorKind::SurvivalBiomeDimensionTravel
                | ScenarioBehaviorKind::SurvivalSignEditingLive
                | ScenarioBehaviorKind::Combat { .. }
                | ScenarioBehaviorKind::EquipmentUpdate
                | ScenarioBehaviorKind::Projectile { .. }
                | ScenarioBehaviorKind::MultiClientLoadScore
                | ScenarioBehaviorKind::CtfInvalidPickupOwnership
                | ScenarioBehaviorKind::CtfInvalidReturnDrop
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
        matches!(self, ScenarioBehaviorKind::Projectile { damage: true })
    }

    fn is_mcp_controlled_smoke(&self) -> bool {
        matches!(self, ScenarioBehaviorKind::McpControlledSmoke)
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

    fn apply_client_probe_env(
        &self,
        cmd: &mut Command,
        client_index: usize,
        _server_backend: ServerBackend,
    ) {
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
                    cmd.env("MC_COMPAT_NEGATIVE_PROBE", probe);
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
                    .env("MC_COMPAT_NEGATIVE_PROBE", probe);
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
                        session_env_value(client_index),
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
                        session_env_value(client_index),
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
                        session_env_value(FIRST_CLIENT_INDEX),
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
                            session_env_value(client_index),
                        );
                } else {
                    cmd.env(SURVIVAL_WORLD_PERSISTENCE_PROBE_ENV, PROBE_ENABLED_VALUE)
                        .env(
                            SURVIVAL_WORLD_PERSISTENCE_SESSION_ENV,
                            session_env_value(client_index),
                        );
                }
            }
            ScenarioBehaviorKind::SurvivalWorldMultichunkDurability => {
                cmd.env(SURVIVAL_WORLD_MULTICHUNK_PROBE_ENV, PROBE_ENABLED_VALUE)
                    .env(
                        SURVIVAL_WORLD_MULTICHUNK_SESSION_ENV,
                        session_env_value(client_index),
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
    }

    fn apply_valence_server_env(&self, cmd: &mut Command, cfg: &Config) {
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
    }

    fn apply_paper_server_env(&self, cmd: &mut Command, cfg: &Config) -> Result<(), String> {
        match self {
            ScenarioBehaviorKind::SurvivalChestPersistence => {
                add_paper_env(cmd, SURVIVAL_CHEST_FIXTURE_ENV, PROBE_ENABLED_VALUE)
            }
            ScenarioBehaviorKind::SurvivalCraftingTable => {
                add_paper_env(cmd, SURVIVAL_CRAFTING_FIXTURE_ENV, PROBE_ENABLED_VALUE)
            }
            ScenarioBehaviorKind::SurvivalCraftingRecipeBreadth => add_paper_env(
                cmd,
                SURVIVAL_CRAFTING_BREADTH_FIXTURE_ENV,
                PROBE_ENABLED_VALUE,
            ),
            ScenarioBehaviorKind::SurvivalFurnacePersistence => {
                add_paper_env(cmd, SURVIVAL_FURNACE_FIXTURE_ENV, PROBE_ENABLED_VALUE)
            }
            ScenarioBehaviorKind::SurvivalFurnaceSmeltingBreadth => {
                add_paper_env(cmd, SURVIVAL_FURNACE_FIXTURE_ENV, PROBE_ENABLED_VALUE);
                add_paper_env(
                    cmd,
                    SURVIVAL_FURNACE_SMELTING_BREADTH_FIXTURE_ENV,
                    PROBE_ENABLED_VALUE,
                );
            }
            ScenarioBehaviorKind::SurvivalHungerFood => {
                add_paper_env(cmd, SURVIVAL_HUNGER_FOOD_FIXTURE_ENV, PROBE_ENABLED_VALUE)
            }
            ScenarioBehaviorKind::SurvivalHungerHealthCycle => {
                add_paper_env(cmd, SURVIVAL_HUNGER_HEALTH_FIXTURE_ENV, PROBE_ENABLED_VALUE)
            }
            ScenarioBehaviorKind::SurvivalMobDrop => {
                add_paper_env(cmd, SURVIVAL_MOB_DROP_FIXTURE_ENV, PROBE_ENABLED_VALUE)
            }
            ScenarioBehaviorKind::SurvivalMobAiLootBreadth => {
                add_paper_env(cmd, SURVIVAL_MOB_AI_LOOT_FIXTURE_ENV, PROBE_ENABLED_VALUE)
            }
            ScenarioBehaviorKind::SurvivalRedstoneToggle => add_paper_env(
                cmd,
                SURVIVAL_REDSTONE_TOGGLE_FIXTURE_ENV,
                PROBE_ENABLED_VALUE,
            ),
            ScenarioBehaviorKind::SurvivalRedstoneCircuitBreadth => add_paper_env(
                cmd,
                SURVIVAL_REDSTONE_CIRCUIT_FIXTURE_ENV,
                PROBE_ENABLED_VALUE,
            ),
            ScenarioBehaviorKind::WorldPersistenceRestart { block_entity, .. } => {
                if *block_entity {
                    add_paper_persistence_env(
                        cmd,
                        cfg,
                        SURVIVAL_BLOCK_ENTITY_FIXTURE_ENV,
                        SURVIVAL_BLOCK_ENTITY_PHASE_ENV,
                    )?;
                } else {
                    add_paper_persistence_env(
                        cmd,
                        cfg,
                        SURVIVAL_WORLD_PERSISTENCE_FIXTURE_ENV,
                        SURVIVAL_WORLD_PERSISTENCE_PHASE_ENV,
                    )?;
                }
            }
            ScenarioBehaviorKind::SurvivalWorldMultichunkDurability => add_paper_persistence_env(
                cmd,
                cfg,
                SURVIVAL_WORLD_MULTICHUNK_FIXTURE_ENV,
                SURVIVAL_WORLD_MULTICHUNK_PHASE_ENV,
            )?,
            ScenarioBehaviorKind::SurvivalContainerBlockEntityBreadth => add_paper_env(
                cmd,
                SURVIVAL_CONTAINER_BLOCK_ENTITY_FIXTURE_ENV,
                PROBE_ENABLED_VALUE,
            ),
            ScenarioBehaviorKind::SurvivalBiomeDimensionState => add_paper_env(
                cmd,
                SURVIVAL_BIOME_DIMENSION_FIXTURE_ENV,
                PROBE_ENABLED_VALUE,
            ),
            ScenarioBehaviorKind::SurvivalBiomeDimensionTravel => add_paper_env(
                cmd,
                SURVIVAL_BIOME_DIMENSION_TRAVEL_FIXTURE_ENV,
                PROBE_ENABLED_VALUE,
            ),
            ScenarioBehaviorKind::SurvivalSignEditingLive => {
                add_paper_env(cmd, SURVIVAL_SIGN_EDITING_FIXTURE_ENV, PROBE_ENABLED_VALUE)
            }
            ScenarioBehaviorKind::Combat {
                reference_probe,
                armor_reference,
                ..
            } => {
                if *reference_probe {
                    add_paper_env(cmd, VANILLA_COMBAT_REFERENCE_PROBE_ENV, PROBE_ENABLED_VALUE);
                }
                if *armor_reference {
                    add_paper_env(
                        cmd,
                        VANILLA_COMBAT_ARMOR_REFERENCE_PROBE_ENV,
                        PROBE_ENABLED_VALUE,
                    );
                }
            }
            _ => {}
        }
        Ok(())
    }
}

fn append_count_marker(output: &mut String, marker: &'static str) {
    output.push_str(marker);
    output.push('\n');
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

fn add_paper_env(cmd: &mut Command, key: &'static str, value: &'static str) {
    cmd.arg("-e").arg(format!("{key}={value}"));
}

fn add_paper_persistence_env(
    cmd: &mut Command,
    cfg: &Config,
    fixture_env: &'static str,
    phase_env: &'static str,
) -> Result<(), String> {
    let state_dir = world_persistence_state_dir(cfg, ServerBackend::Paper);
    fs::create_dir_all(&state_dir).map_err(|e| format!("create {}: {e}", state_dir.display()))?;
    let absolute_state_dir = fs::canonicalize(&state_dir)
        .map_err(|e| format!("canonicalize {}: {e}", state_dir.display()))?;
    add_paper_env(cmd, fixture_env, PROBE_ENABLED_VALUE);
    cmd.arg("-e")
        .arg(format!(
            "{phase_env}={}",
            world_persistence_phase_value(cfg)
        ))
        .arg("-v")
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
fn evaluate_scenario(scenario: Scenario, output: &str) -> ScenarioEvidence {
    evaluate_scenario_with_projectile_health(
        scenario,
        output,
        PROJECTILE_DAMAGE_CLIENT_HEALTH_NEEDLE,
    )
}

fn evaluate_scenario_for_config(cfg: &Config, output: &str) -> ScenarioEvidence {
    let health_needle = projectile_damage_client_health_needle(cfg);
    evaluate_scenario_with_projectile_health(cfg.scenario, output, &health_needle)
}

fn evaluate_scenario_with_projectile_health(
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

fn evaluate_server_scenario(
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

#[cfg(test)]
fn parse_typed_event_line(line: &str) -> Result<TypedEvent, String> {
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

fn evaluate_typed_event_graph(
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

fn typed_events_from_receipt_evidence(
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

fn normalize_typed_event_timeline(events: &[TypedEvent]) -> String {
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

fn typed_event_timeline_blake3(timeline: &str) -> String {
    blake3::hash(timeline.as_bytes()).to_hex().to_string()
}

fn typed_event_oracle_receipt_json(artifact: Option<&TypedEventOracleArtifact>) -> String {
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

fn typed_event_oracle_contributes_to_pass_fail(scenario: Scenario) -> bool {
    matches!(
        scenario,
        Scenario::Smoke | Scenario::InventoryInteraction | Scenario::InventoryStackSplitMerge
    )
}

fn validate_typed_event_oracle_for_migrated_scenario(
    cfg: &Config,
    client: &ClientRunEvidence,
) -> Result<(), String> {
    if !typed_event_oracle_contributes_to_pass_fail(cfg.scenario) {
        return Ok(());
    }
    let events = typed_events_from_receipt_evidence(cfg, client)?;
    let required = typed_event_required_events_for_graph(cfg.scenario);
    let required_refs = required.iter().map(String::as_str).collect::<Vec<_>>();
    let forbidden = scenario_forbidden_patterns(cfg.scenario)
        .iter()
        .map(|(name, _)| *name)
        .collect::<Vec<_>>();
    let ordered_edges = typed_event_ordered_edges_for_scenario(cfg.scenario);
    let username = single_typed_event_username(client);
    let session = typed_event_session_id(cfg);
    let result = evaluate_typed_event_graph(
        &events,
        scenario_name(cfg.scenario),
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

fn typed_event_required_events_for_graph(scenario: Scenario) -> Vec<String> {
    let mut required = scenario_required_milestones(scenario)
        .iter()
        .map(|(name, _)| (*name).to_string())
        .collect::<Vec<_>>();
    required.extend(
        server_required_milestones(scenario)
            .iter()
            .map(|(name, _)| (*name).to_string()),
    );
    required
}

fn typed_event_ordered_edges_for_scenario(scenario: Scenario) -> Vec<(&'static str, &'static str)> {
    match scenario {
        Scenario::Smoke => vec![],
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
        _ => vec![],
    }
}

fn projectile_damage_required_steps() -> Vec<&'static str> {
    vec![
        "attacker_client_projectile_use_sent",
        "attacker_client_projectile_swing_sent",
        "server_projectile_use_attacker_victim",
        "server_projectile_hit_attacker_victim_health_delta",
        "victim_client_damage_update",
    ]
}

fn evaluate_projectile_damage_causality(
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

fn evaluate_projectile_damage_causality_for_damage(
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
        "Usage: mc-compat-runner [--config PATH] [--steel-config PATH] [--dry-run|--run|--run-matrix] [--build-client] [--status-only] [--status] [--cleanup [--dry-run|--apply]] [--stop] [--compare-receipts PAPER_RECEIPT VALENCE_RECEIPT] [--scenario {}] [--keep-server] [--server-backend valence|paper] [--client-dir PATH] [--receipt PATH] [--receipt-dir DIR] [--failure-bundle PATH] [--valence-repo PATH] [--valence-rev REV]\n\n\
Automates a local Stevenarella compatibility smoke against a Minecraft {} / protocol {} server.\n\
Default client source is the vendored Stevenarella tree at ./stevenarella; pass --client-dir/CLIENT_DIR to use another source tree.\n\
Pass --config/MC_COMPAT_CONFIG a JSON file exported from legacy Nickel config, or --steel-config/MC_COMPAT_STEEL_CONFIG a restricted Steel module; env vars and later CLI flags override either config source.\n\
Pass --receipt/SMOKE_RECEIPT to write a machine-readable mc.compat.scenario.receipt.v2 JSON receipt for Cairn/Octet evidence flows. Pass --failure-bundle/MC_COMPAT_FAILURE_BUNDLE with a docs/evidence path to write a fail-only diagnostic bundle after failed runs.
Use --scenario valence-compat-bot-probe for a bounded one-client Valence probe with status/login/render milestones and safe non-load receipt fields. Use --scenario flag-score-repeat to require explicit protocol/login/render/team/flag/two-score milestones and forbidden-pattern checks. Use --scenario blue-flag-score to exercise the mirrored BLUE-team flag path. Use --scenario survival-break-place-pickup for the bounded survival fixture. Use --scenario survival-chest-persistence for the two-session chest open/store/close/reconnect/reopen probe. Use --scenario survival-crafting-table for one crafting-table open/input/result/collect rail. Use --scenario survival-crafting-recipe-breadth for one bounded shaped/shapeless/invalid recipe breadth rail. Use --scenario survival-furnace-persistence for one furnace input/fuel/output/reconnect rail. Use --scenario survival-furnace-smelting-breadth for one bounded raw-iron/coal smelt plus invalid-fuel rejection rail. Use --scenario survival-hunger-food for one hunger deficit, food consume, and inventory decrement rail. Use --scenario survival-hunger-health-cycle for the isolated bounded health-cycle row using explicit food, saturation, health recovery, and inventory checkpoints. Use --scenario survival-mob-drop for one configured mob kill, drop, pickup, and inventory increment rail. Use --scenario survival-redstone-toggle for one configured control on/off output update rail. Use --scenario survival-world-persistence-restart for one configured block mutation, controlled reload, reconnect, and post-reload observation rail. Use --scenario survival-crash-recovery-parity for one configured block mutation, forced backend stop, crash-recovery restart, reconnect, and post-crash observation rail. Use --scenario survival-block-entity-persistence-parity for one configured sign block entity, controlled reload, reconnect, and post-reload sign text observation rail. Use --scenario survival-biome-dimension-state for one client-observed dimension/world identifier rail. Use --scenario mcp-controlled-smoke for deterministic MCP receipt/checker dry-run evidence before live client driving. Use --scenario vanilla-combat-armor-reference-parity for one Paper/Valence diamond-chestplate combat reference row. Use --scenario reconnect-flag-state to require disconnect/return state coherence while holding a flag. Use --scenario ctf-invalid-pickup-ownership for one contained own-flag pickup attempt with server rejection evidence. Use --scenario ctf-invalid-return-drop for one contained own-base return/drop attempt with server rejection evidence. Use --scenario ctf-score-limit-win-condition for one near-limit capture that emits exactly one win/end milestone. Use --scenario ctf-simultaneous-pickup-capture-race for one bounded two-client same-flag race with one accepted transition and one rejected duplicate pickup. Use --scenario ctf-spawn-team-balance-reset for one bounded two-client team assignment, spawn/resource, and post-score reset row. Use --scenario reconnect-flag-score to add reconnect evidence; use --scenario multi-client-load-score for two concurrent clients plus server-side correlation.\n\
Use --expect-status-description/--expect-status-version/--expect-status-sample to assert status response fixture data, --packet-capture-summary for redacted capture summary metadata, and --proxy-route/--proxy-forwarding-mode for proxied-route receipt fields.\n\
Use --compare-receipts PAPER_RECEIPT VALENCE_RECEIPT to check the fallback/control and default-backend receipts agree on protocol and headless isolation.\n\
Use --run-matrix --receipt-dir DIR to run Paper and Valence receipts then compare them; add --dry-run after --run-matrix for a non-side-effecting matrix fixture.\n\
Use --status to inspect harness-owned Paper/Valence/tmp state; use --cleanup --dry-run to preview cleanup and --cleanup --apply to remove it.\n\
Default server backend is Valence, using the vendored Valence tree plus an isolated worktree when a pinned revision is requested so the current checkout is untouched.\n\
If the Stevenarella or Valence source tree is missing, restore the vendored tree or pass --client-dir/CLIENT_DIR and --valence-repo/VALENCE_REPO to alternate source trees.\n\
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
    apply_build_env(&mut cmd, &cfg.target_dir);
    run_cmd(cfg, &mut cmd)
}

fn ensure_client_dir_ready(cfg: &Config) -> Result<(), String> {
    if !cfg.client_dir.exists() {
        return Err(format!(
            "Stevenarella source tree not found at {}. Keep the vendored mc/stevenarella tree present or pass --client-dir/CLIENT_DIR to another checkout.",
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

fn start_server(cfg: &Config) -> Result<ManagedServer, String> {
    cfg.server_backend.runtime().start(cfg)
}

fn stop_server(cfg: &Config) -> Result<(), String> {
    cfg.server_backend.runtime().stop(cfg)
}

fn force_stop_server(cfg: &Config) -> Result<(), String> {
    cfg.server_backend.runtime().force_stop(cfg)
}

fn force_stop_valence_server(cfg: &Config) -> Result<(), String> {
    if let Ok(pid) = fs::read_to_string(&cfg.valence_pid_file) {
        let pid = pid.trim();
        if !pid.is_empty() {
            log(format_args!(
                "force stopping managed Valence server process {pid}"
            ));
            let _ = Command::new("kill").arg("-9").arg(pid).status();
        }
        fs::remove_file(&cfg.valence_pid_file)
            .map_err(|e| format!("remove {}: {e}", cfg.valence_pid_file.display()))?;
    }
    Ok(())
}

fn force_stop_paper_server(cfg: &Config) -> Result<(), String> {
    log(format_args!(
        "force stopping managed Paper container {}",
        cfg.server_name
    ));
    let mut remove = Command::new("docker");
    remove.arg("rm").arg("-f").arg(&cfg.server_name);
    run_cmd(cfg, &mut remove)
}

fn stop_paper_server(cfg: &Config) -> Result<(), String> {
    log(format_args!(
        "stopping managed Paper container {} with graceful timeout",
        cfg.server_name
    ));
    let mut stop = Command::new("docker");
    stop.arg("stop")
        .arg("--time")
        .arg(PAPER_GRACEFUL_STOP_TIMEOUT_SECS.to_string())
        .arg(&cfg.server_name);
    run_cmd(cfg, &mut stop)?;
    let mut remove = Command::new("docker");
    remove.arg("rm").arg(&cfg.server_name);
    run_cmd(cfg, &mut remove)
}

fn print_harness_status(cfg: &Config) -> Result<(), String> {
    log(format_args!(
        "harness status for server '{}'",
        cfg.server_name
    ));
    let docker = docker_container_status(&cfg.server_name)?;
    println!("paper_container={docker}");

    let pid_state = valence_pid_state(&cfg.valence_pid_file)?;
    println!("valence_pid={pid_state}");
    println!(
        "valence_worktree={} exists={}",
        cfg.valence_worktree.display(),
        cfg.valence_worktree.exists()
    );
    println!(
        "valence_target_dir={} exists={}",
        cfg.valence_target_dir.display(),
        cfg.valence_target_dir.exists()
    );
    println!(
        "valence_log={} exists={}",
        cfg.valence_log.display(),
        cfg.valence_log.exists()
    );
    let logs = client_log_paths()?;
    println!("client_logs={}", logs.len());
    for path in logs.iter().take(20) {
        println!("client_log={}", path.display());
    }
    if logs.len() > 20 {
        println!("client_logs_omitted={}", logs.len() - 20);
    }
    Ok(())
}

fn cleanup_harness_state(_cfg: &Config, plan: &CleanupPlan) -> Result<(), String> {
    let apply = plan.apply;
    if apply {
        log(format_args!("cleaning harness-owned state"));
    } else {
        log(format_args!(
            "cleanup dry-run; pass --cleanup --apply to remove harness-owned state"
        ));
    }

    cleanup_paper_container(&plan.paper_container, apply)?;
    cleanup_valence_pid(Path::new(&plan.valence_pid_file), apply)?;
    for action in &plan.path_actions {
        cleanup_path(&action.label, Path::new(&action.path), apply)?;
    }
    for path in client_log_paths()? {
        cleanup_path("client log", &path, apply)?;
    }
    Ok(())
}

fn docker_container_status(name: &str) -> Result<String, String> {
    let output = Command::new("docker")
        .arg("ps")
        .arg("-a")
        .arg("--filter")
        .arg(format!("name={name}"))
        .arg("--format")
        .arg("{{.Names}} {{.Status}}")
        .output();
    match output {
        Ok(out) if out.status.success() => {
            let text = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if text.is_empty() {
                Ok("absent".to_string())
            } else {
                Ok(text)
            }
        }
        Ok(out) => Ok(format!(
            "unavailable: docker ps exited {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        )),
        Err(err) => Ok(format!("unavailable: {err}")),
    }
}

fn cleanup_paper_container(name: &str, apply: bool) -> Result<(), String> {
    let state = docker_container_status(name)?;
    if state == "absent" || state.starts_with("unavailable:") {
        println!("cleanup paper_container {name}: {state}");
        return Ok(());
    }
    if apply {
        log(format_args!("removing Paper container {name}"));
        let status = Command::new("docker")
            .arg("rm")
            .arg("-f")
            .arg(name)
            .status()
            .map_err(|e| format!("docker rm -f {name}: {e}"))?;
        if !status.success() {
            return Err(format!("docker rm -f {name} failed with {status}"));
        }
    } else {
        println!("would remove Paper container {name}: {state}");
    }
    Ok(())
}

fn valence_pid_state(pid_file: &Path) -> Result<String, String> {
    let pid = match fs::read_to_string(pid_file) {
        Ok(pid) => pid.trim().to_string(),
        Err(err) if err.kind() == io::ErrorKind::NotFound => return Ok("absent".to_string()),
        Err(err) => return Err(format!("read {}: {err}", pid_file.display())),
    };
    if pid.is_empty() {
        return Ok(format!("empty pid file {}", pid_file.display()));
    }
    let alive = Command::new("kill")
        .arg("-0")
        .arg(&pid)
        .status()
        .map(|status| status.success())
        .unwrap_or(false);
    Ok(format!(
        "pid={} alive={} file={}",
        pid,
        alive,
        pid_file.display()
    ))
}

fn cleanup_valence_pid(pid_file: &Path, apply: bool) -> Result<(), String> {
    let pid = match fs::read_to_string(pid_file) {
        Ok(pid) => pid.trim().to_string(),
        Err(err) if err.kind() == io::ErrorKind::NotFound => {
            println!("cleanup valence_pid_file {}: absent", pid_file.display());
            return Ok(());
        }
        Err(err) => return Err(format!("read {}: {err}", pid_file.display())),
    };
    if !pid.is_empty() {
        if apply {
            log(format_args!("stopping stale Valence process {pid}"));
            let _ = Command::new("kill").arg(&pid).status();
        } else {
            println!("would stop Valence process {pid}");
        }
    }
    if apply {
        fs::remove_file(pid_file).map_err(|e| format!("remove {}: {e}", pid_file.display()))?;
    } else {
        println!("would remove Valence pid file {}", pid_file.display());
    }
    Ok(())
}

fn cleanup_path(label: &str, path: &Path, apply: bool) -> Result<(), String> {
    if !path.exists() {
        println!("cleanup {label} {}: absent", path.display());
        return Ok(());
    }
    if apply {
        log(format_args!("removing {label} {}", path.display()));
        if path.is_dir() {
            fs::remove_dir_all(path).map_err(|e| format!("remove {}: {e}", path.display()))?;
        } else {
            fs::remove_file(path).map_err(|e| format!("remove {}: {e}", path.display()))?;
        }
    } else {
        println!("would remove {label} {}", path.display());
    }
    Ok(())
}

fn client_log_paths() -> Result<Vec<PathBuf>, String> {
    let mut paths = Vec::new();
    let entries = match fs::read_dir("/tmp") {
        Ok(entries) => entries,
        Err(err) if err.kind() == io::ErrorKind::NotFound => return Ok(paths),
        Err(err) => return Err(format!("read /tmp: {err}")),
    };
    for entry in entries {
        let entry = entry.map_err(|e| format!("read /tmp entry: {e}"))?;
        let name = entry.file_name();
        if is_mc_compat_client_log(&name.to_string_lossy()) {
            paths.push(entry.path());
        }
    }
    paths.sort();
    Ok(paths)
}

fn is_mc_compat_client_log(name: &str) -> bool {
    name.starts_with("mc-compat-client.") && name.ends_with(".log")
}

fn prepare_valence_worktree(cfg: &Config) -> Result<(), String> {
    ensure_valence_repo_ready(cfg)?;
    if !cfg.valence_worktree.join(".git").exists() {
        prune_stale_valence_worktrees(cfg)?;
        log(format_args!(
            "creating isolated Valence worktree {} at {}",
            cfg.valence_worktree.display(),
            cfg.valence_rev
        ));
        let mut cmd = Command::new("git");
        cmd.arg("-C")
            .arg(&cfg.valence_repo)
            .arg("worktree")
            .arg("add")
            .arg("--detach")
            .arg(&cfg.valence_worktree)
            .arg(&cfg.valence_rev);
        run_cmd(cfg, &mut cmd)?;
    } else {
        ensure_valence_worktree_at_requested_rev(cfg)?;
        log(format_args!(
            "using existing Valence worktree {}",
            cfg.valence_worktree.display()
        ));
    }
    Ok(())
}

fn ensure_valence_worktree_at_requested_rev(cfg: &Config) -> Result<(), String> {
    if cfg.mode == Mode::DryRun {
        return Ok(());
    }
    let current = git_rev_parse(&cfg.valence_worktree, GIT_HEAD_REV)?;
    let requested = git_rev_parse(
        &cfg.valence_repo,
        &format!("{}^{{commit}}", cfg.valence_rev),
    )?;
    if current == requested {
        return Ok(());
    }
    Err(format!(
        "Valence worktree {} is at {current}, but requested {} resolves to {requested}. Remove the stale worktree or pass VALENCE_WORKTREE to a fresh path.",
        cfg.valence_worktree.display(),
        cfg.valence_rev
    ))
}

fn git_rev_parse(repo: &Path, rev: &str) -> Result<String, String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo)
        .arg("rev-parse")
        .arg(rev)
        .output()
        .map_err(|e| format!("git rev-parse {rev} in {}: {e}", repo.display()))?;
    if !output.status.success() {
        return Err(format!(
            "git rev-parse {rev} in {} failed with {}",
            repo.display(),
            output.status
        ));
    }
    String::from_utf8(output.stdout)
        .map(|text| text.trim().to_string())
        .map_err(|e| {
            format!(
                "git rev-parse {rev} output in {} was not UTF-8: {e}",
                repo.display()
            )
        })
}

fn git_scoped_latest_commit(repo: &Path) -> Result<String, String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo)
        .arg("log")
        .arg("-1")
        .arg(GIT_LOG_COMMIT_FORMAT)
        .arg("--")
        .arg(GIT_CURRENT_DIR_PATHSPEC)
        .output()
        .map_err(|e| format!("git scoped log in {}: {e}", repo.display()))?;
    if !output.status.success() {
        return Err(format!(
            "git scoped log in {} failed with {}",
            repo.display(),
            output.status
        ));
    }
    String::from_utf8(output.stdout)
        .map(|text| text.trim().to_string())
        .map_err(|e| {
            format!(
                "git scoped log output in {} was not UTF-8: {e}",
                repo.display()
            )
        })
        .and_then(|commit| {
            if commit.is_empty() {
                Err(format!(
                    "git scoped log in {} did not find a commit for {}",
                    repo.display(),
                    GIT_CURRENT_DIR_PATHSPEC
                ))
            } else {
                Ok(commit)
            }
        })
}

fn git_scoped_worktree_dirty(repo: &Path) -> Result<bool, String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo)
        .arg("status")
        .arg(GIT_STATUS_PORCELAIN_FLAG)
        .arg("--")
        .arg(GIT_CURRENT_DIR_PATHSPEC)
        .output()
        .map_err(|e| format!("git scoped status in {}: {e}", repo.display()))?;
    if !output.status.success() {
        return Err(format!(
            "git scoped status in {} failed with {}",
            repo.display(),
            output.status
        ));
    }
    String::from_utf8(output.stdout)
        .map(|text| !text.trim().is_empty())
        .map_err(|e| {
            format!(
                "git scoped status output in {} was not UTF-8: {e}",
                repo.display()
            )
        })
}

fn build_git_revision_evidence(
    requested_rev: Option<&str>,
    resolved_rev: Result<String, String>,
    dirty: Result<bool, String>,
) -> GitRevisionEvidence {
    match (resolved_rev, dirty) {
        (Ok(resolved_rev), Ok(dirty)) => GitRevisionEvidence {
            requested_rev: requested_rev.map(str::to_string),
            resolved_rev: Some(resolved_rev),
            status: if dirty {
                GIT_STATUS_DIRTY
            } else {
                GIT_STATUS_CLEAN
            },
            dirty,
            diagnostics: Vec::new(),
        },
        (resolved_rev, dirty) => {
            let mut diagnostics = Vec::new();
            if let Err(err) = resolved_rev {
                diagnostics.push(err);
            }
            if let Err(err) = dirty {
                diagnostics.push(err);
            }
            GitRevisionEvidence {
                requested_rev: requested_rev.map(str::to_string),
                resolved_rev: None,
                status: GIT_STATUS_UNAVAILABLE,
                dirty: true,
                diagnostics,
            }
        }
    }
}

fn git_revision_evidence(repo: &Path, requested_rev: Option<&str>) -> GitRevisionEvidence {
    build_git_revision_evidence(
        requested_rev,
        git_scoped_latest_commit(repo),
        git_scoped_worktree_dirty(repo),
    )
}

fn valence_source_dir(cfg: &Config) -> PathBuf {
    if cfg.valence_worktree.join(CARGO_MANIFEST_FILE).exists() {
        cfg.valence_worktree.clone()
    } else {
        let vendored_source = cfg.valence_worktree.join(VALENCE_MONOREPO_SUBTREE_DIR);
        if vendored_source.join(CARGO_MANIFEST_FILE).exists() {
            vendored_source
        } else {
            cfg.valence_worktree.clone()
        }
    }
}

fn valence_revision_dir(cfg: &Config) -> PathBuf {
    if cfg.valence_worktree.exists() {
        valence_source_dir(cfg)
    } else {
        cfg.valence_repo.clone()
    }
}

fn child_revision_evidence_for_receipt(cfg: &Config) -> ChildRevisionEvidence {
    if cfg.mode == Mode::DryRun {
        return ChildRevisionEvidence {
            client: GitRevisionEvidence::dry_run(None),
            valence: GitRevisionEvidence::dry_run(Some(cfg.valence_rev.clone())),
        };
    }
    ChildRevisionEvidence {
        client: git_revision_evidence(&cfg.client_dir, None),
        valence: git_revision_evidence(&valence_revision_dir(cfg), Some(&cfg.valence_rev)),
    }
}

fn prune_stale_valence_worktrees(cfg: &Config) -> Result<(), String> {
    let mut cmd = Command::new("git");
    cmd.arg("-C")
        .arg(&cfg.valence_repo)
        .arg("worktree")
        .arg("prune");
    run_cmd(cfg, &mut cmd)
}

fn ensure_valence_repo_ready(cfg: &Config) -> Result<(), String> {
    if !cfg.valence_repo.exists() {
        return Err(format!(
            "Valence source tree not found at {}. Keep the vendored mc/valence tree present or pass --valence-repo/VALENCE_REPO to another checkout.",
            cfg.valence_repo.display()
        ));
    }
    if cfg.mode == Mode::DryRun {
        return Ok(());
    }

    let status = Command::new("git")
        .arg("-C")
        .arg(&cfg.valence_repo)
        .arg("rev-parse")
        .arg("--verify")
        .arg(format!("{}^{{commit}}", cfg.valence_rev))
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map_err(|e| {
            format!(
                "check Valence source tree {}: {e}",
                cfg.valence_repo.display()
            )
        })?;

    if !status.success() {
        return Err(format!(
            "Valence source tree {} does not contain compatible revision {}. Fetch the parent repository history or pass --valence-repo/VALENCE_REPO to a checkout that has it.",
            cfg.valence_repo.display(),
            cfg.valence_rev
        ));
    }

    Ok(())
}

fn start_valence_server(cfg: &Config) -> Result<ManagedServer, String> {
    prepare_valence_worktree(cfg)?;
    log(format_args!(
        "starting Valence {} example '{}' on 127.0.0.1:{}; log: {}",
        cfg.valence_rev,
        cfg.valence_example,
        cfg.server_port,
        cfg.valence_log.display()
    ));
    if cfg.mode == Mode::DryRun {
        let source_dir = valence_source_dir(cfg);
        log(format_args!(
            "would run Valence example from {}",
            source_dir.display()
        ));
        return Ok(ManagedServer {
            child: None,
            pid_file: cfg.valence_pid_file.clone(),
            paper_container: None,
            keep: true,
        });
    }
    if cfg.server_port != VALENCE_DEFAULT_SERVER_PORT {
        log(format_args!(
            "warning: Valence revision {} defaults to 127.0.0.1:{}; SERVER_PORT={} may only work if the example overrides Config::address",
            cfg.valence_rev, VALENCE_DEFAULT_SERVER_PORT, cfg.server_port
        ));
    }
    stop_valence_server(cfg)?;
    let log_file = File::create(&cfg.valence_log)
        .map_err(|e| format!("create {}: {e}", cfg.valence_log.display()))?;
    let err_file = log_file
        .try_clone()
        .map_err(|e| format!("clone valence log handle: {e}"))?;
    let source_dir = valence_source_dir(cfg);
    let mut cmd = Command::new("cargo");
    cmd.current_dir(&source_dir)
        .arg("run")
        .arg("--example")
        .arg(&cfg.valence_example)
        .stdout(Stdio::from(log_file))
        .stderr(Stdio::from(err_file));
    cmd.env("RUSTC_WRAPPER", "")
        .env("CARGO_TARGET_DIR", &cfg.valence_target_dir);
    scenario_behavior(cfg.scenario).apply_valence_server_env(&mut cmd, cfg);
    if let Some(path) = &cfg.steel_config_path {
        cmd.env("MC_COMPAT_STEEL_CONFIG", path);
    }
    let child = cmd.spawn().map_err(|e| format!("spawn Valence: {e}"))?;
    fs::write(&cfg.valence_pid_file, child.id().to_string())
        .map_err(|e| format!("write {}: {e}", cfg.valence_pid_file.display()))?;
    Ok(ManagedServer {
        child: Some(child),
        pid_file: cfg.valence_pid_file.clone(),
        paper_container: None,
        keep: cfg.keep_server,
    })
}

fn start_paper_server(cfg: &Config) -> Result<(), String> {
    log(format_args!(
        "starting Paper {} server on 127.0.0.1:{} via {} with EULA=TRUE",
        cfg.server_version, cfg.server_port, cfg.docker_image
    ));
    if cfg.mode == Mode::DryRun {
        let mut cmd = Command::new("docker");
        configure_paper_run_command(cfg, &mut cmd)?;
        return run_cmd(cfg, &mut cmd);
    }
    let _ = Command::new("docker")
        .arg("rm")
        .arg("-f")
        .arg(&cfg.server_name)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    let mut cmd = Command::new("docker");
    configure_paper_run_command(cfg, &mut cmd)?;
    run_cmd(cfg, &mut cmd)
}

fn configure_paper_run_command(cfg: &Config, cmd: &mut Command) -> Result<(), String> {
    cmd.arg("run")
        .arg("-d")
        .arg("--name")
        .arg(&cfg.server_name)
        .arg("-p")
        .arg(format!("127.0.0.1:{}:25565", cfg.server_port))
        .arg("-e")
        .arg("EULA=TRUE")
        .arg("-e")
        .arg("TYPE=PAPER")
        .arg("-e")
        .arg(format!("VERSION={}", cfg.server_version))
        .arg("-e")
        .arg("ONLINE_MODE=FALSE")
        .arg("-e")
        .arg("MEMORY=1G")
        .arg("-e")
        .arg(format!("VIEW_DISTANCE={PAPER_VIEW_DISTANCE}"))
        .arg("-e")
        .arg(format!("SIMULATION_DISTANCE={PAPER_SIMULATION_DISTANCE}"));
    scenario_behavior(cfg.scenario).apply_paper_server_env(cmd, cfg)?;
    add_paper_plugin_mount(cfg, cmd)?;
    cmd.arg(&cfg.docker_image);
    Ok(())
}

fn add_paper_plugin_mount(cfg: &Config, cmd: &mut Command) -> Result<(), String> {
    let Some(plugin_jar) = &cfg.paper_plugin_jar else {
        return Ok(());
    };
    let absolute_jar = fs::canonicalize(plugin_jar).map_err(|e| {
        format!(
            "canonicalize PAPER_PLUGIN_JAR {}: {e}",
            plugin_jar.display()
        )
    })?;
    let file_name = absolute_jar.file_name().ok_or_else(|| {
        format!(
            "PAPER_PLUGIN_JAR {} has no file name",
            absolute_jar.display()
        )
    })?;
    let container_path = Path::new(PAPER_PLUGIN_CONTAINER_DIR).join(file_name);
    cmd.arg("-v").arg(format!(
        "{}:{}:ro",
        absolute_jar.display(),
        container_path.display()
    ));
    Ok(())
}

fn stop_valence_server(cfg: &Config) -> Result<(), String> {
    if let Ok(pid) = fs::read_to_string(&cfg.valence_pid_file) {
        let pid = pid.trim();
        if !pid.is_empty() {
            log(format_args!(
                "stopping managed Valence server process {pid}"
            ));
            let _ = Command::new("kill").arg(pid).status();
        }
        fs::remove_file(&cfg.valence_pid_file)
            .map_err(|e| format!("remove {}: {e}", cfg.valence_pid_file.display()))?;
    }
    Ok(())
}

fn probe_status(cfg: &Config) -> Result<(), String> {
    log(format_args!(
        "probing status 127.0.0.1:{} expecting protocol {}",
        cfg.server_port, cfg.server_protocol
    ));
    if cfg.mode == Mode::DryRun {
        log(format_args!("would run Rust protocol status probe"));
        return Ok(());
    }
    let mut last = String::new();
    for _ in 0..90 {
        match read_status(cfg.server_port, cfg.server_protocol) {
            Ok(status) => {
                println!("{status}");
                let needle = format!("\"protocol\":{}", cfg.server_protocol);
                let spaced = format!("\"protocol\": {}", cfg.server_protocol);
                if status.contains(&needle) || status.contains(&spaced) {
                    assert_status_expectations(cfg, &status)?;
                    return Ok(());
                }
                return Err(format!(
                    "protocol mismatch in status response; expected {}",
                    cfg.server_protocol
                ));
            }
            Err(err) => last = err,
        }
        thread::sleep(Duration::from_secs(2));
    }
    Err(format!("server status probe failed: {last}"))
}

fn assert_status_expectations(cfg: &Config, status: &str) -> Result<(), String> {
    if let Some(expected) = &cfg.expected_status_description {
        if !status.contains(expected) {
            return Err(format!(
                "status response missing expected description {expected:?}"
            ));
        }
    }
    if let Some(expected) = &cfg.expected_status_version_name {
        if !status.contains(expected) {
            return Err(format!(
                "status response missing expected version {expected:?}"
            ));
        }
    }
    for expected in &cfg.expected_status_sample {
        if !status.contains(expected) {
            return Err(format!(
                "status response missing expected sample {expected:?}"
            ));
        }
    }
    Ok(())
}

fn read_status(port: u16, protocol: u32) -> Result<String, String> {
    let mut stream = TcpStream::connect(("127.0.0.1", port)).map_err(|e| e.to_string())?;
    stream
        .set_read_timeout(Some(Duration::from_secs(STATUS_SOCKET_TIMEOUT_SECS)))
        .map_err(|e| e.to_string())?;
    stream
        .set_write_timeout(Some(Duration::from_secs(STATUS_SOCKET_TIMEOUT_SECS)))
        .map_err(|e| e.to_string())?;
    let mut payload = Vec::new();
    payload.write_varint(protocol)?;
    payload.write_mc_string(STATUS_LOCALHOST_ADDRESS)?;
    payload.extend_from_slice(&port.to_be_bytes());
    payload.write_varint(STATUS_HANDSHAKE_NEXT_STATE)?;
    stream.write_packet(STATUS_PACKET_ID, &payload)?;
    stream.write_packet(STATUS_PACKET_ID, &[])?;
    let _packet_len = stream.read_varint()?;
    let packet_id = stream.read_varint()?;
    if packet_id != STATUS_PACKET_ID {
        return Err(format!("unexpected status packet id {packet_id}"));
    }
    stream.read_mc_string()
}

#[derive(Debug)]
struct SingleClientRun {
    username: String,
    log_path: PathBuf,
    exit_code: Option<i32>,
    output: String,
    matched_success_pattern: Option<String>,
}

fn run_client(cfg: &Config) -> Result<ClientRunEvidence, String> {
    log(format_args!(
        "running Stevenarella headless scenario '{}' isolated from host Wayland compositor",
        scenario_name(cfg.scenario)
    ));
    if cfg.mode == Mode::DryRun {
        log(format_args!("would run Stevenarella under xvfb-run"));
        let behavior = scenario_behavior(cfg.scenario);
        if behavior.is_mcp_controlled_smoke() {
            return Ok(mcp_controlled_dry_run_evidence(cfg));
        }
        if behavior.uses_dynamic_projectile_health() {
            return Ok(projectile_damage_dry_run_evidence(cfg));
        }
        let scenario = evaluate_scenario_for_config(cfg, "");
        let server_scenario = Some(evaluate_server_scenario(
            cfg.scenario,
            "",
            &cfg.client_username,
        ));
        return Ok(ClientRunEvidence {
            log_path: None,
            log_paths: Vec::new(),
            usernames: planned_client_usernames(cfg),
            exit_code: None,
            classification: "dry-run",
            matched_success_pattern: None,
            scenario: Some(scenario),
            server_scenario,
            projectile_damage_causality: None,
            mcp_control: None,
            frame_artifacts: None,
        });
    }

    let behavior = scenario_behavior(cfg.scenario);
    if behavior.is_mcp_controlled_smoke() {
        return run_mcp_controlled_live_client(cfg);
    }

    let runs = match behavior.run_strategy() {
        ScenarioRunStrategy::ReconnectSequence => run_reconnect_sequence_scenario(cfg)?,
        ScenarioRunStrategy::MultiClient => run_multi_client_load_scenario(cfg)?,
        ScenarioRunStrategy::SingleClient => vec![run_single_client(
            cfg,
            &cfg.client_username,
            FIRST_CLIENT_INDEX,
        )?],
    };

    let mut combined_output = String::new();
    behavior.append_client_count_markers(runs.len(), &mut combined_output);
    if behavior.uses_reconnect_session_marker() {
        append_count_marker(&mut combined_output, RECONNECT_SESSION_COUNT_NEEDLE);
    }
    for run in &runs {
        combined_output.push_str(&run.output);
        if !combined_output.ends_with('\n') {
            combined_output.push('\n');
        }
    }
    if behavior.uses_crash_recovery_restart() {
        combined_output.push_str(&derive_survival_crash_recovery_client_milestones(
            &combined_output,
        ));
    }
    print!("{combined_output}");
    io::stdout().flush().map_err(|e| e.to_string())?;

    let matched_success_pattern = cfg
        .client_success_needles
        .iter()
        .find(|needle| combined_output.contains(needle.as_str()))
        .cloned();
    let scenario = evaluate_scenario_for_config(cfg, &combined_output);
    if cfg.scenario != Scenario::Smoke && !scenario.passed {
        return Err(format!(
            "scenario {} failed: missing={:?} forbidden={:?}; logs={}",
            scenario_name(cfg.scenario),
            scenario.missing_milestones,
            scenario.forbidden_matches,
            runs.iter()
                .map(|run| run.log_path.display().to_string())
                .collect::<Vec<_>>()
                .join(",")
        ));
    }

    let server_scenario = read_server_scenario_evidence(cfg, &runs)?;
    if requires_server_correlation(cfg) {
        if let Some(server) = &server_scenario {
            if !server.passed {
                return Err(format!(
                    "server correlation for scenario {} failed: missing={:?} forbidden={:?}; log={}",
                    scenario_name(cfg.scenario),
                    server.missing_milestones,
                    server.forbidden_matches,
                    server_log_label(cfg)
                ));
            }
        }
    }

    let projectile_damage_causality = if behavior.uses_dynamic_projectile_health() {
        let server_log = read_valence_log(cfg)?;
        let client_logs = runs
            .iter()
            .map(|run| ClientLogSlice {
                username: &run.username,
                output: &run.output,
            })
            .collect::<Vec<_>>();
        let expected_damage = projectile_damage_amount_needle(cfg);
        let causality = evaluate_projectile_damage_causality_for_damage(
            &client_logs,
            &server_log,
            &cfg.client_username,
            &expected_damage,
        );
        if !causality.passed {
            return Err(format!(
                "projectile damage causality failed: missing={:?} order_violations={:?}; client_logs={}; server_log={}",
                causality.missing_steps,
                causality.order_violations,
                runs.iter()
                    .map(|run| run.log_path.display().to_string())
                    .collect::<Vec<_>>()
                    .join(","),
                cfg.valence_log.display()
            ));
        }
        Some(causality)
    } else {
        None
    };

    let all_success = runs.iter().all(|run| run.exit_code == Some(0));
    let timeout_success = runs
        .iter()
        .all(|run| run.exit_code == Some(124) && run.matched_success_pattern.is_some());
    let mixed_success = runs.iter().all(|run| {
        run.exit_code == Some(0)
            || (run.exit_code == Some(124) && run.matched_success_pattern.is_some())
    });
    let classification =
        if behavior.run_strategy() != ScenarioRunStrategy::SingleClient && mixed_success {
            "multi-client-load-evidence"
        } else if all_success {
            "client-exited-success"
        } else if timeout_success {
            "timeout-success-evidence"
        } else {
            return Err(format!(
                "client scenario failed; exits={:?}; logs={}",
                runs.iter().map(|run| run.exit_code).collect::<Vec<_>>(),
                runs.iter()
                    .map(|run| run.log_path.display().to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            ));
        };

    for run in &runs {
        log(format_args!(
            "client {} finished {:?}; log: {}",
            run.username,
            run.exit_code,
            run.log_path.display()
        ));
    }
    let log_paths = runs
        .iter()
        .map(|run| run.log_path.clone())
        .collect::<Vec<_>>();
    let usernames = runs
        .iter()
        .map(|run| run.username.clone())
        .collect::<Vec<_>>();
    let evidence = ClientRunEvidence {
        log_path: log_paths.first().cloned(),
        log_paths,
        usernames,
        exit_code: runs.first().and_then(|run| run.exit_code),
        classification,
        matched_success_pattern,
        scenario: Some(scenario),
        server_scenario,
        projectile_damage_causality,
        mcp_control: None,
        frame_artifacts: None,
    };
    validate_typed_event_oracle_for_migrated_scenario(cfg, &evidence)?;
    Ok(evidence)
}

fn mcp_controlled_dry_run_evidence(cfg: &Config) -> ClientRunEvidence {
    let output = mcp_controlled_success_output();
    ClientRunEvidence {
        log_path: None,
        log_paths: Vec::new(),
        usernames: planned_client_usernames(cfg),
        exit_code: None,
        classification: "dry-run",
        matched_success_pattern: None,
        scenario: Some(evaluate_scenario_for_config(cfg, &output)),
        server_scenario: Some(evaluate_server_scenario(
            cfg.scenario,
            "",
            &cfg.client_username,
        )),
        projectile_damage_causality: None,
        mcp_control: Some(mcp_control_dry_run_control_evidence()),
        frame_artifacts: Some(evaluate_frame_artifacts_receipt(cfg, None)),
    }
}

fn mcp_controlled_success_output() -> String {
    [
        "mcp_control_dry_run",
        "mcp_initialize",
        "mcp_tools_list",
        "mcp_status_call",
        "mcp_command_outcomes",
    ]
    .join("\n")
        + "\n"
}

fn mcp_control_dry_run_control_evidence() -> McpControlRunEvidence {
    McpControlRunEvidence {
        handshake_success: true,
        tool_list_digest: mcp_control_tool_list_digest(),
        tool_names: MCP_CONTROL_TOOL_NAMES.to_vec(),
        calls_attempted: MCP_CONTROL_REQUIRED_CALLS.to_vec(),
        calls_succeeded: MCP_CONTROL_REQUIRED_CALLS.to_vec(),
        first_failure: None,
        stdout_clean: true,
        command_outcome_ids: MCP_CONTROL_REQUIRED_OUTCOME_IDS.to_vec(),
    }
}

#[derive(Debug)]
struct McpControlledLivePaths {
    stderr_log_path: PathBuf,
    transcript_log_path: PathBuf,
    capture_dir: PathBuf,
}

struct McpJsonRpcSession {
    stdin: std::process::ChildStdin,
    stdout: BufReader<std::process::ChildStdout>,
    transcript: File,
    stdout_clean: bool,
}

struct KillOnDropChild {
    child: Child,
}

impl Drop for KillOnDropChild {
    fn drop(&mut self) {
        let process_group = format!("-{}", self.child.id());
        let _ = Command::new(MCP_CONTROL_TERMINATE_COMMAND)
            .arg(MCP_CONTROL_TERMINATE_SIGNAL)
            .arg(&process_group)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
        thread::sleep(Duration::from_millis(MCP_CONTROL_TERMINATE_GRACE_MILLIS));
        let _ = self.child.kill();
        let _ = self.child.wait();
        let _ = Command::new(MCP_CONTROL_TERMINATE_COMMAND)
            .arg(MCP_CONTROL_KILL_SIGNAL)
            .arg(&process_group)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    }
}

impl McpJsonRpcSession {
    fn request(&mut self, id: &str, request: &str) -> Result<String, String> {
        writeln!(self.transcript, "> {request}")
            .map_err(|err| format!("write transcript: {err}"))?;
        self.stdin
            .write_all(request.as_bytes())
            .map_err(|err| format!("write MCP request {id}: {err}"))?;
        self.stdin
            .write_all(b"\n")
            .map_err(|err| format!("write MCP request newline {id}: {err}"))?;
        self.stdin
            .flush()
            .map_err(|err| format!("flush MCP request {id}: {err}"))?;

        loop {
            let mut line = String::new();
            let bytes = self
                .stdout
                .read_line(&mut line)
                .map_err(|err| format!("read MCP response {id}: {err}"))?;
            if bytes == 0 {
                return Err(format!("MCP response stream closed before id {id}"));
            }
            let trimmed = line.trim_end_matches(['\r', '\n']);
            writeln!(self.transcript, "< {trimmed}")
                .map_err(|err| format!("write transcript: {err}"))?;
            if !mcp_stdout_line_is_clean_jsonrpc(trimmed) {
                self.stdout_clean = false;
                continue;
            }
            if mcp_response_matches_id(trimmed, id) {
                return Ok(trimmed.to_string());
            }
        }
    }
}

fn mcp_controlled_live_paths(cfg: &Config) -> Result<McpControlledLivePaths, String> {
    let (base_dir, stem) = match &cfg.receipt_path {
        Some(receipt_path) => {
            let parent = receipt_path
                .parent()
                .filter(|parent| !parent.as_os_str().is_empty())
                .unwrap_or_else(|| Path::new("."));
            let stem = receipt_path
                .file_stem()
                .and_then(|stem| stem.to_str())
                .unwrap_or(MCP_CONTROLLED_SMOKE_SCENARIO)
                .to_string();
            (parent.to_path_buf(), stem)
        }
        None => (
            cfg.target_dir.join(MCP_CONTROLLED_SMOKE_SCENARIO),
            MCP_CONTROLLED_SMOKE_SCENARIO.to_string(),
        ),
    };
    let base_dir = absolute_child_path(&cfg.root, &base_dir);
    fs::create_dir_all(&base_dir)
        .map_err(|err| format!("create MCP evidence dir {}: {err}", base_dir.display()))?;
    let capture_dir = base_dir.join(format!("{stem}-{MCP_CONTROL_LIVE_CAPTURE_DIR_SUFFIX}"));
    fs::create_dir_all(&capture_dir)
        .map_err(|err| format!("create MCP capture dir {}: {err}", capture_dir.display()))?;
    Ok(McpControlledLivePaths {
        stderr_log_path: base_dir.join(format!("{stem}.{MCP_CONTROL_LIVE_STDERR_LOG_EXTENSION}")),
        transcript_log_path: base_dir
            .join(format!("{stem}.{MCP_CONTROL_LIVE_TRANSCRIPT_EXTENSION}")),
        capture_dir,
    })
}

fn absolute_child_path(root: &Path, path: &Path) -> PathBuf {
    if path.is_absolute() {
        return path.to_path_buf();
    }
    root.join(path)
}

fn uses_isolated_restart_storage(scenario: Scenario) -> bool {
    scenario_behavior(scenario).uses_isolated_restart_storage()
}

fn world_persistence_artifact_dir_name(scenario: Scenario) -> &'static str {
    scenario_behavior(scenario).world_persistence_artifact_dir_name()
}

fn world_persistence_state_dir(cfg: &Config, backend: ServerBackend) -> PathBuf {
    let backend_name = backend_name(backend);
    cfg.root
        .join("target")
        .join(world_persistence_artifact_dir_name(cfg.scenario))
        .join(backend_name)
}

fn world_persistence_restart_phase_path(cfg: &Config) -> PathBuf {
    let backend_name = backend_name(cfg.server_backend);
    cfg.root
        .join("target")
        .join(format!(
            "{}-pre-restart",
            world_persistence_artifact_dir_name(cfg.scenario)
        ))
        .join(format!("{backend_name}.phase"))
}

fn world_persistence_phase_value(cfg: &Config) -> &'static str {
    if world_persistence_restart_phase_path(cfg).exists() {
        SURVIVAL_WORLD_PERSISTENCE_POST_RESTART_PHASE
    } else {
        SURVIVAL_WORLD_PERSISTENCE_INITIAL_PHASE
    }
}

fn mark_world_persistence_post_restart_phase(cfg: &Config) -> Result<(), String> {
    let path = world_persistence_restart_phase_path(cfg);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("create {}: {e}", parent.display()))?;
    }
    fs::write(&path, SURVIVAL_WORLD_PERSISTENCE_POST_RESTART_PHASE)
        .map_err(|e| format!("write {}: {e}", path.display()))
}

fn run_mcp_controlled_live_client(cfg: &Config) -> Result<ClientRunEvidence, String> {
    let paths = mcp_controlled_live_paths(cfg)?;
    let mut child = KillOnDropChild {
        child: spawn_mcp_controlled_client_process(cfg, &paths)?,
    };
    let stdin = child
        .child
        .stdin
        .take()
        .ok_or_else(|| "MCP client stdin pipe missing".to_string())?;
    let stdout = child
        .child
        .stdout
        .take()
        .ok_or_else(|| "MCP client stdout pipe missing".to_string())?;
    let transcript = File::create(&paths.transcript_log_path).map_err(|err| {
        format!(
            "create MCP transcript {}: {err}",
            paths.transcript_log_path.display()
        )
    })?;
    let mut session = McpJsonRpcSession {
        stdin,
        stdout: BufReader::new(stdout),
        transcript,
        stdout_clean: true,
    };
    let mut control = McpControlRunEvidence {
        handshake_success: false,
        tool_list_digest: mcp_control_tool_list_digest(),
        tool_names: MCP_CONTROL_TOOL_NAMES.to_vec(),
        calls_attempted: Vec::new(),
        calls_succeeded: Vec::new(),
        first_failure: None,
        stdout_clean: true,
        command_outcome_ids: Vec::new(),
    };

    control.calls_attempted.push("initialize");
    let initialize = session
        .request(
            MCP_CONTROL_INITIALIZE_ID,
            &mcp_jsonrpc_request(MCP_CONTROL_INITIALIZE_ID, "initialize", "{}"),
        )
        .map_err(|err| mcp_live_failure(&mut control, MCP_CONTROL_FAILURE_HANDSHAKE, err))?;
    if !mcp_response_has_result(&initialize) {
        return Err(mcp_live_failure(
            &mut control,
            MCP_CONTROL_FAILURE_HANDSHAKE,
            initialize,
        ));
    }
    control.handshake_success = true;
    control.calls_succeeded.push("initialize");

    control.calls_attempted.push("tools/list");
    let tools = session
        .request(
            MCP_CONTROL_TOOLS_LIST_ID,
            &mcp_jsonrpc_request(MCP_CONTROL_TOOLS_LIST_ID, "tools/list", "{}"),
        )
        .map_err(|err| mcp_live_failure(&mut control, MCP_CONTROL_FAILURE_TOOLS_LIST, err))?;
    if !mcp_tools_list_contains_required_tools(&tools) {
        return Err(mcp_live_failure(
            &mut control,
            MCP_CONTROL_FAILURE_TOOLS_LIST,
            tools,
        ));
    }
    control.calls_succeeded.push("tools/list");

    wait_for_mcp_connected_status(&mut session, &mut control)?;
    run_mcp_control_command(
        &mut session,
        &mut control,
        MCP_CONTROL_LOOK_ID,
        "tools/call look",
        "look.applied",
        r#"{"action":"look","yaw_delta":0.0,"pitch_delta":0.0}"#,
    )?;
    run_mcp_control_command(
        &mut session,
        &mut control,
        MCP_CONTROL_KEY_ID,
        "tools/call key",
        "key.applied",
        r#"{"action":"key","key":"jump","down":false}"#,
    )?;
    run_mcp_control_command(
        &mut session,
        &mut control,
        MCP_CONTROL_CHAT_ID,
        "tools/call chat",
        "chat.applied",
        r#"{"action":"chat","message":"mcp controlled smoke"}"#,
    )?;

    control
        .calls_attempted
        .push("tools/call capture_latest_frame");
    let capture_response = session
        .request(
            MCP_CONTROL_CAPTURE_ID,
            &mcp_capture_latest_frame_request(MCP_CONTROL_CAPTURE_ID),
        )
        .map_err(|err| mcp_live_failure(&mut control, MCP_CONTROL_FAILURE_FRAME_CAPTURE, err))?;
    let artifact = mcp_frame_artifact_from_response(&capture_response, &paths.capture_dir)
        .map_err(|err| mcp_live_failure(&mut control, MCP_CONTROL_FAILURE_FRAME_CAPTURE, err))?;
    control
        .calls_succeeded
        .push("tools/call capture_latest_frame");
    control
        .command_outcome_ids
        .push("capture_latest_frame.captured");
    control.stdout_clean = session.stdout_clean;
    if !control.stdout_clean {
        control.first_failure = Some(MCP_CONTROL_FAILURE_HANDSHAKE);
        return Err("MCP stdio stdout was contaminated by non-JSON-RPC output".to_string());
    }

    let output = mcp_controlled_success_output();
    let frame_artifacts = FrameArtifactsReceiptEvidence {
        selected: true,
        capture_requested: true,
        artifact_count: 1,
        artifacts: vec![artifact],
        missing_digests: Vec::new(),
        path_containment_checked: true,
        promotion_ready: paths
            .capture_dir
            .display()
            .to_string()
            .contains("docs/evidence/"),
        non_claims: FRAME_ARTIFACT_NON_CLAIMS.to_vec(),
    };
    Ok(ClientRunEvidence {
        log_path: Some(paths.transcript_log_path.clone()),
        log_paths: vec![paths.transcript_log_path, paths.stderr_log_path],
        usernames: planned_client_usernames(cfg),
        exit_code: None,
        classification: "mcp-controlled-live-evidence",
        matched_success_pattern: Some("mcp_command_outcomes".to_string()),
        scenario: Some(evaluate_scenario_for_config(cfg, &output)),
        server_scenario: Some(evaluate_server_scenario(
            cfg.scenario,
            "",
            &cfg.client_username,
        )),
        projectile_damage_causality: None,
        mcp_control: Some(control),
        frame_artifacts: Some(frame_artifacts),
    })
}

fn spawn_mcp_controlled_client_process(
    cfg: &Config,
    paths: &McpControlledLivePaths,
) -> Result<Child, String> {
    let err_file = File::create(&paths.stderr_log_path)
        .map_err(|err| format!("create {}: {err}", paths.stderr_log_path.display()))?;
    let mut cmd = Command::new(MCP_CONTROL_PROCESS_GROUP_COMMAND);
    cmd.arg("timeout")
        .arg(cfg.client_timeout.as_secs().to_string())
        .arg("xvfb-run")
        .arg("-a")
        .arg("-s")
        .arg(XVFB_SERVER_ARGS)
        .arg(cfg.target_dir.join("debug/stevenarella"))
        .arg("--server")
        .arg(format!("127.0.0.1:{}", cfg.server_port))
        .arg("--username")
        .arg(&cfg.client_username)
        .arg("--default-protocol-version")
        .arg(cfg.server_protocol.to_string())
        .arg("--mcp-stdio")
        .arg("--capture-dir")
        .arg(&paths.capture_dir)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::from(err_file));
    apply_build_env(&mut cmd, &cfg.target_dir);
    apply_headless_env(&mut cmd);
    apply_scenario_probe_env(&mut cmd, cfg.scenario, 0, cfg.server_backend);
    cmd.spawn()
        .map_err(|err| format!("run MCP-controlled client {}: {err}", cfg.client_username))
}

fn mcp_live_failure(
    control: &mut McpControlRunEvidence,
    first_failure: &'static str,
    detail: String,
) -> String {
    control.first_failure = Some(first_failure);
    format!("{first_failure}: {detail}")
}

fn mcp_jsonrpc_request(id: &str, method: &str, params_json: &str) -> String {
    format!(
        r#"{{"jsonrpc":"2.0","id":{id},"method":{method},"params":{params}}}"#,
        id = json_string(id),
        method = json_string(method),
        params = params_json,
    )
}

fn mcp_control_tool_call_request(id: &str, command_json: &str) -> String {
    format!(
        r#"{{"jsonrpc":"2.0","id":{id},"method":"tools/call","params":{{"name":"stevenarella.enqueue_control","arguments":{{"command":{command}}}}}}}"#,
        id = json_string(id),
        command = command_json,
    )
}

fn mcp_capture_latest_frame_request(id: &str) -> String {
    format!(
        r#"{{"jsonrpc":"2.0","id":{id},"method":"tools/call","params":{{"name":"stevenarella.capture_latest_frame","arguments":{{"output":"artifact","format":"png","relative_path":{relative_path},"include_ui":true}}}}}}"#,
        id = json_string(id),
        relative_path = json_string(MCP_CONTROL_LIVE_CAPTURE_RELATIVE_PATH),
    )
}

fn wait_for_mcp_connected_status(
    session: &mut McpJsonRpcSession,
    control: &mut McpControlRunEvidence,
) -> Result<(), String> {
    control.calls_attempted.push("tools/call status");
    for poll in 0..MCP_CONTROL_MAX_STATUS_POLLS {
        let id = format!("{MCP_CONTROL_STATUS_ID_PREFIX}-{poll}");
        let response = session
            .request(
                &id,
                &mcp_control_tool_call_request(&id, r#"{"action":"status"}"#),
            )
            .map_err(|err| mcp_live_failure(control, MCP_CONTROL_FAILURE_COMMAND, err))?;
        if mcp_control_response_applied(&response) {
            if !control.calls_succeeded.contains(&"tools/call status") {
                control.calls_succeeded.push("tools/call status");
                control.command_outcome_ids.push("status.applied");
            }
            if response.contains(MCP_CONTROL_CONNECTED_TOKEN) {
                return Ok(());
            }
        }
        thread::sleep(Duration::from_millis(MCP_CONTROL_STATUS_POLL_MILLIS));
    }
    Err(mcp_live_failure(
        control,
        MCP_CONTROL_FAILURE_STATUS_TIMEOUT,
        "status never reported connected=true".to_string(),
    ))
}

fn run_mcp_control_command(
    session: &mut McpJsonRpcSession,
    control: &mut McpControlRunEvidence,
    id: &'static str,
    call_label: &'static str,
    outcome_id: &'static str,
    command_json: &str,
) -> Result<(), String> {
    control.calls_attempted.push(call_label);
    let response = session
        .request(id, &mcp_control_tool_call_request(id, command_json))
        .map_err(|err| mcp_live_failure(control, MCP_CONTROL_FAILURE_COMMAND, err))?;
    if !mcp_control_response_applied(&response) {
        return Err(mcp_live_failure(
            control,
            MCP_CONTROL_FAILURE_COMMAND,
            response,
        ));
    }
    control.calls_succeeded.push(call_label);
    control.command_outcome_ids.push(outcome_id);
    Ok(())
}

fn mcp_stdout_line_is_clean_jsonrpc(line: &str) -> bool {
    line.starts_with('{') && line.contains(MCP_CONTROL_JSONRPC_VERSION_NEEDLE)
}

fn mcp_response_matches_id(line: &str, id: &str) -> bool {
    line.contains(&format!("\"id\":{}", json_string(id)))
}

fn mcp_response_has_result(line: &str) -> bool {
    line.contains(MCP_CONTROL_RESULT_NEEDLE) && !line.contains("\"error\"")
}

fn mcp_tools_list_contains_required_tools(line: &str) -> bool {
    mcp_response_has_result(line)
        && line.contains(MCP_CONTROL_TOOLS_ARRAY_NEEDLE)
        && MCP_CONTROL_TOOL_NAMES
            .iter()
            .all(|tool| line.contains(tool))
}

fn mcp_control_response_applied(line: &str) -> bool {
    mcp_response_has_result(line) && line.contains(MCP_CONTROL_OUTCOME_APPLIED_ESCAPED)
}

fn mcp_frame_artifact_from_response(
    response: &str,
    capture_dir: &Path,
) -> Result<FrameArtifactReceiptItem, String> {
    if !mcp_response_has_result(response) {
        return Err(format!("capture response was not successful: {response}"));
    }
    let metadata = json_string_field(response, "text")?;
    let relative_path = json_string_field(&metadata, "relative_path")?;
    let relative = PathBuf::from(&relative_path);
    if !relative_artifact_path_is_contained(&relative) {
        return Err(format!(
            "capture artifact path escapes capture dir: {relative_path}"
        ));
    }
    let artifact_path = capture_dir.join(&relative);
    let artifact_bytes = fs::read(&artifact_path)
        .map_err(|err| format!("read capture artifact {}: {err}", artifact_path.display()))?;
    let actual_digest = blake3::hash(&artifact_bytes).to_hex().to_string();
    let recorded_digest = json_string_field(&metadata, "blake3_digest")?;
    if recorded_digest != actual_digest {
        return Err(format!(
            "capture artifact digest mismatch for {}: metadata={} actual={}",
            artifact_path.display(),
            recorded_digest,
            actual_digest
        ));
    }
    Ok(FrameArtifactReceiptItem {
        path: artifact_path.display().to_string(),
        relative_path,
        format: json_string_field(&metadata, "format")?,
        width_px: json_u32_field(&metadata, "width_px")?,
        height_px: json_u32_field(&metadata, "height_px")?,
        frame_id: json_u64_field(&metadata, "frame_id")?,
        sequence_id: json_u64_field(&metadata, "sequence_id")?,
        byte_len: json_u64_field(&metadata, "byte_len")?,
        blake3: recorded_digest,
        redaction: json_string_field(&metadata, "redaction")?,
        includes_ui: json_bool_field(&metadata, "includes_ui")?,
    })
}

fn relative_artifact_path_is_contained(path: &Path) -> bool {
    let mut saw_component = false;
    for component in path.components() {
        match component {
            std::path::Component::Normal(name) if !name.is_empty() => saw_component = true,
            _ => return false,
        }
    }
    saw_component
}

fn projectile_damage_dry_run_evidence(cfg: &Config) -> ClientRunEvidence {
    let attacker_username = format!(
        "{}{}",
        cfg.client_username, PROJECTILE_DAMAGE_ATTACKER_SUFFIX
    );
    let victim_username = format!("{}{}", cfg.client_username, PROJECTILE_DAMAGE_VICTIM_SUFFIX);
    let attacker_log = format!(
        "Detected server protocol version {}\njoin_game\nrender_tick_with_player\nYou are on team RED!\nremote_player_spawn\n{} hand=main {}\n{} hand=main\n",
        cfg.server_protocol,
        PROJECTILE_DAMAGE_CLIENT_USE_NEEDLE,
        PROJECTILE_DAMAGE_SEQUENCE_NEEDLE,
        PROJECTILE_DAMAGE_CLIENT_SWING_NEEDLE
    );
    let client_health_needle = projectile_damage_client_health_needle(cfg);
    let server_damage_needle = projectile_damage_amount_needle(cfg);
    let server_health_after_needle = projectile_damage_server_health_after_needle(cfg);
    let victim_log = format!(
        "Detected server protocol version {}\njoin_game\nrender_tick_with_player\nYou are on team BLUE!\nremote_player_spawn\n{}\n",
        cfg.server_protocol,
        client_health_needle
    );
    let server_log = format!(
        "{attacker_username} joined\n{victim_username} joined\nMC-COMPAT-MILESTONE projectile_loadout username={attacker_username} slot=0 item=Bow arrows=16\n{} attacker={attacker_username} victim={victim_username} hand=Main {} {}\n{} attacker={attacker_username} victim={victim_username} victim_health_before=20.0 {}\n",
        PROJECTILE_DAMAGE_SERVER_USE_NEEDLE,
        PROJECTILE_DAMAGE_SEQUENCE_NEEDLE,
        server_damage_needle,
        PROJECTILE_DAMAGE_SERVER_HIT_NEEDLE,
        server_health_after_needle
    );
    let combined_output =
        format!("mc_compat_projectile_damage_client_count=2\n{attacker_log}{victim_log}");
    let client_logs = [
        ClientLogSlice {
            username: &attacker_username,
            output: &attacker_log,
        },
        ClientLogSlice {
            username: &victim_username,
            output: &victim_log,
        },
    ];
    let scenario = evaluate_scenario_for_config(cfg, &combined_output);
    let server_scenario = evaluate_server_scenario(cfg.scenario, &server_log, &cfg.client_username);
    let projectile_damage_causality = evaluate_projectile_damage_causality_for_damage(
        &client_logs,
        &server_log,
        &cfg.client_username,
        &server_damage_needle,
    );
    ClientRunEvidence {
        log_path: None,
        log_paths: Vec::new(),
        usernames: vec![attacker_username, victim_username],
        exit_code: None,
        classification: "dry-run",
        matched_success_pattern: Some("Detected server protocol version".to_string()),
        scenario: Some(scenario),
        server_scenario: Some(server_scenario),
        projectile_damage_causality: Some(projectile_damage_causality),
        mcp_control: None,
        frame_artifacts: None,
    }
}

fn run_reconnect_sequence_scenario(cfg: &Config) -> Result<Vec<SingleClientRun>, String> {
    let username = cfg.client_username.clone();
    let scenario = scenario_name(cfg.scenario);
    let mut runs = Vec::new();
    let mut restarted_server: Option<ManagedServer> = None;
    for idx in 0..RECONNECT_SEQUENCE_SESSION_COUNT {
        let log_path = std::env::temp_dir().join(format!(
            "mc-compat-client.{username}.{scenario}-session-{}.{}.log",
            idx + 1,
            std::process::id()
        ));
        let mut child = spawn_client_process(cfg, &username, idx, &log_path)?;
        let status = child
            .wait()
            .map_err(|e| format!("wait {scenario} client session {}: {e}", idx + 1))?;
        let output = fs::read_to_string(&log_path)
            .map_err(|e| format!("read {}: {e}", log_path.display()))?;
        let matched_success_pattern = cfg
            .client_success_needles
            .iter()
            .find(|needle| output.contains(needle.as_str()))
            .cloned();
        runs.push(SingleClientRun {
            username: username.clone(),
            log_path,
            exit_code: status.code(),
            output,
            matched_success_pattern,
        });
        if uses_isolated_restart_storage(cfg.scenario) && idx == FIRST_CLIENT_INDEX {
            restarted_server = Some(run_world_persistence_restart_transition(cfg)?);
        }
        thread::sleep(Duration::from_secs(RECONNECT_SEQUENCE_PAUSE_SECS));
    }
    if restarted_server.is_some() {
        append_world_persistence_post_restart_server_log(cfg)?;
    }
    drop(restarted_server);
    Ok(runs)
}

fn run_world_persistence_restart_transition(cfg: &Config) -> Result<ManagedServer, String> {
    let behavior = scenario_behavior(cfg.scenario);
    write_world_persistence_pre_restart_server_log(cfg)?;
    if behavior.uses_crash_recovery_restart() {
        force_stop_server(cfg)?;
        append_world_persistence_orchestration_milestone(
            cfg,
            SURVIVAL_CRASH_RECOVERY_SERVER_FORCED_STOP_NEEDLE,
        )?;
    } else {
        stop_server(cfg)?;
        append_world_persistence_orchestration_milestone(cfg, restart_clean_milestone(behavior))?;
    }
    mark_world_persistence_post_restart_phase(cfg)?;
    let restarted_server = start_server(cfg)?;
    probe_status(cfg)?;
    if behavior.uses_crash_recovery_restart() {
        append_world_persistence_orchestration_milestone(
            cfg,
            SURVIVAL_CRASH_RECOVERY_SERVER_RESTART_NEEDLE,
        )?;
    } else {
        append_world_persistence_orchestration_milestone(cfg, restart_backend_milestone(behavior))?;
    }
    Ok(restarted_server)
}

fn restart_clean_milestone(behavior: &'static dyn ScenarioBehavior) -> &'static str {
    if behavior.uses_block_entity_persistence_storage() {
        SURVIVAL_BLOCK_ENTITY_SERVER_CLEAN_NEEDLE
    } else if behavior.uses_world_multichunk_storage() {
        SURVIVAL_WORLD_MULTICHUNK_SERVER_CLEAN_NEEDLE
    } else {
        SURVIVAL_WORLD_PERSISTENCE_SERVER_CLEAN_NEEDLE
    }
}

fn restart_backend_milestone(behavior: &'static dyn ScenarioBehavior) -> &'static str {
    if behavior.uses_block_entity_persistence_storage() {
        SURVIVAL_BLOCK_ENTITY_SERVER_RESTART_NEEDLE
    } else if behavior.uses_world_multichunk_storage() {
        SURVIVAL_WORLD_MULTICHUNK_SERVER_RESTART_NEEDLE
    } else {
        SURVIVAL_WORLD_PERSISTENCE_SERVER_RESTART_NEEDLE
    }
}

fn run_multi_client_load_scenario(cfg: &Config) -> Result<Vec<SingleClientRun>, String> {
    let usernames = planned_client_usernames(cfg);
    let mut children = Vec::new();
    for (idx, username) in usernames.iter().enumerate() {
        let log_path = temp_client_log_for(username);
        let child = spawn_client_process(cfg, username, idx, &log_path)?;
        children.push((username.clone(), log_path, child));
        if cfg.scenario != Scenario::CtfSimultaneousPickupCaptureRace {
            thread::sleep(Duration::from_secs(MULTI_CLIENT_START_STAGGER_SECS));
        }
    }
    let mut runs = Vec::new();
    for (username, log_path, mut child) in children {
        let status = child
            .wait()
            .map_err(|e| format!("wait client {username}: {e}"))?;
        let output = fs::read_to_string(&log_path)
            .map_err(|e| format!("read {}: {e}", log_path.display()))?;
        let matched_success_pattern = cfg
            .client_success_needles
            .iter()
            .find(|needle| output.contains(needle.as_str()))
            .cloned();
        runs.push(SingleClientRun {
            username,
            log_path,
            exit_code: status.code(),
            output,
            matched_success_pattern,
        });
    }
    Ok(runs)
}

fn run_single_client(
    cfg: &Config,
    username: &str,
    client_index: usize,
) -> Result<SingleClientRun, String> {
    let log_path = env_path("CLIENT_LOG").unwrap_or_else(|| temp_client_log_for(username));
    let mut child = spawn_client_process(cfg, username, client_index, &log_path)?;
    let status = child.wait().map_err(|e| format!("wait client: {e}"))?;
    let output =
        fs::read_to_string(&log_path).map_err(|e| format!("read {}: {e}", log_path.display()))?;
    let matched_success_pattern = cfg
        .client_success_needles
        .iter()
        .find(|needle| output.contains(needle.as_str()))
        .cloned();
    Ok(SingleClientRun {
        username: username.to_string(),
        log_path,
        exit_code: status.code(),
        output,
        matched_success_pattern,
    })
}

fn derive_survival_crash_recovery_client_milestones(output: &str) -> String {
    let mut derived = String::new();
    append_derived_line_if_contains(
        &mut derived,
        output,
        SURVIVAL_WORLD_PERSISTENCE_CLIENT_MUTATION_NEEDLE,
        SURVIVAL_CRASH_RECOVERY_CLIENT_MUTATION_NEEDLE,
    );
    append_derived_line_if_contains(
        &mut derived,
        output,
        SURVIVAL_WORLD_PERSISTENCE_CLIENT_PRE_RESTART_NEEDLE,
        SURVIVAL_CRASH_RECOVERY_CLIENT_PRE_CRASH_NEEDLE,
    );
    append_derived_line_if_contains(
        &mut derived,
        output,
        SURVIVAL_WORLD_PERSISTENCE_CLIENT_RECONNECT_NEEDLE,
        SURVIVAL_CRASH_RECOVERY_CLIENT_RECONNECT_NEEDLE,
    );
    append_derived_line_if_contains(
        &mut derived,
        output,
        SURVIVAL_WORLD_PERSISTENCE_CLIENT_POST_RESTART_NEEDLE,
        SURVIVAL_CRASH_RECOVERY_CLIENT_POST_CRASH_NEEDLE,
    );
    derived
}

fn derive_survival_crash_recovery_server_milestones(log: &str) -> String {
    let mut derived = String::new();
    append_derived_line_if_contains(
        &mut derived,
        log,
        SURVIVAL_WORLD_PERSISTENCE_SERVER_MUTATION_NEEDLE,
        SURVIVAL_CRASH_RECOVERY_SERVER_MUTATION_NEEDLE,
    );
    append_derived_line_if_contains(
        &mut derived,
        log,
        SURVIVAL_WORLD_PERSISTENCE_SERVER_POST_NEEDLE,
        SURVIVAL_CRASH_RECOVERY_SERVER_POST_NEEDLE,
    );
    if log.contains(SURVIVAL_WORLD_PERSISTENCE_SERVER_STATE_NEEDLE)
        && log.contains(SURVIVAL_CRASH_RECOVERY_SERVER_FORCED_STOP_NEEDLE)
        && log.contains(SURVIVAL_CRASH_RECOVERY_SERVER_RESTART_NEEDLE)
    {
        append_derived_line(&mut derived, SURVIVAL_CRASH_RECOVERY_SERVER_STATE_NEEDLE);
    }
    derived
}

fn append_derived_line_if_contains(
    output: &mut String,
    haystack: &str,
    source_needle: &str,
    derived_line: &str,
) {
    if haystack.contains(source_needle) {
        append_derived_line(output, derived_line);
    }
}

fn append_derived_line(output: &mut String, line: &str) {
    output.push_str(line);
    output.push('\n');
}

fn spawn_client_process(
    cfg: &Config,
    username: &str,
    client_index: usize,
    log_path: &Path,
) -> Result<Child, String> {
    let log_file =
        File::create(log_path).map_err(|e| format!("create {}: {e}", log_path.display()))?;
    let err_file = log_file
        .try_clone()
        .map_err(|e| format!("clone client log handle: {e}"))?;
    let mut cmd = Command::new("timeout");
    cmd.arg(client_timeout_secs(cfg, client_index).to_string())
        .arg("xvfb-run")
        .arg("-a")
        .arg("-s")
        .arg(XVFB_SERVER_ARGS)
        .arg(cfg.target_dir.join("debug/stevenarella"))
        .arg("--server")
        .arg(format!("127.0.0.1:{}", cfg.server_port))
        .arg("--username")
        .arg(username)
        .arg("--default-protocol-version")
        .arg(cfg.server_protocol.to_string())
        .stdout(Stdio::from(log_file))
        .stderr(Stdio::from(err_file));
    apply_build_env(&mut cmd, &cfg.target_dir);
    apply_headless_env(&mut cmd);
    apply_scenario_probe_env(&mut cmd, cfg.scenario, client_index, cfg.server_backend);
    cmd.spawn()
        .map_err(|e| format!("run client {username}: {e}"))
}

fn client_timeout_secs(cfg: &Config, client_index: usize) -> u64 {
    if cfg.scenario == Scenario::MultiClientLoadScore && client_index > 0 {
        cfg.client_timeout
            .as_secs()
            .min(MULTI_CLIENT_LOAD_PEER_TIMEOUT_SECS)
    } else {
        cfg.client_timeout.as_secs()
    }
}

fn apply_scenario_probe_env(
    cmd: &mut Command,
    scenario: Scenario,
    client_index: usize,
    server_backend: ServerBackend,
) {
    scenario_behavior(scenario).apply_client_probe_env(cmd, client_index, server_backend);
}

fn planned_client_usernames(cfg: &Config) -> Vec<String> {
    if scenario_behavior(cfg.scenario).run_strategy() == ScenarioRunStrategy::MultiClient {
        vec![
            format!("{}a", cfg.client_username),
            format!("{}b", cfg.client_username),
        ]
    } else {
        vec![cfg.client_username.clone()]
    }
}

fn server_log_label(cfg: &Config) -> String {
    cfg.server_backend.runtime().log_label(cfg)
}

fn read_server_scenario_evidence(
    cfg: &Config,
    runs: &[SingleClientRun],
) -> Result<Option<ServerScenarioEvidence>, String> {
    let server_log = cfg.server_backend.runtime().read_log(cfg)?;
    let mut correlation_log = server_log;
    if uses_isolated_restart_storage(cfg.scenario) {
        correlation_log.push('\n');
        correlation_log.push_str(&read_world_persistence_pre_restart_server_log(cfg)?);
    }
    for run in runs {
        correlation_log.push('\n');
        correlation_log.push_str(&run.output);
    }
    if scenario_behavior(cfg.scenario).uses_crash_recovery_restart() {
        let derived = derive_survival_crash_recovery_server_milestones(&correlation_log);
        correlation_log.push_str(&derived);
    }
    let username = &cfg.client_username;
    Ok(Some(evaluate_server_scenario(
        cfg.scenario,
        &correlation_log,
        username,
    )))
}

fn world_persistence_pre_restart_server_log_path(cfg: &Config) -> PathBuf {
    let backend_name = backend_name(cfg.server_backend);
    cfg.root
        .join("target")
        .join(format!(
            "{}-pre-restart",
            world_persistence_artifact_dir_name(cfg.scenario)
        ))
        .join(format!("{backend_name}.log"))
}

fn write_world_persistence_pre_restart_server_log(cfg: &Config) -> Result<(), String> {
    let text = cfg.server_backend.runtime().read_log(cfg)?;
    let path = world_persistence_pre_restart_server_log_path(cfg);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("create {}: {e}", parent.display()))?;
    }
    fs::write(&path, text).map_err(|e| format!("write {}: {e}", path.display()))
}

fn append_world_persistence_orchestration_milestone(
    cfg: &Config,
    milestone: &str,
) -> Result<(), String> {
    append_world_persistence_pre_restart_server_log(
        cfg,
        &format!("MC-COMPAT-MILESTONE {milestone}\n"),
    )
}

fn append_world_persistence_post_restart_server_log(cfg: &Config) -> Result<(), String> {
    let text = cfg.server_backend.runtime().read_log(cfg)?;
    append_world_persistence_pre_restart_server_log(cfg, &text)
}

fn append_world_persistence_pre_restart_server_log(cfg: &Config, text: &str) -> Result<(), String> {
    let path = world_persistence_pre_restart_server_log_path(cfg);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("create {}: {e}", parent.display()))?;
    }
    let mut options = fs::OpenOptions::new();
    options.create(true).append(true);
    options
        .open(&path)
        .and_then(|mut file| file.write_all(text.as_bytes()))
        .map_err(|e| format!("append {}: {e}", path.display()))
}

fn read_world_persistence_pre_restart_server_log(cfg: &Config) -> Result<String, String> {
    let path = world_persistence_pre_restart_server_log_path(cfg);
    match fs::read_to_string(&path) {
        Ok(text) => Ok(text),
        Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(String::new()),
        Err(err) => Err(format!("read {}: {err}", path.display())),
    }
}

fn read_valence_log(cfg: &Config) -> Result<String, String> {
    match fs::read_to_string(&cfg.valence_log) {
        Ok(text) => Ok(text),
        Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(String::new()),
        Err(err) => Err(format!("read {}: {err}", cfg.valence_log.display())),
    }
}

fn read_paper_log(cfg: &Config) -> Result<String, String> {
    if cfg.mode == Mode::DryRun {
        return Ok(String::new());
    }
    let output = Command::new("docker")
        .arg("logs")
        .arg(&cfg.server_name)
        .output()
        .map_err(|e| format!("docker logs {}: {e}", cfg.server_name))?;
    let mut text = String::new();
    text.push_str(&String::from_utf8_lossy(&output.stdout));
    text.push_str(&String::from_utf8_lossy(&output.stderr));
    Ok(text)
}

fn requires_server_correlation(cfg: &Config) -> bool {
    scenario_behavior(cfg.scenario).requires_server_correlation()
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

fn harness_plan_from_config(cfg: &Config) -> Result<HarnessPlan, Vec<PlanningDiagnostic>> {
    let mut diagnostics = Vec::new();
    let receipt = receipt_output_plan_from_config(cfg, &mut diagnostics);
    let artifacts = artifact_collection_plan_from_config(cfg, &mut diagnostics);
    let cleanup = cleanup_plan_from_config(cfg, &mut diagnostics);
    let matrix = matrix_plan_from_config(cfg, &mut diagnostics);
    let plan = HarnessPlan {
        server: server_startup_plan_from_config(cfg),
        client_sessions: client_session_plans_from_config(cfg),
        receipt,
        artifacts,
        cleanup,
        matrix,
        non_claims: vec![PLAN_NON_CLAIM_ARCHITECTURE_ONLY.to_string()],
    };
    if diagnostics.is_empty() {
        Ok(plan)
    } else {
        Err(diagnostics)
    }
}

fn server_startup_plan_from_config(cfg: &Config) -> ServerStartupPlan {
    let (valence_worktree, valence_log, docker_image) = match cfg.server_backend {
        ServerBackend::Valence => (
            Some(cfg.valence_worktree.display().to_string()),
            Some(cfg.valence_log.display().to_string()),
            None,
        ),
        ServerBackend::Paper => (None, None, Some(cfg.docker_image.clone())),
    };
    ServerStartupPlan {
        backend: backend_name(cfg.server_backend).to_string(),
        protocol: cfg.server_protocol,
        port: cfg.server_port,
        server_name: cfg.server_name.clone(),
        keep_server: cfg.keep_server || cfg.mode == Mode::DryRun,
        eula_acceptance_required: cfg.server_backend == ServerBackend::Paper,
        valence_worktree,
        valence_log,
        docker_image,
    }
}

fn client_session_plans_from_config(cfg: &Config) -> Vec<ClientSessionPlan> {
    let usernames = planned_client_usernames(cfg);
    let session_count = planned_client_session_count(cfg);
    usernames
        .into_iter()
        .enumerate()
        .map(|(index, username)| ClientSessionPlan {
            index,
            username,
            timeout_secs: client_timeout_secs(cfg, index),
            scenario: scenario_name(cfg.scenario).to_string(),
            session_count,
            log_path_strategy: client_log_path_strategy(cfg),
        })
        .collect()
}

fn planned_client_session_count(cfg: &Config) -> usize {
    if scenario_behavior(cfg.scenario).run_strategy() == ScenarioRunStrategy::ReconnectSequence {
        RECONNECT_SEQUENCE_SESSION_COUNT
    } else {
        1
    }
}

fn client_log_path_strategy(cfg: &Config) -> String {
    match scenario_behavior(cfg.scenario).run_strategy() {
        ScenarioRunStrategy::ReconnectSequence => PLAN_CLIENT_LOG_RECONNECT_TEMP.to_string(),
        ScenarioRunStrategy::MultiClient => PLAN_CLIENT_LOG_TEMP.to_string(),
        ScenarioRunStrategy::SingleClient => PLAN_CLIENT_LOG_ENV_OR_TEMP.to_string(),
    }
}

fn receipt_output_plan_from_config(
    cfg: &Config,
    diagnostics: &mut Vec<PlanningDiagnostic>,
) -> ReceiptOutputPlan {
    let receipt_path = cfg.receipt_path.as_ref().map(|path| display_path(path));
    let receipt_dir = cfg.receipt_dir.as_ref().map(|path| display_path(path));
    let failure_bundle_path = cfg
        .failure_bundle_path
        .as_ref()
        .map(|path| display_path(path));
    if cfg.failure_bundle_path.is_some() && cfg.receipt_path.is_none() {
        push_plan_diagnostic(
            diagnostics,
            "failure_bundle_path",
            "failure bundle planning requires a receipt path for reviewable artifact identity",
        );
    }
    if let Some(path) = &cfg.failure_bundle_path {
        validate_reviewable_plan_path(
            diagnostics,
            "failure_bundle_path",
            &cfg.root,
            path,
            "failure bundle output",
        );
    }
    ReceiptOutputPlan {
        receipt_path,
        receipt_dir,
        failure_bundle_path,
        schema: SCENARIO_RECEIPT_SCHEMA.to_string(),
    }
}

fn artifact_collection_plan_from_config(
    cfg: &Config,
    diagnostics: &mut Vec<PlanningDiagnostic>,
) -> ArtifactCollectionPlan {
    let typed_event_log_path = cfg
        .receipt_path
        .as_ref()
        .map(|path| display_path(&typed_event_log_path_for_receipt(path)));
    let failure_bundle_path = cfg
        .failure_bundle_path
        .as_ref()
        .map(|path| display_path(path));
    let mut failure_artifact_candidates = Vec::new();
    for (kind, path) in failure_bundle_artifact_candidates(cfg) {
        if cfg.failure_bundle_path.is_some() && kind != FAILURE_BUNDLE_ARTIFACT_SERVER_LOG {
            validate_reviewable_plan_path(
                diagnostics,
                "failure_artifact_candidate",
                &cfg.root,
                &path,
                kind,
            );
        }
        failure_artifact_candidates.push(ArtifactCandidatePlan {
            kind: kind.to_string(),
            path: display_path(&path),
        });
    }
    ArtifactCollectionPlan {
        typed_event_log_path,
        failure_bundle_path,
        failure_artifact_candidates,
    }
}

fn cleanup_plan_from_config(
    cfg: &Config,
    diagnostics: &mut Vec<PlanningDiagnostic>,
) -> CleanupPlan {
    if cfg.mode == Mode::Cleanup {
        validate_cleanup_plan_path(
            diagnostics,
            &cfg.root,
            "valence pid file",
            &cfg.valence_pid_file,
        );
        validate_cleanup_plan_path(
            diagnostics,
            &cfg.root,
            "valence target dir",
            &cfg.valence_target_dir,
        );
        validate_cleanup_plan_path(diagnostics, &cfg.root, "valence log", &cfg.valence_log);
    }
    CleanupPlan {
        apply: cfg.cleanup_apply,
        paper_container: cfg.server_name.clone(),
        valence_pid_file: display_path(&cfg.valence_pid_file),
        path_actions: vec![
            CleanupPathPlan {
                label: "valence target dir".to_string(),
                path: display_path(&cfg.valence_target_dir),
            },
            CleanupPathPlan {
                label: "valence log".to_string(),
                path: display_path(&cfg.valence_log),
            },
        ],
        client_log_discovery: PLAN_CLEANUP_CLIENT_LOG_DISCOVERY.to_string(),
    }
}

fn matrix_plan_from_config(
    cfg: &Config,
    diagnostics: &mut Vec<PlanningDiagnostic>,
) -> Option<MatrixPlan> {
    if cfg.mode != Mode::RunMatrix {
        return None;
    }
    if cfg.receipt_path.is_some() {
        push_plan_diagnostic(
            diagnostics,
            "receipt_path",
            "run-matrix planning writes backend receipts under receipt_dir and rejects a single receipt path",
        );
    }
    let receipt_dir = cfg
        .receipt_dir
        .clone()
        .unwrap_or_else(|| cfg.root.join(DEFAULT_MATRIX_RECEIPT_DIR));
    let paper_receipt = receipt_dir.join("paper.json");
    let valence_receipt = receipt_dir.join("valence.json");
    Some(MatrixPlan {
        dry_run: cfg.matrix_dry_run,
        matrix_mode: if cfg.matrix_dry_run { "dry-run" } else { "run" }.to_string(),
        receipt_dir: display_path(&receipt_dir),
        paper_receipt: display_path(&paper_receipt),
        valence_receipt: display_path(&valence_receipt),
    })
}

fn validate_reviewable_plan_path(
    diagnostics: &mut Vec<PlanningDiagnostic>,
    field: &str,
    root: &Path,
    path: &Path,
    label: &str,
) {
    let Some(review_path) = plan_reviewable_path(root, path) else {
        push_plan_diagnostic(
            diagnostics,
            field,
            &format!("{label} path must be under docs/evidence for review"),
        );
        return;
    };
    if let Err(err) = validate_failure_bundle_artifact_path(&review_path) {
        push_plan_diagnostic(diagnostics, field, &err);
    }
}

fn plan_reviewable_path(root: &Path, path: &Path) -> Option<String> {
    if path.is_absolute() {
        let relative = path.strip_prefix(root).ok()?;
        return path_to_forward_slashes(relative);
    }
    path_to_forward_slashes(path)
}

fn validate_cleanup_plan_path(
    diagnostics: &mut Vec<PlanningDiagnostic>,
    root: &Path,
    label: &str,
    path: &Path,
) {
    if path.as_os_str().is_empty() {
        push_plan_diagnostic(diagnostics, "cleanup", &format!("{label} path is empty"));
        return;
    }
    if path == Path::new(CLEANUP_ROOT_PATH)
        || cleanup_component_count(path) < CLEANUP_MIN_SAFE_COMPONENTS
    {
        push_plan_diagnostic(
            diagnostics,
            "cleanup",
            &format!("{label} path is too broad for cleanup: {}", path.display()),
        );
        return;
    }
    if path
        .components()
        .any(|component| matches!(component, std::path::Component::ParentDir))
    {
        push_plan_diagnostic(
            diagnostics,
            "cleanup",
            &format!("{label} path contains parent traversal: {}", path.display()),
        );
        return;
    }
    let absolute_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        root.join(path)
    };
    let target_root = root.join("target");
    if !absolute_path.starts_with(Path::new(HARNESS_TEMP_ROOT))
        && !absolute_path.starts_with(target_root)
    {
        push_plan_diagnostic(
            diagnostics,
            "cleanup",
            &format!(
                "{label} path is outside harness-owned cleanup roots: {}",
                path.display()
            ),
        );
    }
}

fn cleanup_component_count(path: &Path) -> usize {
    path.components()
        .filter(|component| matches!(component, std::path::Component::Normal(_)))
        .count()
}

fn push_plan_diagnostic(diagnostics: &mut Vec<PlanningDiagnostic>, field: &str, message: &str) {
    diagnostics.push(PlanningDiagnostic {
        field: field.to_string(),
        message: message.to_string(),
    });
}

fn format_plan_diagnostics(diagnostics: Vec<PlanningDiagnostic>) -> String {
    let rendered = diagnostics
        .into_iter()
        .map(|diagnostic| format!("{}: {}", diagnostic.field, diagnostic.message))
        .collect::<Vec<_>>();
    format!("harness planning failed: {}", rendered.join("; "))
}

fn display_path(path: &Path) -> String {
    path.display().to_string()
}

fn log_harness_plan(plan: &HarnessPlan) {
    log(format_args!(
        "plan: build client, start {} server, wait for protocol {}, run {} client session plan(s) under isolated Xvfb/X11",
        plan_backend_display_name(&plan.server.backend),
        plan.server.protocol,
        plan.client_sessions.len()
    ));
}

fn plan_backend_display_name(backend: &str) -> &str {
    match backend {
        "paper" => "Paper",
        "valence" => "Valence",
        _ => backend,
    }
}

const FAILURE_BUNDLE_SCHEMA: &str = "mc.compat.failure.bundle.v1";
const FAILURE_BUNDLE_OUTCOME_FAILED: &str = "failed";
const FAILURE_BUNDLE_OUTCOME_BLOCKED: &str = "blocked";
const FAILURE_BUNDLE_ARTIFACT_RECEIPT: &str = "receipt";
const FAILURE_BUNDLE_ARTIFACT_TYPED_EVENTS: &str = "typed_events";
const FAILURE_BUNDLE_ARTIFACT_MCP_TRANSCRIPT: &str = "mcp_transcript";
const FAILURE_BUNDLE_ARTIFACT_STDERR: &str = "stderr";
const FAILURE_BUNDLE_ARTIFACT_SERVER_LOG: &str = "server_log";
const FAILURE_BUNDLE_REVIEW_STORAGE_PREFIX: &str = "docs/evidence/";
const FAILURE_BUNDLE_TARGET_COMPONENT: &str = "target";
const FAILURE_BUNDLE_HASH_BUFFER_BYTES: usize = 8192;
const FAILURE_BUNDLE_BLAKE3_HEX_CHARS: usize = 64;
const FAILURE_BUNDLE_MAX_FIRST_FAILURE_CHARS: usize = 512;
const FAILURE_BUNDLE_MAX_COMMAND_SUMMARY_CHARS: usize = 256;
const FAILURE_BUNDLE_NON_CLAIM_SCENARIO_SUCCESS: &str = "scenario_success";
const FAILURE_BUNDLE_NON_CLAIM_GAMEPLAY_PARITY: &str = "gameplay_parity";
const FAILURE_BUNDLE_NON_CLAIM_FULL_PROTOCOL: &str = "full_protocol_compatibility";
const FAILURE_BUNDLE_NON_CLAIM_PUBLIC_SERVER_SAFETY: &str = "public_server_safety";
const FAILURE_BUNDLE_NON_CLAIM_PRODUCTION_READINESS: &str = "production_readiness";
const FAILURE_BUNDLE_NON_CLAIM_SEMANTIC_EQUIVALENCE: &str = "semantic_equivalence";
const FAILURE_BUNDLE_REQUIRED_NON_CLAIMS: &[&str] = &[
    FAILURE_BUNDLE_NON_CLAIM_SCENARIO_SUCCESS,
    FAILURE_BUNDLE_NON_CLAIM_GAMEPLAY_PARITY,
    FAILURE_BUNDLE_NON_CLAIM_FULL_PROTOCOL,
    FAILURE_BUNDLE_NON_CLAIM_PUBLIC_SERVER_SAFETY,
    FAILURE_BUNDLE_NON_CLAIM_PRODUCTION_READINESS,
    FAILURE_BUNDLE_NON_CLAIM_SEMANTIC_EQUIVALENCE,
];

#[derive(Debug, Clone, PartialEq, Eq)]
struct FailureBundleArtifact {
    kind: String,
    path: String,
    blake3: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FailureEvidenceBundle {
    schema: String,
    outcome: String,
    scenario: String,
    backend: String,
    mode: String,
    command_summary: String,
    first_failure: String,
    artifacts: Vec<FailureBundleArtifact>,
    non_claims: Vec<String>,
}

fn validate_failure_evidence_bundle(bundle: &FailureEvidenceBundle) -> Result<(), Vec<String>> {
    let mut diagnostics = Vec::new();
    if bundle.schema != FAILURE_BUNDLE_SCHEMA {
        diagnostics.push(format!(
            "unexpected failure bundle schema {}",
            bundle.schema
        ));
    }
    if !matches!(
        bundle.outcome.as_str(),
        FAILURE_BUNDLE_OUTCOME_FAILED | FAILURE_BUNDLE_OUTCOME_BLOCKED
    ) {
        diagnostics.push(format!(
            "failure bundle outcome must be failed or blocked, found {}",
            bundle.outcome
        ));
    }
    if bundle.scenario.is_empty() {
        diagnostics.push("failure bundle missing scenario".to_string());
    }
    if !matches!(bundle.backend.as_str(), "paper" | "valence") {
        diagnostics.push(format!(
            "failure bundle has unsupported backend {}",
            bundle.backend
        ));
    }
    if bundle.mode.is_empty() {
        diagnostics.push("failure bundle missing mode".to_string());
    }
    if bundle.command_summary.is_empty() {
        diagnostics.push("failure bundle missing command summary".to_string());
    }
    if bundle.first_failure.is_empty() {
        diagnostics.push("failure bundle missing first failure".to_string());
    }
    if bundle.artifacts.is_empty() {
        diagnostics.push("failure bundle missing artifacts".to_string());
    }
    for artifact in &bundle.artifacts {
        validate_failure_bundle_artifact(artifact, &mut diagnostics);
    }
    for required in FAILURE_BUNDLE_REQUIRED_NON_CLAIMS {
        if !bundle.non_claims.iter().any(|claim| claim == required) {
            diagnostics.push(format!("failure bundle missing non_claim {required}"));
        }
    }
    if diagnostics.is_empty() {
        Ok(())
    } else {
        Err(diagnostics)
    }
}

fn validate_failure_bundle_artifact(
    artifact: &FailureBundleArtifact,
    diagnostics: &mut Vec<String>,
) {
    if artifact.kind.is_empty() {
        diagnostics.push("failure bundle artifact missing kind".to_string());
    }
    if let Err(err) = validate_failure_bundle_artifact_path(&artifact.path) {
        diagnostics.push(err);
    }
    if !is_blake3_hex(&artifact.blake3) {
        diagnostics.push(format!(
            "failure bundle artifact {} has malformed BLAKE3 digest",
            artifact.kind
        ));
    }
}

fn validate_failure_bundle_artifact_path(path: &str) -> Result<(), String> {
    if path.is_empty() || path.contains('\0') {
        return Err("failure bundle artifact path is empty or contains NUL".to_string());
    }
    let path_value = Path::new(path);
    if path_value.is_absolute() {
        return Err(format!(
            "failure bundle artifact path must be repo-relative: {path}"
        ));
    }
    for component in path_value.components() {
        match component {
            std::path::Component::ParentDir => {
                return Err(format!("failure bundle artifact path escapes repo: {path}"));
            }
            std::path::Component::Normal(value) if value == FAILURE_BUNDLE_TARGET_COMPONENT => {
                return Err(format!(
                    "failure bundle artifact path is target-only evidence: {path}"
                ));
            }
            _ => {}
        }
    }
    if !path.starts_with(FAILURE_BUNDLE_REVIEW_STORAGE_PREFIX) {
        return Err(format!(
            "failure bundle artifact path must be copied under {FAILURE_BUNDLE_REVIEW_STORAGE_PREFIX}: {path}"
        ));
    }
    Ok(())
}

fn is_blake3_hex(value: &str) -> bool {
    value.len() == FAILURE_BUNDLE_BLAKE3_HEX_CHARS && value.chars().all(|ch| ch.is_ascii_hexdigit())
}

fn failure_bundle_from_config(
    cfg: &Config,
    first_failure: &str,
    artifacts: Vec<FailureBundleArtifact>,
) -> FailureEvidenceBundle {
    FailureEvidenceBundle {
        schema: FAILURE_BUNDLE_SCHEMA.to_string(),
        outcome: FAILURE_BUNDLE_OUTCOME_FAILED.to_string(),
        scenario: scenario_name(cfg.scenario).to_string(),
        backend: backend_name(cfg.server_backend).to_string(),
        mode: mode_name(cfg.mode).to_string(),
        command_summary: bounded_failure_bundle_text(
            &failure_bundle_command_summary(cfg),
            FAILURE_BUNDLE_MAX_COMMAND_SUMMARY_CHARS,
        ),
        first_failure: bounded_failure_bundle_text(
            first_failure,
            FAILURE_BUNDLE_MAX_FIRST_FAILURE_CHARS,
        ),
        artifacts,
        non_claims: FAILURE_BUNDLE_REQUIRED_NON_CLAIMS
            .iter()
            .map(|claim| (*claim).to_string())
            .collect(),
    }
}

fn failure_bundle_command_summary(cfg: &Config) -> String {
    format!(
        "mc-compat-runner --{} --scenario {} --server-backend {}",
        mode_name(cfg.mode),
        scenario_name(cfg.scenario),
        backend_name(cfg.server_backend)
    )
}

fn bounded_failure_bundle_text(value: &str, max_chars: usize) -> String {
    value.chars().take(max_chars).collect()
}

fn render_failure_evidence_bundle_json(bundle: &FailureEvidenceBundle) -> String {
    format!(
        r#"{{
  "schema": {schema},
  "outcome": {outcome},
  "scenario": {scenario},
  "backend": {backend},
  "mode": {mode},
  "command_summary": {command_summary},
  "first_failure": {first_failure},
  "artifacts": {artifacts},
  "non_claims": {non_claims},
  "diagnostic_only": true,
  "claims_scenario_success": false,
  "claims_gameplay_parity": false,
  "claims_full_protocol_compatibility": false,
  "claims_public_server_safety": false,
  "claims_production_readiness": false,
  "claims_semantic_equivalence": false
}}"#,
        schema = json_string(&bundle.schema),
        outcome = json_string(&bundle.outcome),
        scenario = json_string(&bundle.scenario),
        backend = json_string(&bundle.backend),
        mode = json_string(&bundle.mode),
        command_summary = json_string(&bundle.command_summary),
        first_failure = json_string(&bundle.first_failure),
        artifacts = render_failure_bundle_artifacts_json(&bundle.artifacts),
        non_claims = json_string_vec(&bundle.non_claims),
    )
}

fn render_failure_bundle_artifacts_json(artifacts: &[FailureBundleArtifact]) -> String {
    let rendered = artifacts
        .iter()
        .map(render_failure_bundle_artifact_json)
        .collect::<Vec<_>>();
    format!("[{}]", rendered.join(", "))
}

fn render_failure_bundle_artifact_json(artifact: &FailureBundleArtifact) -> String {
    format!(
        r#"{{"kind": {kind}, "path": {path}, "blake3": {blake3}}}"#,
        kind = json_string(&artifact.kind),
        path = json_string(&artifact.path),
        blake3 = json_string(&artifact.blake3),
    )
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

#[derive(Debug, Clone, PartialEq, Eq)]
struct LatencyJitterTelemetryReceipt {
    selected: bool,
    mechanism: String,
    target_rail: String,
    delay_ms: String,
    jitter_ms: String,
    loss_percent: String,
    timeout_secs: u64,
    duration_secs: u64,
    client_count: usize,
    reconnect_count: u32,
    target_ownership: String,
    authorization: String,
    hygiene_status: &'static str,
}

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

fn render_latency_jitter_receipt_json(receipt: &LatencyJitterTelemetryReceipt) -> String {
    format!(
        r#"{{
    "selected": {selected},
    "mechanism": {mechanism},
    "target_rail": {target_rail},
    "delay_ms": {delay_ms},
    "jitter_ms": {jitter_ms},
    "loss_percent": {loss_percent},
    "timeout_secs": {timeout_secs},
    "duration_secs": {duration_secs},
    "client_count": {client_count},
    "reconnect_count": {reconnect_count},
    "target_ownership": {target_ownership},
    "authorization": {authorization},
    "telemetry_samples": {telemetry_samples},
    "pass_fail_criteria": {pass_fail_criteria},
    "abort_reason": {abort_reason},
    "hygiene_status": {hygiene_status},
    "privileged_network_mutation_required": false,
    "fail_closed_when_unavailable": true,
    "claims_wan_safety": false,
    "claims_packet_loss_tolerance": false,
    "claims_internet_path_safety": false,
    "claims_adversarial_network_safety": false,
    "claims_public_server_safety": false,
    "claims_production_readiness": false
  }}"#,
        selected = if receipt.selected { "true" } else { "false" },
        mechanism = json_string(&receipt.mechanism),
        target_rail = json_string(&receipt.target_rail),
        delay_ms = json_string(&receipt.delay_ms),
        jitter_ms = json_string(&receipt.jitter_ms),
        loss_percent = json_string(&receipt.loss_percent),
        timeout_secs = receipt.timeout_secs,
        duration_secs = receipt.duration_secs,
        client_count = receipt.client_count,
        reconnect_count = receipt.reconnect_count,
        target_ownership = json_string(&receipt.target_ownership),
        authorization = json_string(&receipt.authorization),
        telemetry_samples = json_string_array(WAN_TELEMETRY_SAMPLES),
        pass_fail_criteria = json_string(WAN_PASS_FAIL_CRITERIA),
        abort_reason = json_string(WAN_ABORT_REASON_NONE),
        hygiene_status = json_string(receipt.hygiene_status),
    )
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

#[derive(Debug, Clone, PartialEq, Eq)]
struct PublicServerAuthorizedSafetyReceipt {
    selected: bool,
    target_owner: String,
    authorization_artifact: String,
    target_scope: String,
    client_count: usize,
    duration_secs: u64,
    checkpoint_decision: String,
    live_traffic_enabled: bool,
}

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

fn render_public_server_authorized_safety_receipt_json(
    receipt: &PublicServerAuthorizedSafetyReceipt,
) -> String {
    format!(
        r#"{{
    "selected": {selected},
    "target_owner": {target_owner},
    "authorization_artifact": {authorization_artifact},
    "target_scope": {target_scope},
    "client_count": {client_count},
    "duration_secs": {duration_secs},
    "traffic_limits": {traffic_limits},
    "telemetry_fields": {telemetry_fields},
    "abort_criteria": {abort_criteria},
    "redaction_policy": {redaction_policy},
    "checkpoint_decision": {checkpoint_decision},
    "live_traffic_enabled": {live_traffic_enabled},
    "fixture_only": true,
    "claims_authorized_public_envelope": {claims_authorized_public_envelope},
    "claims_live_public_server_safety": false,
    "claims_third_party_target_safety_without_authorization": false,
    "claims_production_readiness": false,
    "claims_adversarial_safety": false,
    "claims_wan_tolerance": false,
    "claims_load_safety_beyond_configured_bounds": false,
    "claims_unbounded_public_testing": false
  }}"#,
        selected = receipt.selected,
        target_owner = json_string(&receipt.target_owner),
        authorization_artifact = json_string(&receipt.authorization_artifact),
        target_scope = json_string(&receipt.target_scope),
        client_count = receipt.client_count,
        duration_secs = receipt.duration_secs,
        traffic_limits = json_string_array(PUBLIC_SERVER_TRAFFIC_LIMITS),
        telemetry_fields = json_string_array(PUBLIC_SERVER_TELEMETRY_FIELDS),
        abort_criteria = json_string(PUBLIC_SERVER_ABORT_CRITERIA),
        redaction_policy = json_string(PUBLIC_SERVER_REDACTION_POLICY),
        checkpoint_decision = json_string(&receipt.checkpoint_decision),
        live_traffic_enabled = receipt.live_traffic_enabled,
        claims_authorized_public_envelope = receipt.selected,
    )
}

fn render_negative_live_rail_json(evidence: &NegativeLiveRailEvidence) -> String {
    format!(
        r#"{{
    "selected": {selected},
    "rail": {rail},
    "invalid_action": {invalid_action},
    "expected_outcome": {expected_outcome},
    "observed_outcome": {observed_outcome},
    "observed_outcome_source": {observed_outcome_source},
    "postcondition_milestone": {postcondition_milestone},
    "telemetry_present": {telemetry_present},
    "target_scope": {target_scope},
    "owned_local_target": {owned_local_target},
    "explicit_authorization": {explicit_authorization},
    "public_target": {public_target},
    "planned_clients": {planned_clients},
    "max_clients": {max_clients},
    "timeout_secs": {timeout_secs},
    "evidence_fields": {evidence_fields},
    "missing_fields": {missing_fields},
    "bound_violations": {bound_violations},
    "preflight_passed": {preflight_passed},
    "non_claims": {non_claims}
  }}"#,
        selected = evidence.selected,
        rail = json_optional_string(evidence.rail),
        invalid_action = json_optional_string(evidence.invalid_action),
        expected_outcome = json_optional_string(evidence.expected_outcome),
        observed_outcome = json_optional_string(evidence.observed_outcome),
        observed_outcome_source = json_optional_string(evidence.observed_outcome_source.as_deref()),
        postcondition_milestone = json_optional_string(evidence.postcondition_milestone),
        telemetry_present = evidence.telemetry_present,
        target_scope = json_string(evidence.target_scope),
        owned_local_target = evidence.owned_local_target,
        explicit_authorization = evidence.explicit_authorization,
        public_target = evidence.public_target,
        planned_clients = evidence.planned_clients,
        max_clients = evidence.max_clients,
        timeout_secs = evidence.timeout_secs,
        evidence_fields = json_string_array(NEGATIVE_LIVE_RAIL_EVIDENCE_FIELDS),
        missing_fields = json_string_array(&evidence.missing_fields),
        bound_violations = json_string_array(&evidence.bound_violations),
        preflight_passed = evidence.preflight_passed,
        non_claims = json_string_array(NEGATIVE_LIVE_RAIL_NON_CLAIMS),
    )
}

fn render_armor_loadout_enchantment_status_matrix_json(
    evidence: &ArmorLoadoutEnchantmentStatusMatrixEvidence,
) -> String {
    format!(
        r#"{{
    "selected": {selected},
    "row_id": {row_id},
    "loadout_id": {loadout_id},
    "equipment_slots": {equipment_slots},
    "enchantments": {enchantments},
    "status_effects": {status_effects},
    "attack_type": {attack_type},
    "reference_required": {reference_required},
    "reference_receipt": {reference_receipt},
    "live_receipt": {live_receipt},
    "promotion_ready": {promotion_ready},
    "required_client_milestones": {required_client_milestones},
    "observed_client_milestones": {observed_client_milestones},
    "required_server_milestones": {required_server_milestones},
    "observed_server_milestones": {observed_server_milestones},
    "non_claims": {non_claims}
  }}"#,
        selected = evidence.selected,
        row_id = json_string(evidence.row_id),
        loadout_id = json_string(evidence.loadout_id),
        equipment_slots = json_string_array(&evidence.equipment_slots),
        enchantments = json_string_array(&evidence.enchantments),
        status_effects = json_string_array(&evidence.status_effects),
        attack_type = json_string(evidence.attack_type),
        reference_required = evidence.reference_required,
        reference_receipt = json_string(evidence.reference_receipt),
        live_receipt = evidence.live_receipt,
        promotion_ready = evidence.promotion_ready,
        required_client_milestones = json_string_array(&evidence.required_client_milestones),
        observed_client_milestones = json_string_array(&evidence.observed_client_milestones),
        required_server_milestones = json_string_array(&evidence.required_server_milestones),
        observed_server_milestones = json_string_array(&evidence.observed_server_milestones),
        non_claims = json_string_array(&evidence.non_claims),
    )
}

fn render_equipment_slot_item_matrix_expansion_json(
    evidence: &EquipmentSlotItemMatrixExpansionEvidence,
) -> String {
    format!(
        r#"{{
    "selected": {selected},
    "row_id": {row_id},
    "actor_username": {actor_username},
    "observer_username": {observer_username},
    "remote_entity_id": {remote_entity_id},
    "semantic_slot": {semantic_slot},
    "wire_slot": {wire_slot},
    "item_id": {item_id},
    "item_count": {item_count},
    "transition_kind": {transition_kind},
    "update_order": {update_order},
    "reference_required": {reference_required},
    "reference_receipt": {reference_receipt},
    "live_receipt": {live_receipt},
    "promotion_ready": {promotion_ready},
    "required_client_milestones": {required_client_milestones},
    "observed_client_milestones": {observed_client_milestones},
    "required_server_milestones": {required_server_milestones},
    "observed_server_milestones": {observed_server_milestones},
    "non_claims": {non_claims}
  }}"#,
        selected = evidence.selected,
        row_id = json_string(evidence.row_id),
        actor_username = json_string(evidence.actor_username),
        observer_username = json_string(evidence.observer_username),
        remote_entity_id = json_string(evidence.remote_entity_id),
        semantic_slot = json_string(evidence.semantic_slot),
        wire_slot = json_string(evidence.wire_slot),
        item_id = json_string(evidence.item_id),
        item_count = json_string(evidence.item_count),
        transition_kind = json_string(evidence.transition_kind),
        update_order = json_string(evidence.update_order),
        reference_required = evidence.reference_required,
        reference_receipt = json_string(evidence.reference_receipt),
        live_receipt = evidence.live_receipt,
        promotion_ready = evidence.promotion_ready,
        required_client_milestones = json_string_array(&evidence.required_client_milestones),
        observed_client_milestones = json_string_array(&evidence.observed_client_milestones),
        required_server_milestones = json_string_array(&evidence.required_server_milestones),
        observed_server_milestones = json_string_array(&evidence.observed_server_milestones),
        non_claims = json_string_array(&evidence.non_claims),
    )
}

fn render_load_network_safety_json(evidence: &LoadNetworkSafetyEvidence) -> String {
    format!(
        r#"{{
    "target_scope": {target_scope},
    "owned_local_target": {owned_local_target},
    "explicit_authorization": {explicit_authorization},
    "public_target": {public_target},
    "authorized": {authorized},
    "planned_clients": {planned_clients},
    "max_clients": {max_clients},
    "duration_secs": {duration_secs},
    "max_duration_secs": {max_duration_secs},
    "reconnect_sessions": {reconnect_sessions},
    "latency_ms": {latency_ms},
    "jitter_ms": {jitter_ms},
    "loss_percent": {loss_percent},
    "telemetry_present": {telemetry_present},
    "live_receipt": {live_receipt},
    "missing_fields": {missing_fields},
    "bound_violations": {bound_violations},
    "preflight_passed": {preflight_passed},
    "promotion_ready": {promotion_ready},
    "claims_public_server_safety": false,
    "claims_production_readiness": false,
    "claims_unbounded_soak": false,
    "claims_unbounded_reconnect": false,
    "claims_wan_safety": false,
    "claims_adversarial_network_safety": false
  }}"#,
        target_scope = json_string(evidence.target_scope),
        owned_local_target = evidence.owned_local_target,
        explicit_authorization = evidence.explicit_authorization,
        public_target = evidence.public_target,
        authorized = evidence.authorized,
        planned_clients = evidence.planned_clients,
        max_clients = evidence.max_clients,
        duration_secs = evidence.duration_secs,
        max_duration_secs = evidence.max_duration_secs,
        reconnect_sessions = evidence.reconnect_sessions,
        latency_ms = json_string(&evidence.latency_ms),
        jitter_ms = json_string(&evidence.jitter_ms),
        loss_percent = json_string(&evidence.loss_percent),
        telemetry_present = evidence.telemetry_present,
        live_receipt = evidence.live_receipt,
        missing_fields = json_string_array(&evidence.missing_fields),
        bound_violations = json_string_array(&evidence.bound_violations),
        preflight_passed = evidence.preflight_passed,
        promotion_ready = evidence.promotion_ready,
    )
}

fn mcp_control_tool_list_digest() -> String {
    blake3::hash(
        MCP_CONTROL_TOOL_NAMES
            .join(MCP_CONTROL_TOOL_LIST_DIGEST_SEPARATOR)
            .as_bytes(),
    )
    .to_hex()
    .to_string()
}

fn evaluate_mcp_control_receipt(
    cfg: &Config,
    child_revision: &GitRevisionEvidence,
    client: Option<&ClientRunEvidence>,
) -> McpControlReceiptEvidence {
    let selected = scenario_behavior(cfg.scenario).is_mcp_controlled_smoke();
    if !selected {
        return McpControlReceiptEvidence {
            selected,
            endpoint_mode: MCP_CONTROL_ENDPOINT_STDIO,
            handshake_success: false,
            tool_list_digest: String::new(),
            tool_names: Vec::new(),
            calls_attempted: Vec::new(),
            calls_succeeded: Vec::new(),
            first_failure: None,
            stdout_clean: false,
            command_outcome_ids: Vec::new(),
            stevenarella_child_revision: None,
            revision_status: child_revision.status,
            dry_run_fixture: false,
            live_receipt: false,
            prerequisites: Vec::new(),
            non_claims: MCP_CONTROL_NON_CLAIMS.to_vec(),
            passed: true,
        };
    }

    let dry_run_fixture = cfg.mode == Mode::DryRun;
    let live_receipt = cfg.mode == Mode::Run;
    let fallback;
    let run_evidence = if dry_run_fixture {
        fallback = mcp_control_dry_run_control_evidence();
        Some(&fallback)
    } else {
        client.and_then(|evidence| evidence.mcp_control.as_ref())
    };
    let revision_failure = mcp_control_revision_failure(child_revision, dry_run_fixture);
    let first_failure = match run_evidence {
        Some(evidence) => evidence.first_failure.or(revision_failure),
        None if live_receipt => Some(MCP_CONTROL_FAILURE_LIVE_EVIDENCE_MISSING),
        None => revision_failure,
    };
    let required_calls_present = run_evidence
        .map(|evidence| {
            MCP_CONTROL_REQUIRED_CALLS
                .iter()
                .all(|call| evidence.calls_succeeded.contains(call))
        })
        .unwrap_or(false);
    let required_outcomes_present = run_evidence
        .map(|evidence| {
            MCP_CONTROL_REQUIRED_OUTCOME_IDS
                .iter()
                .all(|outcome| evidence.command_outcome_ids.contains(outcome))
        })
        .unwrap_or(false);
    let revision_promotable = mcp_control_revision_promotable(child_revision, dry_run_fixture);
    let passed = run_evidence
        .map(|evidence| {
            evidence.handshake_success
                && evidence.stdout_clean
                && evidence.first_failure.is_none()
                && required_calls_present
                && required_outcomes_present
                && revision_promotable
        })
        .unwrap_or(false);
    McpControlReceiptEvidence {
        selected,
        endpoint_mode: MCP_CONTROL_ENDPOINT_STDIO,
        handshake_success: run_evidence
            .map(|evidence| evidence.handshake_success)
            .unwrap_or(false),
        tool_list_digest: run_evidence
            .map(|evidence| evidence.tool_list_digest.clone())
            .unwrap_or_else(mcp_control_tool_list_digest),
        tool_names: run_evidence
            .map(|evidence| evidence.tool_names.clone())
            .unwrap_or_else(|| MCP_CONTROL_TOOL_NAMES.to_vec()),
        calls_attempted: run_evidence
            .map(|evidence| evidence.calls_attempted.clone())
            .unwrap_or_else(|| MCP_CONTROL_LIVE_CALLS.to_vec()),
        calls_succeeded: run_evidence
            .map(|evidence| evidence.calls_succeeded.clone())
            .unwrap_or_default(),
        first_failure,
        stdout_clean: run_evidence
            .map(|evidence| evidence.stdout_clean)
            .unwrap_or(false),
        command_outcome_ids: run_evidence
            .map(|evidence| evidence.command_outcome_ids.clone())
            .unwrap_or_default(),
        stevenarella_child_revision: child_revision.resolved_rev.clone(),
        revision_status: child_revision.status,
        dry_run_fixture,
        live_receipt,
        prerequisites: MCP_CONTROL_PREREQUISITES.to_vec(),
        non_claims: MCP_CONTROL_NON_CLAIMS.to_vec(),
        passed,
    }
}

fn mcp_control_revision_failure(
    child_revision: &GitRevisionEvidence,
    dry_run_fixture: bool,
) -> Option<&'static str> {
    if dry_run_fixture {
        return None;
    }
    match child_revision.status {
        GIT_STATUS_CLEAN => None,
        GIT_STATUS_DIRTY => Some(MCP_CONTROL_FAILURE_REVISION_DIRTY),
        _ => Some(MCP_CONTROL_FAILURE_REVISION_UNAVAILABLE),
    }
}

fn mcp_control_revision_promotable(
    child_revision: &GitRevisionEvidence,
    dry_run_fixture: bool,
) -> bool {
    if dry_run_fixture {
        return child_revision.resolved_rev.is_some();
    }
    child_revision.resolved_rev.is_some() && child_revision.status == GIT_STATUS_CLEAN
}

fn render_mcp_control_receipt_json(evidence: &McpControlReceiptEvidence) -> String {
    let first_failure_json = json_optional_string(evidence.first_failure);
    let child_revision_json = json_optional_string(evidence.stevenarella_child_revision.as_deref());
    format!(
        r#"{{
    "selected": {selected},
    "endpoint_mode": {endpoint_mode_json},
    "handshake_success": {handshake_success},
    "tool_list_digest": {tool_list_digest_json},
    "tool_names": {tool_names_json},
    "calls_attempted": {calls_attempted_json},
    "calls_succeeded": {calls_succeeded_json},
    "first_failure": {first_failure_json},
    "stdout_clean": {stdout_clean},
    "command_outcome_ids": {command_outcome_ids_json},
    "stevenarella_child_revision": {child_revision_json},
    "revision_status": {revision_status_json},
    "dry_run_fixture": {dry_run_fixture},
    "live_receipt": {live_receipt},
    "prerequisites": {prerequisites_json},
    "non_claims": {non_claims_json},
    "passed": {passed}
  }}"#,
        selected = evidence.selected,
        endpoint_mode_json = json_string(evidence.endpoint_mode),
        handshake_success = evidence.handshake_success,
        tool_list_digest_json = json_string(&evidence.tool_list_digest),
        tool_names_json = json_string_array(&evidence.tool_names),
        calls_attempted_json = json_string_array(&evidence.calls_attempted),
        calls_succeeded_json = json_string_array(&evidence.calls_succeeded),
        first_failure_json = first_failure_json,
        stdout_clean = evidence.stdout_clean,
        command_outcome_ids_json = json_string_array(&evidence.command_outcome_ids),
        child_revision_json = child_revision_json,
        revision_status_json = json_string(evidence.revision_status),
        dry_run_fixture = evidence.dry_run_fixture,
        live_receipt = evidence.live_receipt,
        prerequisites_json = json_string_array(&evidence.prerequisites),
        non_claims_json = json_string_array(&evidence.non_claims),
        passed = evidence.passed,
    )
}

fn evaluate_frame_artifacts_receipt(
    cfg: &Config,
    client: Option<&ClientRunEvidence>,
) -> FrameArtifactsReceiptEvidence {
    if let Some(frame_artifacts) = client.and_then(|evidence| evidence.frame_artifacts.as_ref()) {
        return frame_artifacts.clone();
    }
    FrameArtifactsReceiptEvidence {
        selected: false,
        capture_requested: scenario_behavior(cfg.scenario).is_mcp_controlled_smoke(),
        artifact_count: 0,
        artifacts: Vec::new(),
        missing_digests: Vec::new(),
        path_containment_checked: true,
        promotion_ready: false,
        non_claims: FRAME_ARTIFACT_NON_CLAIMS.to_vec(),
    }
}

fn render_frame_artifacts_receipt_json(evidence: &FrameArtifactsReceiptEvidence) -> String {
    format!(
        r#"{{
    "selected": {selected},
    "capture_requested": {capture_requested},
    "artifact_count": {artifact_count},
    "artifacts": {artifacts_json},
    "missing_digests": {missing_digests_json},
    "path_containment_checked": {path_containment_checked},
    "promotion_ready": {promotion_ready},
    "non_claims": {non_claims_json}
  }}"#,
        selected = evidence.selected,
        capture_requested = evidence.capture_requested,
        artifact_count = evidence.artifact_count,
        artifacts_json = frame_artifact_items_json(&evidence.artifacts),
        missing_digests_json = json_string_array(&evidence.missing_digests),
        path_containment_checked = evidence.path_containment_checked,
        promotion_ready = evidence.promotion_ready,
        non_claims_json = json_string_array(&evidence.non_claims),
    )
}

fn frame_artifact_items_json(items: &[FrameArtifactReceiptItem]) -> String {
    let mut out = String::from("[");
    for (index, item) in items.iter().enumerate() {
        if index > 0 {
            out.push_str(", ");
        }
        out.push_str(&format!(
            r#"{{"path": {path}, "relative_path": {relative_path}, "format": {format}, "width_px": {width_px}, "height_px": {height_px}, "frame_id": {frame_id}, "sequence_id": {sequence_id}, "byte_len": {byte_len}, "blake3": {blake3}, "redaction": {redaction}, "includes_ui": {includes_ui}}}"#,
            path = json_string(&item.path),
            relative_path = json_string(&item.relative_path),
            format = json_string(&item.format),
            width_px = item.width_px,
            height_px = item.height_px,
            frame_id = item.frame_id,
            sequence_id = item.sequence_id,
            byte_len = item.byte_len,
            blake3 = json_string(&item.blake3),
            redaction = json_string(&item.redaction),
            includes_ui = item.includes_ui,
        ));
    }
    out.push(']');
    out
}

#[cfg(test)]
fn smoke_receipt_json(cfg: &Config, result: Result<&Option<ClientRunEvidence>, &str>) -> String {
    smoke_receipt_json_with_typed_event_oracle(cfg, result, None)
}

fn smoke_receipt_json_with_typed_event_oracle(
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
    "octet_producer_surface": "tools/mc-compat-runner/src/main.rs",
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

struct EnrichedTriageInput<'a> {
    scenario: &'a ScenarioEvidence,
    server_scenario: &'a ServerScenarioEvidence,
    scenario_name: &'a str,
    usernames: &'a [String],
    error: Option<&'a str>,
    first_missing_client: Option<&'a str>,
    first_missing_server: Option<&'a str>,
    first_forbidden_source: Option<&'a str>,
    first_forbidden_pattern: Option<&'a str>,
    suggested_boundary: &'a str,
}

fn build_enriched_triage(input: EnrichedTriageInput<'_>) -> EnrichedTriage {
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

#[derive(Debug, Clone, PartialEq, Eq)]
struct ReceiptSummary {
    path: PathBuf,
    schema: String,
    status: String,
    dry_run: bool,
    backend: String,
    protocol: u32,
    port: u16,
    classification: String,
    matched_success_pattern: Option<String>,
    xvfb: bool,
    x11_backend: bool,
    software_gl: bool,
    wayland_socket_inherited: bool,
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

fn read_receipt_summary_from_text(path: PathBuf, text: &str) -> Result<ReceiptSummary, String> {
    Ok(ReceiptSummary {
        path,
        schema: json_string_field(text, "schema")?,
        status: json_string_field(text, "status")?,
        dry_run: json_bool_field(text, "dry_run")?,
        backend: json_object_string_field(text, "server", "backend")?,
        protocol: json_object_u32_field(text, "server", "protocol")?,
        port: json_object_u32_field(text, "server", "port")? as u16,
        classification: json_object_string_field(text, "client", "classification")?,
        matched_success_pattern: json_object_optional_string_field(
            text,
            "client",
            "matched_success_pattern",
        )?,
        xvfb: json_object_bool_field(text, "headless_isolation", "xvfb")?,
        x11_backend: json_object_bool_field(text, "headless_isolation", "x11_backend")?,
        software_gl: json_object_bool_field(text, "headless_isolation", "software_gl")?,
        wayland_socket_inherited: json_object_bool_field(
            text,
            "headless_isolation",
            "wayland_socket_inherited",
        )?,
    })
}

fn validate_receipt_pair(
    left: &ReceiptSummary,
    right: &ReceiptSummary,
    expected_protocol: u32,
) -> Result<(), String> {
    validate_receipt_summary(left)?;
    validate_receipt_summary(right)?;
    let backends = [left.backend.as_str(), right.backend.as_str()];
    if !(backends.contains(&"paper") && backends.contains(&"valence")) {
        return Err(format!(
            "expected one paper receipt and one valence receipt, got {} and {}",
            left.backend, right.backend
        ));
    }
    if left.protocol != right.protocol {
        return Err(format!(
            "receipt protocol mismatch: {} has {}, {} has {}",
            left.path.display(),
            left.protocol,
            right.path.display(),
            right.protocol
        ));
    }
    if left.protocol != expected_protocol {
        return Err(format!(
            "expected protocol {}, got {}",
            expected_protocol, left.protocol
        ));
    }
    for receipt in [left, right] {
        match receipt.backend.as_str() {
            "paper" if receipt.port != 25566 => {
                return Err(format!(
                    "paper receipt port must be 25566, got {}",
                    receipt.port
                ));
            }
            "valence" if receipt.port != 25565 => {
                return Err(format!(
                    "valence receipt port must be 25565, got {}",
                    receipt.port
                ));
            }
            _ => {}
        }
    }
    Ok(())
}

fn validate_receipt_summary(receipt: &ReceiptSummary) -> Result<(), String> {
    if !matches!(
        receipt.schema.as_str(),
        "mc.compat.smoke.receipt.v1" | "mc.compat.scenario.receipt.v2"
    ) {
        return Err(format!(
            "{} has unexpected schema {}",
            receipt.path.display(),
            receipt.schema
        ));
    }
    if receipt.status != "pass" {
        return Err(format!(
            "{} did not pass; status={}",
            receipt.path.display(),
            receipt.status
        ));
    }
    let classification_supported = matches!(
        receipt.classification.as_str(),
        "timeout-success-evidence" | "client-exited-success" | "multi-client-load-evidence"
    ) || (receipt.dry_run && receipt.classification == "dry-run");
    if !classification_supported {
        return Err(format!(
            "{} has unsupported client classification {}",
            receipt.path.display(),
            receipt.classification
        ));
    }
    if receipt.matched_success_pattern.is_none() && !receipt.dry_run {
        return Err(format!(
            "{} is missing matched client success pattern",
            receipt.path.display()
        ));
    }
    if !(receipt.xvfb && receipt.x11_backend && receipt.software_gl)
        || receipt.wayland_socket_inherited
    {
        return Err(format!(
            "{} does not prove niri-safe headless isolation",
            receipt.path.display()
        ));
    }
    Ok(())
}

fn json_object_string_field(text: &str, object: &str, key: &str) -> Result<String, String> {
    json_string_field(json_object_slice(text, object)?, key)
}

fn json_object_optional_string_field(
    text: &str,
    object: &str,
    key: &str,
) -> Result<Option<String>, String> {
    json_optional_string_field(json_object_slice(text, object)?, key)
}

fn json_object_u32_field(text: &str, object: &str, key: &str) -> Result<u32, String> {
    json_u32_field(json_object_slice(text, object)?, key)
}

fn json_object_bool_field(text: &str, object: &str, key: &str) -> Result<bool, String> {
    json_bool_field(json_object_slice(text, object)?, key)
}

fn json_object_slice<'a>(text: &'a str, object: &str) -> Result<&'a str, String> {
    let key = format!("\"{object}\"");
    let mut search_start = 0usize;
    while let Some(relative_start) = text[search_start..].find(&key) {
        let start = search_start + relative_start;
        let after_key = &text[start + key.len()..];
        let after_colon = match after_key.trim_start().strip_prefix(':') {
            Some(value) => value,
            None => {
                search_start = start + key.len();
                continue;
            }
        };
        let brace_offset = after_colon
            .find('{')
            .ok_or_else(|| format!("missing object body for {object}"))?;
        let body_start = text.len() - after_colon.len() + brace_offset;
        let mut depth = 0usize;
        for (offset, ch) in text[body_start..].char_indices() {
            match ch {
                '{' => depth += 1,
                '}' => {
                    depth -= 1;
                    if depth == 0 {
                        return Ok(&text[body_start..=body_start + offset]);
                    }
                }
                _ => {}
            }
        }
        return Err(format!("unterminated object {object}"));
    }
    Err(format!("missing object {object}"))
}

fn json_string_field(text: &str, key: &str) -> Result<String, String> {
    let after_colon = json_field_value(text, key)?;
    parse_json_string(after_colon).map(|(value, _)| value)
}

fn json_optional_string_field(text: &str, key: &str) -> Result<Option<String>, String> {
    let Some(after_colon) = json_field_value_opt(text, key)? else {
        return Ok(None);
    };
    if after_colon.trim_start().starts_with("null") {
        Ok(None)
    } else {
        parse_json_string(after_colon).map(|(value, _)| Some(value))
    }
}

fn json_optional_bool_field(text: &str, key: &str) -> Result<Option<bool>, String> {
    let Some(after_colon) = json_field_value_opt(text, key)? else {
        return Ok(None);
    };
    if after_colon.starts_with("true") {
        Ok(Some(true))
    } else if after_colon.starts_with("false") {
        Ok(Some(false))
    } else {
        Err(format!("field {key} must be a boolean"))
    }
}

fn json_optional_u32_field(text: &str, key: &str) -> Result<Option<u32>, String> {
    let Some(value) = json_field_value_opt(text, key)? else {
        return Ok(None);
    };
    parse_json_u32_value(key, value).map(Some)
}

fn json_u32_field(text: &str, key: &str) -> Result<u32, String> {
    parse_json_u32_value(key, json_field_value(text, key)?)
}

fn json_u64_field(text: &str, key: &str) -> Result<u64, String> {
    parse_json_u64_value(key, json_field_value(text, key)?)
}

fn parse_json_u32_value(key: &str, value: &str) -> Result<u32, String> {
    let value = value.trim_start();
    let digits: String = value.chars().take_while(|ch| ch.is_ascii_digit()).collect();
    if digits.is_empty() {
        return Err(format!("field {key} is not an unsigned integer"));
    }
    digits
        .parse()
        .map_err(|e| format!("parse field {key}: {e}"))
}

fn parse_json_u64_value(key: &str, value: &str) -> Result<u64, String> {
    let value = value.trim_start();
    let digits: String = value.chars().take_while(|ch| ch.is_ascii_digit()).collect();
    if digits.is_empty() {
        return Err(format!("field {key} is not an unsigned integer"));
    }
    digits
        .parse()
        .map_err(|e| format!("parse field {key}: {e}"))
}

fn json_optional_string_array_field(text: &str, key: &str) -> Result<Option<Vec<String>>, String> {
    let Some(value) = json_field_value_opt(text, key)? else {
        return Ok(None);
    };
    parse_json_string_array(value).map(Some)
}

fn json_bool_field(text: &str, key: &str) -> Result<bool, String> {
    let value = json_field_value(text, key)?.trim_start();
    if value.starts_with("true") {
        Ok(true)
    } else if value.starts_with("false") {
        Ok(false)
    } else {
        Err(format!("field {key} is not a bool"))
    }
}

fn json_field_value<'a>(text: &'a str, key: &str) -> Result<&'a str, String> {
    json_field_value_opt(text, key)?.ok_or_else(|| format!("missing field {key}"))
}

fn json_field_value_opt<'a>(text: &'a str, key: &str) -> Result<Option<&'a str>, String> {
    let needle = format!("\"{key}\"");
    let Some(start) = text.find(&needle) else {
        return Ok(None);
    };
    let after_key = &text[start + needle.len()..];
    let colon = after_key
        .find(':')
        .ok_or_else(|| format!("missing colon for field {key}"))?;
    Ok(Some(&after_key[colon + 1..]))
}

fn parse_json_string(text: &str) -> Result<(String, &str), String> {
    let text = text.trim_start();
    let mut chars = text.char_indices();
    match chars.next() {
        Some((_, '"')) => {}
        _ => return Err("expected JSON string".to_string()),
    }
    let mut out = String::new();
    let mut escape = false;
    for (idx, ch) in chars {
        if escape {
            match ch {
                '"' => out.push('"'),
                '\\' => out.push('\\'),
                '/' => out.push('/'),
                'n' => out.push('\n'),
                'r' => out.push('\r'),
                't' => out.push('\t'),
                other => out.push(other),
            }
            escape = false;
        } else if ch == '\\' {
            escape = true;
        } else if ch == '"' {
            return Ok((out, &text[idx + 1..]));
        } else {
            out.push(ch);
        }
    }
    Err("unterminated JSON string".to_string())
}

fn parse_json_string_array(text: &str) -> Result<Vec<String>, String> {
    let mut rest = text.trim_start();
    if !rest.starts_with('[') {
        return Err("expected JSON string array".to_string());
    }
    rest = &rest[1..];
    let mut out = Vec::new();
    loop {
        rest = rest.trim_start();
        if let Some(after) = rest.strip_prefix(']') {
            let _ = after;
            return Ok(out);
        }
        let (value, after_string) = parse_json_string(rest)?;
        out.push(value);
        rest = after_string.trim_start();
        if let Some(after) = rest.strip_prefix(',') {
            rest = after;
        } else if rest.starts_with(']') {
            continue;
        } else {
            return Err("expected comma or closing bracket in JSON string array".to_string());
        }
    }
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

fn json_optional_string(value: Option<&str>) -> String {
    value.map(json_string).unwrap_or_else(|| "null".to_string())
}

fn json_string_array(values: &[&str]) -> String {
    json_string_iter(values.iter().copied())
}

fn json_string_vec(values: &[String]) -> String {
    json_string_iter(values.iter().map(String::as_str))
}

fn json_string_iter<'a>(values: impl IntoIterator<Item = &'a str>) -> String {
    let mut out = String::from("[");
    for (idx, value) in values.into_iter().enumerate() {
        if idx > 0 {
            out.push_str(", ");
        }
        out.push_str(&json_string(value));
    }
    out.push(']');
    out
}

fn json_string(value: &str) -> String {
    let mut out = String::with_capacity(value.len() + 2);
    out.push('"');
    for ch in value.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            ch if ch.is_control() => out.push_str(&format!("\\u{:04x}", ch as u32)),
            ch => out.push(ch),
        }
    }
    out.push('"');
    out
}

fn apply_build_env(cmd: &mut Command, target_dir: &Path) {
    cmd.env("RUSTC_WRAPPER", "")
        .env("CARGO_TARGET_DIR", target_dir)
        .env("CMAKE_POLICY_VERSION_MINIMUM", "3.5");
}

fn apply_headless_env(cmd: &mut Command) {
    cmd.env_remove("WAYLAND_DISPLAY")
        .env_remove("WAYLAND_SOCKET")
        .env_remove("XDG_CURRENT_DESKTOP")
        .env("XDG_SESSION_TYPE", "x11")
        .env("WINIT_UNIX_BACKEND", "x11")
        .env("GDK_BACKEND", "x11")
        .env("SDL_VIDEODRIVER", "x11")
        .env("LIBGL_ALWAYS_SOFTWARE", "1")
        .env("MESA_LOADER_DRIVER_OVERRIDE", "llvmpipe");
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

trait McWrite: Write {
    fn write_varint(&mut self, value: u32) -> Result<(), String> {
        let mut remaining = value;
        loop {
            let mut byte = (remaining & VARINT_SEGMENT_MASK) as u8;
            remaining >>= VARINT_SEGMENT_BITS;
            if remaining != 0 {
                byte |= VARINT_CONTINUATION_BIT;
            }
            self.write_all(&[byte]).map_err(|e| e.to_string())?;
            if remaining == 0 {
                return Ok(());
            }
        }
    }

    fn write_mc_string(&mut self, value: &str) -> Result<(), String> {
        let len = u32::try_from(value.len()).map_err(|e| e.to_string())?;
        self.write_varint(len)?;
        self.write_all(value.as_bytes()).map_err(|e| e.to_string())
    }

    fn write_packet(&mut self, id: u32, payload: &[u8]) -> Result<(), String> {
        let mut body = Vec::new();
        body.write_varint(id)?;
        body.extend_from_slice(payload);
        let mut packet = Vec::new();
        packet.write_varint(u32::try_from(body.len()).map_err(|e| e.to_string())?)?;
        packet.extend_from_slice(&body);
        self.write_all(&packet).map_err(|e| e.to_string())
    }
}

impl<T> McWrite for T where T: Write + ?Sized {}

trait McRead: Read {
    fn read_varint(&mut self) -> Result<u32, String> {
        let mut value = 0u32;
        for shift in (0..VARINT_MAX_SHIFT_EXCLUSIVE).step_by(VARINT_SEGMENT_BITS_USIZE) {
            let mut byte = [0u8; 1];
            self.read_exact(&mut byte).map_err(|e| e.to_string())?;
            value |= u32::from(byte[0] & (VARINT_SEGMENT_MASK as u8)) << shift;
            if byte[0] & VARINT_CONTINUATION_BIT == 0 {
                return Ok(value);
            }
        }
        Err("varint too long".to_string())
    }

    fn read_mc_string(&mut self) -> Result<String, String> {
        let string_len = self.read_varint()? as usize;
        let mut buf = vec![0; string_len];
        self.read_exact(&mut buf).map_err(|e| e.to_string())?;
        String::from_utf8(buf).map_err(|e| e.to_string())
    }
}

impl<T> McRead for T where T: Read + ?Sized {}

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
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use std::io::Cursor;

    const TEST_VARINT_SINGLE_BYTE_VALUE: u32 = VARINT_SEGMENT_MASK;
    const TEST_VARINT_TWO_BYTE_VALUE: u32 = VARINT_SEGMENT_MASK + STATUS_HANDSHAKE_NEXT_STATE;
    const TEST_PACKET_PAYLOAD_FIRST_BYTE: u8 = 0x02;
    const TEST_PACKET_PAYLOAD_SECOND_BYTE: u8 = 0x03;
    const TEST_PACKET_BODY_LENGTH: u8 = 0x03;
    const TEST_MC_STRING: &str = "mc";
    const TEST_MC_STRING_LENGTH: u8 = 0x02;
    const TEST_STATUS_PORT: u16 = 25565;
    const TEST_STATUS_PACKET_ID_BYTE_LENGTH: usize = 1;
    const TEST_TOO_LONG_VARINT_BYTES: usize =
        (VARINT_MAX_SHIFT_EXCLUSIVE / VARINT_SEGMENT_BITS) as usize + 1;
    const TEST_GIT_USER_EMAIL: &str = "mc-compat@example.invalid";
    const TEST_GIT_USER_NAME: &str = "mc-compat";
    const TEST_STEVENARELLA_SUBTREE_DIR: &str = "mc/stevenarella";

    fn test_config(args: &[&str], env: &[(&str, &str)]) -> Result<Config, String> {
        let env: BTreeMap<String, String> = env
            .iter()
            .map(|(key, value)| ((*key).to_string(), (*value).to_string()))
            .collect();
        Config::from_sources(
            PathBuf::from("/workspace/mc"),
            |name| env.get(name).cloned(),
            args.iter().map(|arg| (*arg).to_string()),
        )
    }

    fn fake_stevenarella_checkout(label: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "mc-compat-stevenarella-{label}-{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).expect("create fake Stevenarella checkout");
        fs::write(
            dir.join(CARGO_MANIFEST_FILE),
            "[package]\nname = \"stevenarella\"\nversion = \"0.0.0\"\nedition = \"2021\"\n",
        )
        .expect("write fake Stevenarella manifest");
        dir
    }

    fn git_fixture_root(label: &str) -> PathBuf {
        let millis = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time is after Unix epoch")
            .as_millis();
        let dir = std::env::temp_dir().join(format!(
            "mc-compat-git-fixture-{label}-{}-{millis}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).expect("create git fixture root");
        dir
    }

    fn git_available() -> bool {
        Command::new("git")
            .arg("--version")
            .status()
            .map(|status| status.success())
            .unwrap_or(false)
    }

    fn run_git_fixture(repo: &Path, args: &[&str]) -> String {
        let output = Command::new("git")
            .arg("-C")
            .arg(repo)
            .args(args)
            .output()
            .expect("git command starts");
        assert!(
            output.status.success(),
            "git {:?} failed with {}\nstdout={}\nstderr={}",
            args,
            output.status,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8(output.stdout)
            .expect("git stdout is UTF-8")
            .trim()
            .to_string()
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct StructuredReceiptSummary {
        schema: String,
        status: String,
        dry_run: bool,
        contract_claims_correctness: bool,
        contract_claims_semantic_equivalence: bool,
        scenario_name: String,
        backend: String,
        client_classification: Option<String>,
        matched_success_pattern: Option<String>,
        client_git_rev: Option<String>,
        client_git_status: String,
        client_git_dirty: bool,
        valence_git_rev_requested: Option<String>,
        valence_git_rev_resolved: Option<String>,
        valence_git_status: String,
        valence_git_dirty: bool,
        wayland_socket_inherited: bool,
        gameplay_non_claims: Vec<String>,
        typed_event: StructuredTypedEventReceipt,
        mcp_control: StructuredMcpControlReceipt,
        frame_artifacts: StructuredFrameArtifactReceipt,
        armor_matrix: StructuredReferenceMatrixReceipt,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct StructuredTypedEventReceipt {
        selected: bool,
        migration_status: String,
        event_log_path: Option<String>,
        timeline_blake3: Option<String>,
        event_count: u32,
        contributes_to_pass_fail: bool,
        raw_payloads_recorded: bool,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct StructuredMcpControlReceipt {
        selected: bool,
        endpoint_mode: String,
        handshake_success: bool,
        stdout_clean: bool,
        command_outcome_ids: Vec<String>,
        revision_status: String,
        non_claims: Vec<String>,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct StructuredFrameArtifactReceipt {
        selected: bool,
        artifact_count: u32,
        path: Option<String>,
        blake3: Option<String>,
        path_containment_checked: bool,
        promotion_ready: bool,
        non_claims: Vec<String>,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct StructuredReferenceMatrixReceipt {
        selected: bool,
        reference_required: bool,
        reference_receipt: String,
        live_receipt: bool,
        promotion_ready: bool,
        non_claims: Vec<String>,
    }

    const RECEIPT_SCHEMA_V2: &str = "mc.compat.scenario.receipt.v2";
    const RECEIPT_OVERCLAIM_TRUE_PATTERNS: &[&str] = &[
        "\"claims_correctness\": true",
        "\"claims_semantic_equivalence\": true",
        "\"claims_broad_minecraft_compatibility\": true",
        "\"claims_production_readiness\": true",
    ];
    const RECEIPT_REQUIRED_GAMEPLAY_NON_CLAIM: &str = "broad_minecraft_compatibility";
    const RECEIPT_REQUIRED_ARMOR_NON_CLAIM: &str = "all_armor_permutations";
    const RECEIPT_REQUIRED_MCP_NON_CLAIM: &str = "semantic_equivalence";
    const RECEIPT_REQUIRED_FRAME_NON_CLAIM: &str = "semantic_equivalence";
    const RECEIPT_BLAKE3_HEX_CHARS: usize = 64;
    const RECEIPT_PARSE_ERROR_PREVIEW_CHARS: usize = 240;

    fn parse_structured_receipt_summary(text: &str) -> Result<StructuredReceiptSummary, String> {
        ensure_unique_receipt_field(text, "schema", "receipt")?;
        ensure_unique_receipt_field(text, "status", "receipt")?;
        ensure_unique_receipt_field(text, "dry_run", "receipt")?;
        for pattern in RECEIPT_OVERCLAIM_TRUE_PATTERNS {
            if text.contains(pattern) {
                return Err(format!("receipt contains overclaim field {pattern}"));
            }
        }

        let contract = json_object_slice(text, "contract")?;
        ensure_unique_receipt_field(contract, "claims_correctness", "contract")?;
        ensure_unique_receipt_field(contract, "claims_semantic_equivalence", "contract")?;
        let scenario = json_object_slice(text, "scenario")?;
        let server = json_object_slice(text, "server")?;
        let client = json_object_slice(text, "client")?;
        let headless = json_object_slice(client, "headless_isolation")?;
        let valence = json_object_slice(text, "valence")?;
        let gameplay = json_object_slice(text, "gameplay_oracles")?;
        let typed_event = json_object_slice(text, "typed_event_oracle")?;
        let mcp_control = json_object_slice(text, "mcp_control")?;
        let frame_artifacts = json_object_slice(text, "frame_artifacts")?;
        let armor_matrix = json_object_slice(text, "armor_loadout_enchantment_status_matrix")?;

        Ok(StructuredReceiptSummary {
            schema: json_string_field(text, "schema")?,
            status: json_string_field(text, "status")?,
            dry_run: json_bool_field(text, "dry_run")?,
            contract_claims_correctness: json_bool_field(contract, "claims_correctness")?,
            contract_claims_semantic_equivalence: json_bool_field(
                contract,
                "claims_semantic_equivalence",
            )?,
            scenario_name: json_string_field(scenario, "name")?,
            backend: json_string_field(server, "backend")?,
            client_classification: json_optional_string_field(client, "classification")?,
            matched_success_pattern: json_optional_string_field(client, "matched_success_pattern")?,
            client_git_rev: json_optional_string_field(client, "git_rev")?,
            client_git_status: json_string_field(client, "git_status")
                .map_err(|err| receipt_parse_context("client", client, err))?,
            client_git_dirty: json_bool_field(client, "git_dirty")
                .map_err(|err| receipt_parse_context("client", client, err))?,
            valence_git_rev_requested: json_optional_string_field(valence, "git_rev_requested")?,
            valence_git_rev_resolved: json_optional_string_field(valence, "git_rev_resolved")?,
            valence_git_status: json_string_field(valence, "git_status")
                .map_err(|err| receipt_parse_context("valence", valence, err))?,
            valence_git_dirty: json_bool_field(valence, "git_dirty")
                .map_err(|err| receipt_parse_context("valence", valence, err))?,
            wayland_socket_inherited: json_bool_field(headless, "wayland_socket_inherited")?,
            gameplay_non_claims: json_optional_string_array_field(gameplay, "non_claims")?
                .unwrap_or_default(),
            typed_event: parse_structured_typed_event_receipt(typed_event)?,
            mcp_control: parse_structured_mcp_control_receipt(mcp_control)?,
            frame_artifacts: parse_structured_frame_artifact_receipt(frame_artifacts)?,
            armor_matrix: parse_structured_reference_matrix_receipt(armor_matrix)?,
        })
    }

    fn parse_structured_typed_event_receipt(
        text: &str,
    ) -> Result<StructuredTypedEventReceipt, String> {
        ensure_unique_receipt_field(text, "selected", "typed_event_oracle")?;
        ensure_unique_receipt_field(text, "migration_status", "typed_event_oracle")?;
        ensure_unique_receipt_field(text, "event_count", "typed_event_oracle")?;
        Ok(StructuredTypedEventReceipt {
            selected: json_bool_field(text, "selected")?,
            migration_status: json_string_field(text, "migration_status")?,
            event_log_path: json_optional_string_field(text, "event_log_path")?,
            timeline_blake3: json_optional_string_field(text, "timeline_blake3")?,
            event_count: json_u32_field(text, "event_count")?,
            contributes_to_pass_fail: json_bool_field(text, "contributes_to_pass_fail")?,
            raw_payloads_recorded: json_bool_field(text, "raw_payloads_recorded")?,
        })
    }

    fn parse_structured_mcp_control_receipt(
        text: &str,
    ) -> Result<StructuredMcpControlReceipt, String> {
        ensure_unique_receipt_field(text, "selected", "mcp_control")?;
        ensure_unique_receipt_field(text, "endpoint_mode", "mcp_control")?;
        Ok(StructuredMcpControlReceipt {
            selected: json_bool_field(text, "selected")?,
            endpoint_mode: json_string_field(text, "endpoint_mode")?,
            handshake_success: json_bool_field(text, "handshake_success")?,
            stdout_clean: json_bool_field(text, "stdout_clean")?,
            command_outcome_ids: json_optional_string_array_field(text, "command_outcome_ids")?
                .unwrap_or_default(),
            revision_status: json_string_field(text, "revision_status")?,
            non_claims: json_optional_string_array_field(text, "non_claims")?.unwrap_or_default(),
        })
    }

    fn parse_structured_frame_artifact_receipt(
        text: &str,
    ) -> Result<StructuredFrameArtifactReceipt, String> {
        ensure_unique_receipt_field(text, "selected", "frame_artifacts")?;
        ensure_unique_receipt_field(text, "artifact_count", "frame_artifacts")?;
        Ok(StructuredFrameArtifactReceipt {
            selected: json_bool_field(text, "selected")?,
            artifact_count: json_u32_field(text, "artifact_count")?,
            path: json_optional_string_field(text, "path")?,
            blake3: json_optional_string_field(text, "blake3")?,
            path_containment_checked: json_bool_field(text, "path_containment_checked")?,
            promotion_ready: json_bool_field(text, "promotion_ready")?,
            non_claims: json_optional_string_array_field(text, "non_claims")?.unwrap_or_default(),
        })
    }

    fn parse_structured_reference_matrix_receipt(
        text: &str,
    ) -> Result<StructuredReferenceMatrixReceipt, String> {
        ensure_unique_receipt_field(text, "selected", "armor_loadout_enchantment_status_matrix")?;
        ensure_unique_receipt_field(
            text,
            "reference_required",
            "armor_loadout_enchantment_status_matrix",
        )?;
        Ok(StructuredReferenceMatrixReceipt {
            selected: json_bool_field(text, "selected")?,
            reference_required: json_bool_field(text, "reference_required")?,
            reference_receipt: json_string_field(text, "reference_receipt")?,
            live_receipt: json_bool_field(text, "live_receipt")?,
            promotion_ready: json_bool_field(text, "promotion_ready")?,
            non_claims: json_optional_string_array_field(text, "non_claims")?.unwrap_or_default(),
        })
    }

    fn receipt_parse_context(scope: &str, text: &str, err: String) -> String {
        let preview: String = text
            .chars()
            .take(RECEIPT_PARSE_ERROR_PREVIEW_CHARS)
            .collect();
        format!("{err}; {scope} preview={preview:?}")
    }

    fn validate_structured_receipt_summary(
        receipt: &StructuredReceiptSummary,
    ) -> Result<(), String> {
        if receipt.schema != RECEIPT_SCHEMA_V2 {
            return Err(format!("unexpected receipt schema {}", receipt.schema));
        }
        if receipt.contract_claims_correctness || receipt.contract_claims_semantic_equivalence {
            return Err("receipt contract overclaims compatibility".to_string());
        }
        if !matches!(receipt.backend.as_str(), "paper" | "valence") {
            return Err(format!("unsupported receipt backend {}", receipt.backend));
        }
        if receipt.wayland_socket_inherited {
            return Err("receipt does not prove headless Wayland isolation".to_string());
        }
        if receipt.status == "pass" && !receipt.dry_run && receipt.client_classification.is_none() {
            return Err("passing live receipt missing client classification".to_string());
        }
        if receipt.status == "pass" && !receipt.dry_run && receipt.matched_success_pattern.is_none()
        {
            return Err("passing live receipt missing matched success pattern".to_string());
        }
        validate_structured_child_revision(
            "client",
            receipt.dry_run,
            &receipt.client_git_status,
            receipt.client_git_dirty,
        )?;
        validate_structured_child_revision(
            "valence",
            receipt.dry_run,
            &receipt.valence_git_status,
            receipt.valence_git_dirty,
        )?;
        if !receipt
            .gameplay_non_claims
            .iter()
            .any(|claim| claim == RECEIPT_REQUIRED_GAMEPLAY_NON_CLAIM)
        {
            return Err(format!(
                "receipt missing non_claim {RECEIPT_REQUIRED_GAMEPLAY_NON_CLAIM}"
            ));
        }
        validate_structured_typed_event_receipt(&receipt.typed_event)?;
        validate_structured_mcp_control_receipt(&receipt.mcp_control)?;
        validate_structured_frame_artifact_receipt(&receipt.frame_artifacts)?;
        validate_structured_reference_matrix_receipt(&receipt.armor_matrix)?;
        Ok(())
    }

    fn validate_structured_child_revision(
        label: &str,
        dry_run: bool,
        status: &str,
        dirty: bool,
    ) -> Result<(), String> {
        if dry_run {
            if status != GIT_STATUS_DRY_RUN || dirty {
                return Err(format!(
                    "{label} dry-run child revision is not deterministic"
                ));
            }
            return Ok(());
        }
        if status != GIT_STATUS_CLEAN || dirty {
            return Err(format!("{label} child revision is not clean"));
        }
        Ok(())
    }

    fn validate_structured_typed_event_receipt(
        typed_event: &StructuredTypedEventReceipt,
    ) -> Result<(), String> {
        if typed_event.raw_payloads_recorded {
            return Err("typed event oracle records raw payloads".to_string());
        }
        if typed_event.selected {
            if typed_event.migration_status != TYPED_EVENT_MIGRATION_DERIVED_FROM_MILESTONES {
                return Err(
                    "typed event oracle selected without derived migration status".to_string(),
                );
            }
            let path = typed_event
                .event_log_path
                .as_deref()
                .ok_or_else(|| "typed event oracle missing event_log_path".to_string())?;
            validate_structured_artifact_path("typed event oracle", path)?;
            let digest = typed_event
                .timeline_blake3
                .as_deref()
                .ok_or_else(|| "typed event oracle missing timeline_blake3".to_string())?;
            validate_blake3_hex("typed event oracle", digest)?;
            if typed_event.event_count == 0 {
                return Err("typed event oracle selected with zero events".to_string());
            }
        } else {
            if typed_event.migration_status != TYPED_EVENT_MIGRATION_FALLBACK {
                return Err("typed event oracle fallback has wrong migration status".to_string());
            }
            if typed_event.event_log_path.is_some() || typed_event.timeline_blake3.is_some() {
                return Err("typed event fallback records artifact paths".to_string());
            }
            if typed_event.event_count != 0 || typed_event.contributes_to_pass_fail {
                return Err("typed event fallback records pass/fail event evidence".to_string());
            }
        }
        Ok(())
    }

    fn validate_structured_mcp_control_receipt(
        mcp: &StructuredMcpControlReceipt,
    ) -> Result<(), String> {
        if !mcp.selected {
            return Ok(());
        }
        if mcp.endpoint_mode != "stdio" {
            return Err(format!(
                "mcp_control wrong endpoint mode {}",
                mcp.endpoint_mode
            ));
        }
        if !mcp.handshake_success || !mcp.stdout_clean {
            return Err("mcp_control missing handshake/stdout proof".to_string());
        }
        if !matches!(
            mcp.revision_status.as_str(),
            GIT_STATUS_CLEAN | GIT_STATUS_DRY_RUN
        ) {
            return Err(format!(
                "mcp_control has unacceptable child revision status {}",
                mcp.revision_status
            ));
        }
        if !mcp
            .command_outcome_ids
            .iter()
            .any(|outcome| outcome == "status.applied")
        {
            return Err("mcp_control missing status.applied outcome".to_string());
        }
        if !mcp
            .non_claims
            .iter()
            .any(|claim| claim == RECEIPT_REQUIRED_MCP_NON_CLAIM)
        {
            return Err(format!(
                "mcp_control missing non_claim {RECEIPT_REQUIRED_MCP_NON_CLAIM}"
            ));
        }
        Ok(())
    }

    fn validate_structured_frame_artifact_receipt(
        frame: &StructuredFrameArtifactReceipt,
    ) -> Result<(), String> {
        if frame.selected {
            if !frame.path_containment_checked {
                return Err("frame artifacts missing path containment check".to_string());
            }
            let path = frame
                .path
                .as_deref()
                .ok_or_else(|| "frame artifacts missing path".to_string())?;
            validate_structured_artifact_path("frame artifacts", path)?;
            let digest = frame
                .blake3
                .as_deref()
                .ok_or_else(|| "frame artifacts missing blake3".to_string())?;
            validate_blake3_hex("frame artifacts", digest)?;
            if frame.artifact_count == 0 {
                return Err("frame artifacts selected with zero artifacts".to_string());
            }
        }
        if !frame
            .non_claims
            .iter()
            .any(|claim| claim == RECEIPT_REQUIRED_FRAME_NON_CLAIM)
        {
            return Err(format!(
                "frame artifacts missing non_claim {RECEIPT_REQUIRED_FRAME_NON_CLAIM}"
            ));
        }
        Ok(())
    }

    fn validate_structured_reference_matrix_receipt(
        matrix: &StructuredReferenceMatrixReceipt,
    ) -> Result<(), String> {
        if matrix.selected {
            if matrix.reference_required
                && matrix.reference_receipt == ARMOR_MATRIX_REFERENCE_RECEIPT_NONE
            {
                return Err("selected armor matrix requires missing reference receipt".to_string());
            }
            if matrix.promotion_ready && !matrix.live_receipt {
                return Err("selected armor matrix promotes without live receipt".to_string());
            }
        }
        if !matrix
            .non_claims
            .iter()
            .any(|claim| claim == RECEIPT_REQUIRED_ARMOR_NON_CLAIM)
        {
            return Err(format!(
                "armor matrix missing non_claim {RECEIPT_REQUIRED_ARMOR_NON_CLAIM}"
            ));
        }
        Ok(())
    }

    fn validate_structured_artifact_path(label: &str, value: &str) -> Result<(), String> {
        if value.is_empty() || value.contains('\0') {
            return Err(format!("{label} artifact path is empty or contains NUL"));
        }
        if Path::new(value)
            .components()
            .any(|component| matches!(component, std::path::Component::ParentDir))
        {
            return Err(format!("{label} artifact path escapes evidence root"));
        }
        Ok(())
    }

    fn validate_blake3_hex(label: &str, value: &str) -> Result<(), String> {
        if value.len() != RECEIPT_BLAKE3_HEX_CHARS
            || !value.chars().all(|ch| ch.is_ascii_hexdigit())
        {
            return Err(format!("{label} has malformed BLAKE3 digest"));
        }
        Ok(())
    }

    fn ensure_unique_receipt_field(text: &str, key: &str, scope: &str) -> Result<(), String> {
        let expected_count = 1;
        let count = receipt_key_occurrence_count(text, key);
        if count == expected_count {
            return Ok(());
        }
        Err(format!("{scope} field {key} expected once, found {count}"))
    }

    fn receipt_key_occurrence_count(text: &str, key: &str) -> usize {
        let needle = format!("\"{key}\"");
        text.match_indices(&needle).count()
    }

    fn structured_receipt_from_text(text: &str) -> Result<StructuredReceiptSummary, String> {
        let summary = parse_structured_receipt_summary(text)?;
        validate_structured_receipt_summary(&summary)?;
        Ok(summary)
    }

    const TEST_FAILURE_BUNDLE_ARTIFACT_PATH: &str = "docs/evidence/failure-receipt.json";
    const TEST_FAILURE_BUNDLE_KIND: &str = "receipt";
    const TEST_FAILURE_BUNDLE_FIRST_FAILURE: &str = "scenario missing required milestone";
    const TEST_FAILURE_BUNDLE_PATH_ESCAPE: &str = "docs/evidence/../secret.log";
    const TEST_FAILURE_BUNDLE_TARGET_PATH: &str = "target/failure-receipt.json";
    const TEST_FAILURE_BUNDLE_MALFORMED_DIGEST: &str = "not-a-blake3-digest";
    const TEST_FAILURE_BUNDLE_SUCCESS_OUTCOME: &str = "pass";

    fn failure_bundle_digest_fixture() -> String {
        blake3::hash(b"failure bundle artifact")
            .to_hex()
            .to_string()
    }

    fn failure_bundle_artifact_fixture() -> FailureBundleArtifact {
        FailureBundleArtifact {
            kind: TEST_FAILURE_BUNDLE_KIND.to_string(),
            path: TEST_FAILURE_BUNDLE_ARTIFACT_PATH.to_string(),
            blake3: failure_bundle_digest_fixture(),
        }
    }

    fn failure_bundle_fixture() -> FailureEvidenceBundle {
        FailureEvidenceBundle {
            schema: FAILURE_BUNDLE_SCHEMA.to_string(),
            outcome: FAILURE_BUNDLE_OUTCOME_FAILED.to_string(),
            scenario: "smoke".to_string(),
            backend: "valence".to_string(),
            mode: "run".to_string(),
            command_summary: "mc-compat-runner --run --scenario smoke --server-backend valence"
                .to_string(),
            first_failure: TEST_FAILURE_BUNDLE_FIRST_FAILURE.to_string(),
            artifacts: vec![failure_bundle_artifact_fixture()],
            non_claims: FAILURE_BUNDLE_REQUIRED_NON_CLAIMS
                .iter()
                .map(|claim| (*claim).to_string())
                .collect(),
        }
    }

    fn failure_bundle_diagnostics(bundle: &FailureEvidenceBundle) -> String {
        validate_failure_evidence_bundle(bundle)
            .expect_err("failure bundle fixture should fail")
            .join("; ")
    }

    #[test]
    fn failure_bundle_validator_accepts_complete_fail_only_bundle() {
        let bundle = failure_bundle_fixture();
        validate_failure_evidence_bundle(&bundle).expect("valid failure bundle passes");
        let json = render_failure_evidence_bundle_json(&bundle);

        assert!(json.contains(FAILURE_BUNDLE_SCHEMA));
        assert!(json.contains("\"diagnostic_only\": true"));
        assert!(json.contains("\"claims_scenario_success\": false"));
        assert!(json.contains(TEST_FAILURE_BUNDLE_ARTIFACT_PATH));
        assert!(is_blake3_hex(&bundle.artifacts[0].blake3));
    }

    #[test]
    fn failure_bundle_validator_rejects_negative_fixtures() {
        let mut missing_artifacts = failure_bundle_fixture();
        missing_artifacts.artifacts.clear();
        assert!(failure_bundle_diagnostics(&missing_artifacts).contains("missing artifacts"));

        let mut path_escape = failure_bundle_fixture();
        path_escape.artifacts[0].path = TEST_FAILURE_BUNDLE_PATH_ESCAPE.to_string();
        assert!(failure_bundle_diagnostics(&path_escape).contains("escapes repo"));

        let mut target_only = failure_bundle_fixture();
        target_only.artifacts[0].path = TEST_FAILURE_BUNDLE_TARGET_PATH.to_string();
        assert!(failure_bundle_diagnostics(&target_only).contains("target-only"));

        let mut malformed_digest = failure_bundle_fixture();
        malformed_digest.artifacts[0].blake3 = TEST_FAILURE_BUNDLE_MALFORMED_DIGEST.to_string();
        assert!(failure_bundle_diagnostics(&malformed_digest).contains("malformed BLAKE3"));

        let mut missing_nonclaim = failure_bundle_fixture();
        missing_nonclaim
            .non_claims
            .retain(|claim| claim != FAILURE_BUNDLE_NON_CLAIM_SEMANTIC_EQUIVALENCE);
        assert!(failure_bundle_diagnostics(&missing_nonclaim).contains("missing non_claim"));

        let mut success_labeled = failure_bundle_fixture();
        success_labeled.outcome = TEST_FAILURE_BUNDLE_SUCCESS_OUTCOME.to_string();
        assert!(failure_bundle_diagnostics(&success_labeled).contains("failed or blocked"));
    }

    #[test]
    fn failure_bundle_shell_writes_reviewable_bundle_for_failed_result() {
        let temp_root =
            std::env::temp_dir().join(format!("mc-compat-failure-bundle-{}", std::process::id()));
        let _ = fs::remove_dir_all(&temp_root);
        let evidence_dir = temp_root.join("docs/evidence");
        fs::create_dir_all(&evidence_dir).expect("create evidence dir");
        let receipt_path = evidence_dir.join("failed-receipt.json");
        let bundle_path = evidence_dir.join("failed-bundle.json");
        fs::write(&receipt_path, "receipt bytes").expect("write receipt artifact");

        let mut cfg =
            test_config(&["--run", "--scenario=smoke"], &[]).expect("failure bundle config parses");
        cfg.root = temp_root.clone();
        cfg.receipt_path = Some(receipt_path);
        cfg.failure_bundle_path = Some(bundle_path.clone());
        cfg.valence_log = evidence_dir.join("valence.log");
        let first_failure = TEST_FAILURE_BUNDLE_FIRST_FAILURE.to_string();
        let result: Result<&Option<ClientRunEvidence>, &String> = Err(&first_failure);

        write_failure_evidence_bundle(&cfg, result).expect("failure bundle writes");
        let json = fs::read_to_string(&bundle_path).expect("read failure bundle");

        assert!(json.contains(FAILURE_BUNDLE_SCHEMA));
        assert!(json.contains(TEST_FAILURE_BUNDLE_FIRST_FAILURE));
        assert!(json.contains("docs/evidence/failed-receipt.json"));
        assert!(json.contains(&blake3::hash(b"receipt bytes").to_hex().to_string()));

        fs::remove_dir_all(&temp_root).expect("remove temp failure bundle root");
    }

    #[test]
    fn runner_result_preserves_original_failure_when_follow_up_fails() {
        let err = combine_runner_result(
            Err("original failure".to_string()),
            vec!["failed to write failure bundle: validation failed".to_string()],
        )
        .expect_err("combined failure remains failing");

        assert!(err.contains("original failure"));
        assert!(err.contains("failed to write failure bundle"));
    }

    fn assert_plan_is_deterministic(cfg: &Config) -> HarnessPlan {
        let first = harness_plan_from_config(cfg).expect("first plan succeeds");
        let second = harness_plan_from_config(cfg).expect("second plan succeeds");
        assert_eq!(first, second);
        first
    }

    fn plan_diagnostic_text(result: Result<HarnessPlan, Vec<PlanningDiagnostic>>) -> String {
        format_plan_diagnostics(result.expect_err("plan should fail"))
    }

    #[test]
    fn planning_core_positive_fixtures_cover_representative_modes() {
        let dry_paper = test_config(
            &[
                "--dry-run",
                "--server-backend=paper",
                "--receipt=docs/evidence/smoke.json",
            ],
            &[],
        )
        .expect("dry paper config parses");
        let dry_plan = assert_plan_is_deterministic(&dry_paper);
        assert_eq!(dry_plan.server.backend, "paper");
        assert!(dry_plan.server.eula_acceptance_required);
        assert_eq!(dry_plan.client_sessions.len(), 1);
        assert_eq!(
            dry_plan.receipt.receipt_path.as_deref(),
            Some("docs/evidence/smoke.json")
        );

        let live_valence = test_config(
            &[
                "--run",
                "--server-backend=valence",
                "--scenario=inventory-interaction",
                "--receipt=docs/evidence/live.json",
            ],
            &[],
        )
        .expect("live valence config parses");
        let live_plan = assert_plan_is_deterministic(&live_valence);
        assert_eq!(live_plan.server.backend, "valence");
        assert_eq!(
            live_plan.client_sessions[0].scenario,
            "inventory-interaction"
        );
        assert!(!live_plan.server.keep_server);

        let matrix = test_config(
            &[
                "--run-matrix",
                "--dry-run",
                "--receipt-dir=target/matrix-plan",
            ],
            &[],
        )
        .expect("matrix config parses");
        let matrix_plan = assert_plan_is_deterministic(&matrix)
            .matrix
            .expect("matrix plan present");
        assert!(matrix_plan.dry_run);
        assert!(matrix_plan.paper_receipt.ends_with("paper.json"));
        assert!(matrix_plan.valence_receipt.ends_with("valence.json"));

        let reconnect = test_config(&["--dry-run", "--scenario=reconnect-flag-state"], &[])
            .expect("reconnect config parses");
        let reconnect_plan = assert_plan_is_deterministic(&reconnect);
        assert_eq!(
            reconnect_plan.client_sessions[0].session_count,
            RECONNECT_SEQUENCE_SESSION_COUNT
        );
        assert_eq!(
            reconnect_plan.client_sessions[0].log_path_strategy,
            PLAN_CLIENT_LOG_RECONNECT_TEMP
        );

        let multi_client = test_config(&["--dry-run", "--scenario=multi-client-load-score"], &[])
            .expect("multi-client config parses");
        let multi_plan = assert_plan_is_deterministic(&multi_client);
        assert_eq!(multi_plan.client_sessions.len(), MULTI_CLIENT_READY_COUNT);
        assert_eq!(
            multi_plan.client_sessions[SECOND_CLIENT_INDEX].username,
            "compatbotb"
        );

        let cleanup = test_config(&["--cleanup", "--dry-run"], &[]).expect("cleanup config parses");
        let cleanup_plan = assert_plan_is_deterministic(&cleanup).cleanup;
        assert!(!cleanup_plan.apply);
        assert!(cleanup_plan
            .path_actions
            .iter()
            .any(|action| action.label == "valence target dir"));
        assert!(cleanup_plan
            .path_actions
            .iter()
            .any(|action| action.label == "valence log"));
        assert_eq!(
            cleanup_plan.client_log_discovery,
            PLAN_CLEANUP_CLIENT_LOG_DISCOVERY
        );

        let failure_bundle = test_config(
            &[
                "--run",
                "--receipt=docs/evidence/failed-receipt.json",
                "--failure-bundle=docs/evidence/failed-bundle.json",
            ],
            &[("VALENCE_LOG", "docs/evidence/failed-valence.log")],
        )
        .expect("failure bundle config parses");
        let failure_plan = assert_plan_is_deterministic(&failure_bundle);
        assert_eq!(
            failure_plan.receipt.failure_bundle_path.as_deref(),
            Some("docs/evidence/failed-bundle.json")
        );
        assert!(failure_plan
            .artifacts
            .failure_artifact_candidates
            .iter()
            .any(|artifact| artifact.kind == FAILURE_BUNDLE_ARTIFACT_RECEIPT));
    }

    #[test]
    fn planning_core_negative_fixtures_fail_before_side_effects() {
        let missing_receipt = test_config(
            &["--run", "--failure-bundle=docs/evidence/failed-bundle.json"],
            &[],
        )
        .expect("missing receipt config parses");
        let missing_receipt_err = plan_diagnostic_text(harness_plan_from_config(&missing_receipt));
        assert!(missing_receipt_err.contains("requires a receipt path"));

        let path_escape = test_config(
            &[
                "--run",
                "--receipt=docs/evidence/failed-receipt.json",
                "--failure-bundle=../failed-bundle.json",
            ],
            &[],
        )
        .expect("path escape config parses");
        let path_escape_err = plan_diagnostic_text(harness_plan_from_config(&path_escape));
        assert!(path_escape_err.contains("escapes repo"));

        let target_artifact = test_config(
            &[
                "--run",
                "--receipt=target/failed-receipt.json",
                "--failure-bundle=docs/evidence/failed-bundle.json",
            ],
            &[],
        )
        .expect("target artifact config parses");
        let target_artifact_err = plan_diagnostic_text(harness_plan_from_config(&target_artifact));
        assert!(target_artifact_err.contains("target-only"));

        let mut matrix_conflict =
            test_config(&["--run-matrix"], &[]).expect("matrix conflict base config parses");
        matrix_conflict.receipt_path = Some(PathBuf::from("docs/evidence/one.json"));
        let matrix_conflict_err = plan_diagnostic_text(harness_plan_from_config(&matrix_conflict));
        assert!(matrix_conflict_err.contains("run-matrix planning"));

        let mut cleanup_hazard =
            test_config(&["--cleanup", "--apply"], &[]).expect("cleanup hazard config parses");
        cleanup_hazard.valence_target_dir = PathBuf::from(CLEANUP_ROOT_PATH);
        let cleanup_hazard_err = plan_diagnostic_text(harness_plan_from_config(&cleanup_hazard));
        assert!(cleanup_hazard_err.contains("too broad for cleanup"));
    }

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
    fn git_revision_evidence_core_reports_clean_dirty_and_unavailable() {
        let clean_evidence =
            build_git_revision_evidence(Some("HEAD"), Ok("abc123".to_string()), Ok(false));
        assert_eq!(clean_evidence.status, GIT_STATUS_CLEAN);
        assert!(!clean_evidence.dirty);
        assert_eq!(clean_evidence.requested_rev.as_deref(), Some("HEAD"));
        assert_eq!(clean_evidence.resolved_rev.as_deref(), Some("abc123"));
        assert!(clean_evidence.diagnostics.is_empty(), "{clean_evidence:?}");

        let dirty_evidence =
            build_git_revision_evidence(Some("HEAD"), Ok("abc123".to_string()), Ok(true));
        assert_eq!(dirty_evidence.status, GIT_STATUS_DIRTY);
        assert!(dirty_evidence.dirty);
        assert_eq!(dirty_evidence.resolved_rev.as_deref(), Some("abc123"));

        let unavailable_evidence = build_git_revision_evidence(
            None,
            Err("missing rev".to_string()),
            Err("missing status".to_string()),
        );
        assert_eq!(unavailable_evidence.status, GIT_STATUS_UNAVAILABLE);
        assert!(unavailable_evidence.dirty);
        assert!(unavailable_evidence.resolved_rev.is_none());
        let expected_diagnostic_count = 2;
        assert_eq!(
            unavailable_evidence.diagnostics.len(),
            expected_diagnostic_count
        );
    }

    #[test]
    fn git_revision_evidence_scopes_to_vendored_source_directory() {
        if !git_available() {
            return;
        }
        let root = git_fixture_root("scoped-revision");
        run_git_fixture(&root, &["init"]);
        run_git_fixture(&root, &["config", "user.email", TEST_GIT_USER_EMAIL]);
        run_git_fixture(&root, &["config", "user.name", TEST_GIT_USER_NAME]);
        let source_dir = root.join(TEST_STEVENARELLA_SUBTREE_DIR);
        fs::create_dir_all(&source_dir).expect("create vendored source dir");
        fs::write(
            source_dir.join(CARGO_MANIFEST_FILE),
            "[package]\nname = \"stevenarella\"\nversion = \"0.0.0\"\nedition = \"2021\"\n",
        )
        .expect("write vendored manifest");
        run_git_fixture(&root, &["add", TEST_STEVENARELLA_SUBTREE_DIR]);
        run_git_fixture(&root, &["commit", "-m", "add vendored client"]);
        let source_commit = git_rev_parse(&root, GIT_HEAD_REV).expect("source commit resolves");

        fs::write(root.join("README.md"), "parent docs\n").expect("write parent docs");
        run_git_fixture(&root, &["add", "README.md"]);
        run_git_fixture(&root, &["commit", "-m", "update parent docs"]);
        let parent_commit = git_rev_parse(&root, GIT_HEAD_REV).expect("parent commit resolves");
        assert_ne!(source_commit, parent_commit);

        fs::write(root.join("UNTRACKED_PARENT.txt"), "outside subtree\n")
            .expect("write unrelated parent dirt");
        let clean_evidence = git_revision_evidence(&source_dir, None);
        assert_eq!(clean_evidence.status, GIT_STATUS_CLEAN);
        assert!(!clean_evidence.dirty);
        assert_eq!(
            clean_evidence.resolved_rev.as_deref(),
            Some(source_commit.as_str())
        );

        fs::write(source_dir.join("UNTRACKED_SOURCE.txt"), "inside subtree\n")
            .expect("write source dirt");
        let dirty_evidence = git_revision_evidence(&source_dir, None);
        assert_eq!(dirty_evidence.status, GIT_STATUS_DIRTY);
        assert!(dirty_evidence.dirty);
        assert_eq!(
            dirty_evidence.resolved_rev.as_deref(),
            Some(source_commit.as_str())
        );

        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn valence_source_dir_detects_monorepo_and_legacy_worktree_shapes() {
        let root = git_fixture_root("valence-source-dir");
        let legacy = root.join("legacy-valence");
        fs::create_dir_all(&legacy).expect("create legacy Valence worktree");
        fs::write(
            legacy.join(CARGO_MANIFEST_FILE),
            "[package]\nname = \"valence\"\n",
        )
        .expect("write legacy manifest");
        let mut legacy_cfg = test_config(&[], &[]).expect("default config parses");
        legacy_cfg.valence_worktree = legacy.clone();
        assert_eq!(valence_source_dir(&legacy_cfg), legacy);

        let monorepo = root.join("monorepo-worktree");
        let vendored = monorepo.join(VALENCE_MONOREPO_SUBTREE_DIR);
        fs::create_dir_all(&vendored).expect("create monorepo Valence subtree");
        fs::write(
            vendored.join(CARGO_MANIFEST_FILE),
            "[package]\nname = \"valence\"\n",
        )
        .expect("write vendored manifest");
        let mut monorepo_cfg = test_config(&[], &[]).expect("default config parses");
        monorepo_cfg.valence_worktree = monorepo;
        assert_eq!(valence_source_dir(&monorepo_cfg), vendored);

        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn dry_run_receipt_records_deterministic_child_revision_placeholders() {
        let cfg = test_config(&["--scenario=survival-break-place-pickup"], &[])
            .expect("dry-run config parses");
        let json = smoke_receipt_json(&cfg, Ok(&None));

        let receipt = structured_receipt_from_text(&json).expect("dry-run receipt validates");
        assert_eq!(
            receipt.client_git_rev.as_deref(),
            Some(GIT_REV_DRY_RUN_PLACEHOLDER)
        );
        assert_eq!(receipt.client_git_status, GIT_STATUS_DRY_RUN);
        assert!(!receipt.client_git_dirty);
        assert_eq!(
            receipt.valence_git_rev_requested.as_deref(),
            Some(DEFAULT_VALENCE_REV)
        );
        assert_eq!(
            receipt.valence_git_rev_resolved.as_deref(),
            Some(GIT_REV_DRY_RUN_PLACEHOLDER)
        );
        assert_eq!(receipt.valence_git_status, GIT_STATUS_DRY_RUN);
        assert!(!receipt.valence_git_dirty);
    }

    #[test]
    fn mcp_controlled_dry_run_receipt_records_control_contract() {
        let cfg = test_config(&["--scenario", MCP_CONTROLLED_SMOKE_SCENARIO], &[])
            .expect("mcp-controlled config parses");
        let evidence = mcp_controlled_dry_run_evidence(&cfg);
        let json = smoke_receipt_json(&cfg, Ok(&Some(evidence)));

        let receipt = structured_receipt_from_text(&json).expect("mcp dry-run receipt validates");
        assert_eq!(receipt.scenario_name, MCP_CONTROLLED_SMOKE_SCENARIO);
        assert!(receipt.mcp_control.selected);
        assert_eq!(receipt.mcp_control.endpoint_mode, "stdio");
        assert!(receipt.mcp_control.handshake_success);
        assert!(receipt.mcp_control.stdout_clean);
        assert!(receipt
            .mcp_control
            .command_outcome_ids
            .iter()
            .any(|outcome| outcome == "status.applied"));
        assert_eq!(receipt.mcp_control.revision_status, GIT_STATUS_DRY_RUN);
        assert!(!receipt.frame_artifacts.promotion_ready);
        assert!(receipt
            .mcp_control
            .non_claims
            .iter()
            .any(|claim| claim == RECEIPT_REQUIRED_MCP_NON_CLAIM));
    }

    #[test]
    fn mcp_controlled_live_preflight_allows_bounded_local_rail() {
        let cfg = test_config(&["--run", "--scenario", MCP_CONTROLLED_SMOKE_SCENARIO], &[])
            .expect("mcp-controlled live config parses");
        validate_mcp_controlled_live_preflight(&cfg)
            .expect("bounded local MCP-controlled live rail preflight passes");
    }

    #[test]
    fn mcp_controlled_live_preflight_rejects_unbounded_timeout() {
        let mut cfg = test_config(&["--run", "--scenario", MCP_CONTROLLED_SMOKE_SCENARIO], &[])
            .expect("mcp-controlled live config parses");
        cfg.client_timeout = Duration::from_secs(SAFETY_MAX_DURATION_SECS + 1);

        let err = validate_mcp_controlled_live_preflight(&cfg)
            .expect_err("unbounded MCP-controlled live rail fails preflight");

        assert!(err.contains("client timeout exceeds"), "{err}");
    }

    #[test]
    fn mcp_controlled_live_receipt_uses_observed_control_and_frame_evidence() {
        let cfg = test_config(&["--run", "--scenario", MCP_CONTROLLED_SMOKE_SCENARIO], &[])
            .expect("mcp-controlled live config parses");
        let child_revision = GitRevisionEvidence {
            requested_rev: None,
            resolved_rev: Some("4d1b1554650bd91924f7ce99c9dab69a91142edc".to_string()),
            status: GIT_STATUS_CLEAN,
            dirty: false,
            diagnostics: Vec::new(),
        };
        let client = ClientRunEvidence {
            log_path: Some(PathBuf::from("docs/evidence/mcp.transcript.log")),
            log_paths: vec![PathBuf::from("docs/evidence/mcp.transcript.log")],
            usernames: vec![TEST_USERNAME.to_string()],
            exit_code: None,
            classification: "mcp-controlled-live-evidence",
            matched_success_pattern: Some("mcp_command_outcomes".to_string()),
            scenario: Some(evaluate_scenario_for_config(
                &cfg,
                &mcp_controlled_success_output(),
            )),
            server_scenario: Some(evaluate_server_scenario(
                Scenario::McpControlledSmoke,
                "",
                TEST_USERNAME,
            )),
            projectile_damage_causality: None,
            mcp_control: Some(McpControlRunEvidence {
                handshake_success: true,
                tool_list_digest: mcp_control_tool_list_digest(),
                tool_names: MCP_CONTROL_TOOL_NAMES.to_vec(),
                calls_attempted: MCP_CONTROL_LIVE_CALLS.to_vec(),
                calls_succeeded: MCP_CONTROL_LIVE_CALLS.to_vec(),
                first_failure: None,
                stdout_clean: true,
                command_outcome_ids: MCP_CONTROL_LIVE_OUTCOME_IDS.to_vec(),
            }),
            frame_artifacts: Some(FrameArtifactsReceiptEvidence {
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
                    blake3: "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
                        .to_string(),
                    redaction: "not_reviewed".to_string(),
                    includes_ui: true,
                }],
                missing_digests: Vec::new(),
                path_containment_checked: true,
                promotion_ready: true,
                non_claims: FRAME_ARTIFACT_NON_CLAIMS.to_vec(),
            }),
        };

        let mcp = evaluate_mcp_control_receipt(&cfg, &child_revision, Some(&client));
        let frame = evaluate_frame_artifacts_receipt(&cfg, Some(&client));

        assert!(mcp.passed, "{mcp:?}");
        assert!(mcp.live_receipt);
        assert!(!mcp.dry_run_fixture);
        assert_eq!(mcp.first_failure, None);
        assert!(frame.selected);
        assert_eq!(frame.artifact_count, 1);
        assert!(frame.promotion_ready);
    }

    #[test]
    fn mcp_controlled_live_receipt_fails_dirty_child_revision() {
        let cfg = test_config(&["--run", "--scenario", MCP_CONTROLLED_SMOKE_SCENARIO], &[])
            .expect("mcp-controlled live config parses");
        let child_revision = GitRevisionEvidence {
            requested_rev: None,
            resolved_rev: Some("4d1b1554650bd91924f7ce99c9dab69a91142edc".to_string()),
            status: GIT_STATUS_DIRTY,
            dirty: true,
            diagnostics: Vec::new(),
        };
        let client = mcp_controlled_dry_run_evidence(&cfg);

        let mcp = evaluate_mcp_control_receipt(&cfg, &child_revision, Some(&client));

        assert!(!mcp.passed, "{mcp:?}");
        assert_eq!(mcp.first_failure, Some(MCP_CONTROL_FAILURE_REVISION_DIRTY));
    }

    #[test]
    fn defaults_to_valence_protocol_and_port() {
        let cfg = test_config(&[], &[]).expect("default config parses");

        assert_eq!(cfg.root, PathBuf::from("/workspace/mc"));
        assert_eq!(cfg.client_dir, PathBuf::from("/workspace/mc/stevenarella"));
        assert_eq!(cfg.valence_repo, PathBuf::from("/workspace/mc/valence"));
        assert_eq!(cfg.server_backend, ServerBackend::Valence);
        assert_eq!(cfg.server_protocol, DEFAULT_SERVER_PROTOCOL);
        assert_eq!(cfg.server_port, VALENCE_DEFAULT_SERVER_PORT);
        assert_eq!(cfg.valence_rev, DEFAULT_VALENCE_REV);
        assert_eq!(cfg.mode, Mode::DryRun);
    }

    #[test]
    fn backend_runtime_dispatch_preserves_pure_facts_and_log_labels() {
        let valence = test_config(&[], &[]).expect("default config parses");
        let paper =
            test_config(&["--server-backend=paper"], &[]).expect("paper backend config parses");

        assert_eq!(backend_name(ServerBackend::Valence), "valence");
        assert_eq!(backend_name(ServerBackend::Paper), "paper");
        assert_eq!(
            default_port(ServerBackend::Valence),
            VALENCE_DEFAULT_SERVER_PORT
        );
        assert_eq!(
            default_port(ServerBackend::Paper),
            PAPER_DEFAULT_SERVER_PORT
        );
        assert_eq!(
            server_log_label(&valence),
            valence.valence_log.display().to_string()
        );
        assert_eq!(
            server_log_label(&paper),
            format!("docker logs {}", paper.server_name)
        );
        assert!(
            world_persistence_state_dir(&valence, ServerBackend::Valence)
                .display()
                .to_string()
                .contains("valence"),
            "Valence persistence path uses stable backend name"
        );
        assert!(
            world_persistence_state_dir(&paper, ServerBackend::Paper)
                .display()
                .to_string()
                .contains("paper"),
            "Paper persistence path uses stable backend name"
        );
    }

    #[test]
    fn backend_runtime_dry_run_lifecycle_uses_expected_managed_server_shape() {
        let temp_root =
            std::env::temp_dir().join(format!("mc-compat-backend-runtime-{}", std::process::id()));
        let _ = fs::remove_dir_all(&temp_root);
        fs::create_dir_all(&temp_root).expect("create backend runtime temp root");

        let mut valence = test_config(&[], &[]).expect("default config parses");
        valence.valence_repo = temp_root.join("valence-repo");
        valence.valence_worktree = temp_root.join("valence-worktree");
        valence.valence_pid_file = temp_root.join("valence.pid");
        fs::create_dir_all(&valence.valence_repo).expect("create fake Valence repo");
        let valence_server = start_server(&valence).expect("Valence dry-run lifecycle starts");
        assert!(valence_server.child.is_none());
        assert!(valence_server.paper_container.is_none());
        assert_eq!(valence_server.pid_file, valence.valence_pid_file);
        assert!(valence_server.keep);

        let mut paper =
            test_config(&["--server-backend=paper"], &[]).expect("paper backend config parses");
        paper.valence_pid_file = temp_root.join("paper.pid");
        let paper_server = start_server(&paper).expect("Paper dry-run lifecycle starts");
        assert!(paper_server.child.is_none());
        assert_eq!(
            paper_server.paper_container.as_deref(),
            Some(paper.server_name.as_str())
        );
        assert_eq!(paper_server.pid_file, paper.valence_pid_file);
        assert!(paper_server.keep);

        let _ = fs::remove_dir_all(&temp_root);
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
    fn cleanup_client_log_match_is_narrow() {
        assert!(is_mc_compat_client_log("mc-compat-client.123.log"));
        assert!(!is_mc_compat_client_log("mc-compat-client.123.txt"));
        assert!(!is_mc_compat_client_log("other-mc-compat-client.123.log"));
    }

    #[test]
    fn protocol_varint_round_trips_in_memory() {
        let cases = [
            STATUS_PACKET_ID,
            STATUS_HANDSHAKE_NEXT_STATE,
            TEST_VARINT_SINGLE_BYTE_VALUE,
            TEST_VARINT_TWO_BYTE_VALUE,
            DEFAULT_SERVER_PROTOCOL,
        ];

        for value in cases {
            let mut bytes = Vec::new();
            bytes.write_varint(value).expect("write varint");
            let decoded = Cursor::new(bytes).read_varint().expect("read varint");
            assert_eq!(decoded, value);
        }
    }

    #[test]
    fn protocol_string_and_packet_framing_match_expected_bytes() {
        let mut string_bytes = Vec::new();
        string_bytes
            .write_mc_string(TEST_MC_STRING)
            .expect("write string");
        assert_eq!(
            string_bytes,
            vec![TEST_MC_STRING_LENGTH, b'm', b'c'],
            "Minecraft string framing changed"
        );
        assert_eq!(
            Cursor::new(string_bytes)
                .read_mc_string()
                .expect("read string"),
            TEST_MC_STRING
        );

        let payload = [
            TEST_PACKET_PAYLOAD_FIRST_BYTE,
            TEST_PACKET_PAYLOAD_SECOND_BYTE,
        ];
        let mut packet = Vec::new();
        packet
            .write_packet(STATUS_HANDSHAKE_NEXT_STATE, &payload)
            .expect("write packet");
        assert_eq!(
            packet,
            vec![
                TEST_PACKET_BODY_LENGTH,
                STATUS_HANDSHAKE_NEXT_STATE as u8,
                TEST_PACKET_PAYLOAD_FIRST_BYTE,
                TEST_PACKET_PAYLOAD_SECOND_BYTE,
            ]
        );
    }

    #[test]
    fn protocol_status_handshake_fixture_bytes_are_stable() {
        let mut payload = Vec::new();
        payload
            .write_varint(DEFAULT_SERVER_PROTOCOL)
            .expect("protocol varint");
        payload
            .write_mc_string(STATUS_LOCALHOST_ADDRESS)
            .expect("host string");
        payload.extend_from_slice(&TEST_STATUS_PORT.to_be_bytes());
        payload
            .write_varint(STATUS_HANDSHAKE_NEXT_STATE)
            .expect("next state");

        let mut framed = Vec::new();
        framed
            .write_packet(STATUS_PACKET_ID, &payload)
            .expect("status handshake packet");
        let mut cursor = Cursor::new(framed);
        let packet_length = cursor.read_varint().expect("packet length");
        let packet_id = cursor.read_varint().expect("packet id");

        assert_eq!(packet_id, STATUS_PACKET_ID);
        assert_eq!(
            packet_length as usize,
            payload.len() + TEST_STATUS_PACKET_ID_BYTE_LENGTH
        );
    }

    #[test]
    fn protocol_invalid_inputs_fail_closed() {
        let eof = Cursor::new(Vec::new()).read_varint().unwrap_err();
        assert!(eof.contains("failed to fill whole buffer"), "{eof}");

        let too_long = vec![VARINT_CONTINUATION_BIT; TEST_TOO_LONG_VARINT_BYTES];
        let err = Cursor::new(too_long).read_varint().unwrap_err();
        assert_eq!(err, "varint too long");

        let truncated_string = vec![TEST_MC_STRING_LENGTH, b'm'];
        let err = Cursor::new(truncated_string).read_mc_string().unwrap_err();
        assert!(err.contains("failed to fill whole buffer"), "{err}");
    }

    #[test]
    fn cleanup_path_dry_run_preserves_existing_files() {
        let dir =
            std::env::temp_dir().join(format!("mc-compat-cleanup-dry-run-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).expect("create cleanup dry-run fixture");
        let file = dir.join("artifact.log");
        fs::write(&file, "keep me").expect("write cleanup fixture");

        cleanup_path("test artifact", &file, false).expect("dry-run cleanup succeeds");
        assert!(file.exists(), "dry-run cleanup must not remove files");

        cleanup_path("test artifact", &file, true).expect("apply cleanup removes file");
        assert!(!file.exists(), "apply cleanup removes files");
        let _ = fs::remove_dir_all(&dir);
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
        let mut cfg = Config::defaults(PathBuf::from("/workspace/mc"));

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

        let err =
            test_config(&["--steel-config", path.to_str().expect("utf8 path")], &[]).unwrap_err();
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
        let cli =
            test_config(&["--scenario", "flag-score-repeat"], &[]).expect("scenario CLI parses");
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

        let blue =
            test_config(&["--scenario", "blue-flag-score"], &[]).expect("blue scenario parses");
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

        let crafting_breadth =
            test_config(&["--scenario", "survival-crafting-recipe-breadth"], &[])
                .expect("survival crafting recipe breadth scenario parses");
        assert_eq!(
            crafting_breadth.scenario,
            Scenario::SurvivalCraftingRecipeBreadth
        );

        let furnace = test_config(&["--scenario", "survival-furnace-persistence"], &[])
            .expect("survival furnace scenario parses");
        assert_eq!(furnace.scenario, Scenario::SurvivalFurnacePersistence);

        let furnace_breadth =
            test_config(&["--scenario", "survival-furnace-smelting-breadth"], &[])
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

    #[test]
    fn evidence_matchers_cover_supported_positive_cases() {
        let corpus = EvidenceCorpus::new(
            "Detected server protocol version\nCompatBotA joined\nSCOREBOARD flag update\nupdate_health health=17.0\n",
        );
        let context = EvidenceContext {
            username: "CompatBot",
        };

        assert!(
            MatcherKind::Literal("Detected server protocol version").is_match(&corpus, &context)
        );
        assert!(MatcherKind::CaseInsensitive("scoreboard").is_match(&corpus, &context));
        assert!(MatcherKind::DynamicUsername.is_match(&corpus, &context));
        assert!(MatcherKind::DynamicClientSuffix(CLIENT_A_SUFFIX).is_match(&corpus, &context));
        assert!(
            MatcherKind::AnyOfCaseInsensitive(FLAG_OR_SCORE_NEEDLES).is_match(&corpus, &context)
        );

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

    #[test]
    fn enriched_triage_receipt_preserves_existing_fields_and_adds_context() {
        let cfg = test_config(
            &[
                "--server-backend=valence",
                "--scenario=flag-score-repeat",
                "--receipt=/tmp/receipt.json",
                "--client-dir=/tmp/stevenarella",
            ],
            &[],
        )
        .expect("receipt config parses");
        let err = "server status probe failed with token=secret /tmp/private".to_string();

        let json = smoke_receipt_json(&cfg, Err(&err));

        assert!(json.contains("\"suggested_boundary\""), "{json}");
        assert!(json.contains("\"enriched\""), "{json}");
        assert!(json.contains("\"boundary_confidence\""), "{json}");
        assert!(json.contains(TRIAGE_REDACTED), "{json}");
    }

    fn typed_event_fixture_lines() -> Vec<&'static str> {
        vec![
            "MC-COMPAT-EVENT schema=1 source=client scenario=smoke session=s1 username=compatbot seq=1 event=protocol_detected",
            "MC-COMPAT-EVENT schema=1 source=client scenario=smoke session=s1 username=compatbot seq=2 event=join_game",
            "MC-COMPAT-EVENT schema=1 source=client scenario=smoke session=s1 username=compatbot seq=3 event=render_tick",
        ]
    }

    fn typed_event_fixture() -> Vec<TypedEvent> {
        typed_event_fixture_lines()
            .into_iter()
            .map(|line| parse_typed_event_line(line).expect("typed event parses"))
            .collect()
    }

    const TEST_SESSION_ID: &str = "s1";
    const TEST_USERNAME: &str = "compatbot";
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
            .all(|event| event.source == "client" || event.source == "server"));
        assert!(events
            .windows(2)
            .all(|pair| pair[0].sequence < pair[1].sequence));
        assert!(result.passed, "{result:?}");
        assert_eq!(result.observed_events.len(), required_events.len());
        assert!(result.missing_events.is_empty(), "{result:?}");
        assert!(result.forbidden_events.is_empty(), "{result:?}");
        assert!(result.order_violations.is_empty(), "{result:?}");
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
            Scenario::InventoryInteraction
        ));
        assert!(typed_event_oracle_contributes_to_pass_fail(
            Scenario::InventoryStackSplitMerge
        ));
        assert!(!typed_event_oracle_contributes_to_pass_fail(
            Scenario::InventoryDragTransactions
        ));
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

    #[test]
    fn typed_event_receipt_artifact_records_reviewable_timeline_hash() {
        let events = typed_event_fixture();
        let timeline = normalize_typed_event_timeline(&events);
        let timeline_blake3 = typed_event_timeline_blake3(&timeline);
        let artifact = TypedEventOracleArtifact {
            event_log_path: PathBuf::from("/tmp/mc-compat.typed-events.log"),
            timeline_blake3: timeline_blake3.clone(),
            event_count: events.len(),
            contributes_to_pass_fail: false,
        };

        let json = typed_event_oracle_receipt_json(Some(&artifact));
        let receipt = parse_structured_typed_event_receipt(&json)
            .expect("typed event receipt parses structurally");
        validate_structured_typed_event_receipt(&receipt)
            .expect("typed event receipt validates structurally");

        assert!(receipt.selected);
        assert_eq!(
            receipt.migration_status,
            TYPED_EVENT_MIGRATION_DERIVED_FROM_MILESTONES
        );
        assert_eq!(
            receipt.event_log_path.as_deref(),
            Some("/tmp/mc-compat.typed-events.log")
        );
        assert_eq!(
            receipt.timeline_blake3.as_deref(),
            Some(timeline_blake3.as_str())
        );
        assert_eq!(receipt.event_count as usize, events.len());
        assert!(!receipt.contributes_to_pass_fail);
        assert!(!receipt.raw_payloads_recorded);
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
    fn typed_event_oracle_receipt_records_migration_fallback() {
        let cfg = test_config(
            &[
                "--server-backend=paper",
                "--receipt",
                "/tmp/receipt.json",
                "--client-dir",
                "/tmp/stevenarella",
            ],
            &[],
        )
        .expect("receipt config parses");
        let json = smoke_receipt_json(&cfg, Err("preflight"));
        let typed_event = parse_structured_typed_event_receipt(
            json_object_slice(&json, "typed_event_oracle").expect("typed event object"),
        )
        .expect("typed event fallback parses");

        validate_structured_typed_event_receipt(&typed_event)
            .expect("typed event fallback validates");
        assert!(!typed_event.selected);
        assert_eq!(typed_event.migration_status, TYPED_EVENT_MIGRATION_FALLBACK);
        assert!(!typed_event.raw_payloads_recorded);
    }

    #[test]
    fn status_packet_proxy_and_gameplay_receipt_blocks_are_recorded() {
        let mut cfg = test_config(
            &[
                "--server-backend=valence",
                "--scenario=reconnect-flag-score",
                "--expect-status-description=compat fixture",
                "--expect-status-version=compat-version",
                "--expect-status-sample=compatbot,observer",
                "--packet-capture-summary",
                "--proxy-route=velocity-local",
                "--proxy-forwarding-mode=modern",
                "--client-dir=/tmp/stevenarella",
            ],
            &[],
        )
        .expect("extended receipt config parses");
        cfg.server_port = 25565;
        let scenario = evaluate_scenario(
            Scenario::ReconnectFlagScore,
            "Detected server protocol version 763
join_game
render_tick_with_player
You are on team RED!
You have the flag!
You captured the flag!
RED: 1
mc_compat_reconnect_session=2
",
        );
        assert!(scenario.passed, "{scenario:?}");
        let client = Some(ClientRunEvidence {
            log_path: Some(PathBuf::from("/tmp/client.log")),
            log_paths: vec![PathBuf::from("/tmp/client.log")],
            usernames: vec!["compatbot".to_string()],
            exit_code: Some(124),
            classification: "timeout-success-evidence",
            matched_success_pattern: Some("Detected server protocol version".to_string()),
            scenario: Some(scenario),
            server_scenario: Some(evaluate_server_scenario(
                Scenario::ReconnectFlagScore,
                "compatbot joined
red flag captured
",
                "compatbot",
            )),
            projectile_damage_causality: None,
            mcp_control: None,
            frame_artifacts: None,
        });

        let json = smoke_receipt_json(&cfg, Ok(&client));

        assert!(
            json.contains("\"name\": \"reconnect-flag-score\""),
            "{json}"
        );
        assert!(json.contains("\"status_response_resource\""), "{json}");
        assert!(json.contains("\"configured\": true"), "{json}");
        assert!(json.contains("compat fixture"), "{json}");
        assert!(json.contains("compat-version"), "{json}");
        assert!(json.contains("compatbot"), "{json}");
        assert!(json.contains("\"packet_capture_oracle\""), "{json}");
        assert!(json.contains("\"selected\": true"), "{json}");
        assert!(json.contains("\"raw_payloads_recorded\": false"), "{json}");
        assert!(json.contains("\"proxy_compat_seam\""), "{json}");
        assert!(json.contains("\"route\": \"velocity-local\""), "{json}");
        assert!(json.contains("\"forwarding_mode\": \"modern\""), "{json}");
        assert!(json.contains("\"mtls_ported\": false"), "{json}");
        assert!(json.contains("\"gameplay_oracles\""), "{json}");
        assert!(json.contains("reconnect_session"), "{json}");
        assert!(json.contains("full_ctf_correctness"), "{json}");
    }

    #[test]
    fn compat_bot_probe_scenario_is_bounded_and_receipted() {
        let pass = evaluate_scenario(
            Scenario::CompatBotProbe,
            "Detected server protocol version 763\njoin_game\nrender_tick_with_player\n",
        );
        assert!(pass.passed, "{pass:?}");
        assert_eq!(pass.missing_milestones, Vec::<&str>::new());

        let fail = evaluate_scenario(
            Scenario::CompatBotProbe,
            "Detected server protocol version 763\n",
        );
        assert!(!fail.passed, "{fail:?}");
        assert!(fail.missing_milestones.contains(&"join_game"));
        assert!(fail.missing_milestones.contains(&"render_tick"));

        let mut cfg = test_config(
            &[
                "--server-backend=valence",
                "--scenario=valence-compat-bot-probe",
                "--receipt=/tmp/receipt.json",
                "--client-dir=/tmp/stevenarella",
            ],
            &[],
        )
        .expect("receipt config parses");
        cfg.server_port = 25565;
        let client = Some(ClientRunEvidence {
            log_path: Some(PathBuf::from("/tmp/client.log")),
            log_paths: vec![PathBuf::from("/tmp/client.log")],
            usernames: vec!["compatbot".to_string()],
            exit_code: Some(124),
            classification: "timeout-success-evidence",
            matched_success_pattern: Some("Detected server protocol version".to_string()),
            scenario: Some(pass),
            server_scenario: Some(evaluate_server_scenario(
                Scenario::CompatBotProbe,
                "compatbot joined\n",
                "compatbot",
            )),
            projectile_damage_causality: None,
            mcp_control: None,
            frame_artifacts: None,
        });

        let json = smoke_receipt_json(&cfg, Ok(&client));

        assert!(
            json.contains("\"name\": \"valence-compat-bot-probe\""),
            "{json}"
        );
        assert!(json.contains("\"compat_bot_probe\""), "{json}");
        assert!(json.contains("\"selected\": true"), "{json}");
        assert!(json.contains("\"safe_bounded_probe\": true"), "{json}");
        assert!(
            json.contains("\"external_server_load_authorized\": false"),
            "{json}"
        );
        assert!(json.contains("\"public_stress_tool\": false"), "{json}");
        assert!(json.contains("\"planned_clients\": 1"), "{json}");
        assert!(json.contains("\"max_clients\": 1"), "{json}");
        assert!(
            json.contains("\"target_address\": \"127.0.0.1:25565\""),
            "{json}"
        );
        assert!(json.contains("\"load_network_safety\""), "{json}");
        assert!(
            json.contains("\"target_scope\": \"owned-local-loopback\""),
            "{json}"
        );
        assert!(
            json.contains("\"claims_public_server_safety\": false"),
            "{json}"
        );
        assert!(json.contains("\"claims_unbounded_soak\": false"), "{json}");
    }

    #[test]
    fn latency_jitter_receipt_renders_bounded_wan_telemetry_fields() {
        const TEST_TIMEOUT_SECS: u64 = 180;
        const TEST_CLIENT_COUNT: usize = 1;
        let receipt = LatencyJitterTelemetryReceipt {
            selected: true,
            mechanism: LATENCY_JITTER_DEFAULT_MECHANISM.to_string(),
            target_rail: "inventory-interaction".to_string(),
            delay_ms: "80".to_string(),
            jitter_ms: "30".to_string(),
            loss_percent: LATENCY_JITTER_DEFAULT_METRIC.to_string(),
            timeout_secs: TEST_TIMEOUT_SECS,
            duration_secs: TEST_TIMEOUT_SECS,
            client_count: TEST_CLIENT_COUNT,
            reconnect_count: NO_RECONNECT_SESSIONS,
            target_ownership: WAN_TARGET_OWNERSHIP_OWNED_LOCAL.to_string(),
            authorization: WAN_AUTHORIZATION_OWNED_LOCAL.to_string(),
            hygiene_status: LATENCY_JITTER_ENABLED_HYGIENE_STATUS,
        };

        let json = render_latency_jitter_receipt_json(&receipt);

        assert!(json.contains("\"selected\": true"), "{json}");
        assert!(
            json.contains("\"target_ownership\": \"owned-local-loopback\""),
            "{json}"
        );
        assert!(
            json.contains("\"authorization\": \"owned-local-fixture-approved\""),
            "{json}"
        );
        assert!(json.contains("\"duration_secs\": 180"), "{json}");
        assert!(json.contains("\"client_count\": 1"), "{json}");
        assert!(json.contains("\"reconnect_count\": 0"), "{json}");
        assert!(json.contains("\"telemetry_samples\""), "{json}");
        assert!(json.contains("\"scenario_observed_milestones\""), "{json}");
        assert!(
            json.contains(
                "\"pass_fail_criteria\": \"inventory_interaction_client_server_milestones\""
            ),
            "{json}"
        );
        assert!(json.contains("\"claims_wan_safety\": false"), "{json}");
        assert!(
            json.contains("\"claims_packet_loss_tolerance\": false"),
            "{json}"
        );
        assert!(
            json.contains("\"claims_public_server_safety\": false"),
            "{json}"
        );
        assert!(
            json.contains("\"claims_production_readiness\": false"),
            "{json}"
        );
    }

    #[test]
    fn latency_jitter_receipt_disabled_path_stays_non_claim() {
        const TEST_TIMEOUT_SECS: u64 = 180;
        const TEST_CLIENT_COUNT: usize = 1;
        let receipt = LatencyJitterTelemetryReceipt {
            selected: false,
            mechanism: LATENCY_JITTER_DEFAULT_MECHANISM.to_string(),
            target_rail: "smoke".to_string(),
            delay_ms: LATENCY_JITTER_DEFAULT_METRIC.to_string(),
            jitter_ms: LATENCY_JITTER_DEFAULT_METRIC.to_string(),
            loss_percent: LATENCY_JITTER_DEFAULT_METRIC.to_string(),
            timeout_secs: TEST_TIMEOUT_SECS,
            duration_secs: TEST_TIMEOUT_SECS,
            client_count: TEST_CLIENT_COUNT,
            reconnect_count: NO_RECONNECT_SESSIONS,
            target_ownership: WAN_TARGET_OWNERSHIP_OWNED_LOCAL.to_string(),
            authorization: WAN_AUTHORIZATION_OWNED_LOCAL.to_string(),
            hygiene_status: LATENCY_JITTER_DISABLED_HYGIENE_STATUS,
        };

        let json = render_latency_jitter_receipt_json(&receipt);

        assert!(json.contains("\"selected\": false"), "{json}");
        assert!(
            json.contains("\"hygiene_status\": \"not-selected\""),
            "{json}"
        );
        assert!(
            json.contains("\"fail_closed_when_unavailable\": true"),
            "{json}"
        );
        assert!(json.contains("\"claims_wan_safety\": false"), "{json}");
        assert!(
            json.contains("\"claims_internet_path_safety\": false"),
            "{json}"
        );
    }

    #[test]
    fn latency_jitter_reconnect_count_is_explicit() {
        assert_eq!(
            latency_jitter_reconnect_count(Scenario::InventoryInteraction),
            NO_RECONNECT_SESSIONS
        );
        assert_eq!(
            latency_jitter_reconnect_count(Scenario::ReconnectFlagState),
            SINGLE_RECONNECT_SESSION
        );
        assert_eq!(
            latency_jitter_reconnect_count(Scenario::NegativeReconnectRace),
            SINGLE_RECONNECT_SESSION
        );
    }

    #[test]
    fn public_server_authorized_safety_receipt_renders_fixture_envelope() {
        const TEST_DURATION_SECS: u64 = 30;
        const TEST_CLIENT_COUNT: usize = 1;
        let receipt = PublicServerAuthorizedSafetyReceipt {
            selected: true,
            target_owner: PUBLIC_SERVER_DEFAULT_TARGET_OWNER.to_string(),
            authorization_artifact: PUBLIC_SERVER_DEFAULT_AUTHORIZATION_ARTIFACT.to_string(),
            target_scope: PUBLIC_SERVER_DEFAULT_TARGET_SCOPE.to_string(),
            client_count: TEST_CLIENT_COUNT,
            duration_secs: TEST_DURATION_SECS,
            checkpoint_decision: PUBLIC_SERVER_DEFAULT_CHECKPOINT_DECISION.to_string(),
            live_traffic_enabled: false,
        };

        let json = render_public_server_authorized_safety_receipt_json(&receipt);

        assert!(json.contains("\"selected\": true"), "{json}");
        assert!(
            json.contains("\"target_owner\": \"review-fixture-owner\""),
            "{json}"
        );
        assert!(
            json.contains("\"target_scope\": \"authorized-non-loopback-fixture\""),
            "{json}"
        );
        assert!(json.contains("\"client_count\": 1"), "{json}");
        assert!(json.contains("\"duration_secs\": 30"), "{json}");
        assert!(json.contains("\"status_probe_only\""), "{json}");
        assert!(json.contains("\"redaction_policy\""), "{json}");
        assert!(
            json.contains("\"checkpoint_decision\": \"approved_for_deterministic_fixture_only\""),
            "{json}"
        );
        assert!(json.contains("\"live_traffic_enabled\": false"), "{json}");
        assert!(
            json.contains("\"claims_authorized_public_envelope\": true"),
            "{json}"
        );
        assert!(
            json.contains("\"claims_live_public_server_safety\": false"),
            "{json}"
        );
        assert!(
            json.contains("\"claims_production_readiness\": false"),
            "{json}"
        );
        assert!(json.contains("\"claims_wan_tolerance\": false"), "{json}");
    }

    #[test]
    fn public_server_authorized_safety_live_mode_fails_closed() {
        assert!(public_server_authorized_safety_selected(true, Mode::DryRun));
        assert!(!public_server_authorized_safety_selected(true, Mode::Run));
        assert!(!public_server_authorized_safety_selected(
            false,
            Mode::DryRun
        ));
    }

    #[test]
    fn public_server_authorized_safety_disabled_path_stays_non_claim() {
        const TEST_DURATION_SECS: u64 = 30;
        const TEST_CLIENT_COUNT: usize = 1;
        let receipt = PublicServerAuthorizedSafetyReceipt {
            selected: false,
            target_owner: PUBLIC_SERVER_DEFAULT_TARGET_OWNER.to_string(),
            authorization_artifact: PUBLIC_SERVER_DEFAULT_AUTHORIZATION_ARTIFACT.to_string(),
            target_scope: PUBLIC_SERVER_DEFAULT_TARGET_SCOPE.to_string(),
            client_count: TEST_CLIENT_COUNT,
            duration_secs: TEST_DURATION_SECS,
            checkpoint_decision: PUBLIC_SERVER_DEFAULT_CHECKPOINT_DECISION.to_string(),
            live_traffic_enabled: false,
        };

        let json = render_public_server_authorized_safety_receipt_json(&receipt);

        assert!(json.contains("\"selected\": false"), "{json}");
        assert!(
            json.contains("\"claims_authorized_public_envelope\": false"),
            "{json}"
        );
        assert!(
            json.contains("\"claims_live_public_server_safety\": false"),
            "{json}"
        );
        assert!(
            json.contains("\"claims_third_party_target_safety_without_authorization\": false"),
            "{json}"
        );
    }

    fn baseline_negative_live_rail_inputs() -> NegativeLiveRailInputs {
        NegativeLiveRailInputs {
            selected: true,
            rail: Some("negative-custom-payload"),
            invalid_action: Some("malformed_custom_payload"),
            expected_outcome: Some(NEGATIVE_LIVE_RAIL_EXPECTED_OUTCOME),
            observed_outcome: Some(NEGATIVE_LIVE_RAIL_OBSERVED_OUTCOME_CONTAINMENT),
            observed_outcome_source: Some(
                "client_milestone:negative_custom_payload_contained".to_string(),
            ),
            postcondition_milestone: Some("negative_custom_payload_contained"),
            telemetry_required: true,
            telemetry_present: true,
            target_scope: SAFETY_OWNED_LOCAL_SCOPE,
            explicit_authorization: false,
            public_target: false,
            planned_clients: 1,
            max_clients: NEGATIVE_LIVE_RAIL_MAX_CLIENTS,
            timeout_secs: 20,
        }
    }

    #[test]
    fn negative_live_rail_checker_rejects_unbounded_public_unauthenticated_inputs() {
        let mut inputs = baseline_negative_live_rail_inputs();
        inputs.public_target = true;
        inputs.planned_clients = NEGATIVE_LIVE_RAIL_MAX_CLIENTS + 1;
        let evidence = evaluate_negative_live_rail_safety_from_inputs(inputs);
        assert!(!evidence.preflight_passed, "{evidence:?}");
        assert!(evidence
            .bound_violations
            .contains(&"public_target_without_authorization"));
        assert!(evidence
            .bound_violations
            .contains(&"planned_clients_exceed_negative_max"));
    }

    #[test]
    fn negative_live_rail_checker_rejects_missing_telemetry() {
        let mut inputs = baseline_negative_live_rail_inputs();
        inputs.telemetry_present = false;
        inputs.observed_outcome = None;
        inputs.observed_outcome_source = None;
        let evidence = evaluate_negative_live_rail_safety_from_inputs(inputs);
        assert!(!evidence.preflight_passed, "{evidence:?}");
        assert!(evidence.missing_fields.contains(&"telemetry"));
    }

    #[test]
    fn negative_live_rail_checker_rejects_missing_expected_outcome() {
        let mut inputs = baseline_negative_live_rail_inputs();
        inputs.expected_outcome = None;
        let evidence = evaluate_negative_live_rail_safety_from_inputs(inputs);
        assert!(!evidence.preflight_passed, "{evidence:?}");
        assert!(evidence.missing_fields.contains(&"expected_outcome"));
    }

    #[test]
    fn negative_live_rail_receipt_records_observed_containment_outcome() {
        let cfg = test_config(&["--run", "--scenario", "negative-custom-payload"], &[])
            .expect("negative live rail config parses");
        let scenario = evaluate_scenario(
            Scenario::NegativeCustomPayload,
            "Detected server protocol version 763
join_game
render_tick_with_player
negative_custom_payload_sent
negative_custom_payload_contained
",
        );
        assert!(scenario.passed, "{scenario:?}");
        let client = Some(ClientRunEvidence {
            log_path: Some(PathBuf::from("/tmp/client.log")),
            log_paths: vec![PathBuf::from("/tmp/client.log")],
            usernames: vec!["compatbot".to_string()],
            exit_code: Some(124),
            classification: "timeout-success-evidence",
            matched_success_pattern: Some("Detected server protocol version".to_string()),
            scenario: Some(scenario),
            server_scenario: Some(evaluate_server_scenario(
                Scenario::NegativeCustomPayload,
                "compatbot joined\n",
                "compatbot",
            )),
            projectile_damage_causality: None,
            mcp_control: None,
            frame_artifacts: None,
        });

        let json = smoke_receipt_json(&cfg, Ok(&client));
        assert!(
            json.contains("\"observed_outcome\": \"containment_observed\""),
            "{json}"
        );
        assert!(
            json.contains("client_milestone:negative_custom_payload_contained"),
            "{json}"
        );
        assert!(json.contains("\"telemetry_present\": true"), "{json}");
        assert!(json.contains("\"preflight_passed\": true"), "{json}");
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
    fn negative_live_rail_envelope_records_expected_outcome_and_non_claims() {
        let cfg = test_config(
            &["--dry-run", "--scenario", "negative-inventory-stale-state"],
            &[],
        )
        .expect("negative rail config parses");
        let evidence = evaluate_negative_live_rail_safety(&cfg);
        assert!(evidence.selected, "{evidence:?}");
        assert_eq!(evidence.rail, Some("negative-inventory-stale-state"));
        assert_eq!(evidence.invalid_action, Some("stale_inventory_state_id"));
        assert_eq!(
            evidence.expected_outcome,
            Some(NEGATIVE_LIVE_RAIL_EXPECTED_OUTCOME)
        );
        assert!(evidence.owned_local_target, "{evidence:?}");
        assert!(evidence.preflight_passed, "{evidence:?}");

        let json = smoke_receipt_json(&cfg, Ok(&None));
        assert!(json.contains("\"negative_live_rail\""), "{json}");
        assert!(json.contains("\"selected\": true"), "{json}");
        assert!(json.contains("stale_inventory_state_id"), "{json}");
        assert!(json.contains(NEGATIVE_LIVE_RAIL_EXPECTED_OUTCOME), "{json}");
        assert!(json.contains("broad_invalid_input_coverage"), "{json}");
        assert!(json.contains("\"raw_payloads_recorded\": false"), "{json}");
    }

    #[test]
    fn negative_live_rail_preflight_rejects_public_unowned_targets() {
        let cfg = test_config(
            &["--dry-run", "--scenario", "negative-custom-payload"],
            &[("MC_COMPAT_PUBLIC_TARGET", "1")],
        )
        .expect("negative rail config parses");
        let err = validate_negative_live_rail_preflight(&cfg)
            .expect_err("public negative rail without authorization fails");
        assert!(err.contains("public_target_without_authorization"), "{err}");
    }

    #[test]
    fn load_network_safety_envelope_fails_closed_for_unsafe_inputs() {
        let safe = evaluate_load_network_safety(LoadNetworkSafetyInputs {
            target_scope: SAFETY_OWNED_LOCAL_SCOPE,
            owned_local_target: true,
            explicit_authorization: false,
            public_target: false,
            planned_clients: SAFETY_MAX_LOCAL_CLIENTS,
            max_clients: SAFETY_MAX_LOCAL_CLIENTS,
            duration_secs: SAFETY_MAX_DURATION_SECS,
            max_duration_secs: SAFETY_MAX_DURATION_SECS,
            reconnect_sessions: SAFETY_SINGLE_SESSION_COUNT,
            latency_ms: SAFETY_ZERO_VALUE.to_string(),
            jitter_ms: SAFETY_ZERO_VALUE.to_string(),
            loss_percent: SAFETY_ZERO_VALUE.to_string(),
            telemetry_present: true,
            live_receipt: true,
        });
        assert!(safe.preflight_passed, "{safe:?}");
        assert!(safe.promotion_ready, "{safe:?}");

        let unsafe_public = evaluate_load_network_safety(LoadNetworkSafetyInputs {
            target_scope: "public",
            owned_local_target: false,
            explicit_authorization: false,
            public_target: true,
            planned_clients: SAFETY_MAX_LOCAL_CLIENTS + 1,
            max_clients: SAFETY_MAX_LOCAL_CLIENTS,
            duration_secs: SAFETY_MAX_DURATION_SECS + 1,
            max_duration_secs: SAFETY_MAX_DURATION_SECS,
            reconnect_sessions: SAFETY_SINGLE_SESSION_COUNT,
            latency_ms: String::new(),
            jitter_ms: SAFETY_ZERO_VALUE.to_string(),
            loss_percent: SAFETY_ZERO_VALUE.to_string(),
            telemetry_present: false,
            live_receipt: false,
        });
        assert!(!unsafe_public.preflight_passed, "{unsafe_public:?}");
        assert!(!unsafe_public.promotion_ready, "{unsafe_public:?}");
        assert!(unsafe_public.missing_fields.contains(&"latency_ms"));
        assert!(unsafe_public
            .bound_violations
            .contains(&"public_target_without_authorization"));
        assert!(unsafe_public
            .bound_violations
            .contains(&"planned_clients_exceed_max"));
        assert!(unsafe_public
            .bound_violations
            .contains(&"duration_exceeds_max"));
    }

    #[test]
    fn multi_client_scenario_tracks_client_and_server_evidence() {
        let cfg = test_config(
            &["--scenario", "multi-client-load-score"],
            &[("CLIENT_TIMEOUT", "150")],
        )
        .expect("multi-client config parses");
        assert_eq!(
            planned_client_usernames(&cfg),
            vec!["compatbota", "compatbotb"]
        );
        assert_eq!(client_timeout_secs(&cfg, 0), 150);
        assert_eq!(
            client_timeout_secs(&cfg, 1),
            MULTI_CLIENT_LOAD_PEER_TIMEOUT_SECS
        );

        let client = evaluate_scenario(
            Scenario::MultiClientLoadScore,
            "mc_compat_multi_client_count=2\nDetected server protocol version 763\njoin_game\nrender_tick_with_player\nYou are on team RED!\nYou have the flag!\nYou captured the flag!\nRED: 1\n",
        );
        assert!(client.passed, "{client:?}");

        let server = evaluate_server_scenario(
            Scenario::MultiClientLoadScore,
            "compatbota joined\ncompatbotb joined\nred flag captured\n",
            "compatbot",
        );
        assert!(server.passed, "{server:?}");

        let missing_peer = evaluate_server_scenario(
            Scenario::MultiClientLoadScore,
            "compatbota joined\nred flag captured\n",
            "compatbot",
        );
        assert!(!missing_peer.passed, "{missing_peer:?}");
        assert!(missing_peer
            .missing_milestones
            .contains(&"server_client_b_seen"));
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
    fn armor_loadout_enchantment_status_matrix_receipt_keeps_nonclaims() {
        let cfg = test_config(
            &["--scenario", "armor-loadout-enchantment-status-matrix"],
            &[],
        )
        .expect("armor matrix config parses");
        let scenario = evaluate_scenario(
            Scenario::ArmorLoadoutEnchantmentStatusMatrix,
            "mc_compat_combat_client_count=2\nDetected server protocol version 763\njoin_game\nrender_tick_with_player\nYou are on team RED!\nYou are on team BLUE!\nremote_player_spawn\ninventory_probe_set_slot\ncombat_probe_attack_sent\nupdate_health health=18.0\n",
        );
        let server = evaluate_server_scenario(
            Scenario::ArmorLoadoutEnchantmentStatusMatrix,
            "compatbota joined\ncompatbotb joined\nMC-COMPAT-MILESTONE armor_equipment_state username=compatbotb slot=chest item=DiamondChestplate source=team_inventory_setup\nMC-COMPAT-MILESTONE combat_damage attacker=compatbota victim=compatbotb damage=2.0 victim_health_before=20.0 victim_health_after=18.0 attacker_item=WoodenSword\nMC-COMPAT-MILESTONE combat_armor_mitigation attacker=compatbota victim=compatbotb base_damage=4.0 mitigation=2.0 final_damage=2.0 chest_item=DiamondChestplate victim_health_before=20.0 victim_health_after=18.0\n",
            "compatbot",
        );
        let matrix = evaluate_armor_loadout_enchantment_status_matrix(&cfg, &scenario, &server);
        assert!(matrix.selected, "{matrix:?}");
        assert!(!matrix.live_receipt, "{matrix:?}");
        assert!(!matrix.promotion_ready, "{matrix:?}");
        assert_eq!(matrix.row_id, ARMOR_MATRIX_ROW_ID);
        let json = render_armor_loadout_enchantment_status_matrix_json(&matrix);
        assert!(
            json.contains("\"row_id\": \"chest_diamond_none_none_melee\""),
            "{json}"
        );
        assert!(
            json.contains("\"loadout_id\": \"armor_loadout_chest_only\""),
            "{json}"
        );
        assert!(json.contains("\"reference_required\": false"), "{json}");
        assert!(json.contains("\"promotion_ready\": false"), "{json}");
        assert!(json.contains("\"all_enchantments\""), "{json}");
        assert!(json.contains("\"full_combat_correctness\""), "{json}");
    }

    #[test]
    fn projectile_damage_attribution_scenario_tracks_client_and_server_evidence() {
        let cfg = test_config(
            &["--scenario", "projectile-damage-attribution"],
            &[("CLIENT_TIMEOUT", "150")],
        )
        .expect("projectile damage config parses");
        assert_eq!(
            planned_client_usernames(&cfg),
            vec!["compatbota", "compatbotb"]
        );

        let client = evaluate_scenario(
            Scenario::ProjectileDamageAttribution,
            "mc_compat_projectile_damage_client_count=2\nDetected server protocol version 763\njoin_game\nrender_tick_with_player\nYou are on team RED!\nYou are on team BLUE!\nremote_player_spawn\nprojectile_probe_use_item_sent\nprojectile_probe_swing_sent\nupdate_health health=17.0\n",
        );
        assert!(client.passed, "{client:?}");

        let server = evaluate_server_scenario(
            Scenario::ProjectileDamageAttribution,
            "compatbota joined\ncompatbotb joined\nMC-COMPAT-MILESTONE projectile_loadout username=compatbota slot=0 item=Bow arrows=16\nMC-COMPAT-MILESTONE projectile_use attacker=compatbota victim=compatbotb hand=Main sequence=303 damage=3.0\nMC-COMPAT-MILESTONE projectile_hit attacker=compatbota victim=compatbotb victim_health_before=20.0 victim_health_after=17.0\n",
            "compatbot",
        );
        assert!(server.passed, "{server:?}");

        let missing_hit = evaluate_server_scenario(
            Scenario::ProjectileDamageAttribution,
            "compatbota joined\ncompatbotb joined\nMC-COMPAT-MILESTONE projectile_loadout username=compatbota slot=0 item=Bow arrows=16\nMC-COMPAT-MILESTONE projectile_use attacker=compatbota victim=compatbotb hand=Main sequence=303 damage=3.0\n",
            "compatbot",
        );
        assert!(!missing_hit.passed, "{missing_hit:?}");
        assert!(missing_hit
            .missing_milestones
            .contains(&"server_projectile_hit"));

        let attacker_log = "projectile_probe_use_item_sent hand=main sequence=303\nprojectile_probe_swing_sent hand=main\n";
        let victim_log = "update_health health=17.0 food=20 saturation=0.0\n";
        let server_log = "MC-COMPAT-MILESTONE projectile_use attacker=compatbota victim=compatbotb hand=Main sequence=303 damage=3.0\nMC-COMPAT-MILESTONE projectile_hit attacker=compatbota victim=compatbotb victim_health_before=20.0 victim_health_after=17.0\n";
        let client_logs = [
            ClientLogSlice {
                username: "compatbota",
                output: attacker_log,
            },
            ClientLogSlice {
                username: "compatbotb",
                output: victim_log,
            },
        ];
        let causal = evaluate_projectile_damage_causality(&client_logs, server_log, "compatbot");
        assert!(causal.passed, "{causal:?}");
        assert!(causal.missing_steps.is_empty(), "{causal:?}");
        assert!(causal.order_violations.is_empty(), "{causal:?}");

        let out_of_order_server = "MC-COMPAT-MILESTONE projectile_hit attacker=compatbota victim=compatbotb victim_health_before=20.0 victim_health_after=17.0\nMC-COMPAT-MILESTONE projectile_use attacker=compatbota victim=compatbotb hand=Main sequence=303 damage=3.0\n";
        let causal_order_fail =
            evaluate_projectile_damage_causality(&client_logs, out_of_order_server, "compatbot");
        assert!(!causal_order_fail.passed, "{causal_order_fail:?}");
        assert!(causal_order_fail
            .order_violations
            .contains(&"server_projectile_use_before_hit"));

        let missing_victim_health = evaluate_projectile_damage_causality(
            &[ClientLogSlice {
                username: "compatbota",
                output: attacker_log,
            }],
            server_log,
            "compatbot",
        );
        assert!(!missing_victim_health.passed, "{missing_victim_health:?}");
        assert!(missing_victim_health
            .missing_steps
            .contains(&"victim_client_damage_update"));
    }

    #[test]
    fn projectile_damage_dry_run_uses_steel_arrow_policy() {
        let mut cfg = test_config(
            &[
                "--scenario",
                "projectile-damage-attribution",
                "--valence-rev",
                PINNED_PROJECTILE_DAMAGE_VALENCE_REV,
            ],
            &[],
        )
        .expect("projectile damage config parses");
        cfg.arrow_damage_policy = runtime_config::ArrowDamagePolicy {
            base_damage: 4.0,
            velocity_multiplier: DEFAULT_ARROW_VELOCITY_MULTIPLIER,
            max_damage: DEFAULT_ARROW_MAX_DAMAGE,
        };

        let evidence = projectile_damage_dry_run_evidence(&cfg);
        assert!(
            evidence
                .scenario
                .as_ref()
                .expect("scenario evidence")
                .passed,
            "{evidence:?}"
        );
        let causality = evidence
            .projectile_damage_causality
            .as_ref()
            .expect("causality evidence");
        assert!(causality.passed, "{causality:?}");
        assert!(causality
            .observed_steps
            .contains(&"server_projectile_hit_attacker_victim_health_delta"));
    }

    #[test]
    fn projectile_damage_attribution_requires_pinned_valence_revision() {
        let cfg = test_config(
            &[
                "--dry-run",
                "--scenario",
                "projectile-damage-attribution",
                "--valence-rev",
                "HEAD",
            ],
            &[],
        )
        .expect("config parses before execution validation");
        let err = validate_projectile_damage_dependency(&cfg).unwrap_err();
        assert!(err.contains(PINNED_PROJECTILE_DAMAGE_VALENCE_REV), "{err}");

        let pinned = test_config(
            &[
                "--dry-run",
                "--scenario",
                "projectile-damage-attribution",
                "--valence-rev",
                PINNED_PROJECTILE_DAMAGE_VALENCE_REV,
            ],
            &[],
        )
        .expect("pinned config parses");
        validate_projectile_damage_dependency(&pinned).expect("pinned revision accepted");
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
    fn equipment_slot_item_matrix_expansion_receipt_keeps_nonclaims() {
        let cfg = test_config(&["--scenario", "equipment-slot-item-matrix-expansion"], &[])
            .expect("equipment matrix config parses");
        let scenario = evaluate_scenario(
            Scenario::EquipmentSlotItemMatrixExpansion,
            "mc_compat_equipment_update_client_count=2\nDetected server protocol version 763\njoin_game\nrender_tick_with_player\nYou are on team RED!\nYou are on team BLUE!\nremote_player_spawn entity_id=4\nequipment_probe_entity_equipment entity_id=4 entries=1 slots=slot4:id=829:count=1\n",
        );
        let server = evaluate_server_scenario(
            Scenario::EquipmentSlotItemMatrixExpansion,
            "compatbota joined\ncompatbotb joined\nMC-COMPAT-MILESTONE equipment_update_state username=compatbotb slot=main_hand item_id=829 count=1\n",
            "compatbot",
        );
        let matrix = evaluate_equipment_slot_item_matrix_expansion(&cfg, &scenario, &server);
        assert!(matrix.selected, "{matrix:?}");
        assert!(!matrix.live_receipt, "{matrix:?}");
        assert!(!matrix.promotion_ready, "{matrix:?}");
        assert_eq!(matrix.row_id, EQUIPMENT_MATRIX_ROW_ID);
        let json = render_equipment_slot_item_matrix_expansion_json(&matrix);
        assert!(
            json.contains("\"row_id\": \"remote_main_hand_slot4_item829_count1_non_empty\""),
            "{json}"
        );
        assert!(json.contains("\"wire_slot\": \"slot4\""), "{json}");
        assert!(json.contains("\"item_id\": \"829\""), "{json}");
        assert!(json.contains("\"promotion_ready\": false"), "{json}");
        assert!(json.contains("\"all_equipment_slots\""), "{json}");
        assert!(json.contains("\"full_equipment_semantics\""), "{json}");
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
        let raw_client_result =
            evaluate_scenario(Scenario::SurvivalCrashRecoveryParity, raw_client);
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

    #[test]
    fn missing_valence_checkout_has_actionable_diagnostic() {
        let missing =
            std::env::temp_dir().join(format!("mc-compat-missing-valence-{}", std::process::id()));
        let cfg = test_config(&["--valence-repo", missing.to_str().unwrap()], &[])
            .expect("config with missing Valence repo parses");

        let err = ensure_valence_repo_ready(&cfg).unwrap_err();

        assert!(err.contains("Valence source tree not found"), "{err}");
        assert!(err.contains("vendored mc/valence"), "{err}");
        assert!(err.contains("--valence-repo/VALENCE_REPO"), "{err}");
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
        assert!(err.contains("vendored mc/stevenarella"), "{err}");
        assert!(err.contains("--client-dir/CLIENT_DIR"), "{err}");
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

    #[test]
    fn smoke_receipt_records_cairn_contract_and_octet_surface() {
        let mut cfg = test_config(
            &[
                "--server-backend=paper",
                "--receipt",
                "/tmp/receipt.json",
                "--client-dir",
                "/tmp/stevenarella",
            ],
            &[],
        )
        .expect("receipt config parses");
        cfg.server_port = 25566;
        let client = Some(ClientRunEvidence {
            log_path: Some(PathBuf::from("/tmp/client.log")),
            log_paths: vec![PathBuf::from("/tmp/client.log")],
            usernames: vec!["compatbot".to_string()],
            exit_code: Some(124),
            classification: "timeout-success-evidence",
            matched_success_pattern: Some("Detected server protocol version".to_string()),
            scenario: Some(evaluate_scenario(
                Scenario::Smoke,
                "Detected server protocol version",
            )),
            server_scenario: Some(evaluate_server_scenario(Scenario::Smoke, "", "compatbot")),
            projectile_damage_causality: None,
            mcp_control: None,
            frame_artifacts: None,
        });

        let json = smoke_receipt_json(&cfg, Ok(&client));
        let receipt = structured_receipt_from_text(&json).expect("smoke receipt validates");
        let contract = json_object_slice(&json, "contract").expect("contract object");
        let scenario = json_object_slice(&json, "scenario").expect("scenario object");
        let server = json_object_slice(&json, "server").expect("server object");
        let correlation =
            json_object_slice(server, "client_server_correlation").expect("correlation object");
        let client_object = json_object_slice(&json, "client").expect("client object");
        let triage = json_object_slice(&json, "triage").expect("triage object");

        assert_eq!(receipt.schema, RECEIPT_SCHEMA_V2);
        assert_eq!(
            json_string_field(contract, "cairn_contract").expect("cairn contract"),
            RECEIPT_SCHEMA_V2
        );
        assert_eq!(
            json_string_field(contract, "octet_producer_surface").expect("octet surface"),
            "tools/mc-compat-runner/src/main.rs"
        );
        assert_eq!(
            receipt.client_classification.as_deref(),
            Some("timeout-success-evidence")
        );
        assert_eq!(
            receipt.matched_success_pattern.as_deref(),
            Some("Detected server protocol version")
        );
        assert_eq!(receipt.scenario_name, "smoke");
        assert_eq!(
            json_optional_string_array_field(scenario, "observed_milestones")
                .expect("observed milestones"),
            Some(vec!["protocol_detected".to_string()])
        );
        assert!(json_bool_field(scenario, "passed").expect("scenario passed"));
        assert!(json_bool_field(correlation, "passed").expect("correlation passed"));
        assert_eq!(
            json_optional_string_array_field(client_object, "usernames").expect("usernames"),
            Some(vec!["compatbot".to_string()])
        );
        assert_eq!(
            json_optional_string_array_field(client_object, "log_paths").expect("log paths"),
            Some(vec!["/tmp/client.log".to_string()])
        );
        assert_eq!(
            json_optional_string_field(triage, "suggested_boundary").expect("triage boundary"),
            Some("none".to_string())
        );
        assert!(!receipt.wayland_socket_inherited);
    }

    #[test]
    fn scenario_receipt_records_actionable_failure_triage() {
        let mut cfg = test_config(
            &[
                "--server-backend=valence",
                "--scenario=flag-score-repeat",
                "--receipt=/tmp/receipt.json",
                "--client-dir=/tmp/stevenarella",
            ],
            &[],
        )
        .expect("receipt config parses");
        cfg.valence_log = PathBuf::from("/tmp/valence.log");
        let client = Some(ClientRunEvidence {
            log_path: Some(PathBuf::from("/tmp/client.log")),
            log_paths: vec![PathBuf::from("/tmp/client.log")],
            usernames: vec!["compatbot".to_string()],
            exit_code: Some(124),
            classification: "failure-missing-scenario-evidence",
            matched_success_pattern: None,
            scenario: Some(evaluate_scenario(
                Scenario::FlagScoreRepeat,
                "Detected server protocol version 763\njoin_game\nUnexpectedEof\n",
            )),
            server_scenario: Some(evaluate_server_scenario(
                Scenario::FlagScoreRepeat,
                "compatbot joined\n",
                "compatbot",
            )),
            projectile_damage_causality: None,
            mcp_control: None,
            frame_artifacts: None,
        });

        let json = smoke_receipt_json(&cfg, Ok(&client));

        assert!(
            json.contains("\"first_missing_client_milestone\": \"render_tick\""),
            "{json}"
        );
        assert!(
            json.contains("\"first_missing_server_milestone\": \"server_flag_or_score\""),
            "{json}"
        );
        assert!(
            json.contains("\"first_forbidden_pattern\": \"unexpected_eof\""),
            "{json}"
        );
        assert!(
            json.contains("\"first_forbidden_source\": \"client\""),
            "{json}"
        );
        assert!(
            json.contains("\"suggested_boundary\": \"protocol-runtime\""),
            "{json}"
        );
        assert!(
            json.contains("\"client_log_paths\": [\"/tmp/client.log\"]"),
            "{json}"
        );
        assert!(
            json.contains("\"server_log_path\": \"/tmp/valence.log\""),
            "{json}"
        );
    }

    #[test]
    fn failed_preflight_receipt_triages_before_client_evidence() {
        let cfg =
            test_config(&["--receipt=/tmp/receipt.json"], &[]).expect("receipt config parses");
        let err = "server status probe failed".to_string();

        let json = smoke_receipt_json(&cfg, Err(&err));

        assert!(
            json.contains("\"first_missing_client_milestone\": \"protocol_detected\""),
            "{json}"
        );
        assert!(
            json.contains("\"suggested_boundary\": \"preflight-or-server-startup\""),
            "{json}"
        );
    }

    fn receipt_fixture(backend: &str, protocol: u32, port: u16) -> String {
        receipt_fixture_with_classification(backend, protocol, port, "timeout-success-evidence")
    }

    fn receipt_fixture_with_classification(
        backend: &str,
        protocol: u32,
        port: u16,
        classification: &str,
    ) -> String {
        format!(
            "{{\n  \"schema\": \"mc.compat.smoke.receipt.v1\",\n  \"status\": \"pass\",\n  \"mode\": \"run\",\n  \"dry_run\": false,\n  \"contract\": {{\n    \"claims_correctness\": false,\n    \"claims_semantic_equivalence\": false\n  }},\n  \"server\": {{\n    \"backend\": \"{backend}\",\n    \"version\": \"1.18.2\",\n    \"protocol\": {protocol},\n    \"port\": {port}\n  }},\n  \"client\": {{\n    \"headless_isolation\": {{\n      \"xvfb\": true,\n      \"x11_backend\": true,\n      \"software_gl\": true,\n      \"wayland_socket_inherited\": false\n    }},\n    \"classification\": \"{classification}\",\n    \"matched_success_pattern\": \"Detected server protocol version\"\n  }},\n  \"error\": null\n}}\n"
        )
    }

    #[test]
    fn compares_paper_and_valence_receipts() {
        let paper = read_receipt_summary_from_text(
            PathBuf::from("paper.json"),
            &receipt_fixture("paper", 758, 25566),
        )
        .expect("paper fixture parses");
        let valence = read_receipt_summary_from_text(
            PathBuf::from("valence.json"),
            &receipt_fixture("valence", 758, 25565),
        )
        .expect("valence fixture parses");

        validate_receipt_pair(&paper, &valence, DEFAULT_SERVER_PROTOCOL)
            .expect("matching receipts compare");
    }

    #[test]
    fn rejects_receipt_protocol_mismatch() {
        let paper = read_receipt_summary_from_text(
            PathBuf::from("paper.json"),
            &receipt_fixture("paper", 758, 25566),
        )
        .expect("paper fixture parses");
        let valence = read_receipt_summary_from_text(
            PathBuf::from("valence.json"),
            &receipt_fixture("valence", 759, 25565),
        )
        .expect("valence fixture parses");

        let err = validate_receipt_pair(&paper, &valence, DEFAULT_SERVER_PROTOCOL).unwrap_err();
        assert!(err.contains("receipt protocol mismatch"), "{err}");
    }

    #[test]
    fn compares_protocol_763_matrix_receipts_when_configured() {
        const PROTOCOL_763: u32 = 763;
        let paper = read_receipt_summary_from_text(
            PathBuf::from("paper.json"),
            &receipt_fixture("paper", PROTOCOL_763, 25566),
        )
        .expect("paper fixture parses");
        let valence = read_receipt_summary_from_text(
            PathBuf::from("valence.json"),
            &receipt_fixture("valence", PROTOCOL_763, 25565),
        )
        .expect("valence fixture parses");

        validate_receipt_pair(&paper, &valence, PROTOCOL_763)
            .expect("configured protocol receipts compare");
    }

    #[test]
    fn compares_reconnect_sequence_receipts_with_multi_client_classification() {
        const PROTOCOL_763: u32 = 763;
        let paper = read_receipt_summary_from_text(
            PathBuf::from("paper.json"),
            &receipt_fixture_with_classification(
                "paper",
                PROTOCOL_763,
                25566,
                "multi-client-load-evidence",
            ),
        )
        .expect("paper fixture parses");
        let valence = read_receipt_summary_from_text(
            PathBuf::from("valence.json"),
            &receipt_fixture_with_classification(
                "valence",
                PROTOCOL_763,
                25565,
                "multi-client-load-evidence",
            ),
        )
        .expect("valence fixture parses");

        validate_receipt_pair(&paper, &valence, PROTOCOL_763)
            .expect("reconnect sequence receipts compare");
    }

    #[test]
    fn receipt_summary_mutations_fail_closed() {
        let missing_success = read_receipt_summary_from_text(
            PathBuf::from("missing-success.json"),
            &receipt_fixture("paper", DEFAULT_SERVER_PROTOCOL, 25566).replace(
                "\"matched_success_pattern\": \"Detected server protocol version\"",
                "\"matched_success_pattern\": null",
            ),
        )
        .expect("missing success fixture parses");
        let err = validate_receipt_summary(&missing_success).unwrap_err();
        assert!(
            err.contains("missing matched client success pattern"),
            "{err}"
        );

        let bad_headless = read_receipt_summary_from_text(
            PathBuf::from("bad-headless.json"),
            &receipt_fixture("paper", DEFAULT_SERVER_PROTOCOL, 25566).replace(
                "\"wayland_socket_inherited\": false",
                "\"wayland_socket_inherited\": true",
            ),
        )
        .expect("bad headless fixture parses");
        let err = validate_receipt_summary(&bad_headless).unwrap_err();
        assert!(err.contains("headless isolation"), "{err}");

        let failed_status = read_receipt_summary_from_text(
            PathBuf::from("failed-status.json"),
            &receipt_fixture("paper", DEFAULT_SERVER_PROTOCOL, 25566)
                .replace("\"status\": \"pass\"", "\"status\": \"fail\""),
        )
        .expect("failed status fixture parses");
        let err = validate_receipt_summary(&failed_status).unwrap_err();
        assert!(err.contains("did not pass"), "{err}");

        let unsupported_classification = read_receipt_summary_from_text(
            PathBuf::from("unsupported-classification.json"),
            &receipt_fixture_with_classification(
                "paper",
                DEFAULT_SERVER_PROTOCOL,
                25566,
                "unchecked-live-claim",
            ),
        )
        .expect("unsupported classification fixture parses");
        let err = validate_receipt_summary(&unsupported_classification).unwrap_err();
        assert!(err.contains("unsupported client classification"), "{err}");
    }

    #[test]
    fn rejects_receipts_that_do_not_match_configured_protocol() {
        const PROTOCOL_763: u32 = 763;
        let paper = read_receipt_summary_from_text(
            PathBuf::from("paper.json"),
            &receipt_fixture("paper", PROTOCOL_763, 25566),
        )
        .expect("paper fixture parses");
        let valence = read_receipt_summary_from_text(
            PathBuf::from("valence.json"),
            &receipt_fixture("valence", PROTOCOL_763, 25565),
        )
        .expect("valence fixture parses");

        let err = validate_receipt_pair(&paper, &valence, DEFAULT_SERVER_PROTOCOL).unwrap_err();
        assert!(
            err.contains(&format!(
                "expected protocol {DEFAULT_SERVER_PROTOCOL}, got {PROTOCOL_763}"
            )),
            "{err}"
        );
    }

    #[test]
    fn structured_receipt_schema_parses_representative_shapes() {
        let dry_cfg = test_config(&["--scenario=survival-break-place-pickup"], &[])
            .expect("dry-run config parses");
        let dry_json = smoke_receipt_json(&dry_cfg, Ok(&None));
        let dry_receipt = structured_receipt_from_text(&dry_json).expect("dry receipt validates");
        assert_eq!(dry_receipt.scenario_name, "survival-break-place-pickup");
        assert!(!dry_receipt.typed_event.selected);

        let events = typed_event_fixture();
        let timeline = normalize_typed_event_timeline(&events);
        let typed_artifact = TypedEventOracleArtifact {
            event_log_path: PathBuf::from("/tmp/mc-compat.typed-events.log"),
            timeline_blake3: typed_event_timeline_blake3(&timeline),
            event_count: events.len(),
            contributes_to_pass_fail: true,
        };
        let typed_json = typed_event_oracle_receipt_json(Some(&typed_artifact));
        let typed_receipt =
            parse_structured_typed_event_receipt(&typed_json).expect("typed-event receipt parses");
        validate_structured_typed_event_receipt(&typed_receipt)
            .expect("typed-event receipt validates");
        assert!(typed_receipt.selected);

        let mcp_cfg = test_config(&["--scenario", MCP_CONTROLLED_SMOKE_SCENARIO], &[])
            .expect("mcp config parses");
        let mcp_evidence = mcp_controlled_dry_run_evidence(&mcp_cfg);
        let mcp_json = smoke_receipt_json(&mcp_cfg, Ok(&Some(mcp_evidence)));
        let mcp_receipt = structured_receipt_from_text(&mcp_json).expect("mcp receipt validates");
        assert!(mcp_receipt.mcp_control.selected);

        let armor_cfg = test_config(
            &["--scenario", "armor-loadout-enchantment-status-matrix"],
            &[],
        )
        .expect("armor matrix config parses");
        let scenario = evaluate_scenario(
            Scenario::ArmorLoadoutEnchantmentStatusMatrix,
            "mc_compat_combat_client_count=2\nDetected server protocol version 763\njoin_game\nrender_tick_with_player\nYou are on team RED!\nYou are on team BLUE!\nremote_player_spawn\ninventory_probe_set_slot\ncombat_probe_attack_sent\nupdate_health health=18.0\n",
        );
        let server = evaluate_server_scenario(
            Scenario::ArmorLoadoutEnchantmentStatusMatrix,
            "compatbota joined\ncompatbotb joined\nMC-COMPAT-MILESTONE armor_equipment_state username=compatbotb slot=chest item=DiamondChestplate source=team_inventory_setup\nMC-COMPAT-MILESTONE combat_damage attacker=compatbota victim=compatbotb damage=2.0 victim_health_before=20.0 victim_health_after=18.0 attacker_item=WoodenSword\nMC-COMPAT-MILESTONE combat_armor_mitigation attacker=compatbota victim=compatbotb base_damage=4.0 mitigation=2.0 final_damage=2.0 chest_item=DiamondChestplate victim_health_before=20.0 victim_health_after=18.0\n",
            "compatbot",
        );
        let matrix =
            evaluate_armor_loadout_enchantment_status_matrix(&armor_cfg, &scenario, &server);
        let matrix_json = render_armor_loadout_enchantment_status_matrix_json(&matrix);
        let matrix_receipt = parse_structured_reference_matrix_receipt(&matrix_json)
            .expect("reference matrix receipt parses");
        validate_structured_reference_matrix_receipt(&matrix_receipt)
            .expect("reference matrix receipt validates");
        assert!(matrix_receipt.selected);
    }

    #[test]
    fn structured_receipt_schema_negative_fixtures_fail_closed() {
        let cfg = test_config(&["--scenario=survival-break-place-pickup"], &[])
            .expect("dry-run config parses");
        let json = smoke_receipt_json(&cfg, Ok(&None));

        let missing_nonclaim = json.replace(RECEIPT_REQUIRED_GAMEPLAY_NON_CLAIM, "removed_claim");
        let err = structured_receipt_from_text(&missing_nonclaim)
            .expect_err("missing nonclaim fails closed");
        assert!(err.contains("non_claim"), "{err}");

        let dirty_child = json.replacen(
            "\"git_status\": \"dry-run\",\n    \"git_dirty\": false",
            "\"git_status\": \"dirty\",\n    \"git_dirty\": true",
            1,
        );
        let err = structured_receipt_from_text(&dirty_child)
            .expect_err("dirty child revision fails closed");
        assert!(err.contains("child revision"), "{err}");

        let wrong_backend = json.replacen("\"backend\": \"valence\"", "\"backend\": \"spigot\"", 1);
        let err =
            structured_receipt_from_text(&wrong_backend).expect_err("wrong backend fails closed");
        assert!(err.contains("backend"), "{err}");

        let duplicate_status = json.replacen(
            "\"status\": \"pass\",",
            "\"status\": \"pass\",\n  \"status\": \"pass\",",
            1,
        );
        let err = parse_structured_receipt_summary(&duplicate_status)
            .expect_err("duplicate top-level field fails closed");
        assert!(err.contains("expected once"), "{err}");

        let wrong_typed_field = json.replacen("\"dry_run\": true", "\"dry_run\": \"true\"", 1);
        let err = parse_structured_receipt_summary(&wrong_typed_field)
            .expect_err("wrong typed field fails closed");
        assert!(err.contains("dry_run") || err.contains("bool"), "{err}");

        let overclaim = json.replacen(
            "\"claims_correctness\": false",
            "\"claims_correctness\": true",
            1,
        );
        let err =
            parse_structured_receipt_summary(&overclaim).expect_err("overclaim field fails closed");
        assert!(err.contains("overclaim"), "{err}");

        let events = typed_event_fixture();
        let timeline = normalize_typed_event_timeline(&events);
        let typed_artifact = TypedEventOracleArtifact {
            event_log_path: PathBuf::from("/tmp/mc-compat.typed-events.log"),
            timeline_blake3: typed_event_timeline_blake3(&timeline),
            event_count: events.len(),
            contributes_to_pass_fail: true,
        };
        let typed_json = typed_event_oracle_receipt_json(Some(&typed_artifact));
        let zero_count = typed_json.replacen(
            &format!("\"event_count\": {}", events.len()),
            "\"event_count\": 0",
            1,
        );
        let typed_receipt = parse_structured_typed_event_receipt(&zero_count)
            .expect("zero-count typed-event receipt parses");
        let err = validate_structured_typed_event_receipt(&typed_receipt)
            .expect_err("missing typed events fail closed");
        assert!(err.contains("zero events"), "{err}");

        let frame = FrameArtifactsReceiptEvidence {
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
                blake3: "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
                    .to_string(),
                redaction: "not_reviewed".to_string(),
                includes_ui: true,
            }],
            missing_digests: Vec::new(),
            path_containment_checked: true,
            promotion_ready: true,
            non_claims: FRAME_ARTIFACT_NON_CLAIMS.to_vec(),
        };
        let frame_json = render_frame_artifacts_receipt_json(&frame);
        let escaped_frame_path = frame_json.replace(
            "docs/evidence/mcp-controlled-smoke-frames/latest-frame.png",
            "../escape.png",
        );
        let frame_receipt = parse_structured_frame_artifact_receipt(&escaped_frame_path)
            .expect("escaped frame receipt parses");
        let err = validate_structured_frame_artifact_receipt(&frame_receipt)
            .expect_err("escaped artifact path fails closed");
        assert!(err.contains("escapes"), "{err}");
    }

    #[test]
    fn smoke_receipt_records_failures_without_success_claims() {
        let cfg =
            test_config(&["--receipt=/tmp/receipt.json"], &[]).expect("receipt config parses");
        let err = "server status probe failed".to_string();

        let json = smoke_receipt_json(&cfg, Err(&err));

        assert!(json.contains("\"status\": \"fail\""), "{json}");
        assert!(json.contains("\"classification\": null"), "{json}");
        assert!(
            json.contains("\"error\": \"server status probe failed\""),
            "{json}"
        );
        assert!(json.contains("\"claims_correctness\": false"), "{json}");
        assert!(
            json.contains("\"claims_semantic_equivalence\": false"),
            "{json}"
        );
    }
}
