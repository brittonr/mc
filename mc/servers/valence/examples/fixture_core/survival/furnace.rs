use super::containers;
use super::types::FixtureStack;

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
    count == 0
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
    const ITEM_COUNT: i8 = 1;
    const EMPTY_COUNT: i8 = 0;

    fn stack(item_name: &'static str, count: i8) -> FixtureStack<'static> {
        FixtureStack { item_name, count }
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
}
