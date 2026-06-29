use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use valence_server::registry::biome::BiomeId;
use valence_server::registry::BiomeRegistry;
use valence_server::{ChunkPos, Ident};

use crate::{RawChunk, RegionFolder};

use super::model::{
    ChunkFailure, MissingRegionPolicy, PartialLoadPolicy, RawSnapshotChunk, RegionCoord,
    SnapshotLoadError, SnapshotLoadReport, SnapshotPlan, SnapshotPlanInput, StaticWorldSnapshot,
};
use super::planning::{region_for_chunk, validate_snapshot_plan};
use super::validation::normalize_chunk_snapshot;

pub(super) const REGION_DIRECTORY_NAME: &str = "region";
const REGION_FILE_PREFIX: &str = "r";
const REGION_FILE_EXTENSION: &str = "mca";

/// Loads a static snapshot from a dimension folder using the configured shell policies.
pub fn load_static_world_snapshot(
    input: SnapshotPlanInput,
    biomes: &BiomeRegistry,
) -> Result<StaticWorldSnapshot, SnapshotLoadError> {
    load_static_world_snapshot_with_cancel(input, biomes, || false)
}

/// Loads a static snapshot and checks a cancellation boundary between chunks.
pub fn load_static_world_snapshot_with_cancel<F>(
    input: SnapshotPlanInput,
    biomes: &BiomeRegistry,
    mut is_cancelled: F,
) -> Result<StaticWorldSnapshot, SnapshotLoadError>
where
    F: FnMut() -> bool,
{
    let plan = validate_snapshot_plan(input)?;
    let mut report = SnapshotLoadReport::default();
    let mut loaded_chunks = Vec::new();

    discover_required_region_files(&plan, &mut report)?;

    let biome_to_id = biome_to_id_map(biomes);
    let mut region_folder = RegionFolder::new(plan.dimension_root.join(REGION_DIRECTORY_NAME));

    for pos in plan.chunks.iter().copied() {
        if is_cancelled() {
            return Err(SnapshotLoadError::Cancelled {
                loaded_chunks: loaded_chunks.len(),
            });
        }
        if report.missing_regions.contains(&region_for_chunk(pos)) {
            continue;
        }

        let Some(raw_chunk) = region_folder.get_chunk::<String>(pos.x, pos.z)? else {
            report.empty_chunks = report.empty_chunks.saturating_add(1);
            continue;
        };
        let raw_chunk = raw_snapshot_chunk(pos, raw_chunk);

        match normalize_chunk_snapshot(raw_chunk, &plan, &biome_to_id) {
            Ok(chunk) => loaded_chunks.push(chunk),
            Err(error) => match plan.partial_load_policy {
                PartialLoadPolicy::Reject => {
                    return Err(SnapshotLoadError::Chunk { pos, error });
                }
                PartialLoadPolicy::CommitSuccessful => {
                    report.partial = true;
                    report.failed_chunks.push(ChunkFailure {
                        pos,
                        error: error.to_string(),
                    });
                }
            },
        }
    }

    report.loaded_chunks = loaded_chunks.len();
    if !report.missing_regions.is_empty() || !report.failed_chunks.is_empty() {
        report.partial = true;
    }

    Ok(StaticWorldSnapshot {
        plan,
        chunks: loaded_chunks,
        report,
    })
}

pub(super) fn region_file_path(dimension_root: &Path, region: RegionCoord) -> PathBuf {
    dimension_root.join(REGION_DIRECTORY_NAME).join(format!(
        "{REGION_FILE_PREFIX}.{}.{}.{REGION_FILE_EXTENSION}",
        region.x, region.z
    ))
}

pub(super) fn biome_to_id_map(biomes: &BiomeRegistry) -> BTreeMap<Ident<String>, BiomeId> {
    biomes
        .iter()
        .map(|(id, name, _)| (name.to_string_ident(), id))
        .collect()
}

pub(super) fn raw_snapshot_chunk(pos: ChunkPos, raw_chunk: RawChunk<String>) -> RawSnapshotChunk {
    RawSnapshotChunk {
        pos,
        data: raw_chunk.data,
        timestamp: raw_chunk.timestamp,
    }
}

fn discover_required_region_files(
    plan: &SnapshotPlan,
    report: &mut SnapshotLoadReport,
) -> Result<(), SnapshotLoadError> {
    for region in &plan.regions {
        let path = region_file_path(&plan.dimension_root, *region);
        if path.exists() {
            continue;
        }

        match plan.missing_region_policy {
            MissingRegionPolicy::Fail => {
                return Err(SnapshotLoadError::MissingRegionFile {
                    region: *region,
                    path,
                });
            }
            MissingRegionPolicy::Skip => {
                report.missing_regions.push(*region);
            }
        }
    }
    Ok(())
}
