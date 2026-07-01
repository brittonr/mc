#!/usr/bin/env -S CARGO_TARGET_DIR=target/check-crafting-recipe-core-script nix shell "github:nix-community/fenix?rev=092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain" nixpkgs#gcc -c cargo -q -Zscript

use std::env;
use std::fmt::Debug;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const SELF_TEST_FLAG: &str = "--self-test";
const HELP_FLAG: &str = "--help";
const FIXTURE_FLAG: &str = "--fixture";
const HELP_TEXT: &str = "usage: check_crafting_recipe_core.rs [--self-test] [--fixture PATH]";
const SUCCESS_MESSAGE: &str = "crafting recipe selected-matrix core check passed";
const SUCCESS: ExitCode = ExitCode::SUCCESS;
const FAILURE: ExitCode = ExitCode::FAILURE;

const CRAFTING_GRID_WIDTH: usize = 3;
const CRAFTING_GRID_HEIGHT: usize = 3;
const CRAFTING_GRID_SLOT_COUNT: usize = CRAFTING_GRID_WIDTH * CRAFTING_GRID_HEIGHT;
const CHEST_RECIPE_PATTERN_ROWS: usize = CRAFTING_GRID_HEIGHT;
const CHEST_RECIPE_KEY_COUNT: usize = 1;
const SHAPELESS_OAK_PLANKS_INGREDIENT_COUNT: usize = 1;
const SELECTED_RECIPE_COUNT: usize = 2;
const UNSUPPORTED_COLLECTION_MODE_COUNT: usize = 5;

const SLOT_TOP_LEFT: usize = 0;
const SLOT_TOP_MIDDLE: usize = 1;
const SLOT_TOP_RIGHT: usize = 2;
const SLOT_MIDDLE_LEFT: usize = 3;
const SLOT_MIDDLE_CENTER: usize = 4;
const SLOT_MIDDLE_RIGHT: usize = 5;
const SLOT_BOTTOM_LEFT: usize = 6;
const SLOT_BOTTOM_MIDDLE: usize = 7;
const SLOT_BOTTOM_RIGHT: usize = 8;

const ONE_ITEM: u32 = 1;
const EMPTY_COUNT: u32 = 0;
const OAK_PLANKS_OUTPUT_COUNT: u32 = 4;
const MAX_STACK_SIZE: u32 = 64;
const RESULT_SLOT_STACK_LIMIT: u32 = MAX_STACK_SIZE;
const EXISTING_TARGET_PLANKS: u32 = 12;
const EXPECTED_MERGED_TARGET_PLANKS: u32 = EXISTING_TARGET_PLANKS + OAK_PLANKS_OUTPUT_COUNT;
const TARGET_PROTOCOL: u32 = 763;
const PRIMARY_CHEST_TARGET_SLOT: u16 = 36;
const PRIMARY_PLANKS_TARGET_SLOT: u16 = 37;

const MINECRAFT_NAMESPACE: &str = "minecraft";
const TARGET_EDITION: &str = "Java Edition";
const TARGET_VERSION: &str = "1.20.1";
const CHEST_RECIPE_ID: &str = "minecraft:chest";
const OAK_PLANKS_RECIPE_ID: &str = "minecraft:oak_planks";
const STICK_REJECTION_ID: &str = "minecraft:stick_insufficient_input_rejection";
const OAK_PLANKS: &str = "minecraft:oak_planks";
const OAK_LOG: &str = "minecraft:oak_log";
const CHEST: &str = "minecraft:chest";
const COBBLESTONE: &str = "minecraft:cobblestone";
const INVALID_ITEM_ID: &str = "Minecraft:Chest";
const UNSUPPORTED_RECIPE_KIND: &str = "minecraft:stonecutting";
const RADIX_TEN: u32 = 10;
const ARGUMENT_STEP: usize = 1;

const TARGET_EDITION_BINDING: &str = "target_edition";
const TARGET_GAME_VERSION_BINDING: &str = "target_game_version";
const TARGET_PROTOCOL_BINDING: &str = "target_protocol";
const GRID_WIDTH_BINDING: &str = "selected_grid_width";
const GRID_HEIGHT_BINDING: &str = "selected_grid_height";
const MAX_STACK_SIZE_BINDING: &str = "selected_max_stack_size";
const RESULT_SLOT_STACK_LIMIT_BINDING: &str = "selected_result_slot_stack_limit";
const CHEST_OUTPUT_COUNT_BINDING: &str = "selected_chest_output_count_value";
const OAK_PLANKS_OUTPUT_COUNT_BINDING: &str = "selected_oak_planks_output_count_value";
const CHEST_TARGET_SLOT_BINDING: &str = "selected_chest_target_slot_value";
const OAK_PLANKS_TARGET_SLOT_BINDING: &str = "selected_oak_planks_target_slot_value";
const SHAPED_KIND_BINDING: &str = "selected_shaped_recipe_kind";
const SHAPELESS_KIND_BINDING: &str = "selected_shapeless_recipe_kind";
const REJECTED_NO_RESULT_KIND_BINDING: &str = "selected_rejected_no_result_kind";
const COLLECTION_MODE_BINDING: &str = "selected_collection_mode";
const CHEST_RECIPE_ID_BINDING: &str = "selected_chest_recipe_id";
const CHEST_KEY_SYMBOL_BINDING: &str = "selected_chest_key_symbol";
const CHEST_PATTERN_TOP_BINDING: &str = "selected_chest_pattern_top";
const CHEST_PATTERN_MIDDLE_BINDING: &str = "selected_chest_pattern_middle";
const CHEST_PATTERN_BOTTOM_BINDING: &str = "selected_chest_pattern_bottom";
const CHEST_KEY_ITEM_BINDING: &str = "selected_chest_key_item";
const CHEST_KEY_COUNT_BINDING: &str = "selected_chest_key_count";
const CHEST_OUTPUT_ITEM_BINDING: &str = "selected_chest_output_item";
const OAK_PLANKS_RECIPE_ID_BINDING: &str = "selected_oak_planks_recipe_id";
const SHAPELESS_INPUT_ITEM_BINDING: &str = "selected_shapeless_input_item";
const SHAPELESS_INPUT_COUNT_BINDING: &str = "selected_shapeless_input_count";
const SHAPELESS_OUTPUT_ITEM_BINDING: &str = "selected_shapeless_output_item";
const INVALID_PROBE_ID_BINDING: &str = "selected_invalid_probe_id";
const INVALID_PROBE_INPUT_ITEM_BINDING: &str = "selected_invalid_probe_input_item";
const INVALID_PROBE_INPUT_COUNT_BINDING: &str = "selected_invalid_probe_input_count";
const INVALID_PROBE_DIAGNOSTIC_BINDING: &str = "selected_invalid_probe_diagnostic";
const SHAPED_KIND: &str = "shaped";
const SHAPELESS_KIND: &str = "shapeless";
const REJECTED_NO_RESULT_KIND: &str = "rejected_no_result";
const COLLECTION_MODE: &str = "primary_click";
const INVALID_PROBE_DIAGNOSTIC: &str = "no_result";

const CHEST_SYMBOL: char = 'P';
const EMPTY_PATTERN_SYMBOL: char = ' ';
const CHEST_PATTERN_TOP: &str = "PPP";
const CHEST_PATTERN_MIDDLE: &str = "P P";
const CHEST_PATTERN_BOTTOM: &str = "PPP";
const MALFORMED_PATTERN_ROW: &str = "PP";

const TARGET_SCOPE: TargetScope<'static> = TargetScope {
    edition: TARGET_EDITION,
    version: TARGET_VERSION,
    protocol: TARGET_PROTOCOL,
};

