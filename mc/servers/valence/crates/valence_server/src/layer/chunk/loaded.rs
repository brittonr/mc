use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet};
use std::mem;
use std::sync::atomic::{AtomicU32, Ordering};

use parking_lot::Mutex; // Using nonstandard mutex to avoid poisoning API.
use valence_generated::block::{PropName, PropValue};
use valence_nbt::{compound, Compound, Value};
use valence_protocol::encode::{PacketWriter, WritePacket};
use valence_protocol::packets::play::chunk_data_s2c::ChunkDataBlockEntity;
use valence_protocol::packets::play::chunk_delta_update_s2c::ChunkDeltaUpdateEntry;
use valence_protocol::packets::play::{BlockEntityUpdateS2c, BlockUpdateS2c, ChunkDeltaUpdateS2c};
use valence_protocol::{BlockPos, BlockState, ChunkPos, ChunkSectionPos, Encode};
use valence_registry::biome::BiomeId;
use valence_registry::RegistryIdx;

use super::chunk::{
    bit_width, check_biome_oob, check_block_oob, check_section_oob, BiomeContainer,
    BlockStateContainer, Chunk, SECTION_BLOCK_COUNT,
};
use super::egress_cache::{self, CachedChunkPacket, ChunkEgressCacheProbe, ChunkEgressSnapshot};
use super::paletted_container::PalettedContainer;
use super::unloaded::{self, UnloadedChunk};
use super::{ChunkLayerInfo, ChunkLayerMessages, LocalMsg};

#[derive(Debug)]
pub struct LoadedChunk {
    /// A count of the clients viewing this chunk. Useful for knowing if it's
    /// necessary to record changes, since no client would be in view to receive
    /// the changes if this were zero.
    viewer_count: AtomicU32,
    /// Block and biome data for the chunk.
    sections: Box<[Section]>,
    /// The block entities in this chunk.
    block_entities: BTreeMap<u32, Compound>,
    /// The set of block entities that have been modified this tick.
    changed_block_entities: BTreeSet<u32>,
    /// If any biomes in this chunk have been modified this tick.
    changed_biomes: bool,
    /// Monotonic version for client-visible content that participates in keyed cached egress.
    content_version: u64,
    /// Keyed cached bytes of the chunk initialization packet for optional cached egress.
    cached_init_packet: Mutex<Option<CachedChunkPacket>>,
}

const INITIAL_CONTENT_VERSION: u64 = 0;
const CONTENT_VERSION_INCREMENT: u64 = 1;
const BLOCK_PALETTE_MIN_INDIRECT_BITS: usize = 4;
const BLOCK_PALETTE_MAX_INDIRECT_BITS: usize = 8;
const BIOME_PALETTE_MIN_INDIRECT_BITS: usize = 0;
const BIOME_PALETTE_MAX_INDIRECT_BITS: usize = 3;

#[derive(Clone, Default, Debug)]
struct Section {
    block_states: BlockStateContainer,
    biomes: BiomeContainer,
    /// Contains modifications for the update section packet. (Or the regular
    /// block update packet if len == 1).
    updates: Vec<ChunkDeltaUpdateEntry>,
}

impl Section {
    fn count_non_air_blocks(&self) -> u16 {
        let count = self.block_states.count_matching(|state| !state.is_air());
        u16::try_from(count).expect("section block count should fit in u16")
    }
}

impl LoadedChunk {
    pub(crate) fn new(height: u32) -> Self {
        Self {
            viewer_count: AtomicU32::new(0),
            sections: vec![Section::default(); height as usize / 16].into(),
            block_entities: BTreeMap::new(),
            changed_block_entities: BTreeSet::new(),
            changed_biomes: false,
            content_version: INITIAL_CONTENT_VERSION,
            cached_init_packet: Mutex::new(None),
        }
    }

