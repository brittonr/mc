#[allow(clippy::module_inception)]
mod chunk;
mod egress_cache;
mod entry;
mod layer_impl;
mod messages;
mod storage;
mod systems;
pub(crate) mod targeting;
mod writers;

pub mod loaded;
mod paletted_container;
pub mod unloaded;

pub use chunk::{MAX_HEIGHT, *};
pub use entry::{ChunkEntry, OccupiedChunkEntry, VacantChunkEntry};
pub use loaded::LoadedChunk;
pub use storage::ChunkLayer;
pub use unloaded::UnloadedChunk;
pub use writers::{ExceptWriter, RadiusExceptWriter, RadiusWriter, ViewExceptWriter, ViewWriter};

pub(crate) use entry::{chunk_state_message_plan, ChunkStateMessagePlan};
pub(crate) use messages::{
    global_message_targets_client, local_message_targets_client, ChunkLayerMessages, GlobalMsg,
    LocalMsg,
};
pub(crate) use storage::ChunkLayerInfo;

pub(super) fn build(app: &mut bevy_app::App) {
    systems::build(app);
}
