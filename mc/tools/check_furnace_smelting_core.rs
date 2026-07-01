#!/usr/bin/env -S CARGO_TARGET_DIR=target/check-furnace-smelting-core-script nix shell "github:nix-community/fenix?rev=092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain" nixpkgs#gcc -c cargo -q -Zscript

use std::env;
use std::fmt::Debug;
use std::process::ExitCode;

const SELF_TEST_FLAG: &str = "--self-test";
const HELP_FLAG: &str = "--help";
const SUCCESS_MESSAGE: &str = "furnace smelting selected-row core check passed";
const HELP_TEXT: &str = "usage: check_furnace_smelting_core.rs [--self-test]";
const SUCCESS: ExitCode = ExitCode::SUCCESS;
const FAILURE: ExitCode = ExitCode::FAILURE;

const STANDARD_FURNACE_COOK_TICKS: u32 = 200;
const MAX_STACK_SIZE: u32 = 64;
const TICK_INCREMENT: u32 = 1;
const ONE_ITEM: u32 = 1;
const EMPTY_COUNT: u32 = 0;
const INITIAL_COOK_PROGRESS: u32 = 0;
const INITIAL_RECIPE_COUNT: u32 = 0;
const NO_BURN_TICKS: u32 = 0;
const COAL_BURN_TICKS: u32 = 1_600;
const SELECTED_RECIPE_OUTPUT_COUNT: u32 = 1;
const COMPATIBLE_OUTPUT_START_COUNT: u32 = 7;
const TWO_INPUT_ITEMS: u32 = 2;
const NEAR_COMPLETE_PROGRESS: u32 = STANDARD_FURNACE_COOK_TICKS - TICK_INCREMENT;
const EXPECTED_ONE_COMPLETED_RECIPE: u32 = 1;
const EXPECTED_MERGED_OUTPUT_COUNT: u32 = COMPATIBLE_OUTPUT_START_COUNT + SELECTED_RECIPE_OUTPUT_COUNT;
const EXPECTED_COAL_AFTER_START: u32 = COAL_BURN_TICKS - TICK_INCREMENT;
const EXPECTED_PROGRESS_AFTER_ONE_TICK: u32 = INITIAL_COOK_PROGRESS + TICK_INCREMENT;
const EXPECTED_INPUT_AFTER_COMPLETION: u32 = TWO_INPUT_ITEMS - ONE_ITEM;