const CHEST_PATTERN: [&str; CHEST_RECIPE_PATTERN_ROWS] = [
    CHEST_PATTERN_TOP,
    CHEST_PATTERN_MIDDLE,
    CHEST_PATTERN_BOTTOM,
];
const CHEST_KEY: [SymbolIngredient<'static>; CHEST_RECIPE_KEY_COUNT] = [SymbolIngredient {
    symbol: CHEST_SYMBOL,
    item: OAK_PLANKS,
    count: ONE_ITEM,
}];
const OAK_PLANKS_INGREDIENTS: [ItemStack<'static>; SHAPELESS_OAK_PLANKS_INGREDIENT_COUNT] =
    [ItemStack {
        item: OAK_LOG,
        count: ONE_ITEM,
    }];
const EXPECTED_SELECTED_RECIPE_IDS: [&str; SELECTED_RECIPE_COUNT] =
    [CHEST_RECIPE_ID, OAK_PLANKS_RECIPE_ID];
const SELECTED_RECIPES: [RecipeRow<'static>; SELECTED_RECIPE_COUNT] = [
    RecipeRow {
        id: CHEST_RECIPE_ID,
        kind: RecipeKind::Shaped {
            pattern: &CHEST_PATTERN,
            key: &CHEST_KEY,
        },
        output: ItemStack {
            item: CHEST,
            count: ONE_ITEM,
        },
        target: Some(TARGET_SCOPE),
    },
    RecipeRow {
        id: OAK_PLANKS_RECIPE_ID,
        kind: RecipeKind::Shapeless {
            ingredients: &OAK_PLANKS_INGREDIENTS,
        },
        output: ItemStack {
            item: OAK_PLANKS,
            count: OAK_PLANKS_OUTPUT_COUNT,
        },
        target: Some(TARGET_SCOPE),
    },
];
const UNSUPPORTED_COLLECTION_MODES: [CollectionMode; UNSUPPORTED_COLLECTION_MODE_COUNT] = [
    CollectionMode::ShiftClick,
    CollectionMode::Drag,
    CollectionMode::Split,
    CollectionMode::RecipeBookUi,
    CollectionMode::AutomatedCrafter,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ItemStack<'a> {
    item: &'a str,
    count: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CraftingGrid<'a> {
    slots: [Option<ItemStack<'a>>; CRAFTING_GRID_SLOT_COUNT],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct OutputSlotState<'a> {
    stack: Option<ItemStack<'a>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct InventorySlot<'a> {
    index: u16,
    stack: Option<ItemStack<'a>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CollectionRequest<'a> {
    mode: CollectionMode,
    target_slot: InventorySlot<'a>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CollectionMode {
    PrimaryClick,
    ShiftClick,
    Drag,
    Split,
    RecipeBookUi,
    AutomatedCrafter,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CraftingLimits {
    max_stack_size: u32,
    result_slot_stack_limit: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TargetScope<'a> {
    edition: &'a str,
    version: &'a str,
    protocol: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SymbolIngredient<'a> {
    symbol: char,
    item: &'a str,
    count: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RecipeKind<'a> {
    Shaped {
        pattern: &'a [&'a str],
        key: &'a [SymbolIngredient<'a>],
    },
    Shapeless {
        ingredients: &'a [ItemStack<'a>],
    },
    Unsupported {
        name: &'a str,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RecipeRow<'a> {
    id: &'a str,
    kind: RecipeKind<'a>,
    output: ItemStack<'a>,
    target: Option<TargetScope<'a>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RecipeMatrix<'a> {
    rows: &'a [RecipeRow<'a>],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct InventoryDelta<'a> {
    slot_index: u16,
    item: &'a str,
    count_before: u32,
    count_after: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NoResultReason {
    SelectedProbeRejected(&'static str),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CraftingDecision<'a> {
    Matched {
        recipe_id: &'a str,
        output: ItemStack<'a>,
        grid_after: CraftingGrid<'a>,
        inventory_delta: InventoryDelta<'a>,
    },
    NoResult {
        reason: NoResultReason,
        grid_after: CraftingGrid<'a>,
        inventory_after: InventorySlot<'a>,
    },
    OutputBlocked {
        recipe_id: &'a str,
        grid_after: CraftingGrid<'a>,
        inventory_after: InventorySlot<'a>,
        output_slot: OutputSlotState<'a>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CraftingError<'a> {
    MissingSelectedData(&'static str),
    DuplicateRecipeId(&'a str),
    MalformedShapedRow(&'a str),
    MalformedShapelessRow(&'a str),
    InvalidItemId(&'a str),
    ZeroOutputCount(&'a str),
    MissingTargetData(&'a str),
    UnsupportedRecipeKind(&'a str),
    UnsupportedCollectionMode(CollectionMode),
    InventoryCapacityBlocked,
}

fn main() -> ExitCode {
    match parse_command() {
        Ok(Command::Help) => {
            println!("{HELP_TEXT}");
            SUCCESS
        }
        Ok(Command::SelfTest { fixture_path }) => {
            run_and_report_self_tests(fixture_path.as_deref())
        }
        Err(error) => {
            eprintln!("{error}");
            FAILURE
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Command {
    SelfTest { fixture_path: Option<PathBuf> },
    Help,
}

fn parse_command() -> Result<Command, String> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let mut fixture_path = None;
    let mut saw_self_test = false;
    let mut index = 0;

    while index < args.len() {
        let arg = &args[index];
        if arg == SELF_TEST_FLAG {
            saw_self_test = true;
            index += ARGUMENT_STEP;
            continue;
        }
        if arg == HELP_FLAG {
            return Ok(Command::Help);
        }
        if arg == FIXTURE_FLAG {
            index += ARGUMENT_STEP;
            let Some(path) = args.get(index) else {
                return Err(format!("{FIXTURE_FLAG} requires a path"));
            };
            fixture_path = Some(PathBuf::from(path));
            index += ARGUMENT_STEP;
            continue;
        }
        return Err(format!("unknown argument: {arg}"));
    }

    let _self_test_requested = saw_self_test;
    Ok(Command::SelfTest { fixture_path })
}

fn run_and_report_self_tests(fixture_path: Option<&Path>) -> ExitCode {
    match run_self_tests(fixture_path) {
        Ok(()) => {
            println!("{SUCCESS_MESSAGE}");
            SUCCESS
        }
        Err(error) => {
            eprintln!("crafting recipe selected-matrix core check failed: {error}");
            FAILURE
        }
    }
}

fn evaluate_selected_matrix<'a>(
    grid: CraftingGrid<'a>,
    matrix: RecipeMatrix<'a>,
    output_slot: OutputSlotState<'a>,
    request: CollectionRequest<'a>,
    limits: CraftingLimits,
) -> Result<CraftingDecision<'a>, CraftingError<'a>> {
    validate_collection_request(request.mode)?;
    validate_recipe_matrix(matrix, limits)?;

    let Some(recipe) = find_matching_recipe(grid, matrix)? else {
        return Ok(CraftingDecision::NoResult {
            reason: NoResultReason::SelectedProbeRejected(STICK_REJECTION_ID),
            grid_after: grid,
            inventory_after: request.target_slot,
        });
    };

    if !can_accept_output_slot(output_slot, recipe.output, limits) {
        return Ok(CraftingDecision::OutputBlocked {
            recipe_id: recipe.id,
            grid_after: grid,
            inventory_after: request.target_slot,
            output_slot,
        });
    }

    let inventory_delta = build_inventory_delta(request.target_slot, recipe.output, limits)?;
    let grid_after = consume_recipe_ingredients(grid, recipe);

    Ok(CraftingDecision::Matched {
        recipe_id: recipe.id,
        output: recipe.output,
        grid_after,
        inventory_delta,
    })
}

fn validate_collection_request(mode: CollectionMode) -> Result<(), CraftingError<'static>> {
    match mode {
        CollectionMode::PrimaryClick => Ok(()),
        unsupported => Err(CraftingError::UnsupportedCollectionMode(unsupported)),
    }
}

fn validate_recipe_matrix<'a>(
    matrix: RecipeMatrix<'a>,
    limits: CraftingLimits,
) -> Result<(), CraftingError<'a>> {
    for (left_index, left) in matrix.rows.iter().enumerate() {
        for right in matrix.rows.iter().skip(left_index + 1) {
            if left.id == right.id {
                return Err(CraftingError::DuplicateRecipeId(left.id));
            }
        }
    }

    for row in matrix.rows {
        validate_recipe_row(*row, limits)?;
    }

    for expected_id in EXPECTED_SELECTED_RECIPE_IDS {
        if !matrix.rows.iter().any(|row| row.id == expected_id) {
            return Err(CraftingError::MissingSelectedData(expected_id));
        }
    }

    Ok(())
}

fn validate_recipe_row<'a>(
    row: RecipeRow<'a>,
    limits: CraftingLimits,
) -> Result<(), CraftingError<'a>> {
    validate_item_id(row.id)?;
    validate_item_id(row.output.item)?;
    if row.target.is_none() {
        return Err(CraftingError::MissingTargetData(row.id));
    }
    if row.output.count == EMPTY_COUNT {
        return Err(CraftingError::ZeroOutputCount(row.id));
    }
    if row.output.count > limits.max_stack_size {
        return Err(CraftingError::MalformedShapedRow(row.id));
    }

    match row.kind {
        RecipeKind::Shaped { pattern, key } => validate_shaped_row(row.id, pattern, key),
        RecipeKind::Shapeless { ingredients } => validate_shapeless_row(row.id, ingredients),
        RecipeKind::Unsupported { name } => Err(CraftingError::UnsupportedRecipeKind(name)),
    }
}

fn validate_shaped_row<'a>(
    recipe_id: &'a str,
    pattern: &[&str],
    key: &[SymbolIngredient<'a>],
) -> Result<(), CraftingError<'a>> {
    if pattern.len() != CRAFTING_GRID_HEIGHT || key.is_empty() {
        return Err(CraftingError::MalformedShapedRow(recipe_id));
    }

    for ingredient in key {
        validate_item_id(ingredient.item)?;
        if ingredient.count == EMPTY_COUNT {
            return Err(CraftingError::MalformedShapedRow(recipe_id));
        }
    }

    for row in pattern {
        if row.chars().count() != CRAFTING_GRID_WIDTH {
            return Err(CraftingError::MalformedShapedRow(recipe_id));
        }
        for symbol in row.chars() {
            if symbol == EMPTY_PATTERN_SYMBOL {
                continue;
            }
            if find_key_ingredient(symbol, key).is_none() {
                return Err(CraftingError::MalformedShapedRow(recipe_id));
            }
        }
    }

    Ok(())
}

fn validate_shapeless_row<'a>(
    recipe_id: &'a str,
    ingredients: &[ItemStack<'a>],
) -> Result<(), CraftingError<'a>> {
    if ingredients.is_empty() {
        return Err(CraftingError::MalformedShapelessRow(recipe_id));
    }

    for ingredient in ingredients {
        validate_item_id(ingredient.item)?;
        if ingredient.count == EMPTY_COUNT {
            return Err(CraftingError::MalformedShapelessRow(recipe_id));
        }
    }

    Ok(())
}

fn validate_item_id<'a>(item_id: &'a str) -> Result<(), CraftingError<'a>> {
    if is_valid_minecraft_item_id(item_id) {
        Ok(())
    } else {
        Err(CraftingError::InvalidItemId(item_id))
    }
}

fn is_valid_minecraft_item_id(item_id: &str) -> bool {
    let Some((namespace, path)) = item_id.split_once(':') else {
        return false;
    };
    namespace == MINECRAFT_NAMESPACE
        && !path.is_empty()
        && path.chars().all(|character| {
            character.is_ascii_lowercase() || character.is_ascii_digit() || character == '_'
        })
}

fn find_matching_recipe<'a>(
    grid: CraftingGrid<'a>,
    matrix: RecipeMatrix<'a>,
) -> Result<Option<RecipeRow<'a>>, CraftingError<'a>> {
    for row in matrix.rows {
        let matched = match row.kind {
            RecipeKind::Shaped { pattern, key } => shaped_matches(grid, pattern, key),
            RecipeKind::Shapeless { ingredients } => shapeless_matches(grid, ingredients),
            RecipeKind::Unsupported { name } => {
                return Err(CraftingError::UnsupportedRecipeKind(name))
            }
        };
        if matched {
            return Ok(Some(*row));
        }
    }
    Ok(None)
}

fn shaped_matches(grid: CraftingGrid<'_>, pattern: &[&str], key: &[SymbolIngredient<'_>]) -> bool {
    for (row_index, pattern_row) in pattern.iter().enumerate() {
        for (column_index, symbol) in pattern_row.chars().enumerate() {
            let slot_index = row_index * CRAFTING_GRID_WIDTH + column_index;
            let actual = grid.slots[slot_index];
            if !slot_matches_symbol(actual, symbol, key) {
                return false;
            }
        }
    }
    true
}

fn slot_matches_symbol(
    actual: Option<ItemStack<'_>>,
    symbol: char,
    key: &[SymbolIngredient<'_>],
) -> bool {
    if symbol == EMPTY_PATTERN_SYMBOL {
        return actual.is_none();
    }

    let Some(expected) = find_key_ingredient(symbol, key) else {
        return false;
    };

    matches!(
        actual,
        Some(stack) if stack.item == expected.item && stack.count >= expected.count
    )
}

fn shapeless_matches(grid: CraftingGrid<'_>, ingredients: &[ItemStack<'_>]) -> bool {
    let mut used = vec![false; ingredients.len()];
    let mut occupied_slots = EMPTY_COUNT;

    for slot in grid.slots.iter().flatten() {
        occupied_slots = occupied_slots.saturating_add(ONE_ITEM);
        let Some(index) = ingredients
            .iter()
            .enumerate()
            .find(|(index, ingredient)| {
                !used[*index] && slot.item == ingredient.item && slot.count >= ingredient.count
            })
            .map(|(index, _ingredient)| index)
        else {
            return false;
        };
        used[index] = true;
    }

    occupied_slots == ingredients.len() as u32
        && used.iter().all(|ingredient_used| *ingredient_used)
}

fn find_key_ingredient<'a>(
    symbol: char,
    key: &[SymbolIngredient<'a>],
) -> Option<SymbolIngredient<'a>> {
    key.iter()
        .copied()
        .find(|ingredient| ingredient.symbol == symbol)
}

fn can_accept_output_slot(
    output_slot: OutputSlotState<'_>,
    output: ItemStack<'_>,
    limits: CraftingLimits,
) -> bool {
    match output_slot.stack {
        None => output.count <= limits.result_slot_stack_limit,
        Some(stack) if stack.item != output.item => false,
        Some(stack) => stack.count.saturating_add(output.count) <= limits.result_slot_stack_limit,
    }
}

fn build_inventory_delta<'a>(
    target_slot: InventorySlot<'a>,
    output: ItemStack<'a>,
    limits: CraftingLimits,
) -> Result<InventoryDelta<'a>, CraftingError<'a>> {
    let count_before = target_slot
        .stack
        .map(|stack| stack.count)
        .unwrap_or(EMPTY_COUNT);
    let compatible_slot = match target_slot.stack {
        None => true,
        Some(stack) => stack.item == output.item,
    };
    let count_after = count_before.saturating_add(output.count);

    if compatible_slot && count_after <= limits.max_stack_size {
        Ok(InventoryDelta {
            slot_index: target_slot.index,
            item: output.item,
            count_before,
            count_after,
        })
    } else {
        Err(CraftingError::InventoryCapacityBlocked)
    }
}

fn consume_recipe_ingredients<'a>(
    grid: CraftingGrid<'a>,
    recipe: RecipeRow<'a>,
) -> CraftingGrid<'a> {
    match recipe.kind {
        RecipeKind::Shaped { pattern, key } => consume_shaped_ingredients(grid, pattern, key),
        RecipeKind::Shapeless { ingredients } => consume_shapeless_ingredients(grid, ingredients),
        RecipeKind::Unsupported { .. } => grid,
    }
}

fn consume_shaped_ingredients<'a>(
    grid: CraftingGrid<'a>,
    pattern: &[&str],
    key: &[SymbolIngredient<'a>],
) -> CraftingGrid<'a> {
    let mut slots = grid.slots;
    for (row_index, pattern_row) in pattern.iter().enumerate() {
        for (column_index, symbol) in pattern_row.chars().enumerate() {
            if symbol == EMPTY_PATTERN_SYMBOL {
                continue;
            }
            let Some(ingredient) = find_key_ingredient(symbol, key) else {
                continue;
            };
            let slot_index = row_index * CRAFTING_GRID_WIDTH + column_index;
            slots[slot_index] = decrement_slot(slots[slot_index], ingredient.count);
        }
    }
    CraftingGrid { slots }
}

fn consume_shapeless_ingredients<'a>(
    grid: CraftingGrid<'a>,
    ingredients: &[ItemStack<'a>],
) -> CraftingGrid<'a> {
    let mut slots = grid.slots;
    let mut used_slots = [false; CRAFTING_GRID_SLOT_COUNT];

    for ingredient in ingredients {
        let Some(slot_index) = slots
            .iter()
            .enumerate()
            .find(|(index, slot)| {
                !used_slots[*index]
                    && matches!(
                        slot,
                        Some(stack) if stack.item == ingredient.item && stack.count >= ingredient.count
                    )
            })
            .map(|(index, _slot)| index)
        else {
            continue;
        };
        used_slots[slot_index] = true;
        slots[slot_index] = decrement_slot(slots[slot_index], ingredient.count);
    }

    CraftingGrid { slots }
}

fn decrement_slot<'a>(slot: Option<ItemStack<'a>>, count: u32) -> Option<ItemStack<'a>> {
    match slot {
        Some(stack) if stack.count > count => Some(ItemStack {
            item: stack.item,
            count: stack.count - count,
        }),
        Some(_stack) => None,
        None => None,
    }
}