    /// Sets the content of this chunk to the supplied [`UnloadedChunk`]. The
    /// given unloaded chunk is [resized] to match the height of this loaded
    /// chunk prior to insertion.
    ///
    /// The previous chunk data is returned.
    ///
    /// [resized]: UnloadedChunk::set_height
    pub(crate) fn insert(&mut self, mut chunk: UnloadedChunk) -> UnloadedChunk {
        chunk.set_height(self.height());

        let old_sections = self
            .sections
            .iter_mut()
            .zip(chunk.sections)
            .map(|(sect, other_sect)| {
                sect.updates.clear();

                unloaded::Section {
                    block_states: mem::replace(&mut sect.block_states, other_sect.block_states),
                    biomes: mem::replace(&mut sect.biomes, other_sect.biomes),
                }
            })
            .collect();
        let old_block_entities = mem::replace(&mut self.block_entities, chunk.block_entities);
        self.changed_block_entities.clear();
        self.changed_biomes = false;
        self.invalidate_init_packet_cache();
        self.assert_no_changes();

        UnloadedChunk {
            sections: old_sections,
            block_entities: old_block_entities,
        }
    }

    pub(crate) fn remove(&mut self) -> UnloadedChunk {
        let old_sections = self
            .sections
            .iter_mut()
            .map(|sect| {
                sect.updates.clear();

                unloaded::Section {
                    block_states: mem::take(&mut sect.block_states),
                    biomes: mem::take(&mut sect.biomes),
                }
            })
            .collect();
        let old_block_entities = mem::take(&mut self.block_entities);
        self.changed_block_entities.clear();
        self.changed_biomes = false;
        self.invalidate_init_packet_cache();

        self.assert_no_changes();

        UnloadedChunk {
            sections: old_sections,
            block_entities: old_block_entities,
        }
    }

    /// Returns the number of clients in view of this chunk.
    pub fn viewer_count(&self) -> u32 {
        self.viewer_count.load(Ordering::Relaxed)
    }

    /// Like [`Self::viewer_count`], but avoids an atomic operation.
    pub fn viewer_count_mut(&mut self) -> u32 {
        *self.viewer_count.get_mut()
    }

    /// Increments the viewer count.
    pub(crate) fn inc_viewer_count(&self) {
        self.viewer_count.fetch_add(1, Ordering::Relaxed);
    }

    /// Decrements the viewer count.
    #[track_caller]
    pub(crate) fn dec_viewer_count(&self) {
        let old = self.viewer_count.fetch_sub(1, Ordering::Relaxed);
        debug_assert_ne!(old, 0, "viewer count underflow!");
    }

    /// Performs the changes necessary to prepare this chunk for client updates.
    /// - Chunk change messages are written to the layer.
    /// - Recorded changes are cleared.
    pub(crate) fn update_pre_client(
        &mut self,
        pos: ChunkPos,
        info: &ChunkLayerInfo,
        messages: &mut ChunkLayerMessages,
    ) {
        if *self.viewer_count.get_mut() == 0 {
            // Nobody is viewing the chunk, so no need to send any update packets. There
            // also shouldn't be any changes that need to be cleared.
            self.assert_no_changes();

            return;
        }

        // Block states
        for (sect_y, sect) in self.sections.iter_mut().enumerate() {
            match sect.updates.as_slice() {
                &[] => {}
                &[entry] => {
                    let global_x = pos.x * 16 + i32::from(entry.off_x());
                    let global_y = info.min_y + sect_y as i32 * 16 + i32::from(entry.off_y());
                    let global_z = pos.z * 16 + i32::from(entry.off_z());

                    messages.send_local_infallible(LocalMsg::PacketAt { pos }, |buf| {
                        let mut writer = PacketWriter::new(buf, info.threshold);

                        writer.write_packet(&BlockUpdateS2c {
                            position: BlockPos::new(global_x, global_y, global_z),
                            block_id: BlockState::from_raw(entry.block_state() as u16).unwrap(),
                        });
                    });
                }
                entries => {
                    let chunk_sect_pos = ChunkSectionPos {
                        x: pos.x,
                        y: sect_y as i32 + info.min_y.div_euclid(16),
                        z: pos.z,
                    };

                    messages.send_local_infallible(LocalMsg::PacketAt { pos }, |buf| {
                        let mut writer = PacketWriter::new(buf, info.threshold);

                        writer.write_packet(&ChunkDeltaUpdateS2c {
                            chunk_sect_pos,
                            blocks: Cow::Borrowed(entries),
                        });
                    });
                }
            }

            sect.updates.clear();
        }

        // Block entities
        for &idx in &self.changed_block_entities {
            let Some(nbt) = self.block_entities.get(&idx) else {
                continue;
            };

            let x = idx % 16;
            let z = (idx / 16) % 16;
            let y = idx / 16 / 16;

            let state = self.sections[y as usize / 16]
                .block_states
                .get(idx as usize % SECTION_BLOCK_COUNT);

            let Some(kind) = state.block_entity_kind() else {
                continue;
            };

            let global_x = pos.x * 16 + x as i32;
            let global_y = info.min_y + y as i32;
            let global_z = pos.z * 16 + z as i32;

            messages.send_local_infallible(LocalMsg::PacketAt { pos }, |buf| {
                let mut writer = PacketWriter::new(buf, info.threshold);

                writer.write_packet(&BlockEntityUpdateS2c {
                    position: BlockPos::new(global_x, global_y, global_z),
                    kind,
                    data: Cow::Borrowed(nbt),
                });
            });
        }

        self.changed_block_entities.clear();

        // Biomes
        if self.changed_biomes {
            self.changed_biomes = false;

            messages.send_local_infallible(LocalMsg::ChangeBiome { pos }, |buf| {
                for sect in &self.sections {
                    sect.biomes
                        .encode_mc_format(
                            &mut *buf,
                            |b| b.to_index() as u64,
                            0,
                            3,
                            bit_width(info.biome_registry_len - 1),
                        )
                        .expect("paletted container encode should always succeed");
                }
            });
        }

        // All changes should be cleared.
        self.assert_no_changes();
    }

