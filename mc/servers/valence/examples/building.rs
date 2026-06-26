#![allow(clippy::type_complexity)]

use bevy_ecs::prelude::SystemSet;
use valence::interact_block::InteractBlockEvent;
use valence::inventory::HeldItem;
use valence::prelude::*;

const SPAWN_Y: i32 = 64;

#[derive(SystemSet, Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum BuildingExamplePhase {
    Input,
    RuleEvaluation,
    WorldMutation,
    Cleanup,
}

#[derive(Resource, Clone, Copy, Debug, PartialEq)]
struct BuildingExamplePluginContract {
    update_phase_order: &'static [BuildingExamplePhase],
}

const BUILDING_EXAMPLE_PHASE_ORDER: &[BuildingExamplePhase] = &[
    BuildingExamplePhase::Input,
    BuildingExamplePhase::RuleEvaluation,
    BuildingExamplePhase::WorldMutation,
    BuildingExamplePhase::Cleanup,
];

struct BuildingExamplePlugin;

impl Plugin for BuildingExamplePlugin {
    fn build(&self, app: &mut App) {
        let contract = BuildingExamplePluginContract {
            update_phase_order: BUILDING_EXAMPLE_PHASE_ORDER,
        };

        app.insert_resource(contract)
            .configure_sets(
                Update,
                (
                    BuildingExamplePhase::Input,
                    BuildingExamplePhase::RuleEvaluation,
                    BuildingExamplePhase::WorldMutation,
                    BuildingExamplePhase::Cleanup,
                )
                    .chain(),
            )
            .add_systems(Startup, setup)
            .add_systems(Update, init_clients.in_set(BuildingExamplePhase::Input))
            .add_systems(
                Update,
                toggle_gamemode_on_sneak.in_set(BuildingExamplePhase::RuleEvaluation),
            )
            .add_systems(
                Update,
                (digging, place_blocks).in_set(BuildingExamplePhase::WorldMutation),
            )
            .add_systems(
                Update,
                despawn_disconnected_clients.in_set(BuildingExamplePhase::Cleanup),
            );
    }
}

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BuildingExamplePlugin)
        .run();
}

fn setup(
    mut commands: Commands,
    server: Res<Server>,
    dimensions: Res<DimensionTypeRegistry>,
    biomes: Res<BiomeRegistry>,
) {
    let mut layer = LayerBundle::new(ident!("overworld"), &dimensions, &biomes, &server);

    for z in -5..5 {
        for x in -5..5 {
            layer.chunk.insert_chunk([x, z], UnloadedChunk::new());
        }
    }

    for z in -50..50 {
        for x in -50..50 {
            layer
                .chunk
                .set_block([x, SPAWN_Y, z], BlockState::GRASS_BLOCK);
        }
    }

    commands.spawn(layer);
}

fn init_clients(
    mut clients: Query<
        (
            &mut Client,
            &mut EntityLayerId,
            &mut VisibleChunkLayer,
            &mut VisibleEntityLayers,
            &mut Position,
            &mut GameMode,
        ),
        Added<Client>,
    >,
    layers: Query<Entity, (With<ChunkLayer>, With<EntityLayer>)>,
) {
    for (
        mut client,
        mut layer_id,
        mut visible_chunk_layer,
        mut visible_entity_layers,
        mut pos,
        mut game_mode,
    ) in &mut clients
    {
        let layer = layers.single();

        layer_id.0 = layer;
        visible_chunk_layer.0 = layer;
        visible_entity_layers.0.insert(layer);
        pos.set([0.0, f64::from(SPAWN_Y) + 1.0, 0.0]);
        *game_mode = GameMode::Creative;

        client.send_chat_message("Welcome to Valence! Build something cool.".italic());
    }
}

fn toggle_gamemode_on_sneak(
    mut clients: Query<&mut GameMode>,
    mut events: EventReader<SneakEvent>,
) {
    for event in events.read() {
        let Ok(mut mode) = clients.get_mut(event.client) else {
            continue;
        };
        if event.state == SneakState::Start {
            *mode = toggled_building_game_mode(*mode);
        }
    }
}

