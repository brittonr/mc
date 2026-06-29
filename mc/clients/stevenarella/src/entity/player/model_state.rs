use crate::render::model;

const PLAYER_MODEL_PART_COUNT: usize = 7;
const INITIAL_WALK_DIRECTION: i32 = 0;
const FORWARD_WALK_DIRECTION: i32 = 1;
const BACKWARD_WALK_DIRECTION: i32 = -1;
const WALK_CYCLE_MIDPOINT: f64 = 15.0;
const WALK_CYCLE_MAX: f64 = 30.0;
const WALK_CYCLE_STEP_SCALE: f64 = 1.5;
const STILL_TIME_TO_CENTER_WALK: f64 = 2.0;
const IDLE_TIME_STEP_SCALE: f64 = 0.02;
const IDLE_TIME_WRAP: f64 = std::f64::consts::PI * 2.0;
const WALK_ANGLE_SCALE: f64 = std::f64::consts::PI / 4.0;
const ARM_TIME_MIN: f64 = 0.0;

pub struct PlayerModel {
    pub(super) model: Option<model::ModelKey>,
    pub(super) skin_url: Option<String>,
    pub(super) dirty: bool,
    pub(super) name: String,

    pub(super) has_head: bool,
    pub(super) has_name_tag: bool,
    pub(super) first_person: bool,

