#![allow(clippy::type_complexity)]

use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

use valence::entity::iron_golem::IronGolemEntityBundle;
use valence::entity::item::{ItemEntityBundle, Stack as ItemEntityStack};
use valence::entity::living::Health;
use valence::entity::player::{Food, Saturation};
use valence::entity::{EntityId, EntityManager};
use valence::event_loop::PacketEvent;
use valence::interact_block::InteractBlockEvent;
use valence::interact_entity::{EntityInteraction, InteractEntityEvent};
use valence::interact_item::InteractItemEvent;
use valence::inventory::{ClickSlotEvent, CursorItem, HeldItem, OpenInventory, SlotChange};
use valence::log::info;
use valence::nbt::{compound, List, Value};
use valence::prelude::*;
use valence::protocol::packets::play::{
    BlockUpdateS2c, CloseHandledScreenC2s, ItemPickupAnimationS2c,
};
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
const SURVIVAL_CHEST_FIXTURE_ENV: &str = "MC_COMPAT_SURVIVAL_CHEST_FIXTURE";
const SURVIVAL_CHEST_X: i32 = 8;
const SURVIVAL_CHEST_Y: i32 = FLOOR_Y;
const SURVIVAL_CHEST_Z: i32 = 0;
const SURVIVAL_CHEST_SLOT: u16 = 0;
const SURVIVAL_CHEST_SLOT_ID: i16 = 0;
const SURVIVAL_CHEST_WINDOW: u8 = 1;
const SURVIVAL_CHEST_ITEM_COUNT: i8 = 1;
const SURVIVAL_CHEST_TITLE: &str = "MC Compat Chest";
const SURVIVAL_CRAFTING_FIXTURE_ENV: &str = "MC_COMPAT_SURVIVAL_CRAFTING_FIXTURE";
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
const SURVIVAL_HUNGER_FOOD_USE_SEQUENCE: i32 = 810;
const SURVIVAL_MOB_DROP_FIXTURE_ENV: &str = "MC_COMPAT_SURVIVAL_MOB_DROP_FIXTURE";
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
const SURVIVAL_BIOME_DIMENSION_FIXTURE_ENV: &str = "MC_COMPAT_SURVIVAL_BIOME_DIMENSION_FIXTURE";
const SURVIVAL_OVERWORLD_ID: &str = "minecraft:overworld";
const SURVIVAL_NETHER_ID: &str = "minecraft:the_nether";
const SURVIVAL_END_ID: &str = "minecraft:the_end";
const SURVIVAL_UNKNOWN_ENVIRONMENT_ID: &str = "unknown";

#[derive(Resource)]
struct SurvivalChestFixture {
    inventory: Entity,
    open_clients: HashSet<Entity>,
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
            open_clients: HashSet::new(),
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
    open_clients: HashSet<Entity>,
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
            open_clients: HashSet::new(),
            open_logged: false,
            input_a_logged: false,
            input_b_logged: false,
            result_logged: false,
            collect_logged: false,
        }
    }
}

#[derive(Resource)]
struct SurvivalFurnaceFixture {
    inventory: Entity,
    open_clients: HashSet<Entity>,
    open_logged: bool,
    input_logged: bool,
    fuel_logged: bool,
    burn_logged: bool,
    output_logged: bool,
    collect_logged: bool,
    reopen_logged: bool,
    state_logged: bool,
}

#[derive(Resource, Default)]
struct SurvivalHungerFoodFixture {
    pre_logged: bool,
    consume_start_logged: bool,
    consume_finish_logged: bool,
    inventory_logged: bool,
    state_logged: bool,
}

