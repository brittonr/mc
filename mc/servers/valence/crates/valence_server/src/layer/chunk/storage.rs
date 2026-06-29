use std::borrow::Cow;
use std::collections::hash_map::Entry;
use std::fmt;

use bevy_ecs::prelude::*;
use rustc_hash::FxHashMap;
use valence_math::{DVec3, Vec3};
use valence_nbt::Compound;
use valence_protocol::encode::WritePacket;
use valence_protocol::packets::play::particle_s2c::Particle;
use valence_protocol::packets::play::{ParticleS2c, PlaySoundS2c};
use valence_protocol::sound::{Sound, SoundCategory, SoundId};
use valence_protocol::{BiomePos, BlockPos, ChunkPos, CompressionThreshold, Ident};
use valence_registry::biome::{BiomeId, BiomeRegistry};
use valence_registry::DimensionTypeRegistry;
use valence_server_common::Server;

use crate::layer::Layer;

use super::{
    Block, BlockRef, Chunk, ChunkEntry, ChunkLayerMessages, IntoBlock, LoadedChunk, LocalMsg,
    OccupiedChunkEntry, UnloadedChunk, VacantChunkEntry, MAX_HEIGHT,
};
use crate::layer::message::Messages;

const CHUNK_BLOCK_AXIS_LENGTH: i32 = 16;
const BIOME_BLOCK_AXIS_LENGTH: i32 = 4;
const BIOME_BLOCK_HEIGHT_SCALE: u32 = 4;
const SOUND_POSITION_FIXED_POINT_SCALE: f64 = 8.0;

/// A [`Component`] containing the [chunks](LoadedChunk) and [dimension
/// information](valence_registry::dimension_type::DimensionTypeId) of a
/// Minecraft world.
#[derive(Component, Debug)]
pub struct ChunkLayer {
    pub(super) messages: ChunkLayerMessages,
    pub(super) chunks: FxHashMap<ChunkPos, LoadedChunk>,
    pub(super) info: ChunkLayerInfo,
}

/// Chunk layer information.
pub(crate) struct ChunkLayerInfo {
    pub(super) dimension_type_name: Ident<String>,
    pub(super) height: u32,
    pub(super) min_y: i32,
    pub(super) biome_registry_len: usize,
    pub(super) threshold: CompressionThreshold,
    pub(super) cached_chunk_egress: bool,
}

impl fmt::Debug for ChunkLayerInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ChunkLayerInfo")
            .field("dimension_type_name", &self.dimension_type_name)
            .field("height", &self.height)
            .field("min_y", &self.min_y)
            .field("biome_registry_len", &self.biome_registry_len)
            .field("threshold", &self.threshold)
            .field("cached_chunk_egress", &self.cached_chunk_egress)
            // Ignore sky light mask and array.
            .finish()
    }
}

impl ChunkLayer {
    pub(crate) const LOAD: u8 = 0;
    pub(crate) const UNLOAD: u8 = 1;
    pub(crate) const OVERWRITE: u8 = 2;

    /// Creates a new chunk layer.
    #[track_caller]
    pub fn new<N: Into<Ident<String>>>(
        dimension_type_name: N,
        dimensions: &DimensionTypeRegistry,
        biomes: &BiomeRegistry,
        server: &Server,
    ) -> Self {
        let dimension_type_name = dimension_type_name.into();

        let dim = &dimensions[dimension_type_name.as_str_ident()];

        assert!(
            (0..MAX_HEIGHT as i32).contains(&dim.height),
            "invalid dimension height of {}",
            dim.height
        );

        Self {
            messages: Messages::new(),
            chunks: Default::default(),
            info: ChunkLayerInfo {
                dimension_type_name,
                height: dim.height as u32,
                min_y: dim.min_y,
                biome_registry_len: biomes.iter().len(),
                threshold: server.compression_threshold(),
                cached_chunk_egress: false,
            },
        }
    }

    /// The name of the dimension this chunk layer is using.
    pub fn dimension_type_name(&self) -> Ident<&str> {
        self.info.dimension_type_name.as_str_ident()
    }

    /// The height of this instance's dimension.
    pub fn height(&self) -> u32 {
        self.info.height
    }

    /// The `min_y` of this instance's dimension.
    pub fn min_y(&self) -> i32 {
        self.info.min_y
    }

    /// Returns `true` when chunk initialization packets may use the keyed cached egress path.
    pub fn cached_chunk_egress_enabled(&self) -> bool {
        self.info.cached_chunk_egress
    }

    /// Selects whether chunk initialization packets may use the keyed cached egress path.
    pub fn set_cached_chunk_egress_enabled(&mut self, enabled: bool) {
        self.info.cached_chunk_egress = enabled;
    }

