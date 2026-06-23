use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use thiserror::Error;
use valence_nbt::{Compound, List, Value};
use valence_server::layer::chunk::{Chunk, UnloadedChunk, MAX_HEIGHT};
use valence_server::registry::biome::BiomeId;
use valence_server::registry::BiomeRegistry;
use valence_server::{ChunkLayer, ChunkPos, Ident};

use crate::parsing::{parse_chunk, ParseChunkError};
use crate::{RawChunk, RegionError, RegionFolder};

const REGION_DIRECTORY_NAME: &str = "region";
const REGION_FILE_PREFIX: &str = "r";
const REGION_FILE_EXTENSION: &str = "mca";
const REGION_CHUNK_AXIS: i32 = 32;
const REGION_CHUNK_AXIS_USIZE: usize = 32;
const REGION_CHUNK_COUNT: usize = REGION_CHUNK_AXIS_USIZE * REGION_CHUNK_AXIS_USIZE;
const SECTION_HEIGHT_BLOCKS: i32 = 16;
const SECTION_HEIGHT_BLOCKS_U32: u32 = 16;
const DEFAULT_MAX_REGION_COUNT: usize = 16;
const DEFAULT_MAX_CHUNK_COUNT: usize = DEFAULT_MAX_REGION_COUNT * REGION_CHUNK_COUNT;
const MAX_SECTIONS_PER_CHUNK: usize = (MAX_HEIGHT / SECTION_HEIGHT_BLOCKS_U32) as usize;

/// Typed input used to build a static world snapshot load plan.
#[derive(Clone, Debug)]
pub struct SnapshotPlanInput {
    /// Dimension root containing the `region` directory.
    pub dimension_root: PathBuf,
    /// Dimension expected by the target layer.
    pub dimension: SnapshotDimension,
    /// Explicit region ranges selected for loading.
    pub region_ranges: Vec<RegionRange>,
    /// Explicit chunk positions selected for loading.
    pub chunk_positions: Vec<ChunkPos>,
    /// Resource limits checked before filesystem access.
    pub resource_limits: SnapshotResourceLimits,
    /// Biome identifiers accepted by this static snapshot.
    pub allowed_biomes: BTreeSet<Ident<String>>,
    /// Missing region file policy.
    pub missing_region_policy: MissingRegionPolicy,
    /// Partial load policy for chunk-level failures.
    pub partial_load_policy: PartialLoadPolicy,
    /// Adapter policy selected by the caller.
    pub adapter_policy: SnapshotAdapterPolicy,
}

impl SnapshotPlanInput {
    /// Creates a new snapshot plan input with conservative defaults.
    pub fn new(dimension_root: PathBuf, dimension: SnapshotDimension) -> Self {
        Self {
            dimension_root,
            dimension,
            region_ranges: Vec::new(),
            chunk_positions: Vec::new(),
            resource_limits: SnapshotResourceLimits::default(),
            allowed_biomes: BTreeSet::new(),
            missing_region_policy: MissingRegionPolicy::Fail,
            partial_load_policy: PartialLoadPolicy::Reject,
            adapter_policy: SnapshotAdapterPolicy::default(),
        }
    }
}

/// Dimension bounds expected for client-visible snapshot chunks.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SnapshotDimension {
    /// Valence dimension type name for the target layer.
    pub dimension_type_name: Ident<String>,
    /// Lowest block Y accepted by the target layer.
    pub min_y: i32,
    /// Total dimension height in blocks.
    pub height: u32,
}

/// Inclusive region range selected for a snapshot load.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RegionRange {
    /// Minimum region X coordinate.
    pub min_x: i32,
    /// Maximum region X coordinate.
    pub max_x: i32,
    /// Minimum region Z coordinate.
    pub min_z: i32,
    /// Maximum region Z coordinate.
    pub max_z: i32,
}

impl RegionRange {
    /// Creates an inclusive region range.
    pub const fn new(min_x: i32, max_x: i32, min_z: i32, max_z: i32) -> Self {
        Self {
            min_x,
            max_x,
            min_z,
            max_z,
        }
    }
}

/// Resource limits checked before any filesystem adapter runs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SnapshotResourceLimits {
    /// Maximum selected region files.
    pub max_regions: usize,
    /// Maximum selected chunks after expanding region ranges.
    pub max_chunks: usize,
    /// Maximum section count accepted for one chunk.
    pub max_sections_per_chunk: usize,
}

