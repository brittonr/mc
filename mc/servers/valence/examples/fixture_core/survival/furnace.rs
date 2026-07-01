use super::containers;
use super::types::FixtureStack;

pub const SELECTED_STANDARD_FURNACE_COOK_TICKS: u32 = 200;
pub const SELECTED_COAL_BURN_TICKS: u32 = 1_600;

pub const SELECTED_MAX_STACK_SIZE: i8 = 64;
pub const SELECTED_RECIPE_OUTPUT_COUNT: i8 = 1;
pub const FURNACE_TICK_INCREMENT: u32 = 1;
pub const EMPTY_ITEM_COUNT: i8 = 0;
pub const INITIAL_COOK_PROGRESS_TICKS: u32 = 0;
pub const INITIAL_RECIPE_COUNT: u32 = 0;
pub const NO_BURN_TICKS: u32 = 0;

pub const SELECTED_RAW_IRON_ITEM: &str = "minecraft:raw_iron";
pub const SELECTED_COAL_ITEM: &str = "minecraft:coal";
pub const SELECTED_IRON_INGOT_ITEM: &str = "minecraft:iron_ingot";
pub const UNSUPPORTED_FURNACE_ITEM: &str = "minecraft:unsupported_fixture_item";

const ONE_ITEM_COUNT: i8 = 1;
const ONE_COMPLETED_RECIPE: u32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FurnaceKind {
    Standard,
    Smoker,
    BlastFurnace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FurnaceStack<'a> {
    pub item: &'a str,
    pub count: i8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FurnaceRecipeRow<'a> {
    pub input: &'a str,
    pub output: &'a str,
    pub output_count: i8,
    pub cook_ticks: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FurnaceFuelRow<'a> {
    pub item: &'a str,
    pub burn_ticks: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FurnaceLimits {
    pub max_stack_size: i8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FurnaceState<'a> {
    pub kind: FurnaceKind,
    pub input: Option<FurnaceStack<'a>>,
    pub fuel: Option<FurnaceStack<'a>>,
    pub output: Option<FurnaceStack<'a>>,
    pub cook_progress_ticks: u32,
    pub remaining_burn_ticks: u32,
    pub recipes_completed: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FurnaceTick<'a> {
    pub state: FurnaceState<'a>,
    pub transition: FurnaceTransition,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FurnaceTransition {
    StartedFuel,
    AdvancedCooking,
    ProducedOutput,
    PausedNoFuel,
    PausedNoRecipe,
    PausedOutputBlocked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FurnaceError {
    UnsupportedFurnaceKind,
    MalformedRecipeRow,
    MalformedFuelRow,
}

pub fn selected_recipe_rows() -> [FurnaceRecipeRow<'static>; 1] {
    [FurnaceRecipeRow {
        input: SELECTED_RAW_IRON_ITEM,
        output: SELECTED_IRON_INGOT_ITEM,
        output_count: SELECTED_RECIPE_OUTPUT_COUNT,
        cook_ticks: SELECTED_STANDARD_FURNACE_COOK_TICKS,
    }]
}

pub fn selected_fuel_rows() -> [FurnaceFuelRow<'static>; 1] {
    [FurnaceFuelRow {
        item: SELECTED_COAL_ITEM,
        burn_ticks: SELECTED_COAL_BURN_TICKS,
    }]
}

pub const fn selected_limits() -> FurnaceLimits {
    FurnaceLimits {
        max_stack_size: SELECTED_MAX_STACK_SIZE,
    }
}

pub fn tick_selected_standard_furnace<'a>(
    state: FurnaceState<'a>,
    recipes: &[FurnaceRecipeRow<'a>],
    fuels: &[FurnaceFuelRow<'a>],
    limits: FurnaceLimits,
) -> Result<FurnaceTick<'a>, FurnaceError> {
    if state.kind != FurnaceKind::Standard {
        return Err(FurnaceError::UnsupportedFurnaceKind);
    }

    let Some(input) = state.input else {
        return Ok(paused(state, FurnaceTransition::PausedNoRecipe));
    };

    if input.count == EMPTY_ITEM_COUNT {
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

    next.remaining_burn_ticks = next
        .remaining_burn_ticks
        .saturating_sub(FURNACE_TICK_INCREMENT);
    next.cook_progress_ticks = next
        .cook_progress_ticks
        .saturating_add(FURNACE_TICK_INCREMENT);

    if next.cook_progress_ticks >= recipe.cook_ticks {
        next.input = decrement_stack(input);
        next.output = add_recipe_output(next.output, recipe);
        next.cook_progress_ticks = INITIAL_COOK_PROGRESS_TICKS;
        next.recipes_completed = next.recipes_completed.saturating_add(ONE_COMPLETED_RECIPE);
        transition = FurnaceTransition::ProducedOutput;
    }

    Ok(FurnaceTick {
        state: next,
        transition,
    })
}

pub fn furnace_stack(item: &str, count: i8) -> FurnaceStack<'_> {
    FurnaceStack { item, count }
}

pub fn initial_furnace_state<'a>(
    input: Option<FurnaceStack<'a>>,
    fuel: Option<FurnaceStack<'a>>,
    output: Option<FurnaceStack<'a>>,
) -> FurnaceState<'a> {
    FurnaceState {
        kind: FurnaceKind::Standard,
        input,
        fuel,
        output,
        cook_progress_ticks: INITIAL_COOK_PROGRESS_TICKS,
        remaining_burn_ticks: NO_BURN_TICKS,
        recipes_completed: INITIAL_RECIPE_COUNT,
    }
}

pub fn slot_event_matches(
    window_id: u8,
    slot_id: i16,
    expected_window_id: u8,
    expected_slot_id: i16,
) -> bool {
    window_id == expected_window_id && slot_id == expected_slot_id
}

pub fn collect_output_matches(
    window_id: u8,
    slot_id: i16,
    carried_stack: FixtureStack<'_>,
    expected_window_id: u8,
    expected_output_slot_id: i16,
    expected_output_stack: FixtureStack<'_>,
) -> bool {
    containers::collect_event_matches(
        window_id,
        slot_id,
        carried_stack,
        expected_window_id,
        expected_output_slot_id,
        expected_output_stack,
    )
}

pub fn should_emit_furnace_breadth_rejection(
    collect_logged: bool,
    invalid_rejection_logged: bool,
) -> bool {
    collect_logged && !invalid_rejection_logged
}

pub fn should_reject_furnace_invalid_fuel(
    breadth_enabled: bool,
    collect_logged: bool,
    window_id: u8,
    slot_id: i16,
    expected_window_id: u8,
    expected_fuel_slot_id: i16,
) -> bool {
    breadth_enabled
        && collect_logged
        && window_id == expected_window_id
        && slot_id == expected_fuel_slot_id
}

pub fn item_count_is_empty(count: i8) -> bool {
    count == EMPTY_ITEM_COUNT
}

fn paused<'a>(state: FurnaceState<'a>, transition: FurnaceTransition) -> FurnaceTick<'a> {
    FurnaceTick { state, transition }
}

