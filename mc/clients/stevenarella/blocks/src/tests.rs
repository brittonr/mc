use crate::generated::{
    check_generated_data_freshness, validate_generated_data_samples, GeneratedDataSample,
};
use crate::shared::Direction;
use crate::*;
use std::collections::HashMap;

const LEGACY_PROTOCOL_1_12_2: i32 = 340;
const FLATTENED_PROTOCOL_1_13_2: i32 = 404;
const PROTOCOL_1_14_4: i32 = 477;
const PROTOCOL_1_15_1: i32 = 575;
const PROTOCOL_1_16: i32 = 735;
const PROTOCOL_1_16_2: i32 = 751;
const LEGACY_STRUCTURE_BLOCK_ID: usize = 255;
const LEGACY_STRUCTURE_DATA_MODE: usize = 3;
const LEGACY_BLOCK_DATA_BITS: usize = 4;
const STRUCTURE_BLOCK_SAVE_FLAT_ID_1_13_2: usize = 8595;
const STRUCTURE_BLOCK_DATA_FLAT_ID_1_13_2: usize = 8598;
const CONDUIT_WATERLOGGED_FLAT_ID: usize = 9113;
const CONDUIT_DRY_FLAT_ID: usize = 9114;
const NOTE_BLOCK_PLING_LAST_NOTE_FLAT_ID: usize = 1048;
const NOTE_BLOCK_LAST_NOTE: u8 = 24;
const UNKNOWN_FLAT_BLOCK_ID: usize = usize::MAX;
const UNSUPPORTED_MODDED_BLOCK_ID: usize = 4096;
const THERMAL_ROCKWOOL_BLOCK_ID: usize = 4097;
const RED_WOOL_DATA: usize = 14;
const INVALID_STRUCTURE_DATA: usize = 4;
const THERMAL_FOUNDATION_ROCKWOOL_NAME: &str = "thermalfoundation:rockwool";
const UNSUPPORTED_MODDED_BLOCK_NAME: &str = "example:unsupported_block";

// Spot check a few blocks across different versions, including the correctly recognized last supported block
// TODO: comprehensive testing against https://github.com/PrismarineJS/minecraft-data/tree/master/data/pc

#[test]
fn hier_1_12_2() {
    let id_map = VanillaIDMap::new(LEGACY_PROTOCOL_1_12_2);
    assert_eq!(
        id_map.by_vanilla_id(
            LEGACY_STRUCTURE_BLOCK_ID << LEGACY_BLOCK_DATA_BITS,
            &HashMap::new()
        ),
        StructureBlock {
            mode: StructureBlockMode::Save
        }
    );
    assert_eq!(
        id_map.by_vanilla_id(
            (LEGACY_STRUCTURE_BLOCK_ID << LEGACY_BLOCK_DATA_BITS) | LEGACY_STRUCTURE_DATA_MODE,
            &HashMap::new()
        ),
        StructureBlock {
            mode: StructureBlockMode::Data
        }
    );
}

#[test]
fn flat_1_13_2() {
    let id_map = VanillaIDMap::new(FLATTENED_PROTOCOL_1_13_2);
    assert_eq!(
        id_map.by_vanilla_id(STRUCTURE_BLOCK_SAVE_FLAT_ID_1_13_2, &HashMap::new()),
        StructureBlock {
            mode: StructureBlockMode::Save
        }
    );
    assert_eq!(
        id_map.by_vanilla_id(STRUCTURE_BLOCK_DATA_FLAT_ID_1_13_2, &HashMap::new()),
        StructureBlock {
            mode: StructureBlockMode::Data
        }
    );
}

#[test]
fn flat_1_14_4() {
    let id_map = VanillaIDMap::new(PROTOCOL_1_14_4);
    assert_eq!(
        id_map.by_vanilla_id(CONDUIT_WATERLOGGED_FLAT_ID, &HashMap::new()),
        Conduit { waterlogged: true }
    );
    assert_eq!(
        id_map.by_vanilla_id(CONDUIT_DRY_FLAT_ID, &HashMap::new()),
        Conduit { waterlogged: false }
    );
}

