use super::scenario_contracts_generated;
use crate::format;
use crate::protocol::packet;
use crate::shared::Position;
use steven_protocol::item;

// CTF probe configuration and pure parsing/observation helpers.
pub(crate) const DEFAULT_FLAG_PROBE_REPEAT_TARGET: u32 = 1;
pub(crate) const MAX_FLAG_PROBE_REPEAT_TARGET: u32 = 8;
pub(crate) const FLAG_PROBE_FIRST_TICK: u32 = 560;
pub(crate) const CTF_SCORE_LIMIT_CLIENT_ENV: &str =
    scenario_contracts_generated::MC_COMPAT_SCORE_LIMIT_PROBE;
pub(crate) const CTF_SCORE_LIMIT_CLIENT_TARGET_SCORE: &str = "RED: 2";
pub(crate) const CTF_SCORE_LIMIT_CLIENT_OPPOSING_SCORE: &str = "BLUE: 0";
pub(crate) const CTF_SCORE_LIMIT_CLIENT_WIN_TEAM: &str = "red";
pub(crate) const CTF_SCORE_LIMIT_CLIENT_END_STATE: &str = "winner_declared";
pub(crate) const FLAG_PROBE_CYCLE_TICKS: u32 = 220;

pub(crate) fn parse_flag_probe_repeat_target(value: Option<&str>) -> u32 {
    value
        .and_then(|raw| raw.trim().parse::<u32>().ok())
        .filter(|target| *target > 0)
        .map(|target| target.min(MAX_FLAG_PROBE_REPEAT_TARGET))
        .unwrap_or(DEFAULT_FLAG_PROBE_REPEAT_TARGET)
}

pub(crate) fn ctf_score_limit_chat_matches(rendered: &str) -> bool {
    rendered.contains(CTF_SCORE_LIMIT_CLIENT_TARGET_SCORE)
        && rendered.contains(CTF_SCORE_LIMIT_CLIENT_OPPOSING_SCORE)
}

// Active/combat probe schedule core. Shell code translates decisions into ECS mutations,
// packet writes, and milestone logs.
pub(crate) const ACTIVE_PROBE_INPUT_START_TICK: u32 = 1;
pub(crate) const STATIONARY_COMBAT_PROBE_ENV: &str =
    scenario_contracts_generated::MC_COMPAT_STATIONARY_COMBAT_PROBE;
