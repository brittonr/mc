#!/usr/bin/env -S CARGO_TARGET_DIR=target/check-crafting-recipe-core-script nix shell "github:nix-community/fenix?rev=092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain" nixpkgs#gcc -c cargo -q -Zscript

use std::env;
use std::fmt::Debug;
use std::process::ExitCode;

const SELF_TEST_FLAG: &str = "--self-test";
const HELP_FLAG: &str = "--help";
const HELP_TEXT: &str = "usage: check_crafting_recipe_core.rs [--self-test]";
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
        Ok(Command::SelfTest) => run_and_report_self_tests(),
        Err(error) => {
            eprintln!("{error}");
            FAILURE
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Command {
    SelfTest,
    Help,
}

fn parse_command() -> Result<Command, String> {
    let mut command = Command::SelfTest;
    for arg in env::args().skip(1) {
        if arg == SELF_TEST_FLAG {
            command = Command::SelfTest;
            continue;
        }
        if arg == HELP_FLAG {
            command = Command::Help;
            continue;
        }
        return Err(format!("unknown argument: {arg}"));
    }
    Ok(command)
}

fn run_and_report_self_tests() -> ExitCode {
    match run_self_tests() {
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

fn run_self_tests() -> Result<(), String> {
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