#[test]
fn flat_1_15_1() {
    let id_map = VanillaIDMap::new(PROTOCOL_1_15_1);
    assert_eq!(
        id_map.by_vanilla_id(CONDUIT_WATERLOGGED_FLAT_ID, &HashMap::new()),
        Conduit { waterlogged: true }
    );
    assert_eq!(
        id_map.by_vanilla_id(CONDUIT_DRY_FLAT_ID, &HashMap::new()),
        Conduit { waterlogged: false }
    );
}

#[test]
fn flat_1_16() {
    let id_map = VanillaIDMap::new(PROTOCOL_1_16);
    assert_eq!(
        id_map.by_vanilla_id(NOTE_BLOCK_PLING_LAST_NOTE_FLAT_ID, &HashMap::new()),
        NoteBlock {
            instrument: NoteBlockInstrument::Pling,
            note: NOTE_BLOCK_LAST_NOTE,
            powered: false
        }
    );
}

#[test]
fn flat_1_16_2() {
    let id_map = VanillaIDMap::new(PROTOCOL_1_16_2);
    assert_eq!(
        id_map.by_vanilla_id(NOTE_BLOCK_PLING_LAST_NOTE_FLAT_ID, &HashMap::new()),
        NoteBlock {
            instrument: NoteBlockInstrument::Pling,
            note: NOTE_BLOCK_LAST_NOTE,
            powered: false
        }
    );
}

#[test]
fn grass_block_model_uses_post_flattening_asset_name() {
    let (plugin, model) = (Block::Grass { snowy: false }).get_model();
    assert_eq!(plugin, "minecraft");
    assert_eq!(model, "grass_block");
}

#[test]
fn missing_block_model_still_uses_steven_fallback_asset() {
    let (plugin, model) = (Block::Missing {}).get_model();
    assert_eq!(plugin, "steven");
    assert_eq!(model, "missing_block");
}

#[test]
fn generated_block_data_freshness_check_passes() {
    check_generated_data_freshness().expect("generated block data samples should stay fresh");
}

#[test]
fn generated_block_data_freshness_check_rejects_stale_samples() {
    let stale_samples = [GeneratedDataSample {
        protocol_version: FLATTENED_PROTOCOL_1_13_2,
        vanilla_id: STRUCTURE_BLOCK_SAVE_FLAT_ID_1_13_2,
        expected: Block::Missing {},
    }];

    let error = validate_generated_data_samples(&stale_samples).expect_err("stale sample");
    assert_eq!(error.expected, Block::Missing {});
    assert_eq!(
        error.actual,
        Block::StructureBlock {
            mode: StructureBlockMode::Save
        }
    );
}

#[test]
fn public_reexports_material_and_collision_access_remain_available() {
    let exported_variant: Block = Stone {
        variant: StoneVariant::Normal,
    };
    assert_eq!(
        exported_variant,
        Block::Stone {
            variant: StoneVariant::Normal,
        }
    );
    assert!(exported_variant.get_material().collidable);
    assert_eq!(exported_variant.get_collision_boxes().len(), 1);
    assert!((Block::Air {}).get_collision_boxes().is_empty());
}

#[test]
fn legacy_modded_block_fallback_still_resolves_supported_blocks() {
    let id_map = VanillaIDMap::new(LEGACY_PROTOCOL_1_12_2);
    let mut modded_block_ids = HashMap::new();
    modded_block_ids.insert(
        THERMAL_ROCKWOOL_BLOCK_ID,
        THERMAL_FOUNDATION_ROCKWOOL_NAME.to_string(),
    );

    assert_eq!(
        id_map.by_vanilla_id(
            (THERMAL_ROCKWOOL_BLOCK_ID << LEGACY_BLOCK_DATA_BITS) | RED_WOOL_DATA,
            &modded_block_ids,
        ),
        Block::ThermalFoundationRockwool {
            color: ColoredVariant::Red,
        }
    );
}