    pub(super) dir: i32,
    pub(super) time: f64,
    pub(super) still_time: f64,
    pub(super) idle_time: f64,
    pub(super) arm_time: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct PlayerVisibility {
    pub(super) has_head: bool,
    pub(super) has_name_tag: bool,
    pub(super) first_person: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum PlayerModelPart {
    Head,
    Body,
    LegLeft,
    LegRight,
    ArmLeft,
    ArmRight,
    NameTag,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct AnimationState {
    pub(super) dir: i32,
    pub(super) time: f64,
    pub(super) still_time: f64,
    pub(super) idle_time: f64,
    pub(super) arm_time: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct AnimationInput {
    pub(super) delta: f64,
    pub(super) position_moved: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct AnimationRenderValues {
    pub(super) walk_angle: f64,
    pub(super) idle_time: f64,
    pub(super) arm_time: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct AnimationFrame {
    pub(super) render: AnimationRenderValues,
    pub(super) next_state: AnimationState,
}

impl PlayerModel {
    pub fn new(name: &str, has_head: bool, has_name_tag: bool, first_person: bool) -> PlayerModel {
        PlayerModel {
            model: None,
            skin_url: None,
            dirty: false,
            name: name.to_owned(),

            has_head,
            has_name_tag,
            first_person,

            dir: INITIAL_WALK_DIRECTION,
            time: 0.0,
            still_time: 0.0,
            idle_time: 0.0,
            arm_time: 0.0,
        }
    }

    pub fn set_skin(&mut self, skin: Option<String>) {
        self.skin_url = skin;
        self.dirty = true;
    }

    #[cfg(test)]
    pub(super) fn name(&self) -> &str {
        &self.name
    }

    pub(super) fn visibility(&self) -> PlayerVisibility {
        PlayerVisibility {
            has_head: self.has_head,
            has_name_tag: self.has_name_tag,
            first_person: self.first_person,
        }
    }

    pub(super) fn animation_state(&self) -> AnimationState {
        AnimationState {
            dir: self.dir,
            time: self.time,
            still_time: self.still_time,
            idle_time: self.idle_time,
            arm_time: self.arm_time,
        }
    }

    pub(super) fn set_animation_state(&mut self, state: AnimationState) {
        self.dir = state.dir;
        self.time = state.time;
        self.still_time = state.still_time;
        self.idle_time = state.idle_time;
        self.arm_time = state.arm_time;
    }
}

impl PlayerModelPart {
    pub(super) fn as_index(self) -> usize {
        match self {
            PlayerModelPart::Head => 0,
            PlayerModelPart::Body => 1,
            PlayerModelPart::LegLeft => 2,
            PlayerModelPart::LegRight => 3,
            PlayerModelPart::ArmLeft => 4,
            PlayerModelPart::ArmRight => 5,
            PlayerModelPart::NameTag => 6,
        }
    }
}

pub(super) fn visible_model_parts(visibility: PlayerVisibility) -> [bool; PLAYER_MODEL_PART_COUNT] {
    let mut parts = [true; PLAYER_MODEL_PART_COUNT];
    parts[PlayerModelPart::Head.as_index()] = visibility.has_head;
    parts[PlayerModelPart::NameTag.as_index()] = visibility.has_name_tag;
    parts
}

pub(super) fn animation_frame(state: AnimationState, input: AnimationInput) -> AnimationFrame {
    let mut render_time = state.time;
    let mut dir = state.dir;
    if dir == INITIAL_WALK_DIRECTION {
        dir = FORWARD_WALK_DIRECTION;
        render_time = WALK_CYCLE_MIDPOINT;
    }

    let render = AnimationRenderValues {
        walk_angle: walk_angle(render_time),
        idle_time: next_idle_time(state.idle_time, input.delta),
        arm_time: next_arm_time(state.arm_time, input.delta),
    };

    let mut next_state = AnimationState {
        dir,
        time: render_time,
        still_time: state.still_time,
        idle_time: render.idle_time,
        arm_time: render.arm_time,
    };
    advance_walk_state(&mut next_state, input);

    AnimationFrame { render, next_state }
}

fn walk_angle(time: f64) -> f64 {
    ((time / WALK_CYCLE_MIDPOINT) - 1.0) * WALK_ANGLE_SCALE
}

fn next_idle_time(idle_time: f64, delta: f64) -> f64 {
    let mut next = idle_time + delta * IDLE_TIME_STEP_SCALE;
    if next > IDLE_TIME_WRAP {
        next -= IDLE_TIME_WRAP;
    }
    next
}

fn next_arm_time(arm_time: f64, delta: f64) -> f64 {
    if arm_time <= ARM_TIME_MIN {
        ARM_TIME_MIN
    } else {
        arm_time - delta
    }
}

fn advance_walk_state(state: &mut AnimationState, input: AnimationInput) {
    let mut update = true;
    if input.position_moved {
        state.still_time = 0.0;
    } else if state.still_time > STILL_TIME_TO_CENTER_WALK {
        if (state.time - WALK_CYCLE_MIDPOINT).abs() <= WALK_CYCLE_STEP_SCALE * input.delta {
            state.time = WALK_CYCLE_MIDPOINT;
            update = false;
        }
        state.dir = (WALK_CYCLE_MIDPOINT - state.time).signum() as i32;
    } else {
        state.still_time += input.delta;
    }

    if update {
        state.time += input.delta * WALK_CYCLE_STEP_SCALE * f64::from(state.dir);
        if state.time > WALK_CYCLE_MAX {
            state.time = WALK_CYCLE_MAX;
            state.dir = BACKWARD_WALK_DIRECTION;
        } else if state.time < 0.0 {
            state.time = 0.0;
            state.dir = FORWARD_WALK_DIRECTION;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DELTA: f64 = 1.0;
    const TEST_ARM_TIME: f64 = 4.0;
    const TEST_IDLE_TIME: f64 = std::f64::consts::PI * 2.0;
    const MODEL_NAME: &str = "model-name";

    #[test]
    fn player_model_visibility_core_keeps_expected_local_parts_hidden() {
        let visibility = PlayerVisibility {
            has_head: false,
            has_name_tag: false,
            first_person: true,
        };
        let parts = visible_model_parts(visibility);

        assert!(!parts[PlayerModelPart::Head.as_index()]);
        assert!(parts[PlayerModelPart::Body.as_index()]);
        assert!(parts[PlayerModelPart::LegLeft.as_index()]);
        assert!(parts[PlayerModelPart::LegRight.as_index()]);
        assert!(parts[PlayerModelPart::ArmLeft.as_index()]);
        assert!(parts[PlayerModelPart::ArmRight.as_index()]);
        assert!(!parts[PlayerModelPart::NameTag.as_index()]);
    }

    #[test]
    fn player_model_visibility_core_keeps_expected_remote_parts_visible() {
        let visibility = PlayerVisibility {
            has_head: true,
            has_name_tag: true,
            first_person: false,
        };
        let parts = visible_model_parts(visibility);

        for part_is_visible in parts {
            assert!(part_is_visible);
        }
    }

    #[test]
    fn player_model_set_skin_marks_model_dirty() {
        let mut model = PlayerModel::new(MODEL_NAME, true, true, false);

        model.set_skin(Some(MODEL_NAME.to_owned()));

        assert_eq!(model.skin_url.as_deref(), Some(MODEL_NAME));
        assert!(model.dirty);
        assert_eq!(model.name(), MODEL_NAME);
    }

    #[test]
    fn player_animation_core_advances_moving_walk_cycle() {
        let state = AnimationState {
            dir: INITIAL_WALK_DIRECTION,
            time: 0.0,
            still_time: STILL_TIME_TO_CENTER_WALK,
            idle_time: 0.0,
            arm_time: TEST_ARM_TIME,
        };

        let frame = animation_frame(
            state,
            AnimationInput {
                delta: TEST_DELTA,
                position_moved: true,
            },
        );

        assert_eq!(frame.render.walk_angle, 0.0);
        assert_eq!(frame.next_state.still_time, 0.0);
        assert_eq!(
            frame.next_state.time,
            WALK_CYCLE_MIDPOINT + WALK_CYCLE_STEP_SCALE
        );
        assert_eq!(frame.next_state.dir, FORWARD_WALK_DIRECTION);
        assert_eq!(frame.render.arm_time, TEST_ARM_TIME - TEST_DELTA);
    }

    #[test]
    fn player_animation_core_recenters_idle_walk_cycle() {
        let state = AnimationState {
            dir: FORWARD_WALK_DIRECTION,
            time: WALK_CYCLE_MIDPOINT,
            still_time: STILL_TIME_TO_CENTER_WALK + TEST_DELTA,
            idle_time: TEST_IDLE_TIME,
            arm_time: 0.0,
        };

        let frame = animation_frame(
            state,
            AnimationInput {
                delta: TEST_DELTA,
                position_moved: false,
            },
        );

        assert_eq!(frame.next_state.time, WALK_CYCLE_MIDPOINT);
        assert_eq!(frame.next_state.arm_time, ARM_TIME_MIN);
        assert!(frame.next_state.idle_time < TEST_IDLE_TIME);
    }
}