impl Default for SnapshotResourceLimits {
    fn default() -> Self {
        Self {
            max_regions: DEFAULT_MAX_REGION_COUNT,
            max_chunks: DEFAULT_MAX_CHUNK_COUNT,
            max_sections_per_chunk: MAX_SECTIONS_PER_CHUNK,
        }
    }
}

/// Policy for absent region files selected by a plan.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MissingRegionPolicy {
    /// Fail before chunk loading begins.
    Fail,
    /// Skip the missing region and record it in the load report.
    Skip,
}

/// Policy for chunk-level parse/validation failures after region discovery.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PartialLoadPolicy {
    /// Reject the whole snapshot when any selected chunk fails.
    Reject,
    /// Keep successfully normalized chunks and report failures.
    CommitSuccessful,
}

/// Memory mapping policy for snapshot adapters.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MemoryMappingPolicy {
    /// Do not require memory mapping.
    Disabled,
    /// Allow an adapter to memory-map region bytes when it owns the safety audit.
    AdapterOptional,
}

/// Async read policy for snapshot adapters.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AsyncReadPolicy {
    /// Run region reads in the caller's blocking shell.
    BlockingShell,
    /// Permit a shell adapter to spawn bounded async work.
    SpawnBounded,
}

/// Adapter boundaries selected for filesystem and scheduling shells.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SnapshotAdapterPolicy {
    /// Memory mapping policy for region access.
    pub memory_mapping: MemoryMappingPolicy,
    /// Async read policy for region access.
    pub async_reads: AsyncReadPolicy,
}

impl Default for SnapshotAdapterPolicy {
    fn default() -> Self {
        Self {
            memory_mapping: MemoryMappingPolicy::Disabled,
            async_reads: AsyncReadPolicy::BlockingShell,
        }
    }
}

/// Validated, deterministic snapshot load plan.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SnapshotPlan {
    /// Dimension root containing the `region` directory.
    pub dimension_root: PathBuf,
    /// Target dimension bounds.
    pub dimension: SnapshotDimension,
    /// Region files required by this load.
    pub regions: Vec<RegionCoord>,
    /// Chunk positions selected by this load.
    pub chunks: Vec<ChunkPos>,
    /// Resource limits copied from the input.
    pub resource_limits: SnapshotResourceLimits,
    /// Biome identifiers accepted by this static snapshot.
    pub allowed_biomes: BTreeSet<Ident<String>>,
    /// Missing region file policy.
    pub missing_region_policy: MissingRegionPolicy,
    /// Partial load policy.
    pub partial_load_policy: PartialLoadPolicy,
    /// Adapter policy.
    pub adapter_policy: SnapshotAdapterPolicy,
}

/// Region file coordinate.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RegionCoord {
    /// Region X coordinate.
    pub x: i32,
    /// Region Z coordinate.
    pub z: i32,
}

impl RegionCoord {
    /// Creates a new region coordinate.
    pub const fn new(x: i32, z: i32) -> Self {
        Self { x, z }
    }
}

