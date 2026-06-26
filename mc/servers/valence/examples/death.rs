#![allow(clippy::type_complexity)]

use bevy_ecs::prelude::SystemSet;
use valence::prelude::*;
use valence::status::RequestRespawnEvent;

const SPAWN_Y: i32 = 64;

#[derive(SystemSet, Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum DeathExamplePhase {
    Input,
    RuleEvaluation,
    WorldMutation,
    Cleanup,
}

#[derive(Resource, Clone, Copy, Debug, PartialEq)]
struct DeathExamplePluginContract {
    update_phase_order: &'static [DeathExamplePhase],
}

const DEATH_EXAMPLE_PHASE_ORDER: &[DeathExamplePhase] = &[
    DeathExamplePhase::Input,
    DeathExamplePhase::RuleEvaluation,
    DeathExamplePhase::WorldMutation,
    DeathExamplePhase::Cleanup,
];

struct DeathExamplePlugin;

impl Plugin for DeathExamplePlugin {
    fn build(&self, app: &mut App) {
        let contract = DeathExamplePluginContract {
            update_phase_order: DEATH_EXAMPLE_PHASE_ORDER,
        };

        app.insert_resource(contract)
            .configure_sets(
                Update,
                (
                    DeathExamplePhase::Input,
                    DeathExamplePhase::RuleEvaluation,
                    DeathExamplePhase::WorldMutation,
                    DeathExamplePhase::Cleanup,
                )
                    .chain(),
            )
            .add_systems(Startup, setup)
            .add_systems(Update, init_clients.in_set(DeathExamplePhase::Input))
            .add_systems(
                Update,
                squat_and_die.in_set(DeathExamplePhase::RuleEvaluation),
            )
            .add_systems(Update, necromancy.in_set(DeathExamplePhase::WorldMutation))
            .add_systems(
                Update,
                despawn_disconnected_clients.in_set(DeathExamplePhase::Cleanup),
            );
    }
}

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DeathExamplePlugin)
        .run();
}

fn setup(
    mut commands: Commands,
    server: Res<Server>,
    dimensions: Res<DimensionTypeRegistry>,
    biomes: Res<BiomeRegistry>,
) {
    for block in [
        BlockState::GRASS_BLOCK,
        BlockState::DEEPSLATE,
        BlockState::MAGMA_BLOCK,
    ] {
        let mut layer = LayerBundle::new(ident!("overworld"), &dimensions, &biomes, &server);

        for z in -5..5 {
            for x in -5..5 {
                layer.chunk.insert_chunk([x, z], UnloadedChunk::new());
            }
        }

        for z in -25..25 {
            for x in -25..25 {
                layer.chunk.set_block([x, SPAWN_Y, z], block);
            }
        }

        commands.spawn(layer);
    }
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
        let layer = layers.iter().next().unwrap();

        layer_id.0 = layer;
        visible_chunk_layer.0 = layer;
        visible_entity_layers.0.insert(layer);
        pos.set([0.0, f64::from(SPAWN_Y) + 1.0, 0.0]);
        *game_mode = GameMode::Creative;

        client.send_chat_message(
            "Welcome to Valence! Sneak to die in the game (but not in real life).".italic(),
        );
    }
}

fn squat_and_die(mut clients: Query<&mut Client>, mut events: EventReader<SneakEvent>) {
    for event in events.read() {
        if event.state == SneakState::Start {
            if let Ok(mut client) = clients.get_mut(event.client) {
                client.kill("Squatted too hard.");
            }
        }
    }
}

fn necromancy(
    mut clients: Query<(
        &mut EntityLayerId,
        &mut VisibleChunkLayer,
        &mut VisibleEntityLayers,
        &mut RespawnPosition,
    )>,
    mut events: EventReader<RequestRespawnEvent>,
    layers: Query<Entity, (With<ChunkLayer>, With<EntityLayer>)>,
) {
    for event in events.read() {
        if let Ok((
            mut layer_id,
            mut visible_chunk_layer,
            mut visible_entity_layers,
            mut respawn_pos,
        )) = clients.get_mut(event.client)
        {
            respawn_pos.pos = BlockPos::new(0, SPAWN_Y, 0);

            // make the client respawn in another chunk layer.

            let idx = layers.iter().position(|l| l == layer_id.0).unwrap();
            let count = layers.iter().len();
            let Some(next_index) = next_respawn_layer_index(idx, count) else {
                continue;
            };
            let layer = layers.into_iter().nth(next_index).unwrap();

            layer_id.0 = layer;
            visible_chunk_layer.0 = layer;
            visible_entity_layers.0.clear();
            visible_entity_layers.0.insert(layer);
        }
    }
}

fn next_respawn_layer_index(current_index: usize, layer_count: usize) -> Option<usize> {
    if layer_count == 0 {
        return None;
    }

    Some((current_index + 1) % layer_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIRST_LAYER_INDEX: usize = 0;
    const SECOND_LAYER_INDEX: usize = 1;
    const LAYER_COUNT: usize = 3;
    const LAST_LAYER_INDEX: usize = LAYER_COUNT - 1;
    const NO_LAYERS: usize = 0;

    #[test]
    fn death_example_plugin_installs_contract() {
        let mut app = App::new();

        app.add_plugins(DeathExamplePlugin);

        let contract = app.world().resource::<DeathExamplePluginContract>();
        assert_eq!(contract.update_phase_order, DEATH_EXAMPLE_PHASE_ORDER);
    }

    #[test]
    fn disabled_death_example_plugin_installs_no_contract() {
        let app = App::new();

        assert!(!app
            .world()
            .contains_resource::<DeathExamplePluginContract>());
    }

    #[test]
    fn next_respawn_layer_index_advances_to_next_layer() {
        assert_eq!(
            next_respawn_layer_index(FIRST_LAYER_INDEX, LAYER_COUNT),
            Some(SECOND_LAYER_INDEX)
        );
    }

    #[test]
    fn next_respawn_layer_index_wraps_last_layer() {
        assert_eq!(
            next_respawn_layer_index(LAST_LAYER_INDEX, LAYER_COUNT),
            Some(FIRST_LAYER_INDEX)
        );
    }

    #[test]
    fn next_respawn_layer_index_rejects_empty_layers() {
        assert_eq!(next_respawn_layer_index(FIRST_LAYER_INDEX, NO_LAYERS), None);
    }
}
