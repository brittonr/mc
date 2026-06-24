use std::borrow::Cow;

use valence_nbt::Compound;
use valence_protocol::encode::{PacketWriter, WritePacket};
use valence_protocol::packets::play::chunk_data_s2c::ChunkDataBlockEntity;
use valence_protocol::packets::play::ChunkDataS2c;
use valence_protocol::{ChunkPos, CompressionThreshold, Encode};

const CHUNK_EGRESS_CONTENT_HASH_CONTEXT: &[u8] = b"valence.chunk_egress.content.v1";
const CHUNK_EGRESS_EMPTY_LIGHT_CONTEXT: &[u8] = b"valence.chunk_egress.empty_light.v1";
const HASH_FIELD_SEPARATOR: &[u8] = b"\0";

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct ChunkEgressCacheKey {
    pub(super) pos: ChunkPos,
    pub(super) protocol_version: i32,
    pub(super) dimension_type_name: String,
    pub(super) height: u32,
    pub(super) min_y: i32,
    pub(super) biome_registry_len: usize,
    pub(super) compression_threshold: CompressionThreshold,
    pub(super) content_version: u64,
    pub(super) content_fingerprint: blake3::Hash,
    pub(super) light_fingerprint: blake3::Hash,
}

#[derive(Copy, Clone, Debug)]
pub(super) struct ChunkEgressCacheProbe<'a> {
    pub(super) pos: ChunkPos,
    pub(super) protocol_version: i32,
    pub(super) dimension_type_name: &'a str,
    pub(super) height: u32,
    pub(super) min_y: i32,
    pub(super) biome_registry_len: usize,
    pub(super) compression_threshold: CompressionThreshold,
    pub(super) content_version: u64,
    pub(super) light_fingerprint: Option<blake3::Hash>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct CachedChunkPacket {
    key: ChunkEgressCacheKey,
    bytes: Vec<u8>,
}

#[derive(Debug)]
pub(super) struct ChunkEgressSnapshot<'a> {
    pub(super) probe: ChunkEgressCacheProbe<'a>,
    pub(super) heightmaps: Compound,
    pub(super) blocks_and_biomes: Vec<u8>,
    pub(super) block_entities: Vec<ChunkDataBlockEntity<'a>>,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub(super) enum ChunkEgressError {
    MissingLightInputs,
    EncodeFailed,
}

impl ChunkEgressCacheKey {
    fn matches_probe(&self, probe: &ChunkEgressCacheProbe<'_>) -> bool {
        self.pos == probe.pos
            && self.protocol_version == probe.protocol_version
            && self.dimension_type_name == probe.dimension_type_name
            && self.height == probe.height
            && self.min_y == probe.min_y
            && self.biome_registry_len == probe.biome_registry_len
            && self.compression_threshold == probe.compression_threshold
            && self.content_version == probe.content_version
            && Some(self.light_fingerprint) == probe.light_fingerprint
    }
}

impl ChunkEgressCacheProbe<'_> {
    fn into_key(
        self,
        content_fingerprint: blake3::Hash,
        light_fingerprint: blake3::Hash,
    ) -> ChunkEgressCacheKey {
        ChunkEgressCacheKey {
            pos: self.pos,
            protocol_version: self.protocol_version,
            dimension_type_name: self.dimension_type_name.to_owned(),
            height: self.height,
            min_y: self.min_y,
            biome_registry_len: self.biome_registry_len,
            compression_threshold: self.compression_threshold,
            content_version: self.content_version,
            content_fingerprint,
            light_fingerprint,
        }
    }
}

impl CachedChunkPacket {
    pub(super) fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub(super) fn shrink_to_fit(&mut self) {
        self.bytes.shrink_to_fit();
    }
}

pub(super) fn cached_packet_bytes<'a>(
    cached: Option<&'a CachedChunkPacket>,
    probe: &ChunkEgressCacheProbe<'_>,
) -> Option<&'a [u8]> {
    let cached = cached?;

    if cached.key.matches_probe(probe) {
        Some(cached.bytes())
    } else {
        None
    }
}

pub(super) fn empty_light_fingerprint() -> blake3::Hash {
    blake3::hash(CHUNK_EGRESS_EMPTY_LIGHT_CONTEXT)
}

