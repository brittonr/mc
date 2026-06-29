#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArmorState {
    DiamondChestplate,
    Other,
    Empty,
}

pub fn reference_hit_for(
    enabled: bool,
    attacker: &str,
    victim: &str,
    expected_attacker: &str,
    expected_victim: &str,
) -> bool {
    enabled && attacker == expected_attacker && victim == expected_victim
}

pub fn combat_armor_mitigation_for(
    reference_probe_enabled: bool,
    armor_probe_enabled: bool,
    chest_item: ArmorState,
    base_damage: f32,
    compatibility_mitigation: f32,
    diamond_armor_points: f32,
    diamond_toughness: f32,
) -> f32 {
    if reference_probe_enabled && chest_item == ArmorState::DiamondChestplate {
        return vanilla_armor_mitigation_for(base_damage, diamond_armor_points, diamond_toughness);
    }
    if armor_probe_enabled && chest_item == ArmorState::DiamondChestplate {
        return compatibility_mitigation;
    }
    0.0
}

pub fn vanilla_armor_mitigation_for(base_damage: f32, armor_points: f32, toughness: f32) -> f32 {
    let toughness_reduction = armor_points
        - base_damage
            / (toughness / VANILLA_ARMOR_TOUGHNESS_QUARTER_DIVISOR + VANILLA_ARMOR_TOUGHNESS_BASE);
    let minimum_reduction = armor_points / VANILLA_ARMOR_MIN_REDUCTION_DIVISOR;
    let reduction_points = toughness_reduction
        .max(minimum_reduction)
        .min(VANILLA_ARMOR_MAX_REDUCTION_POINTS);
    base_damage * reduction_points / VANILLA_COMBAT_ARMOR_REDUCTION_DENOMINATOR
}

pub fn knockback_metric(knockback_velocity: [f32; KNOCKBACK_VECTOR_COMPONENT_COUNT]) -> f64 {
    f64::from(knockback_velocity[KNOCKBACK_X_COMPONENT])
        .hypot(f64::from(knockback_velocity[KNOCKBACK_Z_COMPONENT]))
        / VANILLA_COMBAT_REFERENCE_KNOCKBACK_SCALE
}

const KNOCKBACK_VECTOR_COMPONENT_COUNT: usize = 3;
const KNOCKBACK_X_COMPONENT: usize = 0;
const KNOCKBACK_Z_COMPONENT: usize = 2;
const VANILLA_COMBAT_REFERENCE_KNOCKBACK_SCALE: f64 = 20.0;
const VANILLA_COMBAT_ARMOR_REDUCTION_DENOMINATOR: f32 = 25.0;
const VANILLA_ARMOR_TOUGHNESS_QUARTER_DIVISOR: f32 = 4.0;
const VANILLA_ARMOR_TOUGHNESS_BASE: f32 = 2.0;
const VANILLA_ARMOR_MIN_REDUCTION_DIVISOR: f32 = 5.0;
const VANILLA_ARMOR_MAX_REDUCTION_POINTS: f32 = 20.0;
