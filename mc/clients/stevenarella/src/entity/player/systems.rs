use super::collision::check_collisions;
use super::movement::PlayerMovement;
use super::rendering::PlayerRenderer;
use crate::ecs;
use crate::entity::{Bounds, Gravity, Rotation, TargetPosition, Velocity};
use crate::render;
use crate::settings::Stevenkey;
use crate::types::Gamemode;
use crate::world;
use cgmath::Point3;
use collision::{Aabb, Aabb3};
use instant::Instant;

const CHUNK_COORDINATE_SHIFT: i32 = 4;
const WALK_SPEED: f64 = 0.21585;
const SPRINT_SPEED: f64 = 0.2806;
const FLYING_SPEED_MULTIPLIER: f64 = 2.5;
const JUMP_VELOCITY: f64 = 0.42;
const JUMP_VELOCITY_EPSILON: f64 = 0.001;
const GRAVITY_ACCELERATION: f64 = 0.08;
const MAX_FALL_SPEED: f64 = -3.92;
const VELOCITY_DAMPING: f64 = 0.98;
const PLAYER_COLLISION_RADIUS: f64 = 0.3;
const STEP_OFFSET_COUNT: i32 = 9;
const STEP_OFFSET_START: i32 = 1;
const STEP_HEIGHT_UNIT: f64 = 16.0;
const GROUND_CHECK_MIN_Y: f64 = -0.005;
const GROUND_CHECK_MAX_Y: f64 = 0.0;

pub(super) fn add_systems(m: &mut ecs::Manager) {
    let sys = MovementHandler::new(m);
    m.add_system(sys);
    let sys = PlayerRenderer::new(m);
    m.add_render_system(sys);
}

struct MovementHandler {
    filter: ecs::Filter,
    movement: ecs::Key<PlayerMovement>,
    gravity: ecs::Key<Gravity>,
    gamemode: ecs::Key<Gamemode>,
    position: ecs::Key<TargetPosition>,
    velocity: ecs::Key<Velocity>,
    bounds: ecs::Key<Bounds>,
    rotation: ecs::Key<Rotation>,
}

impl MovementHandler {
    pub fn new(m: &mut ecs::Manager) -> MovementHandler {
        let movement = m.get_key();
        let position = m.get_key();
        let velocity = m.get_key();
        let bounds = m.get_key();
        let rotation = m.get_key();
        MovementHandler {
            filter: ecs::Filter::new()
                .with(movement)
                .with(position)
                .with(velocity)
                .with(bounds)
                .with(rotation),
            movement,
            gravity: m.get_key(),
            gamemode: m.get_key(),
            position,
            velocity,
            bounds,
            rotation,
        }
    }
}

impl ecs::System for MovementHandler {
    fn filter(&self) -> &ecs::Filter {
        &self.filter
    }

