#![doc = include_str!("../README.md")]

mod components;
pub mod event;
mod systems;

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
pub use bevy_hierarchy;
pub use components::*;
type Client = valence_server::client::Client;

pub struct AdvancementPlugin;

#[derive(SystemSet, Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub struct WriteAdvancementPacketToClientsSet;

#[derive(SystemSet, Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub struct WriteAdvancementToCacheSet;

impl Plugin for AdvancementPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.add_plugins(bevy_hierarchy::HierarchyPlugin)
            .configure_sets(
                PostUpdate,
                (
                    WriteAdvancementPacketToClientsSet
                        .before(valence_server::client::FlushPacketsSet),
                    WriteAdvancementToCacheSet.before(WriteAdvancementPacketToClientsSet),
                ),
            )
            .add_event::<event::AdvancementTabChangeEvent>()
            .add_systems(
                PreUpdate,
                (
                    add_advancement_update_component_to_new_clients
                        .after(valence_server::client::SpawnClientsSet),
                    event::handle_advancement_tab_change,
                ),
            )
            .add_systems(
                PostUpdate,
                (
                    systems::update_advancement_cached_bytes.in_set(WriteAdvancementToCacheSet),
                    systems::send_advancement_update_packet
                        .in_set(WriteAdvancementPacketToClientsSet),
                ),
            );
    }
}

/// Components for advancement that are required
/// Optional components:
/// [`AdvancementDisplay`]
/// [`Parent`] - parent advancement
#[derive(Bundle)]
pub struct AdvancementBundle {
    pub advancement: Advancement,
    pub requirements: AdvancementRequirements,
    pub cached_bytes: AdvancementCachedBytes,
}

fn add_advancement_update_component_to_new_clients(
    mut commands: Commands,
    query: Query<Entity, Added<Client>>,
) {
    for client in query.iter() {
        commands
            .entity(client)
            .insert(AdvancementClientUpdate::default());
    }
}