    /// Enables the keyed cached chunk egress path for future chunk initialization sends.
    pub fn enable_cached_chunk_egress(&mut self) {
        self.set_cached_chunk_egress_enabled(true);
    }

    /// Disables the keyed cached chunk egress path and keeps future sends on the uncached renderer.
    pub fn disable_cached_chunk_egress(&mut self) {
        self.set_cached_chunk_egress_enabled(false);
    }

    /// Get a reference to the chunk at the given position, if it is loaded.
    pub fn chunk<P: Into<ChunkPos>>(&self, pos: P) -> Option<&LoadedChunk> {
        self.chunks.get(&pos.into())
    }

    /// Get a mutable reference to the chunk at the given position, if it is
    /// loaded.
    pub fn chunk_mut<P: Into<ChunkPos>>(&mut self, pos: P) -> Option<&mut LoadedChunk> {
        self.chunks.get_mut(&pos.into())
    }

    /// Insert a chunk into the instance at the given position. The previous
    /// chunk data is returned.
    pub fn insert_chunk<P: Into<ChunkPos>>(
        &mut self,
        pos: P,
        chunk: UnloadedChunk,
    ) -> Option<UnloadedChunk> {
        match self.chunk_entry(pos) {
            ChunkEntry::Occupied(mut oe) => Some(oe.insert(chunk)),
            ChunkEntry::Vacant(ve) => {
                ve.insert(chunk);
                None
            }
        }
    }

    /// Unload the chunk at the given position, if it is loaded. Returns the
    /// chunk if it was loaded.
    pub fn remove_chunk<P: Into<ChunkPos>>(&mut self, pos: P) -> Option<UnloadedChunk> {
        match self.chunk_entry(pos) {
            ChunkEntry::Occupied(oe) => Some(oe.remove()),
            ChunkEntry::Vacant(_) => None,
        }
    }

    /// Unload all chunks in this instance.
    pub fn clear_chunks(&mut self) {
        self.retain_chunks(|_, _| false)
    }

    /// Retain only the chunks for which the given predicate returns `true`.
    pub fn retain_chunks<F>(&mut self, mut f: F)
    where
        F: FnMut(ChunkPos, &mut LoadedChunk) -> bool,
    {
        self.chunks.retain(|pos, chunk| {
            if !f(*pos, chunk) {
                self.messages
                    .send_local_infallible(LocalMsg::ChangeChunkState { pos: *pos }, |b| {
                        b.push(Self::UNLOAD)
                    });

                false
            } else {
                true
            }
        });
    }