pub(super) fn render_chunk_packet(
    snapshot: ChunkEgressSnapshot<'_>,
) -> Result<CachedChunkPacket, ChunkEgressError> {
    let light_fingerprint = snapshot
        .probe
        .light_fingerprint
        .ok_or(ChunkEgressError::MissingLightInputs)?;
    let content_fingerprint = fingerprint_chunk_content(&snapshot, light_fingerprint)?;
    let key = snapshot
        .probe
        .into_key(content_fingerprint, light_fingerprint);

    let mut bytes = Vec::new();
    PacketWriter::new(&mut bytes, snapshot.probe.compression_threshold)
        .write_packet_fallible(&ChunkDataS2c {
            pos: snapshot.probe.pos,
            heightmaps: Cow::Owned(snapshot.heightmaps),
            blocks_and_biomes: &snapshot.blocks_and_biomes,
            block_entities: Cow::Owned(snapshot.block_entities),
            sky_light_mask: Cow::Borrowed(&[]),
            block_light_mask: Cow::Borrowed(&[]),
            empty_sky_light_mask: Cow::Borrowed(&[]),
            empty_block_light_mask: Cow::Borrowed(&[]),
            sky_light_arrays: Cow::Borrowed(&[]),
            block_light_arrays: Cow::Borrowed(&[]),
        })
        .map_err(|_| ChunkEgressError::EncodeFailed)?;

    Ok(CachedChunkPacket { key, bytes })
}

fn fingerprint_chunk_content(
    snapshot: &ChunkEgressSnapshot<'_>,
    light_fingerprint: blake3::Hash,
) -> Result<blake3::Hash, ChunkEgressError> {
    let mut hasher = blake3::Hasher::new();
    hasher.update(CHUNK_EGRESS_CONTENT_HASH_CONTEXT);

    let mut heightmap_bytes = Vec::new();
    snapshot
        .heightmaps
        .encode(&mut heightmap_bytes)
        .map_err(|_| ChunkEgressError::EncodeFailed)?;
    update_hash_field(&mut hasher, b"heightmaps", &heightmap_bytes);
    update_hash_field(
        &mut hasher,
        b"blocks_and_biomes",
        &snapshot.blocks_and_biomes,
    );
    update_hash_len(
        &mut hasher,
        b"block_entity_count",
        snapshot.block_entities.len(),
    );

    let mut block_entity_bytes = Vec::new();
    for block_entity in &snapshot.block_entities {
        block_entity_bytes.clear();
        block_entity
            .encode(&mut block_entity_bytes)
            .map_err(|_| ChunkEgressError::EncodeFailed)?;
        update_hash_field(&mut hasher, b"block_entity", &block_entity_bytes);
    }

    update_hash_field(&mut hasher, b"light", light_fingerprint.as_bytes());

    Ok(hasher.finalize())
}

fn update_hash_len(hasher: &mut blake3::Hasher, label: &[u8], len: usize) {
    let len = u64::try_from(len).expect("chunk egress hash length should fit in u64");
    update_hash_field(hasher, label, &len.to_le_bytes());
}

fn update_hash_field(hasher: &mut blake3::Hasher, label: &[u8], bytes: &[u8]) {
    hasher.update(label);
    hasher.update(HASH_FIELD_SEPARATOR);
    update_hash_len_prefix(hasher, bytes.len());
    hasher.update(bytes);
}

fn update_hash_len_prefix(hasher: &mut blake3::Hasher, len: usize) {
    let len = u64::try_from(len).expect("chunk egress hash length prefix should fit in u64");
    hasher.update(&len.to_le_bytes());
}

#[cfg(test)]
mod tests {
    use valence_nbt::Compound;
    use valence_protocol::{ChunkPos, CompressionThreshold, PROTOCOL_VERSION};

    use super::*;

    const TEST_CHUNK_X: i32 = 3;
    const TEST_CHUNK_Z: i32 = -4;
    const TEST_DIMENSION_TYPE: &str = "minecraft:overworld";
    const TEST_HEIGHT: u32 = 384;
    const TEST_MIN_Y: i32 = -64;
    const TEST_BIOME_REGISTRY_LEN: usize = 64;
    const TEST_CONTENT_VERSION: u64 = 7;
    const TEST_CONTENT_VERSION_STEP: u64 = 1;
    const NEXT_CONTENT_VERSION: u64 = TEST_CONTENT_VERSION + TEST_CONTENT_VERSION_STEP;
    const TEST_PROTOCOL_VERSION_STEP: i32 = 1;
    const TEST_ALTERNATE_HEIGHT: u32 = 320;
    const TEST_ALTERNATE_MIN_Y: i32 = -32;
    const TEST_ALTERNATE_BIOME_REGISTRY_LEN: usize = 128;
    const TEST_COMPRESSION_THRESHOLD: i32 = 256;
    const TEST_BLOCKS_AND_BIOMES: &[u8] = b"blocks-and-biomes";
    const TEST_ALTERNATE_LIGHT_BYTES: &[u8] = b"alternate-light";
    const STALE_PACKET_BYTES: &[u8] = b"stale-bytes";

