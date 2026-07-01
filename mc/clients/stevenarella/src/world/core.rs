use crate::shared::Position;
use std::convert::TryFrom;
use std::fmt;

pub(crate) const DEFAULT_WORLD_MIN_Y: i32 = 0;
pub(crate) const DEFAULT_WORLD_HEIGHT: i32 = 256;
pub(crate) const CHUNK_SECTION_HEIGHT_BLOCKS: i32 = 16;
pub(crate) const CHUNK_SECTION_BLOCK_COUNT: usize = 4096;
pub(crate) const LEGACY_CHUNK_SECTION_COUNT: usize = 16;
pub(crate) const LEGACY_MIN_SECTION_Y: i32 = 0;
pub(crate) const LEGACY_MAX_SECTION_Y: i32 = 15;
pub(crate) const PROTOCOL_1_18: i32 = 757;
pub(crate) const PROTOCOL_1_20_1: i32 = 763;
pub(crate) const LIGHT_ARRAY_BYTES: usize = 2048;
pub(crate) const EMPTY_LIGHT_BYTES: [u8; LIGHT_ARRAY_BYTES] = [0u8; LIGHT_ARRAY_BYTES];

const MAX_MASKABLE_CHUNK_SECTIONS: usize = u64::BITS as usize;
const MIN_CHUNK_SECTION_COUNT: usize = 1;
const BLOCK_UPDATE_RADIUS: i32 = 1;
const BLOCK_UPDATE_TARGET_COUNT: usize = 27;
const LIGHT_MASK_BITS_PER_WORD: usize = u64::BITS as usize;
const LIGHT_MASK_BITS_PER_WORD_I32: i32 = u64::BITS as i32;
const LIGHT_MASK_EXTRA_SECTION_OFFSET: i32 = -1;
const BLOCK_INDIRECT_MIN_BITS: u8 = 4;
const BLOCK_INDIRECT_MAX_EXCLUSIVE: u8 = 9;
const DIRECT_PALETTE_MAX_EXCLUSIVE: u8 = 17;
const BIOME_INDIRECT_MAX_EXCLUSIVE: u8 = 4;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct DimensionBounds {
    pub(crate) min_y: i32,
    pub(crate) height: i32,
}