fn run_self_tests(fixture_path: Option<&Path>) -> Result<(), String> {
    shaped_chest_matches()?;
    shapeless_oak_planks_matches()?;
    primary_click_collection_merges_with_compatible_inventory()?;
    insufficient_stick_input_returns_no_result()?;
    blocked_output_preserves_state()?;
    missing_selected_data_fails()?;
    duplicate_recipe_ids_fail()?;
    malformed_shaped_rows_fail()?;
    malformed_shapeless_rows_fail()?;
    invalid_item_ids_fail()?;
    zero_output_counts_fail()?;
    missing_target_data_fails()?;
    unsupported_recipe_kind_fails()?;
    unsupported_collection_modes_fail()?;
    inventory_capacity_block_fails()?;
    if let Some(path) = fixture_path {
        selected_fixture_handoff_uses_core_rows(path)?;
    }
    Ok(())
}

fn shaped_chest_matches() -> Result<(), String> {
    let grid = chest_grid();
    let decision = evaluate_selected_matrix(
        grid,
        selected_matrix(),
        empty_output_slot(),
        collection_request(
            PRIMARY_CHEST_TARGET_SLOT,
            None,
            CollectionMode::PrimaryClick,
        ),
        selected_limits(),
    )
    .map_err(format_error("shaped_chest unexpected error"))?;

    match decision {
        CraftingDecision::Matched {
            recipe_id,
            output,
            grid_after,
            inventory_delta,
        } => {
            expect_equal("shaped_chest recipe id", recipe_id, CHEST_RECIPE_ID)?;
            expect_equal("shaped_chest output", output, stack(CHEST, ONE_ITEM))?;
            expect_equal("shaped_chest grid consumed", grid_after, empty_grid())?;
            expect_equal(
                "shaped_chest inventory delta",
                inventory_delta,
                InventoryDelta {
                    slot_index: PRIMARY_CHEST_TARGET_SLOT,
                    item: CHEST,
                    count_before: EMPTY_COUNT,
                    count_after: ONE_ITEM,
                },
            )
        }
        other => Err(format!(
            "shaped_chest expected matched decision, got {other:?}"
        )),
    }
}

