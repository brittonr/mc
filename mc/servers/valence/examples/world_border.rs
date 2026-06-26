#![allow(clippy::type_complexity)]

use bevy_app::App;
use bevy_ecs::prelude::SystemSet;
use valence::client::despawn_disconnected_clients;
use valence::inventory::HeldItem;
use valence::message::{ChatMessageEvent, SendMessage};
use valence::prelude::*;
use valence::world_border::*;

const SPAWN_Y: i32 = 64;
const INITIAL_BORDER_DIAMETER: f64 = 10.0;
const BORDER_ADD_COMMAND: &str = "add";
const BORDER_CENTER_COMMAND: &str = "center";

#[derive(SystemSet, Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum WorldBorderExamplePhase {
    Input,
    WorldMutation,
    Presentation,
    Cleanup,
}

#[derive(Resource, Clone, Copy, Debug, PartialEq)]
struct WorldBorderExamplePluginContract {
    update_phase_order: &'static [WorldBorderExamplePhase],
}

const WORLD_BORDER_EXAMPLE_PHASE_ORDER: &[WorldBorderExamplePhase] = &[
    WorldBorderExamplePhase::Input,
    WorldBorderExamplePhase::WorldMutation,
    WorldBorderExamplePhase::Presentation,
    WorldBorderExamplePhase::Cleanup,
];

struct WorldBorderExamplePlugin;

impl Plugin for WorldBorderExamplePlugin {
    fn build(&self, app: &mut App) {
        let contract = WorldBorderExamplePluginContract {
            update_phase_order: WORLD_BORDER_EXAMPLE_PHASE_ORDER,
        };

        app.insert_resource(contract)
            .configure_sets(
                Update,
                (
                    WorldBorderExamplePhase::Input,
                    WorldBorderExamplePhase::WorldMutation,
                    WorldBorderExamplePhase::Presentation,
                    WorldBorderExamplePhase::Cleanup,
                )
                    .chain(),
            )
            .add_systems(Startup, setup)
            .add_systems(Update, init_clients.in_set(WorldBorderExamplePhase::Input))
            .add_systems(
                Update,
                border_controls.in_set(WorldBorderExamplePhase::WorldMutation),
            )
            .add_systems(
                Update,
                display_diameter.in_set(WorldBorderExamplePhase::Presentation),
            )
            .add_systems(
                Update,
                despawn_disconnected_clients.in_set(WorldBorderExamplePhase::Cleanup),
            );
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldBorderExamplePlugin)
        .run();
}

fn setup(
    mut commands: Commands,
    server: Res<Server>,
    biomes: Res<BiomeRegistry>,
    dimensions: Res<DimensionTypeRegistry>,
) {
    let mut layer = LayerBundle::new(ident!("overworld"), &dimensions, &biomes, &server);

    for z in -5..5 {
        for x in -5..5 {
            layer.chunk.insert_chunk([x, z], UnloadedChunk::new());
        }
    }

    for z in -25..25 {
        for x in -25..25 {
            layer
                .chunk
                .set_block([x, SPAWN_Y, z], BlockState::MOSSY_COBBLESTONE);
        }
    }

    commands.spawn((
        layer,
        WorldBorderBundle {
            lerp: WorldBorderLerp {
                target_diameter: INITIAL_BORDER_DIAMETER,
                ..Default::default()
            },
            ..Default::default()
        },
    ));
}

fn init_clients(
    mut clients: Query<
        (
            &mut Client,
            &mut EntityLayerId,
            &mut VisibleChunkLayer,
            &mut VisibleEntityLayers,
            &mut Position,
            &mut Inventory,
            &HeldItem,
        ),
        Added<Client>,
    >,
    layers: Query<Entity, With<ChunkLayer>>,
) {
    for (
        mut client,
        mut layer_id,
        mut visible_chunk_layer,
        mut visible_entity_layers,
        mut pos,
        mut inv,
        main_slot,
    ) in &mut clients
    {
        let layer = layers.single();

        layer_id.0 = layer;
        visible_chunk_layer.0 = layer;
        visible_entity_layers.0.insert(layer);
        pos.set([0.5, f64::from(SPAWN_Y) + 1.0, 0.5]);
        let pickaxe = ItemStack::new(ItemKind::WoodenPickaxe, 1, None);
        inv.set_slot(main_slot.slot(), pickaxe);
        client
            .send_chat_message("Use `add` and `center` chat messages to change the world border.");
    }
}

fn display_diameter(mut layers: Query<(&mut ChunkLayer, &WorldBorderLerp)>) {
    for (mut layer, lerp) in &mut layers {
        if lerp.remaining_ticks > 0 {
            layer.send_chat_message(format!("diameter = {}", lerp.current_diameter));
        }
    }
}