const RAW_IRON: &str = "minecraft:raw_iron";
const IRON_INGOT: &str = "minecraft:iron_ingot";
const GOLD_INGOT: &str = "minecraft:gold_ingot";
const COBBLESTONE: &str = "minecraft:cobblestone";
const COAL: &str = "minecraft:coal";
const EMPTY_ITEM_ID: &str = "";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FurnaceKind {
    Standard,
    Smoker,
    BlastFurnace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ItemStack<'a> {
    item: &'a str,
    count: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RecipeRow<'a> {
    input: &'a str,
    output: &'a str,
    output_count: u32,
    cook_ticks: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FuelRow<'a> {
    item: &'a str,
    burn_ticks: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FurnaceLimits {
    max_stack_size: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FurnaceState<'a> {
    kind: FurnaceKind,
    input: Option<ItemStack<'a>>,
    fuel: Option<ItemStack<'a>>,
    output: Option<ItemStack<'a>>,
    cook_progress_ticks: u32,
    remaining_burn_ticks: u32,
    recipes_completed: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FurnaceTick<'a> {
    state: FurnaceState<'a>,
    transition: FurnaceTransition,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FurnaceTransition {
    StartedFuel,
    AdvancedCooking,
    ProducedOutput,
    PausedNoFuel,
    PausedNoRecipe,
    PausedOutputBlocked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FurnaceError {
    UnsupportedFurnaceKind,
    MalformedRecipeRow,
    MalformedFuelRow,
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
    let mut saw_self_test = false;

    for arg in env::args().skip(1) {
        if arg == SELF_TEST_FLAG {
            saw_self_test = true;
        } else if arg == HELP_FLAG {
            return Ok(Command::Help);
        } else {
            return Err(format!("unknown argument: {arg}"));
        }
    }

    let _self_test_requested = saw_self_test;
    Ok(Command::SelfTest)
}

fn run_and_report_self_tests() -> ExitCode {
    match run_self_tests() {
        Ok(()) => {
            println!("{SUCCESS_MESSAGE}");
            SUCCESS
        }
        Err(error) => {
            eprintln!("furnace smelting selected-row core check failed: {error}");
            FAILURE
        }
    }
}

fn tick_selected_standard_furnace<'a>(
    state: FurnaceState<'a>,
    recipes: &[RecipeRow<'a>],
    fuels: &[FuelRow<'a>],
    limits: FurnaceLimits,
) -> Result<FurnaceTick<'a>, FurnaceError> {
    if state.kind != FurnaceKind::Standard {
        return Err(FurnaceError::UnsupportedFurnaceKind);
    }

    let Some(input) = state.input else {
        return Ok(paused(state, FurnaceTransition::PausedNoRecipe));
    };

    if input.count == EMPTY_COUNT {
        return Ok(paused(state, FurnaceTransition::PausedNoRecipe));
    }

    let Some(recipe) = find_recipe(input.item, recipes) else {
        return Ok(paused(state, FurnaceTransition::PausedNoRecipe));
    };

    validate_recipe_row(recipe, limits)?;

    if !can_accept_output(state.output, recipe, limits) {
        return Ok(paused(state, FurnaceTransition::PausedOutputBlocked));
    }

    let mut next = state;
    let mut transition = FurnaceTransition::AdvancedCooking;

    if next.remaining_burn_ticks == NO_BURN_TICKS {
        let Some(fuel) = consume_one_fuel(next.fuel, fuels)? else {
            return Ok(paused(state, FurnaceTransition::PausedNoFuel));
        };
        next.fuel = fuel.next_stack;
        next.remaining_burn_ticks = fuel.burn_ticks;
        transition = FurnaceTransition::StartedFuel;
    }

    next.remaining_burn_ticks = next.remaining_burn_ticks.saturating_sub(TICK_INCREMENT);
    next.cook_progress_ticks = next.cook_progress_ticks.saturating_add(TICK_INCREMENT);

    if next.cook_progress_ticks >= recipe.cook_ticks {
        next.input = consume_one_input(input);
        next.output = add_recipe_output(next.output, recipe);
        next.cook_progress_ticks = INITIAL_COOK_PROGRESS;
        next.recipes_completed = next.recipes_completed.saturating_add(ONE_ITEM);
        transition = FurnaceTransition::ProducedOutput;
    }

    Ok(FurnaceTick {
        state: next,
        transition,
    })
}

fn paused<'a>(state: FurnaceState<'a>, transition: FurnaceTransition) -> FurnaceTick<'a> {
    FurnaceTick { state, transition }
}

fn find_recipe<'a>(input_item: &str, recipes: &[RecipeRow<'a>]) -> Option<RecipeRow<'a>> {
    recipes
        .iter()
        .copied()
        .find(|recipe| recipe.input == input_item)
}

fn validate_recipe_row(recipe: RecipeRow<'_>, limits: FurnaceLimits) -> Result<(), FurnaceError> {
    let has_valid_items = !recipe.input.is_empty() && !recipe.output.is_empty();
    let has_valid_output_count = recipe.output_count > EMPTY_COUNT && recipe.output_count <= limits.max_stack_size;
    let has_valid_cook_ticks = recipe.cook_ticks > NO_BURN_TICKS;

    if has_valid_items && has_valid_output_count && has_valid_cook_ticks {
        Ok(())
    } else {
        Err(FurnaceError::MalformedRecipeRow)
    }
}

fn validate_fuel_row(fuel: FuelRow<'_>) -> Result<(), FurnaceError> {
    let has_valid_item = !fuel.item.is_empty();
    let has_valid_burn_ticks = fuel.burn_ticks > NO_BURN_TICKS;

    if has_valid_item && has_valid_burn_ticks {
        Ok(())
    } else {
        Err(FurnaceError::MalformedFuelRow)
    }
}

