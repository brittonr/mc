//! Static world snapshot loading, validation, and application boundaries.
//!
//! The public API is re-exported from focused submodules so callers can keep
//! using `valence_anvil::snapshot::*` while implementation owners stay split by
//! responsibility.

mod bevy_adapter;
mod cache_policy;
mod filesystem;
mod model;
mod planning;
mod validation;

pub use bevy_adapter::{
    apply_snapshot_to_layer, plan_snapshot_update, validate_layer_for_snapshot,
    SnapshotChunkPresence, SnapshotChunkUpdate, SnapshotUpdateAction, SnapshotUpdatePlan,
};
pub use cache_policy::{
    plan_snapshot_cache, SnapshotCacheAction, SnapshotCacheEntry, SnapshotCachePlanInput,
    SnapshotCachePolicy, SnapshotSourceState,
};
pub use filesystem::{load_static_world_snapshot, load_static_world_snapshot_with_cancel};
pub use model::{
    AsyncReadPolicy, ChunkFailure, ChunkSnapshotSummary, LoadedSnapshotChunk, MemoryMappingPolicy,
    MissingRegionPolicy, PartialLoadPolicy, RawSnapshotChunk, RegionCoord, RegionRange,
    SnapshotAdapterPolicy, SnapshotApplyError, SnapshotApplyReport, SnapshotChunkError,
    SnapshotDimension, SnapshotLayerDescriptor, SnapshotLoadError, SnapshotLoadReport,
    SnapshotNbtError, SnapshotPlan, SnapshotPlanError, SnapshotPlanInput, SnapshotResourceLimits,
    StaticWorldSnapshot,
};
pub use planning::validate_snapshot_plan;
pub use validation::{normalize_chunk_snapshot, summarize_chunk_nbt};

#[cfg(test)]
mod tests;