/// Plan validation diagnostic.
#[derive(Debug, Error, PartialEq, Eq)]
#[non_exhaustive]
pub enum SnapshotPlanError {
    /// The input path was empty.
    #[error("dimension root path is empty")]
    EmptyDimensionRoot,
    /// No region or chunk selection was provided.
    #[error("snapshot plan must select at least one region or chunk")]
    EmptySelection,
    /// Biome validation cannot run without an allow-list.
    #[error("snapshot plan must provide at least one allowed biome")]
    EmptyAllowedBiomes,
    /// A region range is inverted.
    #[error("region range has min greater than max")]
    InvertedRegionRange,
    /// A resource limit was zero.
    #[error("resource limit {limit_name} must be greater than zero")]
    ZeroResourceLimit { limit_name: &'static str },
    /// Dimension height is invalid.
    #[error("dimension height {height} is invalid")]
    InvalidDimensionHeight { height: u32 },
    /// Dimension minimum Y is invalid.
    #[error("dimension min_y {min_y} is invalid")]
    InvalidDimensionMinY { min_y: i32 },
    /// Region or chunk expansion overflowed.
    #[error("snapshot selection overflowed coordinate arithmetic")]
    CoordinateOverflow,
    /// A resource limit was exceeded.
    #[error("resource limit {limit_name} exceeded: actual {actual}, limit {limit}")]
    ResourceLimitExceeded {
        limit_name: &'static str,
        actual: usize,
        limit: usize,
    },
}

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

/// Raw chunk data presented to the pure snapshot normalization core.
#[derive(Debug)]
pub struct RawSnapshotChunk {
    /// Chunk position.
    pub pos: ChunkPos,
    /// Chunk NBT payload.
    pub data: Compound,
    /// Region timestamp in seconds since the epoch.
    pub timestamp: u32,
}

/// Normalized chunk ready for layer insertion by a shell adapter.
#[derive(Debug)]
pub struct LoadedSnapshotChunk {
    /// Chunk position.
    pub pos: ChunkPos,
    /// Unloaded chunk payload.
    pub chunk: UnloadedChunk,
    /// Region timestamp in seconds since the epoch.
    pub timestamp: u32,
    /// Pure validation summary.
    pub summary: ChunkSnapshotSummary,
}

/// Pure summary of a normalized chunk.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChunkSnapshotSummary {
    /// Chunk position.
    pub pos: ChunkPos,
    /// Number of NBT sections observed.
    pub section_count: usize,
    /// Unique biome identifiers observed in the chunk.
    pub biomes: BTreeSet<Ident<String>>,
    /// Minimum section Y observed in NBT.
    pub min_section_y: i32,
    /// Maximum section Y observed in NBT.
    pub max_section_y: i32,
}

/// Chunk normalization diagnostic.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum SnapshotChunkError {
    /// The NBT shape was not sufficient for snapshot validation.
    #[error("snapshot NBT error: {0}")]
    Nbt(#[from] SnapshotNbtError),
    /// Valence Anvil parsing failed.
    #[error("chunk parse error: {0}")]
    Parse(#[from] ParseChunkError),
    /// Chunk sections do not fit target dimension bounds.
    #[error(
        "chunk dimension mismatch at {pos:?}: section_y {section_y} is outside min_y {min_y} height {height}"
    )]
    DimensionMismatch {
        /// Chunk position.
        pos: ChunkPos,
        /// Section Y that violated the bounds.
        section_y: i32,
        /// Expected minimum block Y.
        min_y: i32,
        /// Expected height.
        height: u32,
    },
    /// The parsed chunk height does not match the target layer height.
    #[error(
        "chunk height mismatch at {pos:?}: actual {actual_height}, expected {expected_height}"
    )]
    HeightMismatch {
        /// Chunk position.
        pos: ChunkPos,
        /// Parsed chunk height.
        actual_height: u32,
        /// Expected chunk height.
        expected_height: u32,
    },
    /// A chunk biome was not in the allow-list.
    #[error("biome {biome} at {pos:?} is not allowed by the snapshot plan")]
    BiomeMismatch {
        /// Chunk position.
        pos: ChunkPos,
        /// Rejected biome identifier.
        biome: Ident<String>,
    },
    /// A chunk exceeded the section count limit.
    #[error("chunk section limit exceeded at {pos:?}: actual {actual}, limit {limit}")]
    SectionLimitExceeded {
        /// Chunk position.
        pos: ChunkPos,
        /// Actual section count.
        actual: usize,
        /// Section count limit.
        limit: usize,
    },
}

/// NBT summary diagnostic.
#[derive(Debug, Error, PartialEq, Eq)]
#[non_exhaustive]
pub enum SnapshotNbtError {
    /// Required sections list is missing.
    #[error("missing chunk sections")]
    MissingSections,
    /// Section list is empty.
    #[error("empty chunk sections")]
    EmptySections,
    /// Section Y is missing.
    #[error("missing chunk section Y")]
    MissingSectionY,
    /// Biome compound is missing.
    #[error("missing biomes")]
    MissingBiomes,
    /// Biome palette is missing.
    #[error("missing biome palette")]
    MissingBiomePalette,
    /// Biome identifier is invalid.
    #[error("biome name is not a valid resource identifier")]
    BadBiomeName,
}