    /// Generates the `MOTION_BLOCKING` heightmap for this chunk, which stores
    /// the height of the highest non motion-blocking block in each column.
    ///
    /// The lowest value of the heightmap is 0, which means that there are no
    /// motion-blocking blocks in the column. In this case, rain will fall
    /// through the void and there will be no rain particles.
    ///
    /// A value of 1 means that rain particles will appear at the lowest
    /// possible height given by [`DimensionType::min_y`]. Note that
    /// blocks cannot be placed at `min_y - 1`.
    ///
    /// We take these two special cases into account by adding a value of 2 to
    /// our heightmap if we find a motion-blocking block, since
    /// `self.block_state(x, 0, z)` corresponds to the block at `(x, min_y, z)`
    /// ingame.
    ///
    /// [`DimensionType::min_y`]: valence_registry::dimension_type::DimensionType::min_y
    #[allow(clippy::needless_range_loop)]
    fn motion_blocking(&self) -> Vec<Vec<u32>> {
        let mut heightmap: Vec<Vec<u32>> = vec![vec![0; 16]; 16];

        for z in 0..16 {
            for x in 0..16 {
                for y in (0..self.height()).rev() {
                    let state = self.block_state(x as u32, y, z as u32);
                    if state.blocks_motion()
                        || state.is_liquid()
                        || state.get(PropName::Waterlogged) == Some(PropValue::True)
                    {
                        heightmap[z][x] = y + 2;
                        break;
                    }
                }
            }
        }

        heightmap
    }

    /// Encodes a given heightmap into the correct format of the
    /// `ChunkDataS2c` packet.
    ///
    /// The heightmap values are stored in a long array. Each value is encoded
    /// as a 9-bit unsigned integer, so every long with 64 bits can hold at
    /// most seven values. The long is padded at the left side with a single
    /// zero. Since there are 256 values for 256 columns in a chunk, there
    /// will be 36 fully filled longs and one half-filled long with four
    /// values. The remaining three values in the last long are left unused.
    ///
    /// For example, the `MOTION_BLOCKING` heightmap in an empty superflat
    /// world is always 4. The first 36 long values will then be
    ///
    /// 0 000000100 000000100 000000100 000000100 000000100 000000100 000000100,
    ///
    /// and the last long will be
    ///
    /// 0 000000000 000000000 000000000 000000100 000000100 000000100 000000100.
    fn encode_heightmap(heightmap: Vec<Vec<u32>>) -> Value {
        const BITS_PER_ENTRY: u32 = 9;
        const ENTRIES_PER_LONG: u32 = i64::BITS / BITS_PER_ENTRY;

        // Unless `ENTRIES_PER_LONG` is a power of 2 and therefore evenly divides 16*16,
        // we need to add one extra long to fit all values in the packet.
        const LONGS_PER_PACKET: u32 =
            16 * 16 / ENTRIES_PER_LONG + (16 * 16 % ENTRIES_PER_LONG != 0) as u32;

        let mut encoded: Vec<i64> = vec![0; LONGS_PER_PACKET as usize];
        let mut iter = heightmap.into_iter().flatten();

        for long in &mut encoded {
            for j in 0..ENTRIES_PER_LONG {
                match iter.next() {
                    None => break,
                    Some(y) => *long += i64::from(y) << (BITS_PER_ENTRY * j),
                }
            }
        }

        Value::LongArray(encoded)
    }