fn can_accept_output(output: Option<ItemStack<'_>>, recipe: RecipeRow<'_>, limits: FurnaceLimits) -> bool {
    match output {
        Some(stack) if stack.item != recipe.output => false,
        Some(stack) => stack.count.saturating_add(recipe.output_count) <= limits.max_stack_size,
        None => recipe.output_count <= limits.max_stack_size,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ConsumedFuel<'a> {
    next_stack: Option<ItemStack<'a>>,
    burn_ticks: u32,
}

fn consume_one_fuel<'a>(
    fuel_stack: Option<ItemStack<'a>>,
    fuels: &[FuelRow<'a>],
) -> Result<Option<ConsumedFuel<'a>>, FurnaceError> {
    let Some(stack) = fuel_stack else {
        return Ok(None);
    };

    if stack.count == EMPTY_COUNT {
        return Ok(None);
    }

    let Some(fuel) = fuels.iter().copied().find(|fuel| fuel.item == stack.item) else {
        return Ok(None);
    };

    validate_fuel_row(fuel)?;

    Ok(Some(ConsumedFuel {
        next_stack: decrement_stack(stack),
        burn_ticks: fuel.burn_ticks,
    }))
}

fn consume_one_input<'a>(input: ItemStack<'a>) -> Option<ItemStack<'a>> {
    decrement_stack(input)
}

