use super::types::{
    FixtureBlockPos, FixtureGameMode, FixtureHand, FixtureSlotChange, FixtureStack,
};

pub fn should_open_fixture_container(
    game_mode: FixtureGameMode,
    hand: FixtureHand,
    position: FixtureBlockPos,
    target: FixtureBlockPos,
) -> bool {
    game_mode == FixtureGameMode::Survival && hand == FixtureHand::Main && position == target
}

pub fn slot_event_matches<'a>(
    window_id: u8,
    slot_id: i16,
    slot_changes: &[FixtureSlotChange<'a>],
    expected_window_id: u8,
    expected_slot_id: i16,
    expected_stack: FixtureStack<'a>,
) -> bool {
    window_id == expected_window_id
        && slot_id == expected_slot_id
        && slot_changes.iter().any(|change| {
            change.slot == expected_slot_id && stack_matches(change.stack, expected_stack)
        })
}

pub fn stack_matches(observed: FixtureStack<'_>, expected: FixtureStack<'_>) -> bool {
    observed.item_name == expected.item_name && observed.count == expected.count
}

pub fn collect_event_matches(
    window_id: u8,
    slot_id: i16,
    stack: FixtureStack<'_>,
    expected_window_id: u8,
    expected_slot_id: i16,
    expected_stack: FixtureStack<'_>,
) -> bool {
    window_id == expected_window_id
        && slot_id == expected_slot_id
        && stack_matches(stack, expected_stack)
}

#[cfg(test)]
mod tests {
    use super::*;

    const CHEST_POS: FixtureBlockPos = FixtureBlockPos { x: 8, y: 64, z: 0 };
    const OTHER_POS: FixtureBlockPos = FixtureBlockPos { x: 0, y: 64, z: 1 };
    const WINDOW: u8 = 1;
    const SLOT: i16 = 0;
    const OTHER_SLOT: i16 = 1;
    const DIRT: &str = "Dirt";
    const STONE: &str = "Stone";
    const STACK_COUNT: i8 = 1;
    const EMPTY_COUNT: i8 = 0;

    fn stack(item_name: &'static str, count: i8) -> FixtureStack<'static> {
        FixtureStack { item_name, count }
    }

    #[test]
    fn container_open_requires_survival_main_hand_target() {
        assert!(should_open_fixture_container(
            FixtureGameMode::Survival,
            FixtureHand::Main,
            CHEST_POS,
            CHEST_POS,
        ));
        assert!(!should_open_fixture_container(
            FixtureGameMode::Other,
            FixtureHand::Main,
            CHEST_POS,
            CHEST_POS,
        ));
        assert!(!should_open_fixture_container(
            FixtureGameMode::Survival,
            FixtureHand::Other,
            CHEST_POS,
            CHEST_POS,
        ));
        assert!(!should_open_fixture_container(
            FixtureGameMode::Survival,
            FixtureHand::Main,
            OTHER_POS,
            CHEST_POS,
        ));
    }

    #[test]
    fn slot_and_collect_events_require_exact_window_slot_and_stack() {
        let expected = stack(DIRT, STACK_COUNT);
        let change = FixtureSlotChange {
            slot: SLOT,
            stack: expected,
        };
        assert!(slot_event_matches(
            WINDOW,
            SLOT,
            &[change],
            WINDOW,
            SLOT,
            expected,
        ));
        assert!(!slot_event_matches(
            WINDOW + 1,
            SLOT,
            &[change],
            WINDOW,
            SLOT,
            expected,
        ));
        assert!(!slot_event_matches(
            WINDOW,
            OTHER_SLOT,
            &[change],
            WINDOW,
            SLOT,
            expected,
        ));
        assert!(!slot_event_matches(
            WINDOW,
            SLOT,
            &[FixtureSlotChange {
                slot: SLOT,
                stack: stack(STONE, STACK_COUNT),
            }],
            WINDOW,
            SLOT,
            expected,
        ));
        assert!(collect_event_matches(
            WINDOW, SLOT, expected, WINDOW, SLOT, expected,
        ));
        assert!(!collect_event_matches(
            WINDOW,
            SLOT,
            stack(DIRT, EMPTY_COUNT),
            WINDOW,
            SLOT,
            expected,
        ));
    }
}