    /// Writes the packet data needed to initialize this chunk.
    pub(crate) fn write_init_packets(
        &self,
        mut writer: impl WritePacket,
        pos: ChunkPos,
        info: &ChunkLayerInfo,
    ) {
        if info.cached_chunk_egress {
            self.write_init_packets_cached(&mut writer, pos, info);
        } else {
            self.write_init_packets_uncached(&mut writer, pos, info);
        }
    }

    fn write_init_packets_cached(
        &self,
        writer: &mut impl WritePacket,
        pos: ChunkPos,
        info: &ChunkLayerInfo,
    ) {
        let probe = self.cache_probe(pos, info);
        let mut cached = self.cached_init_packet.lock();

        if let Some(bytes) = egress_cache::cached_packet_bytes(cached.as_ref(), &probe) {
            writer.write_packet_bytes(bytes);
            return;
        }

        let packet = self.render_init_packet(pos, info);
        writer.write_packet_bytes(packet.bytes());
        *cached = Some(packet);
    }

    fn write_init_packets_uncached(
        &self,
        writer: &mut impl WritePacket,
        pos: ChunkPos,
        info: &ChunkLayerInfo,
    ) {
        let packet = self.render_init_packet(pos, info);
        writer.write_packet_bytes(packet.bytes());
    }

    fn render_init_packet(&self, pos: ChunkPos, info: &ChunkLayerInfo) -> CachedChunkPacket {
        egress_cache::render_chunk_packet(self.init_packet_snapshot(pos, info))
            .expect("chunk init packet render should have complete inputs")
    }

    fn cache_probe<'a>(
        &'a self,
        pos: ChunkPos,
        info: &'a ChunkLayerInfo,
    ) -> ChunkEgressCacheProbe<'a> {
        ChunkEgressCacheProbe {
            pos,
            protocol_version: valence_protocol::PROTOCOL_VERSION,
            dimension_type_name: info.dimension_type_name.as_str(),
            height: info.height,
            min_y: info.min_y,
            biome_registry_len: info.biome_registry_len,
            compression_threshold: info.threshold,
            content_version: self.content_version,
            light_fingerprint: Some(egress_cache::empty_light_fingerprint()),
        }
    }

    fn init_packet_snapshot<'a>(
        &'a self,
        pos: ChunkPos,
        info: &'a ChunkLayerInfo,
    ) -> ChunkEgressSnapshot<'a> {
        let heightmaps = compound! {
            "MOTION_BLOCKING" => LoadedChunk::encode_heightmap(self.motion_blocking()),
            // TODO Implement `WORLD_SURFACE` (or explain why we don't need it)
            // "WORLD_SURFACE" => self.encode_heightmap(self.world_surface()),
        };

        let mut blocks_and_biomes: Vec<u8> = vec![];

        for sect in &self.sections {
            sect.count_non_air_blocks()
                .encode(&mut blocks_and_biomes)
                .unwrap();

            sect.block_states
                .encode_mc_format(
                    &mut blocks_and_biomes,
                    |b| b.to_raw().into(),
                    BLOCK_PALETTE_MIN_INDIRECT_BITS,
                    BLOCK_PALETTE_MAX_INDIRECT_BITS,
                    bit_width(BlockState::max_raw().into()),
                )
                .expect("paletted container encode should always succeed");

            sect.biomes
                .encode_mc_format(
                    &mut blocks_and_biomes,
                    |b| b.to_index() as u64,
                    BIOME_PALETTE_MIN_INDIRECT_BITS,
                    BIOME_PALETTE_MAX_INDIRECT_BITS,
                    bit_width(info.biome_registry_len - 1),
                )
                .expect("paletted container encode should always succeed");
        }

        let block_entities: Vec<_> = self
            .block_entities
            .iter()
            .filter_map(|(&idx, nbt)| {
                let x = idx % 16;
                let z = idx / 16 % 16;
                let y = idx / 16 / 16;

                let kind = self.sections[y as usize / 16]
                    .block_states
                    .get(idx as usize % SECTION_BLOCK_COUNT)
                    .block_entity_kind();

                kind.map(|kind| ChunkDataBlockEntity {
                    packed_xz: ((x << 4) | z) as i8,
                    y: y as i16 + info.min_y as i16,
                    kind,
                    data: Cow::Borrowed(nbt),
                })
            })
            .collect();

        ChunkEgressSnapshot {
            probe: self.cache_probe(pos, info),
            heightmaps,
            blocks_and_biomes,
            block_entities,
        }
    }

    fn invalidate_init_packet_cache(&mut self) {
        invalidate_init_packet_cache_parts(
            &mut self.content_version,
            self.cached_init_packet.get_mut(),
        );
    }

    /// Asserts that no changes to this chunk are currently recorded.
    #[track_caller]
    fn assert_no_changes(&self) {
        #[cfg(debug_assertions)]
        {
            assert!(!self.changed_biomes);
            assert!(self.changed_block_entities.is_empty());

            for sect in &self.sections {
                assert!(sect.updates.is_empty());
            }
        }
    }
}