#[test]
fn unknown_flat_and_legacy_paths_return_missing_block() {
    let flat_map = VanillaIDMap::new(FLATTENED_PROTOCOL_1_13_2);
    assert_eq!(
        flat_map.by_vanilla_id(UNKNOWN_FLAT_BLOCK_ID, &HashMap::new()),
        Block::Missing {}
    );

    let legacy_map = VanillaIDMap::new(LEGACY_PROTOCOL_1_12_2);
    assert_eq!(
        legacy_map.by_vanilla_id(
            UNSUPPORTED_MODDED_BLOCK_ID << LEGACY_BLOCK_DATA_BITS,
            &HashMap::new(),
        ),
        Block::Missing {}
    );
}

#[test]
fn unsupported_modded_ids_and_invalid_legacy_data_return_missing_block() {
    let id_map = VanillaIDMap::new(LEGACY_PROTOCOL_1_12_2);
    let mut modded_block_ids = HashMap::new();
    modded_block_ids.insert(
        UNSUPPORTED_MODDED_BLOCK_ID,
        UNSUPPORTED_MODDED_BLOCK_NAME.to_string(),
    );

    assert_eq!(
        id_map.by_vanilla_id(
            UNSUPPORTED_MODDED_BLOCK_ID << LEGACY_BLOCK_DATA_BITS,
            &modded_block_ids,
        ),
        Block::Missing {}
    );
    assert_eq!(
        id_map.by_vanilla_id(
            (LEGACY_STRUCTURE_BLOCK_ID << LEGACY_BLOCK_DATA_BITS) | INVALID_STRUCTURE_DATA,
            &HashMap::new(),
        ),
        Block::Missing {}
    );
}