impl DimensionBounds {
    pub(crate) fn new(min_y: i32, height: i32) -> Option<Self> {
        if !is_valid_dimension_bounds(min_y, height) {
            return None;
        }

        Some(Self { min_y, height })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct DimensionTypeSummary<'a> {
    pub(crate) name: &'a str,
    pub(crate) min_y: Option<i32>,
    pub(crate) height: Option<i32>,
}

impl<'a> DimensionTypeSummary<'a> {
    pub(crate) fn new(name: &'a str, min_y: Option<i32>, height: Option<i32>) -> Self {
        Self {
            name,
            min_y,
            height,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum DimensionBoundsSelection {
    Selected(DimensionBounds),
    Missing,
    Invalid,
}

pub(crate) fn select_dimension_bounds(
    dimension_type_name: &str,
    summaries: &[DimensionTypeSummary<'_>],
) -> DimensionBoundsSelection {
    for summary in summaries {
        if summary.name != dimension_type_name {
            continue;
        }

        let Some(min_y) = summary.min_y else {
            return DimensionBoundsSelection::Invalid;
        };
        let Some(height) = summary.height else {
            return DimensionBoundsSelection::Invalid;
        };
        let Some(bounds) = DimensionBounds::new(min_y, height) else {
            return DimensionBoundsSelection::Invalid;
        };

        return DimensionBoundsSelection::Selected(bounds);
    }

    DimensionBoundsSelection::Missing
}

pub(crate) fn is_valid_dimension_bounds(min_y: i32, height: i32) -> bool {
    min_y % CHUNK_SECTION_HEIGHT_BLOCKS == 0 && section_count_from_height(height).is_ok()
}

pub(crate) fn section_count_from_height(height: i32) -> Result<usize, WorldCoreError> {
    if height <= 0 || height % CHUNK_SECTION_HEIGHT_BLOCKS != 0 {
        return Err(WorldCoreError::InvalidDimensionHeight { height });
    }

    let sections = height / CHUNK_SECTION_HEIGHT_BLOCKS;
    let section_count =
        usize::try_from(sections).map_err(|_| WorldCoreError::InvalidDimensionHeight { height })?;
    validate_section_count(section_count)?;
    Ok(section_count)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ChunkLayout {
    min_section_y: i32,
    section_count: usize,
}

impl ChunkLayout {
    pub(crate) fn from_min_y_and_height(min_y: i32, height: i32) -> Result<Self, WorldCoreError> {
        let section_count = section_count_from_height(height)?;
        Self::from_section_count(min_y, section_count)
    }

    pub(crate) fn from_section_count(
        min_y: i32,
        section_count: usize,
    ) -> Result<Self, WorldCoreError> {
        if min_y % CHUNK_SECTION_HEIGHT_BLOCKS != 0 {
            return Err(WorldCoreError::InvalidDimensionMinY { min_y });
        }

        validate_section_count(section_count)?;
        Ok(Self {
            min_section_y: min_y / CHUNK_SECTION_HEIGHT_BLOCKS,
            section_count,
        })
    }

    pub(crate) fn section_count(self) -> usize {
        self.section_count
    }

    #[cfg(test)]
    pub(crate) fn min_section_y(self) -> i32 {
        self.min_section_y
    }

    pub(crate) fn section_y_at_index(self, section_index: usize) -> Option<i32> {
        if section_index >= self.section_count {
            return None;
        }

        let section_offset = i32::try_from(section_index).ok()?;
        self.min_section_y.checked_add(section_offset)
    }

    pub(crate) fn all_sections_mask(self) -> Result<u64, WorldCoreError> {
        mask_for_section_count(self.section_count)
    }
}

pub(crate) fn section_mask_bit(section_index: usize) -> Result<u64, WorldCoreError> {
    if section_index >= MAX_MASKABLE_CHUNK_SECTIONS {
        return Err(WorldCoreError::InvalidSectionIndex {
            section_index,
            section_count: MAX_MASKABLE_CHUNK_SECTIONS,
        });
    }

    Ok(1u64 << section_index)
}

pub(crate) fn active_section_indices(
    mask: u64,
    section_count: usize,
) -> Result<Vec<usize>, WorldCoreError> {
    validate_chunk_mask(mask, section_count)?;

    let mut sections = Vec::with_capacity(section_count);
    for section_index in 0..section_count {
        if mask & section_mask_bit(section_index)? == 0 {
            continue;
        }
        sections.push(section_index);
    }

    Ok(sections)
}

pub(crate) fn validate_chunk_mask(mask: u64, section_count: usize) -> Result<(), WorldCoreError> {
    let allowed_mask = mask_for_section_count(section_count)?;
    if mask & !allowed_mask != 0 {
        return Err(WorldCoreError::ChunkMaskOutsideLayout { mask, allowed_mask });
    }

    Ok(())
}

pub(crate) fn should_fill_sky(
    section_index: usize,
    has_existing_section_above: bool,
    mask: u64,
) -> Result<bool, WorldCoreError> {
    if has_existing_section_above {
        return Ok(false);
    }

    let current_and_below = if section_index + 1 == MAX_MASKABLE_CHUNK_SECTIONS {
        u64::MAX
    } else {
        section_mask_bit(section_index + 1)? - 1
    };
    Ok(mask & !current_and_below == 0)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct SectionCoord {
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) z: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct BlockUpdateTarget {
    pub(crate) block_pos: Position,
    pub(crate) dirty_section: SectionCoord,
}

pub(crate) fn plan_block_update(center: Position) -> Vec<BlockUpdateTarget> {
    let mut targets = Vec::with_capacity(BLOCK_UPDATE_TARGET_COUNT);

    for yy in -BLOCK_UPDATE_RADIUS..=BLOCK_UPDATE_RADIUS {
        for zz in -BLOCK_UPDATE_RADIUS..=BLOCK_UPDATE_RADIUS {
            for xx in -BLOCK_UPDATE_RADIUS..=BLOCK_UPDATE_RADIUS {
                let block_pos = center + (xx, yy, zz);
                targets.push(BlockUpdateTarget {
                    block_pos,
                    dirty_section: SectionCoord {
                        x: block_pos.x >> 4,
                        y: block_pos.y >> 4,
                        z: block_pos.z >> 4,
                    },
                });
            }
        }
    }

    debug_assert_eq!(targets.len(), BLOCK_UPDATE_TARGET_COUNT);
    targets
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct BlockEntityActionPlan {
    pub(crate) remove_existing: bool,
    pub(crate) create_new: bool,
}

pub(crate) fn plan_block_entity_actions(
    existing_block_entity: bool,
    new_block_has_entity: bool,
) -> BlockEntityActionPlan {
    BlockEntityActionPlan {
        remove_existing: existing_block_entity,
        create_new: new_block_has_entity,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum StorageWritePlan {
    Noop,
    WriteExistingSection,
    CreateSection { fill_sky: bool },
}

pub(crate) fn plan_storage_write(
    section_exists: bool,
    current_block_matches_new: bool,
    new_block_is_air: bool,
    has_section_above: bool,
) -> StorageWritePlan {
    if current_block_matches_new {
        return StorageWritePlan::Noop;
    }

    if section_exists {
        return StorageWritePlan::WriteExistingSection;
    }

    if new_block_is_air {
        return StorageWritePlan::Noop;
    }

    StorageWritePlan::CreateSection {
        fill_sky: has_section_above,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct LightPayloadUpdate {
    pub(crate) payload_index: usize,
    pub(crate) section_y: i32,
}

pub(crate) fn plan_light_payload_updates(
    min_y: i32,
    masks: &[i64],
    payload_lengths: &[usize],
) -> Result<Vec<LightPayloadUpdate>, WorldCoreError> {
    let mut updates = Vec::new();
    let mut payload_index = 0usize;

    for (mask_word_index, mask) in masks.iter().enumerate() {
        let unsigned_mask = *mask as u64;
        for bit_index in 0..LIGHT_MASK_BITS_PER_WORD {
            if unsigned_mask & section_mask_bit(bit_index)? == 0 {
                continue;
            }

            let Some(payload_length) = payload_lengths.get(payload_index) else {
                return Err(WorldCoreError::MissingLightPayload { payload_index });
            };
            validate_light_payload_length(payload_index, *payload_length)?;

            updates.push(LightPayloadUpdate {
                payload_index,
                section_y: light_section_y(min_y, mask_word_index, bit_index)?,
            });
            payload_index += 1;
        }
    }

    Ok(updates)
}

pub(crate) fn plan_light_clear_sections(
    min_y: i32,
    masks: &[i64],
) -> Result<Vec<i32>, WorldCoreError> {
    let mut sections = Vec::new();

    for (mask_word_index, mask) in masks.iter().enumerate() {
        let unsigned_mask = *mask as u64;
        for bit_index in 0..LIGHT_MASK_BITS_PER_WORD {
            if unsigned_mask & section_mask_bit(bit_index)? == 0 {
                continue;
            }

            sections.push(light_section_y(min_y, mask_word_index, bit_index)?);
        }
    }

    Ok(sections)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum PaletteKind {
    BlockStates,
    Biomes,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum PaletteFormat {
    SingleValued(usize),
    Indirect(Vec<usize>, u8),
    Direct(u8),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum PaletteShape {
    SingleValued,
    Indirect { bits_per_entry: u8 },
    Direct { bits_per_entry: u8 },
}

pub(crate) fn validate_palette_kind_supported(
    protocol_version: i32,
    kind: PaletteKind,
) -> Result<(), WorldCoreError> {
    if kind == PaletteKind::Biomes && protocol_version < PROTOCOL_1_18 {
        return Err(WorldCoreError::BiomePaletteUnsupportedProtocol { protocol_version });
    }

    Ok(())
}

pub(crate) fn palette_shape(
    protocol_version: i32,
    kind: PaletteKind,
    raw_bits_per_entry: u8,
) -> Result<PaletteShape, WorldCoreError> {
    validate_palette_kind_supported(protocol_version, kind)?;

    let bits_per_entry = if protocol_version < PROTOCOL_1_18
        && kind == PaletteKind::BlockStates
        && raw_bits_per_entry == 0
    {
        BLOCK_INDIRECT_MIN_BITS
    } else {
        raw_bits_per_entry
    };

    match kind {
        PaletteKind::BlockStates => match bits_per_entry {
            0 => Ok(PaletteShape::SingleValued),
            n if n < BLOCK_INDIRECT_MAX_EXCLUSIVE => Ok(PaletteShape::Indirect {
                bits_per_entry: n.max(BLOCK_INDIRECT_MIN_BITS),
            }),
            n if n < DIRECT_PALETTE_MAX_EXCLUSIVE => Ok(PaletteShape::Direct { bits_per_entry: n }),
            n => Err(WorldCoreError::PaletteBitsUnsupported {
                kind,
                bits_per_entry: n,
            }),
        },
        PaletteKind::Biomes => match bits_per_entry {
            0 => Ok(PaletteShape::SingleValued),
            n if n < BIOME_INDIRECT_MAX_EXCLUSIVE => {
                Ok(PaletteShape::Indirect { bits_per_entry: n })
            }
            n if n < DIRECT_PALETTE_MAX_EXCLUSIVE => Ok(PaletteShape::Direct { bits_per_entry: n }),
            n => Err(WorldCoreError::PaletteBitsUnsupported {
                kind,
                bits_per_entry: n,
            }),
        },
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum WorldCoreError {
    InvalidDimensionMinY {
        min_y: i32,
    },
    InvalidDimensionHeight {
        height: i32,
    },
    InvalidSectionCount {
        section_count: usize,
    },
    InvalidSectionIndex {
        section_index: usize,
        section_count: usize,
    },
    ChunkMaskOutsideLayout {
        mask: u64,
        allowed_mask: u64,
    },
    MissingLightPayload {
        payload_index: usize,
    },
    InvalidLightPayloadLength {
        payload_index: usize,
        actual_len: usize,
        expected_len: usize,
    },
    BiomePaletteUnsupportedProtocol {
        protocol_version: i32,
    },
    PaletteBitsUnsupported {
        kind: PaletteKind,
        bits_per_entry: u8,
    },
    IntegerOverflow {
        field: &'static str,
    },
}

impl fmt::Display for WorldCoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WorldCoreError::InvalidDimensionMinY { min_y } => {
                write!(f, "invalid dimension min_y {min_y}")
            }
            WorldCoreError::InvalidDimensionHeight { height } => {
                write!(f, "invalid dimension height {height}")
            }
            WorldCoreError::InvalidSectionCount { section_count } => {
                write!(f, "invalid chunk section count {section_count}")
            }
            WorldCoreError::InvalidSectionIndex {
                section_index,
                section_count,
            } => write!(
                f,
                "section index {section_index} is outside section count {section_count}"
            ),
            WorldCoreError::ChunkMaskOutsideLayout { mask, allowed_mask } => write!(
                f,
                "chunk mask {mask:#x} contains sections outside allowed mask {allowed_mask:#x}"
            ),
            WorldCoreError::MissingLightPayload { payload_index } => {
                write!(f, "missing light payload at index {payload_index}")
            }
            WorldCoreError::InvalidLightPayloadLength {
                payload_index,
                actual_len,
                expected_len,
            } => write!(
                f,
                "light payload {payload_index} has length {actual_len}, expected {expected_len}"
            ),
            WorldCoreError::BiomePaletteUnsupportedProtocol { protocol_version } => write!(
                f,
                "protocol {protocol_version} does not support biome palettes"
            ),
            WorldCoreError::PaletteBitsUnsupported {
                kind,
                bits_per_entry,
            } => write!(
                f,
                "unsupported {kind:?} palette bits_per_entry {bits_per_entry}"
            ),
            WorldCoreError::IntegerOverflow { field } => {
                write!(f, "integer overflow while deriving {field}")
            }
        }
    }
}

fn validate_section_count(section_count: usize) -> Result<(), WorldCoreError> {
    if !(MIN_CHUNK_SECTION_COUNT..=MAX_MASKABLE_CHUNK_SECTIONS).contains(&section_count) {
        return Err(WorldCoreError::InvalidSectionCount { section_count });
    }

    Ok(())
}

fn mask_for_section_count(section_count: usize) -> Result<u64, WorldCoreError> {
    validate_section_count(section_count)?;
    if section_count == MAX_MASKABLE_CHUNK_SECTIONS {
        return Ok(u64::MAX);
    }

    Ok((1u64 << section_count) - 1)
}

fn light_section_y(
    min_y: i32,
    mask_word_index: usize,
    bit_index: usize,
) -> Result<i32, WorldCoreError> {
    if min_y % CHUNK_SECTION_HEIGHT_BLOCKS != 0 {
        return Err(WorldCoreError::InvalidDimensionMinY { min_y });
    }

    let mask_word_index =
        i32::try_from(mask_word_index).map_err(|_| WorldCoreError::IntegerOverflow {
            field: "light mask word index",
        })?;
    let bit_index = i32::try_from(bit_index).map_err(|_| WorldCoreError::IntegerOverflow {
        field: "light bit index",
    })?;
    let word_offset = mask_word_index
        .checked_mul(LIGHT_MASK_BITS_PER_WORD_I32)
        .ok_or(WorldCoreError::IntegerOverflow {
            field: "light mask word offset",
        })?;
    let min_section_y = min_y / CHUNK_SECTION_HEIGHT_BLOCKS;

    min_section_y
        .checked_add(word_offset)
        .and_then(|value| value.checked_add(bit_index))
        .and_then(|value| value.checked_add(LIGHT_MASK_EXTRA_SECTION_OFFSET))
        .ok_or(WorldCoreError::IntegerOverflow {
            field: "light section y",
        })
}

fn validate_light_payload_length(
    payload_index: usize,
    actual_len: usize,
) -> Result<(), WorldCoreError> {
    if actual_len != LIGHT_ARRAY_BYTES {
        return Err(WorldCoreError::InvalidLightPayloadLength {
            payload_index,
            actual_len,
            expected_len: LIGHT_ARRAY_BYTES,
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const OVERWORLD_NAME: &str = "minecraft:overworld";
    const NETHER_NAME: &str = "minecraft:the_nether";
    const OVERWORLD_MIN_Y: i32 = -64;
    const OVERWORLD_HEIGHT: i32 = 384;
    const OVERWORLD_SECTION_COUNT: usize = 24;
    const OVERWORLD_MIN_SECTION_Y: i32 = -4;
    const OVERWORLD_LAST_SECTION_INDEX: usize = OVERWORLD_SECTION_COUNT - 1;
    const OVERWORLD_LAST_SECTION_Y: i32 = 19;
    const DEFAULT_SECTION_COUNT: usize = 16;
    const UNALIGNED_MIN_Y: i32 = -63;
    const UNALIGNED_HEIGHT: i32 = 10;
    const ZERO_HEIGHT: i32 = 0;
    const TWO_SECTION_COUNT: usize = 2;
    const TWO_SECTION_ALLOWED_MASK: u64 = 0b11;
    const TWO_SECTION_EXTRA_MASK: u64 = 0b100;
    const FIRST_SECTION_MASK: u64 = 0b1;
    const FIRST_PAYLOAD_INDEX: usize = 0;
    const FIRST_LIGHT_SECTION_Y: i32 = OVERWORLD_MIN_SECTION_Y - 1;
    const VALID_BLOCK_BITS: u8 = 8;
    const VALID_BIOME_BITS: u8 = 3;
    const INVALID_PALETTE_BITS: u8 = 17;
    const LEGACY_PROTOCOL: i32 = 340;
    const CENTER_X: i32 = 10;
    const CENTER_Y: i32 = 65;
    const CENTER_Z: i32 = -4;

    #[test]
    fn selects_dimension_bounds_by_name() {
        let summaries = [
            DimensionTypeSummary::new(
                NETHER_NAME,
                Some(DEFAULT_WORLD_MIN_Y),
                Some(DEFAULT_WORLD_HEIGHT),
            ),
            DimensionTypeSummary::new(
                OVERWORLD_NAME,
                Some(OVERWORLD_MIN_Y),
                Some(OVERWORLD_HEIGHT),
            ),
        ];

        let selection = select_dimension_bounds(OVERWORLD_NAME, &summaries);

        assert_eq!(
            selection,
            DimensionBoundsSelection::Selected(DimensionBounds {
                min_y: OVERWORLD_MIN_Y,
                height: OVERWORLD_HEIGHT,
            })
        );
    }

    #[test]
    fn rejects_missing_or_invalid_dimension_bounds() {
        let missing_selection = select_dimension_bounds(OVERWORLD_NAME, &[]);
        let missing_height = [DimensionTypeSummary::new(
            OVERWORLD_NAME,
            Some(OVERWORLD_MIN_Y),
            None,
        )];
        let unaligned_min_y = [DimensionTypeSummary::new(
            OVERWORLD_NAME,
            Some(UNALIGNED_MIN_Y),
            Some(OVERWORLD_HEIGHT),
        )];
        let zero_height = [DimensionTypeSummary::new(
            OVERWORLD_NAME,
            Some(OVERWORLD_MIN_Y),
            Some(ZERO_HEIGHT),
        )];

        assert_eq!(missing_selection, DimensionBoundsSelection::Missing);
        assert_eq!(
            select_dimension_bounds(OVERWORLD_NAME, &missing_height),
            DimensionBoundsSelection::Invalid
        );
        assert_eq!(
            select_dimension_bounds(OVERWORLD_NAME, &unaligned_min_y),
            DimensionBoundsSelection::Invalid
        );
        assert_eq!(
            select_dimension_bounds(OVERWORLD_NAME, &zero_height),
            DimensionBoundsSelection::Invalid
        );
    }

    #[test]
    fn derives_chunk_layout_from_dimension_bounds() {
        let layout = ChunkLayout::from_min_y_and_height(OVERWORLD_MIN_Y, OVERWORLD_HEIGHT).unwrap();

        assert_eq!(layout.section_count(), OVERWORLD_SECTION_COUNT);
        assert_eq!(layout.min_section_y(), OVERWORLD_MIN_SECTION_Y);
        assert_eq!(
            layout.section_y_at_index(FIRST_PAYLOAD_INDEX),
            Some(OVERWORLD_MIN_SECTION_Y)
        );
        assert_eq!(
            layout.section_y_at_index(OVERWORLD_LAST_SECTION_INDEX),
            Some(OVERWORLD_LAST_SECTION_Y)
        );
        assert_eq!(layout.all_sections_mask().unwrap(), 0x00ff_ffff);
    }

    #[test]
    fn rejects_invalid_chunk_layouts_and_masks() {
        let unaligned = ChunkLayout::from_min_y_and_height(UNALIGNED_MIN_Y, OVERWORLD_HEIGHT);
        let unaligned_height =
            ChunkLayout::from_min_y_and_height(OVERWORLD_MIN_Y, UNALIGNED_HEIGHT);
        let extra_mask = validate_chunk_mask(TWO_SECTION_EXTRA_MASK, TWO_SECTION_COUNT);

        assert!(matches!(
            unaligned,
            Err(WorldCoreError::InvalidDimensionMinY { .. })
        ));
        assert!(matches!(
            unaligned_height,
            Err(WorldCoreError::InvalidDimensionHeight { .. })
        ));
        assert_eq!(
            mask_for_section_count(TWO_SECTION_COUNT).unwrap(),
            TWO_SECTION_ALLOWED_MASK
        );
        assert!(matches!(
            extra_mask,
            Err(WorldCoreError::ChunkMaskOutsideLayout { .. })
        ));
    }

    #[test]
    fn accepts_biome_and_block_palette_shapes() {
        let block_shape = palette_shape(PROTOCOL_1_18, PaletteKind::BlockStates, VALID_BLOCK_BITS);
        let biome_shape = palette_shape(PROTOCOL_1_18, PaletteKind::Biomes, VALID_BIOME_BITS);
        let legacy_zero_bits = palette_shape(LEGACY_PROTOCOL, PaletteKind::BlockStates, 0);

        assert_eq!(
            block_shape.unwrap(),
            PaletteShape::Indirect {
                bits_per_entry: VALID_BLOCK_BITS,
            }
        );
        assert_eq!(
            biome_shape.unwrap(),
            PaletteShape::Indirect {
                bits_per_entry: VALID_BIOME_BITS,
            }
        );
        assert_eq!(
            legacy_zero_bits.unwrap(),
            PaletteShape::Indirect {
                bits_per_entry: BLOCK_INDIRECT_MIN_BITS,
            }
        );
    }

    #[test]
    fn rejects_malformed_or_unsupported_palette_shapes() {
        let unsupported_legacy_biome =
            palette_shape(LEGACY_PROTOCOL, PaletteKind::Biomes, VALID_BIOME_BITS);
        let malformed_block = palette_shape(
            PROTOCOL_1_18,
            PaletteKind::BlockStates,
            INVALID_PALETTE_BITS,
        );
        let malformed_biome =
            palette_shape(PROTOCOL_1_18, PaletteKind::Biomes, INVALID_PALETTE_BITS);

        assert!(matches!(
            unsupported_legacy_biome,
            Err(WorldCoreError::BiomePaletteUnsupportedProtocol { .. })
        ));
        assert!(matches!(
            malformed_block,
            Err(WorldCoreError::PaletteBitsUnsupported { .. })
        ));
        assert!(matches!(
            malformed_biome,
            Err(WorldCoreError::PaletteBitsUnsupported { .. })
        ));
    }

    #[test]
    fn plans_light_payload_sections_from_dimension_min_y() {
        let updates = plan_light_payload_updates(
            OVERWORLD_MIN_Y,
            &[FIRST_SECTION_MASK as i64],
            &[LIGHT_ARRAY_BYTES],
        )
        .unwrap();
        let clears =
            plan_light_clear_sections(OVERWORLD_MIN_Y, &[FIRST_SECTION_MASK as i64]).unwrap();

        assert_eq!(updates.len(), MIN_CHUNK_SECTION_COUNT);
        assert_eq!(
            updates[FIRST_PAYLOAD_INDEX].payload_index,
            FIRST_PAYLOAD_INDEX
        );
        assert_eq!(
            updates[FIRST_PAYLOAD_INDEX].section_y,
            FIRST_LIGHT_SECTION_Y
        );
        assert_eq!(clears, vec![FIRST_LIGHT_SECTION_Y]);
    }

    #[test]
    fn rejects_missing_or_malformed_light_payloads() {
        let missing =
            plan_light_payload_updates(OVERWORLD_MIN_Y, &[FIRST_SECTION_MASK as i64], &[]);
        let malformed = plan_light_payload_updates(
            OVERWORLD_MIN_Y,
            &[FIRST_SECTION_MASK as i64],
            &[LIGHT_ARRAY_BYTES - 1],
        );

        assert!(matches!(
            missing,
            Err(WorldCoreError::MissingLightPayload { .. })
        ));
        assert!(matches!(
            malformed,
            Err(WorldCoreError::InvalidLightPayloadLength { .. })
        ));
    }

    #[test]
    fn plans_block_update_targets_and_storage_writes() {
        let center = Position::new(CENTER_X, CENTER_Y, CENTER_Z);
        let targets = plan_block_update(center);
        let first_target = targets.first().unwrap();
        let last_target = targets.last().unwrap();

        assert_eq!(targets.len(), BLOCK_UPDATE_TARGET_COUNT);
        assert_eq!(
            first_target.block_pos,
            center
                + (
                    -BLOCK_UPDATE_RADIUS,
                    -BLOCK_UPDATE_RADIUS,
                    -BLOCK_UPDATE_RADIUS
                )
        );
        assert_eq!(
            last_target.block_pos,
            center
                + (
                    BLOCK_UPDATE_RADIUS,
                    BLOCK_UPDATE_RADIUS,
                    BLOCK_UPDATE_RADIUS
                )
        );
        assert_eq!(
            plan_storage_write(false, false, false, true),
            StorageWritePlan::CreateSection { fill_sky: true }
        );
        assert_eq!(
            plan_storage_write(true, false, false, false),
            StorageWritePlan::WriteExistingSection
        );
    }

    #[test]
    fn rejects_noop_storage_writes_and_plans_block_entity_actions() {
        assert_eq!(
            plan_storage_write(false, false, true, false),
            StorageWritePlan::Noop
        );
        assert_eq!(
            plan_storage_write(true, true, false, false),
            StorageWritePlan::Noop
        );
        assert_eq!(
            plan_block_entity_actions(true, false),
            BlockEntityActionPlan {
                remove_existing: true,
                create_new: false,
            }
        );
        assert_eq!(
            plan_block_entity_actions(false, true),
            BlockEntityActionPlan {
                remove_existing: false,
                create_new: true,
            }
        );
    }

    #[test]
    fn plans_sky_fill_from_existing_and_masked_sections_above() {
        assert!(should_fill_sky(FIRST_PAYLOAD_INDEX, false, FIRST_SECTION_MASK).unwrap());
        assert!(!should_fill_sky(FIRST_PAYLOAD_INDEX, true, FIRST_SECTION_MASK).unwrap());
        assert!(!should_fill_sky(FIRST_PAYLOAD_INDEX, false, TWO_SECTION_ALLOWED_MASK).unwrap());
        assert_eq!(
            ChunkLayout::from_min_y_and_height(DEFAULT_WORLD_MIN_Y, DEFAULT_WORLD_HEIGHT)
                .unwrap()
                .section_count(),
            DEFAULT_SECTION_COUNT
        );
    }
}