fn shapeless_oak_planks_matches() -> Result<(), String> {
    let grid = oak_log_grid();
    let decision = evaluate_selected_matrix(
        grid,
        selected_matrix(),
        empty_output_slot(),
        collection_request(
            PRIMARY_PLANKS_TARGET_SLOT,
            None,
            CollectionMode::PrimaryClick,
        ),
        selected_limits(),
    )
    .map_err(format_error("shapeless_oak_planks unexpected error"))?;

    match decision {
        CraftingDecision::Matched {
            recipe_id,
            output,
            grid_after,
            inventory_delta,
        } => {
            expect_equal("shapeless recipe id", recipe_id, OAK_PLANKS_RECIPE_ID)?;
            expect_equal(
                "shapeless output",
                output,
                stack(OAK_PLANKS, OAK_PLANKS_OUTPUT_COUNT),
            )?;
            expect_equal("shapeless grid consumed", grid_after, empty_grid())?;
            expect_equal(
                "shapeless inventory delta",
                inventory_delta,
                InventoryDelta {
                    slot_index: PRIMARY_PLANKS_TARGET_SLOT,
                    item: OAK_PLANKS,
                    count_before: EMPTY_COUNT,
                    count_after: OAK_PLANKS_OUTPUT_COUNT,
                },
            )
        }
        other => Err(format!(
            "shapeless_oak_planks expected matched decision, got {other:?}"
        )),
    }
}

fn primary_click_collection_merges_with_compatible_inventory() -> Result<(), String> {
    let decision = evaluate_selected_matrix(
        oak_log_grid(),
        selected_matrix(),
        empty_output_slot(),
        collection_request(
            PRIMARY_PLANKS_TARGET_SLOT,
            Some(stack(OAK_PLANKS, EXISTING_TARGET_PLANKS)),
            CollectionMode::PrimaryClick,
        ),
        selected_limits(),
    )
    .map_err(format_error("primary_click_collection unexpected error"))?;

    match decision {
        CraftingDecision::Matched {
            inventory_delta, ..
        } => expect_equal(
            "primary_click inventory merge",
            inventory_delta,
            InventoryDelta {
                slot_index: PRIMARY_PLANKS_TARGET_SLOT,
                item: OAK_PLANKS,
                count_before: EXISTING_TARGET_PLANKS,
                count_after: EXPECTED_MERGED_TARGET_PLANKS,
            },
        ),
        other => Err(format!(
            "primary_click_collection expected matched decision, got {other:?}"
        )),
    }
}