#[test]
fn verify_blocks() {
    let dirt = Block::Dirt {
        snowy: true,
        variant: DirtVariant::Normal,
    };
    let stone = Block::Stone {
        variant: StoneVariant::Normal,
    };
    let vine = Block::Vine {
        up: false,
        south: false,
        west: false,
        north: true,
        east: false,
    };
    let pumpkin_lit = Block::PumpkinLit {
        facing: Direction::North,
        without_face: true,
    };
    let cocoa = Block::Cocoa {
        age: 1,
        facing: Direction::North,
    };
    let leaves = Block::Leaves {
        variant: TreeVariant::Oak,
        decayable: false,
        check_decay: false,
        distance: 1,
    };
    let leaves2 = Block::Leaves2 {
        variant: TreeVariant::Oak,
        decayable: false,
        check_decay: false,
    };
    let wool = Block::Wool {
        color: ColoredVariant::White,
    };
    let tall_seagrass = Block::TallSeagrass {
        half: TallSeagrassHalf::Upper,
    };
    let data = [
        (dirt, None, Some(0.75)),
        (dirt, Some(Tool::Shovel(ToolMaterial::Wood)), Some(0.4)),
        (dirt, Some(Tool::Pickaxe(ToolMaterial::Wood)), Some(0.75)),
        (stone, None, Some(7.5)),
        (stone, Some(Tool::Shovel(ToolMaterial::Wood)), Some(7.5)),
        (stone, Some(Tool::Pickaxe(ToolMaterial::Wood)), Some(1.15)),
        (Block::Obsidian {}, None, Some(250.0)),
        (
            Block::Obsidian {},
            Some(Tool::Pickaxe(ToolMaterial::Wood)),
            Some(125.0),
        ),
        (
            Block::Obsidian {},
            Some(Tool::Pickaxe(ToolMaterial::Stone)),
            Some(62.5),
        ),
        (
            Block::Obsidian {},
            Some(Tool::Pickaxe(ToolMaterial::Iron)),
            Some(41.7),
        ),
        (
            Block::Obsidian {},
            Some(Tool::Pickaxe(ToolMaterial::Diamond)),
            Some(9.4),
        ),
        (
            Block::Obsidian {},
            Some(Tool::Pickaxe(ToolMaterial::Netherite)),
            Some(8.35),
        ),
        (
            Block::Obsidian {},
            Some(Tool::Pickaxe(ToolMaterial::Gold)),
            Some(20.85),
        ),
        (Block::Bedrock {}, None, None),
        (
            Block::Bedrock {},
            Some(Tool::Pickaxe(ToolMaterial::Wood)),
            None,
        ),
        (
            Block::Bedrock {},
            Some(Tool::Pickaxe(ToolMaterial::Stone)),
            None,
        ),
        (
            Block::Bedrock {},
            Some(Tool::Pickaxe(ToolMaterial::Iron)),
            None,
        ),
        (
            Block::Bedrock {},
            Some(Tool::Pickaxe(ToolMaterial::Diamond)),
            None,
        ),
        (
            Block::Bedrock {},
            Some(Tool::Pickaxe(ToolMaterial::Netherite)),
            None,
        ),
        (
            Block::Bedrock {},
            Some(Tool::Pickaxe(ToolMaterial::Gold)),
            None,
        ),
        (Block::Web {}, None, Some(20.0)),
        (
            Block::Web {},
            Some(Tool::Pickaxe(ToolMaterial::Wood)),
            Some(20.0),
        ),
        (vine, None, Some(0.3)),
        (vine, Some(Tool::Pickaxe(ToolMaterial::Wood)), Some(0.3)),
        (vine, Some(Tool::Axe(ToolMaterial::Wood)), Some(0.15)),
        (vine, Some(Tool::Axe(ToolMaterial::Stone)), Some(0.1)),
        (vine, Some(Tool::Axe(ToolMaterial::Iron)), Some(0.05)),
        (vine, Some(Tool::Axe(ToolMaterial::Diamond)), Some(0.05)),
        (wool, None, Some(1.2)),
        (leaves, None, Some(0.3)),
        (leaves, Some(Tool::Hoe(ToolMaterial::Wood)), Some(0.15)),
        (leaves, Some(Tool::Hoe(ToolMaterial::Stone)), Some(0.1)),
        (leaves, Some(Tool::Hoe(ToolMaterial::Iron)), Some(0.05)),
        (leaves, Some(Tool::Hoe(ToolMaterial::Diamond)), Some(0.05)),
        (leaves2, None, Some(0.3)),
        (leaves2, Some(Tool::Hoe(ToolMaterial::Wood)), Some(0.15)),
        (leaves2, Some(Tool::Hoe(ToolMaterial::Stone)), Some(0.1)),
        (leaves2, Some(Tool::Hoe(ToolMaterial::Iron)), Some(0.05)),
        (leaves2, Some(Tool::Hoe(ToolMaterial::Diamond)), Some(0.05)),
        (Block::DeadBush {}, None, Some(0.05)),
        (Block::DeadBush {}, Some(Tool::Shears), Some(0.05)),
        (Block::Seagrass {}, None, Some(0.05)),
        (Block::Seagrass {}, Some(Tool::Shears), Some(0.05)),
        (tall_seagrass, None, Some(0.05)),
        (tall_seagrass, Some(Tool::Shears), Some(0.05)),
        (cocoa, None, Some(0.3)),
        (cocoa, Some(Tool::Axe(ToolMaterial::Wood)), Some(0.15)),
        (cocoa, Some(Tool::Axe(ToolMaterial::Stone)), Some(0.1)),
        (cocoa, Some(Tool::Axe(ToolMaterial::Iron)), Some(0.05)),
        (cocoa, Some(Tool::Axe(ToolMaterial::Diamond)), Some(0.05)),
        (Block::MelonBlock {}, None, Some(1.5)),
        (
            Block::MelonBlock {},
            Some(Tool::Axe(ToolMaterial::Wood)),
            Some(0.75),
        ),
        (
            Block::MelonBlock {},
            Some(Tool::Axe(ToolMaterial::Stone)),
            Some(0.4),
        ),
        (
            Block::MelonBlock {},
            Some(Tool::Axe(ToolMaterial::Iron)),
            Some(0.25),
        ),
        (
            Block::MelonBlock {},
            Some(Tool::Axe(ToolMaterial::Diamond)),
            Some(0.2),
        ),
        (
            Block::MelonBlock {},
            Some(Tool::Axe(ToolMaterial::Netherite)),
            Some(0.2),
        ),
        (
            Block::MelonBlock {},
            Some(Tool::Axe(ToolMaterial::Gold)),
            Some(0.15),
        ),
        (Block::Pumpkin {}, None, Some(1.5)),
        (
            Block::Pumpkin {},
            Some(Tool::Axe(ToolMaterial::Wood)),
            Some(0.75),
        ),
        (
            Block::Pumpkin {},
            Some(Tool::Axe(ToolMaterial::Stone)),
            Some(0.4),
        ),
        (
            Block::Pumpkin {},
            Some(Tool::Axe(ToolMaterial::Iron)),
            Some(0.25),
        ),
        (pumpkin_lit, None, Some(1.5)),
        (pumpkin_lit, Some(Tool::Axe(ToolMaterial::Wood)), Some(0.75)),
        (pumpkin_lit, Some(Tool::Axe(ToolMaterial::Stone)), Some(0.4)),
        (pumpkin_lit, Some(Tool::Axe(ToolMaterial::Iron)), Some(0.25)),
        // TODO: Fix special sword rules
        //(Block::Web {}, Some(Tool::Sword(ToolMaterial::Wood)), Some(0.4)),
        //(Block::Web {}, Some(Tool::Sword(ToolMaterial::Stone)), Some(0.4)),
        //(cocoa, Some(Tool::Sword(ToolMaterial::Stone)), Some(0.2)),
        //(leaves, Some(Tool::Sword(ToolMaterial::St
        //(leaves2, Some(Tool::Sword(ToolMaterial::Stone)), Some(0.2)),
        //(Block::MelonBlock {}, Some(Tool::Sword(ToolMaterial::Stone)), Some(1.0)),
        //(Block::Pumpkin {}, Some(Tool::Sword(ToolMaterial::Stone)), Some(1.0)),
        //(pumpkin_lit, Some(Tool::Sword(ToolMaterial::Stone)), Some(1.0)),
        //(vine, Some(Tool::Sword(ToolMaterial::Stone)), Some(0.2)),

        // TODO: Fix special shears rules
        //(Block::Web {}, Some(Tool::Shears), Some(0.4)),
        //(wool, Some(Tool::Shears), Some(0.25)),
        //(leaves, Some(Tool::Shears), Some(0.05)),
        //(leaves2, Some(Tool::Shears), Some(0.05)),
        //(vine, Some(Tool::Shears), Some(0.3)),
    ];
    for (block, tool, time) in data {
        let result = block.get_mining_time(&tool).map(|d| d.as_secs_f64());
        match (time, result) {
            (Some(time), Some(result)) => assert_eq!(
                result, time,
                "Expected to mine block {:?} with {:?} in {} seconds, but it took {} seconds",
                block, tool, time, result
            ),
            (None, Some(result)) => panic!(
                "Expected to never mine block {:?} with {:?}, but it took {} seconds",
                block, tool, result
            ),
            (Some(time), None) => panic!(
                "Expected to mine block {:?} with {:?} in {} seconds, but it will never be mined",
                block, tool, time
            ),
            _ => {}
        }
    }
}