fn invalidate_init_packet_cache_parts(
    content_version: &mut u64,
    cached_init_packet: &mut Option<CachedChunkPacket>,
) {
    *content_version = content_version
        .checked_add(CONTENT_VERSION_INCREMENT)
        .expect("chunk content version should not overflow");
    cached_init_packet.take();
}

impl Chunk for LoadedChunk {
    fn height(&self) -> u32 {
        self.sections.len() as u32 * 16
    }

    fn block_state(&self, x: u32, y: u32, z: u32) -> BlockState {
        check_block_oob(self, x, y, z);

        let idx = x + z * 16 + y % 16 * 16 * 16;
        self.sections[y as usize / 16]
            .block_states
            .get(idx as usize)
    }

    fn set_block_state(&mut self, x: u32, y: u32, z: u32, block: BlockState) -> BlockState {
        check_block_oob(self, x, y, z);

        let sect_y = y / 16;
        let sect = &mut self.sections[sect_y as usize];
        let idx = x + z * 16 + y % 16 * 16 * 16;

        let old_block = sect.block_states.set(idx as usize, block);

        if block != old_block {
            invalidate_init_packet_cache_parts(
                &mut self.content_version,
                self.cached_init_packet.get_mut(),
            );

            if *self.viewer_count.get_mut() > 0 {
                sect.updates.push(
                    ChunkDeltaUpdateEntry::new()
                        .with_off_x(x as u8)
                        .with_off_y((y % 16) as u8)
                        .with_off_z(z as u8)
                        .with_block_state(block.to_raw().into()),
                );
            }
        }

        old_block
    }

    fn fill_block_state_section(&mut self, sect_y: u32, block: BlockState) {
        check_section_oob(self, sect_y);

        let sect = &mut self.sections[sect_y as usize];

        if let PalettedContainer::Single(b) = &sect.block_states {
            if *b != block {
                invalidate_init_packet_cache_parts(
                    &mut self.content_version,
                    self.cached_init_packet.get_mut(),
                );

                if *self.viewer_count.get_mut() > 0 {
                    // The whole section is being modified, so any previous modifications would
                    // be overwritten.
                    sect.updates.clear();

                    // Push section updates for all the blocks in the section.
                    sect.updates.reserve_exact(SECTION_BLOCK_COUNT);
                    for z in 0..16 {
                        for x in 0..16 {
                            for y in 0..16 {
                                sect.updates.push(
                                    ChunkDeltaUpdateEntry::new()
                                        .with_off_x(x)
                                        .with_off_y(y)
                                        .with_off_z(z)
                                        .with_block_state(block.to_raw().into()),
                                );
                            }
                        }
                    }
                }
            }
        } else {
            for z in 0..16 {
                for x in 0..16 {
                    for y in 0..16 {
                        let idx = x + z * 16 + (sect_y * 16 + y) * (16 * 16);

                        if block != sect.block_states.get(idx as usize) {
                            invalidate_init_packet_cache_parts(
                                &mut self.content_version,
                                self.cached_init_packet.get_mut(),
                            );

                            if *self.viewer_count.get_mut() > 0 {
                                sect.updates.push(
                                    ChunkDeltaUpdateEntry::new()
                                        .with_off_x(x as u8)
                                        .with_off_y(y as u8)
                                        .with_off_z(z as u8)
                                        .with_block_state(block.to_raw().into()),
                                );
                            }
                        }
                    }
                }
            }
        }

        sect.block_states.fill(block);
    }