fn find_recipe<'a>(
    input_item: &str,
    recipes: &[FurnaceRecipeRow<'a>],
) -> Option<FurnaceRecipeRow<'a>> {
    recipes
        .iter()
        .copied()
        .find(|recipe| recipe.input == input_item)
}

fn validate_recipe_row(
    recipe: FurnaceRecipeRow<'_>,
    limits: FurnaceLimits,
) -> Result<(), FurnaceError> {
    let has_valid_items = !recipe.input.is_empty() && !recipe.output.is_empty();
    let has_valid_output_count =
        recipe.output_count > EMPTY_ITEM_COUNT && recipe.output_count <= limits.max_stack_size;
    let has_valid_cook_ticks = recipe.cook_ticks > NO_BURN_TICKS;

    if has_valid_items && has_valid_output_count && has_valid_cook_ticks {
        Ok(())
    } else {
        Err(FurnaceError::MalformedRecipeRow)
    }
}

fn validate_fuel_row(fuel: FurnaceFuelRow<'_>) -> Result<(), FurnaceError> {
    let has_valid_item = !fuel.item.is_empty();
    let has_valid_burn_ticks = fuel.burn_ticks > NO_BURN_TICKS;

    if has_valid_item && has_valid_burn_ticks {
        Ok(())
    } else {
        Err(FurnaceError::MalformedFuelRow)
    }
}

