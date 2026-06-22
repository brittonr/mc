#![doc = include_str!("../README.md")]

mod components;
mod systems;

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
pub use components::*;

/// Provides all necessary systems to manage scoreboards.
pub struct ScoreboardPlugin;

impl Plugin for ScoreboardPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            PostUpdate,
            ScoreboardSet.before(valence_server::layer::UpdateLayersPreClientSet),
        );

        app.add_systems(
            PostUpdate,
            (
                systems::create_or_update_objectives,
                systems::display_objectives.after(systems::create_or_update_objectives),
            )
                .in_set(ScoreboardSet),
        )
        .add_systems(
            PostUpdate,
            systems::remove_despawned_objectives.in_set(ScoreboardSet),
        )
        .add_systems(
            PostUpdate,
            systems::handle_new_clients.in_set(ScoreboardSet),
        )
        .add_systems(
            PostUpdate,
            systems::update_scores
                .after(systems::create_or_update_objectives)
                .after(systems::handle_new_clients)
                .in_set(ScoreboardSet),
        );
    }
}

#[derive(SystemSet, Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ScoreboardSet;
