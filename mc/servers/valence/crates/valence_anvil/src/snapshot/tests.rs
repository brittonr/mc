use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use tempfile::tempdir;
use valence_nbt::{compound, Compound, List};
use valence_server::registry::biome::ident;
use valence_server::registry::biome::{Biome, BiomeRegistry};
use valence_server::ChunkPos;

use super::filesystem::{biome_to_id_map, region_file_path, REGION_DIRECTORY_NAME};
use super::planning::{REGION_CHUNK_AXIS, REGION_CHUNK_COUNT};
use super::*;
use crate::parsing::ParseChunkError;
use crate::RegionFolder;

const TEST_HEIGHT: u32 = 16;
const TEST_MIN_Y: i32 = 0;
const TEST_TIMESTAMP: u32 = 0;
const TEST_REGION_HEADER_BYTES: usize = 8192;
const FRESH_CACHE_TIMESTAMP: u32 = 20;
const CURRENT_SOURCE_TIMESTAMP: u32 = 20;
const STALE_CACHE_TIMESTAMP: u32 = 10;
const NEWER_SOURCE_TIMESTAMP: u32 = 30;

#[test]
fn plan_validation_accepts_explicit_chunk() {
    let input = plan_input_with_chunk(tempdir().unwrap().path().to_path_buf(), ChunkPos::new(0, 0));

    let plan = validate_snapshot_plan(input).unwrap();

    assert_eq!(plan.regions, vec![RegionCoord::new(0, 0)]);
    assert_eq!(plan.chunks, vec![ChunkPos::new(0, 0)]);
}

#[test]
fn plan_validation_maps_negative_chunks_to_floor_divided_region() {
    let input = plan_input_with_chunk(
        tempdir().unwrap().path().to_path_buf(),
        ChunkPos::new(-1, -REGION_CHUNK_AXIS - 1),
    );

    let plan = validate_snapshot_plan(input).unwrap();

    assert_eq!(plan.regions, vec![RegionCoord::new(-1, -2)]);
}

#[test]
fn region_range_expands_chunk_lookup_plan() {
    let mut input = SnapshotPlanInput::new(
        tempdir().unwrap().path().to_path_buf(),
        snapshot_dimension(),
    );
    input.region_ranges.push(RegionRange::new(-1, -1, 1, 1));
    input.allowed_biomes = BTreeSet::from([ident!("minecraft:plains").to_string_ident()]);

    let plan = validate_snapshot_plan(input).unwrap();

    assert_eq!(plan.regions, vec![RegionCoord::new(-1, 1)]);
    assert_eq!(plan.chunks.len(), REGION_CHUNK_COUNT);
    assert!(plan
        .chunks
        .contains(&ChunkPos::new(-REGION_CHUNK_AXIS, REGION_CHUNK_AXIS)));
    assert!(plan
        .chunks
        .contains(&ChunkPos::new(-1, REGION_CHUNK_AXIS * 2 - 1)));
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
fn region_coordinate_overflow_rejected() {
    let mut input = SnapshotPlanInput::new(
        tempdir().unwrap().path().to_path_buf(),
        snapshot_dimension(),
    );
    input
        .region_ranges
        .push(RegionRange::new(i32::MAX, i32::MAX, 0, 0));
    input.allowed_biomes = BTreeSet::from([ident!("minecraft:plains").to_string_ident()]);

    let err = validate_snapshot_plan(input).unwrap_err();

    assert_eq!(err, SnapshotPlanError::CoordinateOverflow);
}

#[test]
fn valid_parse_summary_reports_expected_sections_and_biomes() {
    let summary =
        summarize_chunk_nbt(ChunkPos::new(0, 0), &valid_chunk_nbt_with_section_y(0)).unwrap();

    assert_eq!(summary.section_count, 1);
    assert_eq!(summary.min_section_y, 0);
    assert_eq!(summary.max_section_y, 0);
    assert_eq!(
        summary.biomes,
        BTreeSet::from([ident!("minecraft:plains").to_string_ident()])
    );
}

#[test]
fn malformed_parse_summary_rejects_missing_biomes() {
    let nbt = compound! {
        "sections" => List::Compound(vec![compound! {
            "Y" => 0_i8,
            "block_states" => compound! {
                "palette" => List::Compound(vec![compound! {
                    "Name" => "minecraft:air",
                }]),
            },
        }]),
        "block_entities" => List::Compound(Vec::new()),
    };

    let err = summarize_chunk_nbt(ChunkPos::new(0, 0), &nbt).unwrap_err();

    assert_eq!(err, SnapshotNbtError::MissingBiomes);
}

#[test]
fn cache_plan_reuses_fresh_entry() {
    let action = plan_snapshot_cache(SnapshotCachePlanInput {
        cached: Some(SnapshotCacheEntry {
            timestamp: FRESH_CACHE_TIMESTAMP,
        }),
        source: SnapshotSourceState {
            timestamp: Some(CURRENT_SOURCE_TIMESTAMP),
            available: true,
        },
        policy: SnapshotCachePolicy::ReuseFresh,
    });

    assert_eq!(action, SnapshotCacheAction::ReuseCached);
}

#[test]
fn cache_plan_evicts_stale_entry() {
    let action = plan_snapshot_cache(SnapshotCachePlanInput {
        cached: Some(SnapshotCacheEntry {
            timestamp: STALE_CACHE_TIMESTAMP,
        }),
        source: SnapshotSourceState {
            timestamp: Some(NEWER_SOURCE_TIMESTAMP),
            available: true,
        },
        policy: SnapshotCachePolicy::ReuseFresh,
    });

    assert_eq!(action, SnapshotCacheAction::EvictStale);
}

#[test]
fn snapshot_update_plan_counts_inserts_and_replacements() {
    let presence = [
        SnapshotChunkPresence {
            pos: ChunkPos::new(0, 0),
            present: false,
        },
        SnapshotChunkPresence {
            pos: ChunkPos::new(1, 0),
            present: true,
        },
    ];

    let plan = plan_snapshot_update(&presence);

    assert_eq!(plan.report.inserted_chunks, 1);
    assert_eq!(plan.report.replaced_chunks, 1);
    assert_eq!(
        plan.updates,
        vec![
            SnapshotChunkUpdate {
                pos: ChunkPos::new(0, 0),
                action: SnapshotUpdateAction::Insert,
            },
            SnapshotChunkUpdate {
                pos: ChunkPos::new(1, 0),
                action: SnapshotUpdateAction::Replace,
            },
        ]
    );
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
fn unavailable_dimension_directory_fails_closed() {
    let dir = tempdir().unwrap();
    let unavailable_root = dir.path().join("not-a-directory");
    fs::write(&unavailable_root, b"not a directory").unwrap();
    let input = plan_input_with_chunk(unavailable_root, ChunkPos::new(0, 0));
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

fn plan_input_with_chunk(
    dimension_root: std::path::PathBuf,
    chunk_pos: ChunkPos,
) -> SnapshotPlanInput {
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