fn can_accept_output(
    output: Option<FurnaceStack<'_>>,
    recipe: FurnaceRecipeRow<'_>,
    limits: FurnaceLimits,
) -> bool {
    match output {
        Some(stack) if stack.item != recipe.output => false,
        Some(stack) => stack.count.saturating_add(recipe.output_count) <= limits.max_stack_size,
        None => recipe.output_count <= limits.max_stack_size,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ConsumedFuel<'a> {
    next_stack: Option<FurnaceStack<'a>>,
    burn_ticks: u32,
}

fn consume_one_fuel<'a>(
    fuel_stack: Option<FurnaceStack<'a>>,
    fuels: &[FurnaceFuelRow<'a>],
) -> Result<Option<ConsumedFuel<'a>>, FurnaceError> {
    let Some(stack) = fuel_stack else {
        return Ok(None);
    };

    if stack.count == EMPTY_ITEM_COUNT {
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

fn decrement_stack<'a>(stack: FurnaceStack<'a>) -> Option<FurnaceStack<'a>> {
    if stack.count <= ONE_ITEM_COUNT {
        None
    } else {
        Some(FurnaceStack {
            item: stack.item,
            count: stack.count - ONE_ITEM_COUNT,
        })
    }
}

fn add_recipe_output<'a>(
    output: Option<FurnaceStack<'a>>,
    recipe: FurnaceRecipeRow<'a>,
) -> Option<FurnaceStack<'a>> {
    match output {
        Some(stack) => Some(FurnaceStack {
            item: stack.item,
            count: stack.count + recipe.output_count,
        }),
        None => Some(FurnaceStack {
            item: recipe.output,
            count: recipe.output_count,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const WINDOW: u8 = 1;
    const INPUT_SLOT: i16 = 0;
    const FUEL_SLOT: i16 = 1;
    const OUTPUT_SLOT: i16 = 2;
    const IRON_INGOT: &str = "IronIngot";
    const RAW_IRON: &str = "RawIron";
    const GOLD_INGOT: &str = "minecraft:gold_ingot";
    const COBBLESTONE: &str = "minecraft:cobblestone";
    const ITEM_COUNT: i8 = 1;
    const EMPTY_COUNT: i8 = 0;
    const TWO_ITEMS: i8 = 2;
    const COMPATIBLE_OUTPUT_START_COUNT: i8 = 7;
    const EXPECTED_MERGED_OUTPUT_COUNT: i8 = 8;
    const EXPECTED_COAL_AFTER_START: u32 = SELECTED_COAL_BURN_TICKS - FURNACE_TICK_INCREMENT;
    const EXPECTED_PROGRESS_AFTER_TICK: u32 = INITIAL_COOK_PROGRESS_TICKS + FURNACE_TICK_INCREMENT;
    const NEAR_COMPLETE_PROGRESS: u32 =
        SELECTED_STANDARD_FURNACE_COOK_TICKS - FURNACE_TICK_INCREMENT;

    fn stack(item_name: &'static str, count: i8) -> FixtureStack<'static> {
        FixtureStack { item_name, count }
    }

    fn furnace_state<'a>(
        input: Option<FurnaceStack<'a>>,
        fuel: Option<FurnaceStack<'a>>,
        output: Option<FurnaceStack<'a>>,
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

    #[test]
    fn furnace_slots_match_exact_window_and_slot() {
        assert!(slot_event_matches(WINDOW, INPUT_SLOT, WINDOW, INPUT_SLOT));
        assert!(slot_event_matches(WINDOW, FUEL_SLOT, WINDOW, FUEL_SLOT));
        assert!(!slot_event_matches(
            WINDOW + 1,
            INPUT_SLOT,
            WINDOW,
            INPUT_SLOT
        ));
        assert!(!slot_event_matches(WINDOW, FUEL_SLOT, WINDOW, INPUT_SLOT));
    }

    #[test]
    fn furnace_collect_requires_output_slot_and_stack() {
        assert!(collect_output_matches(
            WINDOW,
            OUTPUT_SLOT,
            stack(IRON_INGOT, ITEM_COUNT),
            WINDOW,
            OUTPUT_SLOT,
            stack(IRON_INGOT, ITEM_COUNT),
        ));
        assert!(!collect_output_matches(
            WINDOW,
            INPUT_SLOT,
            stack(IRON_INGOT, ITEM_COUNT),
            WINDOW,
            OUTPUT_SLOT,
            stack(IRON_INGOT, ITEM_COUNT),
        ));
        assert!(!collect_output_matches(
            WINDOW,
            OUTPUT_SLOT,
            stack(RAW_IRON, ITEM_COUNT),
            WINDOW,
            OUTPUT_SLOT,
            stack(IRON_INGOT, ITEM_COUNT),
        ));
    }

    #[test]
    fn furnace_breadth_invalid_fuel_fails_closed() {
        assert!(should_emit_furnace_breadth_rejection(true, false));
        assert!(!should_emit_furnace_breadth_rejection(false, false));
        assert!(should_reject_furnace_invalid_fuel(
            true, true, WINDOW, FUEL_SLOT, WINDOW, FUEL_SLOT,
        ));
        assert!(!should_reject_furnace_invalid_fuel(
            false, true, WINDOW, FUEL_SLOT, WINDOW, FUEL_SLOT,
        ));
        assert!(!should_reject_furnace_invalid_fuel(
            true, false, WINDOW, FUEL_SLOT, WINDOW, FUEL_SLOT,
        ));
        assert!(!should_reject_furnace_invalid_fuel(
            true, true, WINDOW, INPUT_SLOT, WINDOW, FUEL_SLOT,
        ));
        assert!(item_count_is_empty(EMPTY_COUNT));
        assert!(!item_count_is_empty(ITEM_COUNT));
    }

    #[test]
    fn selected_row_core_starts_fuel_and_advances() {
        let state = initial_furnace_state(
            Some(furnace_stack(SELECTED_RAW_IRON_ITEM, ITEM_COUNT)),
            Some(furnace_stack(SELECTED_COAL_ITEM, ITEM_COUNT)),
            None,
        );

        let tick = tick_selected_standard_furnace(
            state,
            &selected_recipe_rows(),
            &selected_fuel_rows(),
            selected_limits(),
        )
        .unwrap();

        assert_eq!(tick.transition, FurnaceTransition::StartedFuel);
        assert_eq!(tick.state.fuel, None);
        assert_eq!(tick.state.remaining_burn_ticks, EXPECTED_COAL_AFTER_START);
        assert_eq!(tick.state.cook_progress_ticks, EXPECTED_PROGRESS_AFTER_TICK);
    }

    #[test]
    fn selected_row_core_advances_active_burn_and_merges_output() {
        let active_state = furnace_state(
            Some(furnace_stack(SELECTED_RAW_IRON_ITEM, ITEM_COUNT)),
            Some(furnace_stack(SELECTED_COAL_ITEM, ITEM_COUNT)),
            None,
            INITIAL_COOK_PROGRESS_TICKS,
            SELECTED_COAL_BURN_TICKS,
        );
        let active_tick = tick_selected_standard_furnace(
            active_state,
            &selected_recipe_rows(),
            &selected_fuel_rows(),
            selected_limits(),
        )
        .unwrap();

        assert_eq!(active_tick.transition, FurnaceTransition::AdvancedCooking);
        assert_eq!(active_tick.state.fuel, active_state.fuel);
        assert_eq!(
            active_tick.state.cook_progress_ticks,
            EXPECTED_PROGRESS_AFTER_TICK
        );

        let merge_state = furnace_state(
            Some(furnace_stack(SELECTED_RAW_IRON_ITEM, ITEM_COUNT)),
            None,
            Some(furnace_stack(
                SELECTED_IRON_INGOT_ITEM,
                COMPATIBLE_OUTPUT_START_COUNT,
            )),
            NEAR_COMPLETE_PROGRESS,
            FURNACE_TICK_INCREMENT,
        );
        let merge_tick = tick_selected_standard_furnace(
            merge_state,
            &selected_recipe_rows(),
            &selected_fuel_rows(),
            selected_limits(),
        )
        .unwrap();

        assert_eq!(merge_tick.transition, FurnaceTransition::ProducedOutput);
        assert_eq!(
            merge_tick.state.output,
            Some(furnace_stack(
                SELECTED_IRON_INGOT_ITEM,
                EXPECTED_MERGED_OUTPUT_COUNT,
            ))
        );
    }

    #[test]
    fn selected_row_core_rejects_invalid_and_blocked_inputs() {
        let missing_recipe_state = initial_furnace_state(
            Some(furnace_stack(COBBLESTONE, ITEM_COUNT)),
            Some(furnace_stack(SELECTED_COAL_ITEM, ITEM_COUNT)),
            None,
        );
        let missing_recipe_tick = tick_selected_standard_furnace(
            missing_recipe_state,
            &selected_recipe_rows(),
            &selected_fuel_rows(),
            selected_limits(),
        )
        .unwrap();

        assert_eq!(
            missing_recipe_tick.transition,
            FurnaceTransition::PausedNoRecipe
        );
        assert_eq!(missing_recipe_tick.state, missing_recipe_state);

        let missing_fuel_state = initial_furnace_state(
            Some(furnace_stack(SELECTED_RAW_IRON_ITEM, ITEM_COUNT)),
            None,
            None,
        );
        let missing_fuel_tick = tick_selected_standard_furnace(
            missing_fuel_state,
            &selected_recipe_rows(),
            &selected_fuel_rows(),
            selected_limits(),
        )
        .unwrap();

        assert_eq!(
            missing_fuel_tick.transition,
            FurnaceTransition::PausedNoFuel
        );
        assert_eq!(missing_fuel_tick.state, missing_fuel_state);

        let blocked_state = initial_furnace_state(
            Some(furnace_stack(SELECTED_RAW_IRON_ITEM, ITEM_COUNT)),
            Some(furnace_stack(SELECTED_COAL_ITEM, ITEM_COUNT)),
            Some(furnace_stack(GOLD_INGOT, ITEM_COUNT)),
        );
        let blocked_tick = tick_selected_standard_furnace(
            blocked_state,
            &selected_recipe_rows(),
            &selected_fuel_rows(),
            selected_limits(),
        )
        .unwrap();

        assert_eq!(
            blocked_tick.transition,
            FurnaceTransition::PausedOutputBlocked
        );
        assert_eq!(blocked_tick.state, blocked_state);
    }

    #[test]
    fn selected_row_core_fails_malformed_or_unsupported_rows() {
        let state = initial_furnace_state(
            Some(furnace_stack(SELECTED_RAW_IRON_ITEM, ITEM_COUNT)),
            Some(furnace_stack(SELECTED_COAL_ITEM, ITEM_COUNT)),
            None,
        );
        let malformed_recipes = [FurnaceRecipeRow {
            input: SELECTED_RAW_IRON_ITEM,
            output: "",
            output_count: SELECTED_RECIPE_OUTPUT_COUNT,
            cook_ticks: SELECTED_STANDARD_FURNACE_COOK_TICKS,
        }];
        let malformed_fuels = [FurnaceFuelRow {
            item: SELECTED_COAL_ITEM,
            burn_ticks: NO_BURN_TICKS,
        }];
        let mut smoker_state = state;
        smoker_state.kind = FurnaceKind::Smoker;
        let mut blast_state = state;
        blast_state.kind = FurnaceKind::BlastFurnace;

        assert_eq!(
            tick_selected_standard_furnace(
                state,
                &malformed_recipes,
                &selected_fuel_rows(),
                selected_limits(),
            ),
            Err(FurnaceError::MalformedRecipeRow)
        );
        assert_eq!(
            tick_selected_standard_furnace(
                state,
                &selected_recipe_rows(),
                &malformed_fuels,
                selected_limits(),
            ),
            Err(FurnaceError::MalformedFuelRow)
        );
        assert_eq!(
            tick_selected_standard_furnace(
                smoker_state,
                &selected_recipe_rows(),
                &selected_fuel_rows(),
                selected_limits(),
            ),
            Err(FurnaceError::UnsupportedFurnaceKind)
        );
        assert_eq!(
            tick_selected_standard_furnace(
                blast_state,
                &selected_recipe_rows(),
                &selected_fuel_rows(),
                selected_limits(),
            ),
            Err(FurnaceError::UnsupportedFurnaceKind)
        );
    }

    #[test]
    fn selected_row_core_produces_exact_output() {
        let state = furnace_state(
            Some(furnace_stack(SELECTED_RAW_IRON_ITEM, TWO_ITEMS)),
            None,
            None,
            NEAR_COMPLETE_PROGRESS,
            FURNACE_TICK_INCREMENT,
        );

        let tick = tick_selected_standard_furnace(
            state,
            &selected_recipe_rows(),
            &selected_fuel_rows(),
            selected_limits(),
        )
        .unwrap();

        assert_eq!(tick.transition, FurnaceTransition::ProducedOutput);
        assert_eq!(
            tick.state.input,
            Some(furnace_stack(SELECTED_RAW_IRON_ITEM, ITEM_COUNT))
        );
        assert_eq!(
            tick.state.output,
            Some(furnace_stack(SELECTED_IRON_INGOT_ITEM, ITEM_COUNT))
        );
        assert_eq!(tick.state.cook_progress_ticks, INITIAL_COOK_PROGRESS_TICKS);
    }
}