    fn update(&mut self, m: &mut ecs::Manager, world: &mut world::World, _: &mut render::Renderer) {
        for e in m.find(&self.filter) {
            let gamemode = *m.get_component(e, self.gamemode).unwrap();
            let rotation_yaw = m.get_component(e, self.rotation).unwrap().yaw;
            let player_bounds = m.get_component(e, self.bounds).unwrap().bounds;
            let movement_flying = m.get_component(e, self.movement).unwrap().flying;
            let gravity_present = m.get_component(e, self.gravity).is_some();
            if movement_flying && gravity_present {
                m.remove_component(e, self.gravity);
            } else if !movement_flying && !gravity_present {
                m.add_component(e, self.gravity, Gravity::new());
            }
            let gravity_state = m.get_component(e, self.gravity).copied();

            let (flying, jump_pressed, sneak_pressed, sprint_pressed, forward, yaw) = {
                let movement = m.get_component_mut(e, self.movement).unwrap();
                movement.flying |= gamemode.always_fly();

                // Detect double-tapping jump to toggle creative flight
                if movement.is_key_pressed(Stevenkey::Jump) {
                    if movement.when_last_jump_pressed.is_none() {
                        movement.when_last_jump_pressed = Some(Instant::now());
                        if movement.when_last_jump_released.is_some() {
                            let dt = movement.when_last_jump_pressed.unwrap()
                                - movement.when_last_jump_released.unwrap();
                            if dt.as_secs() == 0
                                && dt.subsec_millis() <= crate::settings::DOUBLE_JUMP_MS
                            {
                                movement.want_to_fly = !movement.want_to_fly;

                                if gamemode.can_fly() && !gamemode.always_fly() {
                                    movement.flying = movement.want_to_fly;
                                }
                            }
                        }
                    }
                } else if movement.when_last_jump_pressed.is_some() {
                    movement.when_last_jump_released = Some(Instant::now());
                    movement.when_last_jump_pressed = None;
                }

                let movement_vector = movement.calculate_movement(rotation_yaw);
                (
                    movement.flying,
                    movement.is_key_pressed(Stevenkey::Jump),
                    movement.is_key_pressed(Stevenkey::Sneak),
                    movement.is_key_pressed(Stevenkey::Sprint),
                    movement_vector.forward,
                    movement_vector.yaw,
                )
            };

            let mut ground_update = None;
            {
                let (position, velocity) = m
                    .get_two_components_mut(e, self.position, self.velocity)
                    .unwrap();
                let mut last_position = position.position;

                if world.is_chunk_loaded(
                    (position.position.x as i32) >> CHUNK_COORDINATE_SHIFT,
                    (position.position.z as i32) >> CHUNK_COORDINATE_SHIFT,
                ) {
                    let mut speed = if sprint_pressed {
                        SPRINT_SPEED
                    } else {
                        WALK_SPEED
                    };
                    if flying {
                        speed *= FLYING_SPEED_MULTIPLIER;

                        if jump_pressed {
                            position.position.y += speed;
                        }
                        if sneak_pressed {
                            position.position.y -= speed;
                        }
                    } else if gravity_state.map_or(false, |v| v.on_ground) {
                        if jump_pressed && velocity.velocity.y.abs() < JUMP_VELOCITY_EPSILON {
                            velocity.velocity.y = JUMP_VELOCITY;
                        }
                    } else {
                        velocity.velocity.y -= GRAVITY_ACCELERATION;
                        if velocity.velocity.y < MAX_FALL_SPEED {
                            velocity.velocity.y = MAX_FALL_SPEED;
                        }
                    }
                    velocity.velocity.y *= VELOCITY_DAMPING;
                    position.position.x += forward * yaw.cos() * speed;
                    position.position.z -= forward * yaw.sin() * speed;
                    position.position.y += velocity.velocity.y;

                    if !gamemode.noclip() {
                        let mut target = position.position;
                        position.position.y = last_position.y;
                        position.position.z = last_position.z;

                        // We handle each axis separately to allow for a sliding
                        // effect when pushing up against walls.

                        let (bounds, xhit) =
                            check_collisions(world, position, &last_position, player_bounds);
                        position.position.x = bounds.min.x + PLAYER_COLLISION_RADIUS;
                        last_position.x = position.position.x;

                        position.position.z = target.z;
                        let (bounds, zhit) =
                            check_collisions(world, position, &last_position, player_bounds);
                        position.position.z = bounds.min.z + PLAYER_COLLISION_RADIUS;
                        last_position.z = position.position.z;

                        // Half block jumps
                        // Minecraft lets you 'jump' up 0.5 blocks
                        // for slabs and stairs (or smaller blocks).
                        // Currently we implement this as a teleport to the
                        // top of the block if we could move there
                        // but this isn't smooth.
                        if (xhit || zhit) && gravity_state.map_or(false, |v| v.on_ground) {
                            let mut ox = position.position.x;
                            let mut oz = position.position.z;
                            position.position.x = target.x;
                            position.position.z = target.z;
                            for offset in STEP_OFFSET_START..STEP_OFFSET_COUNT {
                                let mini = player_bounds.add_v(cgmath::Vector3::new(
                                    0.0,
                                    f64::from(offset) / STEP_HEIGHT_UNIT,
                                    0.0,
                                ));
                                let (_, hit) =
                                    check_collisions(world, position, &last_position, mini);
                                if !hit {
                                    target.y += f64::from(offset) / STEP_HEIGHT_UNIT;
                                    ox = target.x;
                                    oz = target.z;
                                    break;
                                }
                            }
                            position.position.x = ox;
                            position.position.z = oz;
                        }

                        position.position.y = target.y;
                        let (bounds, yhit) =
                            check_collisions(world, position, &last_position, player_bounds);
                        position.position.y = bounds.min.y;
                        last_position.y = position.position.y;
                        if yhit {
                            velocity.velocity.y = 0.0;
                        }

                        if let Some(gravity) = gravity_state {
                            let ground = Aabb3::new(
                                Point3::new(
                                    -PLAYER_COLLISION_RADIUS,
                                    GROUND_CHECK_MIN_Y,
                                    -PLAYER_COLLISION_RADIUS,
                                ),
                                Point3::new(
                                    PLAYER_COLLISION_RADIUS,
                                    GROUND_CHECK_MAX_Y,
                                    PLAYER_COLLISION_RADIUS,
                                ),
                            );
                            let (_, hit) =
                                check_collisions(world, position, &last_position, ground);
                            ground_update = Some((hit, !gravity.on_ground && hit));
                        }
                    }
                }
            }

            if let Some((on_ground, did_touch_ground)) = ground_update {
                if let Some(gravity) = m.get_component_mut(e, self.gravity) {
                    gravity.on_ground = on_ground;
                }
                if did_touch_ground {
                    m.get_component_mut(e, self.movement)
                        .unwrap()
                        .did_touch_ground = true;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn player_movement_system_filter_matches_no_empty_entities() {
        let mut manager = ecs::Manager::new();
        let handler = MovementHandler::new(&mut manager);

        assert!(manager.find(&handler.filter).is_empty());
    }
}