fn digging(
    clients: Query<&GameMode>,
    mut layers: Query<&mut ChunkLayer>,
    mut events: EventReader<DiggingEvent>,
) {
    let mut layer = layers.single_mut();

    for event in events.read() {
        let Ok(game_mode) = clients.get(event.client) else {
            continue;
        };

        if should_break_building_block(*game_mode, event.state) {
            layer.set_block(event.position, BlockState::AIR);
        }
    }
}

fn place_blocks(
    mut clients: Query<(&mut Inventory, &GameMode, &HeldItem)>,
    mut layers: Query<&mut ChunkLayer>,
    mut events: EventReader<InteractBlockEvent>,
) {
    let mut layer = layers.single_mut();

    for event in events.read() {
        let Ok((mut inventory, game_mode, held)) = clients.get_mut(event.client) else {
            continue;
        };
        if event.hand != Hand::Main {
            continue;
        }

        // get the held item
        let slot_id = held.slot();
        let stack = inventory.slot(slot_id);
        if stack.is_empty() {
            // no item in the slot
            continue;
        };

        let Some(block_kind) = BlockKind::from_item_kind(stack.item) else {
            // can't place this item as a block
            continue;
        };

        if *game_mode == GameMode::Survival {
            // check if the player has the item in their inventory and remove
            // it.
            if stack.count > 1 {
                let amount = stack.count - 1;
                inventory.set_slot_amount(slot_id, amount);
            } else {
                inventory.set_slot(slot_id, ItemStack::EMPTY);
            }
        }
        let real_pos = event.position.get_in_direction(event.face);
        let state = block_kind
            .to_state()
            .set(PropName::Axis, placement_axis(event.face));
        layer.set_block(real_pos, state);
    }
}

fn toggled_building_game_mode(game_mode: GameMode) -> GameMode {
    match game_mode {
        GameMode::Survival => GameMode::Creative,
        GameMode::Creative => GameMode::Survival,
        _ => GameMode::Creative,
    }
}

fn should_break_building_block(game_mode: GameMode, digging_state: DiggingState) -> bool {
    (game_mode == GameMode::Creative && digging_state == DiggingState::Start)
        || (game_mode == GameMode::Survival && digging_state == DiggingState::Stop)
}

fn placement_axis(face: Direction) -> PropValue {
    match face {
        Direction::Down | Direction::Up => PropValue::Y,
        Direction::North | Direction::South => PropValue::Z,
        Direction::West | Direction::East => PropValue::X,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn building_example_plugin_installs_contract() {
        let mut app = App::new();

        app.add_plugins(BuildingExamplePlugin);

        let contract = app.world().resource::<BuildingExamplePluginContract>();
        assert_eq!(contract.update_phase_order, BUILDING_EXAMPLE_PHASE_ORDER);
    }

    #[test]
    fn disabled_building_example_plugin_installs_no_contract() {
        let app = App::new();

        assert!(!app
            .world()
            .contains_resource::<BuildingExamplePluginContract>());
    }

    #[test]
    fn building_mode_toggle_preserves_existing_cycle() {
        assert_eq!(
            toggled_building_game_mode(GameMode::Creative),
            GameMode::Survival
        );
        assert_eq!(
            toggled_building_game_mode(GameMode::Survival),
            GameMode::Creative
        );
    }

    #[test]
    fn building_mode_toggle_falls_back_to_creative_for_other_modes() {
        assert_eq!(
            toggled_building_game_mode(GameMode::Adventure),
            GameMode::Creative
        );
    }

    #[test]
    fn block_break_policy_accepts_matching_mode_states() {
        assert!(should_break_building_block(
            GameMode::Creative,
            DiggingState::Start
        ));
        assert!(should_break_building_block(
            GameMode::Survival,
            DiggingState::Stop
        ));
    }

    #[test]
    fn block_break_policy_rejects_mismatched_mode_states() {
        assert!(!should_break_building_block(
            GameMode::Creative,
            DiggingState::Stop
        ));
        assert!(!should_break_building_block(
            GameMode::Survival,
            DiggingState::Start
        ));
    }

    #[test]
    fn placement_axis_matches_clicked_face() {
        assert_eq!(placement_axis(Direction::Up), PropValue::Y);
        assert_eq!(placement_axis(Direction::North), PropValue::Z);
        assert_eq!(placement_axis(Direction::East), PropValue::X);
    }
}
