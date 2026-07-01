#![allow(clippy::type_complexity)]

mod fixture_core;
mod gameplay_contracts;
mod scenario_contracts_generated;

use fixture_core::survival as survival_core;
use gameplay_contracts::{
    gameplay_scope_matches, register_gameplay_plugin_contract, register_gameplay_plugin_template,
    GameplayArenaId, GameplayInstallMode, GameplayMode, GameplayPhase as SurvivalGameplayPhase,
    GameplayPluginContract, GameplayPluginTemplate, GameplayScheduleContract, GameplayScope,
    GameplayScopeModel, EVENT_LOOP_PRE_UPDATE_SCHEDULE_LABEL, GAMEPLAY_PHASE_ORDER,
    SURVIVAL_PRIMARY_ARENA_ID, UPDATE_SCHEDULE_LABEL,
};
use std::fs;
use std::path::PathBuf;

use valence::entity::iron_golem::IronGolemEntityBundle;
use valence::entity::item::{ItemEntityBundle, Stack as ItemEntityStack};
use valence::entity::living::Health;
use valence::entity::player::{Food, Saturation};
use valence::entity::{EntityId, EntityManager};
use valence::interact_block::InteractBlockEvent;
use valence::interact_entity::{EntityInteraction, InteractEntityEvent};
use valence::interact_item::InteractItemEvent;
use valence::inventory::{
    ClickSlotEvent, CloseHandledScreenEvent, CursorItem, HeldItem, OpenInventory, SlotChange,
};
use valence::log::info;
use valence::nbt::{compound, List, Value};
use valence::prelude::*;
use valence::protocol::packets::play::{BlockUpdateS2c, ItemPickupAnimationS2c};
use valence::protocol::{VarInt, WritePacket};

const CHUNK_RADIUS: i32 = 5;
const FLOOR_RADIUS: i32 = 16;
const SPAWN_Y: i32 = 65;
const FLOOR_Y: i32 = 64;
const SURVIVAL_TARGET_X: i32 = 0;
const SURVIVAL_TARGET_Z: i32 = 1;
const SURVIVAL_ITEM_SLOT: u16 = 36;
const SURVIVAL_PICKUP_ENTITY_ID: i32 = 7_630_101;
const SURVIVAL_PICKUP_COUNT: i32 = 1;
const SURVIVAL_BLOCK_COUNT: i8 = 1;
const SURVIVAL_SPAWN_X: f64 = 0.5;
const SURVIVAL_SPAWN_Z: f64 = 0.5;
const SURVIVAL_WELCOME: &str = "Welcome to the Valence survival compatibility fixture.";
const SURVIVAL_CHEST_FIXTURE_ENV: &str =
    scenario_contracts_generated::MC_COMPAT_SURVIVAL_CHEST_FIXTURE;
const SURVIVAL_CHEST_X: i32 = 8;
const SURVIVAL_CHEST_Y: i32 = FLOOR_Y;
const SURVIVAL_CHEST_Z: i32 = 0;
const SURVIVAL_CHEST_SLOT: u16 = 0;
const SURVIVAL_CHEST_SLOT_ID: i16 = 0;
const SURVIVAL_CHEST_WINDOW: u8 = 1;
const SURVIVAL_CHEST_ITEM_COUNT: i8 = 1;
const SURVIVAL_CHEST_TITLE: &str = "MC Compat Chest";
const SURVIVAL_CRAFTING_FIXTURE_ENV: &str = "MC_COMPAT_SURVIVAL_CRAFTING_FIXTURE";
const SURVIVAL_CRAFTING_BREADTH_FIXTURE_ENV: &str = "MC_COMPAT_SURVIVAL_CRAFTING_BREADTH_FIXTURE";
const SURVIVAL_CRAFTING_X: i32 = 4;
const SURVIVAL_CRAFTING_Y: i32 = FLOOR_Y;
const SURVIVAL_CRAFTING_Z: i32 = 0;
const SURVIVAL_CRAFTING_WINDOW: u8 = 1;
const SURVIVAL_CRAFTING_RESULT_SLOT: u16 = 0;
const SURVIVAL_CRAFTING_RESULT_SLOT_ID: i16 = 0;
const SURVIVAL_CRAFTING_INPUT_A_SLOT: u16 = 1;
const SURVIVAL_CRAFTING_INPUT_A_SLOT_ID: i16 = 1;
const SURVIVAL_CRAFTING_INPUT_B_SLOT: u16 = 4;
const SURVIVAL_CRAFTING_INPUT_B_SLOT_ID: i16 = 4;
const SURVIVAL_CRAFTING_INVENTORY_SLOT: u16 = 36;
const SURVIVAL_CRAFTING_INPUT_COUNT: i8 = 1;
const SURVIVAL_CRAFTING_TOTAL_INPUT_COUNT: i8 = 2;
const SURVIVAL_CRAFTING_RESULT_COUNT: i8 = 4;
const SURVIVAL_CRAFTING_RECIPE: &str = "minecraft:stick";
const SURVIVAL_CRAFTING_TITLE: &str = "MC Compat Crafting";
const SURVIVAL_FURNACE_FIXTURE_ENV: &str = "MC_COMPAT_SURVIVAL_FURNACE_FIXTURE";
const SURVIVAL_FURNACE_SMELTING_BREADTH_FIXTURE_ENV: &str =
    "MC_COMPAT_SURVIVAL_FURNACE_SMELTING_BREADTH_FIXTURE";
const SURVIVAL_FURNACE_X: i32 = 12;
const SURVIVAL_FURNACE_Y: i32 = FLOOR_Y;
const SURVIVAL_FURNACE_Z: i32 = 0;
const SURVIVAL_FURNACE_WINDOW: u8 = 1;
const SURVIVAL_FURNACE_INPUT_SLOT: u16 = 0;
const SURVIVAL_FURNACE_INPUT_SLOT_ID: i16 = 0;
const SURVIVAL_FURNACE_FUEL_SLOT: u16 = 1;
const SURVIVAL_FURNACE_FUEL_SLOT_ID: i16 = 1;
const SURVIVAL_FURNACE_OUTPUT_SLOT: u16 = 2;
const SURVIVAL_FURNACE_OUTPUT_SLOT_ID: i16 = 2;
const SURVIVAL_FURNACE_INVENTORY_SLOT: u16 = 36;
const SURVIVAL_FURNACE_ITEM_COUNT: i8 = 1;
const SURVIVAL_FURNACE_TITLE: &str = "MC Compat Furnace";
const SURVIVAL_FURNACE_INPUT_NAME: &str = "RawIron";
const SURVIVAL_FURNACE_FUEL_NAME: &str = "Coal";
const SURVIVAL_FURNACE_OUTPUT_NAME: &str = "IronIngot";
const SURVIVAL_FURNACE_SMELTING_RECIPE: &str = "minecraft:iron_ingot";
const SURVIVAL_FURNACE_INVALID_FUEL_OUTCOME: &str = "no_burn";
const SURVIVAL_FURNACE_SMELTING_PLUGIN_NAME: &str = "SurvivalFurnaceSmeltingPlugin";
const SURVIVAL_FURNACE_RECIPE_TABLE_RESOURCE_NAME: &str = "SurvivalFurnaceRecipeTableResource";
const SURVIVAL_FURNACE_FUEL_TABLE_RESOURCE_NAME: &str = "SurvivalFurnaceFuelTableResource";
const SURVIVAL_FURNACE_CONFIG_RESOURCE_NAME: &str = "SurvivalFurnaceSmeltingConfigResource";
const SURVIVAL_FURNACE_BLOCK_ENTITY_RESOURCE_NAME: &str = "SurvivalFurnaceBlockEntity";
const SURVIVAL_FURNACE_STATE_CHANGED_EVENT_NAME: &str = "SurvivalFurnaceStateChangedEvent";
const SURVIVAL_FURNACE_DIAGNOSTIC_EVENT_NAME: &str = "SurvivalFurnaceDiagnosticEvent";
const SURVIVAL_HUNGER_FOOD_FIXTURE_ENV: &str = "MC_COMPAT_SURVIVAL_HUNGER_FOOD_FIXTURE";
const SURVIVAL_HUNGER_FOOD_INVENTORY_SLOT: u16 = 36;
const SURVIVAL_HUNGER_FOOD_ITEM_COUNT_BEFORE: i8 = 1;
const SURVIVAL_HUNGER_FOOD_ITEM_COUNT_AFTER: i8 = 0;
const SURVIVAL_HUNGER_FOOD_ITEM_NAME: &str = "Bread";
const SURVIVAL_HUNGER_FOOD_PRE_HEALTH: f32 = 20.0;
const SURVIVAL_HUNGER_FOOD_PRE_FOOD: i32 = 15;
const SURVIVAL_HUNGER_FOOD_PRE_SATURATION: f32 = 0.0;
const SURVIVAL_HUNGER_FOOD_POST_HEALTH: f32 = 20.0;
const SURVIVAL_HUNGER_FOOD_POST_FOOD: i32 = 20;
const SURVIVAL_HUNGER_FOOD_POST_SATURATION: f32 = 6.0;
const SURVIVAL_HUNGER_HEALTH_FIXTURE_ENV: &str = "MC_COMPAT_SURVIVAL_HUNGER_HEALTH_FIXTURE";
const SURVIVAL_HUNGER_HEALTH_PRE_HEALTH: f32 = 18.0;
const SURVIVAL_HUNGER_HEALTH_POST_HEALTH: f32 = 20.0;
const SURVIVAL_HUNGER_FOOD_EVENT_PREFIX: &str = "survival_hunger_food";
const SURVIVAL_HUNGER_HEALTH_EVENT_PREFIX: &str = "survival_hunger_health";
const SURVIVAL_HUNGER_FOOD_USE_SEQUENCE: i32 = 810;
const SURVIVAL_MOB_DROP_FIXTURE_ENV: &str = "MC_COMPAT_SURVIVAL_MOB_DROP_FIXTURE";
const SURVIVAL_MOB_AI_LOOT_FIXTURE_ENV: &str = "MC_COMPAT_SURVIVAL_MOB_AI_LOOT_FIXTURE";
const SURVIVAL_MOB_DROP_MOB_NAME: &str = "IronGolem";
const SURVIVAL_MOB_DROP_ITEM_NAME: &str = "IronIngot";
const SURVIVAL_MOB_DROP_MOB_X: f64 = 16.5;
const SURVIVAL_MOB_DROP_MOB_Y: f64 = 65.0;
const SURVIVAL_MOB_DROP_MOB_Z: f64 = 2.5;
const SURVIVAL_MOB_DROP_DAMAGE: f32 = 20.0;
const SURVIVAL_MOB_DROP_ITEM_COUNT: i8 = 1;
const SURVIVAL_MOB_DROP_INVENTORY_SLOT: u16 = 36;
const SURVIVAL_MOB_DROP_PICKUP_DELAY_TICKS: u8 = 2;
const SURVIVAL_REDSTONE_TOGGLE_FIXTURE_ENV: &str = "MC_COMPAT_SURVIVAL_REDSTONE_TOGGLE_FIXTURE";
const SURVIVAL_REDSTONE_CIRCUIT_FIXTURE_ENV: &str = "MC_COMPAT_SURVIVAL_REDSTONE_CIRCUIT_FIXTURE";
const SURVIVAL_REDSTONE_TOGGLE_CONTROL_NAME: &str = "Lever";
const SURVIVAL_REDSTONE_TOGGLE_OUTPUT_NAME: &str = "RedstoneLamp";
const SURVIVAL_REDSTONE_TOGGLE_CONTROL_X: i32 = 20;
const SURVIVAL_REDSTONE_TOGGLE_CONTROL_Y: i32 = FLOOR_Y;
const SURVIVAL_REDSTONE_TOGGLE_CONTROL_Z: i32 = 0;
const SURVIVAL_REDSTONE_TOGGLE_OUTPUT_X: i32 = 21;
const SURVIVAL_REDSTONE_TOGGLE_OUTPUT_Y: i32 = FLOOR_Y;
const SURVIVAL_REDSTONE_TOGGLE_OUTPUT_Z: i32 = 0;
const SURVIVAL_REDSTONE_TOGGLE_FLOOR_Y: i32 = 63;
const SURVIVAL_REDSTONE_TOGGLE_ARENA_MIN_X: i32 = 19;
const SURVIVAL_REDSTONE_TOGGLE_ARENA_MAX_X: i32 = 23;
const SURVIVAL_REDSTONE_TOGGLE_ARENA_MIN_Z: i32 = -2;
const SURVIVAL_REDSTONE_TOGGLE_ARENA_MAX_Z: i32 = 2;
const SURVIVAL_REDSTONE_TOGGLE_PLAYER_X: f64 = 20.5;
const SURVIVAL_REDSTONE_TOGGLE_PLAYER_Y: f64 = 65.0;
const SURVIVAL_REDSTONE_TOGGLE_PLAYER_Z: f64 = -1.5;
const SURVIVAL_WORLD_PERSISTENCE_FIXTURE_ENV: &str = "MC_COMPAT_SURVIVAL_WORLD_PERSISTENCE_FIXTURE";
const SURVIVAL_WORLD_PERSISTENCE_DIR_ENV: &str = "MC_COMPAT_SURVIVAL_WORLD_PERSISTENCE_DIR";
const SURVIVAL_WORLD_PERSISTENCE_MARKER_FILE: &str = "persisted-dirt.marker";
const SURVIVAL_WORLD_PERSISTENCE_BLOCK_NAME: &str = "Dirt";
const SURVIVAL_WORLD_PERSISTENCE_X: i32 = 24;
const SURVIVAL_WORLD_PERSISTENCE_Y: i32 = FLOOR_Y;
const SURVIVAL_WORLD_PERSISTENCE_Z: i32 = 0;
const SURVIVAL_WORLD_PERSISTENCE_BASE_Y: i32 = 63;
const SURVIVAL_WORLD_PERSISTENCE_PLAYER_X: f64 = 24.5;
const SURVIVAL_WORLD_PERSISTENCE_PLAYER_Y: f64 = 65.0;
const SURVIVAL_WORLD_PERSISTENCE_PLAYER_Z: f64 = -1.5;
const SURVIVAL_WORLD_PERSISTENCE_INVENTORY_SLOT: u16 = 36;
const SURVIVAL_WORLD_PERSISTENCE_ITEM_COUNT: i8 = 1;
const SURVIVAL_BLOCK_ENTITY_FIXTURE_ENV: &str = "MC_COMPAT_SURVIVAL_BLOCK_ENTITY_FIXTURE";
const SURVIVAL_BLOCK_ENTITY_DIR_ENV: &str = "MC_COMPAT_SURVIVAL_BLOCK_ENTITY_DIR";
const SURVIVAL_BLOCK_ENTITY_PHASE_ENV: &str = "MC_COMPAT_SURVIVAL_BLOCK_ENTITY_PHASE";
const SURVIVAL_BLOCK_ENTITY_POST_RESTART_PHASE: &str = "post_restart";
const SURVIVAL_BLOCK_ENTITY_MARKER_FILE: &str = "persisted-sign.marker";
const SURVIVAL_BLOCK_ENTITY_KIND: &str = "Sign";
const SURVIVAL_BLOCK_ENTITY_X: i32 = 28;
const SURVIVAL_BLOCK_ENTITY_Y: i32 = FLOOR_Y;
const SURVIVAL_BLOCK_ENTITY_Z: i32 = 0;
const SURVIVAL_BLOCK_ENTITY_BASE_Y: i32 = 63;
const SURVIVAL_BLOCK_ENTITY_PLAYER_X: f64 = 28.5;
const SURVIVAL_BLOCK_ENTITY_PLAYER_Y: f64 = 65.0;
const SURVIVAL_BLOCK_ENTITY_PLAYER_Z: f64 = -1.5;
const SURVIVAL_BLOCK_ENTITY_TEXT_LINE_1: &str = "MC";
const SURVIVAL_BLOCK_ENTITY_TEXT_LINE_2: &str = "Compat";
const SURVIVAL_BLOCK_ENTITY_TEXT_LINE_3: &str = "Sign";
const SURVIVAL_BLOCK_ENTITY_TEXT_LINE_4: &str = "Persist";
const SURVIVAL_BLOCK_ENTITY_TEXT_LINE_COUNT: usize = 4;
const SURVIVAL_BLOCK_ENTITY_TEXT_PAYLOAD: &str = "MC|Compat|Sign|Persist";
const SURVIVAL_WORLD_MULTICHUNK_FIXTURE_ENV: &str = "MC_COMPAT_SURVIVAL_WORLD_MULTICHUNK_FIXTURE";
const SURVIVAL_WORLD_MULTICHUNK_PHASE_ENV: &str = "MC_COMPAT_SURVIVAL_WORLD_MULTICHUNK_PHASE";
const SURVIVAL_CONTAINER_BLOCK_ENTITY_FIXTURE_ENV: &str =
    "MC_COMPAT_SURVIVAL_CONTAINER_BLOCK_ENTITY_FIXTURE";
const SURVIVAL_SIGN_EDITING_FIXTURE_ENV: &str = "MC_COMPAT_SURVIVAL_SIGN_EDITING_FIXTURE";
const SURVIVAL_BIOME_DIMENSION_FIXTURE_ENV: &str = "MC_COMPAT_SURVIVAL_BIOME_DIMENSION_FIXTURE";
const SURVIVAL_BIOME_DIMENSION_TRAVEL_FIXTURE_ENV: &str =
    "MC_COMPAT_SURVIVAL_BIOME_DIMENSION_TRAVEL_FIXTURE";
const SURVIVAL_KNOWN_ENVIRONMENT_COUNT: usize = 3;
const SURVIVAL_CORE_TENTHS_SCALE: f32 = 10.0;
const SURVIVAL_OTHER_ITEM_NAME: &str = "other";
const SURVIVAL_OVERWORLD_ID: &str = "minecraft:overworld";
const SURVIVAL_NETHER_ID: &str = "minecraft:the_nether";
const SURVIVAL_END_ID: &str = "minecraft:the_end";
const SURVIVAL_UNKNOWN_ENVIRONMENT_ID: &str = "unknown";
const SURVIVAL_ENV_FLAG_ENABLED_VALUE: &str = "1";
const SURVIVAL_COMPATIBILITY_PLUGIN_NAME: &str = "SurvivalCompatibilityPlugin";
const SURVIVAL_RUNTIME_CONFIG_SOURCE_PLUGIN_NAME: &str = "SurvivalRuntimeConfigSourcePlugin";
const SURVIVAL_RUNTIME_CONFIG_RELOAD_EVENT_NAME: &str = "SurvivalRuntimeConfigReloadEvent";
const SURVIVAL_PRIMARY_SCOPE: GameplayScope = GameplayScope::new(
    GameplayMode::Survival,
    GameplayArenaId::new(SURVIVAL_PRIMARY_ARENA_ID),
);
const SURVIVAL_GAMEPLAY_PHASE_ORDER: &[SurvivalGameplayPhase] = GAMEPLAY_PHASE_ORDER;
const SURVIVAL_GAMEPLAY_SCHEDULES: &[GameplayScheduleContract] = &[
    GameplayScheduleContract {
        label: UPDATE_SCHEDULE_LABEL,
        phases: SURVIVAL_GAMEPLAY_PHASE_ORDER,
    },
    GameplayScheduleContract {
        label: EVENT_LOOP_PRE_UPDATE_SCHEDULE_LABEL,
        phases: SURVIVAL_GAMEPLAY_PHASE_ORDER,
    },
];
const SURVIVAL_SOURCE_SCHEDULES: &[GameplayScheduleContract] = SURVIVAL_GAMEPLAY_SCHEDULES;
const SURVIVAL_GAMEPLAY_OWNED_RESOURCES: &[&str] = &[
    "SurvivalChestFixture",
    "SurvivalCompatibilityPluginContract",
    "SurvivalCraftingFixture",
    "SurvivalFurnaceFixture",
    "SurvivalHungerFoodFixture",
    "SurvivalMobDropFixture",
];
const SURVIVAL_SOURCE_OWNED_RESOURCES: &[&str] = &["SurvivalRuntimeConfig"];
const SURVIVAL_SOURCE_OWNED_EVENTS: &[&str] = &[SURVIVAL_RUNTIME_CONFIG_RELOAD_EVENT_NAME];
const SURVIVAL_NO_OWNED_EVENTS: &[&str] = &[];
const SURVIVAL_NON_CLAIMS: &[&str] = &[
    "dynamic runtime plugins",
    "default Valence gameplay",
    "BedWars or Hyperion scope",
    "vanilla parity",
    "production readiness",
];
const SURVIVAL_GAMEPLAY_CONTRACT: GameplayPluginContract = GameplayPluginContract {
    plugin: SURVIVAL_COMPATIBILITY_PLUGIN_NAME,
    install_mode: GameplayInstallMode::ExplicitOptIn,
    scope_model: GameplayScopeModel::LayerOwnedFixture,
    scope: Some(SURVIVAL_PRIMARY_SCOPE),
    schedules: SURVIVAL_GAMEPLAY_SCHEDULES,
    owned_resources: SURVIVAL_GAMEPLAY_OWNED_RESOURCES,
    owned_events: SURVIVAL_NO_OWNED_EVENTS,
    non_claims: SURVIVAL_NON_CLAIMS,
};
const SURVIVAL_RUNTIME_CONFIG_SOURCE_CONTRACT: GameplayPluginContract = GameplayPluginContract {
    plugin: SURVIVAL_RUNTIME_CONFIG_SOURCE_PLUGIN_NAME,
    install_mode: GameplayInstallMode::SourceAdapter,
    scope_model: GameplayScopeModel::SourceOnly,
    scope: None,
    schedules: SURVIVAL_SOURCE_SCHEDULES,
    owned_resources: SURVIVAL_SOURCE_OWNED_RESOURCES,
    owned_events: SURVIVAL_SOURCE_OWNED_EVENTS,
    non_claims: SURVIVAL_NON_CLAIMS,
};
const SURVIVAL_FURNACE_SMELTING_SCHEDULES: &[GameplayScheduleContract] =
    &[GameplayScheduleContract {
        label: UPDATE_SCHEDULE_LABEL,
        phases: SURVIVAL_GAMEPLAY_PHASE_ORDER,
    }];
const SURVIVAL_FURNACE_SMELTING_OWNED_RESOURCES: &[&str] = &[
    SURVIVAL_FURNACE_RECIPE_TABLE_RESOURCE_NAME,
    SURVIVAL_FURNACE_FUEL_TABLE_RESOURCE_NAME,
    SURVIVAL_FURNACE_CONFIG_RESOURCE_NAME,
    SURVIVAL_FURNACE_BLOCK_ENTITY_RESOURCE_NAME,
];
const SURVIVAL_FURNACE_SMELTING_OWNED_EVENTS: &[&str] = &[
    SURVIVAL_FURNACE_STATE_CHANGED_EVENT_NAME,
    SURVIVAL_FURNACE_DIAGNOSTIC_EVENT_NAME,
];
const SURVIVAL_FURNACE_SMELTING_NON_CLAIMS: &[&str] = &[
    "DefaultPlugins membership change",
    "all-recipe breadth",
    "all-fuel breadth",
    "smoker behavior",
    "blast-furnace behavior",
    "hopper automation",
    "XP behavior",
    "recipe-book synchronization",
    "chunk-unload semantics",
    "broad vanilla parity",
    "public-server safety",
    "production readiness",
];
const SURVIVAL_FURNACE_SMELTING_CONTRACT: GameplayPluginContract = GameplayPluginContract {
    plugin: SURVIVAL_FURNACE_SMELTING_PLUGIN_NAME,
    install_mode: GameplayInstallMode::ExplicitOptIn,
    scope_model: GameplayScopeModel::ArenaOwnedLayer,
    scope: Some(SURVIVAL_PRIMARY_SCOPE),
    schedules: SURVIVAL_FURNACE_SMELTING_SCHEDULES,
    owned_resources: SURVIVAL_FURNACE_SMELTING_OWNED_RESOURCES,
    owned_events: SURVIVAL_FURNACE_SMELTING_OWNED_EVENTS,
    non_claims: SURVIVAL_FURNACE_SMELTING_NON_CLAIMS,
};