fn insufficient_stick_input_returns_no_result() -> Result<(), String> {
    let grid = invalid_stick_probe_grid();
    let target_slot = inventory_slot(PRIMARY_PLANKS_TARGET_SLOT, None);
    let decision = evaluate_selected_matrix(
        grid,
        selected_matrix(),
        empty_output_slot(),
        CollectionRequest {
            mode: CollectionMode::PrimaryClick,
            target_slot,
        },
        selected_limits(),
    )
    .map_err(format_error("insufficient_stick unexpected error"))?;

    match decision {
        CraftingDecision::NoResult {
            reason,
            grid_after,
            inventory_after,
        } => {
            expect_equal(
                "insufficient_stick reason",
                reason,
                NoResultReason::SelectedProbeRejected(STICK_REJECTION_ID),
            )?;
            expect_equal("insufficient_stick grid preserved", grid_after, grid)?;
            expect_equal(
                "insufficient_stick inventory preserved",
                inventory_after,
                target_slot,
            )
        }
        other => Err(format!(
            "insufficient_stick expected no-result decision, got {other:?}"
        )),
    }
}

fn blocked_output_preserves_state() -> Result<(), String> {
    let grid = chest_grid();
    let target_slot = inventory_slot(PRIMARY_CHEST_TARGET_SLOT, None);
    let output_slot = OutputSlotState {
        stack: Some(stack(COBBLESTONE, ONE_ITEM)),
    };
    let decision = evaluate_selected_matrix(
        grid,
        selected_matrix(),
        output_slot,
        CollectionRequest {
            mode: CollectionMode::PrimaryClick,
            target_slot,
        },
        selected_limits(),
    )
    .map_err(format_error("blocked_output unexpected error"))?;

    match decision {
        CraftingDecision::OutputBlocked {
            recipe_id,
            grid_after,
            inventory_after,
            output_slot: blocked_slot,
        } => {
            expect_equal("blocked_output recipe id", recipe_id, CHEST_RECIPE_ID)?;
            expect_equal("blocked_output grid preserved", grid_after, grid)?;
            expect_equal(
                "blocked_output inventory preserved",
                inventory_after,
                target_slot,
            )?;
            expect_equal("blocked_output slot preserved", blocked_slot, output_slot)
        }
        other => Err(format!(
            "blocked_output expected output-blocked decision, got {other:?}"
        )),
    }
}

fn missing_selected_data_fails() -> Result<(), String> {
    let chest_only = [selected_chest_recipe()];
    let error = evaluate_selected_matrix(
        chest_grid(),
        RecipeMatrix { rows: &chest_only },
        empty_output_slot(),
        collection_request(
            PRIMARY_CHEST_TARGET_SLOT,
            None,
            CollectionMode::PrimaryClick,
        ),
        selected_limits(),
    )
    .expect_err("missing selected oak-planks row should fail");

    expect_equal(
        "missing_selected_data error",
        error,
        CraftingError::MissingSelectedData(OAK_PLANKS_RECIPE_ID),
    )
}

fn duplicate_recipe_ids_fail() -> Result<(), String> {
    let rows = [
        selected_chest_recipe(),
        selected_chest_recipe(),
        selected_oak_planks_recipe(),
    ];
    let error = evaluate_selected_matrix(
        chest_grid(),
        RecipeMatrix { rows: &rows },
        empty_output_slot(),
        collection_request(
            PRIMARY_CHEST_TARGET_SLOT,
            None,
            CollectionMode::PrimaryClick,
        ),
        selected_limits(),
    )
    .expect_err("duplicate recipe ids should fail");

    expect_equal(
        "duplicate_recipe_ids error",
        error,
        CraftingError::DuplicateRecipeId(CHEST_RECIPE_ID),
    )
}

fn malformed_shaped_rows_fail() -> Result<(), String> {
    let malformed_pattern = [
        MALFORMED_PATTERN_ROW,
        CHEST_PATTERN_MIDDLE,
        CHEST_PATTERN_BOTTOM,
    ];
    let rows = [
        RecipeRow {
            id: CHEST_RECIPE_ID,
            kind: RecipeKind::Shaped {
                pattern: &malformed_pattern,
                key: &CHEST_KEY,
            },
            output: stack(CHEST, ONE_ITEM),
            target: Some(TARGET_SCOPE),
        },
        selected_oak_planks_recipe(),
    ];
    let error = evaluate_selected_matrix(
        chest_grid(),
        RecipeMatrix { rows: &rows },
        empty_output_slot(),
        collection_request(
            PRIMARY_CHEST_TARGET_SLOT,
            None,
            CollectionMode::PrimaryClick,
        ),
        selected_limits(),
    )
    .expect_err("malformed shaped row should fail");

    expect_equal(
        "malformed_shaped_rows error",
        error,
        CraftingError::MalformedShapedRow(CHEST_RECIPE_ID),
    )
}