fn decrement_stack(stack: ItemStack<'_>) -> Option<ItemStack<'_>> {
    if stack.count <= ONE_ITEM {
        None
    } else {
        Some(ItemStack {
            item: stack.item,
            count: stack.count - ONE_ITEM,
        })
    }
}

fn add_recipe_output<'a>(output: Option<ItemStack<'a>>, recipe: RecipeRow<'a>) -> Option<ItemStack<'a>> {
    match output {
        Some(stack) => Some(ItemStack {
            item: stack.item,
            count: stack.count + recipe.output_count,
        }),
        None => Some(ItemStack {
            item: recipe.output,
            count: recipe.output_count,
        }),
    }
}

fn run_self_tests() -> Result<(), String> {
    fuel_start_consumes_one_fuel_and_advances()?;
    active_burn_advances_without_extra_fuel()?;
    compatible_output_merges_on_completion()?;
    completed_cook_produces_exact_output()?;
    missing_recipe_preserves_fuel()?;
    missing_fuel_pauses()?;
    wrong_output_item_blocks()?;
    full_output_stack_blocks()?;
    malformed_recipe_row_fails()?;
    unsupported_smoker_kind_fails()?;
    unsupported_blast_furnace_kind_fails()?;
    malformed_fuel_row_fails()?;
    Ok(())
}

fn fuel_start_consumes_one_fuel_and_advances() -> Result<(), String> {
    let state = base_state(
        Some(stack(RAW_IRON, ONE_ITEM)),
        Some(stack(COAL, ONE_ITEM)),
        None,
        INITIAL_COOK_PROGRESS,
        NO_BURN_TICKS,
    );

    let tick = tick_selected_standard_furnace(state, &selected_recipes(), &selected_fuels(), selected_limits())
        .map_err(format_error("fuel_start unexpected error"))?;

    expect_equal("fuel_start transition", tick.transition, FurnaceTransition::StartedFuel)?;
    expect_equal("fuel_start fuel consumed", tick.state.fuel, None)?;
    expect_equal(
        "fuel_start remaining burn",
        tick.state.remaining_burn_ticks,
        EXPECTED_COAL_AFTER_START,
    )?;
    expect_equal(
        "fuel_start cook progress",
        tick.state.cook_progress_ticks,
        EXPECTED_PROGRESS_AFTER_ONE_TICK,
    )?;
    Ok(())
}

fn active_burn_advances_without_extra_fuel() -> Result<(), String> {
    let starting_fuel = Some(stack(COAL, ONE_ITEM));
    let state = base_state(
        Some(stack(RAW_IRON, ONE_ITEM)),
        starting_fuel,
        None,
        INITIAL_COOK_PROGRESS,
        COAL_BURN_TICKS,
    );

    let tick = tick_selected_standard_furnace(state, &selected_recipes(), &selected_fuels(), selected_limits())
        .map_err(format_error("active_burn unexpected error"))?;

    expect_equal("active_burn transition", tick.transition, FurnaceTransition::AdvancedCooking)?;
    expect_equal("active_burn fuel preserved", tick.state.fuel, starting_fuel)?;
    expect_equal(
        "active_burn remaining burn",
        tick.state.remaining_burn_ticks,
        EXPECTED_COAL_AFTER_START,
    )?;
    expect_equal(
        "active_burn cook progress",
        tick.state.cook_progress_ticks,
        EXPECTED_PROGRESS_AFTER_ONE_TICK,
    )?;
    Ok(())
}

fn compatible_output_merges_on_completion() -> Result<(), String> {
    let state = base_state(
        Some(stack(RAW_IRON, ONE_ITEM)),
        None,
        Some(stack(IRON_INGOT, COMPATIBLE_OUTPUT_START_COUNT)),
        NEAR_COMPLETE_PROGRESS,
        TICK_INCREMENT,
    );

    let tick = tick_selected_standard_furnace(state, &selected_recipes(), &selected_fuels(), selected_limits())
        .map_err(format_error("compatible_output unexpected error"))?;

    expect_equal("compatible_output transition", tick.transition, FurnaceTransition::ProducedOutput)?;
    expect_equal(
        "compatible_output output count",
        tick.state.output,
        Some(stack(IRON_INGOT, EXPECTED_MERGED_OUTPUT_COUNT)),
    )?;
    expect_equal("compatible_output input consumed", tick.state.input, None)?;
    Ok(())
}

fn completed_cook_produces_exact_output() -> Result<(), String> {
    let state = base_state(
        Some(stack(RAW_IRON, TWO_INPUT_ITEMS)),
        None,
        None,
        NEAR_COMPLETE_PROGRESS,
        TICK_INCREMENT,
    );

    let tick = tick_selected_standard_furnace(state, &selected_recipes(), &selected_fuels(), selected_limits())
        .map_err(format_error("completed_cook unexpected error"))?;

    expect_equal("completed_cook transition", tick.transition, FurnaceTransition::ProducedOutput)?;
    expect_equal(
        "completed_cook input decremented",
        tick.state.input,
        Some(stack(RAW_IRON, EXPECTED_INPUT_AFTER_COMPLETION)),
    )?;
    expect_equal(
        "completed_cook output produced",
        tick.state.output,
        Some(stack(IRON_INGOT, SELECTED_RECIPE_OUTPUT_COUNT)),
    )?;
    expect_equal(
        "completed_cook progress reset",
        tick.state.cook_progress_ticks,
        INITIAL_COOK_PROGRESS,
    )?;
    expect_equal(
        "completed_cook count incremented",
        tick.state.recipes_completed,
        EXPECTED_ONE_COMPLETED_RECIPE,
    )?;
    Ok(())
}

fn missing_recipe_preserves_fuel() -> Result<(), String> {
    let starting_fuel = Some(stack(COAL, ONE_ITEM));
    let state = base_state(
        Some(stack(COBBLESTONE, ONE_ITEM)),
        starting_fuel,
        None,
        INITIAL_COOK_PROGRESS,
        NO_BURN_TICKS,
    );

    let tick = tick_selected_standard_furnace(state, &selected_recipes(), &selected_fuels(), selected_limits())
        .map_err(format_error("missing_recipe unexpected error"))?;

    expect_equal("missing_recipe transition", tick.transition, FurnaceTransition::PausedNoRecipe)?;
    expect_equal("missing_recipe state preserved", tick.state, state)?;
    Ok(())
}

fn missing_fuel_pauses() -> Result<(), String> {
    let state = base_state(
        Some(stack(RAW_IRON, ONE_ITEM)),
        None,
        None,
        INITIAL_COOK_PROGRESS,
        NO_BURN_TICKS,
    );

    let tick = tick_selected_standard_furnace(state, &selected_recipes(), &selected_fuels(), selected_limits())
        .map_err(format_error("missing_fuel unexpected error"))?;

    expect_equal("missing_fuel transition", tick.transition, FurnaceTransition::PausedNoFuel)?;
    expect_equal("missing_fuel state preserved", tick.state, state)?;
    Ok(())
}

fn wrong_output_item_blocks() -> Result<(), String> {
    let state = base_state(
        Some(stack(RAW_IRON, ONE_ITEM)),
        Some(stack(COAL, ONE_ITEM)),
        Some(stack(GOLD_INGOT, ONE_ITEM)),
        INITIAL_COOK_PROGRESS,
        NO_BURN_TICKS,
    );

    let tick = tick_selected_standard_furnace(state, &selected_recipes(), &selected_fuels(), selected_limits())
        .map_err(format_error("wrong_output unexpected error"))?;

    expect_equal("wrong_output transition", tick.transition, FurnaceTransition::PausedOutputBlocked)?;
    expect_equal("wrong_output state preserved", tick.state, state)?;
    Ok(())
}

fn full_output_stack_blocks() -> Result<(), String> {
    let state = base_state(
        Some(stack(RAW_IRON, ONE_ITEM)),
        Some(stack(COAL, ONE_ITEM)),
        Some(stack(IRON_INGOT, MAX_STACK_SIZE)),
        INITIAL_COOK_PROGRESS,
        NO_BURN_TICKS,
    );

    let tick = tick_selected_standard_furnace(state, &selected_recipes(), &selected_fuels(), selected_limits())
        .map_err(format_error("full_output unexpected error"))?;

    expect_equal("full_output transition", tick.transition, FurnaceTransition::PausedOutputBlocked)?;
    expect_equal("full_output state preserved", tick.state, state)?;
    Ok(())
}

fn malformed_recipe_row_fails() -> Result<(), String> {
    let state = base_state(
        Some(stack(RAW_IRON, ONE_ITEM)),
        Some(stack(COAL, ONE_ITEM)),
        None,
        INITIAL_COOK_PROGRESS,
        NO_BURN_TICKS,
    );
    let malformed_recipes = [RecipeRow {
        input: RAW_IRON,
        output: EMPTY_ITEM_ID,
        output_count: SELECTED_RECIPE_OUTPUT_COUNT,
        cook_ticks: STANDARD_FURNACE_COOK_TICKS,
    }];

    let error = tick_selected_standard_furnace(state, &malformed_recipes, &selected_fuels(), selected_limits())
        .expect_err("malformed recipe should fail");

    expect_equal("malformed_recipe error", error, FurnaceError::MalformedRecipeRow)
}

fn unsupported_smoker_kind_fails() -> Result<(), String> {
    unsupported_kind_fails(FurnaceKind::Smoker, "unsupported_smoker")
}

fn unsupported_blast_furnace_kind_fails() -> Result<(), String> {
    unsupported_kind_fails(FurnaceKind::BlastFurnace, "unsupported_blast_furnace")
}

fn unsupported_kind_fails(kind: FurnaceKind, test_name: &str) -> Result<(), String> {
    let mut state = base_state(
        Some(stack(RAW_IRON, ONE_ITEM)),
        Some(stack(COAL, ONE_ITEM)),
        None,
        INITIAL_COOK_PROGRESS,
        NO_BURN_TICKS,
    );
    state.kind = kind;

    let error = tick_selected_standard_furnace(state, &selected_recipes(), &selected_fuels(), selected_limits())
        .expect_err("unsupported furnace kind should fail");

    expect_equal(test_name, error, FurnaceError::UnsupportedFurnaceKind)
}

fn malformed_fuel_row_fails() -> Result<(), String> {
    let state = base_state(
        Some(stack(RAW_IRON, ONE_ITEM)),
        Some(stack(COAL, ONE_ITEM)),
        None,
        INITIAL_COOK_PROGRESS,
        NO_BURN_TICKS,
    );
    let malformed_fuels = [FuelRow {
        item: COAL,
        burn_ticks: NO_BURN_TICKS,
    }];

    let error = tick_selected_standard_furnace(state, &selected_recipes(), &malformed_fuels, selected_limits())
        .expect_err("malformed fuel should fail");

    expect_equal("malformed_fuel error", error, FurnaceError::MalformedFuelRow)
}

fn selected_recipes() -> [RecipeRow<'static>; 1] {
    [RecipeRow {
        input: RAW_IRON,
        output: IRON_INGOT,
        output_count: SELECTED_RECIPE_OUTPUT_COUNT,
        cook_ticks: STANDARD_FURNACE_COOK_TICKS,
    }]
}

fn selected_fuels() -> [FuelRow<'static>; 1] {
    [FuelRow {
        item: COAL,
        burn_ticks: COAL_BURN_TICKS,
    }]
}

fn selected_limits() -> FurnaceLimits {
    FurnaceLimits {
        max_stack_size: MAX_STACK_SIZE,
    }
}

fn base_state<'a>(
    input: Option<ItemStack<'a>>,
    fuel: Option<ItemStack<'a>>,
    output: Option<ItemStack<'a>>,
    cook_progress_ticks: u32,
    remaining_burn_ticks: u32,
) -> FurnaceState<'a> {
    FurnaceState {
        kind: FurnaceKind::Standard,
        input,
        fuel,
        output,
        cook_progress_ticks,
        remaining_burn_ticks,
        recipes_completed: INITIAL_RECIPE_COUNT,
    }
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

fn format_error(context: &'static str) -> impl FnOnce(FurnaceError) -> String {
    move |error| format!("{context}: {error:?}")
}
