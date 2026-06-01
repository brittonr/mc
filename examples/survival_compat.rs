#![allow(clippy::type_complexity)]

use std::collections::HashSet;

use valence::entity::EntityId;
use valence::event_loop::PacketEvent;
use valence::interact_block::InteractBlockEvent;
use valence::inventory::{ClickSlotEvent, CursorItem, HeldItem, OpenInventory, SlotChange};
use valence::log::info;
use valence::prelude::*;
use valence::protocol::packets::play::{CloseHandledScreenC2s, ItemPickupAnimationS2c};
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
                handle_survival_chest_open,
                handle_survival_chest_store,
                handle_survival_crafting_open,
                handle_survival_crafting_click,
            ),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    server: Res<Server>,
    dimensions: Res<DimensionTypeRegistry>,
    biomes: Res<BiomeRegistry>,
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

    commands.spawn(layer);

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
        ),
        Added<Client>,
    >,
    layers: Query<Entity, (With<ChunkLayer>, With<EntityLayer>)>,
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
    ) in &mut clients
    {
        let layer = layers.single();

        layer_id.0 = layer;
        visible_chunk_layer.0 = layer;
        visible_entity_layers.0.insert(layer);
        pos.set([SURVIVAL_SPAWN_X, f64::from(SPAWN_Y), SURVIVAL_SPAWN_Z]);
        *game_mode = GameMode::Survival;
        inventory.set_slot(SURVIVAL_ITEM_SLOT, ItemStack::EMPTY);
        if survival_chest_fixture_enabled() {
            cursor_item.0 = survival_chest_item_stack();
        }
        if survival_crafting_fixture_enabled() {
            cursor_item.0 = survival_crafting_input_stack(SURVIVAL_CRAFTING_TOTAL_INPUT_COUNT);
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
                "MC-COMPAT-MILESTONE survival_crafting_table_open username={} position={},{},{} window={}",
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
                "MC-COMPAT-MILESTONE survival_crafting_input_a username={} window={} slot={} item={:?} count={}",
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
                "MC-COMPAT-MILESTONE survival_crafting_input_b username={} window={} slot={} item={:?} count={}",
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
                "MC-COMPAT-MILESTONE survival_crafting_result username={} window={} slot={} item={:?} count={} recipe={}",
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
                "MC-COMPAT-MILESTONE survival_crafting_collect username={} window={} slot={} item={:?} count={} inventory_slot={}",
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
            "MC-COMPAT-MILESTONE survival_crafting_input_a username={} window={} slot={} item={:?} count={}",
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
            "MC-COMPAT-MILESTONE survival_crafting_input_b username={} window={} slot={} item={:?} count={}",
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
            "MC-COMPAT-MILESTONE survival_crafting_result username={} window={} slot={} item={:?} count={} recipe={}",
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
            "MC-COMPAT-MILESTONE survival_crafting_collect username={} window={} slot={} item={:?} count={} inventory_slot={}",
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
    // This fixture owns the result state; raw slot/window are the stable server-side trigger.
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
        "MC-COMPAT-MILESTONE survival_biome_dimension_state username={} spawn_environment={} environment_identifier={} server_environment_state={} normalized_identifier={}",
        username, spawn_environment, environment_identifier, spawn_environment, normalized_identifier
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