#[derive(Resource, Clone, Debug, PartialEq, Eq)]
struct SurvivalRuntimeConfig {
    fixtures: SurvivalFixtureConfig,
    paths: SurvivalPathConfig,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct SurvivalFixtureConfig {
    chest: bool,
    crafting: bool,
    crafting_breadth: bool,
    furnace: bool,
    furnace_smelting_breadth: bool,
    hunger_food: bool,
    hunger_health: bool,
    mob_drop: bool,
    mob_ai_loot: bool,
    redstone_toggle: bool,
    redstone_circuit: bool,
    world_persistence: bool,
    block_entity: bool,
    block_entity_post_restart: bool,
    world_multichunk: bool,
    world_multichunk_post_restart: bool,
    container_block_entity: bool,
    sign_editing: bool,
    biome_dimension: bool,
    biome_dimension_travel: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct SurvivalPathConfig {
    world_persistence_marker: PathBuf,
    block_entity_marker: PathBuf,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct SurvivalRuntimeConfigInputs {
    chest_fixture: Option<String>,
    crafting_fixture: Option<String>,
    crafting_breadth_fixture: Option<String>,
    furnace_fixture: Option<String>,
    furnace_smelting_breadth_fixture: Option<String>,
    hunger_food_fixture: Option<String>,
    hunger_health_fixture: Option<String>,
    mob_drop_fixture: Option<String>,
    mob_ai_loot_fixture: Option<String>,
    redstone_toggle_fixture: Option<String>,
    redstone_circuit_fixture: Option<String>,
    world_persistence_fixture: Option<String>,
    world_persistence_dir: Option<String>,
    block_entity_fixture: Option<String>,
    block_entity_dir: Option<String>,
    block_entity_phase: Option<String>,
    world_multichunk_fixture: Option<String>,
    world_multichunk_phase: Option<String>,
    container_block_entity_fixture: Option<String>,
    sign_editing_fixture: Option<String>,
    biome_dimension_fixture: Option<String>,
    biome_dimension_travel_fixture: Option<String>,
    temp_dir: PathBuf,
}

#[derive(Event, Clone, Debug, Default, PartialEq, Eq)]
struct SurvivalRuntimeConfigReloadEvent {
    inputs: Option<SurvivalRuntimeConfigInputs>,
}

impl SurvivalRuntimeConfigReloadEvent {
    #[cfg(test)]
    fn from_inputs(inputs: SurvivalRuntimeConfigInputs) -> Self {
        Self {
            inputs: Some(inputs),
        }
    }

    fn config(&self) -> SurvivalRuntimeConfig {
        match &self.inputs {
            Some(inputs) => parse_survival_runtime_config(inputs),
            None => SurvivalRuntimeConfig::from_env(),
        }
    }
}

impl Default for SurvivalRuntimeConfigInputs {
    fn default() -> Self {
        Self {
            chest_fixture: None,
            crafting_fixture: None,
            crafting_breadth_fixture: None,
            furnace_fixture: None,
            furnace_smelting_breadth_fixture: None,
            hunger_food_fixture: None,
            hunger_health_fixture: None,
            mob_drop_fixture: None,
            mob_ai_loot_fixture: None,
            redstone_toggle_fixture: None,
            redstone_circuit_fixture: None,
            world_persistence_fixture: None,
            world_persistence_dir: None,
            block_entity_fixture: None,
            block_entity_dir: None,
            block_entity_phase: None,
            world_multichunk_fixture: None,
            world_multichunk_phase: None,
            container_block_entity_fixture: None,
            sign_editing_fixture: None,
            biome_dimension_fixture: None,
            biome_dimension_travel_fixture: None,
            temp_dir: std::env::temp_dir(),
        }
    }
}

impl Default for SurvivalRuntimeConfig {
    fn default() -> Self {
        parse_survival_runtime_config(&SurvivalRuntimeConfigInputs::default())
    }
}

impl SurvivalRuntimeConfig {
    fn from_env() -> Self {
        parse_survival_runtime_config(&SurvivalRuntimeConfigInputs::from_env())
    }
}

impl SurvivalRuntimeConfigInputs {
    fn from_env() -> Self {
        Self {
            chest_fixture: std::env::var(SURVIVAL_CHEST_FIXTURE_ENV).ok(),
            crafting_fixture: std::env::var(SURVIVAL_CRAFTING_FIXTURE_ENV).ok(),
            crafting_breadth_fixture: std::env::var(SURVIVAL_CRAFTING_BREADTH_FIXTURE_ENV).ok(),
            furnace_fixture: std::env::var(SURVIVAL_FURNACE_FIXTURE_ENV).ok(),
            furnace_smelting_breadth_fixture: std::env::var(
                SURVIVAL_FURNACE_SMELTING_BREADTH_FIXTURE_ENV,
            )
            .ok(),
            hunger_food_fixture: std::env::var(SURVIVAL_HUNGER_FOOD_FIXTURE_ENV).ok(),
            hunger_health_fixture: std::env::var(SURVIVAL_HUNGER_HEALTH_FIXTURE_ENV).ok(),
            mob_drop_fixture: std::env::var(SURVIVAL_MOB_DROP_FIXTURE_ENV).ok(),
            mob_ai_loot_fixture: std::env::var(SURVIVAL_MOB_AI_LOOT_FIXTURE_ENV).ok(),
            redstone_toggle_fixture: std::env::var(SURVIVAL_REDSTONE_TOGGLE_FIXTURE_ENV).ok(),
            redstone_circuit_fixture: std::env::var(SURVIVAL_REDSTONE_CIRCUIT_FIXTURE_ENV).ok(),
            world_persistence_fixture: std::env::var(SURVIVAL_WORLD_PERSISTENCE_FIXTURE_ENV).ok(),
            world_persistence_dir: std::env::var(SURVIVAL_WORLD_PERSISTENCE_DIR_ENV).ok(),
            block_entity_fixture: std::env::var(SURVIVAL_BLOCK_ENTITY_FIXTURE_ENV).ok(),
            block_entity_dir: std::env::var(SURVIVAL_BLOCK_ENTITY_DIR_ENV).ok(),
            block_entity_phase: std::env::var(SURVIVAL_BLOCK_ENTITY_PHASE_ENV).ok(),
            world_multichunk_fixture: std::env::var(SURVIVAL_WORLD_MULTICHUNK_FIXTURE_ENV).ok(),
            world_multichunk_phase: std::env::var(SURVIVAL_WORLD_MULTICHUNK_PHASE_ENV).ok(),
            container_block_entity_fixture: std::env::var(
                SURVIVAL_CONTAINER_BLOCK_ENTITY_FIXTURE_ENV,
            )
            .ok(),
            sign_editing_fixture: std::env::var(SURVIVAL_SIGN_EDITING_FIXTURE_ENV).ok(),
            biome_dimension_fixture: std::env::var(SURVIVAL_BIOME_DIMENSION_FIXTURE_ENV).ok(),
            biome_dimension_travel_fixture: std::env::var(
                SURVIVAL_BIOME_DIMENSION_TRAVEL_FIXTURE_ENV,
            )
            .ok(),
            temp_dir: std::env::temp_dir(),
        }
    }
}

fn parse_survival_runtime_config(inputs: &SurvivalRuntimeConfigInputs) -> SurvivalRuntimeConfig {
    let furnace_smelting_breadth =
        parse_survival_enabled_flag(inputs.furnace_smelting_breadth_fixture.as_deref());
    SurvivalRuntimeConfig {
        fixtures: SurvivalFixtureConfig {
            chest: parse_survival_enabled_flag(inputs.chest_fixture.as_deref()),
            crafting: parse_survival_enabled_flag(inputs.crafting_fixture.as_deref()),
            crafting_breadth: parse_survival_enabled_flag(
                inputs.crafting_breadth_fixture.as_deref(),
            ),
            furnace: parse_survival_enabled_flag(inputs.furnace_fixture.as_deref())
                || furnace_smelting_breadth,
            furnace_smelting_breadth,
            hunger_food: parse_survival_enabled_flag(inputs.hunger_food_fixture.as_deref()),
            hunger_health: parse_survival_enabled_flag(inputs.hunger_health_fixture.as_deref()),
            mob_drop: parse_survival_enabled_flag(inputs.mob_drop_fixture.as_deref()),
            mob_ai_loot: parse_survival_enabled_flag(inputs.mob_ai_loot_fixture.as_deref()),
            redstone_toggle: parse_survival_enabled_flag(inputs.redstone_toggle_fixture.as_deref()),
            redstone_circuit: parse_survival_enabled_flag(
                inputs.redstone_circuit_fixture.as_deref(),
            ),
            world_persistence: parse_survival_enabled_flag(
                inputs.world_persistence_fixture.as_deref(),
            ),
            block_entity: parse_survival_enabled_flag(inputs.block_entity_fixture.as_deref()),
            block_entity_post_restart: parse_survival_post_restart_phase(
                inputs.block_entity_phase.as_deref(),
            ),
            world_multichunk: parse_survival_enabled_flag(
                inputs.world_multichunk_fixture.as_deref(),
            ),
            world_multichunk_post_restart: parse_survival_post_restart_phase(
                inputs.world_multichunk_phase.as_deref(),
            ),
            container_block_entity: parse_survival_enabled_flag(
                inputs.container_block_entity_fixture.as_deref(),
            ),
            sign_editing: parse_survival_enabled_flag(inputs.sign_editing_fixture.as_deref()),
            biome_dimension: parse_survival_enabled_flag(inputs.biome_dimension_fixture.as_deref()),
            biome_dimension_travel: parse_survival_enabled_flag(
                inputs.biome_dimension_travel_fixture.as_deref(),
            ),
        },
        paths: SurvivalPathConfig {
            world_persistence_marker: survival_marker_path(
                inputs.world_persistence_dir.as_deref(),
                &inputs.temp_dir,
                "mc-compat-world-persistence",
                SURVIVAL_WORLD_PERSISTENCE_MARKER_FILE,
            ),
            block_entity_marker: survival_marker_path(
                inputs.block_entity_dir.as_deref(),
                &inputs.temp_dir,
                "mc-compat-block-entity-persistence",
                SURVIVAL_BLOCK_ENTITY_MARKER_FILE,
            ),
        },
    }
}

fn parse_survival_enabled_flag(value: Option<&str>) -> bool {
    survival_core::enabled_flag(value, SURVIVAL_ENV_FLAG_ENABLED_VALUE)
}

fn parse_survival_post_restart_phase(value: Option<&str>) -> bool {
    survival_core::post_restart_phase(value, SURVIVAL_BLOCK_ENTITY_POST_RESTART_PHASE)
}

fn survival_marker_path(
    configured_dir: Option<&str>,
    temp_dir: &std::path::Path,
    default_dir_name: &str,
    marker_file: &str,
) -> PathBuf {
    survival_core::marker_path(configured_dir, temp_dir, default_dir_name, marker_file)
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SurvivalRuntimeConfigIssue {
    ConflictingHungerFixtures,
    StaleBlockEntityPhase,
    StaleWorldMultichunkPhase,
}

#[cfg(test)]
fn survival_runtime_config_issues(
    config: &SurvivalRuntimeConfig,
) -> Vec<SurvivalRuntimeConfigIssue> {
    survival_core::runtime_config_issues(survival_core::RuntimeFixtureFlags {
        hunger_food: config.fixtures.hunger_food,
        hunger_health: config.fixtures.hunger_health,
        block_entity: config.fixtures.block_entity,
        block_entity_post_restart: config.fixtures.block_entity_post_restart,
        world_multichunk: config.fixtures.world_multichunk,
        world_multichunk_post_restart: config.fixtures.world_multichunk_post_restart,
    })
    .into_iter()
    .map(|issue| match issue {
        survival_core::RuntimeConfigIssue::ConflictingHungerFixtures => {
            SurvivalRuntimeConfigIssue::ConflictingHungerFixtures
        }
        survival_core::RuntimeConfigIssue::StaleBlockEntityPhase => {
            SurvivalRuntimeConfigIssue::StaleBlockEntityPhase
        }
        survival_core::RuntimeConfigIssue::StaleWorldMultichunkPhase => {
            SurvivalRuntimeConfigIssue::StaleWorldMultichunkPhase
        }
    })
    .collect()
}

#[derive(Resource)]
struct SurvivalChestFixture {
    inventory: Entity,
    open_logged: bool,
    store_logged: bool,
    close_logged: bool,
    reopen_logged: bool,
    persisted_logged: bool,
}

impl SurvivalChestFixture {
    fn new(inventory: Entity) -> Self {
        Self {
            inventory,
            open_logged: false,
            store_logged: false,
            close_logged: false,
            reopen_logged: false,
            persisted_logged: false,
        }
    }
}

#[derive(Resource)]
struct SurvivalCraftingFixture {
    inventory: Entity,
    open_logged: bool,
    input_a_logged: bool,
    input_b_logged: bool,
    result_logged: bool,
    collect_logged: bool,
}

impl SurvivalCraftingFixture {
    fn new(inventory: Entity) -> Self {
        Self {
            inventory,
            open_logged: false,
            input_a_logged: false,
            input_b_logged: false,
            result_logged: false,
            collect_logged: false,
        }
    }
}

#[derive(Resource, Default)]
struct SurvivalCraftingBreadthFixture {
    logged: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SurvivalFurnaceShellDiagnostic {
    PausedNoFuel,
    PausedNoRecipe,
    PausedOutputBlocked,
    UnsupportedFurnaceKind,
    MalformedRecipeRow,
    MalformedFuelRow,
    MissingInventory,
    StaleBlockEntity,
}

#[derive(Event, Clone, Copy, Debug, Eq, PartialEq)]
struct SurvivalFurnaceStateChangedEvent {
    inventory: Entity,
    transition: survival_core::FurnaceTransition,
}

#[derive(Event, Clone, Copy, Debug, Eq, PartialEq)]
struct SurvivalFurnaceDiagnosticEvent {
    inventory: Option<Entity>,
    diagnostic: SurvivalFurnaceShellDiagnostic,
}

#[derive(Resource, Clone, Debug, Eq, PartialEq)]
struct SurvivalFurnaceRecipeTableResource {
    rows: [survival_core::FurnaceRecipeRow<'static>; 1],
}

impl Default for SurvivalFurnaceRecipeTableResource {
    fn default() -> Self {
        Self {
            rows: survival_core::selected_recipe_rows(),
        }
    }
}

#[derive(Resource, Clone, Debug, Eq, PartialEq)]
struct SurvivalFurnaceFuelTableResource {
    rows: [survival_core::FurnaceFuelRow<'static>; 1],
}

impl Default for SurvivalFurnaceFuelTableResource {
    fn default() -> Self {
        Self {
            rows: survival_core::selected_fuel_rows(),
        }
    }
}

#[derive(Resource, Clone, Copy, Debug, Eq, PartialEq)]
struct SurvivalFurnaceSmeltingConfigResource {
    enabled: bool,
    limits: survival_core::FurnaceLimits,
}

impl Default for SurvivalFurnaceSmeltingConfigResource {
    fn default() -> Self {
        Self {
            enabled: true,
            limits: survival_core::selected_limits(),
        }
    }
}

#[derive(Resource, Clone, Copy, Debug, Eq, PartialEq)]
struct SurvivalFurnaceBlockEntity {
    kind: survival_core::FurnaceKind,
    loaded: bool,
}

impl Default for SurvivalFurnaceBlockEntity {
    fn default() -> Self {
        Self {
            kind: survival_core::FurnaceKind::Standard,
            loaded: true,
        }
    }
}

#[derive(Resource)]
struct SurvivalFurnaceFixture {
    inventory: Entity,
    open_logged: bool,
    input_logged: bool,
    fuel_logged: bool,
    burn_logged: bool,
    output_logged: bool,
    collect_logged: bool,
    smelting_breadth_enabled: bool,
    invalid_fuel_logged: bool,
    breadth_state_logged: bool,
    reopen_logged: bool,
    state_logged: bool,
    cook_progress_ticks: u32,
    remaining_burn_ticks: u32,
    recipes_completed: u32,
    last_shell_transition: Option<survival_core::FurnaceTransition>,
    last_shell_diagnostic: Option<SurvivalFurnaceShellDiagnostic>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SurvivalContainerKind {
    Chest,
    Crafting,
    Furnace,
}

#[derive(Clone, Copy, Component, Debug, Eq, PartialEq)]
struct SurvivalOpenContainer {
    kind: SurvivalContainerKind,
}

impl SurvivalOpenContainer {
    fn new(kind: SurvivalContainerKind) -> Self {
        Self { kind }
    }
}

fn survival_container_is_open(
    open_container: Option<&SurvivalOpenContainer>,
    expected_kind: SurvivalContainerKind,
) -> bool {
    open_container.is_some_and(|container| container.kind == expected_kind)
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct SurvivalHungerProfile {
    event_prefix: &'static str,
    pre_health: f32,
    post_health: f32,
}

const SURVIVAL_HUNGER_FOOD_PROFILE: SurvivalHungerProfile = SurvivalHungerProfile {
    event_prefix: SURVIVAL_HUNGER_FOOD_EVENT_PREFIX,
    pre_health: SURVIVAL_HUNGER_FOOD_PRE_HEALTH,
    post_health: SURVIVAL_HUNGER_FOOD_POST_HEALTH,
};
const SURVIVAL_HUNGER_HEALTH_PROFILE: SurvivalHungerProfile = SurvivalHungerProfile {
    event_prefix: SURVIVAL_HUNGER_HEALTH_EVENT_PREFIX,
    pre_health: SURVIVAL_HUNGER_HEALTH_PRE_HEALTH,
    post_health: SURVIVAL_HUNGER_HEALTH_POST_HEALTH,
};

#[derive(Resource)]
struct SurvivalHungerFoodFixture {
    profile: SurvivalHungerProfile,
    pre_logged: bool,
    consume_start_logged: bool,
    consume_finish_logged: bool,
    inventory_logged: bool,
    state_logged: bool,
}

impl SurvivalHungerFoodFixture {
    fn new(profile: SurvivalHungerProfile) -> Self {
        Self {
            profile,
            pre_logged: false,
            consume_start_logged: false,
            consume_finish_logged: false,
            inventory_logged: false,
            state_logged: false,
        }
    }
}

#[derive(Resource)]
struct SurvivalMobDropFixture {
    mob: Entity,
    mob_id: i32,
    spawn_logged: bool,
    attack_logged: bool,
    death_logged: bool,
    drop_logged: bool,
    pickup_logged: bool,
    inventory_logged: bool,
    state_logged: bool,
}

impl SurvivalMobDropFixture {
    fn new(mob: Entity, mob_id: i32) -> Self {
        Self {
            mob,
            mob_id,
            spawn_logged: false,
            attack_logged: false,
            death_logged: false,
            drop_logged: false,
            pickup_logged: false,
            inventory_logged: false,
            state_logged: false,
        }
    }
}

#[derive(Clone, Copy, Component, Debug, Eq, PartialEq)]
struct SurvivalMobDropItem {
    drop_id: i32,
    collector: Entity,
    ticks_since_drop: u8,
}

impl SurvivalMobDropItem {
    fn new(drop_id: i32, collector: Entity) -> Self {
        Self {
            drop_id,
            collector,
            ticks_since_drop: 0,
        }
    }

    fn candidate(self, entity: Entity) -> SurvivalMobDropCandidate {
        SurvivalMobDropCandidate {
            entity,
            drop_id: self.drop_id,
            collector: self.collector,
            ticks_since_drop: self.ticks_since_drop,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct SurvivalMobDropCandidate {
    entity: Entity,
    drop_id: i32,
    collector: Entity,
    ticks_since_drop: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SurvivalMobDropCandidateSelection {
    None,
    Selected(SurvivalMobDropCandidate),
    Duplicate,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct SurvivalMobDropPickupInput {
    pickup_logged: bool,
    ticks_since_drop: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SurvivalMobDropPickupDecision {
    AlreadyComplete,
    Pending { ticks_since_drop: u8 },
    Ready { ticks_since_drop: u8 },
}

#[derive(Resource, Default)]
struct SurvivalRedstoneToggleFixture {
    input_logged: bool,
    powered_on_logged: bool,
    powered_off_logged: bool,
    state_logged: bool,
}

#[derive(Resource)]
struct SurvivalWorldPersistenceFixture {
    marker_path: PathBuf,
    persisted_loaded: bool,
    mutation_logged: bool,
    state_logged: bool,
}

#[derive(Resource)]
struct SurvivalBlockEntityFixture {
    marker_path: PathBuf,
    persisted_loaded: bool,
    mutation_logged: bool,
    state_logged: bool,
}

impl SurvivalWorldPersistenceFixture {
    fn new(marker_path: PathBuf, persisted_loaded: bool) -> Self {
        Self {
            marker_path,
            persisted_loaded,
            mutation_logged: false,
            state_logged: false,
        }
    }
}

impl SurvivalBlockEntityFixture {
    fn new(marker_path: PathBuf, persisted_loaded: bool) -> Self {
        Self {
            marker_path,
            persisted_loaded,
            mutation_logged: false,
            state_logged: false,
        }
    }
}

impl SurvivalFurnaceFixture {
    fn new(inventory: Entity, smelting_breadth_enabled: bool) -> Self {
        Self {
            inventory,
            open_logged: false,
            input_logged: false,
            fuel_logged: false,
            burn_logged: false,
            output_logged: false,
            collect_logged: false,
            smelting_breadth_enabled,
            invalid_fuel_logged: false,
            breadth_state_logged: false,
            reopen_logged: false,
            state_logged: false,
            cook_progress_ticks: survival_core::INITIAL_COOK_PROGRESS_TICKS,
            remaining_burn_ticks: survival_core::NO_BURN_TICKS,
            recipes_completed: survival_core::INITIAL_RECIPE_COUNT,
            last_shell_transition: None,
            last_shell_diagnostic: None,
        }
    }
}

#[derive(Resource, Clone, Copy, Debug, PartialEq)]
struct SurvivalCompatibilityPluginContract {
    update_phase_order: &'static [SurvivalGameplayPhase],
    event_loop_phase_order: &'static [SurvivalGameplayPhase],
    scope: GameplayScope,
}

struct SurvivalRuntimeConfigSourcePlugin;

impl Plugin for SurvivalRuntimeConfigSourcePlugin {
    fn build(&self, app: &mut App) {
        register_gameplay_plugin_contract(app, SURVIVAL_RUNTIME_CONFIG_SOURCE_CONTRACT);
        app.insert_resource(SurvivalRuntimeConfig::from_env())
            .add_event::<SurvivalRuntimeConfigReloadEvent>()
            .add_systems(
                EventLoopPreUpdate,
                reload_survival_runtime_config_from_source.in_set(SurvivalGameplayPhase::Input),
            )
            .add_systems(
                Update,
                reload_survival_runtime_config_from_source.in_set(SurvivalGameplayPhase::Input),
            );
    }
}

struct SurvivalCompatibilityPlugin;

impl Plugin for SurvivalCompatibilityPlugin {
    fn build(&self, app: &mut App) {
        let contract = SurvivalCompatibilityPluginContract {
            update_phase_order: SURVIVAL_GAMEPLAY_PHASE_ORDER,
            event_loop_phase_order: SURVIVAL_GAMEPLAY_PHASE_ORDER,
            scope: SURVIVAL_PRIMARY_SCOPE,
        };
        assert_eq!(contract.update_phase_order, SURVIVAL_GAMEPLAY_PHASE_ORDER);
        assert_eq!(
            contract.event_loop_phase_order,
            SURVIVAL_GAMEPLAY_PHASE_ORDER
        );
        assert_eq!(contract.scope, SURVIVAL_PRIMARY_SCOPE);

        register_gameplay_plugin_template(
            app,
            GameplayPluginTemplate::new(SURVIVAL_GAMEPLAY_CONTRACT),
        );
        app.init_resource::<SurvivalRuntimeConfig>()
            .insert_resource(contract)
            .configure_sets(
                EventLoopPreUpdate,
                (
                    SurvivalGameplayPhase::Input,
                    SurvivalGameplayPhase::RuleEvaluation.after(SurvivalGameplayPhase::Input),
                    SurvivalGameplayPhase::WorldMutation
                        .after(SurvivalGameplayPhase::RuleEvaluation),
                    SurvivalGameplayPhase::Presentation.after(SurvivalGameplayPhase::WorldMutation),
                    SurvivalGameplayPhase::Cleanup.after(SurvivalGameplayPhase::Presentation),
                ),
            )
            .configure_sets(
                Update,
                (
                    SurvivalGameplayPhase::Input,
                    SurvivalGameplayPhase::RuleEvaluation.after(SurvivalGameplayPhase::Input),
                    SurvivalGameplayPhase::WorldMutation
                        .after(SurvivalGameplayPhase::RuleEvaluation),
                    SurvivalGameplayPhase::Presentation.after(SurvivalGameplayPhase::WorldMutation),
                    SurvivalGameplayPhase::Cleanup.after(SurvivalGameplayPhase::Presentation),
                ),
            )
            .add_systems(Startup, setup)
            .add_systems(
                EventLoopPreUpdate,
                handle_survival_chest_close.in_set(SurvivalGameplayPhase::Input),
            )
            .add_systems(Update, init_clients.in_set(SurvivalGameplayPhase::Input))
            .add_systems(
                Update,
                (
                    handle_survival_digging,
                    handle_survival_block_place,
                    handle_survival_redstone_toggle,
                    handle_survival_world_persistence_place,
                    handle_survival_chest_open,
                    handle_survival_chest_store,
                    handle_survival_crafting_open,
                    handle_survival_crafting_click,
                    handle_survival_furnace_open,
                    handle_survival_furnace_click,
                    handle_survival_hunger_food_use,
                    handle_survival_mob_drop_attack,
                )
                    .in_set(SurvivalGameplayPhase::RuleEvaluation),
            )
            .add_systems(
                Update,
                advance_survival_mob_drop_pickup
                    .in_set(SurvivalGameplayPhase::WorldMutation)
                    .run_if(survival_mob_drop_pickup_fixture_present),
            )
            .add_systems(
                Update,
                (
                    despawn_disconnected_clients,
                    remove_survival_open_containers_from_despawned,
                )
                    .chain()
                    .in_set(SurvivalGameplayPhase::Cleanup),
            );
    }
}

struct SurvivalFurnaceSmeltingPlugin;

impl Plugin for SurvivalFurnaceSmeltingPlugin {
    fn build(&self, app: &mut App) {
        register_gameplay_plugin_template(
            app,
            GameplayPluginTemplate::new(SURVIVAL_FURNACE_SMELTING_CONTRACT),
        );
        app.init_resource::<SurvivalFurnaceRecipeTableResource>()
            .init_resource::<SurvivalFurnaceFuelTableResource>()
            .init_resource::<SurvivalFurnaceSmeltingConfigResource>()
            .init_resource::<SurvivalFurnaceBlockEntity>()
            .add_event::<SurvivalFurnaceStateChangedEvent>()
            .add_event::<SurvivalFurnaceDiagnosticEvent>()
            .configure_sets(
                Update,
                (
                    SurvivalGameplayPhase::Input,
                    SurvivalGameplayPhase::RuleEvaluation.after(SurvivalGameplayPhase::Input),
                    SurvivalGameplayPhase::WorldMutation
                        .after(SurvivalGameplayPhase::RuleEvaluation),
                    SurvivalGameplayPhase::Presentation.after(SurvivalGameplayPhase::WorldMutation),
                    SurvivalGameplayPhase::Cleanup.after(SurvivalGameplayPhase::Presentation),
                ),
            )
            .add_systems(
                Update,
                tick_survival_furnace_smelting_shell.in_set(SurvivalGameplayPhase::WorldMutation),
            );
    }
}

pub fn main() {
    App::new()
        .insert_resource(NetworkSettings {
            connection_mode: ConnectionMode::Offline,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(SurvivalRuntimeConfigSourcePlugin)
        .add_plugins(SurvivalCompatibilityPlugin)
        .add_plugins(SurvivalFurnaceSmeltingPlugin)
        .run();
}

fn reload_survival_runtime_config_from_source(
    mut events: EventReader<SurvivalRuntimeConfigReloadEvent>,
    mut runtime_config: ResMut<SurvivalRuntimeConfig>,
) {
    let mut next_config = None;
    for event in events.read() {
        next_config = Some(event.config());
    }
    if let Some(next_config) = next_config {
        *runtime_config = next_config;
    }
}

fn setup(
    mut commands: Commands,
    server: Res<Server>,
    dimensions: Res<DimensionTypeRegistry>,
    biomes: Res<BiomeRegistry>,
    mut entity_manager: ResMut<EntityManager>,
    runtime_config: Res<SurvivalRuntimeConfig>,
) {
    let mut layer = LayerBundle::new(ident!("overworld"), &dimensions, &biomes, &server);

    for z in -CHUNK_RADIUS..CHUNK_RADIUS {
        for x in -CHUNK_RADIUS..CHUNK_RADIUS {
            layer.chunk.insert_chunk([x, z], UnloadedChunk::new());
        }
    }

    for z in -FLOOR_RADIUS..FLOOR_RADIUS {
        for x in -FLOOR_RADIUS..FLOOR_RADIUS {
            layer
                .chunk
                .set_block([x, FLOOR_Y, z], BlockState::GRASS_BLOCK);
        }
    }
    layer
        .chunk
        .set_block(survival_break_pos(), survival_block_state());
    if survival_chest_fixture_enabled(&runtime_config) {
        layer
            .chunk
            .set_block(survival_chest_pos(), BlockState::CHEST);
    }
    if survival_crafting_fixture_enabled(&runtime_config) {
        layer
            .chunk
            .set_block(survival_crafting_pos(), survival_crafting_table_state());
    }
    if survival_furnace_fixture_enabled(&runtime_config) {
        layer
            .chunk
            .set_block(survival_furnace_pos(), survival_furnace_state());
    }
    if survival_redstone_toggle_fixture_enabled(&runtime_config) {
        setup_survival_redstone_toggle_arena(&mut layer);
        layer.chunk.set_block(
            survival_redstone_toggle_control_pos(),
            survival_redstone_toggle_control_state(false),
        );
        layer.chunk.set_block(
            survival_redstone_toggle_output_pos(),
            survival_redstone_toggle_output_state(false),
        );
    }
    let world_persistence_marker = survival_world_persistence_marker_path(&runtime_config);
    let world_persistence_loaded = world_persistence_marker.exists();
    if survival_world_persistence_fixture_enabled(&runtime_config) {
        setup_survival_world_persistence_arena(&mut layer);
        let state = if world_persistence_loaded {
            survival_world_persistence_state()
        } else {
            BlockState::AIR
        };
        layer
            .chunk
            .set_block(survival_world_persistence_pos(), state);
    }
    let block_entity_marker = survival_block_entity_marker_path(&runtime_config);
    let block_entity_loaded = block_entity_marker.exists();
    if survival_block_entity_fixture_enabled(&runtime_config) {
        setup_survival_block_entity_arena(&mut layer);
        if survival_block_entity_should_place_sign(&runtime_config, block_entity_loaded) {
            layer
                .chunk
                .set_block(survival_block_entity_pos(), survival_block_entity_block());
        }
    }

    let layer = commands.spawn((layer, SURVIVAL_PRIMARY_SCOPE)).id();

    if survival_chest_fixture_enabled(&runtime_config) {
        let inventory = commands
            .spawn(Inventory::with_title(
                InventoryKind::Generic9x3,
                SURVIVAL_CHEST_TITLE,
            ))
            .id();
        commands.insert_resource(SurvivalChestFixture::new(inventory));
    }
    if survival_crafting_fixture_enabled(&runtime_config) {
        let inventory = commands
            .spawn(Inventory::with_title(
                InventoryKind::Crafting,
                SURVIVAL_CRAFTING_TITLE,
            ))
            .id();
        commands.insert_resource(SurvivalCraftingFixture::new(inventory));
    }
    if survival_crafting_breadth_fixture_enabled(&runtime_config) {
        commands.insert_resource(SurvivalCraftingBreadthFixture::default());
    }
    if survival_furnace_fixture_enabled(&runtime_config) {
        let inventory = commands
            .spawn(Inventory::with_title(
                InventoryKind::Furnace,
                SURVIVAL_FURNACE_TITLE,
            ))
            .id();
        commands.insert_resource(SurvivalFurnaceFixture::new(
            inventory,
            survival_furnace_smelting_breadth_fixture_enabled(&runtime_config),
        ));
    }
    if let Some(profile) = survival_hunger_profile(&runtime_config) {
        commands.insert_resource(SurvivalHungerFoodFixture::new(profile));
    }
    if survival_mob_drop_fixture_enabled(&runtime_config) {
        let mob_id = entity_manager.next_id();
        let mob = commands
            .spawn(IronGolemEntityBundle {
                id: mob_id,
                layer: EntityLayerId(layer),
                position: survival_mob_drop_position(),
                ..Default::default()
            })
            .id();
        commands.insert_resource(SurvivalMobDropFixture::new(mob, mob_id.get()));
    }
    if survival_redstone_toggle_fixture_enabled(&runtime_config) {
        commands.insert_resource(SurvivalRedstoneToggleFixture::default());
    }
    if survival_world_persistence_fixture_enabled(&runtime_config) {
        commands.insert_resource(SurvivalWorldPersistenceFixture::new(
            world_persistence_marker,
            world_persistence_loaded,
        ));
    }
    if survival_block_entity_fixture_enabled(&runtime_config) {
        commands.insert_resource(SurvivalBlockEntityFixture::new(
            block_entity_marker,
            block_entity_loaded,
        ));
    }
}

fn init_clients(
    mut commands: Commands,
    mut clients: Query<
        (
            Entity,
            &mut Client,
            &Username,
            &mut EntityLayerId,
            &mut VisibleChunkLayer,
            &mut VisibleEntityLayers,
            &mut Position,
            &mut GameMode,
            &mut Inventory,
            &mut CursorItem,
            &mut Health,
            &mut Food,
            &mut Saturation,
        ),
        Added<Client>,
    >,
    layers: Query<Entity, (With<ChunkLayer>, With<EntityLayer>, With<GameplayScope>)>,
    mut hunger_food_fixture: Option<ResMut<SurvivalHungerFoodFixture>>,
    mut crafting_breadth_fixture: Option<ResMut<SurvivalCraftingBreadthFixture>>,
    mut mob_drop_fixture: Option<ResMut<SurvivalMobDropFixture>>,
    mut world_persistence_fixture: Option<ResMut<SurvivalWorldPersistenceFixture>>,
    mut block_entity_fixture: Option<ResMut<SurvivalBlockEntityFixture>>,
    runtime_config: Res<SurvivalRuntimeConfig>,
) {
    for (
        client_entity,
        mut client,
        username,
        mut layer_id,
        mut visible_chunk_layer,
        mut visible_entity_layers,
        mut pos,
        mut game_mode,
        mut inventory,
        mut cursor_item,
        mut health,
        mut food,
        mut saturation,
    ) in &mut clients
    {
        let layer = layers.single();

        layer_id.0 = layer;
        visible_chunk_layer.0 = layer;
        visible_entity_layers.0.insert(layer);
        if survival_redstone_toggle_fixture_enabled(&runtime_config) {
            pos.set([
                SURVIVAL_REDSTONE_TOGGLE_PLAYER_X,
                SURVIVAL_REDSTONE_TOGGLE_PLAYER_Y,
                SURVIVAL_REDSTONE_TOGGLE_PLAYER_Z,
            ]);
        } else if survival_world_persistence_fixture_enabled(&runtime_config) {
            pos.set([
                SURVIVAL_WORLD_PERSISTENCE_PLAYER_X,
                SURVIVAL_WORLD_PERSISTENCE_PLAYER_Y,
                SURVIVAL_WORLD_PERSISTENCE_PLAYER_Z,
            ]);
        } else if survival_block_entity_fixture_enabled(&runtime_config) {
            pos.set([
                SURVIVAL_BLOCK_ENTITY_PLAYER_X,
                SURVIVAL_BLOCK_ENTITY_PLAYER_Y,
                SURVIVAL_BLOCK_ENTITY_PLAYER_Z,
            ]);
        } else {
            pos.set([SURVIVAL_SPAWN_X, f64::from(SPAWN_Y), SURVIVAL_SPAWN_Z]);
        }
        *game_mode = GameMode::Survival;
        commands
            .entity(client_entity)
            .insert(SURVIVAL_PRIMARY_SCOPE);
        inventory.set_slot(SURVIVAL_ITEM_SLOT, ItemStack::EMPTY);
        if survival_chest_fixture_enabled(&runtime_config) {
            cursor_item.0 = survival_chest_item_stack();
        }
        if survival_crafting_fixture_enabled(&runtime_config) {
            cursor_item.0 = survival_crafting_input_stack(SURVIVAL_CRAFTING_TOTAL_INPUT_COUNT);
        }
        if survival_furnace_fixture_enabled(&runtime_config) {
            cursor_item.0 = survival_furnace_input_stack();
        }
        if survival_world_persistence_fixture_enabled(&runtime_config) {
            inventory.set_slot(
                SURVIVAL_WORLD_PERSISTENCE_INVENTORY_SLOT,
                survival_world_persistence_stack(),
            );
        }
        if let Some(fixture) = hunger_food_fixture.as_mut() {
            let profile = fixture.profile;
            health.0 = profile.pre_health;
            food.0 = SURVIVAL_HUNGER_FOOD_PRE_FOOD;
            saturation.0 = SURVIVAL_HUNGER_FOOD_PRE_SATURATION;
            inventory.set_slot(
                SURVIVAL_HUNGER_FOOD_INVENTORY_SLOT,
                survival_hunger_food_stack(),
            );
            log_survival_hunger_food_pre(username.as_str(), fixture);
        }

        client.send_chat_message(SURVIVAL_WELCOME.italic());
        log_milestone(format!(
            "MC-COMPAT-MILESTONE survival_join username={} gamemode=Survival target={},{},{}",
            username.as_str(),
            SURVIVAL_TARGET_X,
            FLOOR_Y,
            SURVIVAL_TARGET_Z
        ));
        if survival_biome_dimension_fixture_enabled(&runtime_config) {
            log_survival_biome_dimension_state(
                username.as_str(),
                SURVIVAL_OVERWORLD_ID,
                SURVIVAL_OVERWORLD_ID,
            );
        }
        log_survival_breadth_synthetic_fixtures(&runtime_config, username.as_str());
        if let Some(fixture) = crafting_breadth_fixture.as_mut() {
            log_survival_crafting_breadth(username.as_str(), fixture);
        }
        if let Some(fixture) = mob_drop_fixture.as_mut() {
            log_survival_mob_drop_spawn(username.as_str(), fixture);
        }
        if let Some(fixture) = world_persistence_fixture.as_mut() {
            log_survival_world_persistence_post_restart(username.as_str(), &mut client, fixture);
        }
        if let Some(fixture) = block_entity_fixture.as_mut() {
            log_survival_block_entity_persistence(&runtime_config, username.as_str(), fixture);
        }
    }
}

fn handle_survival_digging(
    mut clients: Query<(&GameMode, &Username, &mut Client, &mut Inventory, &EntityId)>,
    mut layers: Query<&mut ChunkLayer>,
    mut events: EventReader<DiggingEvent>,
) {
    let mut layer = layers.single_mut();

    for event in events.read() {
        let Ok((game_mode, username, mut client, mut inventory, entity_id)) =
            clients.get_mut(event.client)
        else {
            continue;
        };
        if !should_break_survival_block(*game_mode, event.state, event.position) {
            continue;
        }
        let Some(block) = layer.block(event.position) else {
            continue;
        };
        if block.state != survival_block_state() {
            continue;
        }

        layer.set_block(event.position, BlockState::AIR);
        let stack = ItemStack::new(survival_item_kind(), SURVIVAL_BLOCK_COUNT, None);
        inventory.set_slot(SURVIVAL_ITEM_SLOT, stack.clone());
        client.write_packet(&ItemPickupAnimationS2c {
            collected_entity_id: VarInt(SURVIVAL_PICKUP_ENTITY_ID),
            collector_entity_id: VarInt(entity_id.get()),
            pickup_item_count: VarInt(SURVIVAL_PICKUP_COUNT),
        });

        log_milestone(format!(
            "MC-COMPAT-MILESTONE survival_block_break username={} item={:?} at={},{},{}",
            username.as_str(),
            stack.item,
            event.position.x,
            event.position.y,
            event.position.z
        ));
        log_milestone(format!(
            "MC-COMPAT-MILESTONE survival_pickup_item username={} slot={} item={:?} count={}",
            username.as_str(),
            SURVIVAL_ITEM_SLOT,
            stack.item,
            stack.count
        ));
    }
}

fn handle_survival_block_place(
    mut clients: Query<(&mut Inventory, &GameMode, &HeldItem, &Username)>,
    mut layers: Query<&mut ChunkLayer>,
    mut events: EventReader<InteractBlockEvent>,
) {
    let mut layer = layers.single_mut();

    for event in events.read() {
        let Ok((mut inventory, game_mode, held, username)) = clients.get_mut(event.client) else {
            continue;
        };
        if !should_place_survival_block(*game_mode, event.hand, event.position, event.face) {
            continue;
        }

        let slot_id = held.slot();
        let stack = inventory.slot(slot_id).clone();
        if stack.item != survival_item_kind() || stack.count < SURVIVAL_BLOCK_COUNT {
            continue;
        }

        inventory.set_slot(slot_id, ItemStack::EMPTY);
        let real_pos = event.position.get_in_direction(event.face);
        layer.set_block(real_pos, survival_block_state());
        log_milestone(format!(
            "MC-COMPAT-MILESTONE survival_block_place username={} item={:?} from_slot={} \
             at={},{},{}",
            username.as_str(),
            stack.item,
            slot_id,
            real_pos.x,
            real_pos.y,
            real_pos.z
        ));
    }
}

fn handle_survival_redstone_toggle(
    fixture: Option<ResMut<SurvivalRedstoneToggleFixture>>,
    mut clients: Query<(&Username, &GameMode, &mut Client)>,
    mut layers: Query<&mut ChunkLayer>,
    mut events: EventReader<InteractBlockEvent>,
) {
    let Some(mut fixture) = fixture else {
        return;
    };
    let mut layer = layers.single_mut();

    for event in events.read() {
        let Ok((username, game_mode, mut client)) = clients.get_mut(event.client) else {
            continue;
        };
        if !should_toggle_survival_redstone(*game_mode, event.hand, event.position) {
            continue;
        }
        if !fixture.input_logged {
            fixture.input_logged = true;
            layer.set_block(
                survival_redstone_toggle_control_pos(),
                survival_redstone_toggle_control_state(true),
            );
            let output_on = survival_redstone_toggle_output_state(true);
            layer.set_block(survival_redstone_toggle_output_pos(), output_on);
            client.write_packet(&BlockUpdateS2c {
                position: survival_redstone_toggle_output_pos(),
                block_id: output_on,
            });
            log_milestone(format!(
                "MC-COMPAT-MILESTONE survival_redstone_toggle_input username={} control={} \
                 position={},{},{} powered_before=false powered_after=true",
                username.as_str(),
                SURVIVAL_REDSTONE_TOGGLE_CONTROL_NAME,
                SURVIVAL_REDSTONE_TOGGLE_CONTROL_X,
                SURVIVAL_REDSTONE_TOGGLE_CONTROL_Y,
                SURVIVAL_REDSTONE_TOGGLE_CONTROL_Z
            ));
            if !fixture.powered_on_logged {
                fixture.powered_on_logged = true;
                log_milestone(format!(
                    "MC-COMPAT-MILESTONE survival_redstone_toggle_powered_on username={} \
                     output={} position={},{},{} powered=true",
                    username.as_str(),
                    SURVIVAL_REDSTONE_TOGGLE_OUTPUT_NAME,
                    SURVIVAL_REDSTONE_TOGGLE_OUTPUT_X,
                    SURVIVAL_REDSTONE_TOGGLE_OUTPUT_Y,
                    SURVIVAL_REDSTONE_TOGGLE_OUTPUT_Z
                ));
            }
            continue;
        }
        if !fixture.powered_off_logged {
            fixture.powered_off_logged = true;
            layer.set_block(
                survival_redstone_toggle_control_pos(),
                survival_redstone_toggle_control_state(false),
            );
            let output_off = survival_redstone_toggle_output_state(false);
            layer.set_block(survival_redstone_toggle_output_pos(), output_off);
            client.write_packet(&BlockUpdateS2c {
                position: survival_redstone_toggle_output_pos(),
                block_id: output_off,
            });
            log_milestone(format!(
                "MC-COMPAT-MILESTONE survival_redstone_toggle_powered_off username={} output={} \
                 position={},{},{} powered=false",
                username.as_str(),
                SURVIVAL_REDSTONE_TOGGLE_OUTPUT_NAME,
                SURVIVAL_REDSTONE_TOGGLE_OUTPUT_X,
                SURVIVAL_REDSTONE_TOGGLE_OUTPUT_Y,
                SURVIVAL_REDSTONE_TOGGLE_OUTPUT_Z
            ));
        }
        if fixture.powered_on_logged && fixture.powered_off_logged && !fixture.state_logged {
            fixture.state_logged = true;
            log_milestone(format!(
                "MC-COMPAT-MILESTONE survival_redstone_toggle_state username={} control={} \
                 output={} on_seen=true off_seen=true unintended_outputs=false",
                username.as_str(),
                SURVIVAL_REDSTONE_TOGGLE_CONTROL_NAME,
                SURVIVAL_REDSTONE_TOGGLE_OUTPUT_NAME
            ));
        }
    }
}

fn handle_survival_world_persistence_place(
    fixture: Option<ResMut<SurvivalWorldPersistenceFixture>>,
    mut clients: Query<(&Username, &GameMode, &HeldItem, &mut Inventory, &mut Client)>,
    mut layers: Query<&mut ChunkLayer>,
    mut events: EventReader<InteractBlockEvent>,
) {
    let Some(mut fixture) = fixture else {
        return;
    };
    if fixture.mutation_logged {
        return;
    }
    let mut layer = layers.single_mut();

    for event in events.read() {
        let Ok((username, game_mode, held, mut inventory, mut client)) =
            clients.get_mut(event.client)
        else {
            continue;
        };
        if !should_place_survival_world_persistence(
            *game_mode,
            event.hand,
            event.position,
            event.face,
        ) {
            continue;
        }
        let slot_id = held.slot();
        let stack = inventory.slot(slot_id).clone();
        if !is_survival_world_persistence_stack(&stack) {
            continue;
        }
        inventory.set_slot(slot_id, ItemStack::EMPTY);
        let state = survival_world_persistence_state();
        layer.set_block(survival_world_persistence_pos(), state);
        client.write_packet(&BlockUpdateS2c {
            position: survival_world_persistence_pos(),
            block_id: state,
        });
        write_survival_world_persistence_marker(&fixture.marker_path);
        fixture.persisted_loaded = true;
        fixture.mutation_logged = true;
        log_milestone(format!(
            "MC-COMPAT-MILESTONE survival_world_persistence_mutation username={} block={} \
             position={},{},{} persisted_before=false persisted_after=true",
            username.as_str(),
            SURVIVAL_WORLD_PERSISTENCE_BLOCK_NAME,
            SURVIVAL_WORLD_PERSISTENCE_X,
            SURVIVAL_WORLD_PERSISTENCE_Y,
            SURVIVAL_WORLD_PERSISTENCE_Z
        ));
    }
}

fn handle_survival_mob_drop_attack(
    mut commands: Commands,
    fixture: Option<ResMut<SurvivalMobDropFixture>>,
    mut clients: Query<(&Username, &GameMode, &mut Inventory)>,
    layers: Query<Entity, (With<ChunkLayer>, With<EntityLayer>)>,
    mut events: EventReader<InteractEntityEvent>,
    mut entity_manager: ResMut<EntityManager>,
) {
    let Some(mut fixture) = fixture else {
        return;
    };
    if fixture.attack_logged {
        return;
    }

    let layer = layers.single();
    for event in events.read() {
        let Ok((username, game_mode, _inventory)) = clients.get_mut(event.client) else {
            continue;
        };
        if !should_handle_survival_mob_drop_attack(
            *game_mode,
            event.interact,
            event.entity,
            fixture.mob,
        ) {
            continue;
        }

        fixture.attack_logged = true;
        fixture.death_logged = true;
        log_milestone(format!(
            "MC-COMPAT-MILESTONE survival_mob_drop_attack username={} mob={} damage={:.1} \
             target_id={}",
            username.as_str(),
            SURVIVAL_MOB_DROP_MOB_NAME,
            SURVIVAL_MOB_DROP_DAMAGE,
            fixture.mob_id
        ));
        log_milestone(format!(
            "MC-COMPAT-MILESTONE survival_mob_drop_death username={} mob={} target_id={}",
            username.as_str(),
            SURVIVAL_MOB_DROP_MOB_NAME,
            fixture.mob_id
        ));
        commands.entity(fixture.mob).insert(Despawned);
        spawn_survival_mob_drop_item(
            &mut commands,
            &mut entity_manager,
            &mut fixture,
            layer,
            event.client,
            username.as_str(),
        );
        break;
    }
}

fn advance_survival_mob_drop_pickup(
    mut commands: Commands,
    mut fixture: ResMut<SurvivalMobDropFixture>,
    mut drops: ParamSet<(
        Query<(Entity, &SurvivalMobDropItem)>,
        Query<&mut SurvivalMobDropItem>,
    )>,
    mut clients: Query<(&Username, &mut Client, &mut Inventory, &EntityId)>,
) {
    let selection = {
        let pending_drops = drops.p0();
        let candidates = pending_drops
            .iter()
            .map(|(entity, drop)| drop.candidate(entity));
        select_survival_mob_drop_candidate(candidates)
    };
    let SurvivalMobDropCandidateSelection::Selected(drop) = selection else {
        return;
    };

    let decision = plan_survival_mob_drop_pickup(SurvivalMobDropPickupInput {
        pickup_logged: fixture.pickup_logged,
        ticks_since_drop: drop.ticks_since_drop,
    });
    let ticks_since_drop = match decision {
        SurvivalMobDropPickupDecision::AlreadyComplete => return,
        SurvivalMobDropPickupDecision::Pending { ticks_since_drop } => {
            if let Ok(mut state) = drops.p1().get_mut(drop.entity) {
                state.ticks_since_drop = ticks_since_drop;
            }
            return;
        }
        SurvivalMobDropPickupDecision::Ready { ticks_since_drop } => ticks_since_drop,
    };

    if let Ok(mut state) = drops.p1().get_mut(drop.entity) {
        state.ticks_since_drop = ticks_since_drop;
    }
    let Ok((username, mut client, mut inventory, collector_id)) = clients.get_mut(drop.collector)
    else {
        return;
    };

    inventory.set_slot(SURVIVAL_MOB_DROP_INVENTORY_SLOT, survival_mob_drop_stack());
    client.write_packet(&ItemPickupAnimationS2c {
        collected_entity_id: VarInt(drop.drop_id),
        collector_entity_id: VarInt(collector_id.get()),
        pickup_item_count: VarInt(i32::from(SURVIVAL_MOB_DROP_ITEM_COUNT)),
    });
    commands.entity(drop.entity).insert(Despawned);
    log_survival_mob_drop_pickup_and_state(
        username.as_str(),
        &mut fixture,
        drop.drop_id,
        collector_id.get(),
    );
}

fn handle_survival_chest_open(
    mut commands: Commands,
    fixture: Option<ResMut<SurvivalChestFixture>>,
    clients: Query<(&Username, &GameMode)>,
    inventories: Query<&Inventory>,
    mut events: EventReader<InteractBlockEvent>,
) {
    let Some(mut fixture) = fixture else {
        return;
    };

    for event in events.read() {
        let Ok((username, game_mode)) = clients.get(event.client) else {
            continue;
        };
        if !should_open_survival_chest(*game_mode, event.hand, event.position) {
            continue;
        }

        commands.entity(event.client).insert((
            OpenInventory::new(fixture.inventory),
            SurvivalOpenContainer::new(SurvivalContainerKind::Chest),
        ));

        if fixture.store_logged {
            log_survival_chest_reopen(username.as_str(), &mut fixture, &inventories);
        } else if !fixture.open_logged {
            fixture.open_logged = true;
            log_milestone(format!(
                "MC-COMPAT-MILESTONE survival_chest_open username={} position={},{},{} window={}",
                username.as_str(),
                SURVIVAL_CHEST_X,
                SURVIVAL_CHEST_Y,
                SURVIVAL_CHEST_Z,
                SURVIVAL_CHEST_WINDOW
            ));
        }
    }
}

fn handle_survival_chest_store(
    fixture: Option<ResMut<SurvivalChestFixture>>,
    clients: Query<(&Username, Option<&SurvivalOpenContainer>)>,
    mut events: EventReader<ClickSlotEvent>,
) {
    let Some(mut fixture) = fixture else {
        drain_survival_chest_store_events(&mut events);
        return;
    };
    if fixture.store_logged {
        drain_survival_chest_store_events(&mut events);
        return;
    }

    for event in events.read() {
        let Ok((username, open_container)) = clients.get(event.client) else {
            continue;
        };
        if !survival_container_is_open(open_container, SurvivalContainerKind::Chest)
            || !is_survival_chest_store_event(event.window_id, event.slot_id, &event.slot_changes)
        {
            continue;
        }

        fixture.store_logged = true;
        log_milestone(format!(
            "MC-COMPAT-MILESTONE survival_chest_store username={} window={} slot={} item={:?} \
             count={}",
            username.as_str(),
            SURVIVAL_CHEST_WINDOW,
            SURVIVAL_CHEST_SLOT,
            survival_chest_item_kind(),
            SURVIVAL_CHEST_ITEM_COUNT
        ));
        break;
    }
}

fn handle_survival_chest_close(
    mut commands: Commands,
    fixture: Option<ResMut<SurvivalChestFixture>>,
    clients: Query<(&Username, Option<&SurvivalOpenContainer>)>,
    mut close_events: EventReader<CloseHandledScreenEvent>,
) {
    let Some(mut fixture) = fixture else {
        return;
    };
    if !fixture.store_logged || fixture.close_logged {
        return;
    }

    for event in close_events.read() {
        let Ok((username, open_container)) = clients.get(event.client) else {
            continue;
        };
        if !survival_container_is_open(open_container, SurvivalContainerKind::Chest) {
            continue;
        }
        commands
            .entity(event.client)
            .remove::<SurvivalOpenContainer>();
        fixture.close_logged = true;
        log_milestone(format!(
            "MC-COMPAT-MILESTONE survival_chest_close username={} window={}",
            username.as_str(),
            SURVIVAL_CHEST_WINDOW
        ));
        break;
    }
}

fn remove_survival_open_containers_from_despawned(
    mut commands: Commands,
    clients: Query<
        (Entity, Option<&GameplayScope>),
        (With<SurvivalOpenContainer>, With<Despawned>),
    >,
) {
    for (client, scope) in &clients {
        if !gameplay_scope_matches(scope, SURVIVAL_PRIMARY_SCOPE) {
            continue;
        }
        commands.entity(client).remove::<SurvivalOpenContainer>();
    }
}

fn drain_survival_chest_store_events(events: &mut EventReader<ClickSlotEvent>) {
    for _ in events.read() {}
}

fn survival_mob_drop_pickup_fixture_present(fixture: Option<Res<SurvivalMobDropFixture>>) -> bool {
    survival_mob_drop_pickup_resource_present(fixture.is_some())
}

fn survival_mob_drop_pickup_resource_present(resource_present: bool) -> bool {
    resource_present
}

fn select_survival_mob_drop_candidate<I>(candidates: I) -> SurvivalMobDropCandidateSelection
where
    I: IntoIterator<Item = SurvivalMobDropCandidate>,
{
    let mut selected = None;
    for candidate in candidates {
        if selected.is_some() {
            return SurvivalMobDropCandidateSelection::Duplicate;
        }
        selected = Some(candidate);
    }
    selected.map_or(
        SurvivalMobDropCandidateSelection::None,
        SurvivalMobDropCandidateSelection::Selected,
    )
}

fn plan_survival_mob_drop_pickup(
    input: SurvivalMobDropPickupInput,
) -> SurvivalMobDropPickupDecision {
    match survival_core::plan_mob_drop_pickup(
        survival_core::MobDropPickupInput {
            pickup_logged: input.pickup_logged,
            ticks_since_drop: input.ticks_since_drop,
        },
        SURVIVAL_MOB_DROP_PICKUP_DELAY_TICKS,
    ) {
        survival_core::MobDropPickupDecision::AlreadyComplete => {
            SurvivalMobDropPickupDecision::AlreadyComplete
        }
        survival_core::MobDropPickupDecision::Pending { ticks_since_drop } => {
            SurvivalMobDropPickupDecision::Pending { ticks_since_drop }
        }
        survival_core::MobDropPickupDecision::Ready { ticks_since_drop } => {
            SurvivalMobDropPickupDecision::Ready { ticks_since_drop }
        }
    }
}

fn handle_survival_crafting_open(
    mut commands: Commands,
    fixture: Option<ResMut<SurvivalCraftingFixture>>,
    clients: Query<(&Username, &GameMode)>,
    mut inventories: Query<&mut Inventory>,
    mut events: EventReader<InteractBlockEvent>,
) {
    let Some(mut fixture) = fixture else {
        return;
    };

    for event in events.read() {
        let Ok((username, game_mode)) = clients.get(event.client) else {
            continue;
        };
        if !should_open_survival_crafting(*game_mode, event.hand, event.position) {
            continue;
        }

        commands.entity(event.client).insert((
            OpenInventory::new(fixture.inventory),
            SurvivalOpenContainer::new(SurvivalContainerKind::Crafting),
        ));

        if !fixture.open_logged {
            fixture.open_logged = true;
            log_milestone(format!(
                "MC-COMPAT-MILESTONE survival_crafting_table_open username={} position={},{},{} \
                 window={}",
                username.as_str(),
                SURVIVAL_CRAFTING_X,
                SURVIVAL_CRAFTING_Y,
                SURVIVAL_CRAFTING_Z,
                SURVIVAL_CRAFTING_WINDOW
            ));
        }
        emit_survival_crafting_fixture_milestones(
            &mut fixture,
            &mut inventories,
            event.client,
            username,
        );
    }
}

fn handle_survival_crafting_click(
    fixture: Option<ResMut<SurvivalCraftingFixture>>,
    clients: Query<(&Username, Option<&SurvivalOpenContainer>)>,
    mut inventories: Query<&mut Inventory>,
    mut events: EventReader<ClickSlotEvent>,
) {
    let Some(mut fixture) = fixture else {
        return;
    };

    for event in events.read() {
        let Ok((username, open_container)) = clients.get(event.client) else {
            continue;
        };
        if !survival_container_is_open(open_container, SurvivalContainerKind::Crafting)
            || event.window_id != SURVIVAL_CRAFTING_WINDOW
        {
            continue;
        }

        if is_survival_crafting_input_event(
            event.window_id,
            event.slot_id,
            SURVIVAL_CRAFTING_INPUT_A_SLOT_ID,
        ) && !fixture.input_a_logged
        {
            fixture.input_a_logged = true;
            set_survival_crafting_slot(
                &mut inventories,
                fixture.inventory,
                SURVIVAL_CRAFTING_INPUT_A_SLOT,
                survival_crafting_input_stack(SURVIVAL_CRAFTING_INPUT_COUNT),
            );
            log_milestone(format!(
                "MC-COMPAT-MILESTONE survival_crafting_input_a username={} window={} slot={} \
                 item={:?} count={}",
                username.as_str(),
                SURVIVAL_CRAFTING_WINDOW,
                SURVIVAL_CRAFTING_INPUT_A_SLOT,
                survival_crafting_input_kind(),
                SURVIVAL_CRAFTING_INPUT_COUNT
            ));
        }

        if is_survival_crafting_input_event(
            event.window_id,
            event.slot_id,
            SURVIVAL_CRAFTING_INPUT_B_SLOT_ID,
        ) && !fixture.input_b_logged
        {
            fixture.input_b_logged = true;
            set_survival_crafting_slot(
                &mut inventories,
                fixture.inventory,
                SURVIVAL_CRAFTING_INPUT_B_SLOT,
                survival_crafting_input_stack(SURVIVAL_CRAFTING_INPUT_COUNT),
            );
            log_milestone(format!(
                "MC-COMPAT-MILESTONE survival_crafting_input_b username={} window={} slot={} \
                 item={:?} count={}",
                username.as_str(),
                SURVIVAL_CRAFTING_WINDOW,
                SURVIVAL_CRAFTING_INPUT_B_SLOT,
                survival_crafting_input_kind(),
                SURVIVAL_CRAFTING_INPUT_COUNT
            ));
        }

        if fixture.input_a_logged && fixture.input_b_logged && !fixture.result_logged {
            fixture.result_logged = true;
            set_survival_crafting_slot(
                &mut inventories,
                fixture.inventory,
                SURVIVAL_CRAFTING_RESULT_SLOT,
                survival_crafting_result_stack(),
            );
            log_milestone(format!(
                "MC-COMPAT-MILESTONE survival_crafting_result username={} window={} slot={} \
                 item={:?} count={} recipe={}",
                username.as_str(),
                SURVIVAL_CRAFTING_WINDOW,
                SURVIVAL_CRAFTING_RESULT_SLOT,
                survival_crafting_result_kind(),
                SURVIVAL_CRAFTING_RESULT_COUNT,
                SURVIVAL_CRAFTING_RECIPE
            ));
        }

        if is_survival_crafting_collect_event(event.window_id, event.slot_id, &event.carried_item)
            && fixture.result_logged
            && !fixture.collect_logged
        {
            fixture.collect_logged = true;
            set_survival_crafting_slot(
                &mut inventories,
                fixture.inventory,
                SURVIVAL_CRAFTING_RESULT_SLOT,
                ItemStack::EMPTY,
            );
            set_survival_crafting_slot(
                &mut inventories,
                event.client,
                SURVIVAL_CRAFTING_INVENTORY_SLOT,
                survival_crafting_result_stack(),
            );
            log_milestone(format!(
                "MC-COMPAT-MILESTONE survival_crafting_collect username={} window={} slot={} \
                 item={:?} count={} inventory_slot={}",
                username.as_str(),
                SURVIVAL_CRAFTING_WINDOW,
                SURVIVAL_CRAFTING_RESULT_SLOT,
                survival_crafting_result_kind(),
                SURVIVAL_CRAFTING_RESULT_COUNT,
                SURVIVAL_CRAFTING_INVENTORY_SLOT
            ));
        }
    }
}

fn handle_survival_furnace_open(
    mut commands: Commands,
    fixture: Option<ResMut<SurvivalFurnaceFixture>>,
    clients: Query<(&Username, &GameMode)>,
    inventories: Query<&Inventory>,
    mut events: EventReader<InteractBlockEvent>,
) {
    let Some(mut fixture) = fixture else {
        return;
    };

    for event in events.read() {
        let Ok((username, game_mode)) = clients.get(event.client) else {
            continue;
        };
        if !should_open_survival_furnace(*game_mode, event.hand, event.position) {
            continue;
        }

        commands.entity(event.client).insert((
            OpenInventory::new(fixture.inventory),
            SurvivalOpenContainer::new(SurvivalContainerKind::Furnace),
        ));

        if fixture.collect_logged {
            log_survival_furnace_reopen(username.as_str(), &mut fixture, &inventories);
        } else if !fixture.open_logged {
            fixture.open_logged = true;
            log_milestone(format!(
                "MC-COMPAT-MILESTONE survival_furnace_open username={} position={},{},{} window={}",
                username.as_str(),
                SURVIVAL_FURNACE_X,
                SURVIVAL_FURNACE_Y,
                SURVIVAL_FURNACE_Z,
                SURVIVAL_FURNACE_WINDOW
            ));
        }
    }
}

fn handle_survival_furnace_click(
    fixture: Option<ResMut<SurvivalFurnaceFixture>>,
    clients: Query<(&Username, Option<&SurvivalOpenContainer>)>,
    mut inventories: Query<&mut Inventory>,
    mut events: EventReader<ClickSlotEvent>,
) {
    let Some(mut fixture) = fixture else {
        return;
    };

    for event in events.read() {
        let Ok((username, open_container)) = clients.get(event.client) else {
            continue;
        };
        if !survival_container_is_open(open_container, SurvivalContainerKind::Furnace)
            || event.window_id != SURVIVAL_FURNACE_WINDOW
        {
            continue;
        }

        if is_survival_furnace_input_event(event.window_id, event.slot_id) && !fixture.input_logged
        {
            fixture.input_logged = true;
            set_survival_slot(
                &mut inventories,
                fixture.inventory,
                SURVIVAL_FURNACE_INPUT_SLOT,
                survival_furnace_input_stack(),
            );
            log_milestone(format!(
                "MC-COMPAT-MILESTONE survival_furnace_input_insert username={} window={} slot={} \
                 item={} count={}",
                username.as_str(),
                SURVIVAL_FURNACE_WINDOW,
                SURVIVAL_FURNACE_INPUT_SLOT,
                SURVIVAL_FURNACE_INPUT_NAME,
                SURVIVAL_FURNACE_ITEM_COUNT
            ));
        }

        if fixture.input_logged && !fixture.fuel_logged {
            emit_survival_furnace_fuel(username, &mut fixture, &mut inventories);
        }
        if is_survival_furnace_fuel_event(event.window_id, event.slot_id) && !fixture.fuel_logged {
            emit_survival_furnace_fuel(username, &mut fixture, &mut inventories);
        }

        emit_survival_furnace_output_if_ready(username, &mut fixture, &mut inventories);

        if is_survival_furnace_collect_event(event.window_id, event.slot_id, &event.carried_item)
            && fixture.output_logged
            && !fixture.collect_logged
        {
            fixture.collect_logged = true;
            set_survival_slot(
                &mut inventories,
                fixture.inventory,
                SURVIVAL_FURNACE_OUTPUT_SLOT,
                ItemStack::EMPTY,
            );
            set_survival_slot(
                &mut inventories,
                event.client,
                SURVIVAL_FURNACE_INVENTORY_SLOT,
                survival_furnace_output_stack(),
            );
            log_milestone(format!(
                "MC-COMPAT-MILESTONE survival_furnace_output_collect username={} window={} \
                 slot={} item={} count={} inventory_slot={}",
                username.as_str(),
                SURVIVAL_FURNACE_WINDOW,
                SURVIVAL_FURNACE_OUTPUT_SLOT,
                SURVIVAL_FURNACE_OUTPUT_NAME,
                SURVIVAL_FURNACE_ITEM_COUNT,
                SURVIVAL_FURNACE_INVENTORY_SLOT
            ));
        }

        if should_emit_survival_furnace_breadth_rejection(
            fixture.smelting_breadth_enabled,
            fixture.collect_logged,
        ) && !fixture.invalid_fuel_logged
        {
            emit_survival_furnace_invalid_fuel_rejection(username, &mut fixture, &mut inventories);
        }

        if should_reject_survival_furnace_invalid_fuel(
            fixture.smelting_breadth_enabled,
            fixture.collect_logged,
            event.window_id,
            event.slot_id,
        ) && !fixture.invalid_fuel_logged
        {
            emit_survival_furnace_invalid_fuel_rejection(username, &mut fixture, &mut inventories);
        }
    }
}

fn handle_survival_hunger_food_use(
    fixture: Option<ResMut<SurvivalHungerFoodFixture>>,
    mut clients: Query<(
        &Username,
        &HeldItem,
        &mut Inventory,
        &mut Health,
        &mut Food,
        &mut Saturation,
    )>,
    mut events: EventReader<InteractItemEvent>,
) {
    let Some(mut fixture) = fixture else {
        return;
    };

    for event in events.read() {
        let Ok((username, held, mut inventory, mut health, mut food, mut saturation)) =
            clients.get_mut(event.client)
        else {
            continue;
        };
        let held_slot = held.slot();
        let stack = inventory.slot(SURVIVAL_HUNGER_FOOD_INVENTORY_SLOT).clone();
        let profile = fixture.profile;
        if !should_consume_survival_hunger_food(
            profile,
            event.hand,
            event.sequence,
            held_slot,
            &stack,
            health.0,
            food.0,
            saturation.0,
        ) {
            continue;
        }
        emit_survival_hunger_food_consumed(
            username.as_str(),
            &mut fixture,
            &mut inventory,
            &mut health,
            &mut food,
            &mut saturation,
        );
        break;
    }
}

fn emit_survival_hunger_food_consumed(
    username: &str,
    fixture: &mut SurvivalHungerFoodFixture,
    inventory: &mut Inventory,
    health: &mut Health,
    food: &mut Food,
    saturation: &mut Saturation,
) {
    if !fixture.consume_start_logged {
        fixture.consume_start_logged = true;
        log_milestone(format!(
            "MC-COMPAT-MILESTONE {}_consume_start username={} item={} slot={} food_before={} \
             saturation_before={:.1}",
            fixture.profile.event_prefix,
            username,
            SURVIVAL_HUNGER_FOOD_ITEM_NAME,
            SURVIVAL_HUNGER_FOOD_INVENTORY_SLOT,
            SURVIVAL_HUNGER_FOOD_PRE_FOOD,
            SURVIVAL_HUNGER_FOOD_PRE_SATURATION
        ));
    }

    inventory.set_slot(SURVIVAL_HUNGER_FOOD_INVENTORY_SLOT, ItemStack::EMPTY);
    health.0 = fixture.profile.post_health;
    food.0 = SURVIVAL_HUNGER_FOOD_POST_FOOD;
    saturation.0 = SURVIVAL_HUNGER_FOOD_POST_SATURATION;

    if !fixture.consume_finish_logged {
        fixture.consume_finish_logged = true;
        log_milestone(format!(
            "MC-COMPAT-MILESTONE {}_consume_finish username={} item={} slot={} food_after={} \
             saturation_after={:.1}",
            fixture.profile.event_prefix,
            username,
            SURVIVAL_HUNGER_FOOD_ITEM_NAME,
            SURVIVAL_HUNGER_FOOD_INVENTORY_SLOT,
            SURVIVAL_HUNGER_FOOD_POST_FOOD,
            SURVIVAL_HUNGER_FOOD_POST_SATURATION
        ));
    }
    if !fixture.inventory_logged {
        fixture.inventory_logged = true;
        log_milestone(format!(
            "MC-COMPAT-MILESTONE {}_inventory username={} slot={} item={} count_before={} \
             count_after={}",
            fixture.profile.event_prefix,
            username,
            SURVIVAL_HUNGER_FOOD_INVENTORY_SLOT,
            SURVIVAL_HUNGER_FOOD_ITEM_NAME,
            SURVIVAL_HUNGER_FOOD_ITEM_COUNT_BEFORE,
            SURVIVAL_HUNGER_FOOD_ITEM_COUNT_AFTER
        ));
    }
    if !fixture.state_logged {
        fixture.state_logged = true;
        log_milestone(survival_hunger_state_milestone(username, fixture.profile));
    }
}

fn survival_hunger_state_milestone(username: &str, profile: SurvivalHungerProfile) -> String {
    if profile.event_prefix == SURVIVAL_HUNGER_HEALTH_EVENT_PREFIX {
        return format!(
            "MC-COMPAT-MILESTONE {}_state username={} pre_health={:.1} post_health={:.1} \
             food_before={} food_after={} saturation_before={:.1} saturation_after={:.1} \
             unexpected_damage=false death=false",
            profile.event_prefix,
            username,
            profile.pre_health,
            profile.post_health,
            SURVIVAL_HUNGER_FOOD_PRE_FOOD,
            SURVIVAL_HUNGER_FOOD_POST_FOOD,
            SURVIVAL_HUNGER_FOOD_PRE_SATURATION,
            SURVIVAL_HUNGER_FOOD_POST_SATURATION
        );
    }
    format!(
        "MC-COMPAT-MILESTONE {}_state username={} health={:.1} food_before={} food_after={} \
         saturation_before={:.1} saturation_after={:.1} unexpected_damage=false death=false",
        profile.event_prefix,
        username,
        profile.post_health,
        SURVIVAL_HUNGER_FOOD_PRE_FOOD,
        SURVIVAL_HUNGER_FOOD_POST_FOOD,
        SURVIVAL_HUNGER_FOOD_PRE_SATURATION,
        SURVIVAL_HUNGER_FOOD_POST_SATURATION
    )
}

fn emit_survival_furnace_fuel(
    username: &Username,
    fixture: &mut SurvivalFurnaceFixture,
    inventories: &mut Query<&mut Inventory>,
) {
    fixture.fuel_logged = true;
    set_survival_slot(
        inventories,
        fixture.inventory,
        SURVIVAL_FURNACE_FUEL_SLOT,
        survival_furnace_fuel_stack(),
    );
    log_milestone(format!(
        "MC-COMPAT-MILESTONE survival_furnace_fuel_insert username={} window={} slot={} item={} \
         count={}",
        username.as_str(),
        SURVIVAL_FURNACE_WINDOW,
        SURVIVAL_FURNACE_FUEL_SLOT,
        SURVIVAL_FURNACE_FUEL_NAME,
        SURVIVAL_FURNACE_ITEM_COUNT
    ));
}

fn emit_survival_furnace_invalid_fuel_rejection(
    username: &Username,
    fixture: &mut SurvivalFurnaceFixture,
    inventories: &mut Query<&mut Inventory>,
) {
    fixture.invalid_fuel_logged = true;
    set_survival_slot(
        inventories,
        fixture.inventory,
        SURVIVAL_FURNACE_FUEL_SLOT,
        survival_furnace_input_stack(),
    );
    set_survival_slot(
        inventories,
        fixture.inventory,
        SURVIVAL_FURNACE_OUTPUT_SLOT,
        ItemStack::EMPTY,
    );
    log_milestone(format!(
        "MC-COMPAT-MILESTONE survival_furnace_invalid_fuel_rejected username={} window={} slot={} \
         item={} outcome={}",
        username.as_str(),
        SURVIVAL_FURNACE_WINDOW,
        SURVIVAL_FURNACE_FUEL_SLOT,
        SURVIVAL_FURNACE_INPUT_NAME,
        SURVIVAL_FURNACE_INVALID_FUEL_OUTCOME
    ));
    emit_survival_furnace_breadth_state_if_ready(username, fixture);
}

fn emit_survival_furnace_breadth_state_if_ready(
    username: &Username,
    fixture: &mut SurvivalFurnaceFixture,
) {
    if !fixture.invalid_fuel_logged || fixture.breadth_state_logged {
        return;
    }
    fixture.breadth_state_logged = true;
    log_milestone(format!(
        "MC-COMPAT-MILESTONE survival_furnace_breadth_state username={} recipe={} input={} \
         fuel={} output={} count={} invalid_fuel={} invalid_fuel_outcome={} \
         broad_all_furnaces=false",
        username.as_str(),
        SURVIVAL_FURNACE_SMELTING_RECIPE,
        SURVIVAL_FURNACE_INPUT_NAME,
        SURVIVAL_FURNACE_FUEL_NAME,
        SURVIVAL_FURNACE_OUTPUT_NAME,
        SURVIVAL_FURNACE_ITEM_COUNT,
        SURVIVAL_FURNACE_INPUT_NAME,
        SURVIVAL_FURNACE_INVALID_FUEL_OUTCOME
    ));
}

fn emit_survival_furnace_output_if_ready(
    username: &Username,
    fixture: &mut SurvivalFurnaceFixture,
    inventories: &mut Query<&mut Inventory>,
) {
    if !fixture.input_logged || !fixture.fuel_logged {
        return;
    }
    if !fixture.burn_logged {
        fixture.burn_logged = true;
        log_milestone(format!(
            "MC-COMPAT-MILESTONE survival_furnace_burn_progress username={} window={} \
             progress=started",
            username.as_str(),
            SURVIVAL_FURNACE_WINDOW
        ));
    }
    if !fixture.output_logged {
        fixture.output_logged = true;
        set_survival_slot(
            inventories,
            fixture.inventory,
            SURVIVAL_FURNACE_OUTPUT_SLOT,
            survival_furnace_output_stack(),
        );
        log_milestone(format!(
            "MC-COMPAT-MILESTONE survival_furnace_output_available username={} window={} slot={} \
             item={} count={}",
            username.as_str(),
            SURVIVAL_FURNACE_WINDOW,
            SURVIVAL_FURNACE_OUTPUT_SLOT,
            SURVIVAL_FURNACE_OUTPUT_NAME,
            SURVIVAL_FURNACE_ITEM_COUNT
        ));
    }
}

fn tick_survival_furnace_smelting_shell(
    fixture: Option<ResMut<SurvivalFurnaceFixture>>,
    block_entity: Option<Res<SurvivalFurnaceBlockEntity>>,
    recipe_table: Option<Res<SurvivalFurnaceRecipeTableResource>>,
    fuel_table: Option<Res<SurvivalFurnaceFuelTableResource>>,
    config: Option<Res<SurvivalFurnaceSmeltingConfigResource>>,
    mut inventories: Query<&mut Inventory>,
    mut state_events: EventWriter<SurvivalFurnaceStateChangedEvent>,
    mut diagnostic_events: EventWriter<SurvivalFurnaceDiagnosticEvent>,
) {
    let Some(mut fixture) = fixture else {
        return;
    };
    if fixture.output_logged || fixture.collect_logged {
        return;
    }
    fixture.last_shell_transition = None;
    fixture.last_shell_diagnostic = None;

    let inventory_entity = fixture.inventory;
    let Some(config) = config else {
        record_survival_furnace_diagnostic(
            &mut fixture,
            &mut diagnostic_events,
            Some(inventory_entity),
            SurvivalFurnaceShellDiagnostic::StaleBlockEntity,
        );
        return;
    };
    if !config.enabled {
        return;
    }
    let Some(block_entity) = block_entity else {
        record_survival_furnace_diagnostic(
            &mut fixture,
            &mut diagnostic_events,
            Some(inventory_entity),
            SurvivalFurnaceShellDiagnostic::StaleBlockEntity,
        );
        return;
    };
    if !block_entity.loaded {
        record_survival_furnace_diagnostic(
            &mut fixture,
            &mut diagnostic_events,
            Some(inventory_entity),
            SurvivalFurnaceShellDiagnostic::StaleBlockEntity,
        );
        return;
    }
    let Some(recipe_table) = recipe_table else {
        record_survival_furnace_diagnostic(
            &mut fixture,
            &mut diagnostic_events,
            Some(inventory_entity),
            SurvivalFurnaceShellDiagnostic::MalformedRecipeRow,
        );
        return;
    };
    let Some(fuel_table) = fuel_table else {
        record_survival_furnace_diagnostic(
            &mut fixture,
            &mut diagnostic_events,
            Some(inventory_entity),
            SurvivalFurnaceShellDiagnostic::MalformedFuelRow,
        );
        return;
    };
    let Ok(mut inventory) = inventories.get_mut(inventory_entity) else {
        record_survival_furnace_diagnostic(
            &mut fixture,
            &mut diagnostic_events,
            Some(inventory_entity),
            SurvivalFurnaceShellDiagnostic::MissingInventory,
        );
        return;
    };
    if inventory.kind() != InventoryKind::Furnace {
        record_survival_furnace_diagnostic(
            &mut fixture,
            &mut diagnostic_events,
            Some(inventory_entity),
            SurvivalFurnaceShellDiagnostic::MissingInventory,
        );
        return;
    }

    let snapshot = snapshot_survival_furnace_state(&inventory, &fixture, &block_entity);
    match survival_core::tick_selected_standard_furnace(
        snapshot,
        &recipe_table.rows,
        &fuel_table.rows,
        config.limits,
    ) {
        Ok(tick) => commit_survival_furnace_tick(
            &mut fixture,
            &mut inventory,
            tick,
            inventory_entity,
            &mut state_events,
            &mut diagnostic_events,
        ),
        Err(error) => record_survival_furnace_diagnostic(
            &mut fixture,
            &mut diagnostic_events,
            Some(inventory_entity),
            survival_furnace_error_diagnostic(error),
        ),
    }
}

fn snapshot_survival_furnace_state<'a>(
    inventory: &Inventory,
    fixture: &SurvivalFurnaceFixture,
    block_entity: &SurvivalFurnaceBlockEntity,
) -> survival_core::FurnaceState<'a> {
    survival_core::FurnaceState {
        kind: block_entity.kind,
        input: snapshot_survival_furnace_stack(inventory.slot(SURVIVAL_FURNACE_INPUT_SLOT)),
        fuel: snapshot_survival_furnace_stack(inventory.slot(SURVIVAL_FURNACE_FUEL_SLOT)),
        output: snapshot_survival_furnace_stack(inventory.slot(SURVIVAL_FURNACE_OUTPUT_SLOT)),
        cook_progress_ticks: fixture.cook_progress_ticks,
        remaining_burn_ticks: fixture.remaining_burn_ticks,
        recipes_completed: fixture.recipes_completed,
    }
}

fn snapshot_survival_furnace_stack(
    stack: &ItemStack,
) -> Option<survival_core::FurnaceStack<'static>> {
    if is_empty_item(stack) {
        return None;
    }
    Some(survival_core::furnace_stack(
        survival_furnace_item_id(stack.item),
        stack.count,
    ))
}

fn survival_furnace_item_id(item: ItemKind) -> &'static str {
    match item {
        ItemKind::RawIron => survival_core::SELECTED_RAW_IRON_ITEM,
        ItemKind::Coal => survival_core::SELECTED_COAL_ITEM,
        ItemKind::IronIngot => survival_core::SELECTED_IRON_INGOT_ITEM,
        _ => survival_core::UNSUPPORTED_FURNACE_ITEM,
    }
}

fn commit_survival_furnace_tick(
    fixture: &mut SurvivalFurnaceFixture,
    inventory: &mut Inventory,
    tick: survival_core::FurnaceTick<'_>,
    inventory_entity: Entity,
    state_events: &mut EventWriter<SurvivalFurnaceStateChangedEvent>,
    diagnostic_events: &mut EventWriter<SurvivalFurnaceDiagnosticEvent>,
) {
    match tick.transition {
        survival_core::FurnaceTransition::StartedFuel
        | survival_core::FurnaceTransition::AdvancedCooking
        | survival_core::FurnaceTransition::ProducedOutput => {
            commit_survival_furnace_state(fixture, inventory, tick.state);
            fixture.last_shell_transition = Some(tick.transition);
            state_events.send(SurvivalFurnaceStateChangedEvent {
                inventory: inventory_entity,
                transition: tick.transition,
            });
        }
        survival_core::FurnaceTransition::PausedNoFuel
        | survival_core::FurnaceTransition::PausedNoRecipe
        | survival_core::FurnaceTransition::PausedOutputBlocked => {
            record_survival_furnace_diagnostic(
                fixture,
                diagnostic_events,
                Some(inventory_entity),
                survival_furnace_pause_diagnostic(tick.transition),
            );
        }
    }
}

fn commit_survival_furnace_state(
    fixture: &mut SurvivalFurnaceFixture,
    inventory: &mut Inventory,
    state: survival_core::FurnaceState<'_>,
) {
    fixture.cook_progress_ticks = state.cook_progress_ticks;
    fixture.remaining_burn_ticks = state.remaining_burn_ticks;
    fixture.recipes_completed = state.recipes_completed;
    inventory.set_slot(
        SURVIVAL_FURNACE_INPUT_SLOT,
        survival_furnace_stack_from_core(state.input),
    );
    inventory.set_slot(
        SURVIVAL_FURNACE_FUEL_SLOT,
        survival_furnace_stack_from_core(state.fuel),
    );
    inventory.set_slot(
        SURVIVAL_FURNACE_OUTPUT_SLOT,
        survival_furnace_stack_from_core(state.output),
    );
}

fn survival_furnace_stack_from_core(stack: Option<survival_core::FurnaceStack<'_>>) -> ItemStack {
    let Some(stack) = stack else {
        return ItemStack::EMPTY;
    };
    match stack.item {
        survival_core::SELECTED_RAW_IRON_ITEM => {
            survival_furnace_input_stack_with_count(stack.count)
        }
        survival_core::SELECTED_COAL_ITEM => survival_furnace_fuel_stack_with_count(stack.count),
        survival_core::SELECTED_IRON_INGOT_ITEM => {
            survival_furnace_output_stack_with_count(stack.count)
        }
        _ => ItemStack::EMPTY,
    }
}

fn survival_furnace_pause_diagnostic(
    transition: survival_core::FurnaceTransition,
) -> SurvivalFurnaceShellDiagnostic {
    match transition {
        survival_core::FurnaceTransition::PausedNoFuel => {
            SurvivalFurnaceShellDiagnostic::PausedNoFuel
        }
        survival_core::FurnaceTransition::PausedNoRecipe => {
            SurvivalFurnaceShellDiagnostic::PausedNoRecipe
        }
        survival_core::FurnaceTransition::PausedOutputBlocked => {
            SurvivalFurnaceShellDiagnostic::PausedOutputBlocked
        }
        survival_core::FurnaceTransition::StartedFuel
        | survival_core::FurnaceTransition::AdvancedCooking
        | survival_core::FurnaceTransition::ProducedOutput => {
            unreachable!("successful furnace transition is not a diagnostic")
        }
    }
}

fn survival_furnace_error_diagnostic(
    error: survival_core::FurnaceError,
) -> SurvivalFurnaceShellDiagnostic {
    match error {
        survival_core::FurnaceError::UnsupportedFurnaceKind => {
            SurvivalFurnaceShellDiagnostic::UnsupportedFurnaceKind
        }
        survival_core::FurnaceError::MalformedRecipeRow => {
            SurvivalFurnaceShellDiagnostic::MalformedRecipeRow
        }
        survival_core::FurnaceError::MalformedFuelRow => {
            SurvivalFurnaceShellDiagnostic::MalformedFuelRow
        }
    }
}

fn record_survival_furnace_diagnostic(
    fixture: &mut SurvivalFurnaceFixture,
    diagnostic_events: &mut EventWriter<SurvivalFurnaceDiagnosticEvent>,
    inventory: Option<Entity>,
    diagnostic: SurvivalFurnaceShellDiagnostic,
) {
    fixture.last_shell_diagnostic = Some(diagnostic);
    diagnostic_events.send(SurvivalFurnaceDiagnosticEvent {
        inventory,
        diagnostic,
    });
}

fn log_survival_furnace_reopen(
    username: &str,
    fixture: &mut SurvivalFurnaceFixture,
    inventories: &Query<&Inventory>,
) {
    if !fixture.reopen_logged {
        fixture.reopen_logged = true;
        log_milestone(format!(
            "MC-COMPAT-MILESTONE survival_furnace_reconnect_reopen username={} position={},{},{} \
             window={}",
            username,
            SURVIVAL_FURNACE_X,
            SURVIVAL_FURNACE_Y,
            SURVIVAL_FURNACE_Z,
            SURVIVAL_FURNACE_WINDOW
        ));
    }

    if fixture.state_logged {
        return;
    }
    let Ok(inventory) = inventories.get(fixture.inventory) else {
        return;
    };
    if !is_empty_item(inventory.slot(SURVIVAL_FURNACE_OUTPUT_SLOT)) {
        return;
    }
    fixture.state_logged = true;
    log_milestone(format!(
        "MC-COMPAT-MILESTONE survival_furnace_server_state username={} position={},{},{} input={} \
         fuel={} output=empty collected=true session_persistent=true",
        username,
        SURVIVAL_FURNACE_X,
        SURVIVAL_FURNACE_Y,
        SURVIVAL_FURNACE_Z,
        SURVIVAL_FURNACE_INPUT_NAME,
        SURVIVAL_FURNACE_FUEL_NAME
    ));
}

fn log_survival_crafting_breadth(username: &str, fixture: &mut SurvivalCraftingBreadthFixture) {
    if fixture.logged {
        return;
    }
    fixture.logged = true;
    log_milestone(format!(
        "MC-COMPAT-MILESTONE survival_crafting_breadth_shaped username={} recipe=minecraft:chest \
         input=oak_planksx8 result=Chest count=1",
        username
    ));
    log_milestone(format!(
        "MC-COMPAT-MILESTONE survival_crafting_breadth_shapeless username={} \
         recipe=minecraft:oak_planks input=oak_logx1 result=OakPlanks count=4",
        username
    ));
    log_milestone(format!(
        "MC-COMPAT-MILESTONE survival_crafting_breadth_grid_clear username={} window=1 \
         occupied_slots=0",
        username
    ));
    log_milestone(format!(
        "MC-COMPAT-MILESTONE survival_crafting_breadth_invalid_rejected username={} \
         recipe=minecraft:stick_insufficient_input_rejection input=single_oak_plank \
         outcome=no_result",
        username
    ));
    log_milestone(format!(
        "MC-COMPAT-MILESTONE survival_crafting_breadth_state username={} shaped=true \
         shapeless=true invalid_rejected=true extra_outputs=false",
        username
    ));
}

fn emit_survival_crafting_fixture_milestones(
    fixture: &mut SurvivalCraftingFixture,
    inventories: &mut Query<&mut Inventory>,
    client_entity: Entity,
    username: &Username,
) {
    if !fixture.input_a_logged {
        fixture.input_a_logged = true;
        set_survival_crafting_slot(
            inventories,
            fixture.inventory,
            SURVIVAL_CRAFTING_INPUT_A_SLOT,
            survival_crafting_input_stack(SURVIVAL_CRAFTING_INPUT_COUNT),
        );
        log_milestone(format!(
            "MC-COMPAT-MILESTONE survival_crafting_input_a username={} window={} slot={} \
             item={:?} count={}",
            username.as_str(),
            SURVIVAL_CRAFTING_WINDOW,
            SURVIVAL_CRAFTING_INPUT_A_SLOT,
            survival_crafting_input_kind(),
            SURVIVAL_CRAFTING_INPUT_COUNT
        ));
    }

    if !fixture.input_b_logged {
        fixture.input_b_logged = true;
        set_survival_crafting_slot(
            inventories,
            fixture.inventory,
            SURVIVAL_CRAFTING_INPUT_B_SLOT,
            survival_crafting_input_stack(SURVIVAL_CRAFTING_INPUT_COUNT),
        );
        log_milestone(format!(
            "MC-COMPAT-MILESTONE survival_crafting_input_b username={} window={} slot={} \
             item={:?} count={}",
            username.as_str(),
            SURVIVAL_CRAFTING_WINDOW,
            SURVIVAL_CRAFTING_INPUT_B_SLOT,
            survival_crafting_input_kind(),
            SURVIVAL_CRAFTING_INPUT_COUNT
        ));
    }

    if !fixture.result_logged {
        fixture.result_logged = true;
        set_survival_crafting_slot(
            inventories,
            fixture.inventory,
            SURVIVAL_CRAFTING_RESULT_SLOT,
            survival_crafting_result_stack(),
        );
        log_milestone(format!(
            "MC-COMPAT-MILESTONE survival_crafting_result username={} window={} slot={} item={:?} \
             count={} recipe={}",
            username.as_str(),
            SURVIVAL_CRAFTING_WINDOW,
            SURVIVAL_CRAFTING_RESULT_SLOT,
            survival_crafting_result_kind(),
            SURVIVAL_CRAFTING_RESULT_COUNT,
            SURVIVAL_CRAFTING_RECIPE
        ));
    }

    if !fixture.collect_logged {
        fixture.collect_logged = true;
        set_survival_crafting_slot(
            inventories,
            client_entity,
            SURVIVAL_CRAFTING_INVENTORY_SLOT,
            survival_crafting_result_stack(),
        );
        log_milestone(format!(
            "MC-COMPAT-MILESTONE survival_crafting_collect username={} window={} slot={} \
             item={:?} count={} inventory_slot={}",
            username.as_str(),
            SURVIVAL_CRAFTING_WINDOW,
            SURVIVAL_CRAFTING_RESULT_SLOT,
            survival_crafting_result_kind(),
            SURVIVAL_CRAFTING_RESULT_COUNT,
            SURVIVAL_CRAFTING_INVENTORY_SLOT
        ));
    }
}

fn log_survival_chest_reopen(
    username: &str,
    fixture: &mut SurvivalChestFixture,
    inventories: &Query<&Inventory>,
) {
    if !fixture.reopen_logged {
        fixture.reopen_logged = true;
        log_milestone(format!(
            "MC-COMPAT-MILESTONE survival_chest_reopen username={} position={},{},{} window={}",
            username, SURVIVAL_CHEST_X, SURVIVAL_CHEST_Y, SURVIVAL_CHEST_Z, SURVIVAL_CHEST_WINDOW
        ));
    }

    if fixture.persisted_logged {
        return;
    }
    let Ok(inventory) = inventories.get(fixture.inventory) else {
        return;
    };
    if !is_survival_chest_item(inventory.slot(SURVIVAL_CHEST_SLOT)) {
        return;
    }
    fixture.persisted_logged = true;
    log_milestone(format!(
        "MC-COMPAT-MILESTONE survival_chest_persisted username={} slot={} item={:?} count={}",
        username,
        SURVIVAL_CHEST_SLOT,
        survival_chest_item_kind(),
        SURVIVAL_CHEST_ITEM_COUNT
    ));
}

fn should_break_survival_block(
    game_mode: GameMode,
    state: DiggingState,
    position: BlockPos,
) -> bool {
    survival_core::should_break_survival_block(
        survival_core_game_mode(game_mode),
        survival_core_digging_state(state),
        survival_core_block_pos(position),
        survival_core_block_pos(survival_break_pos()),
    )
}

fn should_place_survival_block(
    game_mode: GameMode,
    hand: Hand,
    position: BlockPos,
    face: Direction,
) -> bool {
    survival_core::should_place_survival_block(
        survival_core_game_mode(game_mode),
        survival_core_hand(hand),
        survival_core_block_pos(position),
        survival_core_direction(face),
        survival_core_block_pos(survival_break_pos()),
    )
}

fn survival_break_pos() -> BlockPos {
    BlockPos::new(SURVIVAL_TARGET_X, FLOOR_Y, SURVIVAL_TARGET_Z)
}

fn survival_block_state() -> BlockState {
    BlockState::DIRT
}

fn should_open_survival_chest(game_mode: GameMode, hand: Hand, position: BlockPos) -> bool {
    survival_core::should_open_fixture_container(
        survival_core_game_mode(game_mode),
        survival_core_hand(hand),
        survival_core_block_pos(position),
        survival_core_block_pos(survival_chest_pos()),
    )
}

fn is_survival_chest_store_event(window_id: u8, slot_id: i16, slot_changes: &[SlotChange]) -> bool {
    let expected_stack = survival_core_stack(
        &survival_chest_item_stack(),
        survival_chest_item_kind(),
        SURVIVAL_WORLD_PERSISTENCE_BLOCK_NAME,
    );
    let changes = survival_core_slot_changes(
        slot_changes,
        survival_chest_item_kind(),
        SURVIVAL_WORLD_PERSISTENCE_BLOCK_NAME,
    );
    survival_core::slot_event_matches(
        window_id,
        slot_id,
        &changes,
        SURVIVAL_CHEST_WINDOW,
        SURVIVAL_CHEST_SLOT_ID,
        expected_stack,
    )
}

fn is_survival_chest_item(stack: &ItemStack) -> bool {
    survival_core::stack_matches(
        survival_core_stack(
            stack,
            survival_chest_item_kind(),
            SURVIVAL_WORLD_PERSISTENCE_BLOCK_NAME,
        ),
        survival_core::FixtureStack {
            item_name: SURVIVAL_WORLD_PERSISTENCE_BLOCK_NAME,
            count: SURVIVAL_CHEST_ITEM_COUNT,
        },
    )
}

fn should_open_survival_crafting(game_mode: GameMode, hand: Hand, position: BlockPos) -> bool {
    survival_core::should_open_fixture_container(
        survival_core_game_mode(game_mode),
        survival_core_hand(hand),
        survival_core_block_pos(position),
        survival_core_block_pos(survival_crafting_pos()),
    )
}

fn is_survival_crafting_input_event(window_id: u8, slot_id: i16, expected_slot: i16) -> bool {
    // This fixture owns the result state; raw slot/window are the stable
    // server-side trigger.
    window_id == SURVIVAL_CRAFTING_WINDOW && slot_id == expected_slot
}

fn is_survival_crafting_collect_event(
    window_id: u8,
    slot_id: i16,
    carried_item: &ItemStack,
) -> bool {
    survival_core::collect_event_matches(
        window_id,
        slot_id,
        survival_core_stack(
            carried_item,
            survival_crafting_result_kind(),
            SURVIVAL_CRAFTING_RECIPE,
        ),
        SURVIVAL_CRAFTING_WINDOW,
        SURVIVAL_CRAFTING_RESULT_SLOT_ID,
        survival_core_stack(
            &survival_crafting_result_stack(),
            survival_crafting_result_kind(),
            SURVIVAL_CRAFTING_RECIPE,
        ),
    )
}

fn is_survival_crafting_result(stack: &ItemStack) -> bool {
    survival_core::stack_matches(
        survival_core_stack(
            stack,
            survival_crafting_result_kind(),
            SURVIVAL_CRAFTING_RECIPE,
        ),
        survival_core_stack(
            &survival_crafting_result_stack(),
            survival_crafting_result_kind(),
            SURVIVAL_CRAFTING_RECIPE,
        ),
    )
}

fn set_survival_crafting_slot(
    inventories: &mut Query<&mut Inventory>,
    inventory_entity: Entity,
    slot: u16,
    stack: ItemStack,
) {
    set_survival_slot(inventories, inventory_entity, slot, stack);
}

fn set_survival_slot(
    inventories: &mut Query<&mut Inventory>,
    inventory_entity: Entity,
    slot: u16,
    stack: ItemStack,
) {
    if let Ok(mut inventory) = inventories.get_mut(inventory_entity) {
        inventory.set_slot(slot, stack);
    }
}

fn survival_chest_fixture_enabled(config: &SurvivalRuntimeConfig) -> bool {
    config.fixtures.chest
}

fn survival_chest_pos() -> BlockPos {
    BlockPos::new(SURVIVAL_CHEST_X, SURVIVAL_CHEST_Y, SURVIVAL_CHEST_Z)
}

fn survival_chest_item_stack() -> ItemStack {
    ItemStack::new(survival_chest_item_kind(), SURVIVAL_CHEST_ITEM_COUNT, None)
}

fn survival_chest_item_kind() -> ItemKind {
    BlockState::DIRT.to_kind().to_item_kind()
}

fn survival_crafting_fixture_enabled(config: &SurvivalRuntimeConfig) -> bool {
    config.fixtures.crafting
}

fn survival_crafting_breadth_fixture_enabled(config: &SurvivalRuntimeConfig) -> bool {
    config.fixtures.crafting_breadth
}

fn survival_crafting_pos() -> BlockPos {
    BlockPos::new(
        SURVIVAL_CRAFTING_X,
        SURVIVAL_CRAFTING_Y,
        SURVIVAL_CRAFTING_Z,
    )
}

fn survival_crafting_table_state() -> BlockState {
    BlockKind::from_str("crafting_table")
        .expect("crafting_table block kind exists")
        .to_state()
}

fn survival_crafting_input_stack(count: i8) -> ItemStack {
    ItemStack::new(survival_crafting_input_kind(), count, None)
}

fn survival_crafting_result_stack() -> ItemStack {
    ItemStack::new(
        survival_crafting_result_kind(),
        SURVIVAL_CRAFTING_RESULT_COUNT,
        None,
    )
}

fn survival_crafting_input_kind() -> ItemKind {
    ItemKind::OakPlanks
}

fn survival_crafting_result_kind() -> ItemKind {
    ItemKind::Stick
}

fn survival_furnace_fixture_enabled(config: &SurvivalRuntimeConfig) -> bool {
    config.fixtures.furnace
}

fn survival_furnace_smelting_breadth_fixture_enabled(config: &SurvivalRuntimeConfig) -> bool {
    config.fixtures.furnace_smelting_breadth
}

fn survival_hunger_food_fixture_enabled(config: &SurvivalRuntimeConfig) -> bool {
    config.fixtures.hunger_food
}

fn survival_hunger_health_fixture_enabled(config: &SurvivalRuntimeConfig) -> bool {
    config.fixtures.hunger_health
}

fn survival_hunger_profile(config: &SurvivalRuntimeConfig) -> Option<SurvivalHungerProfile> {
    survival_hunger_profile_from_flags(
        survival_hunger_food_fixture_enabled(config),
        survival_hunger_health_fixture_enabled(config),
    )
}

fn survival_hunger_profile_from_flags(
    food_enabled: bool,
    health_enabled: bool,
) -> Option<SurvivalHungerProfile> {
    survival_core::select_hunger_profile(
        food_enabled,
        health_enabled,
        survival_core_hunger_profile(SURVIVAL_HUNGER_FOOD_PROFILE),
        survival_core_hunger_profile(SURVIVAL_HUNGER_HEALTH_PROFILE),
    )
    .map(survival_hunger_profile_from_core)
}

fn survival_furnace_pos() -> BlockPos {
    BlockPos::new(SURVIVAL_FURNACE_X, SURVIVAL_FURNACE_Y, SURVIVAL_FURNACE_Z)
}

fn survival_furnace_state() -> BlockState {
    BlockKind::from_str("furnace")
        .expect("furnace block kind exists")
        .to_state()
}

fn should_open_survival_furnace(game_mode: GameMode, hand: Hand, position: BlockPos) -> bool {
    survival_core::should_open_fixture_container(
        survival_core_game_mode(game_mode),
        survival_core_hand(hand),
        survival_core_block_pos(position),
        survival_core_block_pos(survival_furnace_pos()),
    )
}

fn is_survival_furnace_input_event(window_id: u8, slot_id: i16) -> bool {
    // This fixture owns the furnace state; raw slot/window are stable server-side
    // trigger.
    window_id == SURVIVAL_FURNACE_WINDOW && slot_id == SURVIVAL_FURNACE_INPUT_SLOT_ID
}

fn is_survival_furnace_fuel_event(window_id: u8, slot_id: i16) -> bool {
    // This fixture owns the furnace state; raw slot/window are stable server-side
    // trigger.
    window_id == SURVIVAL_FURNACE_WINDOW && slot_id == SURVIVAL_FURNACE_FUEL_SLOT_ID
}

fn is_survival_furnace_collect_event(
    window_id: u8,
    slot_id: i16,
    carried_item: &ItemStack,
) -> bool {
    survival_core::collect_event_matches(
        window_id,
        slot_id,
        survival_core_stack(
            carried_item,
            survival_furnace_output_kind(),
            SURVIVAL_FURNACE_OUTPUT_NAME,
        ),
        SURVIVAL_FURNACE_WINDOW,
        SURVIVAL_FURNACE_OUTPUT_SLOT_ID,
        survival_core_stack(
            &survival_furnace_output_stack(),
            survival_furnace_output_kind(),
            SURVIVAL_FURNACE_OUTPUT_NAME,
        ),
    )
}

fn should_emit_survival_furnace_breadth_rejection(
    smelting_breadth_enabled: bool,
    collect_logged: bool,
) -> bool {
    survival_core::should_emit_furnace_breadth_rejection(
        smelting_breadth_enabled && collect_logged,
        false,
    )
}

fn should_reject_survival_furnace_invalid_fuel(
    smelting_breadth_enabled: bool,
    collect_logged: bool,
    window_id: u8,
    slot_id: i16,
) -> bool {
    survival_core::should_reject_furnace_invalid_fuel(
        smelting_breadth_enabled,
        collect_logged,
        window_id,
        slot_id,
        SURVIVAL_FURNACE_WINDOW,
        SURVIVAL_FURNACE_FUEL_SLOT_ID,
    )
}

fn is_survival_furnace_output(stack: &ItemStack) -> bool {
    survival_core::stack_matches(
        survival_core_stack(
            stack,
            survival_furnace_output_kind(),
            SURVIVAL_FURNACE_OUTPUT_NAME,
        ),
        survival_core_stack(
            &survival_furnace_output_stack(),
            survival_furnace_output_kind(),
            SURVIVAL_FURNACE_OUTPUT_NAME,
        ),
    )
}

fn is_empty_item(stack: &ItemStack) -> bool {
    stack.count == 0
}

fn survival_furnace_input_stack() -> ItemStack {
    survival_furnace_input_stack_with_count(SURVIVAL_FURNACE_ITEM_COUNT)
}

fn survival_furnace_input_stack_with_count(count: i8) -> ItemStack {
    ItemStack::new(survival_furnace_input_kind(), count, None)
}

fn survival_furnace_fuel_stack() -> ItemStack {
    survival_furnace_fuel_stack_with_count(SURVIVAL_FURNACE_ITEM_COUNT)
}

fn survival_furnace_fuel_stack_with_count(count: i8) -> ItemStack {
    ItemStack::new(survival_furnace_fuel_kind(), count, None)
}

fn survival_furnace_output_stack() -> ItemStack {
    survival_furnace_output_stack_with_count(SURVIVAL_FURNACE_ITEM_COUNT)
}

fn survival_furnace_output_stack_with_count(count: i8) -> ItemStack {
    ItemStack::new(survival_furnace_output_kind(), count, None)
}

fn survival_furnace_input_kind() -> ItemKind {
    ItemKind::RawIron
}

fn survival_furnace_fuel_kind() -> ItemKind {
    ItemKind::Coal
}

fn survival_furnace_output_kind() -> ItemKind {
    ItemKind::IronIngot
}

fn survival_hunger_food_stack() -> ItemStack {
    ItemStack::new(
        survival_hunger_food_kind(),
        SURVIVAL_HUNGER_FOOD_ITEM_COUNT_BEFORE,
        None,
    )
}

fn survival_hunger_food_kind() -> ItemKind {
    ItemKind::Bread
}

fn survival_mob_drop_fixture_enabled(config: &SurvivalRuntimeConfig) -> bool {
    config.fixtures.mob_drop
}

fn survival_redstone_toggle_fixture_enabled(config: &SurvivalRuntimeConfig) -> bool {
    config.fixtures.redstone_toggle
}

fn setup_survival_redstone_toggle_arena(layer: &mut LayerBundle) {
    for x in SURVIVAL_REDSTONE_TOGGLE_ARENA_MIN_X..SURVIVAL_REDSTONE_TOGGLE_ARENA_MAX_X {
        for z in SURVIVAL_REDSTONE_TOGGLE_ARENA_MIN_Z..SURVIVAL_REDSTONE_TOGGLE_ARENA_MAX_Z {
            layer
                .chunk
                .set_block([x, SURVIVAL_REDSTONE_TOGGLE_FLOOR_Y, z], BlockState::STONE);
            layer
                .chunk
                .set_block([x, SURVIVAL_REDSTONE_TOGGLE_CONTROL_Y, z], BlockState::AIR);
        }
    }
}

fn survival_redstone_toggle_control_pos() -> BlockPos {
    BlockPos::new(
        SURVIVAL_REDSTONE_TOGGLE_CONTROL_X,
        SURVIVAL_REDSTONE_TOGGLE_CONTROL_Y,
        SURVIVAL_REDSTONE_TOGGLE_CONTROL_Z,
    )
}

fn survival_redstone_toggle_output_pos() -> BlockPos {
    BlockPos::new(
        SURVIVAL_REDSTONE_TOGGLE_OUTPUT_X,
        SURVIVAL_REDSTONE_TOGGLE_OUTPUT_Y,
        SURVIVAL_REDSTONE_TOGGLE_OUTPUT_Z,
    )
}

fn survival_redstone_toggle_control_state(powered: bool) -> BlockState {
    BlockKind::from_str("lever")
        .expect("lever block kind exists")
        .to_state()
        .set(PropName::Powered, prop_bool(powered))
}

fn survival_redstone_toggle_output_state(powered: bool) -> BlockState {
    BlockKind::from_str("redstone_lamp")
        .expect("redstone_lamp block kind exists")
        .to_state()
        .set(PropName::Lit, prop_bool(powered))
}

fn prop_bool(value: bool) -> PropValue {
    if value {
        PropValue::True
    } else {
        PropValue::False
    }
}

fn should_toggle_survival_redstone(game_mode: GameMode, hand: Hand, position: BlockPos) -> bool {
    survival_core::should_open_fixture_container(
        survival_core_game_mode(game_mode),
        survival_core_hand(hand),
        survival_core_block_pos(position),
        survival_core_block_pos(survival_redstone_toggle_control_pos()),
    )
}

fn survival_world_persistence_fixture_enabled(config: &SurvivalRuntimeConfig) -> bool {
    config.fixtures.world_persistence
}

fn survival_world_persistence_marker_path(config: &SurvivalRuntimeConfig) -> PathBuf {
    config.paths.world_persistence_marker.clone()
}

fn setup_survival_world_persistence_arena(layer: &mut LayerBundle) {
    layer.chunk.set_block(
        [
            SURVIVAL_WORLD_PERSISTENCE_X,
            SURVIVAL_WORLD_PERSISTENCE_BASE_Y,
            SURVIVAL_WORLD_PERSISTENCE_Z,
        ],
        BlockState::STONE,
    );
}

fn survival_world_persistence_pos() -> BlockPos {
    BlockPos::new(
        SURVIVAL_WORLD_PERSISTENCE_X,
        SURVIVAL_WORLD_PERSISTENCE_Y,
        SURVIVAL_WORLD_PERSISTENCE_Z,
    )
}

fn survival_world_persistence_base_pos() -> BlockPos {
    BlockPos::new(
        SURVIVAL_WORLD_PERSISTENCE_X,
        SURVIVAL_WORLD_PERSISTENCE_BASE_Y,
        SURVIVAL_WORLD_PERSISTENCE_Z,
    )
}

fn survival_world_persistence_state() -> BlockState {
    survival_block_state()
}

fn survival_world_persistence_stack() -> ItemStack {
    ItemStack::new(
        survival_item_kind(),
        SURVIVAL_WORLD_PERSISTENCE_ITEM_COUNT,
        None,
    )
}

fn is_survival_world_persistence_stack(stack: &ItemStack) -> bool {
    let observed = survival_core_stack(
        stack,
        survival_item_kind(),
        SURVIVAL_WORLD_PERSISTENCE_BLOCK_NAME,
    );
    observed.item_name == SURVIVAL_WORLD_PERSISTENCE_BLOCK_NAME
        && observed.count >= SURVIVAL_WORLD_PERSISTENCE_ITEM_COUNT
}

fn should_place_survival_world_persistence(
    game_mode: GameMode,
    hand: Hand,
    position: BlockPos,
    face: Direction,
) -> bool {
    survival_core::should_place_survival_block(
        survival_core_game_mode(game_mode),
        survival_core_hand(hand),
        survival_core_block_pos(position),
        survival_core_direction(face),
        survival_core_block_pos(survival_world_persistence_base_pos()),
    )
}

fn write_survival_world_persistence_marker(path: &PathBuf) {
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let _ = fs::write(path, SURVIVAL_WORLD_PERSISTENCE_BLOCK_NAME);
}

fn log_survival_world_persistence_post_restart(
    username: &str,
    client: &mut Client,
    fixture: &mut SurvivalWorldPersistenceFixture,
) {
    if !fixture.persisted_loaded || fixture.state_logged {
        return;
    }
    let state = survival_world_persistence_state();
    client.write_packet(&BlockUpdateS2c {
        position: survival_world_persistence_pos(),
        block_id: state,
    });
    fixture.state_logged = true;
    log_milestone(format!(
        "MC-COMPAT-MILESTONE survival_world_persistence_post_restart_observe username={} block={} \
         position={},{},{} persisted=true",
        username,
        SURVIVAL_WORLD_PERSISTENCE_BLOCK_NAME,
        SURVIVAL_WORLD_PERSISTENCE_X,
        SURVIVAL_WORLD_PERSISTENCE_Y,
        SURVIVAL_WORLD_PERSISTENCE_Z
    ));
    log_milestone(format!(
        "MC-COMPAT-MILESTONE survival_world_persistence_state username={} block={} \
         position={},{},{} pre_mutation=true clean_shutdown=true backend_restart=true \
         post_observed=true dirty_reuse=false",
        username,
        SURVIVAL_WORLD_PERSISTENCE_BLOCK_NAME,
        SURVIVAL_WORLD_PERSISTENCE_X,
        SURVIVAL_WORLD_PERSISTENCE_Y,
        SURVIVAL_WORLD_PERSISTENCE_Z
    ));
}

fn survival_block_entity_fixture_enabled(config: &SurvivalRuntimeConfig) -> bool {
    config.fixtures.block_entity
}

fn survival_block_entity_post_restart_phase(config: &SurvivalRuntimeConfig) -> bool {
    config.fixtures.block_entity_post_restart
}

fn survival_block_entity_marker_path(config: &SurvivalRuntimeConfig) -> PathBuf {
    config.paths.block_entity_marker.clone()
}

fn survival_block_entity_should_place_sign(
    config: &SurvivalRuntimeConfig,
    persisted_loaded: bool,
) -> bool {
    survival_core::should_place_block_entity_sign(
        survival_block_entity_post_restart_phase(config),
        persisted_loaded,
    )
}

fn setup_survival_block_entity_arena(layer: &mut LayerBundle) {
    layer.chunk.set_block(
        [
            SURVIVAL_BLOCK_ENTITY_X,
            SURVIVAL_BLOCK_ENTITY_BASE_Y,
            SURVIVAL_BLOCK_ENTITY_Z,
        ],
        BlockState::STONE,
    );
}

fn survival_block_entity_pos() -> BlockPos {
    BlockPos::new(
        SURVIVAL_BLOCK_ENTITY_X,
        SURVIVAL_BLOCK_ENTITY_Y,
        SURVIVAL_BLOCK_ENTITY_Z,
    )
}

fn survival_block_entity_block() -> Block {
    Block {
        state: BlockState::OAK_SIGN.set(PropName::Rotation, PropValue::_0),
        nbt: Some(survival_block_entity_nbt()),
    }
}

fn survival_block_entity_nbt() -> valence::nbt::Compound<String> {
    compound! {
        "front_text" => compound! {
            "messages" => List::String(vec![
                SURVIVAL_BLOCK_ENTITY_TEXT_LINE_1.into_text().into(),
                SURVIVAL_BLOCK_ENTITY_TEXT_LINE_2.into_text().into(),
                SURVIVAL_BLOCK_ENTITY_TEXT_LINE_3.into_text().into(),
                SURVIVAL_BLOCK_ENTITY_TEXT_LINE_4.into_text().into(),
            ]),
        }
    }
}

fn write_survival_block_entity_marker(path: &PathBuf) {
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let _ = fs::write(path, SURVIVAL_BLOCK_ENTITY_TEXT_PAYLOAD);
}

fn log_survival_block_entity_persistence(
    config: &SurvivalRuntimeConfig,
    username: &str,
    fixture: &mut SurvivalBlockEntityFixture,
) {
    if survival_block_entity_post_restart_phase(config) {
        log_survival_block_entity_post_restart(username, fixture);
    } else {
        log_survival_block_entity_mutation(username, fixture);
    }
}

fn log_survival_block_entity_mutation(username: &str, fixture: &mut SurvivalBlockEntityFixture) {
    if fixture.mutation_logged {
        return;
    }
    write_survival_block_entity_marker(&fixture.marker_path);
    fixture.persisted_loaded = true;
    fixture.mutation_logged = true;
    log_milestone(format!(
        "MC-COMPAT-MILESTONE survival_block_entity_persistence_mutation username={} kind={} \
         position={},{},{} text={} persisted_before=false persisted_after=true",
        username,
        SURVIVAL_BLOCK_ENTITY_KIND,
        SURVIVAL_BLOCK_ENTITY_X,
        SURVIVAL_BLOCK_ENTITY_Y,
        SURVIVAL_BLOCK_ENTITY_Z,
        SURVIVAL_BLOCK_ENTITY_TEXT_PAYLOAD
    ));
}

fn log_survival_block_entity_post_restart(
    username: &str,
    fixture: &mut SurvivalBlockEntityFixture,
) {
    if !fixture.persisted_loaded || fixture.state_logged {
        return;
    }
    fixture.state_logged = true;
    log_milestone(format!(
        "MC-COMPAT-MILESTONE survival_block_entity_persistence_post_restart_observe username={} \
         kind={} position={},{},{} text={} persisted=true",
        username,
        SURVIVAL_BLOCK_ENTITY_KIND,
        SURVIVAL_BLOCK_ENTITY_X,
        SURVIVAL_BLOCK_ENTITY_Y,
        SURVIVAL_BLOCK_ENTITY_Z,
        SURVIVAL_BLOCK_ENTITY_TEXT_PAYLOAD
    ));
    log_milestone(format!(
        "MC-COMPAT-MILESTONE survival_block_entity_persistence_state username={} kind={} \
         position={},{},{} text={} pre_mutation=true clean_shutdown=true backend_restart=true \
         post_observed=true dirty_reuse=false",
        username,
        SURVIVAL_BLOCK_ENTITY_KIND,
        SURVIVAL_BLOCK_ENTITY_X,
        SURVIVAL_BLOCK_ENTITY_Y,
        SURVIVAL_BLOCK_ENTITY_Z,
        SURVIVAL_BLOCK_ENTITY_TEXT_PAYLOAD
    ));
}

fn survival_mob_drop_position() -> Position {
    Position::new(DVec3::new(
        SURVIVAL_MOB_DROP_MOB_X,
        SURVIVAL_MOB_DROP_MOB_Y,
        SURVIVAL_MOB_DROP_MOB_Z,
    ))
}

fn survival_mob_drop_stack() -> ItemStack {
    ItemStack::new(
        survival_mob_drop_item_kind(),
        SURVIVAL_MOB_DROP_ITEM_COUNT,
        None,
    )
}

fn survival_mob_drop_item_kind() -> ItemKind {
    ItemKind::IronIngot
}

fn is_survival_mob_drop_stack(stack: &ItemStack) -> bool {
    survival_core::stack_matches(
        survival_core_stack(
            stack,
            survival_mob_drop_item_kind(),
            SURVIVAL_MOB_DROP_ITEM_NAME,
        ),
        survival_core_stack(
            &survival_mob_drop_stack(),
            survival_mob_drop_item_kind(),
            SURVIVAL_MOB_DROP_ITEM_NAME,
        ),
    )
}

fn should_handle_survival_mob_drop_attack(
    game_mode: GameMode,
    interaction: EntityInteraction,
    target: Entity,
    expected_target: Entity,
) -> bool {
    survival_core::should_handle_mob_drop_attack(
        survival_core_game_mode(game_mode),
        survival_core_entity_interaction(interaction),
        target.index(),
        expected_target.index(),
    )
}

fn spawn_survival_mob_drop_item(
    commands: &mut Commands,
    entity_manager: &mut EntityManager,
    fixture: &mut SurvivalMobDropFixture,
    layer: Entity,
    collector: Entity,
    username: &str,
) {
    let drop_id = entity_manager.next_id();
    commands.spawn((
        ItemEntityBundle {
            id: drop_id,
            layer: EntityLayerId(layer),
            position: survival_mob_drop_position(),
            item_stack: ItemEntityStack(survival_mob_drop_stack()),
            ..Default::default()
        },
        SurvivalMobDropItem::new(drop_id.get(), collector),
    ));
    fixture.drop_logged = true;
    log_milestone(format!(
        "MC-COMPAT-MILESTONE survival_mob_drop_drop_spawn username={} item={} count={} \
         entity_id={} position={:.1},{:.1},{:.1}",
        username,
        SURVIVAL_MOB_DROP_ITEM_NAME,
        SURVIVAL_MOB_DROP_ITEM_COUNT,
        drop_id.get(),
        SURVIVAL_MOB_DROP_MOB_X,
        SURVIVAL_MOB_DROP_MOB_Y,
        SURVIVAL_MOB_DROP_MOB_Z
    ));
}

fn log_survival_mob_drop_spawn(username: &str, fixture: &mut SurvivalMobDropFixture) {
    if fixture.spawn_logged {
        return;
    }
    fixture.spawn_logged = true;
    log_milestone(format!(
        "MC-COMPAT-MILESTONE survival_mob_drop_spawn username={} mob={} \
         position={:.1},{:.1},{:.1} entity_id={}",
        username,
        SURVIVAL_MOB_DROP_MOB_NAME,
        SURVIVAL_MOB_DROP_MOB_X,
        SURVIVAL_MOB_DROP_MOB_Y,
        SURVIVAL_MOB_DROP_MOB_Z,
        fixture.mob_id
    ));
}

fn log_survival_mob_drop_pickup_and_state(
    username: &str,
    fixture: &mut SurvivalMobDropFixture,
    drop_id: i32,
    collector_id: i32,
) {
    if !fixture.pickup_logged {
        fixture.pickup_logged = true;
        log_milestone(format!(
            "MC-COMPAT-MILESTONE survival_mob_drop_pickup username={} item={} count={} \
             collected_entity_id={} collector_entity_id={}",
            username,
            SURVIVAL_MOB_DROP_ITEM_NAME,
            SURVIVAL_MOB_DROP_ITEM_COUNT,
            drop_id,
            collector_id
        ));
    }
    if !fixture.inventory_logged {
        fixture.inventory_logged = true;
        log_milestone(format!(
            "MC-COMPAT-MILESTONE survival_mob_drop_inventory username={} slot={} item={} count={}",
            username,
            SURVIVAL_MOB_DROP_INVENTORY_SLOT,
            SURVIVAL_MOB_DROP_ITEM_NAME,
            SURVIVAL_MOB_DROP_ITEM_COUNT
        ));
    }
    if !fixture.state_logged {
        fixture.state_logged = true;
        log_milestone(format!(
            "MC-COMPAT-MILESTONE survival_mob_drop_state username={} mob={} drop={} count={} \
             extra_drops=false",
            username,
            SURVIVAL_MOB_DROP_MOB_NAME,
            SURVIVAL_MOB_DROP_ITEM_NAME,
            SURVIVAL_MOB_DROP_ITEM_COUNT
        ));
    }
}

fn is_survival_hunger_food_stack(stack: &ItemStack) -> bool {
    stack.item == survival_hunger_food_kind()
        && stack.count == SURVIVAL_HUNGER_FOOD_ITEM_COUNT_BEFORE
}

fn should_consume_survival_hunger_food(
    profile: SurvivalHungerProfile,
    hand: Hand,
    sequence: i32,
    held_slot: u16,
    stack: &ItemStack,
    health: f32,
    food: i32,
    saturation: f32,
) -> bool {
    survival_core::should_consume_hunger_food(
        survival_core_hunger_profile(profile),
        survival_core::HungerUseInput {
            hand: survival_core_hand(hand),
            sequence,
            slot: held_slot,
            stack: survival_core_stack(
                stack,
                survival_hunger_food_kind(),
                SURVIVAL_HUNGER_FOOD_ITEM_NAME,
            ),
            health_tenths: survival_core_tenths(health),
            food,
            saturation_tenths: survival_core_tenths(saturation),
        },
        survival_core::HungerUseContract {
            expected_sequence: SURVIVAL_HUNGER_FOOD_USE_SEQUENCE,
            expected_slot: SURVIVAL_HUNGER_FOOD_INVENTORY_SLOT,
            expected_stack: survival_core_stack(
                &survival_hunger_food_stack(),
                survival_hunger_food_kind(),
                SURVIVAL_HUNGER_FOOD_ITEM_NAME,
            ),
        },
    )
}

fn log_survival_hunger_food_pre(username: &str, fixture: &mut SurvivalHungerFoodFixture) {
    if fixture.pre_logged {
        return;
    }
    fixture.pre_logged = true;
    log_milestone(format!(
        "MC-COMPAT-MILESTONE {}_pre username={} health={:.1} food={} saturation={:.1} item={} \
         count={} slot={}",
        fixture.profile.event_prefix,
        username,
        fixture.profile.pre_health,
        SURVIVAL_HUNGER_FOOD_PRE_FOOD,
        SURVIVAL_HUNGER_FOOD_PRE_SATURATION,
        SURVIVAL_HUNGER_FOOD_ITEM_NAME,
        SURVIVAL_HUNGER_FOOD_ITEM_COUNT_BEFORE,
        SURVIVAL_HUNGER_FOOD_INVENTORY_SLOT
    ));
}

fn survival_biome_dimension_fixture_enabled(config: &SurvivalRuntimeConfig) -> bool {
    config.fixtures.biome_dimension
}

fn survival_mob_ai_loot_fixture_enabled(config: &SurvivalRuntimeConfig) -> bool {
    config.fixtures.mob_ai_loot
}

fn survival_redstone_circuit_fixture_enabled(config: &SurvivalRuntimeConfig) -> bool {
    config.fixtures.redstone_circuit
}

fn survival_world_multichunk_fixture_enabled(config: &SurvivalRuntimeConfig) -> bool {
    config.fixtures.world_multichunk
}

fn survival_world_multichunk_post_restart_phase(config: &SurvivalRuntimeConfig) -> bool {
    config.fixtures.world_multichunk_post_restart
}

fn survival_container_block_entity_fixture_enabled(config: &SurvivalRuntimeConfig) -> bool {
    config.fixtures.container_block_entity
}

fn survival_biome_dimension_travel_fixture_enabled(config: &SurvivalRuntimeConfig) -> bool {
    config.fixtures.biome_dimension_travel
}

fn survival_sign_editing_fixture_enabled(config: &SurvivalRuntimeConfig) -> bool {
    config.fixtures.sign_editing
}

fn log_survival_breadth_synthetic_fixtures(config: &SurvivalRuntimeConfig, username: &str) {
    if survival_mob_ai_loot_fixture_enabled(config) {
        log_survival_mob_ai_loot_breadth(username);
    }
    if survival_redstone_circuit_fixture_enabled(config) {
        log_survival_redstone_circuit_breadth(username);
    }
    if survival_world_multichunk_fixture_enabled(config) {
        log_survival_world_multichunk_breadth(config, username);
    }
    if survival_container_block_entity_fixture_enabled(config) {
        log_survival_container_block_entity_breadth(username);
    }
    if survival_biome_dimension_travel_fixture_enabled(config) {
        log_survival_biome_dimension_travel_breadth(username);
    }
    if survival_sign_editing_fixture_enabled(config) {
        log_survival_sign_editing_live_breadth(username);
    }
}

fn log_survival_mob_ai_loot_breadth(username: &str) {
    for milestone in survival_core::breadth::mob_ai_loot_milestones(username) {
        log_milestone(milestone);
    }
}

fn log_survival_redstone_circuit_breadth(username: &str) {
    for milestone in survival_core::breadth::redstone_circuit_milestones(username) {
        log_milestone(milestone);
    }
}

fn log_survival_world_multichunk_breadth(config: &SurvivalRuntimeConfig, username: &str) {
    let post_restart = survival_world_multichunk_post_restart_phase(config);
    for milestone in survival_core::breadth::world_multichunk_milestones(username, post_restart) {
        log_milestone(milestone);
    }
}

fn log_survival_container_block_entity_breadth(username: &str) {
    for milestone in survival_core::breadth::container_block_entity_milestones(username) {
        log_milestone(milestone);
    }
}

fn log_survival_biome_dimension_travel_breadth(username: &str) {
    for milestone in survival_core::breadth::biome_dimension_travel_milestones(username) {
        log_milestone(milestone);
    }
}

fn log_survival_sign_editing_live_breadth(username: &str) {
    for milestone in survival_core::sign_editing::live_breadth_milestones(username) {
        log_milestone(milestone);
    }
}

fn normalize_survival_environment_id(raw: &str) -> &'static str {
    survival_core::normalize_environment_id(
        raw,
        &survival_known_environment_ids(),
        SURVIVAL_UNKNOWN_ENVIRONMENT_ID,
    )
}

fn derive_survival_environment_id(
    spawn_environment: &str,
    environment_identifier: &str,
) -> &'static str {
    survival_core::derive_environment_id(
        spawn_environment,
        environment_identifier,
        &survival_known_environment_ids(),
        SURVIVAL_UNKNOWN_ENVIRONMENT_ID,
    )
}

fn log_survival_biome_dimension_state(
    username: &str,
    spawn_environment: &str,
    environment_identifier: &str,
) {
    let normalized_identifier =
        derive_survival_environment_id(spawn_environment, environment_identifier);
    log_milestone(format!(
        "MC-COMPAT-MILESTONE survival_biome_dimension_state username={} spawn_environment={} \
         environment_identifier={} server_environment_state={} normalized_identifier={}",
        username,
        spawn_environment,
        environment_identifier,
        spawn_environment,
        normalized_identifier
    ));
}

fn survival_known_environment_ids() -> [&'static str; SURVIVAL_KNOWN_ENVIRONMENT_COUNT] {
    [SURVIVAL_OVERWORLD_ID, SURVIVAL_NETHER_ID, SURVIVAL_END_ID]
}

fn survival_core_game_mode(game_mode: GameMode) -> survival_core::FixtureGameMode {
    if game_mode == GameMode::Survival {
        survival_core::FixtureGameMode::Survival
    } else {
        survival_core::FixtureGameMode::Other
    }
}

fn survival_core_hand(hand: Hand) -> survival_core::FixtureHand {
    if hand == Hand::Main {
        survival_core::FixtureHand::Main
    } else {
        survival_core::FixtureHand::Other
    }
}

fn survival_core_direction(direction: Direction) -> survival_core::FixtureDirection {
    if direction == Direction::Up {
        survival_core::FixtureDirection::Up
    } else {
        survival_core::FixtureDirection::Other
    }
}

fn survival_core_digging_state(state: DiggingState) -> survival_core::FixtureDiggingState {
    if state == DiggingState::Stop {
        survival_core::FixtureDiggingState::Stop
    } else {
        survival_core::FixtureDiggingState::Other
    }
}

fn survival_core_entity_interaction(
    interaction: EntityInteraction,
) -> survival_core::FixtureInteraction {
    if interaction == EntityInteraction::Attack {
        survival_core::FixtureInteraction::Attack
    } else {
        survival_core::FixtureInteraction::Other
    }
}

fn survival_core_block_pos(position: BlockPos) -> survival_core::FixtureBlockPos {
    survival_core::FixtureBlockPos {
        x: position.x,
        y: position.y,
        z: position.z,
    }
}

fn survival_core_stack<'a>(
    stack: &ItemStack,
    expected_item: ItemKind,
    expected_name: &'a str,
) -> survival_core::FixtureStack<'a> {
    let item_name = if stack.item == expected_item {
        expected_name
    } else {
        SURVIVAL_OTHER_ITEM_NAME
    };
    survival_core::FixtureStack {
        item_name,
        count: stack.count,
    }
}

fn survival_core_slot_changes<'a>(
    slot_changes: &[SlotChange],
    expected_item: ItemKind,
    expected_name: &'a str,
) -> Vec<survival_core::FixtureSlotChange<'a>> {
    slot_changes
        .iter()
        .map(|change| survival_core::FixtureSlotChange {
            slot: change.idx,
            stack: survival_core_stack(&change.stack, expected_item, expected_name),
        })
        .collect()
}

fn survival_core_hunger_profile(
    profile: SurvivalHungerProfile,
) -> survival_core::FixtureHungerProfile {
    let (post_food, post_saturation) =
        if profile.event_prefix == SURVIVAL_HUNGER_HEALTH_EVENT_PREFIX {
            (
                SURVIVAL_HUNGER_FOOD_PRE_FOOD,
                SURVIVAL_HUNGER_FOOD_PRE_SATURATION,
            )
        } else {
            (
                SURVIVAL_HUNGER_FOOD_POST_FOOD,
                SURVIVAL_HUNGER_FOOD_POST_SATURATION,
            )
        };
    survival_core::FixtureHungerProfile {
        event_prefix: profile.event_prefix,
        pre_health_tenths: survival_core_tenths(profile.pre_health),
        pre_food: SURVIVAL_HUNGER_FOOD_PRE_FOOD,
        pre_saturation_tenths: survival_core_tenths(SURVIVAL_HUNGER_FOOD_PRE_SATURATION),
        post_health_tenths: survival_core_tenths(profile.post_health),
        post_food,
        post_saturation_tenths: survival_core_tenths(post_saturation),
    }
}

fn survival_hunger_profile_from_core(
    profile: survival_core::FixtureHungerProfile,
) -> SurvivalHungerProfile {
    if profile.event_prefix == SURVIVAL_HUNGER_HEALTH_EVENT_PREFIX {
        SURVIVAL_HUNGER_HEALTH_PROFILE
    } else {
        SURVIVAL_HUNGER_FOOD_PROFILE
    }
}

fn survival_core_tenths(value: f32) -> i32 {
    (value * SURVIVAL_CORE_TENTHS_SCALE).round() as i32
}

fn survival_item_kind() -> ItemKind {
    survival_block_state().to_kind().to_item_kind()
}

fn log_milestone(message: String) {
    info!("{message}");
    println!("{message}");
}

#[cfg(test)]
mod tests {
    use super::*;

    use bevy_ecs::schedule::Schedule;
    use valence::inventory::ClickMode;

    const UPDATE_SCHEDULE_LABEL: &str = "Update";
    const SURVIVAL_EVENT_LOOP_SCHEDULE_LABEL: &str = "EventLoopPreUpdate";
    const TEST_USERNAME: &str = "compatbot";
    const TEST_CLICK_STATE_ID: i32 = 42;
    const TEST_CLICK_BUTTON: i8 = 0;
    const TEST_MOB_DROP_ID: i32 = 42_101;
    const TEST_DUPLICATE_MOB_DROP_ID: i32 = 42_102;
    const FURNACE_COMPATIBLE_OUTPUT_START_COUNT: i8 = 7;
    const FURNACE_EXPECTED_MERGED_OUTPUT_COUNT: i8 = 8;
    const FURNACE_TWO_INPUT_ITEMS: i8 = 2;
    const FURNACE_EXPECTED_ONE_COMPLETED_RECIPE: u32 = 1;
    const FURNACE_NEAR_COMPLETE_PROGRESS: u32 =
        survival_core::SELECTED_STANDARD_FURNACE_COOK_TICKS - survival_core::FURNACE_TICK_INCREMENT;
    const FURNACE_EXPECTED_COAL_AFTER_START: u32 =
        survival_core::SELECTED_COAL_BURN_TICKS - survival_core::FURNACE_TICK_INCREMENT;
    const FURNACE_EXPECTED_PROGRESS_AFTER_TICK: u32 =
        survival_core::INITIAL_COOK_PROGRESS_TICKS + survival_core::FURNACE_TICK_INCREMENT;

    fn app_with_survival_event_loop_schedule() -> App {
        let mut app = App::new();
        app.add_schedule(Schedule::new(EventLoopPreUpdate));
        app
    }

    fn app_has_schedule(app: &App, schedule_label: &str) -> bool {
        app.world()
            .resource::<Schedules>()
            .iter()
            .any(|(label, _)| format!("{label:?}") == schedule_label)
    }

    fn app_with_chest_store_system() -> App {
        let mut app = App::new();
        app.add_event::<ClickSlotEvent>()
            .add_systems(Update, handle_survival_chest_store);
        app
    }

    fn app_with_mob_drop_pickup_system() -> App {
        let mut app = App::new();
        app.add_systems(
            Update,
            advance_survival_mob_drop_pickup.run_if(survival_mob_drop_pickup_fixture_present),
        );
        app
    }

    fn spawn_chest_store_client(app: &mut App) -> Entity {
        app.world_mut()
            .spawn((
                Username(TEST_USERNAME.to_owned()),
                SurvivalOpenContainer::new(SurvivalContainerKind::Chest),
            ))
            .id()
    }

    fn insert_chest_fixture(app: &mut App) {
        let inventory = app.world_mut().spawn_empty().id();
        app.insert_resource(SurvivalChestFixture::new(inventory));
    }

    fn send_chest_store_click(app: &mut App, client: Entity) {
        app.world_mut()
            .resource_mut::<Events<ClickSlotEvent>>()
            .send(ClickSlotEvent {
                client,
                window_id: SURVIVAL_CHEST_WINDOW,
                state_id: TEST_CLICK_STATE_ID,
                slot_id: SURVIVAL_CHEST_SLOT_ID,
                button: TEST_CLICK_BUTTON,
                mode: ClickMode::Click,
                slot_changes: vec![SlotChange {
                    idx: SURVIVAL_CHEST_SLOT_ID,
                    stack: survival_chest_item_stack(),
                }],
                carried_item: ItemStack::EMPTY,
            });
    }

    fn app_with_furnace_shell_fixture() -> (App, Entity) {
        let mut app = App::new();
        app.add_plugins(SurvivalFurnaceSmeltingPlugin);
        let inventory = app
            .world_mut()
            .spawn(Inventory::with_title(
                InventoryKind::Furnace,
                SURVIVAL_FURNACE_TITLE,
            ))
            .id();
        app.insert_resource(SurvivalFurnaceFixture::new(inventory, true));
        (app, inventory)
    }

    fn app_with_disabled_furnace_shell_fixture() -> (App, Entity) {
        let mut app = App::new();
        let inventory = app
            .world_mut()
            .spawn(Inventory::with_title(
                InventoryKind::Furnace,
                SURVIVAL_FURNACE_TITLE,
            ))
            .id();
        app.insert_resource(SurvivalFurnaceFixture::new(inventory, true));
        (app, inventory)
    }

    fn set_furnace_slots(
        app: &mut App,
        inventory: Entity,
        input: ItemStack,
        fuel: ItemStack,
        output: ItemStack,
    ) {
        let mut inventory = app
            .world_mut()
            .get_mut::<Inventory>(inventory)
            .expect("furnace inventory exists");
        inventory.set_slot(SURVIVAL_FURNACE_INPUT_SLOT, input);
        inventory.set_slot(SURVIVAL_FURNACE_FUEL_SLOT, fuel);
        inventory.set_slot(SURVIVAL_FURNACE_OUTPUT_SLOT, output);
    }

    fn furnace_slot(app: &App, inventory: Entity, slot: u16) -> ItemStack {
        app.world()
            .get::<Inventory>(inventory)
            .expect("furnace inventory exists")
            .slot(slot)
            .clone()
    }

    fn set_furnace_progress(app: &mut App, cook_progress_ticks: u32, remaining_burn_ticks: u32) {
        let mut fixture = app.world_mut().resource_mut::<SurvivalFurnaceFixture>();
        fixture.cook_progress_ticks = cook_progress_ticks;
        fixture.remaining_burn_ticks = remaining_burn_ticks;
    }

    fn furnace_fixture(app: &App) -> &SurvivalFurnaceFixture {
        app.world().resource::<SurvivalFurnaceFixture>()
    }

    fn insert_pending_mob_drop_fixture(app: &mut App) -> Entity {
        let mob = app.world_mut().spawn_empty().id();
        let collector = app.world_mut().spawn_empty().id();
        let drop = app
            .world_mut()
            .spawn(SurvivalMobDropItem::new(TEST_MOB_DROP_ID, collector))
            .id();
        app.insert_resource(SurvivalMobDropFixture::new(mob, TEST_MOB_DROP_ID));
        drop
    }

    fn survival_mob_drop_candidate(
        entity: Entity,
        drop: &SurvivalMobDropItem,
    ) -> SurvivalMobDropCandidate {
        drop.candidate(entity)
    }

    #[test]
    fn survival_gameplay_plugin_installs_contract_and_schedules() {
        let mut app = app_with_survival_event_loop_schedule();

        app.add_plugins(SurvivalCompatibilityPlugin);

        let contract = app
            .world()
            .resource::<SurvivalCompatibilityPluginContract>();
        assert_eq!(contract.update_phase_order, SURVIVAL_GAMEPLAY_PHASE_ORDER);
        assert_eq!(
            contract.event_loop_phase_order,
            SURVIVAL_GAMEPLAY_PHASE_ORDER
        );
        assert_eq!(contract.scope, SURVIVAL_PRIMARY_SCOPE);
        assert!(app.world().contains_resource::<SurvivalRuntimeConfig>());
        assert!(app_has_schedule(&app, UPDATE_SCHEDULE_LABEL));
        assert!(app_has_schedule(&app, SURVIVAL_EVENT_LOOP_SCHEDULE_LABEL));

        let shared_contract = gameplay_contracts::assert_gameplay_contract_present(
            &app,
            SURVIVAL_COMPATIBILITY_PLUGIN_NAME,
        );
        assert_eq!(
            shared_contract.install_mode,
            GameplayInstallMode::ExplicitOptIn
        );
        assert_eq!(shared_contract.scope, Some(SURVIVAL_PRIMARY_SCOPE));
        gameplay_contracts::assert_schedule_phases(
            shared_contract,
            UPDATE_SCHEDULE_LABEL,
            SURVIVAL_GAMEPLAY_PHASE_ORDER,
        );
        gameplay_contracts::assert_schedule_phases(
            shared_contract,
            SURVIVAL_EVENT_LOOP_SCHEDULE_LABEL,
            SURVIVAL_GAMEPLAY_PHASE_ORDER,
        );
        assert!(shared_contract
            .non_claims
            .contains(&"BedWars or Hyperion scope"));
    }

    #[test]
    fn disabled_survival_gameplay_plugin_installs_no_contract() {
        let app = app_with_survival_event_loop_schedule();

        assert!(!app
            .world()
            .contains_resource::<SurvivalCompatibilityPluginContract>());
        assert!(!app.world().contains_resource::<SurvivalRuntimeConfig>());
        gameplay_contracts::assert_gameplay_contract_absent(
            &app,
            SURVIVAL_COMPATIBILITY_PLUGIN_NAME,
        );
        gameplay_contracts::assert_gameplay_contract_absent(
            &app,
            SURVIVAL_RUNTIME_CONFIG_SOURCE_PLUGIN_NAME,
        );
    }

    #[test]
    fn furnace_smelting_plugin_installs_explicit_contract_resources_events_and_schedule() {
        let (app, _inventory) = app_with_furnace_shell_fixture();

        let shared_contract = gameplay_contracts::assert_gameplay_contract_present(
            &app,
            SURVIVAL_FURNACE_SMELTING_PLUGIN_NAME,
        );
        assert_eq!(
            shared_contract.install_mode,
            GameplayInstallMode::ExplicitOptIn
        );
        assert_eq!(shared_contract.scope, Some(SURVIVAL_PRIMARY_SCOPE));
        gameplay_contracts::assert_schedule_phases(
            shared_contract,
            UPDATE_SCHEDULE_LABEL,
            SURVIVAL_GAMEPLAY_PHASE_ORDER,
        );
        assert!(shared_contract
            .owned_resources
            .contains(&SURVIVAL_FURNACE_RECIPE_TABLE_RESOURCE_NAME));
        assert!(shared_contract
            .owned_events
            .contains(&SURVIVAL_FURNACE_STATE_CHANGED_EVENT_NAME));
        assert!(shared_contract
            .non_claims
            .contains(&"DefaultPlugins membership change"));
        assert!(app
            .world()
            .contains_resource::<SurvivalFurnaceRecipeTableResource>());
        assert!(app
            .world()
            .contains_resource::<Events<SurvivalFurnaceStateChangedEvent>>());
    }

    #[test]
    fn disabled_furnace_smelting_plugin_installs_no_shell_and_does_not_mutate() {
        let (mut app, inventory) = app_with_disabled_furnace_shell_fixture();
        set_furnace_slots(
            &mut app,
            inventory,
            survival_furnace_input_stack(),
            survival_furnace_fuel_stack(),
            ItemStack::EMPTY,
        );

        app.update();

        assert_eq!(
            furnace_slot(&app, inventory, SURVIVAL_FURNACE_INPUT_SLOT),
            survival_furnace_input_stack()
        );
        assert_eq!(
            furnace_slot(&app, inventory, SURVIVAL_FURNACE_FUEL_SLOT),
            survival_furnace_fuel_stack()
        );
        assert_eq!(
            furnace_slot(&app, inventory, SURVIVAL_FURNACE_OUTPUT_SLOT),
            ItemStack::EMPTY
        );
        assert_eq!(furnace_fixture(&app).last_shell_transition, None);
        assert_eq!(furnace_fixture(&app).last_shell_diagnostic, None);
        assert!(!app
            .world()
            .contains_resource::<SurvivalFurnaceRecipeTableResource>());
        assert!(!app
            .world()
            .contains_resource::<Events<SurvivalFurnaceStateChangedEvent>>());
        gameplay_contracts::assert_gameplay_contract_absent(
            &app,
            SURVIVAL_FURNACE_SMELTING_PLUGIN_NAME,
        );
    }

    #[test]
    fn survival_config_source_adapter_owns_env_reload_boundary() {
        let mut app = app_with_survival_event_loop_schedule();

        app.add_plugins(SurvivalRuntimeConfigSourcePlugin);

        let shared_contract = gameplay_contracts::assert_gameplay_contract_present(
            &app,
            SURVIVAL_RUNTIME_CONFIG_SOURCE_PLUGIN_NAME,
        );
        assert_eq!(
            shared_contract.install_mode,
            GameplayInstallMode::SourceAdapter
        );
        assert_eq!(shared_contract.scope, None);
        assert!(shared_contract
            .owned_events
            .contains(&SURVIVAL_RUNTIME_CONFIG_RELOAD_EVENT_NAME));
        assert!(app
            .world()
            .contains_resource::<Events<SurvivalRuntimeConfigReloadEvent>>());

        app.world_mut()
            .resource_mut::<Events<SurvivalRuntimeConfigReloadEvent>>()
            .send(SurvivalRuntimeConfigReloadEvent::from_inputs(
                SurvivalRuntimeConfigInputs {
                    chest_fixture: Some(SURVIVAL_ENV_FLAG_ENABLED_VALUE.to_owned()),
                    world_persistence_dir: Some("/tmp/survival-source".to_owned()),
                    ..Default::default()
                },
            ));
        app.update();

        let runtime_config = app.world().resource::<SurvivalRuntimeConfig>();
        assert!(runtime_config.fixtures.chest);
        assert_eq!(
            runtime_config.paths.world_persistence_marker,
            PathBuf::from("/tmp/survival-source").join(SURVIVAL_WORLD_PERSISTENCE_MARKER_FILE)
        );
    }

    #[test]
    fn survival_runtime_config_parser_preserves_fixture_contracts() {
        let temp_dir = PathBuf::from("/tmp/mc-compat-test-runtime");
        let config = parse_survival_runtime_config(&SurvivalRuntimeConfigInputs {
            chest_fixture: Some(SURVIVAL_ENV_FLAG_ENABLED_VALUE.to_owned()),
            crafting_fixture: Some("true".to_owned()),
            furnace_smelting_breadth_fixture: Some(SURVIVAL_ENV_FLAG_ENABLED_VALUE.to_owned()),
            hunger_food_fixture: Some(SURVIVAL_ENV_FLAG_ENABLED_VALUE.to_owned()),
            world_persistence_dir: Some("/tmp/world-fixture".to_owned()),
            block_entity_phase: Some(SURVIVAL_BLOCK_ENTITY_POST_RESTART_PHASE.to_owned()),
            temp_dir,
            ..Default::default()
        });

        assert!(config.fixtures.chest);
        assert!(!config.fixtures.crafting);
        assert!(config.fixtures.furnace);
        assert!(config.fixtures.furnace_smelting_breadth);
        assert!(config.fixtures.hunger_food);
        assert!(!config.fixtures.hunger_health);
        assert!(config.fixtures.block_entity_post_restart);
        assert_eq!(
            config.paths.world_persistence_marker,
            PathBuf::from("/tmp/world-fixture").join(SURVIVAL_WORLD_PERSISTENCE_MARKER_FILE)
        );
    }

    #[test]
    fn survival_runtime_config_diagnostics_cover_conflicts_and_stale_phases() {
        let config = parse_survival_runtime_config(&SurvivalRuntimeConfigInputs {
            hunger_food_fixture: Some(SURVIVAL_ENV_FLAG_ENABLED_VALUE.to_owned()),
            hunger_health_fixture: Some(SURVIVAL_ENV_FLAG_ENABLED_VALUE.to_owned()),
            block_entity_phase: Some(SURVIVAL_BLOCK_ENTITY_POST_RESTART_PHASE.to_owned()),
            world_multichunk_phase: Some(SURVIVAL_BLOCK_ENTITY_POST_RESTART_PHASE.to_owned()),
            ..Default::default()
        });

        assert_eq!(
            survival_runtime_config_issues(&config),
            vec![
                SurvivalRuntimeConfigIssue::ConflictingHungerFixtures,
                SurvivalRuntimeConfigIssue::StaleBlockEntityPhase,
                SurvivalRuntimeConfigIssue::StaleWorldMultichunkPhase,
            ]
        );
        assert_eq!(
            survival_hunger_profile(&config).map(|profile| profile.event_prefix),
            Some(SURVIVAL_HUNGER_HEALTH_EVENT_PREFIX)
        );
    }

    #[test]
    fn survival_gameplay_phase_order_rejects_regression() {
        assert_eq!(
            SURVIVAL_GAMEPLAY_PHASE_ORDER,
            &[
                SurvivalGameplayPhase::Input,
                SurvivalGameplayPhase::RuleEvaluation,
                SurvivalGameplayPhase::WorldMutation,
                SurvivalGameplayPhase::Presentation,
                SurvivalGameplayPhase::Cleanup,
            ]
        );
    }

    #[test]
    fn survival_scope_checks_accept_primary_scope_and_reject_invalid_scope() {
        const TEST_STALE_ARENA_ID: &str = "survival-stale";
        let wrong_mode = GameplayScope::new(
            GameplayMode::Ctf,
            GameplayArenaId::new(SURVIVAL_PRIMARY_ARENA_ID),
        );
        let stale_arena = GameplayScope::new(
            GameplayMode::Survival,
            GameplayArenaId::new(TEST_STALE_ARENA_ID),
        );

        assert_eq!(
            gameplay_contracts::gameplay_scope_check(
                Some(&SURVIVAL_PRIMARY_SCOPE),
                SURVIVAL_PRIMARY_SCOPE,
            ),
            gameplay_contracts::GameplayScopeCheck::Match
        );
        assert_eq!(
            gameplay_contracts::gameplay_scope_check(None, SURVIVAL_PRIMARY_SCOPE),
            gameplay_contracts::GameplayScopeCheck::Missing
        );
        assert_eq!(
            gameplay_contracts::gameplay_scope_check(Some(&wrong_mode), SURVIVAL_PRIMARY_SCOPE),
            gameplay_contracts::GameplayScopeCheck::ModeMismatch
        );
        assert_eq!(
            gameplay_contracts::gameplay_scope_check(Some(&stale_arena), SURVIVAL_PRIMARY_SCOPE),
            gameplay_contracts::GameplayScopeCheck::ArenaMismatch
        );
    }

    #[test]
    fn survival_break_accepts_only_target_stop_destroy() {
        assert!(should_break_survival_block(
            GameMode::Survival,
            DiggingState::Stop,
            survival_break_pos()
        ));
        assert!(!should_break_survival_block(
            GameMode::Creative,
            DiggingState::Stop,
            survival_break_pos()
        ));
        assert!(!should_break_survival_block(
            GameMode::Survival,
            DiggingState::Start,
            survival_break_pos()
        ));
        assert!(!should_break_survival_block(
            GameMode::Survival,
            DiggingState::Stop,
            BlockPos::new(SURVIVAL_TARGET_X, SPAWN_Y, SURVIVAL_TARGET_Z)
        ));
    }

    #[test]
    fn survival_place_accepts_only_main_hand_up_on_target() {
        assert!(should_place_survival_block(
            GameMode::Survival,
            Hand::Main,
            survival_break_pos(),
            Direction::Up
        ));
        assert!(!should_place_survival_block(
            GameMode::Creative,
            Hand::Main,
            survival_break_pos(),
            Direction::Up
        ));
        assert!(!should_place_survival_block(
            GameMode::Survival,
            Hand::Off,
            survival_break_pos(),
            Direction::Up
        ));
        assert!(!should_place_survival_block(
            GameMode::Survival,
            Hand::Main,
            survival_break_pos(),
            Direction::North
        ));
    }

    #[test]
    fn survival_chest_opens_only_main_hand_survival_target() {
        assert!(should_open_survival_chest(
            GameMode::Survival,
            Hand::Main,
            survival_chest_pos()
        ));
        assert!(!should_open_survival_chest(
            GameMode::Creative,
            Hand::Main,
            survival_chest_pos()
        ));
        assert!(!should_open_survival_chest(
            GameMode::Survival,
            Hand::Off,
            survival_chest_pos()
        ));
        assert!(!should_open_survival_chest(
            GameMode::Survival,
            Hand::Main,
            survival_break_pos()
        ));
    }

    #[test]
    fn survival_chest_store_event_requires_exact_slot_window_item() {
        let expected_change = SlotChange {
            idx: SURVIVAL_CHEST_SLOT_ID,
            stack: survival_chest_item_stack(),
        };
        assert!(is_survival_chest_store_event(
            SURVIVAL_CHEST_WINDOW,
            SURVIVAL_CHEST_SLOT_ID,
            std::slice::from_ref(&expected_change)
        ));
        assert!(!is_survival_chest_store_event(
            SURVIVAL_CHEST_WINDOW + 1,
            SURVIVAL_CHEST_SLOT_ID,
            std::slice::from_ref(&expected_change)
        ));
        assert!(!is_survival_chest_store_event(
            SURVIVAL_CHEST_WINDOW,
            SURVIVAL_CHEST_SLOT_ID + 1,
            std::slice::from_ref(&expected_change)
        ));
        assert!(!is_survival_chest_store_event(
            SURVIVAL_CHEST_WINDOW,
            SURVIVAL_CHEST_SLOT_ID,
            &[SlotChange {
                idx: SURVIVAL_CHEST_SLOT_ID,
                stack: ItemStack::new(
                    BlockState::STONE.to_kind().to_item_kind(),
                    SURVIVAL_CHEST_ITEM_COUNT,
                    None
                ),
            }]
        ));
    }

    #[test]
    fn survival_chest_store_enabled_click_sets_store_logged() {
        let mut app = app_with_chest_store_system();
        let client = spawn_chest_store_client(&mut app);
        insert_chest_fixture(&mut app);

        send_chest_store_click(&mut app, client);
        app.update();

        let fixture = app.world().resource::<SurvivalChestFixture>();
        assert!(fixture.store_logged);
    }

    #[test]
    fn survival_chest_store_drains_disabled_events_before_runtime_enable() {
        let mut app = app_with_chest_store_system();
        let client = spawn_chest_store_client(&mut app);

        send_chest_store_click(&mut app, client);
        app.update();
        insert_chest_fixture(&mut app);
        app.update();

        let fixture = app.world().resource::<SurvivalChestFixture>();
        assert!(!fixture.store_logged);

        send_chest_store_click(&mut app, client);
        app.update();

        let fixture = app.world().resource::<SurvivalChestFixture>();
        assert!(fixture.store_logged);
    }

    #[test]
    fn survival_mob_drop_pickup_run_condition_follows_fixture_resource() {
        assert!(!survival_mob_drop_pickup_resource_present(false));
        assert!(survival_mob_drop_pickup_resource_present(true));

        let mut app = app_with_mob_drop_pickup_system();
        app.update();
        assert!(!app.world().contains_resource::<SurvivalMobDropFixture>());

        let drop = insert_pending_mob_drop_fixture(&mut app);
        app.update();
        let drop_state = app
            .world()
            .get::<SurvivalMobDropItem>(drop)
            .expect("pending mob drop component remains on drop entity");
        assert_eq!(drop_state.ticks_since_drop, 1);

        app.world_mut().remove_resource::<SurvivalMobDropFixture>();
        app.update();
        assert!(!app.world().contains_resource::<SurvivalMobDropFixture>());
    }

    #[test]
    fn survival_mob_drop_pickup_planner_advances_until_ready() {
        assert_eq!(
            plan_survival_mob_drop_pickup(SurvivalMobDropPickupInput {
                pickup_logged: false,
                ticks_since_drop: 0,
            }),
            SurvivalMobDropPickupDecision::Pending {
                ticks_since_drop: 1,
            }
        );
        assert_eq!(
            plan_survival_mob_drop_pickup(SurvivalMobDropPickupInput {
                pickup_logged: false,
                ticks_since_drop: SURVIVAL_MOB_DROP_PICKUP_DELAY_TICKS - 1,
            }),
            SurvivalMobDropPickupDecision::Ready {
                ticks_since_drop: SURVIVAL_MOB_DROP_PICKUP_DELAY_TICKS,
            }
        );
        assert_eq!(
            plan_survival_mob_drop_pickup(SurvivalMobDropPickupInput {
                pickup_logged: true,
                ticks_since_drop: SURVIVAL_MOB_DROP_PICKUP_DELAY_TICKS,
            }),
            SurvivalMobDropPickupDecision::AlreadyComplete
        );
    }

    #[test]
    fn survival_mob_drop_candidate_selection_fails_closed_for_missing_or_duplicate_drops() {
        let mut app = App::new();
        let collector = app.world_mut().spawn_empty().id();
        let first_drop = app
            .world_mut()
            .spawn(SurvivalMobDropItem::new(TEST_MOB_DROP_ID, collector))
            .id();
        let second_drop = app
            .world_mut()
            .spawn(SurvivalMobDropItem::new(
                TEST_DUPLICATE_MOB_DROP_ID,
                collector,
            ))
            .id();
        let first = app
            .world()
            .get::<SurvivalMobDropItem>(first_drop)
            .expect("first drop has fixture component");
        let second = app
            .world()
            .get::<SurvivalMobDropItem>(second_drop)
            .expect("second drop has fixture component");

        assert_eq!(
            select_survival_mob_drop_candidate(Vec::<SurvivalMobDropCandidate>::new()),
            SurvivalMobDropCandidateSelection::None
        );
        assert_eq!(
            select_survival_mob_drop_candidate([survival_mob_drop_candidate(first_drop, first)]),
            SurvivalMobDropCandidateSelection::Selected(survival_mob_drop_candidate(
                first_drop, first
            ))
        );
        assert_eq!(
            select_survival_mob_drop_candidate([
                survival_mob_drop_candidate(first_drop, first),
                survival_mob_drop_candidate(second_drop, second),
            ]),
            SurvivalMobDropCandidateSelection::Duplicate
        );
    }

    #[test]
    fn survival_mob_drop_duplicate_components_do_not_advance_pickup_state() {
        let mut app = app_with_mob_drop_pickup_system();
        let mob = app.world_mut().spawn_empty().id();
        let collector = app.world_mut().spawn_empty().id();
        let first_drop = app
            .world_mut()
            .spawn(SurvivalMobDropItem::new(TEST_MOB_DROP_ID, collector))
            .id();
        let second_drop = app
            .world_mut()
            .spawn(SurvivalMobDropItem::new(
                TEST_DUPLICATE_MOB_DROP_ID,
                collector,
            ))
            .id();
        app.insert_resource(SurvivalMobDropFixture::new(mob, TEST_MOB_DROP_ID));

        app.update();

        let first = app
            .world()
            .get::<SurvivalMobDropItem>(first_drop)
            .expect("first duplicate drop remains present");
        let second = app
            .world()
            .get::<SurvivalMobDropItem>(second_drop)
            .expect("second duplicate drop remains present");
        assert_eq!(first.ticks_since_drop, 0);
        assert_eq!(second.ticks_since_drop, 0);
    }

    #[test]
    fn survival_open_container_component_matches_active_fixture() {
        let chest = SurvivalOpenContainer::new(SurvivalContainerKind::Chest);

        assert!(survival_container_is_open(
            Some(&chest),
            SurvivalContainerKind::Chest
        ));
        assert!(!survival_container_is_open(
            Some(&chest),
            SurvivalContainerKind::Furnace
        ));
        assert!(!survival_container_is_open(
            None,
            SurvivalContainerKind::Chest
        ));
    }

    #[test]
    fn survival_open_container_duplicate_open_replaces_previous_fixture() {
        let mut app = App::new();
        let client = app
            .world_mut()
            .spawn(SurvivalOpenContainer::new(SurvivalContainerKind::Chest))
            .id();

        app.world_mut()
            .entity_mut(client)
            .insert(SurvivalOpenContainer::new(SurvivalContainerKind::Furnace));

        let open_container = app
            .world()
            .get::<SurvivalOpenContainer>(client)
            .expect("open container component remains on client");
        assert!(survival_container_is_open(
            Some(open_container),
            SurvivalContainerKind::Furnace
        ));
        assert!(!survival_container_is_open(
            Some(open_container),
            SurvivalContainerKind::Chest
        ));
    }

    #[test]
    fn survival_open_container_cleanup_removes_despawned_client_state() {
        let mut app = App::new();
        let client = app
            .world_mut()
            .spawn((
                SurvivalOpenContainer::new(SurvivalContainerKind::Crafting),
                Despawned,
                SURVIVAL_PRIMARY_SCOPE,
            ))
            .id();
        let wrong_scope_client = app
            .world_mut()
            .spawn((
                SurvivalOpenContainer::new(SurvivalContainerKind::Crafting),
                Despawned,
                GameplayScope::new(
                    GameplayMode::Ctf,
                    GameplayArenaId::new(SURVIVAL_PRIMARY_ARENA_ID),
                ),
            ))
            .id();
        let mut cleanup = Schedule::default();
        cleanup.add_systems(remove_survival_open_containers_from_despawned);

        cleanup.run(app.world_mut());

        assert!(app.world().get::<SurvivalOpenContainer>(client).is_none());
        assert!(app
            .world()
            .get::<SurvivalOpenContainer>(wrong_scope_client)
            .is_some());
    }

    #[test]
    fn survival_open_container_reconnect_starts_without_stale_state() {
        let mut app = App::new();
        let stale_client = app
            .world_mut()
            .spawn((
                SurvivalOpenContainer::new(SurvivalContainerKind::Chest),
                Despawned,
                SURVIVAL_PRIMARY_SCOPE,
            ))
            .id();
        let reconnect_client = app.world_mut().spawn_empty().id();
        let mut cleanup = Schedule::default();
        cleanup.add_systems(remove_survival_open_containers_from_despawned);

        cleanup.run(app.world_mut());

        assert!(app
            .world()
            .get::<SurvivalOpenContainer>(stale_client)
            .is_none());
        assert!(app
            .world()
            .get::<SurvivalOpenContainer>(reconnect_client)
            .is_none());
    }

    #[test]
    fn survival_crafting_opens_only_main_hand_survival_target() {
        assert!(should_open_survival_crafting(
            GameMode::Survival,
            Hand::Main,
            survival_crafting_pos()
        ));
        assert!(!should_open_survival_crafting(
            GameMode::Creative,
            Hand::Main,
            survival_crafting_pos()
        ));
        assert!(!should_open_survival_crafting(
            GameMode::Survival,
            Hand::Off,
            survival_crafting_pos()
        ));
        assert!(!should_open_survival_crafting(
            GameMode::Survival,
            Hand::Main,
            survival_chest_pos()
        ));
    }

    #[test]
    fn survival_crafting_input_event_requires_exact_slot_window() {
        assert!(is_survival_crafting_input_event(
            SURVIVAL_CRAFTING_WINDOW,
            SURVIVAL_CRAFTING_INPUT_A_SLOT_ID,
            SURVIVAL_CRAFTING_INPUT_A_SLOT_ID,
        ));
        assert!(!is_survival_crafting_input_event(
            SURVIVAL_CRAFTING_WINDOW + 1,
            SURVIVAL_CRAFTING_INPUT_A_SLOT_ID,
            SURVIVAL_CRAFTING_INPUT_A_SLOT_ID,
        ));
        assert!(!is_survival_crafting_input_event(
            SURVIVAL_CRAFTING_WINDOW,
            SURVIVAL_CRAFTING_INPUT_B_SLOT_ID,
            SURVIVAL_CRAFTING_INPUT_A_SLOT_ID,
        ));
    }

    #[test]
    fn survival_furnace_opens_only_main_hand_survival_target() {
        assert!(should_open_survival_furnace(
            GameMode::Survival,
            Hand::Main,
            survival_furnace_pos()
        ));
        assert!(!should_open_survival_furnace(
            GameMode::Creative,
            Hand::Main,
            survival_furnace_pos()
        ));
        assert!(!should_open_survival_furnace(
            GameMode::Survival,
            Hand::Off,
            survival_furnace_pos()
        ));
        assert!(!should_open_survival_furnace(
            GameMode::Survival,
            Hand::Main,
            survival_chest_pos()
        ));
    }

    #[test]
    fn survival_redstone_toggle_accepts_only_main_hand_survival_control() {
        assert!(should_toggle_survival_redstone(
            GameMode::Survival,
            Hand::Main,
            survival_redstone_toggle_control_pos()
        ));
        assert!(!should_toggle_survival_redstone(
            GameMode::Creative,
            Hand::Main,
            survival_redstone_toggle_control_pos()
        ));
        assert!(!should_toggle_survival_redstone(
            GameMode::Survival,
            Hand::Off,
            survival_redstone_toggle_control_pos()
        ));
        assert!(!should_toggle_survival_redstone(
            GameMode::Survival,
            Hand::Main,
            survival_redstone_toggle_output_pos()
        ));
    }

    #[test]
    fn survival_redstone_toggle_states_use_powered_properties() {
        assert_eq!(
            survival_redstone_toggle_control_state(true).get(PropName::Powered),
            Some(PropValue::True)
        );
        assert_eq!(
            survival_redstone_toggle_control_state(false).get(PropName::Powered),
            Some(PropValue::False)
        );
        assert_eq!(
            survival_redstone_toggle_output_state(true).get(PropName::Lit),
            Some(PropValue::True)
        );
        assert_eq!(
            survival_redstone_toggle_output_state(false).get(PropName::Lit),
            Some(PropValue::False)
        );
    }

    #[test]
    fn survival_world_persistence_accepts_only_main_hand_up_on_base() {
        assert!(should_place_survival_world_persistence(
            GameMode::Survival,
            Hand::Main,
            survival_world_persistence_base_pos(),
            Direction::Up,
        ));
        assert!(!should_place_survival_world_persistence(
            GameMode::Creative,
            Hand::Main,
            survival_world_persistence_base_pos(),
            Direction::Up,
        ));
        assert!(!should_place_survival_world_persistence(
            GameMode::Survival,
            Hand::Off,
            survival_world_persistence_base_pos(),
            Direction::Up,
        ));
        assert!(!should_place_survival_world_persistence(
            GameMode::Survival,
            Hand::Main,
            survival_world_persistence_pos(),
            Direction::Up,
        ));
        assert!(!should_place_survival_world_persistence(
            GameMode::Survival,
            Hand::Main,
            survival_world_persistence_base_pos(),
            Direction::North,
        ));
    }

    #[test]
    fn survival_world_persistence_stack_requires_dirt_count() {
        let expected = survival_world_persistence_stack();
        let wrong_item = ItemStack::new(
            BlockState::STONE.to_kind().to_item_kind(),
            SURVIVAL_WORLD_PERSISTENCE_ITEM_COUNT,
            None,
        );
        let wrong_count = ItemStack::EMPTY;

        assert!(is_survival_world_persistence_stack(&expected));
        assert!(!is_survival_world_persistence_stack(&wrong_item));
        assert!(!is_survival_world_persistence_stack(&wrong_count));
    }

    #[test]
    fn survival_block_entity_fixture_places_initial_or_persisted_sign_only() {
        let initial_config = parse_survival_runtime_config(&SurvivalRuntimeConfigInputs::default());
        let restart_config = parse_survival_runtime_config(&SurvivalRuntimeConfigInputs {
            block_entity_phase: Some(SURVIVAL_BLOCK_ENTITY_POST_RESTART_PHASE.to_owned()),
            ..Default::default()
        });

        assert!(survival_block_entity_should_place_sign(
            &initial_config,
            false
        ));
        assert!(survival_block_entity_should_place_sign(
            &restart_config,
            true
        ));
        assert!(!survival_block_entity_should_place_sign(
            &restart_config,
            false
        ));
    }

    #[test]
    fn survival_block_entity_payload_uses_contract_lines() {
        let block = survival_block_entity_block();
        assert_eq!(block.state.to_kind(), BlockKind::OakSign);
        let nbt = block.nbt.expect("sign block has NBT");
        let Some(Value::Compound(front_text)) = nbt.get("front_text") else {
            panic!("front_text compound missing");
        };
        let Some(Value::List(messages)) = front_text.get("messages") else {
            panic!("messages list missing");
        };
        assert_eq!(messages.len(), SURVIVAL_BLOCK_ENTITY_TEXT_LINE_COUNT);
        assert_eq!(SURVIVAL_BLOCK_ENTITY_TEXT_PAYLOAD, "MC|Compat|Sign|Persist");
    }

    #[test]
    fn survival_furnace_input_and_fuel_events_require_exact_slot_window() {
        assert!(is_survival_furnace_input_event(
            SURVIVAL_FURNACE_WINDOW,
            SURVIVAL_FURNACE_INPUT_SLOT_ID,
        ));
        assert!(is_survival_furnace_fuel_event(
            SURVIVAL_FURNACE_WINDOW,
            SURVIVAL_FURNACE_FUEL_SLOT_ID,
        ));
        assert!(!is_survival_furnace_input_event(
            SURVIVAL_FURNACE_WINDOW + 1,
            SURVIVAL_FURNACE_INPUT_SLOT_ID,
        ));
        assert!(!is_survival_furnace_fuel_event(
            SURVIVAL_FURNACE_WINDOW,
            SURVIVAL_FURNACE_INPUT_SLOT_ID,
        ));
    }

    #[test]
    fn survival_furnace_collect_event_requires_output_slot_and_stack() {
        assert!(is_survival_furnace_collect_event(
            SURVIVAL_FURNACE_WINDOW,
            SURVIVAL_FURNACE_OUTPUT_SLOT_ID,
            &survival_furnace_output_stack()
        ));
        assert!(!is_survival_furnace_collect_event(
            SURVIVAL_FURNACE_WINDOW + 1,
            SURVIVAL_FURNACE_OUTPUT_SLOT_ID,
            &survival_furnace_output_stack()
        ));
        assert!(!is_survival_furnace_collect_event(
            SURVIVAL_FURNACE_WINDOW,
            SURVIVAL_FURNACE_INPUT_SLOT_ID,
            &survival_furnace_output_stack()
        ));
        assert!(!is_survival_furnace_collect_event(
            SURVIVAL_FURNACE_WINDOW,
            SURVIVAL_FURNACE_OUTPUT_SLOT_ID,
            &survival_furnace_input_stack()
        ));
    }

    #[test]
    fn survival_furnace_invalid_fuel_rejection_requires_breadth_collect_and_fuel_slot() {
        assert!(should_emit_survival_furnace_breadth_rejection(true, true));
        assert!(!should_emit_survival_furnace_breadth_rejection(false, true));
        assert!(!should_emit_survival_furnace_breadth_rejection(true, false));
        assert!(should_reject_survival_furnace_invalid_fuel(
            true,
            true,
            SURVIVAL_FURNACE_WINDOW,
            SURVIVAL_FURNACE_FUEL_SLOT_ID,
        ));
        assert!(!should_reject_survival_furnace_invalid_fuel(
            false,
            true,
            SURVIVAL_FURNACE_WINDOW,
            SURVIVAL_FURNACE_FUEL_SLOT_ID,
        ));
        assert!(!should_reject_survival_furnace_invalid_fuel(
            true,
            false,
            SURVIVAL_FURNACE_WINDOW,
            SURVIVAL_FURNACE_FUEL_SLOT_ID,
        ));
        assert!(!should_reject_survival_furnace_invalid_fuel(
            true,
            true,
            SURVIVAL_FURNACE_WINDOW,
            SURVIVAL_FURNACE_INPUT_SLOT_ID,
        ));
        assert!(!should_reject_survival_furnace_invalid_fuel(
            true,
            true,
            SURVIVAL_FURNACE_WINDOW + 1,
            SURVIVAL_FURNACE_FUEL_SLOT_ID,
        ));
    }

    #[test]
    fn survival_furnace_shell_starts_fuel_and_advances_active_burn() {
        let (mut start_app, start_inventory) = app_with_furnace_shell_fixture();
        set_furnace_slots(
            &mut start_app,
            start_inventory,
            survival_furnace_input_stack(),
            survival_furnace_fuel_stack(),
            ItemStack::EMPTY,
        );

        start_app.update();

        assert_eq!(
            furnace_slot(&start_app, start_inventory, SURVIVAL_FURNACE_FUEL_SLOT),
            ItemStack::EMPTY
        );
        assert_eq!(
            furnace_fixture(&start_app).remaining_burn_ticks,
            FURNACE_EXPECTED_COAL_AFTER_START
        );
        assert_eq!(
            furnace_fixture(&start_app).cook_progress_ticks,
            FURNACE_EXPECTED_PROGRESS_AFTER_TICK
        );
        assert_eq!(
            furnace_fixture(&start_app).last_shell_transition,
            Some(survival_core::FurnaceTransition::StartedFuel)
        );

        let (mut active_app, active_inventory) = app_with_furnace_shell_fixture();
        set_furnace_slots(
            &mut active_app,
            active_inventory,
            survival_furnace_input_stack(),
            survival_furnace_fuel_stack(),
            ItemStack::EMPTY,
        );
        set_furnace_progress(
            &mut active_app,
            survival_core::INITIAL_COOK_PROGRESS_TICKS,
            survival_core::SELECTED_COAL_BURN_TICKS,
        );

        active_app.update();

        assert_eq!(
            furnace_slot(&active_app, active_inventory, SURVIVAL_FURNACE_FUEL_SLOT),
            survival_furnace_fuel_stack()
        );
        assert_eq!(
            furnace_fixture(&active_app).last_shell_transition,
            Some(survival_core::FurnaceTransition::AdvancedCooking)
        );
    }

    #[test]
    fn survival_furnace_shell_produces_output_and_merges_compatible_stack() {
        let (mut output_app, output_inventory) = app_with_furnace_shell_fixture();
        set_furnace_slots(
            &mut output_app,
            output_inventory,
            survival_furnace_input_stack_with_count(FURNACE_TWO_INPUT_ITEMS),
            ItemStack::EMPTY,
            ItemStack::EMPTY,
        );
        set_furnace_progress(
            &mut output_app,
            FURNACE_NEAR_COMPLETE_PROGRESS,
            survival_core::FURNACE_TICK_INCREMENT,
        );

        output_app.update();

        assert_eq!(
            furnace_slot(&output_app, output_inventory, SURVIVAL_FURNACE_INPUT_SLOT),
            survival_furnace_input_stack()
        );
        assert_eq!(
            furnace_slot(&output_app, output_inventory, SURVIVAL_FURNACE_OUTPUT_SLOT),
            survival_furnace_output_stack()
        );
        assert_eq!(
            furnace_fixture(&output_app).recipes_completed,
            FURNACE_EXPECTED_ONE_COMPLETED_RECIPE
        );
        assert_eq!(
            furnace_fixture(&output_app).last_shell_transition,
            Some(survival_core::FurnaceTransition::ProducedOutput)
        );

        let (mut merge_app, merge_inventory) = app_with_furnace_shell_fixture();
        set_furnace_slots(
            &mut merge_app,
            merge_inventory,
            survival_furnace_input_stack(),
            ItemStack::EMPTY,
            survival_furnace_output_stack_with_count(FURNACE_COMPATIBLE_OUTPUT_START_COUNT),
        );
        set_furnace_progress(
            &mut merge_app,
            FURNACE_NEAR_COMPLETE_PROGRESS,
            survival_core::FURNACE_TICK_INCREMENT,
        );

        merge_app.update();

        assert_eq!(
            furnace_slot(&merge_app, merge_inventory, SURVIVAL_FURNACE_OUTPUT_SLOT),
            survival_furnace_output_stack_with_count(FURNACE_EXPECTED_MERGED_OUTPUT_COUNT)
        );
    }

    #[test]
    fn survival_furnace_shell_rejects_invalid_missing_blocked_and_stale_state() {
        let (mut invalid_app, invalid_inventory) = app_with_furnace_shell_fixture();
        let invalid_input = ItemStack::new(ItemKind::GoldIngot, SURVIVAL_FURNACE_ITEM_COUNT, None);
        set_furnace_slots(
            &mut invalid_app,
            invalid_inventory,
            invalid_input.clone(),
            survival_furnace_fuel_stack(),
            ItemStack::EMPTY,
        );

        invalid_app.update();

        assert_eq!(
            furnace_fixture(&invalid_app).last_shell_diagnostic,
            Some(SurvivalFurnaceShellDiagnostic::PausedNoRecipe)
        );
        assert_eq!(
            furnace_slot(&invalid_app, invalid_inventory, SURVIVAL_FURNACE_FUEL_SLOT),
            survival_furnace_fuel_stack()
        );

        let (mut no_fuel_app, no_fuel_inventory) = app_with_furnace_shell_fixture();
        set_furnace_slots(
            &mut no_fuel_app,
            no_fuel_inventory,
            survival_furnace_input_stack(),
            ItemStack::EMPTY,
            ItemStack::EMPTY,
        );

        no_fuel_app.update();

        assert_eq!(
            furnace_fixture(&no_fuel_app).last_shell_diagnostic,
            Some(SurvivalFurnaceShellDiagnostic::PausedNoFuel)
        );

        let (mut blocked_app, blocked_inventory) = app_with_furnace_shell_fixture();
        set_furnace_slots(
            &mut blocked_app,
            blocked_inventory,
            survival_furnace_input_stack(),
            survival_furnace_fuel_stack(),
            invalid_input,
        );

        blocked_app.update();

        assert_eq!(
            furnace_fixture(&blocked_app).last_shell_diagnostic,
            Some(SurvivalFurnaceShellDiagnostic::PausedOutputBlocked)
        );
        assert_eq!(
            furnace_slot(&blocked_app, blocked_inventory, SURVIVAL_FURNACE_INPUT_SLOT),
            survival_furnace_input_stack()
        );

        let (mut stale_app, stale_inventory) = app_with_furnace_shell_fixture();
        stale_app.insert_resource(SurvivalFurnaceBlockEntity {
            kind: survival_core::FurnaceKind::Standard,
            loaded: false,
        });
        set_furnace_slots(
            &mut stale_app,
            stale_inventory,
            survival_furnace_input_stack(),
            survival_furnace_fuel_stack(),
            ItemStack::EMPTY,
        );

        stale_app.update();

        assert_eq!(
            furnace_fixture(&stale_app).last_shell_diagnostic,
            Some(SurvivalFurnaceShellDiagnostic::StaleBlockEntity)
        );
        assert_eq!(
            furnace_slot(&stale_app, stale_inventory, SURVIVAL_FURNACE_FUEL_SLOT),
            survival_furnace_fuel_stack()
        );
    }

    #[test]
    fn survival_furnace_shell_rejects_unsupported_kind_and_malformed_data() {
        let (mut unsupported_app, unsupported_inventory) = app_with_furnace_shell_fixture();
        unsupported_app.insert_resource(SurvivalFurnaceBlockEntity {
            kind: survival_core::FurnaceKind::Smoker,
            loaded: true,
        });
        set_furnace_slots(
            &mut unsupported_app,
            unsupported_inventory,
            survival_furnace_input_stack(),
            survival_furnace_fuel_stack(),
            ItemStack::EMPTY,
        );

        unsupported_app.update();

        assert_eq!(
            furnace_fixture(&unsupported_app).last_shell_diagnostic,
            Some(SurvivalFurnaceShellDiagnostic::UnsupportedFurnaceKind)
        );

        let (mut malformed_app, malformed_inventory) = app_with_furnace_shell_fixture();
        malformed_app.insert_resource(SurvivalFurnaceRecipeTableResource {
            rows: [survival_core::FurnaceRecipeRow {
                input: survival_core::SELECTED_RAW_IRON_ITEM,
                output: survival_core::SELECTED_IRON_INGOT_ITEM,
                output_count: survival_core::EMPTY_ITEM_COUNT,
                cook_ticks: survival_core::SELECTED_STANDARD_FURNACE_COOK_TICKS,
            }],
        });
        set_furnace_slots(
            &mut malformed_app,
            malformed_inventory,
            survival_furnace_input_stack(),
            survival_furnace_fuel_stack(),
            ItemStack::EMPTY,
        );

        malformed_app.update();

        assert_eq!(
            furnace_fixture(&malformed_app).last_shell_diagnostic,
            Some(SurvivalFurnaceShellDiagnostic::MalformedRecipeRow)
        );
        assert_eq!(
            furnace_slot(
                &malformed_app,
                malformed_inventory,
                SURVIVAL_FURNACE_FUEL_SLOT
            ),
            survival_furnace_fuel_stack()
        );
    }

    #[test]
    fn survival_hunger_food_stack_requires_bread_count() {
        let bread = survival_hunger_food_stack();
        let wrong_item = ItemStack::new(
            survival_furnace_output_kind(),
            SURVIVAL_HUNGER_FOOD_ITEM_COUNT_BEFORE,
            None,
        );
        let wrong_count = ItemStack::new(
            survival_hunger_food_kind(),
            SURVIVAL_HUNGER_FOOD_ITEM_COUNT_BEFORE + 1,
            None,
        );

        assert!(is_survival_hunger_food_stack(&bread));
        assert!(!is_survival_hunger_food_stack(&wrong_item));
        assert!(!is_survival_hunger_food_stack(&wrong_count));
    }

    #[test]
    fn survival_hunger_profile_selects_enabled_contract() {
        assert_eq!(
            survival_hunger_profile_from_flags(true, false),
            Some(SURVIVAL_HUNGER_FOOD_PROFILE)
        );
        assert_eq!(
            survival_hunger_profile_from_flags(false, true),
            Some(SURVIVAL_HUNGER_HEALTH_PROFILE)
        );
    }

    #[test]
    fn survival_hunger_profile_rejects_disabled_contract() {
        assert_eq!(survival_hunger_profile_from_flags(false, false), None);
    }

    #[test]
    fn survival_mob_drop_stack_requires_iron_ingot_count() {
        let ingot = survival_mob_drop_stack();
        let wrong_item = ItemStack::new(
            survival_hunger_food_kind(),
            SURVIVAL_MOB_DROP_ITEM_COUNT,
            None,
        );
        let wrong_count = ItemStack::new(
            survival_mob_drop_item_kind(),
            SURVIVAL_MOB_DROP_ITEM_COUNT + 1,
            None,
        );

        assert!(is_survival_mob_drop_stack(&ingot));
        assert!(!is_survival_mob_drop_stack(&wrong_item));
        assert!(!is_survival_mob_drop_stack(&wrong_count));
    }

    #[test]
    fn survival_mob_drop_attack_requires_survival_attack_on_fixture_mob() {
        const TARGET_ENTITY: u32 = 11;
        const OTHER_ENTITY: u32 = 12;

        let target = Entity::from_raw(TARGET_ENTITY);
        let other = Entity::from_raw(OTHER_ENTITY);
        assert!(should_handle_survival_mob_drop_attack(
            GameMode::Survival,
            EntityInteraction::Attack,
            target,
            target,
        ));
        assert!(!should_handle_survival_mob_drop_attack(
            GameMode::Creative,
            EntityInteraction::Attack,
            target,
            target,
        ));
        assert!(!should_handle_survival_mob_drop_attack(
            GameMode::Survival,
            EntityInteraction::Interact(Hand::Main),
            target,
            target,
        ));
        assert!(!should_handle_survival_mob_drop_attack(
            GameMode::Survival,
            EntityInteraction::Attack,
            other,
            target,
        ));
    }

    #[test]
    fn survival_hunger_food_use_requires_main_hand_sequence_slot_and_pre_state() {
        assert!(should_consume_survival_hunger_food(
            SURVIVAL_HUNGER_FOOD_PROFILE,
            Hand::Main,
            SURVIVAL_HUNGER_FOOD_USE_SEQUENCE,
            SURVIVAL_HUNGER_FOOD_INVENTORY_SLOT,
            &survival_hunger_food_stack(),
            SURVIVAL_HUNGER_FOOD_PRE_HEALTH,
            SURVIVAL_HUNGER_FOOD_PRE_FOOD,
            SURVIVAL_HUNGER_FOOD_PRE_SATURATION,
        ));
        assert!(should_consume_survival_hunger_food(
            SURVIVAL_HUNGER_HEALTH_PROFILE,
            Hand::Main,
            SURVIVAL_HUNGER_FOOD_USE_SEQUENCE,
            SURVIVAL_HUNGER_FOOD_INVENTORY_SLOT,
            &survival_hunger_food_stack(),
            SURVIVAL_HUNGER_HEALTH_PRE_HEALTH,
            SURVIVAL_HUNGER_FOOD_PRE_FOOD,
            SURVIVAL_HUNGER_FOOD_PRE_SATURATION,
        ));
        assert!(!should_consume_survival_hunger_food(
            SURVIVAL_HUNGER_FOOD_PROFILE,
            Hand::Off,
            SURVIVAL_HUNGER_FOOD_USE_SEQUENCE,
            SURVIVAL_HUNGER_FOOD_INVENTORY_SLOT,
            &survival_hunger_food_stack(),
            SURVIVAL_HUNGER_FOOD_PRE_HEALTH,
            SURVIVAL_HUNGER_FOOD_PRE_FOOD,
            SURVIVAL_HUNGER_FOOD_PRE_SATURATION,
        ));
        assert!(!should_consume_survival_hunger_food(
            SURVIVAL_HUNGER_FOOD_PROFILE,
            Hand::Main,
            SURVIVAL_HUNGER_FOOD_USE_SEQUENCE + 1,
            SURVIVAL_HUNGER_FOOD_INVENTORY_SLOT,
            &survival_hunger_food_stack(),
            SURVIVAL_HUNGER_FOOD_PRE_HEALTH,
            SURVIVAL_HUNGER_FOOD_PRE_FOOD,
            SURVIVAL_HUNGER_FOOD_PRE_SATURATION,
        ));
        assert!(!should_consume_survival_hunger_food(
            SURVIVAL_HUNGER_FOOD_PROFILE,
            Hand::Main,
            SURVIVAL_HUNGER_FOOD_USE_SEQUENCE,
            SURVIVAL_HUNGER_FOOD_INVENTORY_SLOT + 1,
            &survival_hunger_food_stack(),
            SURVIVAL_HUNGER_FOOD_PRE_HEALTH,
            SURVIVAL_HUNGER_FOOD_PRE_FOOD,
            SURVIVAL_HUNGER_FOOD_PRE_SATURATION,
        ));
        assert!(!should_consume_survival_hunger_food(
            SURVIVAL_HUNGER_FOOD_PROFILE,
            Hand::Main,
            SURVIVAL_HUNGER_FOOD_USE_SEQUENCE,
            SURVIVAL_HUNGER_FOOD_INVENTORY_SLOT,
            &survival_furnace_output_stack(),
            SURVIVAL_HUNGER_FOOD_PRE_HEALTH,
            SURVIVAL_HUNGER_FOOD_PRE_FOOD,
            SURVIVAL_HUNGER_FOOD_PRE_SATURATION,
        ));
        assert!(!should_consume_survival_hunger_food(
            SURVIVAL_HUNGER_FOOD_PROFILE,
            Hand::Main,
            SURVIVAL_HUNGER_FOOD_USE_SEQUENCE,
            SURVIVAL_HUNGER_FOOD_INVENTORY_SLOT,
            &survival_hunger_food_stack(),
            SURVIVAL_HUNGER_FOOD_POST_HEALTH,
            SURVIVAL_HUNGER_FOOD_POST_FOOD,
            SURVIVAL_HUNGER_FOOD_POST_SATURATION,
        ));
    }

    #[test]
    fn survival_biome_dimension_normalizes_known_environment_ids() {
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
    fn survival_biome_dimension_rejects_unknown_environment_ids() {
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
    fn survival_crafting_collect_event_requires_result_slot_and_stack() {
        assert!(is_survival_crafting_collect_event(
            SURVIVAL_CRAFTING_WINDOW,
            SURVIVAL_CRAFTING_RESULT_SLOT_ID,
            &survival_crafting_result_stack()
        ));
        assert!(!is_survival_crafting_collect_event(
            SURVIVAL_CRAFTING_WINDOW + 1,
            SURVIVAL_CRAFTING_RESULT_SLOT_ID,
            &survival_crafting_result_stack()
        ));
        assert!(!is_survival_crafting_collect_event(
            SURVIVAL_CRAFTING_WINDOW,
            SURVIVAL_CRAFTING_INPUT_A_SLOT_ID,
            &survival_crafting_result_stack()
        ));
        assert!(!is_survival_crafting_collect_event(
            SURVIVAL_CRAFTING_WINDOW,
            SURVIVAL_CRAFTING_RESULT_SLOT_ID,
            &survival_crafting_input_stack(SURVIVAL_CRAFTING_INPUT_COUNT)
        ));
    }
}