fn malformed_shapeless_rows_fail() -> Result<(), String> {
    let empty_ingredients: [ItemStack<'static>; 0] = [];
    let rows = [
        selected_chest_recipe(),
        RecipeRow {
            id: OAK_PLANKS_RECIPE_ID,
            kind: RecipeKind::Shapeless {
                ingredients: &empty_ingredients,
            },
            output: stack(OAK_PLANKS, OAK_PLANKS_OUTPUT_COUNT),
            target: Some(TARGET_SCOPE),
        },
    ];
    let error = evaluate_selected_matrix(
        oak_log_grid(),
        RecipeMatrix { rows: &rows },
        empty_output_slot(),
        collection_request(
            PRIMARY_PLANKS_TARGET_SLOT,
            None,
            CollectionMode::PrimaryClick,
        ),
        selected_limits(),
    )
    .expect_err("malformed shapeless row should fail");

    expect_equal(
        "malformed_shapeless_rows error",
        error,
        CraftingError::MalformedShapelessRow(OAK_PLANKS_RECIPE_ID),
    )
}

fn invalid_item_ids_fail() -> Result<(), String> {
    let rows = [
        RecipeRow {
            id: CHEST_RECIPE_ID,
            kind: RecipeKind::Shaped {
                pattern: &CHEST_PATTERN,
                key: &CHEST_KEY,
            },
            output: stack(INVALID_ITEM_ID, ONE_ITEM),
            target: Some(TARGET_SCOPE),
        },
        selected_oak_planks_recipe(),
    ];
    let error = evaluate_selected_matrix(
        chest_grid(),
        RecipeMatrix { rows: &rows },
        empty_output_slot(),
        collection_request(
            PRIMARY_CHEST_TARGET_SLOT,
            None,
            CollectionMode::PrimaryClick,
        ),
        selected_limits(),
    )
    .expect_err("invalid item id should fail");

    expect_equal(
        "invalid_item_ids error",
        error,
        CraftingError::InvalidItemId(INVALID_ITEM_ID),
    )
}

fn zero_output_counts_fail() -> Result<(), String> {
    let rows = [
        RecipeRow {
            id: CHEST_RECIPE_ID,
            kind: RecipeKind::Shaped {
                pattern: &CHEST_PATTERN,
                key: &CHEST_KEY,
            },
            output: stack(CHEST, EMPTY_COUNT),
            target: Some(TARGET_SCOPE),
        },
        selected_oak_planks_recipe(),
    ];
    let error = evaluate_selected_matrix(
        chest_grid(),
        RecipeMatrix { rows: &rows },
        empty_output_slot(),
        collection_request(
            PRIMARY_CHEST_TARGET_SLOT,
            None,
            CollectionMode::PrimaryClick,
        ),
        selected_limits(),
    )
    .expect_err("zero output count should fail");

    expect_equal(
        "zero_output_counts error",
        error,
        CraftingError::ZeroOutputCount(CHEST_RECIPE_ID),
    )
}

fn missing_target_data_fails() -> Result<(), String> {
    let rows = [
        RecipeRow {
            id: CHEST_RECIPE_ID,
            kind: RecipeKind::Shaped {
                pattern: &CHEST_PATTERN,
                key: &CHEST_KEY,
            },
            output: stack(CHEST, ONE_ITEM),
            target: None,
        },
        selected_oak_planks_recipe(),
    ];
    let error = evaluate_selected_matrix(
        chest_grid(),
        RecipeMatrix { rows: &rows },
        empty_output_slot(),
        collection_request(
            PRIMARY_CHEST_TARGET_SLOT,
            None,
            CollectionMode::PrimaryClick,
        ),
        selected_limits(),
    )
    .expect_err("missing target data should fail");

    expect_equal(
        "missing_target_data error",
        error,
        CraftingError::MissingTargetData(CHEST_RECIPE_ID),
    )
}

fn unsupported_recipe_kind_fails() -> Result<(), String> {
    let rows = [
        RecipeRow {
            id: CHEST_RECIPE_ID,
            kind: RecipeKind::Unsupported {
                name: UNSUPPORTED_RECIPE_KIND,
            },
            output: stack(CHEST, ONE_ITEM),
            target: Some(TARGET_SCOPE),
        },
        selected_oak_planks_recipe(),
    ];
    let error = evaluate_selected_matrix(
        chest_grid(),
        RecipeMatrix { rows: &rows },
        empty_output_slot(),
        collection_request(
            PRIMARY_CHEST_TARGET_SLOT,
            None,
            CollectionMode::PrimaryClick,
        ),
        selected_limits(),
    )
    .expect_err("unsupported recipe kind should fail");

    expect_equal(
        "unsupported_recipe_kind error",
        error,
        CraftingError::UnsupportedRecipeKind(UNSUPPORTED_RECIPE_KIND),
    )
}

fn unsupported_collection_modes_fail() -> Result<(), String> {
    for mode in UNSUPPORTED_COLLECTION_MODES {
        let error = evaluate_selected_matrix(
            chest_grid(),
            selected_matrix(),
            empty_output_slot(),
            collection_request(PRIMARY_CHEST_TARGET_SLOT, None, mode),
            selected_limits(),
        )
        .expect_err("unsupported collection mode should fail");

        expect_equal(
            "unsupported_collection_modes error",
            error,
            CraftingError::UnsupportedCollectionMode(mode),
        )?;
    }
    Ok(())
}

fn inventory_capacity_block_fails() -> Result<(), String> {
    let error = evaluate_selected_matrix(
        oak_log_grid(),
        selected_matrix(),
        empty_output_slot(),
        collection_request(
            PRIMARY_PLANKS_TARGET_SLOT,
            Some(stack(COBBLESTONE, ONE_ITEM)),
            CollectionMode::PrimaryClick,
        ),
        selected_limits(),
    )
    .expect_err("incompatible inventory target should fail");

    expect_equal(
        "inventory_capacity_block error",
        error,
        CraftingError::InventoryCapacityBlocked,
    )
}

fn selected_fixture_handoff_uses_core_rows(path: &Path) -> Result<(), String> {
    let text = fs::read_to_string(path).map_err(|error| format!("{}: {error}", path.display()))?;
    let rows = parse_selected_fixture_rows(&text)?;

    expect_equal(
        "fixture target edition",
        rows.target_edition.as_str(),
        TARGET_EDITION,
    )?;
    expect_equal(
        "fixture target version",
        rows.target_game_version.as_str(),
        TARGET_VERSION,
    )?;
    expect_equal(
        "fixture target protocol",
        rows.target_protocol,
        TARGET_PROTOCOL,
    )?;
    expect_equal(
        "fixture grid width",
        rows.grid_width,
        CRAFTING_GRID_WIDTH as u32,
    )?;
    expect_equal(
        "fixture grid height",
        rows.grid_height,
        CRAFTING_GRID_HEIGHT as u32,
    )?;
    expect_equal("fixture max stack", rows.max_stack_size, MAX_STACK_SIZE)?;
    expect_equal(
        "fixture result stack limit",
        rows.result_slot_stack_limit,
        RESULT_SLOT_STACK_LIMIT,
    )?;
    expect_equal(
        "fixture chest recipe id",
        rows.chest_recipe_id.as_str(),
        CHEST_RECIPE_ID,
    )?;
    expect_equal(
        "fixture chest key item",
        rows.chest_key_item.as_str(),
        OAK_PLANKS,
    )?;
    expect_equal(
        "fixture chest output item",
        rows.chest_output_item.as_str(),
        CHEST,
    )?;
    expect_equal(
        "fixture chest output count",
        rows.chest_output_count,
        ONE_ITEM,
    )?;
    expect_equal(
        "fixture oak planks recipe id",
        rows.oak_planks_recipe_id.as_str(),
        OAK_PLANKS_RECIPE_ID,
    )?;
    expect_equal(
        "fixture shapeless input item",
        rows.shapeless_input_item.as_str(),
        OAK_LOG,
    )?;
    expect_equal(
        "fixture shapeless output item",
        rows.shapeless_output_item.as_str(),
        OAK_PLANKS,
    )?;
    expect_equal(
        "fixture shapeless output count",
        rows.oak_planks_output_count,
        OAK_PLANKS_OUTPUT_COUNT,
    )?;
    expect_equal(
        "fixture invalid probe id",
        rows.invalid_probe_id.as_str(),
        STICK_REJECTION_ID,
    )?;
    expect_equal(
        "fixture invalid probe diagnostic",
        rows.invalid_probe_diagnostic.as_str(),
        INVALID_PROBE_DIAGNOSTIC,
    )?;

    let chest_pattern_values = [
        rows.chest_pattern_top.as_str(),
        rows.chest_pattern_middle.as_str(),
        rows.chest_pattern_bottom.as_str(),
    ];
    let chest_key_values = [SymbolIngredient {
        symbol: rows.chest_key_symbol,
        item: rows.chest_key_item.as_str(),
        count: rows.chest_key_count,
    }];
    let shapeless_ingredients = [stack(
        rows.shapeless_input_item.as_str(),
        rows.shapeless_input_count,
    )];
    let selected_rows = [
        RecipeRow {
            id: rows.chest_recipe_id.as_str(),
            kind: RecipeKind::Shaped {
                pattern: &chest_pattern_values,
                key: &chest_key_values,
            },
            output: stack(rows.chest_output_item.as_str(), rows.chest_output_count),
            target: Some(TARGET_SCOPE),
        },
        RecipeRow {
            id: rows.oak_planks_recipe_id.as_str(),
            kind: RecipeKind::Shapeless {
                ingredients: &shapeless_ingredients,
            },
            output: stack(
                rows.shapeless_output_item.as_str(),
                rows.oak_planks_output_count,
            ),
            target: Some(TARGET_SCOPE),
        },
    ];
    let matrix = RecipeMatrix {
        rows: &selected_rows,
    };
    let limits = CraftingLimits {
        max_stack_size: rows.max_stack_size,
        result_slot_stack_limit: rows.result_slot_stack_limit,
    };

    let chest_decision = evaluate_selected_matrix(
        fixture_chest_grid(&rows),
        matrix,
        empty_output_slot(),
        collection_request(
            to_slot_index(rows.chest_target_slot, "fixture chest target slot")?,
            None,
            CollectionMode::PrimaryClick,
        ),
        limits,
    )
    .map_err(format_error("fixture chest handoff unexpected error"))?;
    match chest_decision {
        CraftingDecision::Matched {
            recipe_id,
            output,
            grid_after,
            inventory_delta,
        } => {
            expect_equal("fixture chest matched id", recipe_id, CHEST_RECIPE_ID)?;
            expect_equal(
                "fixture chest matched output",
                output,
                stack(CHEST, ONE_ITEM),
            )?;
            expect_equal("fixture chest grid consumed", grid_after, empty_grid())?;
            expect_equal(
                "fixture chest inventory delta",
                inventory_delta,
                InventoryDelta {
                    slot_index: to_slot_index(rows.chest_target_slot, "fixture chest target slot")?,
                    item: CHEST,
                    count_before: EMPTY_COUNT,
                    count_after: ONE_ITEM,
                },
            )
        }
        other => Err(format!(
            "fixture chest handoff expected matched decision, got {other:?}"
        )),
    }?;

    let shapeless_decision = evaluate_selected_matrix(
        fixture_shapeless_grid(&rows),
        matrix,
        empty_output_slot(),
        collection_request(
            to_slot_index(
                rows.oak_planks_target_slot,
                "fixture oak-planks target slot",
            )?,
            None,
            CollectionMode::PrimaryClick,
        ),
        limits,
    )
    .map_err(format_error("fixture shapeless handoff unexpected error"))?;
    match shapeless_decision {
        CraftingDecision::Matched {
            recipe_id,
            output,
            grid_after,
            inventory_delta,
        } => {
            expect_equal(
                "fixture shapeless matched id",
                recipe_id,
                OAK_PLANKS_RECIPE_ID,
            )?;
            expect_equal(
                "fixture shapeless matched output",
                output,
                stack(OAK_PLANKS, OAK_PLANKS_OUTPUT_COUNT),
            )?;
            expect_equal("fixture shapeless grid consumed", grid_after, empty_grid())?;
            expect_equal(
                "fixture shapeless inventory delta",
                inventory_delta,
                InventoryDelta {
                    slot_index: to_slot_index(
                        rows.oak_planks_target_slot,
                        "fixture oak-planks target slot",
                    )?,
                    item: OAK_PLANKS,
                    count_before: EMPTY_COUNT,
                    count_after: OAK_PLANKS_OUTPUT_COUNT,
                },
            )
        }
        other => Err(format!(
            "fixture shapeless handoff expected matched decision, got {other:?}"
        )),
    }?;

    let invalid_grid = fixture_invalid_probe_grid(&rows);
    let invalid_target_slot = inventory_slot(
        to_slot_index(rows.oak_planks_target_slot, "fixture invalid target slot")?,
        None,
    );
    let invalid_decision = evaluate_selected_matrix(
        invalid_grid,
        matrix,
        empty_output_slot(),
        CollectionRequest {
            mode: CollectionMode::PrimaryClick,
            target_slot: invalid_target_slot,
        },
        limits,
    )
    .map_err(format_error("fixture invalid probe unexpected error"))?;
    match invalid_decision {
        CraftingDecision::NoResult {
            reason,
            grid_after,
            inventory_after,
        } => {
            expect_equal(
                "fixture invalid no-result reason",
                reason,
                NoResultReason::SelectedProbeRejected(STICK_REJECTION_ID),
            )?;
            expect_equal("fixture invalid grid preserved", grid_after, invalid_grid)?;
            expect_equal(
                "fixture invalid inventory preserved",
                inventory_after,
                invalid_target_slot,
            )
        }
        other => Err(format!(
            "fixture invalid probe expected no-result decision, got {other:?}"
        )),
    }
}

fn fixture_chest_grid<'a>(rows: &'a SelectedFixtureRows) -> CraftingGrid<'a> {
    let mut grid = empty_grid();
    let plank = stack(rows.chest_key_item.as_str(), rows.chest_key_count);
    grid.slots[SLOT_TOP_LEFT] = Some(plank);
    grid.slots[SLOT_TOP_MIDDLE] = Some(plank);
    grid.slots[SLOT_TOP_RIGHT] = Some(plank);
    grid.slots[SLOT_MIDDLE_LEFT] = Some(plank);
    grid.slots[SLOT_MIDDLE_CENTER] = None;
    grid.slots[SLOT_MIDDLE_RIGHT] = Some(plank);
    grid.slots[SLOT_BOTTOM_LEFT] = Some(plank);
    grid.slots[SLOT_BOTTOM_MIDDLE] = Some(plank);
    grid.slots[SLOT_BOTTOM_RIGHT] = Some(plank);
    grid
}

