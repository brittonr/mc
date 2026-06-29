use super::containers;
use super::types::{FixtureHand, FixtureHungerProfile, FixtureStack};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HungerUseInput<'a> {
    pub hand: FixtureHand,
    pub sequence: i32,
    pub slot: u16,
    pub stack: FixtureStack<'a>,
    pub health_tenths: i32,
    pub food: i32,
    pub saturation_tenths: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HungerUseContract<'a> {
    pub expected_sequence: i32,
    pub expected_slot: u16,
    pub expected_stack: FixtureStack<'a>,
}

pub fn select_hunger_profile(
    food_enabled: bool,
    health_enabled: bool,
    food_profile: FixtureHungerProfile,
    health_profile: FixtureHungerProfile,
) -> Option<FixtureHungerProfile> {
    if health_enabled {
        return Some(health_profile);
    }
    if food_enabled {
        return Some(food_profile);
    }
    None
}

pub fn should_consume_hunger_food(
    profile: FixtureHungerProfile,
    input: HungerUseInput<'_>,
    contract: HungerUseContract<'_>,
) -> bool {
    input.hand == FixtureHand::Main
        && input.sequence == contract.expected_sequence
        && input.slot == contract.expected_slot
        && containers::stack_matches(input.stack, contract.expected_stack)
        && input.health_tenths == profile.pre_health_tenths
        && input.food == profile.pre_food
        && input.saturation_tenths == profile.pre_saturation_tenths
}

#[cfg(test)]
mod tests {
    use super::*;

    const INVENTORY_SLOT: u16 = 36;
    const USE_SEQUENCE: i32 = 810;
    const BREAD: &str = "Bread";
    const IRON_INGOT: &str = "IronIngot";
    const STACK_COUNT: i8 = 1;
    const FOOD_EVENT: &str = "survival_hunger_food";
    const HEALTH_EVENT: &str = "survival_hunger_health";
    const PRE_HEALTH_TENTHS: i32 = 200;
    const HURT_HEALTH_TENTHS: i32 = 180;
    const POST_HEALTH_TENTHS: i32 = 200;
    const PRE_FOOD: i32 = 15;
    const POST_FOOD: i32 = 20;
    const PRE_SATURATION_TENTHS: i32 = 0;
    const POST_SATURATION_TENTHS: i32 = 60;

    const FOOD_PROFILE: FixtureHungerProfile = FixtureHungerProfile {
        event_prefix: FOOD_EVENT,
        pre_health_tenths: PRE_HEALTH_TENTHS,
        pre_food: PRE_FOOD,
        pre_saturation_tenths: PRE_SATURATION_TENTHS,
        post_health_tenths: POST_HEALTH_TENTHS,
        post_food: POST_FOOD,
        post_saturation_tenths: POST_SATURATION_TENTHS,
    };
    const HEALTH_PROFILE: FixtureHungerProfile = FixtureHungerProfile {
        event_prefix: HEALTH_EVENT,
        pre_health_tenths: HURT_HEALTH_TENTHS,
        pre_food: PRE_FOOD,
        pre_saturation_tenths: PRE_SATURATION_TENTHS,
        post_health_tenths: POST_HEALTH_TENTHS,
        post_food: PRE_FOOD,
        post_saturation_tenths: PRE_SATURATION_TENTHS,
    };

    fn stack(item_name: &'static str, count: i8) -> FixtureStack<'static> {
        FixtureStack { item_name, count }
    }

    #[test]
    fn hunger_profile_prefers_health_then_food_and_rejects_disabled() {
        assert_eq!(
            select_hunger_profile(true, false, FOOD_PROFILE, HEALTH_PROFILE),
            Some(FOOD_PROFILE),
        );
        assert_eq!(
            select_hunger_profile(false, true, FOOD_PROFILE, HEALTH_PROFILE),
            Some(HEALTH_PROFILE),
        );
        assert_eq!(
            select_hunger_profile(false, false, FOOD_PROFILE, HEALTH_PROFILE),
            None,
        );
    }

    #[test]
    fn hunger_consumption_requires_main_hand_sequence_slot_stack_and_pre_state() {
        let contract = HungerUseContract {
            expected_sequence: USE_SEQUENCE,
            expected_slot: INVENTORY_SLOT,
            expected_stack: stack(BREAD, STACK_COUNT),
        };
        assert!(should_consume_hunger_food(
            FOOD_PROFILE,
            HungerUseInput {
                hand: FixtureHand::Main,
                sequence: USE_SEQUENCE,
                slot: INVENTORY_SLOT,
                stack: stack(BREAD, STACK_COUNT),
                health_tenths: PRE_HEALTH_TENTHS,
                food: PRE_FOOD,
                saturation_tenths: PRE_SATURATION_TENTHS,
            },
            contract,
        ));
        assert!(!should_consume_hunger_food(
            FOOD_PROFILE,
            HungerUseInput {
                hand: FixtureHand::Other,
                sequence: USE_SEQUENCE,
                slot: INVENTORY_SLOT,
                stack: stack(BREAD, STACK_COUNT),
                health_tenths: PRE_HEALTH_TENTHS,
                food: PRE_FOOD,
                saturation_tenths: PRE_SATURATION_TENTHS,
            },
            contract,
        ));
        assert!(!should_consume_hunger_food(
            FOOD_PROFILE,
            HungerUseInput {
                hand: FixtureHand::Main,
                sequence: USE_SEQUENCE,
                slot: INVENTORY_SLOT,
                stack: stack(IRON_INGOT, STACK_COUNT),
                health_tenths: PRE_HEALTH_TENTHS,
                food: PRE_FOOD,
                saturation_tenths: PRE_SATURATION_TENTHS,
            },
            contract,
        ));
        assert!(!should_consume_hunger_food(
            FOOD_PROFILE,
            HungerUseInput {
                hand: FixtureHand::Main,
                sequence: USE_SEQUENCE + 1,
                slot: INVENTORY_SLOT,
                stack: stack(BREAD, STACK_COUNT),
                health_tenths: PRE_HEALTH_TENTHS,
                food: PRE_FOOD,
                saturation_tenths: PRE_SATURATION_TENTHS,
            },
            contract,
        ));
    }
}