/// Normalizes one raw snapshot chunk without reading files or mutating layers.
pub fn normalize_chunk_snapshot(
    raw_chunk: RawSnapshotChunk,
    plan: &SnapshotPlan,
    biome_to_id: &BTreeMap<Ident<String>, BiomeId>,
) -> Result<LoadedSnapshotChunk, SnapshotChunkError> {
    let summary = summarize_chunk_nbt(raw_chunk.pos, &raw_chunk.data)?;
    validate_chunk_summary(&summary, plan)?;

    let chunk = parse_chunk(raw_chunk.data, biome_to_id)?;
    if chunk.height() != plan.dimension.height {
        return Err(SnapshotChunkError::HeightMismatch {
            pos: raw_chunk.pos,
            actual_height: chunk.height(),
            expected_height: plan.dimension.height,
        });
    }

    Ok(LoadedSnapshotChunk {
        pos: raw_chunk.pos,
        chunk,
        timestamp: raw_chunk.timestamp,
        summary,
    })
}

/// Summarizes chunk NBT for pure validation.
pub fn summarize_chunk_nbt(
    pos: ChunkPos,
    nbt: &Compound,
) -> Result<ChunkSnapshotSummary, SnapshotNbtError> {
    let Some(Value::List(List::Compound(sections))) = nbt.get("sections") else {
        return Err(SnapshotNbtError::MissingSections);
    };
    if sections.is_empty() {
        return Err(SnapshotNbtError::EmptySections);
    }

    let mut biomes = BTreeSet::new();
    let mut min_section_y = i32::MAX;
    let mut max_section_y = i32::MIN;

    for section in sections {
        let Some(Value::Byte(section_y)) = section.get("Y") else {
            return Err(SnapshotNbtError::MissingSectionY);
        };
        let section_y = i32::from(*section_y);
        min_section_y = min_section_y.min(section_y);
        max_section_y = max_section_y.max(section_y);

        let Some(Value::Compound(section_biomes)) = section.get("biomes") else {
            return Err(SnapshotNbtError::MissingBiomes);
        };
        let Some(Value::List(List::String(palette))) = section_biomes.get("palette") else {
            return Err(SnapshotNbtError::MissingBiomePalette);
        };
        for biome in palette {
            let Ok(ident) = Ident::<Cow<str>>::new(biome.as_str()) else {
                return Err(SnapshotNbtError::BadBiomeName);
            };
            biomes.insert(ident.to_string_ident());
        }
    }

    Ok(ChunkSnapshotSummary {
        pos,
        section_count: sections.len(),
        biomes,
        min_section_y,
        max_section_y,
    })
}

/// Filesystem load report for a static world snapshot.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SnapshotLoadReport {
    /// Successfully normalized chunks.
    pub loaded_chunks: usize,
    /// Selected chunks absent from present region files.
    pub empty_chunks: usize,
    /// Missing region files skipped by policy.
    pub missing_regions: Vec<RegionCoord>,
    /// Chunk errors retained when partial loads are enabled.
    pub failed_chunks: Vec<ChunkFailure>,
    /// True when the result intentionally contains only a subset of the selection.
    pub partial: bool,
}

/// Chunk failure retained in a partial snapshot load report.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChunkFailure {
    /// Chunk position that failed.
    pub pos: ChunkPos,
    /// Deterministic diagnostic text.
    pub error: String,
}

/// Loaded static snapshot returned by filesystem shell adapters.
#[derive(Debug)]
pub struct StaticWorldSnapshot {
    /// Validated plan used for the load.
    pub plan: SnapshotPlan,
    /// Loaded chunks.
    pub chunks: Vec<LoadedSnapshotChunk>,
    /// Load report.
    pub report: SnapshotLoadReport,
}

/// Filesystem adapter diagnostic.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum SnapshotLoadError {
    /// Plan validation failed.
    #[error("snapshot plan error: {0}")]
    Plan(#[from] SnapshotPlanError),
    /// A required region file is missing.
    #[error("missing required region file {path}")]
    MissingRegionFile {
        /// Missing region coordinate.
        region: RegionCoord,
        /// Missing region path.
        path: PathBuf,
    },
    /// Region reading failed.
    #[error("region error: {0}")]
    Region(#[from] RegionError),
    /// Chunk normalization failed.
    #[error("chunk {pos:?} failed snapshot validation: {error}")]
    Chunk {
        /// Chunk position.
        pos: ChunkPos,
        /// Normalization error.
        error: SnapshotChunkError,
    },
    /// Caller cancelled the load before it completed.
    #[error("snapshot load cancelled after {loaded_chunks} chunks")]
    Cancelled {
        /// Count of chunks normalized before cancellation.
        loaded_chunks: usize,
    },
}

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

/// Descriptor used by the pure layer validation core.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SnapshotLayerDescriptor {
    /// Dimension type name.
    pub dimension_type_name: Ident<String>,
    /// Layer minimum Y.
    pub min_y: i32,
    /// Layer height.
    pub height: u32,
}

