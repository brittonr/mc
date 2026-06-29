use super::types::{
    FixtureBlockPos, FixtureDiggingState, FixtureDirection, FixtureGameMode, FixtureHand,
};

pub fn should_break_survival_block(
    game_mode: FixtureGameMode,
    digging_state: FixtureDiggingState,
    position: FixtureBlockPos,
    target: FixtureBlockPos,
) -> bool {
    game_mode == FixtureGameMode::Survival
        && digging_state == FixtureDiggingState::Stop
        && position == target
}

pub fn should_place_survival_block(
    game_mode: FixtureGameMode,
    hand: FixtureHand,
    position: FixtureBlockPos,
    face: FixtureDirection,
    target: FixtureBlockPos,
) -> bool {
    game_mode == FixtureGameMode::Survival
        && hand == FixtureHand::Main
        && position == target
        && face == FixtureDirection::Up
}

#[cfg(test)]
mod tests {
    use super::*;

    const TARGET_POS: FixtureBlockPos = FixtureBlockPos { x: 0, y: 64, z: 1 };
    const OTHER_POS: FixtureBlockPos = FixtureBlockPos { x: 0, y: 65, z: 1 };

    #[test]
    fn break_accepts_survival_stop_on_target_and_rejects_other_inputs() {
        assert!(should_break_survival_block(
            FixtureGameMode::Survival,
            FixtureDiggingState::Stop,
            TARGET_POS,
            TARGET_POS,
        ));
        assert!(!should_break_survival_block(
            FixtureGameMode::Other,
            FixtureDiggingState::Stop,
            TARGET_POS,
            TARGET_POS,
        ));
        assert!(!should_break_survival_block(
            FixtureGameMode::Survival,
            FixtureDiggingState::Other,
            TARGET_POS,
            TARGET_POS,
        ));
        assert!(!should_break_survival_block(
            FixtureGameMode::Survival,
            FixtureDiggingState::Stop,
            OTHER_POS,
            TARGET_POS,
        ));
    }

    #[test]
    fn place_accepts_main_hand_up_on_target_and_rejects_other_inputs() {
        assert!(should_place_survival_block(
            FixtureGameMode::Survival,
            FixtureHand::Main,
            TARGET_POS,
            FixtureDirection::Up,
            TARGET_POS,
        ));
        assert!(!should_place_survival_block(
            FixtureGameMode::Survival,
            FixtureHand::Other,
            TARGET_POS,
            FixtureDirection::Up,
            TARGET_POS,
        ));
        assert!(!should_place_survival_block(
            FixtureGameMode::Survival,
            FixtureHand::Main,
            OTHER_POS,
            FixtureDirection::Up,
            TARGET_POS,
        ));
        assert!(!should_place_survival_block(
            FixtureGameMode::Survival,
            FixtureHand::Main,
            TARGET_POS,
            FixtureDirection::Other,
            TARGET_POS,
        ));
    }
}