fn border_controls(
    mut events: EventReader<ChatMessageEvent>,
    mut layers: Query<(&mut WorldBorderCenter, &mut WorldBorderLerp), With<ChunkLayer>>,
) {
    for event in events.read() {
        let Some(control) = parse_world_border_control(&event.message) else {
            continue;
        };

        let (mut center, mut lerp) = layers.single_mut();
        match plan_world_border_control(control, lerp.current_diameter) {
            WorldBorderMutation::SetDiameter {
                target_diameter,
                remaining_ticks,
            } => {
                lerp.target_diameter = target_diameter;
                lerp.remaining_ticks = remaining_ticks;
            }
            WorldBorderMutation::SetCenter { x, z } => {
                center.x = x;
                center.z = z;
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum WorldBorderControl {
    AddDiameter { value: f64, ticks: u64 },
    SetCenter { x: f64, z: f64 },
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum WorldBorderMutation {
    SetDiameter {
        target_diameter: f64,
        remaining_ticks: u64,
    },
    SetCenter {
        x: f64,
        z: f64,
    },
}

fn parse_world_border_control(message: &str) -> Option<WorldBorderControl> {
    let mut parts = message.split_whitespace();
    match parts.next()? {
        BORDER_ADD_COMMAND => {
            let value = parts.next()?.parse::<f64>().ok()?;
            let ticks = parts.next()?.parse::<u64>().ok()?;
            Some(WorldBorderControl::AddDiameter { value, ticks })
        }
        BORDER_CENTER_COMMAND => {
            let x = parts.next()?.parse::<f64>().ok()?;
            let z = parts.next()?.parse::<f64>().ok()?;
            Some(WorldBorderControl::SetCenter { x, z })
        }
        _ => None,
    }
}

fn plan_world_border_control(
    control: WorldBorderControl,
    current_diameter: f64,
) -> WorldBorderMutation {
    match control {
        WorldBorderControl::AddDiameter { value, ticks } => WorldBorderMutation::SetDiameter {
            target_diameter: current_diameter + value,
            remaining_ticks: ticks,
        },
        WorldBorderControl::SetCenter { x, z } => WorldBorderMutation::SetCenter { x, z },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BORDER_DELTA: f64 = 2.5;
    const BORDER_CURRENT_DIAMETER: f64 = INITIAL_BORDER_DIAMETER;
    const BORDER_TARGET_DIAMETER: f64 = BORDER_CURRENT_DIAMETER + BORDER_DELTA;
    const BORDER_LERP_TICKS: u64 = 20;
    const BORDER_CENTER_X: f64 = 4.0;
    const BORDER_CENTER_Z: f64 = -8.0;

    #[test]
    fn world_border_example_plugin_installs_contract() {
        let mut app = App::new();

        app.add_plugins(WorldBorderExamplePlugin);

        let contract = app.world().resource::<WorldBorderExamplePluginContract>();
        assert_eq!(
            contract.update_phase_order,
            WORLD_BORDER_EXAMPLE_PHASE_ORDER
        );
    }

    #[test]
    fn disabled_world_border_example_plugin_installs_no_contract() {
        let app = App::new();

        assert!(!app
            .world()
            .contains_resource::<WorldBorderExamplePluginContract>());
    }

    #[test]
    fn parses_add_and_center_border_controls() {
        assert_eq!(
            parse_world_border_control("add 2.5 20"),
            Some(WorldBorderControl::AddDiameter {
                value: BORDER_DELTA,
                ticks: BORDER_LERP_TICKS,
            })
        );
        assert_eq!(
            parse_world_border_control("center 4 -8"),
            Some(WorldBorderControl::SetCenter {
                x: BORDER_CENTER_X,
                z: BORDER_CENTER_Z,
            })
        );
    }

    #[test]
    fn rejects_unknown_or_malformed_border_controls() {
        assert_eq!(parse_world_border_control("unknown"), None);
        assert_eq!(parse_world_border_control("add nope 20"), None);
        assert_eq!(parse_world_border_control("center 4"), None);
    }

    #[test]
    fn plans_border_diameter_relative_to_current_diameter() {
        let mutation = plan_world_border_control(
            WorldBorderControl::AddDiameter {
                value: BORDER_DELTA,
                ticks: BORDER_LERP_TICKS,
            },
            BORDER_CURRENT_DIAMETER,
        );

        assert_eq!(
            mutation,
            WorldBorderMutation::SetDiameter {
                target_diameter: BORDER_TARGET_DIAMETER,
                remaining_ticks: BORDER_LERP_TICKS,
            }
        );
    }
}