pub(crate) const ACTIVE_PROBE_JUMP_RELEASE_TICK: u32 = 18;
pub(crate) const ACTIVE_PROBE_TURN_TICK: u32 = 180;
pub(crate) const ACTIVE_PROBE_STOP_TICK: u32 = 300;
pub(crate) const COMBAT_PROBE_ROLE_ATTACKER: &str = "attacker";
pub(crate) const COMBAT_ATTACK_MOVE_TICK: u32 = 620;
pub(crate) const COMBAT_ATTACK_HOLD_START_TICK: u32 = 621;
pub(crate) const COMBAT_ATTACK_HOLD_END_TICK: u32 = 980;
pub(crate) const COMBAT_REGULAR_ATTACK_START_TICK: u32 = 900;
pub(crate) const COMBAT_FLAG_CARRIER_ATTACK_START_TICK: u32 = 980;
pub(crate) const COMBAT_ATTACK_INTERVAL_TICKS: u32 = 20;
pub(crate) const COMBAT_REGULAR_ATTACK_LIMIT: u32 = 10;
pub(crate) const COMBAT_FLAG_CARRIER_ATTACK_LIMIT: u32 = 20;
pub(crate) const COMBAT_REGULAR_ATTACK_X: f64 = 38.0;
pub(crate) const COMBAT_FLAG_CARRIER_ATTACK_X: f64 = -38.0;
pub(crate) const COMBAT_ATTACK_Z: f64 = 0.0;
pub(crate) const COMBAT_ATTACK_Y: f64 = 65.0;
pub(crate) const COMBAT_REGULAR_ATTACK_YAW: f32 = -90.0;
pub(crate) const COMBAT_FLAG_CARRIER_ATTACK_YAW: f32 = 90.0;
pub(crate) const COMBAT_ATTACK_PITCH: f32 = 0.0;
pub(crate) const COMBAT_REGULAR_ATTACK_LABEL: &str = "blue_spawn";
pub(crate) const COMBAT_FLAG_CARRIER_ATTACK_LABEL: &str = "red_flag";
pub(crate) const COMBAT_ATTACK_ENTITY_TYPE: i32 = 1;
pub(crate) const COMBAT_ATTACK_HAND: i32 = 0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct CombatProbePlan {
    pub(crate) attack_x: f64,
    pub(crate) attack_z: f64,
    pub(crate) attack_label: &'static str,
    pub(crate) attack_yaw: f32,
    pub(crate) attack_start_tick: u32,
    pub(crate) attack_limit: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum CombatMovementAction {
    MoveNear,
    HoldPosition,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct CombatProbeDecision {
    pub(crate) plan: CombatProbePlan,
    pub(crate) movement: Option<CombatMovementAction>,
    pub(crate) should_attack: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct CombatProbeInput {
    pub(crate) enabled: bool,
    pub(crate) attacker_role: bool,
    pub(crate) flag_carrier_death_probe: bool,
    pub(crate) active_ticks: u32,
    pub(crate) attacks_sent: u32,
}

pub(crate) fn combat_probe_plan(flag_carrier_death_probe: bool) -> CombatProbePlan {
    if flag_carrier_death_probe {
        CombatProbePlan {
            attack_x: COMBAT_FLAG_CARRIER_ATTACK_X,
            attack_z: COMBAT_ATTACK_Z,
            attack_label: COMBAT_FLAG_CARRIER_ATTACK_LABEL,
            attack_yaw: COMBAT_FLAG_CARRIER_ATTACK_YAW,
            attack_start_tick: COMBAT_FLAG_CARRIER_ATTACK_START_TICK,
            attack_limit: COMBAT_FLAG_CARRIER_ATTACK_LIMIT,
        }
    } else {
        CombatProbePlan {
            attack_x: COMBAT_REGULAR_ATTACK_X,
            attack_z: COMBAT_ATTACK_Z,
            attack_label: COMBAT_REGULAR_ATTACK_LABEL,
            attack_yaw: COMBAT_REGULAR_ATTACK_YAW,
            attack_start_tick: COMBAT_REGULAR_ATTACK_START_TICK,
            attack_limit: COMBAT_REGULAR_ATTACK_LIMIT,
        }
    }
}

pub(crate) fn next_combat_probe_decision(input: CombatProbeInput) -> Option<CombatProbeDecision> {
    if !input.enabled || !input.attacker_role {
        return None;
    }

    let plan = combat_probe_plan(input.flag_carrier_death_probe);
    let movement = match input.active_ticks {
        COMBAT_ATTACK_MOVE_TICK => Some(CombatMovementAction::MoveNear),
        COMBAT_ATTACK_HOLD_START_TICK..=COMBAT_ATTACK_HOLD_END_TICK => {
            Some(CombatMovementAction::HoldPosition)
        }
        _ => None,
    };
    let should_attack = input.active_ticks >= plan.attack_start_tick
        && input.attacks_sent < plan.attack_limit
        && input.active_ticks % COMBAT_ATTACK_INTERVAL_TICKS == 0;

    if movement.is_none() && !should_attack {
        return None;
    }

    Some(CombatProbeDecision {
        plan,
        movement,
        should_attack,
    })
}

// Survival, inventory/window, sign, and dimension probe constants shared by pure cores
// and packet-handler shells.
pub(crate) const SURVIVAL_PROBE_POSITION_TICK: u32 = 60;
pub(crate) const SURVIVAL_PROBE_BREAK_TICK: u32 = 80;
pub(crate) const SURVIVAL_PROBE_PLACE_TICK: u32 = 120;
pub(crate) const SURVIVAL_PROBE_BREAK_X: i32 = 0;
pub(crate) const SURVIVAL_PROBE_BREAK_Y: i32 = 64;
pub(crate) const SURVIVAL_PROBE_BREAK_Z: i32 = 1;
pub(crate) const SURVIVAL_PROBE_PLACE_Y: i32 = 65;
pub(crate) const SURVIVAL_PROBE_START_DESTROY_STATUS: i32 = 0;
pub(crate) const SURVIVAL_PROBE_STOP_DESTROY_STATUS: i32 = 2;
pub(crate) const SURVIVAL_PROBE_FACE_UP: i32 = 1;
pub(crate) const SURVIVAL_PROBE_MAIN_HAND: i32 = 0;
pub(crate) const SURVIVAL_PROBE_HOTBAR_SLOT: i16 = 0;
pub(crate) const SURVIVAL_PROBE_AIR_RAW_ID: i32 = 0;
pub(crate) const SURVIVAL_PROBE_BREAK_START_SEQUENCE: i32 = 404;
pub(crate) const SURVIVAL_PROBE_BREAK_STOP_SEQUENCE: i32 = 405;
pub(crate) const SURVIVAL_PROBE_PLACE_SEQUENCE: i32 = 406;
pub(crate) const SURVIVAL_PROBE_CURSOR_CENTER: f32 = 0.5;
pub(crate) const SURVIVAL_PROBE_CURSOR_TOP: f32 = 1.0;
pub(crate) const SURVIVAL_PROBE_PLAYER_X: f64 = 0.5;
pub(crate) const SURVIVAL_PROBE_PLAYER_Y: f64 = 65.0;
pub(crate) const SURVIVAL_PROBE_PLAYER_Z: f64 = 0.5;
pub(crate) const SURVIVAL_CHEST_FIRST_SESSION: u32 = 1;
pub(crate) const SURVIVAL_CHEST_REOPEN_SESSION: u32 = 2;
pub(crate) const SURVIVAL_CHEST_POSITION_TICK: u32 = 60;
pub(crate) const SURVIVAL_CHEST_OPEN_TICK: u32 = 80;
pub(crate) const SURVIVAL_CHEST_STORE_TICK: u32 = 120;
pub(crate) const SURVIVAL_CHEST_CLOSE_TICK: u32 = 160;
pub(crate) const SURVIVAL_CHEST_X: i32 = 8;
pub(crate) const SURVIVAL_CHEST_Y: i32 = 64;
pub(crate) const SURVIVAL_CHEST_Z: i32 = 0;
pub(crate) const SURVIVAL_CHEST_PLAYER_X: f64 = 8.5;
pub(crate) const SURVIVAL_CHEST_PLAYER_Y: f64 = 65.0;
pub(crate) const SURVIVAL_CHEST_PLAYER_Z: f64 = 0.5;
pub(crate) const SURVIVAL_CHEST_FACE_UP: i32 = 1;
pub(crate) const SURVIVAL_CHEST_MAIN_HAND: i32 = 0;
pub(crate) const SURVIVAL_CHEST_SEQUENCE: i32 = 507;
pub(crate) const SURVIVAL_CHEST_WINDOW_SLOT: i16 = 0;
pub(crate) const SURVIVAL_CHEST_WINDOW_SLOT_INDEX: usize = 0;
pub(crate) const SURVIVAL_CHEST_CLICK_BUTTON: u8 = 0;
pub(crate) const SURVIVAL_CHEST_CLICK_MODE: i32 = 0;
pub(crate) const SURVIVAL_CHEST_ITEM_PROTOCOL_ID: isize = 15;
pub(crate) const SURVIVAL_CHEST_ITEM_COUNT: isize = 1;
pub(crate) const SURVIVAL_CHEST_ITEM_NAME: &str = "Dirt";
pub(crate) const SURVIVAL_CHEST_RECONNECT_SESSION_LABEL: u32 = 1;
pub(crate) const SURVIVAL_CRAFTING_PROBE_ENV: &str = "MC_COMPAT_SURVIVAL_CRAFTING_PROBE";
pub(crate) const SURVIVAL_CRAFTING_BREADTH_PROBE_ENV: &str =
    "MC_COMPAT_SURVIVAL_CRAFTING_BREADTH_PROBE";
pub(crate) const SURVIVAL_CRAFTING_BREADTH_LOG_TICK: u32 = 120;
pub(crate) const SURVIVAL_CRAFTING_POSITION_TICK: u32 = 60;
pub(crate) const SURVIVAL_CRAFTING_OPEN_TICK: u32 = 80;
pub(crate) const SURVIVAL_CRAFTING_INPUT_A_TICK: u32 = 120;
pub(crate) const SURVIVAL_CRAFTING_INPUT_B_TICK: u32 = 140;
pub(crate) const SURVIVAL_CRAFTING_COLLECT_TICK: u32 = 160;
pub(crate) const SURVIVAL_CRAFTING_TABLE_X: i32 = 4;
pub(crate) const SURVIVAL_CRAFTING_TABLE_Y: i32 = 64;
pub(crate) const SURVIVAL_CRAFTING_TABLE_Z: i32 = 0;
pub(crate) const SURVIVAL_CRAFTING_PLAYER_X: f64 = 4.5;
pub(crate) const SURVIVAL_CRAFTING_PLAYER_Y: f64 = 65.0;
pub(crate) const SURVIVAL_CRAFTING_PLAYER_Z: f64 = 0.5;
pub(crate) const SURVIVAL_CRAFTING_FACE_UP: i32 = 1;
pub(crate) const SURVIVAL_CRAFTING_MAIN_HAND: i32 = 0;
pub(crate) const SURVIVAL_CRAFTING_SEQUENCE: i32 = 608;
pub(crate) const SURVIVAL_CRAFTING_INPUT_A_SLOT: i16 = 1;
pub(crate) const SURVIVAL_CRAFTING_INPUT_B_SLOT: i16 = 4;
pub(crate) const SURVIVAL_CRAFTING_RESULT_SLOT: i16 = 0;
pub(crate) const SURVIVAL_CRAFTING_RESULT_INDEX: usize = 0;
pub(crate) const SURVIVAL_CRAFTING_INVENTORY_SLOT: i16 = 36;
pub(crate) const SURVIVAL_CRAFTING_INVENTORY_INDEX: usize = 36;
pub(crate) const SURVIVAL_CRAFTING_OPEN_INVENTORY_MIRROR_SLOT: i16 = 37;
pub(crate) const SURVIVAL_CRAFTING_OPEN_INVENTORY_MIRROR_INDEX: usize = 37;
pub(crate) const SURVIVAL_CRAFTING_INPUT_ITEM_ID: isize = 23;
pub(crate) const SURVIVAL_CRAFTING_RESULT_ITEM_ID: isize = 807;
pub(crate) const SURVIVAL_CRAFTING_INPUT_ITEM_NAME: &str = "OakPlanks";
pub(crate) const SURVIVAL_CRAFTING_RESULT_ITEM_NAME: &str = "Stick";
pub(crate) const SURVIVAL_CRAFTING_RECIPE: &str = "minecraft:stick";
pub(crate) const SURVIVAL_FURNACE_PROBE_ENV: &str = "MC_COMPAT_SURVIVAL_FURNACE_PROBE";
pub(crate) const SURVIVAL_FURNACE_SMELTING_BREADTH_PROBE_ENV: &str =
    "MC_COMPAT_SURVIVAL_FURNACE_SMELTING_BREADTH_PROBE";
pub(crate) const SURVIVAL_FURNACE_SESSION_ENV: &str = "MC_COMPAT_SURVIVAL_FURNACE_SESSION";
pub(crate) const SURVIVAL_FURNACE_FIRST_SESSION: u32 = 1;
pub(crate) const SURVIVAL_FURNACE_REOPEN_SESSION: u32 = 2;
pub(crate) const SURVIVAL_FURNACE_POSITION_TICK: u32 = 60;
pub(crate) const SURVIVAL_FURNACE_OPEN_TICK: u32 = 80;
pub(crate) const SURVIVAL_FURNACE_INPUT_TICK: u32 = 120;
pub(crate) const SURVIVAL_FURNACE_FUEL_TICK: u32 = 140;
pub(crate) const SURVIVAL_FURNACE_COLLECT_TICK: u32 = 180;
pub(crate) const SURVIVAL_FURNACE_INVALID_FUEL_TICK: u32 = 210;
pub(crate) const SURVIVAL_FURNACE_CLOSE_TICK: u32 = 220;
pub(crate) const SURVIVAL_FURNACE_X: i32 = 12;
pub(crate) const SURVIVAL_FURNACE_Y: i32 = 64;
pub(crate) const SURVIVAL_FURNACE_Z: i32 = 0;
pub(crate) const SURVIVAL_FURNACE_PLAYER_X: f64 = 12.5;
pub(crate) const SURVIVAL_FURNACE_PLAYER_Y: f64 = 65.0;
pub(crate) const SURVIVAL_FURNACE_PLAYER_Z: f64 = 0.5;
pub(crate) const SURVIVAL_FURNACE_FACE_UP: i32 = 1;
pub(crate) const SURVIVAL_FURNACE_MAIN_HAND: i32 = 0;
pub(crate) const SURVIVAL_FURNACE_SEQUENCE: i32 = 709;
pub(crate) const SURVIVAL_FURNACE_INPUT_SLOT: i16 = 0;
pub(crate) const SURVIVAL_FURNACE_FUEL_SLOT: i16 = 1;
pub(crate) const SURVIVAL_FURNACE_OUTPUT_SLOT: i16 = 2;
pub(crate) const SURVIVAL_FURNACE_FUEL_INDEX: usize = 1;
pub(crate) const SURVIVAL_FURNACE_OUTPUT_INDEX: usize = 2;
pub(crate) const SURVIVAL_FURNACE_INVENTORY_SLOT: i16 = 36;
pub(crate) const SURVIVAL_FURNACE_INVENTORY_INDEX: usize = 36;
pub(crate) const SURVIVAL_FURNACE_OPEN_INVENTORY_MIRROR_SLOT: i16 = 30;
pub(crate) const SURVIVAL_FURNACE_OPEN_INVENTORY_MIRROR_INDEX: usize = 30;
pub(crate) const SURVIVAL_FURNACE_INPUT_ITEM_ID: isize = 769;
pub(crate) const SURVIVAL_FURNACE_FUEL_ITEM_ID: isize = 762;
pub(crate) const SURVIVAL_FURNACE_OUTPUT_ITEM_ID: isize = 770;
pub(crate) const SURVIVAL_FURNACE_INPUT_ITEM_NAME: &str = "RawIron";
pub(crate) const SURVIVAL_FURNACE_INVALID_FUEL_OUTCOME: &str = "no_burn";
pub(crate) const SURVIVAL_FURNACE_FUEL_ITEM_NAME: &str = "Coal";
pub(crate) const SURVIVAL_FURNACE_OUTPUT_ITEM_NAME: &str = "IronIngot";
pub(crate) const SURVIVAL_FURNACE_ITEM_COUNT: isize = 1;
pub(crate) const SURVIVAL_FURNACE_CLICK_BUTTON: u8 = 0;
pub(crate) const SURVIVAL_FURNACE_CLICK_MODE: i32 = 0;
pub(crate) const SURVIVAL_FURNACE_RECONNECT_SESSION_LABEL: u32 = 1;
pub(crate) const SURVIVAL_HUNGER_FOOD_PROBE_ENV: &str = "MC_COMPAT_SURVIVAL_HUNGER_FOOD_PROBE";
pub(crate) const SURVIVAL_HUNGER_FOOD_USE_TICK: u32 = 120;
pub(crate) const SURVIVAL_HUNGER_FOOD_HOTBAR_SLOT: i16 = 0;
pub(crate) const SURVIVAL_HUNGER_FOOD_INVENTORY_SLOT: i16 = 36;
pub(crate) const SURVIVAL_HUNGER_FOOD_INVENTORY_INDEX: usize = 36;
pub(crate) const SURVIVAL_HUNGER_FOOD_ITEM_ID: isize = 815;
pub(crate) const SURVIVAL_HUNGER_FOOD_ITEM_NAME: &str = "Bread";
pub(crate) const SURVIVAL_HUNGER_FOOD_ITEM_COUNT_BEFORE: isize = 1;
pub(crate) const SURVIVAL_HUNGER_FOOD_ITEM_COUNT_AFTER: isize = 0;
pub(crate) const SURVIVAL_HUNGER_FOOD_PRE_HEALTH: f32 = 20.0;
pub(crate) const SURVIVAL_HUNGER_FOOD_PRE_FOOD: i32 = 15;
pub(crate) const SURVIVAL_HUNGER_FOOD_PRE_SATURATION: f32 = 0.0;
pub(crate) const SURVIVAL_HUNGER_FOOD_POST_HEALTH: f32 = 20.0;
pub(crate) const SURVIVAL_HUNGER_FOOD_POST_FOOD: i32 = 20;
pub(crate) const SURVIVAL_HUNGER_FOOD_POST_SATURATION: f32 = 6.0;
pub(crate) const SURVIVAL_HUNGER_HEALTH_PROBE_ENV: &str = "MC_COMPAT_SURVIVAL_HUNGER_HEALTH_PROBE";
pub(crate) const SURVIVAL_HUNGER_HEALTH_PRE_HEALTH: f32 = 18.0;
pub(crate) const SURVIVAL_HUNGER_HEALTH_PRE_FOOD: i32 = 15;
pub(crate) const SURVIVAL_HUNGER_HEALTH_PRE_SATURATION: f32 = 0.0;
pub(crate) const SURVIVAL_HUNGER_HEALTH_POST_HEALTH: f32 = 20.0;
pub(crate) const SURVIVAL_HUNGER_HEALTH_POST_FOOD: i32 = 20;
pub(crate) const SURVIVAL_HUNGER_HEALTH_POST_SATURATION: f32 = 6.0;
pub(crate) const SURVIVAL_HUNGER_FOOD_FLOAT_TOLERANCE: f32 = 0.01;
pub(crate) const SURVIVAL_HUNGER_FOOD_MAIN_HAND: i32 = 0;
pub(crate) const SURVIVAL_HUNGER_FOOD_USE_SEQUENCE: i32 = 810;
pub(crate) const SURVIVAL_MOB_DROP_PROBE_ENV: &str = "MC_COMPAT_SURVIVAL_MOB_DROP_PROBE";
pub(crate) const SURVIVAL_MOB_DROP_POSITION_TICK: u32 = 60;
pub(crate) const SURVIVAL_MOB_DROP_ATTACK_TICK: u32 = 140;
pub(crate) const SURVIVAL_MOB_DROP_PLAYER_X: f64 = 16.5;
pub(crate) const SURVIVAL_MOB_DROP_PLAYER_Y: f64 = 65.0;
pub(crate) const SURVIVAL_MOB_DROP_PLAYER_Z: f64 = 0.5;
pub(crate) const SURVIVAL_MOB_DROP_TARGET_X: f64 = 16.5;
pub(crate) const SURVIVAL_MOB_DROP_TARGET_Y: f64 = 65.0;
pub(crate) const SURVIVAL_MOB_DROP_TARGET_Z: f64 = 2.5;
pub(crate) const SURVIVAL_MOB_DROP_TARGET_YAW: f32 = 0.0;
pub(crate) const SURVIVAL_MOB_DROP_TARGET_PITCH: f32 = 0.0;
pub(crate) const SURVIVAL_MOB_DROP_POSITION_TOLERANCE: f64 = 0.75;
pub(crate) const SURVIVAL_MOB_DROP_MOB_NAME: &str = "IronGolem";
pub(crate) const SURVIVAL_MOB_DROP_ITEM_NAME: &str = "IronIngot";
pub(crate) const SURVIVAL_MOB_DROP_ITEM_ID: isize = SURVIVAL_FURNACE_OUTPUT_ITEM_ID;
pub(crate) const SURVIVAL_MOB_DROP_PROTOCOL_758_ITEM_ID: isize = 692;
pub(crate) const SURVIVAL_MOB_DROP_ITEM_COUNT: isize = SURVIVAL_FURNACE_ITEM_COUNT;
pub(crate) const SURVIVAL_MOB_DROP_INVENTORY_SLOT: i16 = 36;
pub(crate) const SURVIVAL_MOB_DROP_INVENTORY_INDEX: usize = 36;
pub(crate) const SURVIVAL_MOB_DROP_ATTACK_TYPE: i32 = 1;
pub(crate) const SURVIVAL_MOB_DROP_MAIN_HAND: i32 = 0;
pub(crate) const SURVIVAL_MOB_AI_LOOT_PROBE_ENV: &str = "MC_COMPAT_SURVIVAL_MOB_AI_LOOT_PROBE";
pub(crate) const SURVIVAL_MOB_AI_LOOT_LOG_TICK: u32 = 120;
pub(crate) const SURVIVAL_REDSTONE_TOGGLE_PROBE_ENV: &str =
    "MC_COMPAT_SURVIVAL_REDSTONE_TOGGLE_PROBE";
pub(crate) const SURVIVAL_REDSTONE_TOGGLE_POSITION_TICK: u32 = 60;
pub(crate) const SURVIVAL_REDSTONE_TOGGLE_INPUT_TICK: u32 = 100;
pub(crate) const SURVIVAL_REDSTONE_TOGGLE_RETURN_TICK: u32 = 150;
pub(crate) const SURVIVAL_REDSTONE_TOGGLE_PLAYER_X: f64 = 20.5;
pub(crate) const SURVIVAL_REDSTONE_TOGGLE_PLAYER_Y: f64 = 65.0;
pub(crate) const SURVIVAL_REDSTONE_TOGGLE_PLAYER_Z: f64 = -1.5;
pub(crate) const SURVIVAL_REDSTONE_TOGGLE_TARGET_YAW: f32 = 0.0;
pub(crate) const SURVIVAL_REDSTONE_TOGGLE_TARGET_PITCH: f32 = 30.0;
pub(crate) const SURVIVAL_REDSTONE_TOGGLE_CONTROL_NAME: &str = "Lever";
pub(crate) const SURVIVAL_REDSTONE_TOGGLE_OUTPUT_NAME: &str = "RedstoneLamp";
pub(crate) const SURVIVAL_REDSTONE_TOGGLE_CONTROL_X: i32 = 20;
pub(crate) const SURVIVAL_REDSTONE_TOGGLE_CONTROL_Y: i32 = 64;
pub(crate) const SURVIVAL_REDSTONE_TOGGLE_CONTROL_Z: i32 = 0;
pub(crate) const SURVIVAL_REDSTONE_TOGGLE_OUTPUT_X: i32 = 21;
pub(crate) const SURVIVAL_REDSTONE_TOGGLE_OUTPUT_Y: i32 = 64;
pub(crate) const SURVIVAL_REDSTONE_TOGGLE_OUTPUT_Z: i32 = 0;
pub(crate) const SURVIVAL_REDSTONE_TOGGLE_FACE_UP: i32 = 1;
pub(crate) const SURVIVAL_REDSTONE_TOGGLE_MAIN_HAND: i32 = 0;
pub(crate) const SURVIVAL_REDSTONE_TOGGLE_CURSOR_CENTER: f32 = 0.5;
pub(crate) const SURVIVAL_REDSTONE_TOGGLE_CURSOR_TOP: f32 = 1.0;
pub(crate) const SURVIVAL_REDSTONE_TOGGLE_ON_SEQUENCE: i32 = 911;
pub(crate) const SURVIVAL_REDSTONE_TOGGLE_OFF_SEQUENCE: i32 = 912;
pub(crate) const SURVIVAL_REDSTONE_CIRCUIT_PROBE_ENV: &str =
    "MC_COMPAT_SURVIVAL_REDSTONE_CIRCUIT_PROBE";
pub(crate) const SURVIVAL_REDSTONE_CIRCUIT_LOG_TICK: u32 = 120;
pub(crate) const SURVIVAL_WORLD_PERSISTENCE_PROBE_ENV: &str =
    "MC_COMPAT_SURVIVAL_WORLD_PERSISTENCE_PROBE";
pub(crate) const SURVIVAL_WORLD_PERSISTENCE_SESSION_ENV: &str =
    "MC_COMPAT_SURVIVAL_WORLD_PERSISTENCE_SESSION";
pub(crate) const SURVIVAL_WORLD_PERSISTENCE_FIRST_SESSION: u32 = 1;
pub(crate) const SURVIVAL_WORLD_PERSISTENCE_RESTART_SESSION: u32 = 2;
pub(crate) const SURVIVAL_WORLD_PERSISTENCE_POSITION_TICK: u32 = 60;
pub(crate) const SURVIVAL_WORLD_PERSISTENCE_MUTATION_TICK: u32 = 100;
pub(crate) const SURVIVAL_WORLD_PERSISTENCE_PLAYER_X: f64 = 24.5;
pub(crate) const SURVIVAL_WORLD_PERSISTENCE_PLAYER_Y: f64 = 65.0;
pub(crate) const SURVIVAL_WORLD_PERSISTENCE_PLAYER_Z: f64 = -1.5;
pub(crate) const SURVIVAL_WORLD_PERSISTENCE_TARGET_YAW: f32 = 0.0;
pub(crate) const SURVIVAL_WORLD_PERSISTENCE_TARGET_PITCH: f32 = 30.0;
pub(crate) const SURVIVAL_WORLD_PERSISTENCE_BLOCK_NAME: &str = "Dirt";
pub(crate) const SURVIVAL_WORLD_PERSISTENCE_X: i32 = 24;
pub(crate) const SURVIVAL_WORLD_PERSISTENCE_Y: i32 = 64;
pub(crate) const SURVIVAL_WORLD_PERSISTENCE_Z: i32 = 0;
pub(crate) const SURVIVAL_WORLD_PERSISTENCE_PLACE_BASE_Y: i32 = 63;
pub(crate) const SURVIVAL_WORLD_PERSISTENCE_FACE_UP: i32 = 1;
pub(crate) const SURVIVAL_WORLD_PERSISTENCE_MAIN_HAND: i32 = 0;
pub(crate) const SURVIVAL_WORLD_PERSISTENCE_HOTBAR_SLOT: i16 = 0;
pub(crate) const SURVIVAL_WORLD_PERSISTENCE_INVENTORY_SLOT: i16 = 36;
pub(crate) const SURVIVAL_WORLD_PERSISTENCE_CURSOR_CENTER: f32 = 0.5;
pub(crate) const SURVIVAL_WORLD_PERSISTENCE_CURSOR_TOP: f32 = 1.0;
pub(crate) const SURVIVAL_WORLD_PERSISTENCE_SEQUENCE: i32 = 933;
pub(crate) const SURVIVAL_BLOCK_ENTITY_PROBE_ENV: &str = "MC_COMPAT_SURVIVAL_BLOCK_ENTITY_PROBE";
pub(crate) const SURVIVAL_BLOCK_ENTITY_SESSION_ENV: &str =
    "MC_COMPAT_SURVIVAL_BLOCK_ENTITY_SESSION";
pub(crate) const SURVIVAL_BLOCK_ENTITY_FIRST_SESSION: u32 = 1;
pub(crate) const SURVIVAL_BLOCK_ENTITY_RESTART_SESSION: u32 = 2;
pub(crate) const SURVIVAL_BLOCK_ENTITY_POSITION_TICK: u32 =
    SURVIVAL_WORLD_PERSISTENCE_POSITION_TICK;
pub(crate) const SURVIVAL_BLOCK_ENTITY_PLAYER_X: f64 = 28.5;
pub(crate) const SURVIVAL_BLOCK_ENTITY_PLAYER_Y: f64 = 65.0;
pub(crate) const SURVIVAL_BLOCK_ENTITY_PLAYER_Z: f64 = -1.5;
pub(crate) const SURVIVAL_BLOCK_ENTITY_TARGET_YAW: f32 = 0.0;
pub(crate) const SURVIVAL_BLOCK_ENTITY_TARGET_PITCH: f32 = 30.0;
pub(crate) const SURVIVAL_BLOCK_ENTITY_KIND: &str = "Sign";
pub(crate) const SURVIVAL_BLOCK_ENTITY_X: i32 = 28;
pub(crate) const SURVIVAL_BLOCK_ENTITY_Y: i32 = 64;
pub(crate) const SURVIVAL_BLOCK_ENTITY_Z: i32 = 0;
pub(crate) const SURVIVAL_BLOCK_ENTITY_TEXT_LINE_1: &str = "MC";
pub(crate) const SURVIVAL_BLOCK_ENTITY_TEXT_LINE_2: &str = "Compat";
pub(crate) const SURVIVAL_BLOCK_ENTITY_TEXT_LINE_3: &str = "Sign";
pub(crate) const SURVIVAL_BLOCK_ENTITY_TEXT_LINE_4: &str = "Persist";
pub(crate) const SURVIVAL_BLOCK_ENTITY_TEXT_PAYLOAD: &str = "MC|Compat|Sign|Persist";
pub(crate) const SURVIVAL_WORLD_MULTICHUNK_PROBE_ENV: &str =
    "MC_COMPAT_SURVIVAL_WORLD_MULTICHUNK_PROBE";
pub(crate) const SURVIVAL_WORLD_MULTICHUNK_SESSION_ENV: &str =
    "MC_COMPAT_SURVIVAL_WORLD_MULTICHUNK_SESSION";
pub(crate) const SURVIVAL_WORLD_MULTICHUNK_LOG_TICK: u32 = 120;
pub(crate) const SURVIVAL_CONTAINER_BLOCK_ENTITY_PROBE_ENV: &str =
    "MC_COMPAT_SURVIVAL_CONTAINER_BLOCK_ENTITY_PROBE";
pub(crate) const SURVIVAL_CONTAINER_BLOCK_ENTITY_LOG_TICK: u32 = 120;
pub(crate) const SURVIVAL_SIGN_EDITING_PROBE_ENV: &str = "MC_COMPAT_SURVIVAL_SIGN_EDITING_PROBE";
pub(crate) const SURVIVAL_SIGN_EDITING_LOG_TICK: u32 = 120;
pub(crate) const SIGN_LINE_COUNT: usize = 4;
pub(crate) const SIGN_LINE_INDEX_1: usize = 0;
pub(crate) const SIGN_LINE_INDEX_2: usize = 1;
pub(crate) const SIGN_LINE_INDEX_3: usize = 2;
pub(crate) const SIGN_LINE_INDEX_4: usize = 3;
pub(crate) const MODERN_SIGN_FRONT_TEXT_KEY: &str = "front_text";
pub(crate) const MODERN_SIGN_MESSAGES_KEY: &str = "messages";
pub(crate) const LEGACY_SIGN_ID: &str = "Sign";
pub(crate) const LEGACY_SIGN_BLOCK_ENTITY_ACTION: u8 = 9;
pub(crate) const LEGACY_SIGN_TEXT_KEYS: [&str; SIGN_LINE_COUNT] =
    ["Text1", "Text2", "Text3", "Text4"];
pub(crate) const BLOCK_ENTITY_PACKED_X_SHIFT: u8 = 4;
pub(crate) const BLOCK_ENTITY_PACKED_COORD_MASK: u8 = 0x0F;
pub(crate) const CHUNK_SECTION_WIDTH_LOG2: i32 = 4;
pub(crate) const SURVIVAL_BIOME_DIMENSION_PROBE_ENV: &str =
    "MC_COMPAT_SURVIVAL_BIOME_DIMENSION_PROBE";
pub(crate) const SURVIVAL_BIOME_DIMENSION_TRAVEL_PROBE_ENV: &str =
    "MC_COMPAT_SURVIVAL_BIOME_DIMENSION_TRAVEL_PROBE";
pub(crate) const SURVIVAL_BIOME_DIMENSION_TRAVEL_LOG_TICK: u32 = 120;
pub(crate) const SURVIVAL_OVERWORLD_ID: &str = "minecraft:overworld";
pub(crate) const SURVIVAL_NETHER_ID: &str = "minecraft:the_nether";
pub(crate) const SURVIVAL_END_ID: &str = "minecraft:the_end";
pub(crate) const SURVIVAL_UNKNOWN_ENVIRONMENT_ID: &str = "unknown";
pub(crate) const SURVIVAL_CRAFTING_INPUT_COUNT: isize = 1;
pub(crate) const SURVIVAL_CRAFTING_RESULT_COUNT: isize = 4;
pub(crate) const SURVIVAL_CRAFTING_CLICK_BUTTON: u8 = 0;
pub(crate) const SURVIVAL_CRAFTING_CLICK_MODE: i32 = 0;
pub(crate) const PLAYER_INVENTORY_WINDOW_ID: u8 = 0;
pub(crate) const INVENTORY_STACK_SPLIT_MERGE_PROBE_ENV: &str =
    "MC_COMPAT_INVENTORY_STACK_SPLIT_MERGE_PROBE";
pub(crate) const INVENTORY_DRAG_TRANSACTIONS_PROBE_ENV: &str =
    "MC_COMPAT_INVENTORY_DRAG_TRANSACTIONS_PROBE";
pub(crate) const INVENTORY_STACK_SPLIT_MERGE_FIRST_TICK: u32 = 220;
pub(crate) const INVENTORY_DRAG_TRANSACTIONS_FIRST_TICK: u32 = 220;
pub(crate) const INVENTORY_STACK_SOURCE_SLOT: i16 = 37;
pub(crate) const INVENTORY_STACK_SOURCE_SLOT_INDEX: usize = 37;
pub(crate) const INVENTORY_STACK_DESTINATION_SLOT: i16 = 38;
pub(crate) const INVENTORY_DRAG_TARGET_SLOT_A: i16 = 38;
pub(crate) const INVENTORY_DRAG_TARGET_SLOT_B: i16 = 39;
pub(crate) const INVENTORY_STACK_FULL_COUNT: isize = 64;
pub(crate) const INVENTORY_STACK_HALF_COUNT: isize = 32;
pub(crate) const INVENTORY_STACK_EMPTY_COUNT: isize = 0;
pub(crate) const INVENTORY_STACK_LEFT_BUTTON: u8 = 0;
pub(crate) const INVENTORY_STACK_RIGHT_BUTTON: u8 = 1;
pub(crate) const INVENTORY_STACK_CLICK_MODE: i32 = 0;
pub(crate) const INVENTORY_DRAG_START_BUTTON: u8 = 0;
pub(crate) const INVENTORY_DRAG_ADD_SLOT_BUTTON: u8 = 1;
pub(crate) const INVENTORY_DRAG_END_BUTTON: u8 = 2;
pub(crate) const INVENTORY_DRAG_CLICK_MODE: i32 = 5;
pub(crate) const INVENTORY_DRAG_OUTSIDE_SLOT: i16 = -999;
pub(crate) const INVENTORY_STACK_ITEM_PROTOCOL_ID: isize = 194;
pub(crate) const INVENTORY_STACK_ITEM_NAME: &str = "RedWool";
pub(crate) const EMPTY_WINDOW_ID: u8 = 0;
pub(crate) const EMPTY_WINDOW_STATE_ID: i32 = -1;
pub(crate) const NEGATIVE_INVENTORY_INVALID_SLOT: i16 = 127;
pub(crate) const NEGATIVE_INVENTORY_INVALID_WINDOW_ID: u8 = 127;
pub(crate) const NEGATIVE_INVENTORY_STALE_STATE_OFFSET: i32 = 1;
pub(crate) const NEGATIVE_CLICK_BUTTON: u8 = 0;
pub(crate) const NEGATIVE_CLICK_MODE: i32 = 0;
pub(crate) const NEGATIVE_CUSTOM_PAYLOAD_TICK: u32 = 420;
pub(crate) const NEGATIVE_CUSTOM_PAYLOAD_CONTAINMENT_TICK: u32 = 460;
pub(crate) const NEGATIVE_FLAG_CONTAINMENT_TICK_OFFSET: u32 = 120;
pub(crate) const NEGATIVE_CUSTOM_PAYLOAD_DATA: &[u8] = &[0xff, 0x00, 0xff];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct InventoryStackSplitMergeProbeInput {
    pub(crate) enabled: bool,
    pub(crate) active_ticks: u32,
    pub(crate) source_stack_seen: bool,
    pub(crate) current_state_id: i32,
    pub(crate) split_pickup_sent: bool,
    pub(crate) split_source_seen: bool,
    pub(crate) split_place_sent: bool,
    pub(crate) split_destination_seen: bool,
    pub(crate) merge_pickup_sent: bool,
    pub(crate) merge_destination_empty_seen: bool,
    pub(crate) merge_place_sent: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum InventoryStackSplitMergeAction {
    SplitPickup,
    SplitPlace,
    MergePickup,
    MergePlace,
}

pub(crate) fn next_inventory_stack_split_merge_action(
    input: InventoryStackSplitMergeProbeInput,
) -> Option<InventoryStackSplitMergeAction> {
    if !input.enabled
        || !input.source_stack_seen
        || input.active_ticks < INVENTORY_STACK_SPLIT_MERGE_FIRST_TICK
        || input.current_state_id <= EMPTY_WINDOW_STATE_ID
    {
        return None;
    }

    if !input.split_pickup_sent {
        return Some(InventoryStackSplitMergeAction::SplitPickup);
    }

    if !input.split_source_seen {
        return None;
    }

    if !input.split_place_sent {
        return Some(InventoryStackSplitMergeAction::SplitPlace);
    }

    if !input.split_destination_seen {
        return None;
    }

    if !input.merge_pickup_sent {
        return Some(InventoryStackSplitMergeAction::MergePickup);
    }

    if !input.merge_destination_empty_seen {
        return None;
    }

    if !input.merge_place_sent {
        return Some(InventoryStackSplitMergeAction::MergePlace);
    }

    None
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct InventoryDragTransactionsProbeInput {
    pub(crate) enabled: bool,
    pub(crate) active_ticks: u32,
    pub(crate) source_stack_seen: bool,
    pub(crate) current_state_id: i32,
    pub(crate) pickup_sent: bool,
    pub(crate) source_empty_seen: bool,
    pub(crate) drag_start_sent: bool,
    pub(crate) target_a_sent: bool,
    pub(crate) target_b_sent: bool,
    pub(crate) drag_end_sent: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum InventoryDragTransactionsAction {
    PickupSource,
    DragStart,
    AddTargetA,
    AddTargetB,
    DragEnd,
}

pub(crate) fn next_inventory_drag_transactions_action(
    input: InventoryDragTransactionsProbeInput,
) -> Option<InventoryDragTransactionsAction> {
    if !input.enabled
        || !input.source_stack_seen
        || input.active_ticks < INVENTORY_DRAG_TRANSACTIONS_FIRST_TICK
        || input.current_state_id <= EMPTY_WINDOW_STATE_ID
    {
        return None;
    }

    if !input.pickup_sent {
        return Some(InventoryDragTransactionsAction::PickupSource);
    }

    if !input.source_empty_seen {
        return None;
    }

    if !input.drag_start_sent {
        return Some(InventoryDragTransactionsAction::DragStart);
    }

    if !input.target_a_sent {
        return Some(InventoryDragTransactionsAction::AddTargetA);
    }

    if !input.target_b_sent {
        return Some(InventoryDragTransactionsAction::AddTargetB);
    }

    if !input.drag_end_sent {
        return Some(InventoryDragTransactionsAction::DragEnd);
    }

    None
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct SurvivalMobDropProbeInput {
    pub(crate) enabled: bool,
    pub(crate) active_ticks: u32,
    pub(crate) position_sent: bool,
    pub(crate) mob_seen: bool,
    pub(crate) attack_sent: bool,
    pub(crate) target_entity_known: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SurvivalMobDropAction {
    MoveNearMob,
    AttackMob,
}

pub(crate) fn next_survival_mob_drop_action(
    input: SurvivalMobDropProbeInput,
) -> Option<SurvivalMobDropAction> {
    if !input.enabled {
        return None;
    }

    if input.active_ticks >= SURVIVAL_MOB_DROP_POSITION_TICK && !input.position_sent {
        return Some(SurvivalMobDropAction::MoveNearMob);
    }

    if input.active_ticks >= SURVIVAL_MOB_DROP_ATTACK_TICK
        && input.mob_seen
        && !input.attack_sent
        && input.target_entity_known
    {
        return Some(SurvivalMobDropAction::AttackMob);
    }

    None
}

pub(crate) fn survival_crafting_table_position() -> Position {
    Position::new(
        SURVIVAL_CRAFTING_TABLE_X,
        SURVIVAL_CRAFTING_TABLE_Y,
        SURVIVAL_CRAFTING_TABLE_Z,
    )
}

pub(crate) fn survival_crafting_input_stack() -> item::Stack {
    item::Stack {
        id: SURVIVAL_CRAFTING_INPUT_ITEM_ID,
        count: SURVIVAL_CRAFTING_INPUT_COUNT,
        damage: None,
        tag: None,
    }
}

pub(crate) fn survival_crafting_result_stack() -> item::Stack {
    item::Stack {
        id: SURVIVAL_CRAFTING_RESULT_ITEM_ID,
        count: SURVIVAL_CRAFTING_RESULT_COUNT,
        damage: None,
        tag: None,
    }
}

pub(crate) fn survival_crafting_result_matches(stack: &item::Stack) -> bool {
    stack.id == SURVIVAL_CRAFTING_RESULT_ITEM_ID && stack.count == SURVIVAL_CRAFTING_RESULT_COUNT
}

pub(crate) fn survival_furnace_position() -> Position {
    Position::new(SURVIVAL_FURNACE_X, SURVIVAL_FURNACE_Y, SURVIVAL_FURNACE_Z)
}

pub(crate) fn survival_furnace_input_stack() -> item::Stack {
    survival_furnace_stack(SURVIVAL_FURNACE_INPUT_ITEM_ID)
}

pub(crate) fn survival_furnace_fuel_stack() -> item::Stack {
    survival_furnace_stack(SURVIVAL_FURNACE_FUEL_ITEM_ID)
}

pub(crate) fn survival_furnace_invalid_fuel_stack() -> item::Stack {
    survival_furnace_input_stack()
}

pub(crate) fn survival_furnace_output_stack() -> item::Stack {
    survival_furnace_stack(SURVIVAL_FURNACE_OUTPUT_ITEM_ID)
}

fn survival_furnace_stack(item_id: isize) -> item::Stack {
    item::Stack {
        id: item_id,
        count: SURVIVAL_FURNACE_ITEM_COUNT,
        damage: None,
        tag: None,
    }
}

pub(crate) fn survival_furnace_output_matches(stack: &item::Stack) -> bool {
    stack.id == SURVIVAL_FURNACE_OUTPUT_ITEM_ID && stack.count == SURVIVAL_FURNACE_ITEM_COUNT
}

pub(crate) fn survival_furnace_invalid_fuel_matches(stack: &item::Stack) -> bool {
    stack.id == SURVIVAL_FURNACE_INPUT_ITEM_ID && stack.count == SURVIVAL_FURNACE_ITEM_COUNT
}

pub(crate) fn survival_hunger_food_item_matches(stack: &item::Stack) -> bool {
    stack.id == SURVIVAL_HUNGER_FOOD_ITEM_ID
        && stack.count == SURVIVAL_HUNGER_FOOD_ITEM_COUNT_BEFORE
}

pub(crate) fn survival_hunger_food_slot_is_empty(slot_item: Option<&Option<item::Stack>>) -> bool {
    match slot_item {
        Some(None) => true,
        Some(Some(stack)) => stack.count == SURVIVAL_HUNGER_FOOD_ITEM_COUNT_AFTER,
        None => false,
    }
}

pub(crate) fn survival_hunger_food_float_matches(observed: f32, expected: f32) -> bool {
    (observed - expected).abs() <= SURVIVAL_HUNGER_FOOD_FLOAT_TOLERANCE
}

pub(crate) fn survival_hunger_food_pre_update_matches(
    update: &packet::play::clientbound::UpdateHealth,
) -> bool {
    survival_hunger_food_float_matches(update.health, SURVIVAL_HUNGER_FOOD_PRE_HEALTH)
        && update.food.0 == SURVIVAL_HUNGER_FOOD_PRE_FOOD
        && survival_hunger_food_float_matches(
            update.food_saturation,
            SURVIVAL_HUNGER_FOOD_PRE_SATURATION,
        )
}

pub(crate) fn survival_hunger_food_post_update_matches(
    update: &packet::play::clientbound::UpdateHealth,
) -> bool {
    survival_hunger_food_float_matches(update.health, SURVIVAL_HUNGER_FOOD_POST_HEALTH)
        && update.food.0 == SURVIVAL_HUNGER_FOOD_POST_FOOD
        && survival_hunger_food_float_matches(
            update.food_saturation,
            SURVIVAL_HUNGER_FOOD_POST_SATURATION,
        )
}

pub(crate) fn survival_hunger_health_pre_update_matches(
    update: &packet::play::clientbound::UpdateHealth,
) -> bool {
    survival_hunger_food_float_matches(update.health, SURVIVAL_HUNGER_HEALTH_PRE_HEALTH)
        && update.food.0 == SURVIVAL_HUNGER_HEALTH_PRE_FOOD
        && survival_hunger_food_float_matches(
            update.food_saturation,
            SURVIVAL_HUNGER_HEALTH_PRE_SATURATION,
        )
}

pub(crate) fn survival_hunger_health_post_update_matches(
    update: &packet::play::clientbound::UpdateHealth,
) -> bool {
    survival_hunger_food_float_matches(update.health, SURVIVAL_HUNGER_HEALTH_POST_HEALTH)
        && update.food.0 == SURVIVAL_HUNGER_HEALTH_POST_FOOD
        && survival_hunger_food_float_matches(
            update.food_saturation,
            SURVIVAL_HUNGER_HEALTH_POST_SATURATION,
        )
}

pub(crate) fn survival_mob_drop_item_matches(stack: &item::Stack) -> bool {
    survival_mob_drop_item_id_matches(stack.id) && stack.count == SURVIVAL_MOB_DROP_ITEM_COUNT
}

fn survival_mob_drop_item_id_matches(item_id: isize) -> bool {
    item_id == SURVIVAL_MOB_DROP_ITEM_ID || item_id == SURVIVAL_MOB_DROP_PROTOCOL_758_ITEM_ID
}

pub(crate) fn survival_mob_drop_position_matches(x: f64, y: f64, z: f64) -> bool {
    (x - SURVIVAL_MOB_DROP_TARGET_X).abs() <= SURVIVAL_MOB_DROP_POSITION_TOLERANCE
        && (y - SURVIVAL_MOB_DROP_TARGET_Y).abs() <= SURVIVAL_MOB_DROP_POSITION_TOLERANCE
        && (z - SURVIVAL_MOB_DROP_TARGET_Z).abs() <= SURVIVAL_MOB_DROP_POSITION_TOLERANCE
}

pub(crate) fn survival_redstone_toggle_output_position_matches(location: Position) -> bool {
    location.x == SURVIVAL_REDSTONE_TOGGLE_OUTPUT_X
        && location.y == SURVIVAL_REDSTONE_TOGGLE_OUTPUT_Y
        && location.z == SURVIVAL_REDSTONE_TOGGLE_OUTPUT_Z
}

pub(crate) fn survival_world_persistence_position_matches(location: Position) -> bool {
    location.x == SURVIVAL_WORLD_PERSISTENCE_X
        && location.y == SURVIVAL_WORLD_PERSISTENCE_Y
        && location.z == SURVIVAL_WORLD_PERSISTENCE_Z
}

pub(crate) fn survival_block_entity_position_matches(location: Position) -> bool {
    location.x == SURVIVAL_BLOCK_ENTITY_X
        && location.y == SURVIVAL_BLOCK_ENTITY_Y
        && location.z == SURVIVAL_BLOCK_ENTITY_Z
}

pub(crate) fn survival_block_entity_expected_lines() -> [String; SIGN_LINE_COUNT] {
    [
        SURVIVAL_BLOCK_ENTITY_TEXT_LINE_1.to_string(),
        SURVIVAL_BLOCK_ENTITY_TEXT_LINE_2.to_string(),
        SURVIVAL_BLOCK_ENTITY_TEXT_LINE_3.to_string(),
        SURVIVAL_BLOCK_ENTITY_TEXT_LINE_4.to_string(),
    ]
}

pub(crate) fn sign_lines_match_payload(lines: &[String; SIGN_LINE_COUNT]) -> bool {
    lines[SIGN_LINE_INDEX_1] == SURVIVAL_BLOCK_ENTITY_TEXT_LINE_1
        && lines[SIGN_LINE_INDEX_2] == SURVIVAL_BLOCK_ENTITY_TEXT_LINE_2
        && lines[SIGN_LINE_INDEX_3] == SURVIVAL_BLOCK_ENTITY_TEXT_LINE_3
        && lines[SIGN_LINE_INDEX_4] == SURVIVAL_BLOCK_ENTITY_TEXT_LINE_4
}

pub(crate) fn sign_text_payload(lines: &[String; SIGN_LINE_COUNT]) -> String {
    lines.join("|")
}

fn component_text(raw: &str) -> String {
    format::Component::from_string(raw).to_string()
}

fn sign_lines_from_vec(lines: Vec<String>) -> Option<[String; SIGN_LINE_COUNT]> {
    if lines.len() != SIGN_LINE_COUNT {
        return None;
    }
    Some([
        lines[SIGN_LINE_INDEX_1].clone(),
        lines[SIGN_LINE_INDEX_2].clone(),
        lines[SIGN_LINE_INDEX_3].clone(),
        lines[SIGN_LINE_INDEX_4].clone(),
    ])
}

fn extract_modern_sign_lines(nbt: &crate::nbt::NamedTag) -> Option<[String; SIGN_LINE_COUNT]> {
    let root = nbt.1.as_compound()?;
    let front_text = root.get(MODERN_SIGN_FRONT_TEXT_KEY)?.as_compound()?;
    let messages = front_text.get(MODERN_SIGN_MESSAGES_KEY)?.as_list()?;
    if messages.len() != SIGN_LINE_COUNT {
        return None;
    }
    let lines = messages
        .iter()
        .map(|tag| tag.as_str().map(component_text))
        .collect::<Option<Vec<_>>>()?;
    sign_lines_from_vec(lines)
}

fn extract_legacy_sign_lines(nbt: &crate::nbt::NamedTag) -> Option<[String; SIGN_LINE_COUNT]> {
    let lines = LEGACY_SIGN_TEXT_KEYS
        .iter()
        .map(|key| {
            nbt.1
                .get(key)
                .and_then(|tag| tag.as_str())
                .map(component_text)
        })
        .collect::<Option<Vec<_>>>()?;
    sign_lines_from_vec(lines)
}

pub(crate) fn extract_sign_lines_from_nbt(
    nbt: &crate::nbt::NamedTag,
) -> Option<[String; SIGN_LINE_COUNT]> {
    extract_modern_sign_lines(nbt).or_else(|| extract_legacy_sign_lines(nbt))
}

pub(crate) fn packed_block_entity_position(
    chunk_x: i32,
    chunk_z: i32,
    packed_xz: u8,
    y: i16,
) -> Position {
    let chunk_width = 1 << CHUNK_SECTION_WIDTH_LOG2;
    let local_x =
        i32::from((packed_xz >> BLOCK_ENTITY_PACKED_X_SHIFT) & BLOCK_ENTITY_PACKED_COORD_MASK);
    let local_z = i32::from(packed_xz & BLOCK_ENTITY_PACKED_COORD_MASK);
    Position::new(
        chunk_x * chunk_width + local_x,
        i32::from(y),
        chunk_z * chunk_width + local_z,
    )
}

pub(crate) fn should_log_survival_crafting_inventory_slot(
    window_id: u8,
    crafting_window_id: u8,
    slot: i16,
) -> bool {
    (window_id == PLAYER_INVENTORY_WINDOW_ID && slot == SURVIVAL_CRAFTING_INVENTORY_SLOT)
        || (window_id == crafting_window_id && slot == SURVIVAL_CRAFTING_OPEN_INVENTORY_MIRROR_SLOT)
}

pub(crate) fn should_log_survival_crafting_inventory_index(slot_index: usize) -> bool {
    slot_index == SURVIVAL_CRAFTING_INVENTORY_INDEX
        || slot_index == SURVIVAL_CRAFTING_OPEN_INVENTORY_MIRROR_INDEX
}

pub(crate) fn should_log_survival_furnace_inventory_slot(
    window_id: u8,
    furnace_window_id: u8,
    slot: i16,
) -> bool {
    (window_id == PLAYER_INVENTORY_WINDOW_ID && slot == SURVIVAL_FURNACE_INVENTORY_SLOT)
        || (window_id == furnace_window_id
            && furnace_window_id > EMPTY_WINDOW_ID
            && slot == SURVIVAL_FURNACE_OPEN_INVENTORY_MIRROR_SLOT)
}

pub(crate) fn should_log_survival_furnace_inventory_index(slot_index: usize) -> bool {
    slot_index == SURVIVAL_FURNACE_INVENTORY_INDEX
        || slot_index == SURVIVAL_FURNACE_OPEN_INVENTORY_MIRROR_INDEX
}

pub(crate) fn normalize_survival_environment_id(raw: &str) -> &'static str {
    match raw {
        SURVIVAL_OVERWORLD_ID => SURVIVAL_OVERWORLD_ID,
        SURVIVAL_NETHER_ID => SURVIVAL_NETHER_ID,
        SURVIVAL_END_ID => SURVIVAL_END_ID,
        _ => SURVIVAL_UNKNOWN_ENVIRONMENT_ID,
    }
}

pub(crate) fn derive_survival_environment_id(
    dimension_type_name: &str,
    world_name: &str,
) -> &'static str {
    let world_environment = normalize_survival_environment_id(world_name);
    if world_environment != SURVIVAL_UNKNOWN_ENVIRONMENT_ID {
        world_environment
    } else {
        normalize_survival_environment_id(dimension_type_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::{self, packet};
    use crate::shared::Position;
    use steven_protocol::item;

    fn ready_stack_split_merge_input() -> InventoryStackSplitMergeProbeInput {
        InventoryStackSplitMergeProbeInput {
            enabled: true,
            active_ticks: INVENTORY_STACK_SPLIT_MERGE_FIRST_TICK,
            source_stack_seen: true,
            current_state_id: EMPTY_WINDOW_STATE_ID + 1,
            split_pickup_sent: false,
            split_source_seen: false,
            split_place_sent: false,
            split_destination_seen: false,
            merge_pickup_sent: false,
            merge_destination_empty_seen: false,
            merge_place_sent: false,
        }
    }

    fn ready_inventory_drag_transactions_input() -> InventoryDragTransactionsProbeInput {
        InventoryDragTransactionsProbeInput {
            enabled: true,
            active_ticks: INVENTORY_DRAG_TRANSACTIONS_FIRST_TICK,
            source_stack_seen: true,
            current_state_id: EMPTY_WINDOW_STATE_ID + 1,
            pickup_sent: false,
            source_empty_seen: false,
            drag_start_sent: false,
            target_a_sent: false,
            target_b_sent: false,
            drag_end_sent: false,
        }
    }

    fn ready_survival_mob_drop_input() -> SurvivalMobDropProbeInput {
        SurvivalMobDropProbeInput {
            enabled: true,
            active_ticks: SURVIVAL_MOB_DROP_ATTACK_TICK,
            position_sent: true,
            mob_seen: true,
            attack_sent: false,
            target_entity_known: true,
        }
    }

    #[test]
    fn inventory_stack_split_merge_actions_advance_after_observed_state() {
        let split_pickup = ready_stack_split_merge_input();
        assert_eq!(
            next_inventory_stack_split_merge_action(split_pickup),
            Some(InventoryStackSplitMergeAction::SplitPickup)
        );

        let split_place = InventoryStackSplitMergeProbeInput {
            split_pickup_sent: true,
            split_source_seen: true,
            current_state_id: split_pickup.current_state_id,
            ..split_pickup
        };
        assert_eq!(
            next_inventory_stack_split_merge_action(split_place),
            Some(InventoryStackSplitMergeAction::SplitPlace)
        );

        let merge_pickup = InventoryStackSplitMergeProbeInput {
            split_place_sent: true,
            split_destination_seen: true,
            current_state_id: split_place.current_state_id,
            ..split_place
        };
        assert_eq!(
            next_inventory_stack_split_merge_action(merge_pickup),
            Some(InventoryStackSplitMergeAction::MergePickup)
        );

        let merge_place = InventoryStackSplitMergeProbeInput {
            merge_pickup_sent: true,
            merge_destination_empty_seen: true,
            current_state_id: merge_pickup.current_state_id,
            ..merge_pickup
        };
        assert_eq!(
            next_inventory_stack_split_merge_action(merge_place),
            Some(InventoryStackSplitMergeAction::MergePlace)
        );

        let completed = InventoryStackSplitMergeProbeInput {
            merge_place_sent: true,
            current_state_id: merge_place.current_state_id,
            ..merge_place
        };
        assert_eq!(next_inventory_stack_split_merge_action(completed), None);
    }

    #[test]
    fn inventory_stack_split_merge_waits_for_probe_guards() {
        let ready = ready_stack_split_merge_input();
        assert_eq!(
            next_inventory_stack_split_merge_action(InventoryStackSplitMergeProbeInput {
                enabled: false,
                ..ready
            }),
            None
        );
        assert_eq!(
            next_inventory_stack_split_merge_action(InventoryStackSplitMergeProbeInput {
                source_stack_seen: false,
                ..ready
            }),
            None
        );
        assert_eq!(
            next_inventory_stack_split_merge_action(InventoryStackSplitMergeProbeInput {
                active_ticks: INVENTORY_STACK_SPLIT_MERGE_FIRST_TICK - 1,
                ..ready
            }),
            None
        );
        assert_eq!(
            next_inventory_stack_split_merge_action(InventoryStackSplitMergeProbeInput {
                split_pickup_sent: true,
                current_state_id: EMPTY_WINDOW_STATE_ID,
                ..ready
            }),
            None
        );
        assert_eq!(
            next_inventory_stack_split_merge_action(InventoryStackSplitMergeProbeInput {
                split_pickup_sent: true,
                ..ready
            }),
            None
        );
    }

    #[test]
    fn inventory_drag_transactions_actions_advance_after_observed_state() {
        let pickup = ready_inventory_drag_transactions_input();
        assert_eq!(
            next_inventory_drag_transactions_action(pickup),
            Some(InventoryDragTransactionsAction::PickupSource)
        );

        let drag_start = InventoryDragTransactionsProbeInput {
            pickup_sent: true,
            source_empty_seen: true,
            current_state_id: pickup.current_state_id,
            ..pickup
        };
        assert_eq!(
            next_inventory_drag_transactions_action(drag_start),
            Some(InventoryDragTransactionsAction::DragStart)
        );

        let target_a = InventoryDragTransactionsProbeInput {
            drag_start_sent: true,
            current_state_id: drag_start.current_state_id,
            ..drag_start
        };
        assert_eq!(
            next_inventory_drag_transactions_action(target_a),
            Some(InventoryDragTransactionsAction::AddTargetA)
        );

        let target_b = InventoryDragTransactionsProbeInput {
            target_a_sent: true,
            current_state_id: target_a.current_state_id,
            ..target_a
        };
        assert_eq!(
            next_inventory_drag_transactions_action(target_b),
            Some(InventoryDragTransactionsAction::AddTargetB)
        );

        let drag_end = InventoryDragTransactionsProbeInput {
            target_b_sent: true,
            current_state_id: target_b.current_state_id,
            ..target_b
        };
        assert_eq!(
            next_inventory_drag_transactions_action(drag_end),
            Some(InventoryDragTransactionsAction::DragEnd)
        );

        let completed = InventoryDragTransactionsProbeInput {
            drag_end_sent: true,
            current_state_id: drag_end.current_state_id,
            ..drag_end
        };
        assert_eq!(next_inventory_drag_transactions_action(completed), None);
    }

    #[test]
    fn inventory_drag_transactions_waits_for_probe_guards() {
        let ready = ready_inventory_drag_transactions_input();
        assert_eq!(
            next_inventory_drag_transactions_action(InventoryDragTransactionsProbeInput {
                enabled: false,
                ..ready
            }),
            None
        );
        assert_eq!(
            next_inventory_drag_transactions_action(InventoryDragTransactionsProbeInput {
                source_stack_seen: false,
                ..ready
            }),
            None
        );
        assert_eq!(
            next_inventory_drag_transactions_action(InventoryDragTransactionsProbeInput {
                active_ticks: INVENTORY_DRAG_TRANSACTIONS_FIRST_TICK - 1,
                ..ready
            }),
            None
        );
        assert_eq!(
            next_inventory_drag_transactions_action(InventoryDragTransactionsProbeInput {
                pickup_sent: true,
                current_state_id: EMPTY_WINDOW_STATE_ID,
                ..ready
            }),
            None
        );
        assert_eq!(
            next_inventory_drag_transactions_action(InventoryDragTransactionsProbeInput {
                pickup_sent: true,
                ..ready
            }),
            None
        );
    }

    #[test]
    fn combat_probe_decision_preserves_regular_schedule() {
        let movement = next_combat_probe_decision(CombatProbeInput {
            enabled: true,
            attacker_role: true,
            flag_carrier_death_probe: false,
            active_ticks: COMBAT_ATTACK_MOVE_TICK,
            attacks_sent: 0,
        })
        .expect("move decision");
        assert_eq!(movement.movement, Some(CombatMovementAction::MoveNear));
        assert!(!movement.should_attack);
        assert_eq!(movement.plan.attack_label, COMBAT_REGULAR_ATTACK_LABEL);

        let attack = next_combat_probe_decision(CombatProbeInput {
            enabled: true,
            attacker_role: true,
            flag_carrier_death_probe: false,
            active_ticks: COMBAT_REGULAR_ATTACK_START_TICK,
            attacks_sent: 0,
        })
        .expect("attack decision");
        assert!(attack.should_attack);
        assert_eq!(attack.plan.attack_limit, COMBAT_REGULAR_ATTACK_LIMIT);
    }

    #[test]
    fn combat_probe_decision_rejects_disabled_non_attacker_or_limit() {
        let ready = CombatProbeInput {
            enabled: true,
            attacker_role: true,
            flag_carrier_death_probe: true,
            active_ticks: COMBAT_FLAG_CARRIER_ATTACK_START_TICK,
            attacks_sent: COMBAT_FLAG_CARRIER_ATTACK_LIMIT,
        };
        let limited = next_combat_probe_decision(ready).expect("limited hold decision");
        assert_eq!(limited.movement, Some(CombatMovementAction::HoldPosition));
        assert!(!limited.should_attack);

        let after_hold_tick = COMBAT_ATTACK_HOLD_END_TICK + COMBAT_ATTACK_INTERVAL_TICKS;
        assert_eq!(
            next_combat_probe_decision(CombatProbeInput {
                active_ticks: after_hold_tick,
                ..ready
            }),
            None
        );
        assert_eq!(
            next_combat_probe_decision(CombatProbeInput {
                enabled: false,
                attacks_sent: 0,
                ..ready
            }),
            None
        );
        assert_eq!(
            next_combat_probe_decision(CombatProbeInput {
                attacker_role: false,
                attacks_sent: 0,
                ..ready
            }),
            None
        );
    }

    #[test]
    fn ctf_score_limit_chat_requires_both_score_terms() {
        assert!(ctf_score_limit_chat_matches(
            "winner RED: 2 scoreboard BLUE: 0"
        ));
        assert!(!ctf_score_limit_chat_matches("winner RED: 2 only"));
        assert!(!ctf_score_limit_chat_matches("BLUE: 0 only"));
    }

    #[test]
    fn flag_probe_repeat_target_defaults_when_unset_or_invalid() {
        assert_eq!(
            parse_flag_probe_repeat_target(None),
            DEFAULT_FLAG_PROBE_REPEAT_TARGET
        );
        assert_eq!(
            parse_flag_probe_repeat_target(Some("")),
            DEFAULT_FLAG_PROBE_REPEAT_TARGET
        );
        assert_eq!(
            parse_flag_probe_repeat_target(Some("0")),
            DEFAULT_FLAG_PROBE_REPEAT_TARGET
        );
        assert_eq!(
            parse_flag_probe_repeat_target(Some("not-a-number")),
            DEFAULT_FLAG_PROBE_REPEAT_TARGET
        );
    }

    #[test]
    fn flag_probe_repeat_target_accepts_positive_counts() {
        assert_eq!(parse_flag_probe_repeat_target(Some("2")), 2);
        assert_eq!(parse_flag_probe_repeat_target(Some(" 3 ")), 3);
    }

    #[test]
    fn flag_probe_repeat_target_is_capped() {
        assert_eq!(
            parse_flag_probe_repeat_target(Some("999")),
            MAX_FLAG_PROBE_REPEAT_TARGET
        );
    }

    #[test]
    fn survival_mob_drop_actions_progress_and_fail_closed() {
        assert_eq!(
            next_survival_mob_drop_action(SurvivalMobDropProbeInput {
                active_ticks: SURVIVAL_MOB_DROP_POSITION_TICK,
                position_sent: false,
                mob_seen: false,
                target_entity_known: false,
                ..ready_survival_mob_drop_input()
            }),
            Some(SurvivalMobDropAction::MoveNearMob)
        );
        assert_eq!(
            next_survival_mob_drop_action(ready_survival_mob_drop_input()),
            Some(SurvivalMobDropAction::AttackMob)
        );
        assert_eq!(
            next_survival_mob_drop_action(SurvivalMobDropProbeInput {
                target_entity_known: false,
                ..ready_survival_mob_drop_input()
            }),
            None
        );
        assert_eq!(
            next_survival_mob_drop_action(SurvivalMobDropProbeInput {
                enabled: false,
                ..ready_survival_mob_drop_input()
            }),
            None
        );
    }

    #[test]
    fn survival_biome_dimension_environment_prefers_world_identifier() {
        assert_eq!(
            derive_survival_environment_id(SURVIVAL_NETHER_ID, SURVIVAL_OVERWORLD_ID),
            SURVIVAL_OVERWORLD_ID
        );
        assert_eq!(
            derive_survival_environment_id(SURVIVAL_END_ID, "custom:unknown"),
            SURVIVAL_END_ID
        );
    }

    #[test]
    fn survival_biome_dimension_environment_rejects_unknown_identifiers() {
        assert_eq!(
            normalize_survival_environment_id("custom:unknown"),
            SURVIVAL_UNKNOWN_ENVIRONMENT_ID
        );
        assert_eq!(
            derive_survival_environment_id("custom:dimension", "custom:world"),
            SURVIVAL_UNKNOWN_ENVIRONMENT_ID
        );
    }

    #[test]
    fn survival_crafting_fixture_positions_are_named() {
        let position = survival_crafting_table_position();

        assert_eq!(position.x, SURVIVAL_CRAFTING_TABLE_X);
        assert_eq!(position.y, SURVIVAL_CRAFTING_TABLE_Y);
        assert_eq!(position.z, SURVIVAL_CRAFTING_TABLE_Z);
    }

    #[test]
    fn survival_crafting_stacks_use_contract_items() {
        let input = survival_crafting_input_stack();
        let result = survival_crafting_result_stack();

        assert_eq!(input.id, SURVIVAL_CRAFTING_INPUT_ITEM_ID);
        assert_eq!(input.count, SURVIVAL_CRAFTING_INPUT_COUNT);
        assert_eq!(result.id, SURVIVAL_CRAFTING_RESULT_ITEM_ID);
        assert_eq!(result.count, SURVIVAL_CRAFTING_RESULT_COUNT);
        assert!(survival_crafting_result_matches(&result));
    }

    #[test]
    fn survival_crafting_result_match_rejects_wrong_item_or_count() {
        let wrong_item = item::Stack {
            id: SURVIVAL_CRAFTING_INPUT_ITEM_ID,
            count: SURVIVAL_CRAFTING_RESULT_COUNT,
            damage: None,
            tag: None,
        };
        let wrong_count = item::Stack {
            id: SURVIVAL_CRAFTING_RESULT_ITEM_ID,
            count: SURVIVAL_CRAFTING_INPUT_COUNT,
            damage: None,
            tag: None,
        };

        assert!(!survival_crafting_result_matches(&wrong_item));
        assert!(!survival_crafting_result_matches(&wrong_count));
    }

    #[test]
    fn survival_furnace_fixture_positions_are_named() {
        let position = survival_furnace_position();

        assert_eq!(position.x, SURVIVAL_FURNACE_X);
        assert_eq!(position.y, SURVIVAL_FURNACE_Y);
        assert_eq!(position.z, SURVIVAL_FURNACE_Z);
    }

    #[test]
    fn survival_furnace_stacks_use_contract_items() {
        let input = survival_furnace_input_stack();
        let fuel = survival_furnace_fuel_stack();
        let output = survival_furnace_output_stack();
        let invalid_fuel = survival_furnace_invalid_fuel_stack();

        assert_eq!(input.id, SURVIVAL_FURNACE_INPUT_ITEM_ID);
        assert_eq!(input.count, SURVIVAL_FURNACE_ITEM_COUNT);
        assert_eq!(fuel.id, SURVIVAL_FURNACE_FUEL_ITEM_ID);
        assert_eq!(fuel.count, SURVIVAL_FURNACE_ITEM_COUNT);
        assert_eq!(output.id, SURVIVAL_FURNACE_OUTPUT_ITEM_ID);
        assert_eq!(output.count, SURVIVAL_FURNACE_ITEM_COUNT);
        assert_eq!(invalid_fuel.id, SURVIVAL_FURNACE_INPUT_ITEM_ID);
        assert_eq!(invalid_fuel.count, SURVIVAL_FURNACE_ITEM_COUNT);
        assert!(survival_furnace_output_matches(&output));
        assert!(survival_furnace_invalid_fuel_matches(&invalid_fuel));
    }

    #[test]
    fn survival_furnace_output_match_rejects_wrong_item_or_count() {
        let wrong_item = item::Stack {
            id: SURVIVAL_FURNACE_INPUT_ITEM_ID,
            count: SURVIVAL_FURNACE_ITEM_COUNT,
            damage: None,
            tag: None,
        };
        let wrong_count = item::Stack {
            id: SURVIVAL_FURNACE_OUTPUT_ITEM_ID,
            count: SURVIVAL_FURNACE_ITEM_COUNT + 1,
            damage: None,
            tag: None,
        };

        assert!(!survival_furnace_output_matches(&wrong_item));
        assert!(!survival_furnace_output_matches(&wrong_count));
        assert!(!survival_furnace_invalid_fuel_matches(&wrong_count));
    }

    #[test]
    fn survival_hunger_food_stack_match_accepts_only_contract_bread() {
        let bread = item::Stack {
            id: SURVIVAL_HUNGER_FOOD_ITEM_ID,
            count: SURVIVAL_HUNGER_FOOD_ITEM_COUNT_BEFORE,
            damage: None,
            tag: None,
        };
        let wrong_item = item::Stack {
            id: SURVIVAL_FURNACE_OUTPUT_ITEM_ID,
            count: SURVIVAL_HUNGER_FOOD_ITEM_COUNT_BEFORE,
            damage: None,
            tag: None,
        };
        let wrong_count = item::Stack {
            id: SURVIVAL_HUNGER_FOOD_ITEM_ID,
            count: SURVIVAL_HUNGER_FOOD_ITEM_COUNT_BEFORE + 1,
            damage: None,
            tag: None,
        };

        assert!(survival_hunger_food_item_matches(&bread));
        assert!(!survival_hunger_food_item_matches(&wrong_item));
        assert!(!survival_hunger_food_item_matches(&wrong_count));
    }

    #[test]
    fn survival_hunger_food_slot_empty_requires_explicit_empty_slot() {
        let empty_slot = None;
        let bread_slot = Some(item::Stack {
            id: SURVIVAL_HUNGER_FOOD_ITEM_ID,
            count: SURVIVAL_HUNGER_FOOD_ITEM_COUNT_BEFORE,
            damage: None,
            tag: None,
        });

        assert!(survival_hunger_food_slot_is_empty(Some(&empty_slot)));
        assert!(!survival_hunger_food_slot_is_empty(Some(&bread_slot)));
        assert!(!survival_hunger_food_slot_is_empty(None));
    }

    #[test]
    fn survival_mob_drop_item_match_accepts_only_contract_drop() {
        let ingot = item::Stack {
            id: SURVIVAL_MOB_DROP_ITEM_ID,
            count: SURVIVAL_MOB_DROP_ITEM_COUNT,
            damage: None,
            tag: None,
        };
        let legacy_ingot = item::Stack {
            id: SURVIVAL_MOB_DROP_PROTOCOL_758_ITEM_ID,
            count: SURVIVAL_MOB_DROP_ITEM_COUNT,
            damage: None,
            tag: None,
        };
        let wrong_item = item::Stack {
            id: SURVIVAL_MOB_DROP_ITEM_ID + 1,
            count: SURVIVAL_MOB_DROP_ITEM_COUNT,
            damage: None,
            tag: None,
        };
        let wrong_count = item::Stack {
            id: SURVIVAL_MOB_DROP_ITEM_ID,
            count: SURVIVAL_MOB_DROP_ITEM_COUNT + 1,
            damage: None,
            tag: None,
        };

        assert!(survival_mob_drop_item_matches(&ingot));
        assert!(survival_mob_drop_item_matches(&legacy_ingot));
        assert!(!survival_mob_drop_item_matches(&wrong_item));
        assert!(!survival_mob_drop_item_matches(&wrong_count));
    }

    #[test]
    fn survival_mob_drop_position_match_rejects_wrong_target() {
        assert!(survival_mob_drop_position_matches(
            SURVIVAL_MOB_DROP_TARGET_X,
            SURVIVAL_MOB_DROP_TARGET_Y,
            SURVIVAL_MOB_DROP_TARGET_Z
        ));
        assert!(!survival_mob_drop_position_matches(
            SURVIVAL_MOB_DROP_TARGET_X + SURVIVAL_MOB_DROP_POSITION_TOLERANCE + 1.0,
            SURVIVAL_MOB_DROP_TARGET_Y,
            SURVIVAL_MOB_DROP_TARGET_Z
        ));
    }

    #[test]
    fn survival_redstone_toggle_output_position_match_rejects_wrong_target() {
        let expected = Position::new(
            SURVIVAL_REDSTONE_TOGGLE_OUTPUT_X,
            SURVIVAL_REDSTONE_TOGGLE_OUTPUT_Y,
            SURVIVAL_REDSTONE_TOGGLE_OUTPUT_Z,
        );
        let wrong = Position::new(
            SURVIVAL_REDSTONE_TOGGLE_OUTPUT_X,
            SURVIVAL_REDSTONE_TOGGLE_OUTPUT_Y,
            SURVIVAL_REDSTONE_TOGGLE_OUTPUT_Z + 1,
        );

        assert!(survival_redstone_toggle_output_position_matches(expected));
        assert!(!survival_redstone_toggle_output_position_matches(wrong));
    }

    #[test]
    fn survival_hunger_food_health_updates_match_exact_contract() {
        let pre = packet::play::clientbound::UpdateHealth {
            health: SURVIVAL_HUNGER_FOOD_PRE_HEALTH,
            food: protocol::VarInt(SURVIVAL_HUNGER_FOOD_PRE_FOOD),
            food_saturation: SURVIVAL_HUNGER_FOOD_PRE_SATURATION,
        };
        let post = packet::play::clientbound::UpdateHealth {
            health: SURVIVAL_HUNGER_FOOD_POST_HEALTH,
            food: protocol::VarInt(SURVIVAL_HUNGER_FOOD_POST_FOOD),
            food_saturation: SURVIVAL_HUNGER_FOOD_POST_SATURATION,
        };
        let wrong_food = packet::play::clientbound::UpdateHealth {
            health: SURVIVAL_HUNGER_FOOD_PRE_HEALTH,
            food: protocol::VarInt(SURVIVAL_HUNGER_FOOD_POST_FOOD),
            food_saturation: SURVIVAL_HUNGER_FOOD_PRE_SATURATION,
        };

        assert!(survival_hunger_food_pre_update_matches(&pre));
        assert!(survival_hunger_food_post_update_matches(&post));
        assert!(!survival_hunger_food_pre_update_matches(&wrong_food));
        assert!(!survival_hunger_food_post_update_matches(&wrong_food));
        assert!(survival_hunger_food_float_matches(
            SURVIVAL_HUNGER_FOOD_POST_SATURATION,
            SURVIVAL_HUNGER_FOOD_POST_SATURATION
        ));
    }

    #[test]
    fn survival_hunger_health_updates_match_bounded_health_cycle_contract() {
        let pre = packet::play::clientbound::UpdateHealth {
            health: SURVIVAL_HUNGER_HEALTH_PRE_HEALTH,
            food: protocol::VarInt(SURVIVAL_HUNGER_HEALTH_PRE_FOOD),
            food_saturation: SURVIVAL_HUNGER_HEALTH_PRE_SATURATION,
        };
        let post = packet::play::clientbound::UpdateHealth {
            health: SURVIVAL_HUNGER_HEALTH_POST_HEALTH,
            food: protocol::VarInt(SURVIVAL_HUNGER_HEALTH_POST_FOOD),
            food_saturation: SURVIVAL_HUNGER_HEALTH_POST_SATURATION,
        };
        let wrong_health = packet::play::clientbound::UpdateHealth {
            health: SURVIVAL_HUNGER_FOOD_PRE_HEALTH,
            food: protocol::VarInt(SURVIVAL_HUNGER_HEALTH_PRE_FOOD),
            food_saturation: SURVIVAL_HUNGER_HEALTH_PRE_SATURATION,
        };
        let wrong_food = packet::play::clientbound::UpdateHealth {
            health: SURVIVAL_HUNGER_HEALTH_PRE_HEALTH,
            food: protocol::VarInt(SURVIVAL_HUNGER_HEALTH_POST_FOOD),
            food_saturation: SURVIVAL_HUNGER_HEALTH_PRE_SATURATION,
        };

        assert!(survival_hunger_health_pre_update_matches(&pre));
        assert!(survival_hunger_health_post_update_matches(&post));
        assert!(!survival_hunger_health_pre_update_matches(&wrong_health));
        assert!(!survival_hunger_health_pre_update_matches(&wrong_food));
        assert!(!survival_hunger_health_post_update_matches(&wrong_food));
    }

    #[test]
    fn survival_crafting_inventory_updates_use_player_inventory_window() {
        const CRAFTING_WINDOW_ID_FOR_TEST: u8 = 1;

        assert!(should_log_survival_crafting_inventory_slot(
            PLAYER_INVENTORY_WINDOW_ID,
            CRAFTING_WINDOW_ID_FOR_TEST,
            SURVIVAL_CRAFTING_INVENTORY_SLOT
        ));
        assert!(should_log_survival_crafting_inventory_slot(
            CRAFTING_WINDOW_ID_FOR_TEST,
            CRAFTING_WINDOW_ID_FOR_TEST,
            SURVIVAL_CRAFTING_OPEN_INVENTORY_MIRROR_SLOT
        ));
        assert!(!should_log_survival_crafting_inventory_slot(
            PLAYER_INVENTORY_WINDOW_ID,
            CRAFTING_WINDOW_ID_FOR_TEST,
            SURVIVAL_CRAFTING_INPUT_A_SLOT
        ));
        assert!(!should_log_survival_crafting_inventory_slot(
            CRAFTING_WINDOW_ID_FOR_TEST,
            CRAFTING_WINDOW_ID_FOR_TEST,
            SURVIVAL_CRAFTING_INVENTORY_SLOT
        ));
    }

    #[test]
    fn survival_crafting_inventory_indices_accept_player_and_open_mirror() {
        assert!(should_log_survival_crafting_inventory_index(
            SURVIVAL_CRAFTING_INVENTORY_INDEX
        ));
        assert!(should_log_survival_crafting_inventory_index(
            SURVIVAL_CRAFTING_OPEN_INVENTORY_MIRROR_INDEX
        ));
        assert!(!should_log_survival_crafting_inventory_index(
            SURVIVAL_CRAFTING_INPUT_A_SLOT as usize
        ));
    }

    #[test]
    fn survival_furnace_inventory_updates_accept_player_and_open_mirror_slots() {
        const FURNACE_WINDOW_ID_FOR_TEST: u8 = 1;

        assert!(should_log_survival_furnace_inventory_slot(
            PLAYER_INVENTORY_WINDOW_ID,
            FURNACE_WINDOW_ID_FOR_TEST,
            SURVIVAL_FURNACE_INVENTORY_SLOT
        ));
        assert!(should_log_survival_furnace_inventory_slot(
            FURNACE_WINDOW_ID_FOR_TEST,
            FURNACE_WINDOW_ID_FOR_TEST,
            SURVIVAL_FURNACE_OPEN_INVENTORY_MIRROR_SLOT
        ));
        assert!(!should_log_survival_furnace_inventory_slot(
            PLAYER_INVENTORY_WINDOW_ID,
            FURNACE_WINDOW_ID_FOR_TEST,
            SURVIVAL_FURNACE_OPEN_INVENTORY_MIRROR_SLOT
        ));
        assert!(!should_log_survival_furnace_inventory_slot(
            FURNACE_WINDOW_ID_FOR_TEST,
            FURNACE_WINDOW_ID_FOR_TEST,
            SURVIVAL_FURNACE_INVENTORY_SLOT
        ));
    }

    #[test]
    fn survival_furnace_inventory_indices_accept_player_and_open_mirror() {
        assert!(should_log_survival_furnace_inventory_index(
            SURVIVAL_FURNACE_INVENTORY_INDEX
        ));
        assert!(should_log_survival_furnace_inventory_index(
            SURVIVAL_FURNACE_OPEN_INVENTORY_MIRROR_INDEX
        ));
        assert!(!should_log_survival_furnace_inventory_index(
            SURVIVAL_FURNACE_OUTPUT_ITEM_ID as usize
        ));
    }

    #[test]
    fn survival_world_persistence_position_match_rejects_wrong_target() {
        let expected = Position::new(
            SURVIVAL_WORLD_PERSISTENCE_X,
            SURVIVAL_WORLD_PERSISTENCE_Y,
            SURVIVAL_WORLD_PERSISTENCE_Z,
        );
        let wrong = Position::new(
            SURVIVAL_WORLD_PERSISTENCE_X,
            SURVIVAL_WORLD_PERSISTENCE_Y,
            SURVIVAL_WORLD_PERSISTENCE_Z + 1,
        );

        assert!(survival_world_persistence_position_matches(expected));
        assert!(!survival_world_persistence_position_matches(wrong));
    }

    #[test]
    fn survival_block_entity_position_match_rejects_wrong_target() {
        let expected = Position::new(
            SURVIVAL_BLOCK_ENTITY_X,
            SURVIVAL_BLOCK_ENTITY_Y,
            SURVIVAL_BLOCK_ENTITY_Z,
        );
        let wrong = Position::new(
            SURVIVAL_BLOCK_ENTITY_X,
            SURVIVAL_BLOCK_ENTITY_Y,
            SURVIVAL_BLOCK_ENTITY_Z + 1,
        );

        assert!(survival_block_entity_position_matches(expected));
        assert!(!survival_block_entity_position_matches(wrong));
    }

    #[test]
    fn survival_block_entity_sign_lines_match_contract_payload() {
        let lines = survival_block_entity_expected_lines();
        let wrong = [
            lines[SIGN_LINE_INDEX_1].clone(),
            lines[SIGN_LINE_INDEX_2].clone(),
            lines[SIGN_LINE_INDEX_3].clone(),
            "Wrong".to_string(),
        ];

        assert!(sign_lines_match_payload(&lines));
        assert_eq!(
            sign_text_payload(&lines),
            SURVIVAL_BLOCK_ENTITY_TEXT_PAYLOAD
        );
        assert!(!sign_lines_match_payload(&wrong));
    }
}
