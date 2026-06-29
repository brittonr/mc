use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet};

use valence_nbt::{Compound, List, Value};
use valence_server::layer::chunk::Chunk;
use valence_server::registry::biome::BiomeId;
use valence_server::{ChunkPos, Ident};

use crate::parsing::parse_chunk;

use super::model::{
    ChunkSnapshotSummary, LoadedSnapshotChunk, RawSnapshotChunk, SnapshotChunkError,
    SnapshotDimension, SnapshotNbtError, SnapshotPlan,
};
use super::planning::SECTION_HEIGHT_BLOCKS;

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
    let Some(dimension_height) = i32::try_from(dimension.height).ok() else {
        return false;
    };
    let Some(dimension_end_y) = dimension.min_y.checked_add(dimension_height) else {
        return false;
    };
    section_min_y >= dimension.min_y && section_min_y < dimension_end_y
}