    fn block_entity(&self, x: u32, y: u32, z: u32) -> Option<&Compound> {
        check_block_oob(self, x, y, z);

        let idx = x + z * 16 + y * 16 * 16;
        self.block_entities.get(&idx)
    }

    fn block_entity_mut(&mut self, x: u32, y: u32, z: u32) -> Option<&mut Compound> {
        check_block_oob(self, x, y, z);

        let idx = x + z * 16 + y * 16 * 16;

        if let Some(be) = self.block_entities.get_mut(&idx) {
            if *self.viewer_count.get_mut() > 0 {
                self.changed_block_entities.insert(idx);
            }
            invalidate_init_packet_cache_parts(
                &mut self.content_version,
                self.cached_init_packet.get_mut(),
            );

            Some(be)
        } else {
            None
        }
    }

    fn set_block_entity(
        &mut self,
        x: u32,
        y: u32,
        z: u32,
        block_entity: Option<Compound>,
    ) -> Option<Compound> {
        check_block_oob(self, x, y, z);

        let idx = x + z * 16 + y * 16 * 16;

        match block_entity {
            Some(nbt) => {
                if *self.viewer_count.get_mut() > 0 {
                    self.changed_block_entities.insert(idx);
                }
                self.invalidate_init_packet_cache();

                self.block_entities.insert(idx, nbt)
            }
            None => {
                let res = self.block_entities.remove(&idx);

                if res.is_some() {
                    self.invalidate_init_packet_cache();
                }

                res
            }
        }
    }

    fn clear_block_entities(&mut self) {
        if self.block_entities.is_empty() {
            return;
        }

        self.invalidate_init_packet_cache();

        if *self.viewer_count.get_mut() > 0 {
            self.changed_block_entities
                .extend(mem::take(&mut self.block_entities).into_keys());
        } else {
            self.block_entities.clear();
        }
    }

    fn biome(&self, x: u32, y: u32, z: u32) -> BiomeId {
        check_biome_oob(self, x, y, z);

        let idx = x + z * 4 + y % 4 * 4 * 4;
        self.sections[y as usize / 4].biomes.get(idx as usize)
    }

    fn set_biome(&mut self, x: u32, y: u32, z: u32, biome: BiomeId) -> BiomeId {
        check_biome_oob(self, x, y, z);

        let idx = x + z * 4 + y % 4 * 4 * 4;
        let old_biome = self.sections[y as usize / 4]
            .biomes
            .set(idx as usize, biome);

        if biome != old_biome {
            self.invalidate_init_packet_cache();

            if *self.viewer_count.get_mut() > 0 {
                self.changed_biomes = true;
            }
        }

        old_biome
    }

    fn fill_biome_section(&mut self, sect_y: u32, biome: BiomeId) {
        check_section_oob(self, sect_y);

        let sect = &mut self.sections[sect_y as usize];

        if let PalettedContainer::Single(b) = &sect.biomes {
            if *b != biome {
                invalidate_init_packet_cache_parts(
                    &mut self.content_version,
                    self.cached_init_packet.get_mut(),
                );
                self.changed_biomes = *self.viewer_count.get_mut() > 0;
            }
        } else {
            invalidate_init_packet_cache_parts(
                &mut self.content_version,
                self.cached_init_packet.get_mut(),
            );
            self.changed_biomes = *self.viewer_count.get_mut() > 0;
        }

        sect.biomes.fill(biome);
    }

    fn shrink_to_fit(&mut self) {
        if let Some(packet) = self.cached_init_packet.get_mut() {
            packet.shrink_to_fit();
        }

        for sect in &mut self.sections {
            sect.block_states.shrink_to_fit();
            sect.biomes.shrink_to_fit();
            sect.updates.shrink_to_fit();
        }
    }
}

#[cfg(test)]
mod tests {
    use valence_protocol::{ident, CompressionThreshold};

    use super::*;

