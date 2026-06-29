use valence_server::{ChunkLayer, ChunkPos};

use super::model::{
    SnapshotApplyError, SnapshotApplyReport, SnapshotDimension, SnapshotLayerDescriptor,
    StaticWorldSnapshot,
};

/// Presence of one snapshot chunk in the target layer before application.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SnapshotChunkPresence {
    /// Chunk position being planned.
    pub pos: ChunkPos,
    /// True when the target layer already has a chunk at this position.
    pub present: bool,
}

/// Mutation kind selected for one snapshot chunk.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SnapshotUpdateAction {
    /// Insert a chunk into an empty layer position.
    Insert,
    /// Replace a chunk that already exists in the layer.
    Replace,
}

/// Planned mutation for one snapshot chunk.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SnapshotChunkUpdate {
    /// Chunk position being updated.
    pub pos: ChunkPos,
    /// Layer mutation selected for the chunk.
    pub action: SnapshotUpdateAction,
}

/// Pure plan for applying normalized snapshot chunks to a layer.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SnapshotUpdatePlan {
    /// Per-chunk update actions in snapshot chunk order.
    pub updates: Vec<SnapshotChunkUpdate>,
    /// Deterministic summary of the planned updates.
    pub report: SnapshotApplyReport,
}

/// Validates a target layer descriptor before any mutation occurs.
pub fn validate_layer_for_snapshot(
    expected: &SnapshotDimension,
    actual: &SnapshotLayerDescriptor,
) -> Result<(), SnapshotApplyError> {
    if expected.dimension_type_name.as_str() != actual.dimension_type_name.as_str() {
        return Err(SnapshotApplyError::DimensionMismatch {
            reason: "dimension_type_name",
        });
    }
    if expected.min_y != actual.min_y {
        return Err(SnapshotApplyError::DimensionMismatch { reason: "min_y" });
    }
    if expected.height != actual.height {
        return Err(SnapshotApplyError::DimensionMismatch { reason: "height" });
    }
    Ok(())
}

/// Plans layer updates without mutating the layer.
pub fn plan_snapshot_update(presence: &[SnapshotChunkPresence]) -> SnapshotUpdatePlan {
    let mut plan = SnapshotUpdatePlan::default();
    for item in presence {
        let action = if item.present {
            plan.report.replaced_chunks = plan.report.replaced_chunks.saturating_add(1);
            SnapshotUpdateAction::Replace
        } else {
            plan.report.inserted_chunks = plan.report.inserted_chunks.saturating_add(1);
            SnapshotUpdateAction::Insert
        };
        plan.updates.push(SnapshotChunkUpdate {
            pos: item.pos,
            action,
        });
    }
    plan
}

/// Applies a snapshot to a Valence chunk layer after pure descriptor validation.
pub fn apply_snapshot_to_layer(
    layer: &mut ChunkLayer,
    snapshot: StaticWorldSnapshot,
) -> Result<SnapshotApplyReport, SnapshotApplyError> {
    let descriptor = SnapshotLayerDescriptor {
        dimension_type_name: layer.dimension_type_name().to_string_ident(),
        min_y: layer.min_y(),
        height: layer.height(),
    };
    validate_layer_for_snapshot(&snapshot.plan.dimension, &descriptor)?;

    let presence: Vec<_> = snapshot
        .chunks
        .iter()
        .map(|chunk| SnapshotChunkPresence {
            pos: chunk.pos,
            present: layer.chunk(chunk.pos).is_some(),
        })
        .collect();
    let plan = plan_snapshot_update(&presence);

    for (loaded, update) in snapshot.chunks.into_iter().zip(plan.updates.iter()) {
        debug_assert_eq!(loaded.pos, update.pos);
        let replaced = layer.insert_chunk(loaded.pos, loaded.chunk).is_some();
        debug_assert_eq!(
            replaced,
            matches!(update.action, SnapshotUpdateAction::Replace)
        );
    }

    Ok(plan.report)
}
