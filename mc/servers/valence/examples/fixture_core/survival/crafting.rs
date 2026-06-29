use super::containers;
use super::types::FixtureStack;

pub fn input_event_matches(
    window_id: u8,
    slot_id: i16,
    expected_window_id: u8,
    expected_slot_id: i16,
) -> bool {
    window_id == expected_window_id && slot_id == expected_slot_id
}

pub fn collect_result_matches(
    window_id: u8,
    slot_id: i16,
    carried_stack: FixtureStack<'_>,
    expected_window_id: u8,
    expected_result_slot_id: i16,
    expected_result_stack: FixtureStack<'_>,
) -> bool {
    containers::collect_event_matches(
        window_id,
        slot_id,
        carried_stack,
        expected_window_id,
        expected_result_slot_id,
        expected_result_stack,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const WINDOW: u8 = 1;
    const INPUT_SLOT: i16 = 1;
    const RESULT_SLOT: i16 = 0;
    const STICK: &str = "minecraft:stick";
    const OAK_PLANKS: &str = "OakPlanks";
    const RESULT_COUNT: i8 = 4;
    const INPUT_COUNT: i8 = 1;

    fn stack(item_name: &'static str, count: i8) -> FixtureStack<'static> {
        FixtureStack { item_name, count }
    }

    #[test]
    fn crafting_input_matches_exact_window_and_slot() {
        assert!(input_event_matches(WINDOW, INPUT_SLOT, WINDOW, INPUT_SLOT));
        assert!(!input_event_matches(
            WINDOW + 1,
            INPUT_SLOT,
            WINDOW,
            INPUT_SLOT,
        ));
        assert!(!input_event_matches(
            WINDOW,
            RESULT_SLOT,
            WINDOW,
            INPUT_SLOT,
        ));
    }

    #[test]
    fn crafting_collect_requires_result_slot_and_result_stack() {
        assert!(collect_result_matches(
            WINDOW,
            RESULT_SLOT,
            stack(STICK, RESULT_COUNT),
            WINDOW,
            RESULT_SLOT,
            stack(STICK, RESULT_COUNT),
        ));
        assert!(!collect_result_matches(
            WINDOW + 1,
            RESULT_SLOT,
            stack(STICK, RESULT_COUNT),
            WINDOW,
            RESULT_SLOT,
            stack(STICK, RESULT_COUNT),
        ));
        assert!(!collect_result_matches(
            WINDOW,
            INPUT_SLOT,
            stack(STICK, RESULT_COUNT),
            WINDOW,
            RESULT_SLOT,
            stack(STICK, RESULT_COUNT),
        ));
        assert!(!collect_result_matches(
            WINDOW,
            RESULT_SLOT,
            stack(OAK_PLANKS, INPUT_COUNT),
            WINDOW,
            RESULT_SLOT,
            stack(STICK, RESULT_COUNT),
        ));
    }
}
