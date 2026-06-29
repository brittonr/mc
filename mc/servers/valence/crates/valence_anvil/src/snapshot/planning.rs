use std::collections::BTreeSet;

use valence_server::layer::chunk::MAX_HEIGHT;
use valence_server::ChunkPos;

use super::model::{
    RegionCoord, RegionRange, SnapshotDimension, SnapshotPlan, SnapshotPlanError,
    SnapshotPlanInput, SnapshotResourceLimits,
};

pub(super) const REGION_CHUNK_AXIS: i32 = 32;
pub(super) const REGION_CHUNK_AXIS_USIZE: usize = 32;
pub(super) const REGION_CHUNK_COUNT: usize = REGION_CHUNK_AXIS_USIZE * REGION_CHUNK_AXIS_USIZE;
pub(super) const SECTION_HEIGHT_BLOCKS: i32 = 16;
pub(super) const SECTION_HEIGHT_BLOCKS_U32: u32 = 16;
pub(super) const DEFAULT_MAX_REGION_COUNT: usize = 16;
pub(super) const DEFAULT_MAX_CHUNK_COUNT: usize = DEFAULT_MAX_REGION_COUNT * REGION_CHUNK_COUNT;
pub(super) const MAX_SECTIONS_PER_CHUNK: usize = (MAX_HEIGHT / SECTION_HEIGHT_BLOCKS_U32) as usize;

/// Builds a pure validated snapshot plan from typed inputs.
pub fn validate_snapshot_plan(input: SnapshotPlanInput) -> Result<SnapshotPlan, SnapshotPlanError> {
    validate_plan_header(&input)?;

    let mut regions = BTreeSet::new();
    let mut chunks = BTreeSet::new();

    collect_explicit_chunks(&input.chunk_positions, &mut regions, &mut chunks);
    collect_region_ranges(&input.region_ranges, &mut regions, &mut chunks)?;

    if chunks.is_empty() {
        return Err(SnapshotPlanError::EmptySelection);
    }

    check_limit(
        "max_regions",
        regions.len(),
        input.resource_limits.max_regions,
    )?;
    check_limit("max_chunks", chunks.len(), input.resource_limits.max_chunks)?;

    Ok(SnapshotPlan {
        dimension_root: input.dimension_root,
        dimension: input.dimension,
        regions: regions.into_iter().collect(),
        chunks: chunks.into_iter().collect(),
        resource_limits: input.resource_limits,
        allowed_biomes: input.allowed_biomes,
        missing_region_policy: input.missing_region_policy,
        partial_load_policy: input.partial_load_policy,
        adapter_policy: input.adapter_policy,
    })
}

pub(super) fn region_for_chunk(pos: ChunkPos) -> RegionCoord {
    RegionCoord::new(
        pos.x.div_euclid(REGION_CHUNK_AXIS),
        pos.z.div_euclid(REGION_CHUNK_AXIS),
    )
}

fn validate_plan_header(input: &SnapshotPlanInput) -> Result<(), SnapshotPlanError> {
    if input.dimension_root.as_os_str().is_empty() {
        return Err(SnapshotPlanError::EmptyDimensionRoot);
    }
    if input.region_ranges.is_empty() && input.chunk_positions.is_empty() {
        return Err(SnapshotPlanError::EmptySelection);
    }
    if input.allowed_biomes.is_empty() {
        return Err(SnapshotPlanError::EmptyAllowedBiomes);
    }
    validate_resource_limits(&input.resource_limits)?;
    validate_dimension(&input.dimension)
}

fn validate_resource_limits(limits: &SnapshotResourceLimits) -> Result<(), SnapshotPlanError> {
    if limits.max_regions == 0 {
        return Err(SnapshotPlanError::ZeroResourceLimit {
            limit_name: "max_regions",
        });
    }
    if limits.max_chunks == 0 {
        return Err(SnapshotPlanError::ZeroResourceLimit {
            limit_name: "max_chunks",
        });
    }
    if limits.max_sections_per_chunk == 0 {
        return Err(SnapshotPlanError::ZeroResourceLimit {
            limit_name: "max_sections_per_chunk",
        });
    }
    Ok(())
}

fn validate_dimension(dimension: &SnapshotDimension) -> Result<(), SnapshotPlanError> {
    if dimension.height == 0
        || dimension.height > MAX_HEIGHT
        || !dimension.height.is_multiple_of(SECTION_HEIGHT_BLOCKS_U32)
    {
        return Err(SnapshotPlanError::InvalidDimensionHeight {
            height: dimension.height,
        });
    }
    if dimension.min_y % SECTION_HEIGHT_BLOCKS != 0 {
        return Err(SnapshotPlanError::InvalidDimensionMinY {
            min_y: dimension.min_y,
        });
    }
    Ok(())
}

fn collect_explicit_chunks(
    chunk_positions: &[ChunkPos],
    regions: &mut BTreeSet<RegionCoord>,
    chunks: &mut BTreeSet<ChunkPos>,
) {
    for pos in chunk_positions.iter().copied() {
        regions.insert(region_for_chunk(pos));
        chunks.insert(pos);
    }
}

fn collect_region_ranges(
    ranges: &[RegionRange],
    regions: &mut BTreeSet<RegionCoord>,
    chunks: &mut BTreeSet<ChunkPos>,
) -> Result<(), SnapshotPlanError> {
    for range in ranges {
        if range.min_x > range.max_x || range.min_z > range.max_z {
            return Err(SnapshotPlanError::InvertedRegionRange);
        }
        for region_x in range.min_x..=range.max_x {
            for region_z in range.min_z..=range.max_z {
                let region = RegionCoord::new(region_x, region_z);
                regions.insert(region);
                collect_region_chunks(region, chunks)?;
            }
        }
    }
    Ok(())
}

fn collect_region_chunks(
    region: RegionCoord,
    chunks: &mut BTreeSet<ChunkPos>,
) -> Result<(), SnapshotPlanError> {
    let Some(region_chunk_x) = region.x.checked_mul(REGION_CHUNK_AXIS) else {
        return Err(SnapshotPlanError::CoordinateOverflow);
    };
    let Some(region_chunk_z) = region.z.checked_mul(REGION_CHUNK_AXIS) else {
        return Err(SnapshotPlanError::CoordinateOverflow);
    };

    for local_x in 0..REGION_CHUNK_AXIS {
        for local_z in 0..REGION_CHUNK_AXIS {
            let Some(chunk_x) = region_chunk_x.checked_add(local_x) else {
                return Err(SnapshotPlanError::CoordinateOverflow);
            };
            let Some(chunk_z) = region_chunk_z.checked_add(local_z) else {
                return Err(SnapshotPlanError::CoordinateOverflow);
            };
            chunks.insert(ChunkPos::new(chunk_x, chunk_z));
        }
    }
    Ok(())
}

fn check_limit(
    limit_name: &'static str,
    actual: usize,
    limit: usize,
) -> Result<(), SnapshotPlanError> {
    if actual > limit {
        return Err(SnapshotPlanError::ResourceLimitExceeded {
            limit_name,
            actual,
            limit,
        });
    }
    Ok(())
}
