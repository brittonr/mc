#![allow(clippy::type_complexity)]

use valence::entity::EntityId;
use valence::interact_block::InteractBlockEvent;
use valence::inventory::HeldItem;
use valence::log::info;
use valence::prelude::*;
use valence::protocol::packets::play::ItemPickupAnimationS2c;
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

pub fn main() {
    App::new()
        .insert_resource(NetworkSettings {
            connection_mode: ConnectionMode::Offline,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                init_clients,
                despawn_disconnected_clients,
                handle_survival_digging,
                handle_survival_block_place,
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

    commands.spawn(layer);
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
    ) in &mut clients
    {
        let layer = layers.single();

        layer_id.0 = layer;
        visible_chunk_layer.0 = layer;
        visible_entity_layers.0.insert(layer);
        pos.set([SURVIVAL_SPAWN_X, f64::from(SPAWN_Y), SURVIVAL_SPAWN_Z]);
        *game_mode = GameMode::Survival;
        inventory.set_slot(SURVIVAL_ITEM_SLOT, ItemStack::EMPTY);

        client.send_chat_message(SURVIVAL_WELCOME.italic());
        log_milestone(format!(
            "MC-COMPAT-MILESTONE survival_join username={} gamemode=Survival target={},{},{}",
            username.as_str(),
            SURVIVAL_TARGET_X,
            FLOOR_Y,
            SURVIVAL_TARGET_Z
        ));
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
}
