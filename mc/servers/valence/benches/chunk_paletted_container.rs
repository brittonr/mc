use std::hint::black_box;

use divan::Bencher;
use valence::prelude::{BlockState, Chunk, UnloadedChunk};

const CHUNK_HEIGHT: u32 = SECTION_EDGE_BLOCKS;
const SECTION_Y: u32 = 0;
const SECTION_EDGE_BLOCKS: u32 = 16;
const SECTION_LAYER_BLOCKS: u32 = SECTION_EDGE_BLOCKS * SECTION_EDGE_BLOCKS;
const SECTION_BLOCK_COUNT: u32 = SECTION_LAYER_BLOCKS * SECTION_EDGE_BLOCKS;
const INDIRECT_PALETTE_BLOCK_COUNT: usize = 8;
const INDIRECT_MUTATION_COUNT: u32 = SECTION_BLOCK_COUNT;
const INDIRECT_PALETTE_CAPACITY: usize = 16;
const DIRECT_PALETTE_BLOCK_COUNT: usize = INDIRECT_PALETTE_CAPACITY + 1;
const DIRECT_MUTATION_COUNT: u32 = SECTION_BLOCK_COUNT;
const SHRINK_REWRITE_COUNT: u32 = SECTION_BLOCK_COUNT;

const INDIRECT_BLOCKS: [BlockState; INDIRECT_PALETTE_BLOCK_COUNT] = [
    BlockState::STONE,
    BlockState::DIRT,
    BlockState::GRASS_BLOCK,
    BlockState::OAK_LOG,
    BlockState::OAK_PLANKS,
    BlockState::COBBLESTONE,
    BlockState::SAND,
    BlockState::GLASS,
];

const DIRECT_BLOCKS: [BlockState; DIRECT_PALETTE_BLOCK_COUNT] = [
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

#[divan::bench]
pub(crate) fn single_section_read(bencher: Bencher) {
    bencher
        .with_inputs(single_section_fixture)
        .bench_local_values(|chunk| {
            for idx in 0..SECTION_BLOCK_COUNT {
                let (x, y, z) = section_coords(idx);
                black_box(chunk.block_state(x, y, z));
            }
        });
}

#[divan::bench]
pub(crate) fn indirect_palette_growth(bencher: Bencher) {
    bencher
        .with_inputs(empty_section_fixture)
        .bench_local_values(|mut chunk| {
            for idx in 0..INDIRECT_MUTATION_COUNT {
                let palette_idx = idx as usize % INDIRECT_BLOCKS.len();
                let (x, y, z) = section_coords(idx);
                black_box(chunk.set_block_state(x, y, z, INDIRECT_BLOCKS[palette_idx]));
            }
            black_box(chunk);
        });
}

#[divan::bench]
pub(crate) fn direct_fallback_mutation(bencher: Bencher) {
    bencher
        .with_inputs(empty_section_fixture)
        .bench_local_values(|mut chunk| {
            for idx in 0..DIRECT_MUTATION_COUNT {
                let palette_idx = idx as usize % DIRECT_BLOCKS.len();
                let (x, y, z) = section_coords(idx);
                black_box(chunk.set_block_state(x, y, z, DIRECT_BLOCKS[palette_idx]));
            }
            black_box(chunk);
        });
}

#[divan::bench]
pub(crate) fn shrink_direct_to_indirect(bencher: Bencher) {
    bencher
        .with_inputs(direct_section_fixture)
        .bench_local_values(|mut chunk| {
            for idx in 0..SHRINK_REWRITE_COUNT {
                let block = INDIRECT_BLOCKS[idx as usize % INDIRECT_BLOCKS.len()];
                let (x, y, z) = section_coords(idx);
                black_box(chunk.set_block_state(x, y, z, block));
            }
            chunk.shrink_to_fit();
            black_box(chunk);
        });
}

fn single_section_fixture() -> UnloadedChunk {
    let mut chunk = empty_section_fixture();
    chunk.fill_block_state_section(SECTION_Y, BlockState::STONE);
    chunk
}

fn direct_section_fixture() -> UnloadedChunk {
    let mut chunk = empty_section_fixture();
    for (idx, block) in DIRECT_BLOCKS.iter().copied().enumerate() {
        let (x, y, z) = section_coords(idx as u32);
        chunk.set_block_state(x, y, z, block);
    }
    chunk
}

fn empty_section_fixture() -> UnloadedChunk {
    UnloadedChunk::with_height(CHUNK_HEIGHT)
}

fn section_coords(idx: u32) -> (u32, u32, u32) {
    let x = idx % SECTION_EDGE_BLOCKS;
    let z = (idx / SECTION_EDGE_BLOCKS) % SECTION_EDGE_BLOCKS;
    let y = idx / SECTION_LAYER_BLOCKS;
    (x, y, z)
}
