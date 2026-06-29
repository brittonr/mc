use crate::settings::Stevenkey;
use crate::types::hash::FNVHash;
use instant::Instant;
use std::collections::HashMap;
use std::hash::BuildHasherDefault;

const HALF_TURN_RADIANS: f64 = std::f64::consts::PI;
const RIGHT_ANGLE_RADIANS: f64 = std::f64::consts::PI / 2.0;
const MOVING_FORWARD_AMOUNT: f64 = 1.0;
const STATIONARY_FORWARD_AMOUNT: f64 = 0.0;
const DEFAULT_INVALID_YAW: f64 = 0.0;

#[derive(Default)]
pub struct PlayerMovement {
    pub flying: bool,
    pub want_to_fly: bool,
    pub when_last_jump_pressed: Option<Instant>,
    pub when_last_jump_released: Option<Instant>,
    pub did_touch_ground: bool,
    pub pressed_keys: HashMap<Stevenkey, bool, BuildHasherDefault<FNVHash>>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) struct MovementKeys {
    pub(super) forward: bool,
    pub(super) backward: bool,
    pub(super) left: bool,
    pub(super) right: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct MovementVector {
    pub(super) forward: f64,
    pub(super) yaw: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum MovementInputError {
    NonFiniteYaw,
}

impl PlayerMovement {
    pub fn new() -> PlayerMovement {
        Default::default()
    }

    pub(super) fn calculate_movement(&self, player_yaw: f64) -> MovementVector {
        movement_vector(self.movement_keys(), player_yaw)
            .unwrap_or_else(|_| MovementVector::stationary())
    }

    pub(super) fn is_key_pressed(&self, key: Stevenkey) -> bool {
        self.pressed_keys.get(&key).map_or(false, |v| *v)
    }

    fn movement_keys(&self) -> MovementKeys {
        MovementKeys {
            forward: self.is_key_pressed(Stevenkey::Forward),
            backward: self.is_key_pressed(Stevenkey::Backward),
            left: self.is_key_pressed(Stevenkey::Left),
            right: self.is_key_pressed(Stevenkey::Right),
        }
    }
}

impl MovementVector {
    fn stationary() -> MovementVector {
        MovementVector {
            forward: STATIONARY_FORWARD_AMOUNT,
            yaw: DEFAULT_INVALID_YAW,
        }
    }
}

pub(super) fn movement_vector(
    keys: MovementKeys,
    player_yaw: f64,
) -> Result<MovementVector, MovementInputError> {
    if !player_yaw.is_finite() {
        return Err(MovementInputError::NonFiniteYaw);
    }

    let mut forward = STATIONARY_FORWARD_AMOUNT;
    let mut yaw = player_yaw - RIGHT_ANGLE_RADIANS;
    if keys.forward || keys.backward {
        forward = MOVING_FORWARD_AMOUNT;
        if keys.backward {
            yaw += HALF_TURN_RADIANS;
        }
    }
    let change = if keys.left {
        RIGHT_ANGLE_RADIANS / (forward.abs() + MOVING_FORWARD_AMOUNT)
    } else if keys.right {
        -RIGHT_ANGLE_RADIANS / (forward.abs() + MOVING_FORWARD_AMOUNT)
    } else {
        0.0
    };
    if keys.left || keys.right {
        forward = MOVING_FORWARD_AMOUNT;
    }
    if keys.backward {
        yaw -= change;
    } else {
        yaw += change;
    }

    Ok(MovementVector { forward, yaw })
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_YAW: f64 = 0.0;
    const EPSILON: f64 = 0.000_001;
    const BACKWARD_LEFT_EXPECTED_YAW_DIVISOR: f64 = 2.0;

    fn assert_close(left: f64, right: f64) {
        assert!((left - right).abs() < EPSILON, "left={left} right={right}");
    }

    #[test]
    fn player_movement_core_moves_forward_from_yaw() {
        let vector = movement_vector(
            MovementKeys {
                forward: true,
                ..MovementKeys::default()
            },
            TEST_YAW,
        )
        .unwrap();

        assert_eq!(vector.forward, MOVING_FORWARD_AMOUNT);
        assert_close(vector.yaw, -RIGHT_ANGLE_RADIANS);
    }

    #[test]
    fn player_movement_core_moves_diagonal_backward_left() {
        let vector = movement_vector(
            MovementKeys {
                backward: true,
                left: true,
                ..MovementKeys::default()
            },
            TEST_YAW,
        )
        .unwrap();

        assert_eq!(vector.forward, MOVING_FORWARD_AMOUNT);
        assert_close(
            vector.yaw,
            RIGHT_ANGLE_RADIANS / BACKWARD_LEFT_EXPECTED_YAW_DIVISOR,
        );
    }

    #[test]
    fn player_movement_core_rejects_non_finite_yaw() {
        assert_eq!(
            movement_vector(MovementKeys::default(), f64::NAN),
            Err(MovementInputError::NonFiniteYaw)
        );
    }

    #[test]
    fn player_movement_shell_contains_invalid_yaw_as_stationary() {
        let movement = PlayerMovement::new();

        assert_eq!(
            movement.calculate_movement(f64::INFINITY),
            MovementVector::stationary()
        );
    }

    #[test]
    fn player_movement_state_reports_absent_key_as_unpressed() {
        let movement = PlayerMovement::new();

        assert!(!movement.is_key_pressed(Stevenkey::Forward));
    }
}
