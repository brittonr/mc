#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FixtureGameMode {
    Survival,
    Other,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FixtureHand {
    Main,
    Other,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FixtureDirection {
    Up,
    Other,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FixtureDiggingState {
    Stop,
    Other,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FixtureInteraction {
    Attack,
    Other,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FixtureBlockPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FixtureStack<'a> {
    pub item_name: &'a str,
    pub count: i8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FixtureSlotChange<'a> {
    pub slot: i16,
    pub stack: FixtureStack<'a>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FixtureHungerProfile {
    pub event_prefix: &'static str,
    pub pre_health_tenths: i32,
    pub pre_food: i32,
    pub pre_saturation_tenths: i32,
    pub post_health_tenths: i32,
    pub post_food: i32,
    pub post_saturation_tenths: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MarkerDecision {
    WriteMarker,
    MissingMarker,
    ObservePersisted,
}

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

pub fn should_consume_hunger_food(
    profile: FixtureHungerProfile,
    input: HungerUseInput<'_>,
    contract: HungerUseContract<'_>,
) -> bool {
    input.hand == FixtureHand::Main
        && input.sequence == contract.expected_sequence
        && input.slot == contract.expected_slot
        && stack_matches(input.stack, contract.expected_stack)
        && input.health_tenths == profile.pre_health_tenths
        && input.food == profile.pre_food
        && input.saturation_tenths == profile.pre_saturation_tenths
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

pub fn evaluate_marker_decision(post_restart: bool, marker_present: bool) -> MarkerDecision {
    if !post_restart {
        return MarkerDecision::WriteMarker;
    }
    if marker_present {
        MarkerDecision::ObservePersisted
    } else {
        MarkerDecision::MissingMarker
    }
}

pub fn should_place_block_entity_sign(post_restart: bool, marker_present: bool) -> bool {
    matches!(
        evaluate_marker_decision(post_restart, marker_present),
        MarkerDecision::WriteMarker | MarkerDecision::ObservePersisted
    )
}

pub fn validate_block_entity_payload(
    lines: &[&str],
    expected_payload: &str,
    separator: &str,
    expected_line_count: usize,
) -> Result<(), &'static str> {
    if lines.len() != expected_line_count {
        return Err("line_count");
    }
    let payload = lines.join(separator);
    if payload != expected_payload {
        return Err("payload");
    }
    Ok(())
}

pub fn redstone_power_transition(powered: bool) -> bool {
    !powered
}

pub fn normalize_environment_id(
    raw: &str,
    known: &[&'static str],
    unknown: &'static str,
) -> &'static str {
    for known_id in known {
        if raw == *known_id {
            return *known_id;
        }
    }
    unknown
}

pub fn derive_environment_id(
    spawn_environment: &str,
    environment_identifier: &str,
    known: &[&'static str],
    unknown: &'static str,
) -> &'static str {
    let environment = normalize_environment_id(environment_identifier, known, unknown);
    if environment != unknown {
        return environment;
    }
    normalize_environment_id(spawn_environment, known, unknown)
}

pub fn biome_dimension_state_milestone(
    username: &str,
    spawn_environment: &str,
    environment_identifier: &str,
    derived_environment: &str,
) -> String {
    format!(
        "MC-COMPAT-MILESTONE survival_biome_dimension_state username={} spawn_environment={} \
         environment_identifier={} derived_environment={}",
        username, spawn_environment, environment_identifier, derived_environment
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const TARGET_POS: FixtureBlockPos = FixtureBlockPos { x: 0, y: 64, z: 1 };
    const OTHER_POS: FixtureBlockPos = FixtureBlockPos { x: 0, y: 65, z: 1 };
    const CHEST_POS: FixtureBlockPos = FixtureBlockPos { x: 8, y: 64, z: 0 };
    const WINDOW: u8 = 1;
    const SLOT: i16 = 0;
    const OTHER_SLOT: i16 = 1;
    const INVENTORY_SLOT: u16 = 36;
    const USE_SEQUENCE: i32 = 810;
    const DIRT: &str = "Dirt";
    const STONE: &str = "Stone";
    const BREAD: &str = "Bread";
    const IRON_INGOT: &str = "IronIngot";
    const STACK_COUNT: i8 = 1;
    const EMPTY_COUNT: i8 = 0;
    const FOOD_EVENT: &str = "survival_hunger_food";
    const HEALTH_EVENT: &str = "survival_hunger_health";
    const PRE_HEALTH_TENTHS: i32 = 200;
    const HURT_HEALTH_TENTHS: i32 = 180;
    const POST_HEALTH_TENTHS: i32 = 200;
    const PRE_FOOD: i32 = 15;
    const POST_FOOD: i32 = 20;
    const PRE_SATURATION_TENTHS: i32 = 0;
    const POST_SATURATION_TENTHS: i32 = 60;
    const TARGET_ENTITY_ID: u32 = 11;
    const OTHER_ENTITY_ID: u32 = 12;
    const OVERWORLD: &str = "minecraft:overworld";
    const NETHER: &str = "minecraft:the_nether";
    const END: &str = "minecraft:the_end";
    const UNKNOWN: &str = "unknown";
    const SIGN_LINE_COUNT: usize = 4;
    const SIGN_PAYLOAD: &str = "MC|Compat|Sign|Persist";
    const SIGN_SEPARATOR: &str = "|";

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
    fn block_break_place_and_container_decisions_accept_only_survival_main_target() {
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
        assert!(should_open_fixture_container(
            FixtureGameMode::Survival,
            FixtureHand::Main,
            CHEST_POS,
            CHEST_POS,
        ));
        assert!(!should_open_fixture_container(
            FixtureGameMode::Survival,
            FixtureHand::Main,
            TARGET_POS,
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
            expected
        ));
        assert!(!slot_event_matches(
            WINDOW + 1,
            SLOT,
            &[change],
            WINDOW,
            SLOT,
            expected
        ));
        assert!(!slot_event_matches(
            WINDOW,
            OTHER_SLOT,
            &[change],
            WINDOW,
            SLOT,
            expected
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
            WINDOW, SLOT, expected, WINDOW, SLOT, expected
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

    #[test]
    fn furnace_and_hunger_decisions_accept_valid_flow_and_reject_invalid_flow() {
        assert!(should_emit_furnace_breadth_rejection(true, false));
        assert!(!should_emit_furnace_breadth_rejection(false, false));
        assert!(should_reject_furnace_invalid_fuel(
            true, true, WINDOW, OTHER_SLOT, WINDOW, OTHER_SLOT
        ));
        assert!(!should_reject_furnace_invalid_fuel(
            false, true, WINDOW, OTHER_SLOT, WINDOW, OTHER_SLOT
        ));
        assert_eq!(
            select_hunger_profile(true, false, FOOD_PROFILE, HEALTH_PROFILE),
            Some(FOOD_PROFILE)
        );
        assert_eq!(
            select_hunger_profile(false, true, FOOD_PROFILE, HEALTH_PROFILE),
            Some(HEALTH_PROFILE)
        );
        assert_eq!(
            select_hunger_profile(false, false, FOOD_PROFILE, HEALTH_PROFILE),
            None
        );
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
    }

    #[test]
    fn mob_redstone_persistence_and_block_entity_decisions_fail_closed() {
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
        assert!(redstone_power_transition(false));
        assert!(!redstone_power_transition(true));
        assert_eq!(
            evaluate_marker_decision(false, false),
            MarkerDecision::WriteMarker
        );
        assert_eq!(
            evaluate_marker_decision(true, true),
            MarkerDecision::ObservePersisted
        );
        assert_eq!(
            evaluate_marker_decision(true, false),
            MarkerDecision::MissingMarker
        );
        assert!(should_place_block_entity_sign(false, false));
        assert!(!should_place_block_entity_sign(true, false));
        assert_eq!(
            validate_block_entity_payload(
                &["MC", "Compat", "Sign", "Persist"],
                SIGN_PAYLOAD,
                SIGN_SEPARATOR,
                SIGN_LINE_COUNT
            ),
            Ok(())
        );
        assert_eq!(
            validate_block_entity_payload(
                &["MC", "Compat"],
                SIGN_PAYLOAD,
                SIGN_SEPARATOR,
                SIGN_LINE_COUNT
            ),
            Err("line_count")
        );
        assert_eq!(
            validate_block_entity_payload(
                &["MC", "Compat", "Sign", "Edit"],
                SIGN_PAYLOAD,
                SIGN_SEPARATOR,
                SIGN_LINE_COUNT
            ),
            Err("payload")
        );
    }

    #[test]
    fn biome_dimension_core_normalizes_known_ids_and_rejects_unknown_ids() {
        let known = [OVERWORLD, NETHER, END];
        assert_eq!(normalize_environment_id(NETHER, &known, UNKNOWN), NETHER);
        assert_eq!(
            normalize_environment_id("custom:unknown", &known, UNKNOWN),
            UNKNOWN
        );
        assert_eq!(
            derive_environment_id(NETHER, OVERWORLD, &known, UNKNOWN),
            OVERWORLD
        );
        assert_eq!(
            derive_environment_id(END, "custom:unknown", &known, UNKNOWN),
            END
        );
        assert_eq!(
            derive_environment_id("custom:dimension", "custom:world", &known, UNKNOWN),
            UNKNOWN
        );
        let milestone = biome_dimension_state_milestone("compatbot", NETHER, OVERWORLD, OVERWORLD);
        assert!(
            milestone.contains("survival_biome_dimension_state"),
            "{milestone}"
        );
        assert!(
            milestone.contains("derived_environment=minecraft:overworld"),
            "{milestone}"
        );
    }
}