    #[test]
    fn loaded_chunk_unviewed_no_changes() {
        let mut chunk = LoadedChunk::new(512);

        chunk.set_block(0, 10, 0, BlockState::MAGMA_BLOCK);
        chunk.assert_no_changes();

        chunk.set_biome(0, 0, 0, BiomeId::from_index(5));
        chunk.assert_no_changes();

        chunk.fill_block_states(BlockState::ACACIA_BUTTON);
        chunk.assert_no_changes();

        chunk.fill_biomes(BiomeId::from_index(42));
        chunk.assert_no_changes();
    }

    #[test]
    fn loaded_chunk_default_uncached_writer_keeps_packet_cache_empty() {
        let mut chunk = LoadedChunk::new(TEST_CHUNK_HEIGHT);
        let info = test_info(false);
        let mut buf = vec![];
        let mut writer = PacketWriter::new(&mut buf, CompressionThreshold::DEFAULT);

        chunk.write_init_packets(&mut writer, test_pos(), &info);

        assert!(!buf.is_empty());
        assert!(chunk.cached_init_packet.get_mut().is_none());
    }

    #[test]
    fn loaded_chunk_cache_hit_reuses_identical_packet_bytes() {
        let mut chunk = LoadedChunk::new(TEST_CHUNK_HEIGHT);
        let info = test_info(true);
        let mut first_buf = vec![];
        let mut first_writer = PacketWriter::new(&mut first_buf, CompressionThreshold::DEFAULT);
        let mut second_buf = vec![];
        let mut second_writer = PacketWriter::new(&mut second_buf, CompressionThreshold::DEFAULT);

        chunk.write_init_packets(&mut first_writer, test_pos(), &info);
        assert!(chunk.cached_init_packet.get_mut().is_some());

        chunk.write_init_packets(&mut second_writer, test_pos(), &info);

        assert_eq!(first_buf, second_buf);
        assert!(chunk.cached_init_packet.get_mut().is_some());
    }

    #[test]
    fn loaded_chunk_changes_invalidate_optional_packet_cache() {
        #[track_caller]
        fn check<T>(chunk: &mut LoadedChunk, change: impl FnOnce(&mut LoadedChunk) -> T) {
            let info = test_info(true);
            let mut buf = vec![];
            let mut writer = PacketWriter::new(&mut buf, CompressionThreshold::DEFAULT);

            // Rebuild cache.
            chunk.write_init_packets(&mut writer, test_pos(), &info);

            // Check that the cache is built.
            assert!(chunk.cached_init_packet.get_mut().is_some());

            // Making a change should clear the cache.
            change(chunk);
            assert!(chunk.cached_init_packet.get_mut().is_none());

            // Rebuild cache again.
            chunk.write_init_packets(&mut writer, test_pos(), &info);
            assert!(chunk.cached_init_packet.get_mut().is_some());
        }

        let mut chunk = LoadedChunk::new(TEST_CHUNK_HEIGHT);

        check(&mut chunk, |c| {
            c.set_block_state(
                TEST_BLOCK_X,
                TEST_BLOCK_Y,
                TEST_BLOCK_Z,
                BlockState::ACACIA_WOOD,
            )
        });
        check(&mut chunk, |c| {
            c.set_biome(
                TEST_BIOME_X,
                TEST_BIOME_Y,
                TEST_BIOME_Z,
                BiomeId::from_index(TEST_BIOME_ID),
            )
        });
        check(&mut chunk, |c| c.fill_biomes(BiomeId::DEFAULT));
        check(&mut chunk, |c| c.fill_block_states(BlockState::WET_SPONGE));
        check(&mut chunk, |c| {
            c.set_block_entity(
                TEST_BLOCK_ENTITY_X,
                TEST_BLOCK_ENTITY_Y,
                TEST_BLOCK_ENTITY_Z,
                Some(compound! {}),
            )
        });
        check(&mut chunk, |c| {
            c.block_entity_mut(
                TEST_BLOCK_ENTITY_X,
                TEST_BLOCK_ENTITY_Y,
                TEST_BLOCK_ENTITY_Z,
            )
            .unwrap();
        });
        check(&mut chunk, |c| {
            c.set_block_entity(
                TEST_BLOCK_ENTITY_X,
                TEST_BLOCK_ENTITY_Y,
                TEST_BLOCK_ENTITY_Z,
                None,
            )
        });

        // Old block state is the same as new block state, so the cache should still be
        // intact.
        assert_eq!(
            chunk.set_block_state(
                TEST_AIR_BLOCK_X,
                TEST_AIR_BLOCK_Y,
                TEST_AIR_BLOCK_Z,
                BlockState::WET_SPONGE
            ),
            BlockState::WET_SPONGE
        );

        assert!(chunk.cached_init_packet.get_mut().is_some());
    }