fn fixture_shapeless_grid<'a>(rows: &'a SelectedFixtureRows) -> CraftingGrid<'a> {
    let mut grid = empty_grid();
    grid.slots[SLOT_TOP_LEFT] = Some(stack(
        rows.shapeless_input_item.as_str(),
        rows.shapeless_input_count,
    ));
    grid
}

fn fixture_invalid_probe_grid<'a>(rows: &'a SelectedFixtureRows) -> CraftingGrid<'a> {
    let mut grid = empty_grid();
    grid.slots[SLOT_TOP_LEFT] = Some(stack(
        rows.invalid_probe_input_item.as_str(),
        rows.invalid_probe_input_count,
    ));
    grid
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SelectedFixtureRows {
    target_edition: String,
    target_game_version: String,
    target_protocol: u32,
    grid_width: u32,
    grid_height: u32,
    max_stack_size: u32,
    result_slot_stack_limit: u32,
    chest_target_slot: u32,
    oak_planks_target_slot: u32,
    chest_recipe_id: String,
    chest_key_symbol: char,
    chest_pattern_top: String,
    chest_pattern_middle: String,
    chest_pattern_bottom: String,
    chest_key_item: String,
    chest_key_count: u32,
    chest_output_item: String,
    chest_output_count: u32,
    oak_planks_recipe_id: String,
    shapeless_input_item: String,
    shapeless_input_count: u32,
    shapeless_output_item: String,
    oak_planks_output_count: u32,
    invalid_probe_id: String,
    invalid_probe_input_item: String,
    invalid_probe_input_count: u32,
    invalid_probe_diagnostic: String,
}