/// Layer application diagnostic.
#[derive(Debug, Error, PartialEq, Eq)]
#[non_exhaustive]
pub enum SnapshotApplyError {
    /// Layer dimension does not match the snapshot plan.
    #[error("layer dimension mismatch: {reason}")]
    DimensionMismatch {
        /// Deterministic reason.
        reason: &'static str,
    },
}

/// Report returned after applying a snapshot to a layer.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SnapshotApplyReport {
    /// Inserted chunks that were not already present.
    pub inserted_chunks: usize,
    /// Inserted chunks that replaced an existing chunk.
    pub replaced_chunks: usize,
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

    let mut report = SnapshotApplyReport::default();
    for loaded in snapshot.chunks {
        if layer.insert_chunk(loaded.pos, loaded.chunk).is_some() {
            report.replaced_chunks = report.replaced_chunks.saturating_add(1);
        } else {
            report.inserted_chunks = report.inserted_chunks.saturating_add(1);
        }
    }
    Ok(report)
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

fn validate_chunk_summary(
    summary: &ChunkSnapshotSummary,
    plan: &SnapshotPlan,
) -> Result<(), SnapshotChunkError> {
    if summary.section_count > plan.resource_limits.max_sections_per_chunk {
        return Err(SnapshotChunkError::SectionLimitExceeded {
            pos: summary.pos,
            actual: summary.section_count,
            limit: plan.resource_limits.max_sections_per_chunk,
        });
    }

    for section_y in [summary.min_section_y, summary.max_section_y] {
        if !dimension_contains_section(&plan.dimension, section_y) {
            return Err(SnapshotChunkError::DimensionMismatch {
                pos: summary.pos,
                section_y,
                min_y: plan.dimension.min_y,
                height: plan.dimension.height,
            });
        }
    }

    for biome in &summary.biomes {
        if !plan.allowed_biomes.contains(biome) {
            return Err(SnapshotChunkError::BiomeMismatch {
                pos: summary.pos,
                biome: biome.clone(),
            });
        }
    }

    Ok(())
}

