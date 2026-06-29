use bevy_app::prelude::*;
use bevy_ecs::prelude::*;

use crate::layer::{UpdateLayersPostClientSet, UpdateLayersPreClientSet};

use super::targeting::{chunk_layer_update_plan, ChunkLayerUpdatePhase, ChunkLayerUpdateStep};
use super::ChunkLayer;

pub(super) fn build(app: &mut App) {
    app.add_systems(
        PostUpdate,
        (
            update_chunk_layers_pre_client.in_set(UpdateLayersPreClientSet),
            update_chunk_layers_post_client.in_set(UpdateLayersPostClientSet),
        ),
    );
}

fn update_chunk_layers_pre_client(mut layers: Query<&mut ChunkLayer>) {
    for layer in &mut layers {
        let layer = layer.into_inner();

        for step in chunk_layer_update_plan(ChunkLayerUpdatePhase::PreClient) {
            apply_update_step(layer, *step);
        }
    }
}

fn update_chunk_layers_post_client(mut layers: Query<&mut ChunkLayer>) {
    for layer in &mut layers {
        let layer = layer.into_inner();

        for step in chunk_layer_update_plan(ChunkLayerUpdatePhase::PostClient) {
            apply_update_step(layer, *step);
        }
    }
}

fn apply_update_step(layer: &mut ChunkLayer, step: ChunkLayerUpdateStep) {
    match step {
        ChunkLayerUpdateStep::UpdateLoadedChunks => update_loaded_chunks(layer),
        ChunkLayerUpdateStep::ReadyMessages => layer.messages.ready(),
        ChunkLayerUpdateStep::UnreadyMessages => layer.messages.unready(),
    }
}

fn update_loaded_chunks(layer: &mut ChunkLayer) {
    for (&pos, chunk) in &mut layer.chunks {
        chunk.update_pre_client(pos, &layer.info, &mut layer.messages);
    }
}