fn parse_selected_fixture_rows(text: &str) -> Result<SelectedFixtureRows, String> {
    require_fixture_string(text, SHAPED_KIND_BINDING, SHAPED_KIND)?;
    require_fixture_string(text, SHAPELESS_KIND_BINDING, SHAPELESS_KIND)?;
    require_fixture_string(
        text,
        REJECTED_NO_RESULT_KIND_BINDING,
        REJECTED_NO_RESULT_KIND,
    )?;
    require_fixture_string(text, COLLECTION_MODE_BINDING, COLLECTION_MODE)?;

    Ok(SelectedFixtureRows {
        target_edition: read_string_binding(text, TARGET_EDITION_BINDING)?,
        target_game_version: read_string_binding(text, TARGET_GAME_VERSION_BINDING)?,
        target_protocol: read_number_binding(text, TARGET_PROTOCOL_BINDING)?,
        grid_width: read_number_binding(text, GRID_WIDTH_BINDING)?,
        grid_height: read_number_binding(text, GRID_HEIGHT_BINDING)?,
        max_stack_size: read_number_binding(text, MAX_STACK_SIZE_BINDING)?,
        result_slot_stack_limit: read_number_binding(text, RESULT_SLOT_STACK_LIMIT_BINDING)?,
        chest_target_slot: read_number_binding(text, CHEST_TARGET_SLOT_BINDING)?,
        oak_planks_target_slot: read_number_binding(text, OAK_PLANKS_TARGET_SLOT_BINDING)?,
        chest_recipe_id: read_string_binding(text, CHEST_RECIPE_ID_BINDING)?,
        chest_key_symbol: read_symbol_binding(text, CHEST_KEY_SYMBOL_BINDING)?,
        chest_pattern_top: read_string_binding(text, CHEST_PATTERN_TOP_BINDING)?,
        chest_pattern_middle: read_string_binding(text, CHEST_PATTERN_MIDDLE_BINDING)?,
        chest_pattern_bottom: read_string_binding(text, CHEST_PATTERN_BOTTOM_BINDING)?,
        chest_key_item: read_string_binding(text, CHEST_KEY_ITEM_BINDING)?,
        chest_key_count: read_number_binding(text, CHEST_KEY_COUNT_BINDING)?,
        chest_output_item: read_string_binding(text, CHEST_OUTPUT_ITEM_BINDING)?,
        chest_output_count: read_number_binding(text, CHEST_OUTPUT_COUNT_BINDING)?,
        oak_planks_recipe_id: read_string_binding(text, OAK_PLANKS_RECIPE_ID_BINDING)?,
        shapeless_input_item: read_string_binding(text, SHAPELESS_INPUT_ITEM_BINDING)?,
        shapeless_input_count: read_number_binding(text, SHAPELESS_INPUT_COUNT_BINDING)?,
        shapeless_output_item: read_string_binding(text, SHAPELESS_OUTPUT_ITEM_BINDING)?,
        oak_planks_output_count: read_number_binding(text, OAK_PLANKS_OUTPUT_COUNT_BINDING)?,
        invalid_probe_id: read_string_binding(text, INVALID_PROBE_ID_BINDING)?,
        invalid_probe_input_item: read_string_binding(text, INVALID_PROBE_INPUT_ITEM_BINDING)?,
        invalid_probe_input_count: read_number_binding(text, INVALID_PROBE_INPUT_COUNT_BINDING)?,
        invalid_probe_diagnostic: read_string_binding(text, INVALID_PROBE_DIAGNOSTIC_BINDING)?,
    })
}

fn require_fixture_string(text: &str, binding: &str, expected: &str) -> Result<(), String> {
    let actual = read_string_binding(text, binding)?;
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "fixture binding {binding} expected {expected:?}, got {actual:?}"
        ))
    }
}

fn read_symbol_binding(text: &str, binding: &str) -> Result<char, String> {
    let value = read_string_binding(text, binding)?;
    let mut chars = value.chars();
    let Some(symbol) = chars.next() else {
        return Err(format!("fixture symbol binding {binding} is empty"));
    };
    if chars.next().is_some() {
        return Err(format!(
            "fixture symbol binding {binding} must contain one character, got {value:?}"
        ));
    }
    Ok(symbol)
}

fn read_string_binding(text: &str, binding: &str) -> Result<String, String> {
    let needle = format!("let {binding} = \"");
    let Some(start) = text.find(&needle).map(|start| start + needle.len()) else {
        return Err(format!("fixture missing string binding {binding}"));
    };
    let remainder = &text[start..];
    let Some(end) = remainder.find('"') else {
        return Err(format!("fixture string binding {binding} is unterminated"));
    };
    Ok(remainder[..end].to_string())
}

fn read_number_binding(text: &str, binding: &str) -> Result<u32, String> {
    let needle = format!("let {binding} = ");
    let Some(start) = text.find(&needle).map(|start| start + needle.len()) else {
        return Err(format!("fixture missing numeric binding {binding}"));
    };
    let remainder = &text[start..];
    let raw_number = remainder
        .chars()
        .take_while(|character| character.is_ascii_digit() || *character == '_')
        .collect::<String>();
    if raw_number.is_empty() {
        return Err(format!("fixture numeric binding {binding} is empty"));
    }
    let normalized = raw_number.replace('_', "");
    u32::from_str_radix(&normalized, RADIX_TEN)
        .map_err(|error| format!("fixture numeric binding {binding} is invalid: {error}"))
}

fn to_slot_index(value: u32, label: &str) -> Result<u16, String> {
    u16::try_from(value).map_err(|error| format!("{label} {value} is invalid: {error}"))
}

fn selected_matrix<'a>() -> RecipeMatrix<'a> {
    RecipeMatrix {
        rows: &SELECTED_RECIPES,
    }
}

fn selected_chest_recipe<'a>() -> RecipeRow<'a> {
    SELECTED_RECIPES[0]
}

fn selected_oak_planks_recipe<'a>() -> RecipeRow<'a> {
    SELECTED_RECIPES[1]
}

fn selected_limits() -> CraftingLimits {
    CraftingLimits {
        max_stack_size: MAX_STACK_SIZE,
        result_slot_stack_limit: RESULT_SLOT_STACK_LIMIT,
    }
}

fn empty_output_slot<'a>() -> OutputSlotState<'a> {
    OutputSlotState { stack: None }
}

fn collection_request<'a>(
    target_slot: u16,
    target_stack: Option<ItemStack<'a>>,
    mode: CollectionMode,
) -> CollectionRequest<'a> {
    CollectionRequest {
        mode,
        target_slot: inventory_slot(target_slot, target_stack),
    }
}

fn inventory_slot<'a>(index: u16, stack: Option<ItemStack<'a>>) -> InventorySlot<'a> {
    InventorySlot { index, stack }
}

fn empty_grid<'a>() -> CraftingGrid<'a> {
    CraftingGrid {
        slots: [None; CRAFTING_GRID_SLOT_COUNT],
    }
}

fn chest_grid<'a>() -> CraftingGrid<'a> {
    let mut grid = empty_grid();
    grid.slots[SLOT_TOP_LEFT] = Some(stack(OAK_PLANKS, ONE_ITEM));
    grid.slots[SLOT_TOP_MIDDLE] = Some(stack(OAK_PLANKS, ONE_ITEM));
    grid.slots[SLOT_TOP_RIGHT] = Some(stack(OAK_PLANKS, ONE_ITEM));
    grid.slots[SLOT_MIDDLE_LEFT] = Some(stack(OAK_PLANKS, ONE_ITEM));
    grid.slots[SLOT_MIDDLE_CENTER] = None;
    grid.slots[SLOT_MIDDLE_RIGHT] = Some(stack(OAK_PLANKS, ONE_ITEM));
    grid.slots[SLOT_BOTTOM_LEFT] = Some(stack(OAK_PLANKS, ONE_ITEM));
    grid.slots[SLOT_BOTTOM_MIDDLE] = Some(stack(OAK_PLANKS, ONE_ITEM));
    grid.slots[SLOT_BOTTOM_RIGHT] = Some(stack(OAK_PLANKS, ONE_ITEM));
    grid
}

fn oak_log_grid<'a>() -> CraftingGrid<'a> {
    let mut grid = empty_grid();
    grid.slots[SLOT_TOP_LEFT] = Some(stack(OAK_LOG, ONE_ITEM));
    grid
}

fn invalid_stick_probe_grid<'a>() -> CraftingGrid<'a> {
    let mut grid = empty_grid();
    grid.slots[SLOT_TOP_LEFT] = Some(stack(OAK_PLANKS, ONE_ITEM));
    grid
}

fn stack(item: &str, count: u32) -> ItemStack<'_> {
    ItemStack { item, count }
}

fn expect_equal<T>(name: &str, actual: T, expected: T) -> Result<(), String>
where
    T: PartialEq + Debug,
{
    if actual == expected {
        Ok(())
    } else {
        Err(format!("{name}: expected {expected:?}, got {actual:?}"))
    }
}

fn format_error(context: &'static str) -> impl FnOnce(CraftingError<'_>) -> String {
    move |error| format!("{context}: {error:?}")
}