    #[test]
    fn section_non_air_count_tracks_storage_transitions() {
        let mut section = Section::default();
        let section_block_count = u16::try_from(SECTION_BLOCK_COUNT).unwrap();

        assert_eq!(0, section.count_non_air_blocks());

        section.block_states.fill(BlockState::STONE);
        assert_eq!(section_block_count, section.count_non_air_blocks());

        assert_eq!(
            BlockState::STONE,
            section.block_states.set(0, BlockState::AIR)
        );
        assert_eq!(section_block_count - 1, section.count_non_air_blocks());

        let mut direct_section = Section::default();
        for (idx, block) in DIRECT_FALLBACK_BLOCKS.iter().copied().enumerate() {
            direct_section.block_states.set(idx, block);
        }

        assert!(matches!(
            direct_section.block_states,
            PalettedContainer::Direct(_)
        ));
        assert_eq!(
            direct_fallback_non_air_count(),
            direct_section.count_non_air_blocks()
        );

        direct_section.block_states.fill(BlockState::AIR);
        assert_eq!(0, direct_section.count_non_air_blocks());
    }

    const DIRECT_FALLBACK_BLOCK_COUNT: usize = 17;
    const DIRECT_FALLBACK_BLOCKS: [BlockState; DIRECT_FALLBACK_BLOCK_COUNT] = [
        BlockState::AIR,
        BlockState::STONE,
        BlockState::DIRT,
        BlockState::GRASS_BLOCK,
        BlockState::OAK_LOG,
        BlockState::OAK_PLANKS,
        BlockState::COBBLESTONE,
        BlockState::SAND,
        BlockState::GLASS,
        BlockState::WATER,
        BlockState::LAVA,
        BlockState::GOLD_BLOCK,
        BlockState::IRON_BLOCK,
        BlockState::DIAMOND_BLOCK,
        BlockState::EMERALD_BLOCK,
        BlockState::REDSTONE_BLOCK,
        BlockState::COAL_BLOCK,
    ];

    fn direct_fallback_non_air_count() -> u16 {
        DIRECT_FALLBACK_BLOCKS
            .iter()
            .copied()
            .filter(|block| !block.is_air())
            .count()
            .try_into()
            .unwrap()
    }

    const TEST_CHUNK_HEIGHT: u32 = 512;
    const TEST_DIMENSION_MIN_Y: i32 = -16;
    const TEST_BIOME_REGISTRY_LEN: usize = 200;
    const TEST_CHUNK_X: i32 = 3;
    const TEST_CHUNK_Z: i32 = 4;
    const TEST_BLOCK_X: u32 = 0;
    const TEST_BLOCK_Y: u32 = 4;
    const TEST_BLOCK_Z: u32 = 0;
    const TEST_BIOME_X: u32 = 1;
    const TEST_BIOME_Y: u32 = 2;
    const TEST_BIOME_Z: u32 = 3;
    const TEST_BIOME_ID: usize = 4;
    const TEST_BLOCK_ENTITY_X: u32 = 3;
    const TEST_BLOCK_ENTITY_Y: u32 = 40;
    const TEST_BLOCK_ENTITY_Z: u32 = 5;
    const TEST_AIR_BLOCK_X: u32 = 0;
    const TEST_AIR_BLOCK_Y: u32 = 0;
    const TEST_AIR_BLOCK_Z: u32 = 0;

    fn test_info(cached_chunk_egress: bool) -> ChunkLayerInfo {
        ChunkLayerInfo {
            dimension_type_name: ident!("whatever").into(),
            height: TEST_CHUNK_HEIGHT,
            min_y: TEST_DIMENSION_MIN_Y,
            biome_registry_len: TEST_BIOME_REGISTRY_LEN,
            threshold: CompressionThreshold::DEFAULT,
            cached_chunk_egress,
        }
    }

    fn test_pos() -> ChunkPos {
        ChunkPos::new(TEST_CHUNK_X, TEST_CHUNK_Z)
    }
}
