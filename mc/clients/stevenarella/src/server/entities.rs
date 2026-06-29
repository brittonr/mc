use cgmath::Vector3;

const ENTITY_VELOCITY_PROTOCOL_SCALE: f64 = 8000.0;

pub(crate) fn entity_velocity_blocks_per_tick(
    velocity_x: i16,
    velocity_y: i16,
    velocity_z: i16,
) -> Vector3<f64> {
    Vector3::new(
        f64::from(velocity_x) / ENTITY_VELOCITY_PROTOCOL_SCALE,
        f64::from(velocity_y) / ENTITY_VELOCITY_PROTOCOL_SCALE,
        f64::from(velocity_z) / ENTITY_VELOCITY_PROTOCOL_SCALE,
    )
}

pub(crate) fn has_observable_velocity(velocity_x: i16, velocity_y: i16, velocity_z: i16) -> bool {
    velocity_x != 0 || velocity_y != 0 || velocity_z != 0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_AXIS_VELOCITY: i16 = 8000;
    const NEGATIVE_AXIS_VELOCITY: i16 = -4000;
    const EXPECTED_POSITIVE_BLOCKS_PER_TICK: f64 = 1.0;
    const EXPECTED_NEGATIVE_BLOCKS_PER_TICK: f64 = -0.5;

    #[test]
    fn converts_protocol_velocity_to_blocks_per_tick() {
        let velocity =
            entity_velocity_blocks_per_tick(TEST_AXIS_VELOCITY, NEGATIVE_AXIS_VELOCITY, 0);

        assert_eq!(velocity.x, EXPECTED_POSITIVE_BLOCKS_PER_TICK);
        assert_eq!(velocity.y, EXPECTED_NEGATIVE_BLOCKS_PER_TICK);
        assert_eq!(velocity.z, 0.0);
    }

    #[test]
    fn zero_velocity_is_not_observable_for_combat_probe_logs() {
        assert!(!has_observable_velocity(0, 0, 0));
        assert!(has_observable_velocity(TEST_AXIS_VELOCITY, 0, 0));
    }
}
