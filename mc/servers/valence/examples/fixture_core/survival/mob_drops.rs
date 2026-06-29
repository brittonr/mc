use super::types::{FixtureGameMode, FixtureInteraction};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MobDropPickupInput {
    pub pickup_logged: bool,
    pub ticks_since_drop: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MobDropPickupDecision {
    AlreadyComplete,
    Pending { ticks_since_drop: u8 },
    Ready { ticks_since_drop: u8 },
}

pub fn should_handle_mob_drop_attack(
    game_mode: FixtureGameMode,
    interaction: FixtureInteraction,
    target_entity_id: u32,
    fixture_entity_id: u32,
) -> bool {
    game_mode == FixtureGameMode::Survival
        && interaction == FixtureInteraction::Attack
        && target_entity_id == fixture_entity_id
}

pub fn plan_mob_drop_pickup(
    input: MobDropPickupInput,
    pickup_delay_ticks: u8,
) -> MobDropPickupDecision {
    if input.pickup_logged {
        return MobDropPickupDecision::AlreadyComplete;
    }
    let ticks_since_drop = input.ticks_since_drop.saturating_add(1);
    if ticks_since_drop < pickup_delay_ticks {
        return MobDropPickupDecision::Pending { ticks_since_drop };
    }
    MobDropPickupDecision::Ready { ticks_since_drop }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TARGET_ENTITY_ID: u32 = 11;
    const OTHER_ENTITY_ID: u32 = 12;
    const PICKUP_DELAY_TICKS: u8 = 2;

    #[test]
    fn mob_drop_attack_requires_survival_attack_on_fixture_entity() {
        assert!(should_handle_mob_drop_attack(
            FixtureGameMode::Survival,
            FixtureInteraction::Attack,
            TARGET_ENTITY_ID,
            TARGET_ENTITY_ID,
        ));
        assert!(!should_handle_mob_drop_attack(
            FixtureGameMode::Other,
            FixtureInteraction::Attack,
            TARGET_ENTITY_ID,
            TARGET_ENTITY_ID,
        ));
        assert!(!should_handle_mob_drop_attack(
            FixtureGameMode::Survival,
            FixtureInteraction::Other,
            TARGET_ENTITY_ID,
            TARGET_ENTITY_ID,
        ));
        assert!(!should_handle_mob_drop_attack(
            FixtureGameMode::Survival,
            FixtureInteraction::Attack,
            OTHER_ENTITY_ID,
            TARGET_ENTITY_ID,
        ));
    }

    #[test]
    fn pickup_planner_advances_until_ready_and_stops_when_complete() {
        assert_eq!(
            plan_mob_drop_pickup(
                MobDropPickupInput {
                    pickup_logged: false,
                    ticks_since_drop: 0,
                },
                PICKUP_DELAY_TICKS,
            ),
            MobDropPickupDecision::Pending {
                ticks_since_drop: 1,
            },
        );
        assert_eq!(
            plan_mob_drop_pickup(
                MobDropPickupInput {
                    pickup_logged: false,
                    ticks_since_drop: PICKUP_DELAY_TICKS - 1,
                },
                PICKUP_DELAY_TICKS,
            ),
            MobDropPickupDecision::Ready {
                ticks_since_drop: PICKUP_DELAY_TICKS,
            },
        );
        assert_eq!(
            plan_mob_drop_pickup(
                MobDropPickupInput {
                    pickup_logged: true,
                    ticks_since_drop: PICKUP_DELAY_TICKS,
                },
                PICKUP_DELAY_TICKS,
            ),
            MobDropPickupDecision::AlreadyComplete,
        );
    }
}
