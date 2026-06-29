use std::collections::BTreeSet;
use std::path::PathBuf;

use thiserror::Error;
use valence_nbt::Compound;
use valence_server::layer::chunk::UnloadedChunk;
use valence_server::{ChunkPos, Ident};

use crate::parsing::ParseChunkError;
use crate::RegionError;

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
            max_regions: super::planning::DEFAULT_MAX_REGION_COUNT,
            max_chunks: super::planning::DEFAULT_MAX_CHUNK_COUNT,
            max_sections_per_chunk: super::planning::MAX_SECTIONS_PER_CHUNK,
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
        /// Limit that was exceeded.
        limit_name: &'static str,
        /// Actual selected count.
        actual: usize,
        /// Configured maximum count.
        limit: usize,
    },
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