#[derive(Resource)]
struct SurvivalMobDropFixture {
    mob: Entity,
    mob_id: i32,
    drop: Option<Entity>,
    drop_id: Option<i32>,
    collector: Option<Entity>,
    ticks_since_drop: u8,
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
            drop: None,
            drop_id: None,
            collector: None,
            ticks_since_drop: 0,
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
    fn new(inventory: Entity) -> Self {
        Self {
            inventory,
            open_clients: HashSet::new(),
            open_logged: false,
            input_logged: false,
            fuel_logged: false,
            burn_logged: false,
            output_logged: false,
            collect_logged: false,
            reopen_logged: false,
            state_logged: false,
        }
    }
}

pub fn main() {
    App::new()
        .insert_resource(NetworkSettings {
            connection_mode: ConnectionMode::Offline,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(EventLoopPreUpdate, handle_survival_chest_close)
        .add_systems(
            Update,
            (
                init_clients,
                despawn_disconnected_clients,
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
                advance_survival_mob_drop_pickup,
            ),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    server: Res<Server>,
    dimensions: Res<DimensionTypeRegistry>,
    biomes: Res<BiomeRegistry>,
    mut entity_manager: ResMut<EntityManager>,
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
    if survival_chest_fixture_enabled() {
        layer
            .chunk
            .set_block(survival_chest_pos(), BlockState::CHEST);
    }
    if survival_crafting_fixture_enabled() {
        layer
            .chunk
            .set_block(survival_crafting_pos(), survival_crafting_table_state());
    }
    if survival_furnace_fixture_enabled() {
        layer
            .chunk
            .set_block(survival_furnace_pos(), survival_furnace_state());
    }
    if survival_redstone_toggle_fixture_enabled() {
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
    let world_persistence_marker = survival_world_persistence_marker_path();
    let world_persistence_loaded = world_persistence_marker.exists();
    if survival_world_persistence_fixture_enabled() {
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
    let block_entity_marker = survival_block_entity_marker_path();
    let block_entity_loaded = block_entity_marker.exists();
    if survival_block_entity_fixture_enabled() {
        setup_survival_block_entity_arena(&mut layer);
        if survival_block_entity_should_place_sign(block_entity_loaded) {
            layer
                .chunk
                .set_block(survival_block_entity_pos(), survival_block_entity_block());
        }
    }

    let layer = commands.spawn(layer).id();

    if survival_chest_fixture_enabled() {
        let inventory = commands
            .spawn(Inventory::with_title(
                InventoryKind::Generic9x3,
                SURVIVAL_CHEST_TITLE,
            ))
            .id();
        commands.insert_resource(SurvivalChestFixture::new(inventory));
    }
    if survival_crafting_fixture_enabled() {
        let inventory = commands
            .spawn(Inventory::with_title(
                InventoryKind::Crafting,
                SURVIVAL_CRAFTING_TITLE,
            ))
            .id();
        commands.insert_resource(SurvivalCraftingFixture::new(inventory));
    }
    if survival_furnace_fixture_enabled() {
        let inventory = commands
            .spawn(Inventory::with_title(
                InventoryKind::Furnace,
                SURVIVAL_FURNACE_TITLE,
            ))
            .id();
        commands.insert_resource(SurvivalFurnaceFixture::new(inventory));
    }
    if survival_hunger_food_fixture_enabled() {
        commands.insert_resource(SurvivalHungerFoodFixture::default());
    }
    if survival_mob_drop_fixture_enabled() {
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
    if survival_redstone_toggle_fixture_enabled() {
        commands.insert_resource(SurvivalRedstoneToggleFixture::default());
    }
    if survival_world_persistence_fixture_enabled() {
        commands.insert_resource(SurvivalWorldPersistenceFixture::new(
            world_persistence_marker,
            world_persistence_loaded,
        ));
    }
    if survival_block_entity_fixture_enabled() {
        commands.insert_resource(SurvivalBlockEntityFixture::new(
            block_entity_marker,
            block_entity_loaded,
        ));
    }
}

fn init_clients(
    mut clients: Query<
        (
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
    layers: Query<Entity, (With<ChunkLayer>, With<EntityLayer>)>,
    mut hunger_food_fixture: Option<ResMut<SurvivalHungerFoodFixture>>,
    mut mob_drop_fixture: Option<ResMut<SurvivalMobDropFixture>>,
    mut world_persistence_fixture: Option<ResMut<SurvivalWorldPersistenceFixture>>,
    mut block_entity_fixture: Option<ResMut<SurvivalBlockEntityFixture>>,
) {
    for (
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
        if survival_redstone_toggle_fixture_enabled() {
            pos.set([
                SURVIVAL_REDSTONE_TOGGLE_PLAYER_X,
                SURVIVAL_REDSTONE_TOGGLE_PLAYER_Y,
                SURVIVAL_REDSTONE_TOGGLE_PLAYER_Z,
            ]);
        } else if survival_world_persistence_fixture_enabled() {
            pos.set([
                SURVIVAL_WORLD_PERSISTENCE_PLAYER_X,
                SURVIVAL_WORLD_PERSISTENCE_PLAYER_Y,
                SURVIVAL_WORLD_PERSISTENCE_PLAYER_Z,
            ]);
        } else if survival_block_entity_fixture_enabled() {
            pos.set([
                SURVIVAL_BLOCK_ENTITY_PLAYER_X,
                SURVIVAL_BLOCK_ENTITY_PLAYER_Y,
                SURVIVAL_BLOCK_ENTITY_PLAYER_Z,
            ]);
        } else {
            pos.set([SURVIVAL_SPAWN_X, f64::from(SPAWN_Y), SURVIVAL_SPAWN_Z]);
        }
        *game_mode = GameMode::Survival;
        inventory.set_slot(SURVIVAL_ITEM_SLOT, ItemStack::EMPTY);
        if survival_chest_fixture_enabled() {
            cursor_item.0 = survival_chest_item_stack();
        }
        if survival_crafting_fixture_enabled() {
            cursor_item.0 = survival_crafting_input_stack(SURVIVAL_CRAFTING_TOTAL_INPUT_COUNT);
        }
        if survival_furnace_fixture_enabled() {
            cursor_item.0 = survival_furnace_input_stack();
        }
        if survival_world_persistence_fixture_enabled() {
            inventory.set_slot(
                SURVIVAL_WORLD_PERSISTENCE_INVENTORY_SLOT,
                survival_world_persistence_stack(),
            );
        }
        if survival_hunger_food_fixture_enabled() {
            health.0 = SURVIVAL_HUNGER_FOOD_PRE_HEALTH;
            food.0 = SURVIVAL_HUNGER_FOOD_PRE_FOOD;
            saturation.0 = SURVIVAL_HUNGER_FOOD_PRE_SATURATION;
            inventory.set_slot(
                SURVIVAL_HUNGER_FOOD_INVENTORY_SLOT,
                survival_hunger_food_stack(),
            );
            if let Some(fixture) = hunger_food_fixture.as_mut() {
                log_survival_hunger_food_pre(username.as_str(), fixture);
            }
        }

        client.send_chat_message(SURVIVAL_WELCOME.italic());
        log_milestone(format!(
            "MC-COMPAT-MILESTONE survival_join username={} gamemode=Survival target={},{},{}",
            username.as_str(),
            SURVIVAL_TARGET_X,
            FLOOR_Y,
            SURVIVAL_TARGET_Z
        ));
        if survival_biome_dimension_fixture_enabled() {
            log_survival_biome_dimension_state(
                username.as_str(),
                SURVIVAL_OVERWORLD_ID,
                SURVIVAL_OVERWORLD_ID,
            );
        }
        if let Some(fixture) = mob_drop_fixture.as_mut() {
            log_survival_mob_drop_spawn(username.as_str(), fixture);
        }
        if let Some(fixture) = world_persistence_fixture.as_mut() {
            log_survival_world_persistence_post_restart(username.as_str(), &mut client, fixture);
        }
        if let Some(fixture) = block_entity_fixture.as_mut() {
            log_survival_block_entity_persistence(username.as_str(), fixture);
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
    fixture: Option<ResMut<SurvivalMobDropFixture>>,
    mut clients: Query<(&Username, &mut Client, &mut Inventory, &EntityId)>,
) {
    let Some(mut fixture) = fixture else {
        return;
    };
    if fixture.pickup_logged || fixture.drop_id.is_none() {
        return;
    }
    fixture.ticks_since_drop = fixture.ticks_since_drop.saturating_add(1);
    if fixture.ticks_since_drop < SURVIVAL_MOB_DROP_PICKUP_DELAY_TICKS {
        return;
    }

    let Some(collector) = fixture.collector else {
        return;
    };
    let Some(drop) = fixture.drop else {
        return;
    };
    let Some(drop_id) = fixture.drop_id else {
        return;
    };
    let Ok((username, mut client, mut inventory, collector_id)) = clients.get_mut(collector) else {
        return;
    };

    inventory.set_slot(SURVIVAL_MOB_DROP_INVENTORY_SLOT, survival_mob_drop_stack());
    client.write_packet(&ItemPickupAnimationS2c {
        collected_entity_id: VarInt(drop_id),
        collector_entity_id: VarInt(collector_id.get()),
        pickup_item_count: VarInt(i32::from(SURVIVAL_MOB_DROP_ITEM_COUNT)),
    });
    commands.entity(drop).insert(Despawned);
    log_survival_mob_drop_pickup_and_state(username.as_str(), &mut fixture, collector_id.get());
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

        commands
            .entity(event.client)
            .insert(OpenInventory::new(fixture.inventory));
        fixture.open_clients.insert(event.client);

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
    clients: Query<&Username>,
    mut events: EventReader<ClickSlotEvent>,
) {
    let Some(mut fixture) = fixture else {
        return;
    };
    if fixture.store_logged {
        return;
    }

    for event in events.read() {
        let Ok(username) = clients.get(event.client) else {
            continue;
        };
        if !fixture.open_clients.contains(&event.client)
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
    fixture: Option<ResMut<SurvivalChestFixture>>,
    clients: Query<&Username>,
    mut packets: EventReader<PacketEvent>,
) {
    let Some(mut fixture) = fixture else {
        return;
    };
    if !fixture.store_logged || fixture.close_logged {
        return;
    }

    for packet in packets.read() {
        if packet.decode::<CloseHandledScreenC2s>().is_none() {
            continue;
        }
        if !fixture.open_clients.remove(&packet.client) {
            continue;
        }
        let Ok(username) = clients.get(packet.client) else {
            continue;
        };
        fixture.close_logged = true;
        log_milestone(format!(
            "MC-COMPAT-MILESTONE survival_chest_close username={} window={}",
            username.as_str(),
            SURVIVAL_CHEST_WINDOW
        ));
        break;
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

        commands
            .entity(event.client)
            .insert(OpenInventory::new(fixture.inventory));
        fixture.open_clients.insert(event.client);

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
    clients: Query<&Username>,
    mut inventories: Query<&mut Inventory>,
    mut events: EventReader<ClickSlotEvent>,
) {
    let Some(mut fixture) = fixture else {
        return;
    };

    for event in events.read() {
        let Ok(username) = clients.get(event.client) else {
            continue;
        };
        if !fixture.open_clients.contains(&event.client)
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

        commands
            .entity(event.client)
            .insert(OpenInventory::new(fixture.inventory));
        fixture.open_clients.insert(event.client);

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
    clients: Query<&Username>,
    mut inventories: Query<&mut Inventory>,
    mut events: EventReader<ClickSlotEvent>,
) {
    let Some(mut fixture) = fixture else {
        return;
    };

    for event in events.read() {
        let Ok(username) = clients.get(event.client) else {
            continue;
        };
        if !fixture.open_clients.contains(&event.client)
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
        if !should_consume_survival_hunger_food(
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
            "MC-COMPAT-MILESTONE survival_hunger_food_consume_start username={} item={} slot={} \
             food_before={} saturation_before={:.1}",
            username,
            SURVIVAL_HUNGER_FOOD_ITEM_NAME,
            SURVIVAL_HUNGER_FOOD_INVENTORY_SLOT,
            SURVIVAL_HUNGER_FOOD_PRE_FOOD,
            SURVIVAL_HUNGER_FOOD_PRE_SATURATION
        ));
    }

    inventory.set_slot(SURVIVAL_HUNGER_FOOD_INVENTORY_SLOT, ItemStack::EMPTY);
    health.0 = SURVIVAL_HUNGER_FOOD_POST_HEALTH;
    food.0 = SURVIVAL_HUNGER_FOOD_POST_FOOD;
    saturation.0 = SURVIVAL_HUNGER_FOOD_POST_SATURATION;

    if !fixture.consume_finish_logged {
        fixture.consume_finish_logged = true;
        log_milestone(format!(
            "MC-COMPAT-MILESTONE survival_hunger_food_consume_finish username={} item={} slot={} \
             food_after={} saturation_after={:.1}",
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
            "MC-COMPAT-MILESTONE survival_hunger_food_inventory username={} slot={} item={} \
             count_before={} count_after={}",
            username,
            SURVIVAL_HUNGER_FOOD_INVENTORY_SLOT,
            SURVIVAL_HUNGER_FOOD_ITEM_NAME,
            SURVIVAL_HUNGER_FOOD_ITEM_COUNT_BEFORE,
            SURVIVAL_HUNGER_FOOD_ITEM_COUNT_AFTER
        ));
    }
    if !fixture.state_logged {
        fixture.state_logged = true;
        log_milestone(format!(
            "MC-COMPAT-MILESTONE survival_hunger_food_state username={} health={:.1} \
             food_before={} food_after={} saturation_before={:.1} saturation_after={:.1} \
             unexpected_damage=false death=false",
            username,
            SURVIVAL_HUNGER_FOOD_POST_HEALTH,
            SURVIVAL_HUNGER_FOOD_PRE_FOOD,
            SURVIVAL_HUNGER_FOOD_POST_FOOD,
            SURVIVAL_HUNGER_FOOD_PRE_SATURATION,
            SURVIVAL_HUNGER_FOOD_POST_SATURATION
        ));
    }
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
    game_mode == GameMode::Survival
        && state == DiggingState::Stop
        && position == survival_break_pos()
}

fn should_place_survival_block(
    game_mode: GameMode,
    hand: Hand,
    position: BlockPos,
    face: Direction,
) -> bool {
    game_mode == GameMode::Survival
        && hand == Hand::Main
        && position == survival_break_pos()
        && face == Direction::Up
}

fn survival_break_pos() -> BlockPos {
    BlockPos::new(SURVIVAL_TARGET_X, FLOOR_Y, SURVIVAL_TARGET_Z)
}

fn survival_block_state() -> BlockState {
    BlockState::DIRT
}

fn should_open_survival_chest(game_mode: GameMode, hand: Hand, position: BlockPos) -> bool {
    game_mode == GameMode::Survival && hand == Hand::Main && position == survival_chest_pos()
}

fn is_survival_chest_store_event(window_id: u8, slot_id: i16, slot_changes: &[SlotChange]) -> bool {
    window_id == SURVIVAL_CHEST_WINDOW
        && slot_id == SURVIVAL_CHEST_SLOT_ID
        && slot_changes.iter().any(|change| {
            change.idx == SURVIVAL_CHEST_SLOT_ID && is_survival_chest_item(&change.stack)
        })
}

fn is_survival_chest_item(stack: &ItemStack) -> bool {
    stack.item == survival_chest_item_kind() && stack.count == SURVIVAL_CHEST_ITEM_COUNT
}

fn should_open_survival_crafting(game_mode: GameMode, hand: Hand, position: BlockPos) -> bool {
    game_mode == GameMode::Survival && hand == Hand::Main && position == survival_crafting_pos()
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
    window_id == SURVIVAL_CRAFTING_WINDOW
        && slot_id == SURVIVAL_CRAFTING_RESULT_SLOT_ID
        && is_survival_crafting_result(carried_item)
}

fn is_survival_crafting_result(stack: &ItemStack) -> bool {
    stack.item == survival_crafting_result_kind() && stack.count == SURVIVAL_CRAFTING_RESULT_COUNT
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

fn survival_chest_fixture_enabled() -> bool {
    std::env::var(SURVIVAL_CHEST_FIXTURE_ENV).as_deref() == Ok("1")
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

fn survival_crafting_fixture_enabled() -> bool {
    std::env::var(SURVIVAL_CRAFTING_FIXTURE_ENV).as_deref() == Ok("1")
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

fn survival_furnace_fixture_enabled() -> bool {
    std::env::var(SURVIVAL_FURNACE_FIXTURE_ENV).as_deref() == Ok("1")
}

fn survival_hunger_food_fixture_enabled() -> bool {
    std::env::var(SURVIVAL_HUNGER_FOOD_FIXTURE_ENV).as_deref() == Ok("1")
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
    game_mode == GameMode::Survival && hand == Hand::Main && position == survival_furnace_pos()
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
    window_id == SURVIVAL_FURNACE_WINDOW
        && slot_id == SURVIVAL_FURNACE_OUTPUT_SLOT_ID
        && is_survival_furnace_output(carried_item)
}

fn is_survival_furnace_output(stack: &ItemStack) -> bool {
    stack.item == survival_furnace_output_kind() && stack.count == SURVIVAL_FURNACE_ITEM_COUNT
}

fn is_empty_item(stack: &ItemStack) -> bool {
    stack.count == 0
}

fn survival_furnace_input_stack() -> ItemStack {
    ItemStack::new(
        survival_furnace_input_kind(),
        SURVIVAL_FURNACE_ITEM_COUNT,
        None,
    )
}

fn survival_furnace_fuel_stack() -> ItemStack {
    ItemStack::new(
        survival_furnace_fuel_kind(),
        SURVIVAL_FURNACE_ITEM_COUNT,
        None,
    )
}

fn survival_furnace_output_stack() -> ItemStack {
    ItemStack::new(
        survival_furnace_output_kind(),
        SURVIVAL_FURNACE_ITEM_COUNT,
        None,
    )
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

fn survival_mob_drop_fixture_enabled() -> bool {
    std::env::var(SURVIVAL_MOB_DROP_FIXTURE_ENV).as_deref() == Ok("1")
}

fn survival_redstone_toggle_fixture_enabled() -> bool {
    std::env::var(SURVIVAL_REDSTONE_TOGGLE_FIXTURE_ENV).as_deref() == Ok("1")
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
    game_mode == GameMode::Survival
        && hand == Hand::Main
        && position == survival_redstone_toggle_control_pos()
}

fn survival_world_persistence_fixture_enabled() -> bool {
    std::env::var(SURVIVAL_WORLD_PERSISTENCE_FIXTURE_ENV).as_deref() == Ok("1")
}

fn survival_world_persistence_marker_path() -> PathBuf {
    std::env::var(SURVIVAL_WORLD_PERSISTENCE_DIR_ENV)
        .map(PathBuf::from)
        .unwrap_or_else(|_| std::env::temp_dir().join("mc-compat-world-persistence"))
        .join(SURVIVAL_WORLD_PERSISTENCE_MARKER_FILE)
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
    stack.item == survival_item_kind() && stack.count >= SURVIVAL_WORLD_PERSISTENCE_ITEM_COUNT
}

fn should_place_survival_world_persistence(
    game_mode: GameMode,
    hand: Hand,
    position: BlockPos,
    face: Direction,
) -> bool {
    game_mode == GameMode::Survival
        && hand == Hand::Main
        && position == survival_world_persistence_base_pos()
        && face == Direction::Up
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

fn survival_block_entity_fixture_enabled() -> bool {
    std::env::var(SURVIVAL_BLOCK_ENTITY_FIXTURE_ENV).as_deref() == Ok("1")
}

fn survival_block_entity_post_restart_phase() -> bool {
    std::env::var(SURVIVAL_BLOCK_ENTITY_PHASE_ENV).as_deref()
        == Ok(SURVIVAL_BLOCK_ENTITY_POST_RESTART_PHASE)
}

fn survival_block_entity_marker_path() -> PathBuf {
    std::env::var(SURVIVAL_BLOCK_ENTITY_DIR_ENV)
        .map(PathBuf::from)
        .unwrap_or_else(|_| std::env::temp_dir().join("mc-compat-block-entity-persistence"))
        .join(SURVIVAL_BLOCK_ENTITY_MARKER_FILE)
}

fn survival_block_entity_should_place_sign(persisted_loaded: bool) -> bool {
    !survival_block_entity_post_restart_phase() || persisted_loaded
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

fn log_survival_block_entity_persistence(username: &str, fixture: &mut SurvivalBlockEntityFixture) {
    if survival_block_entity_post_restart_phase() {
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
    stack.item == survival_mob_drop_item_kind() && stack.count == SURVIVAL_MOB_DROP_ITEM_COUNT
}

fn should_handle_survival_mob_drop_attack(
    game_mode: GameMode,
    interaction: EntityInteraction,
    target: Entity,
    expected_target: Entity,
) -> bool {
    game_mode == GameMode::Survival
        && interaction == EntityInteraction::Attack
        && target == expected_target
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
    let drop = commands
        .spawn(ItemEntityBundle {
            id: drop_id,
            layer: EntityLayerId(layer),
            position: survival_mob_drop_position(),
            item_stack: ItemEntityStack(survival_mob_drop_stack()),
            ..Default::default()
        })
        .id();
    fixture.drop = Some(drop);
    fixture.drop_id = Some(drop_id.get());
    fixture.collector = Some(collector);
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
            fixture.drop_id.expect("drop id set before pickup"),
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
    hand: Hand,
    sequence: i32,
    held_slot: u16,
    stack: &ItemStack,
    health: f32,
    food: i32,
    saturation: f32,
) -> bool {
    hand == Hand::Main
        && sequence == SURVIVAL_HUNGER_FOOD_USE_SEQUENCE
        && held_slot == SURVIVAL_HUNGER_FOOD_INVENTORY_SLOT
        && is_survival_hunger_food_stack(stack)
        && health == SURVIVAL_HUNGER_FOOD_PRE_HEALTH
        && food == SURVIVAL_HUNGER_FOOD_PRE_FOOD
        && saturation == SURVIVAL_HUNGER_FOOD_PRE_SATURATION
}

fn log_survival_hunger_food_pre(username: &str, fixture: &mut SurvivalHungerFoodFixture) {
    if fixture.pre_logged {
        return;
    }
    fixture.pre_logged = true;
    log_milestone(format!(
        "MC-COMPAT-MILESTONE survival_hunger_food_pre username={} health={:.1} food={} \
         saturation={:.1} item={} count={} slot={}",
        username,
        SURVIVAL_HUNGER_FOOD_PRE_HEALTH,
        SURVIVAL_HUNGER_FOOD_PRE_FOOD,
        SURVIVAL_HUNGER_FOOD_PRE_SATURATION,
        SURVIVAL_HUNGER_FOOD_ITEM_NAME,
        SURVIVAL_HUNGER_FOOD_ITEM_COUNT_BEFORE,
        SURVIVAL_HUNGER_FOOD_INVENTORY_SLOT
    ));
}

fn survival_biome_dimension_fixture_enabled() -> bool {
    std::env::var(SURVIVAL_BIOME_DIMENSION_FIXTURE_ENV).as_deref() == Ok("1")
}

fn normalize_survival_environment_id(raw: &str) -> &'static str {
    match raw {
        SURVIVAL_OVERWORLD_ID => SURVIVAL_OVERWORLD_ID,
        SURVIVAL_NETHER_ID => SURVIVAL_NETHER_ID,
        SURVIVAL_END_ID => SURVIVAL_END_ID,
        _ => SURVIVAL_UNKNOWN_ENVIRONMENT_ID,
    }
}

fn derive_survival_environment_id(
    spawn_environment: &str,
    environment_identifier: &str,
) -> &'static str {
    let environment = normalize_survival_environment_id(environment_identifier);
    if environment != SURVIVAL_UNKNOWN_ENVIRONMENT_ID {
        environment
    } else {
        normalize_survival_environment_id(spawn_environment)
    }
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
        assert!(survival_block_entity_should_place_sign(false));
        assert!(survival_block_entity_should_place_sign(true));
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
            Hand::Main,
            SURVIVAL_HUNGER_FOOD_USE_SEQUENCE,
            SURVIVAL_HUNGER_FOOD_INVENTORY_SLOT,
            &survival_hunger_food_stack(),
            SURVIVAL_HUNGER_FOOD_PRE_HEALTH,
            SURVIVAL_HUNGER_FOOD_PRE_FOOD,
            SURVIVAL_HUNGER_FOOD_PRE_SATURATION,
        ));
        assert!(!should_consume_survival_hunger_food(
            Hand::Off,
            SURVIVAL_HUNGER_FOOD_USE_SEQUENCE,
            SURVIVAL_HUNGER_FOOD_INVENTORY_SLOT,
            &survival_hunger_food_stack(),
            SURVIVAL_HUNGER_FOOD_PRE_HEALTH,
            SURVIVAL_HUNGER_FOOD_PRE_FOOD,
            SURVIVAL_HUNGER_FOOD_PRE_SATURATION,
        ));
        assert!(!should_consume_survival_hunger_food(
            Hand::Main,
            SURVIVAL_HUNGER_FOOD_USE_SEQUENCE + 1,
            SURVIVAL_HUNGER_FOOD_INVENTORY_SLOT,
            &survival_hunger_food_stack(),
            SURVIVAL_HUNGER_FOOD_PRE_HEALTH,
            SURVIVAL_HUNGER_FOOD_PRE_FOOD,
            SURVIVAL_HUNGER_FOOD_PRE_SATURATION,
        ));
        assert!(!should_consume_survival_hunger_food(
            Hand::Main,
            SURVIVAL_HUNGER_FOOD_USE_SEQUENCE,
            SURVIVAL_HUNGER_FOOD_INVENTORY_SLOT + 1,
            &survival_hunger_food_stack(),
            SURVIVAL_HUNGER_FOOD_PRE_HEALTH,
            SURVIVAL_HUNGER_FOOD_PRE_FOOD,
            SURVIVAL_HUNGER_FOOD_PRE_SATURATION,
        ));
        assert!(!should_consume_survival_hunger_food(
            Hand::Main,
            SURVIVAL_HUNGER_FOOD_USE_SEQUENCE,
            SURVIVAL_HUNGER_FOOD_INVENTORY_SLOT,
            &survival_furnace_output_stack(),
            SURVIVAL_HUNGER_FOOD_PRE_HEALTH,
            SURVIVAL_HUNGER_FOOD_PRE_FOOD,
            SURVIVAL_HUNGER_FOOD_PRE_SATURATION,
        ));
        assert!(!should_consume_survival_hunger_food(
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