    #[test]
    fn same_snapshot_renders_same_packet_and_key() {
        let left = render_chunk_packet(test_snapshot()).unwrap();
        let right = render_chunk_packet(test_snapshot()).unwrap();

        assert_eq!(left.key, right.key);
        assert_eq!(left.bytes(), right.bytes());
    }

    #[test]
    fn cache_hit_returns_stored_bytes() {
        let packet = render_chunk_packet(test_snapshot()).unwrap();
        let probe = test_probe();

        assert_eq!(
            cached_packet_bytes(Some(&packet), &probe),
            Some(packet.bytes())
        );
    }

    #[test]
    fn compression_change_misses_cache() {
        let packet = render_chunk_packet(test_snapshot()).unwrap();
        let mut probe = test_probe();
        probe.compression_threshold = CompressionThreshold(TEST_COMPRESSION_THRESHOLD);

        assert!(cached_packet_bytes(Some(&packet), &probe).is_none());
    }

    #[test]
    fn dimension_change_misses_cache() {
        let packet = render_chunk_packet(test_snapshot()).unwrap();
        let mut probe = test_probe();
        probe.dimension_type_name = "minecraft:the_nether";

        assert!(cached_packet_bytes(Some(&packet), &probe).is_none());
    }

    #[test]
    fn render_setting_changes_miss_cache() {
        let packet = render_chunk_packet(test_snapshot()).unwrap();

        assert_probe_misses(&packet, |probe| {
            probe.protocol_version += TEST_PROTOCOL_VERSION_STEP;
        });
        assert_probe_misses(&packet, |probe| {
            probe.height = TEST_ALTERNATE_HEIGHT;
        });
        assert_probe_misses(&packet, |probe| {
            probe.min_y = TEST_ALTERNATE_MIN_Y;
        });
        assert_probe_misses(&packet, |probe| {
            probe.biome_registry_len = TEST_ALTERNATE_BIOME_REGISTRY_LEN;
        });
        assert_probe_misses(&packet, |probe| {
            probe.light_fingerprint = Some(blake3::hash(TEST_ALTERNATE_LIGHT_BYTES));
        });
    }

    #[test]
    fn content_version_change_misses_stale_bytes() {
        let mut packet = render_chunk_packet(test_snapshot()).unwrap();
        packet.bytes = STALE_PACKET_BYTES.to_vec();

        let mut probe = test_probe();
        probe.content_version = NEXT_CONTENT_VERSION;

        assert!(cached_packet_bytes(Some(&packet), &probe).is_none());
    }

    #[test]
    fn missing_light_inputs_fail_closed() {
        let mut snapshot = test_snapshot();
        snapshot.probe.light_fingerprint = None;

        assert_eq!(
            render_chunk_packet(snapshot).unwrap_err(),
            ChunkEgressError::MissingLightInputs
        );
    }

    fn test_snapshot() -> ChunkEgressSnapshot<'static> {
        ChunkEgressSnapshot {
            probe: test_probe(),
            heightmaps: Compound::new(),
            blocks_and_biomes: TEST_BLOCKS_AND_BIOMES.to_vec(),
            block_entities: Vec::new(),
        }
    }

    #[track_caller]
    fn assert_probe_misses(
        packet: &CachedChunkPacket,
        mutate: impl FnOnce(&mut ChunkEgressCacheProbe<'static>),
    ) {
        let mut probe = test_probe();
        mutate(&mut probe);

        assert!(cached_packet_bytes(Some(packet), &probe).is_none());
    }

    fn test_probe() -> ChunkEgressCacheProbe<'static> {
        ChunkEgressCacheProbe {
            pos: ChunkPos::new(TEST_CHUNK_X, TEST_CHUNK_Z),
            protocol_version: PROTOCOL_VERSION,
            dimension_type_name: TEST_DIMENSION_TYPE,
            height: TEST_HEIGHT,
            min_y: TEST_MIN_Y,
            biome_registry_len: TEST_BIOME_REGISTRY_LEN,
            compression_threshold: CompressionThreshold::DEFAULT,
            content_version: TEST_CONTENT_VERSION,
            light_fingerprint: Some(empty_light_fingerprint()),
        }
    }
}