fn dimension_contains_section(dimension: &SnapshotDimension, section_y: i32) -> bool {
    let Some(section_min_y) = section_y.checked_mul(SECTION_HEIGHT_BLOCKS) else {
        return false;
    };
    let Some(dimension_end_y) = dimension
        .min_y
        .checked_add(i32::try_from(dimension.height).unwrap_or(i32::MAX))
    else {
        return false;
    };
    section_min_y >= dimension.min_y && section_min_y < dimension_end_y
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

fn region_file_path(dimension_root: &Path, region: RegionCoord) -> PathBuf {
    dimension_root.join(REGION_DIRECTORY_NAME).join(format!(
        "{REGION_FILE_PREFIX}.{}.{}.{REGION_FILE_EXTENSION}",
        region.x, region.z
    ))
}

fn region_for_chunk(pos: ChunkPos) -> RegionCoord {
    RegionCoord::new(
        pos.x.div_euclid(REGION_CHUNK_AXIS),
        pos.z.div_euclid(REGION_CHUNK_AXIS),
    )
}

fn biome_to_id_map(biomes: &BiomeRegistry) -> BTreeMap<Ident<String>, BiomeId> {
    biomes
        .iter()
        .map(|(id, name, _)| (name.to_string_ident(), id))
        .collect()
}

fn raw_snapshot_chunk(pos: ChunkPos, raw_chunk: RawChunk<String>) -> RawSnapshotChunk {
    RawSnapshotChunk {
        pos,
        data: raw_chunk.data,
        timestamp: raw_chunk.timestamp,
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;
    use valence_nbt::{compound, List};
    use valence_server::registry::biome::ident;
    use valence_server::registry::biome::{Biome, BiomeRegistry};

    use super::*;

    const TEST_HEIGHT: u32 = 16;
    const TEST_MIN_Y: i32 = 0;
    const TEST_TIMESTAMP: u32 = 0;
    const TEST_REGION_HEADER_BYTES: usize = 8192;

    #[test]
    fn plan_validation_accepts_explicit_chunk() {
        let input =
            plan_input_with_chunk(tempdir().unwrap().path().to_path_buf(), ChunkPos::new(0, 0));

        let plan = validate_snapshot_plan(input).unwrap();

        assert_eq!(plan.regions, vec![RegionCoord::new(0, 0)]);
        assert_eq!(plan.chunks, vec![ChunkPos::new(0, 0)]);
    }

    #[test]
    fn plan_validation_rejects_missing_biome_allow_list() {
        let mut input =
            plan_input_with_chunk(tempdir().unwrap().path().to_path_buf(), ChunkPos::new(0, 0));
        input.allowed_biomes.clear();

        let err = validate_snapshot_plan(input).unwrap_err();

        assert_eq!(err, SnapshotPlanError::EmptyAllowedBiomes);
    }

    #[test]
    fn valid_region_fixture_loads_one_chunk() {
        let dir = tempdir().unwrap();
        write_valid_chunk_region(dir.path(), ChunkPos::new(0, 0));
        let input = plan_input_with_chunk(dir.path().to_path_buf(), ChunkPos::new(0, 0));
        let biomes = biome_registry();

        let snapshot = load_static_world_snapshot(input, &biomes).unwrap();

        assert_eq!(snapshot.report.loaded_chunks, 1);
        assert_eq!(snapshot.report.empty_chunks, 0);
        assert!(!snapshot.report.partial);
        assert_eq!(snapshot.chunks[0].summary.section_count, 1);
    }

    #[test]
    fn missing_region_file_fails_by_policy() {
        let dir = tempdir().unwrap();
        let input = plan_input_with_chunk(dir.path().to_path_buf(), ChunkPos::new(0, 0));
        let biomes = biome_registry();

        let err = load_static_world_snapshot(input, &biomes).unwrap_err();

        assert!(matches!(err, SnapshotLoadError::MissingRegionFile { .. }));
    }

    #[test]
    fn corrupt_nbt_reports_parse_diagnostic() {
        let plan = validate_snapshot_plan(plan_input_with_chunk(
            tempdir().unwrap().path().to_path_buf(),
            ChunkPos::new(0, 0),
        ))
        .unwrap();
        let raw = RawSnapshotChunk {
            pos: ChunkPos::new(0, 0),
            data: compound! {
                "sections" => List::Compound(vec![compound! {
                    "Y" => 0_i8,
                    "biomes" => compound! {
                        "palette" => List::String(vec!["minecraft:plains".to_owned()]),
                    },
                }]),
                "block_entities" => List::Compound(Vec::new()),
            },
            timestamp: TEST_TIMESTAMP,
        };
        let biome_to_id = biome_to_id_map(&biome_registry());

        let err = normalize_chunk_snapshot(raw, &plan, &biome_to_id).unwrap_err();

        assert!(matches!(
            err,
            SnapshotChunkError::Parse(ParseChunkError::MissingBlockStates)
        ));
    }

    #[test]
    fn out_of_range_section_rejects_chunk() {
        let plan = validate_snapshot_plan(plan_input_with_chunk(
            tempdir().unwrap().path().to_path_buf(),
            ChunkPos::new(0, 0),
        ))
        .unwrap();
        let raw = RawSnapshotChunk {
            pos: ChunkPos::new(0, 0),
            data: valid_chunk_nbt_with_section_y(1),
            timestamp: TEST_TIMESTAMP,
        };
        let biome_to_id = biome_to_id_map(&biome_registry());

        let err = normalize_chunk_snapshot(raw, &plan, &biome_to_id).unwrap_err();

        assert!(matches!(err, SnapshotChunkError::DimensionMismatch { .. }));
    }

    #[test]
    fn layer_dimension_mismatch_is_pure_and_deterministic() {
        let expected = snapshot_dimension();
        let actual = SnapshotLayerDescriptor {
            dimension_type_name: ident!("minecraft:the_nether").to_string_ident(),
            min_y: TEST_MIN_Y,
            height: TEST_HEIGHT,
        };

        let err = validate_layer_for_snapshot(&expected, &actual).unwrap_err();

        assert_eq!(
            err,
            SnapshotApplyError::DimensionMismatch {
                reason: "dimension_type_name",
            }
        );
    }

    #[test]
    fn biome_mismatch_rejects_chunk() {
        let mut input =
            plan_input_with_chunk(tempdir().unwrap().path().to_path_buf(), ChunkPos::new(0, 0));
        input.allowed_biomes = BTreeSet::from([ident!("minecraft:desert").to_string_ident()]);
        let plan = validate_snapshot_plan(input).unwrap();
        let raw = RawSnapshotChunk {
            pos: ChunkPos::new(0, 0),
            data: valid_chunk_nbt_with_section_y(0),
            timestamp: TEST_TIMESTAMP,
        };
        let biome_to_id = biome_to_id_map(&biome_registry());

        let err = normalize_chunk_snapshot(raw, &plan, &biome_to_id).unwrap_err();

        assert!(matches!(err, SnapshotChunkError::BiomeMismatch { .. }));
    }

    #[test]
    fn partial_load_policy_skips_missing_region_deterministically() {
        let dir = tempdir().unwrap();
        fs::create_dir_all(dir.path().join(REGION_DIRECTORY_NAME)).unwrap();
        fs::write(
            region_file_path(dir.path(), RegionCoord::new(0, 0)),
            vec![0_u8; TEST_REGION_HEADER_BYTES],
        )
        .unwrap();

        let mut input = plan_input_with_chunk(dir.path().to_path_buf(), ChunkPos::new(0, 0));
        input
            .chunk_positions
            .push(ChunkPos::new(REGION_CHUNK_AXIS, 0));
        input.missing_region_policy = MissingRegionPolicy::Skip;
        input.partial_load_policy = PartialLoadPolicy::CommitSuccessful;
        let biomes = biome_registry();

        let snapshot = load_static_world_snapshot(input, &biomes).unwrap();

        assert!(snapshot.report.partial);
        assert_eq!(snapshot.report.loaded_chunks, 0);
        assert_eq!(snapshot.report.empty_chunks, 1);
        assert_eq!(
            snapshot.report.missing_regions,
            vec![RegionCoord::new(1, 0)]
        );
    }

    #[test]
    fn cancellation_stops_before_layer_mutation() {
        let dir = tempdir().unwrap();
        write_valid_chunk_region(dir.path(), ChunkPos::new(0, 0));
        let input = plan_input_with_chunk(dir.path().to_path_buf(), ChunkPos::new(0, 0));
        let biomes = biome_registry();

        let err = load_static_world_snapshot_with_cancel(input, &biomes, || true).unwrap_err();

        assert!(matches!(
            err,
            SnapshotLoadError::Cancelled { loaded_chunks: 0 }
        ));
    }

    fn plan_input_with_chunk(dimension_root: PathBuf, chunk_pos: ChunkPos) -> SnapshotPlanInput {
        let mut input = SnapshotPlanInput::new(dimension_root, snapshot_dimension());
        input.chunk_positions.push(chunk_pos);
        input.allowed_biomes = BTreeSet::from([ident!("minecraft:plains").to_string_ident()]);
        input
    }

    fn snapshot_dimension() -> SnapshotDimension {
        SnapshotDimension {
            dimension_type_name: ident!("minecraft:overworld").to_string_ident(),
            min_y: TEST_MIN_Y,
            height: TEST_HEIGHT,
        }
    }

    fn biome_registry() -> BiomeRegistry {
        let mut biomes = BiomeRegistry::default();
        biomes.insert(ident!("minecraft:plains"), Biome::default());
        biomes.insert(ident!("minecraft:desert"), Biome::default());
        biomes
    }

    fn write_valid_chunk_region(dimension_root: &Path, pos: ChunkPos) {
        let region_root = dimension_root.join(REGION_DIRECTORY_NAME);
        fs::create_dir_all(&region_root).unwrap();
        let mut folder = RegionFolder::new(region_root);
        folder
            .set_chunk(pos.x, pos.z, &valid_chunk_nbt_with_section_y(0))
            .unwrap();
    }

    fn valid_chunk_nbt_with_section_y(section_y: i8) -> Compound {
        compound! {
            "sections" => List::Compound(vec![compound! {
                "Y" => section_y,
                "block_states" => compound! {
                    "palette" => List::Compound(vec![compound! {
                        "Name" => "minecraft:air",
                    }]),
                },
                "biomes" => compound! {
                    "palette" => List::String(vec!["minecraft:plains".to_owned()]),
                },
            }]),
            "block_entities" => List::Compound(Vec::new()),
        }
    }
}