    /// Get a [`ChunkEntry`] for the given position.
    pub fn chunk_entry<P: Into<ChunkPos>>(&mut self, pos: P) -> ChunkEntry<'_> {
        match self.chunks.entry(pos.into()) {
            Entry::Occupied(oe) => ChunkEntry::Occupied(OccupiedChunkEntry {
                messages: &mut self.messages,
                entry: oe,
            }),
            Entry::Vacant(ve) => ChunkEntry::Vacant(VacantChunkEntry {
                height: self.info.height,
                messages: &mut self.messages,
                entry: ve,
            }),
        }
    }

    /// Get an iterator over all loaded chunks in the instance. The order of the
    /// chunks is undefined.
    pub fn chunks(&self) -> impl Iterator<Item = (ChunkPos, &LoadedChunk)> + Clone + '_ {
        self.chunks.iter().map(|(pos, chunk)| (*pos, chunk))
    }

    /// Get an iterator over all loaded chunks in the instance, mutably. The
    /// order of the chunks is undefined.
    pub fn chunks_mut(&mut self) -> impl Iterator<Item = (ChunkPos, &mut LoadedChunk)> + '_ {
        self.chunks.iter_mut().map(|(pos, chunk)| (*pos, chunk))
    }

    /// Optimizes the memory usage of the instance.
    pub fn shrink_to_fit(&mut self) {
        for (_, chunk) in self.chunks_mut() {
            chunk.shrink_to_fit();
        }

        self.chunks.shrink_to_fit();
        self.messages.shrink_to_fit();
    }

    pub fn block<P: Into<BlockPos>>(&self, pos: P) -> Option<BlockRef<'_>> {
        let pos = pos.into();

        let y = pos
            .y
            .checked_sub(self.info.min_y)
            .and_then(|y| y.try_into().ok())?;

        if y >= self.info.height {
            return None;
        }

        let chunk = self.chunk(pos)?;

        let x = pos.x.rem_euclid(CHUNK_BLOCK_AXIS_LENGTH) as u32;
        let z = pos.z.rem_euclid(CHUNK_BLOCK_AXIS_LENGTH) as u32;

        Some(chunk.block(x, y, z))
    }

    pub fn set_block<P, B>(&mut self, pos: P, block: B) -> Option<Block>
    where
        P: Into<BlockPos>,
        B: IntoBlock,
    {
        let pos = pos.into();

        let y = pos
            .y
            .checked_sub(self.info.min_y)
            .and_then(|y| y.try_into().ok())?;

        if y >= self.info.height {
            return None;
        }

        let chunk = self.chunk_mut(pos)?;

        let x = pos.x.rem_euclid(CHUNK_BLOCK_AXIS_LENGTH) as u32;
        let z = pos.z.rem_euclid(CHUNK_BLOCK_AXIS_LENGTH) as u32;

        Some(chunk.set_block(x, y, z, block))
    }

    pub fn block_entity_mut<P: Into<BlockPos>>(&mut self, pos: P) -> Option<&mut Compound> {
        let pos = pos.into();

        let y = pos
            .y
            .checked_sub(self.info.min_y)
            .and_then(|y| y.try_into().ok())?;

        if y >= self.info.height {
            return None;
        }

        let chunk = self.chunk_mut(pos)?;

        let x = pos.x.rem_euclid(CHUNK_BLOCK_AXIS_LENGTH) as u32;
        let z = pos.z.rem_euclid(CHUNK_BLOCK_AXIS_LENGTH) as u32;

        chunk.block_entity_mut(x, y, z)
    }

    pub fn biome<P: Into<BiomePos>>(&self, pos: P) -> Option<BiomeId> {
        let pos = pos.into();

        let y = pos
            .y
            .checked_sub(self.info.min_y / BIOME_BLOCK_AXIS_LENGTH)
            .and_then(|y| y.try_into().ok())?;

        if y >= self.info.height / BIOME_BLOCK_HEIGHT_SCALE {
            return None;
        }

        let chunk = self.chunk(pos)?;

        let x = pos.x.rem_euclid(BIOME_BLOCK_AXIS_LENGTH) as u32;
        let z = pos.z.rem_euclid(BIOME_BLOCK_AXIS_LENGTH) as u32;

        Some(chunk.biome(x, y, z))
    }

    pub fn set_biome<P: Into<BiomePos>>(&mut self, pos: P, biome: BiomeId) -> Option<BiomeId> {
        let pos = pos.into();

        let y = pos
            .y
            .checked_sub(self.info.min_y / BIOME_BLOCK_AXIS_LENGTH)
            .and_then(|y| y.try_into().ok())?;

        if y >= self.info.height / BIOME_BLOCK_HEIGHT_SCALE {
            return None;
        }

        let chunk = self.chunk_mut(pos)?;

        let x = pos.x.rem_euclid(BIOME_BLOCK_AXIS_LENGTH) as u32;
        let z = pos.z.rem_euclid(BIOME_BLOCK_AXIS_LENGTH) as u32;

        Some(chunk.set_biome(x, y, z, biome))
    }

    pub(crate) fn info(&self) -> &ChunkLayerInfo {
        &self.info
    }

    pub(crate) fn messages(&self) -> &ChunkLayerMessages {
        &self.messages
    }

    // TODO: move to `valence_particle`.
    /// Puts a particle effect at the given position in the world. The particle
    /// effect is visible to all players in the instance with the
    /// appropriate chunk in view.
    pub fn play_particle<P, O>(
        &mut self,
        particle: &Particle,
        long_distance: bool,
        position: P,
        offset: O,
        max_speed: f32,
        count: i32,
    ) where
        P: Into<DVec3>,
        O: Into<Vec3>,
    {
        let position = position.into();

        self.view_writer(position).write_packet(&ParticleS2c {
            particle: Cow::Borrowed(particle),
            long_distance,
            position,
            offset: offset.into(),
            max_speed,
            count,
        });
    }

    // TODO: move to `valence_sound`.
    /// Plays a sound effect at the given position in the world. The sound
    /// effect is audible to all players in the instance with the
    /// appropriate chunk in view.
    pub fn play_sound<P: Into<DVec3>>(
        &mut self,
        sound: Sound,
        category: SoundCategory,
        position: P,
        volume: f32,
        pitch: f32,
    ) {
        let position = position.into();

        self.view_writer(position).write_packet(&PlaySoundS2c {
            id: SoundId::Direct {
                id: sound.to_ident().into(),
                range: None,
            },
            category,
            position: (position * SOUND_POSITION_FIXED_POINT_SCALE).as_ivec3(),
            volume,
            pitch,
            seed: rand::random(),
        });
    }
}
