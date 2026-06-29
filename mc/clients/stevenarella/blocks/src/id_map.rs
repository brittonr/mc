use crate::runtime::generated::{gen_id_map, Block};
use std::collections::HashMap;

pub const FLATTENING_PROTOCOL_VERSION: i32 = 404;
pub const LEGACY_BLOCK_DATA_MASK: usize = 0x0f;
pub const LEGACY_BLOCK_ID_SHIFT: usize = 4;
pub const LEGACY_BLOCK_DATA_VALUE_COUNT: usize = 16;

#[derive(Default)]
pub struct VanillaIDMap {
    flat: Vec<Option<Block>>,
    hier: Vec<Option<Block>>,
    modded: HashMap<String, [Option<Block>; LEGACY_BLOCK_DATA_VALUE_COUNT]>,
    protocol_version: i32,
}

impl VanillaIDMap {
    pub fn new(protocol_version: i32) -> VanillaIDMap {
        gen_id_map(protocol_version)
    }

    pub(crate) fn from_parts(
        protocol_version: i32,
        flat: Vec<Option<Block>>,
        hier: Vec<Option<Block>>,
        modded: HashMap<String, [Option<Block>; LEGACY_BLOCK_DATA_VALUE_COUNT]>,
    ) -> VanillaIDMap {
        VanillaIDMap {
            flat,
            hier,
            modded,
            protocol_version,
        }
    }

    pub fn by_vanilla_id(
        &self,
        id: usize,
        modded_block_ids: &HashMap<usize, String>, // TODO: remove and add to constructor, but have to mutate in Server
    ) -> Block {
        if self.protocol_version >= FLATTENING_PROTOCOL_VERSION {
            return lookup_flat_block(&self.flat, id);
        }

        lookup_legacy_block(&self.hier, &self.modded, id, modded_block_ids)
    }
}

fn lookup_flat_block(flat: &[Option<Block>], id: usize) -> Block {
    flat.get(id).and_then(|v| *v).unwrap_or(Block::Missing {})
}

fn lookup_legacy_block(
    hier: &[Option<Block>],
    modded: &HashMap<String, [Option<Block>; LEGACY_BLOCK_DATA_VALUE_COUNT]>,
    id: usize,
    modded_block_ids: &HashMap<usize, String>,
) -> Block {
    if let Some(block) = hier.get(id).and_then(|v| *v) {
        return block;
    }

    let key = legacy_block_key(id);
    let Some(name) = modded_block_ids.get(&key.block_id) else {
        return Block::Missing {};
    };
    let Some(blocks_by_data) = modded.get(name) else {
        // info!("Modded block not supported yet: {}:{} -> {}", key.block_id, key.data, name);
        return Block::Missing {};
    };

    blocks_by_data[key.data].unwrap_or(Block::Missing {})
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct LegacyBlockKey {
    block_id: usize,
    data: usize,
}

fn legacy_block_key(id: usize) -> LegacyBlockKey {
    LegacyBlockKey {
        block_id: id >> LEGACY_BLOCK_ID_SHIFT,
        data: id & LEGACY_BLOCK_DATA_MASK,
    }
}
