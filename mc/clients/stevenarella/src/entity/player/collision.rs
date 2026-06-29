use crate::entity::TargetPosition;
use crate::shared::Position as BPosition;
use crate::world;
use cgmath::{self, Vector3};
use collision::{Aabb, Aabb3};

const COLLISION_SCAN_MARGIN: f64 = 1.0;
const COLLISION_SEPARATION: f64 = 0.0001;

pub(super) fn check_collisions(
    world: &world::World,
    position: &mut TargetPosition,
    last_position: &Vector3<f64>,
    bounds: Aabb3<f64>,
) -> (Aabb3<f64>, bool) {
    let mut bounds = bounds.add_v(position.position);
    let dir = position.position - last_position;
    let scan = scan_bounds(bounds);

    let mut hit = false;
    for y in scan.min_y..scan.max_y {
        for z in scan.min_z..scan.max_z {
            for x in scan.min_x..scan.max_x {
                let block = world.get_block(BPosition::new(x, y, z));
                if block.get_material().collidable {
                    for bb in block.get_collision_boxes() {
                        let bb = bb.add_v(cgmath::Vector3::new(x as f64, y as f64, z as f64));
                        if aabb_collides(&bb, &bounds) {
                            bounds = move_bounds_out_of(bounds, bb, dir);
                            hit = true;
                        }
                    }
                }
            }
        }
    }

    (bounds, hit)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct CollisionScanBounds {
    min_x: i32,
    min_y: i32,
    min_z: i32,
    max_x: i32,
    max_y: i32,
    max_z: i32,
}

fn scan_bounds(bounds: Aabb3<f64>) -> CollisionScanBounds {
    CollisionScanBounds {
        min_x: (bounds.min.x - COLLISION_SCAN_MARGIN) as i32,
        min_y: (bounds.min.y - COLLISION_SCAN_MARGIN) as i32,
        min_z: (bounds.min.z - COLLISION_SCAN_MARGIN) as i32,
        max_x: (bounds.max.x + COLLISION_SCAN_MARGIN) as i32,
        max_y: (bounds.max.y + COLLISION_SCAN_MARGIN) as i32,
        max_z: (bounds.max.z + COLLISION_SCAN_MARGIN) as i32,
    }
}

pub(super) fn aabb_collides(left: &Aabb3<f64>, right: &Aabb3<f64>) -> bool {
    !(right.min.x >= left.max.x
        || right.max.x <= left.min.x
        || right.min.y >= left.max.y
        || right.max.y <= left.min.y
        || right.min.z >= left.max.z
        || right.max.z <= left.min.z)
}

pub(super) fn move_bounds_out_of(
    mut bounds: Aabb3<f64>,
    obstacle: Aabb3<f64>,
    dir: cgmath::Vector3<f64>,
) -> Aabb3<f64> {
    if dir.x != 0.0 {
        if dir.x > 0.0 {
            let old_max_x = bounds.max.x;
            bounds.max.x = obstacle.min.x - COLLISION_SEPARATION;
            bounds.min.x += bounds.max.x - old_max_x;
        } else {
            let old_min_x = bounds.min.x;
            bounds.min.x = obstacle.max.x + COLLISION_SEPARATION;
            bounds.max.x += bounds.min.x - old_min_x;
        }
    }
    if dir.y != 0.0 {
        if dir.y > 0.0 {
            let old_max_y = bounds.max.y;
            bounds.max.y = obstacle.min.y - COLLISION_SEPARATION;
            bounds.min.y += bounds.max.y - old_max_y;
        } else {
            let old_min_y = bounds.min.y;
            bounds.min.y = obstacle.max.y + COLLISION_SEPARATION;
            bounds.max.y += bounds.min.y - old_min_y;
        }
    }
    if dir.z != 0.0 {
        if dir.z > 0.0 {
            let old_max_z = bounds.max.z;
            bounds.max.z = obstacle.min.z - COLLISION_SEPARATION;
            bounds.min.z += bounds.max.z - old_max_z;
        } else {
            let old_min_z = bounds.min.z;
            bounds.min.z = obstacle.max.z + COLLISION_SEPARATION;
            bounds.max.z += bounds.min.z - old_min_z;
        }
    }
    bounds
}

#[cfg(test)]
mod tests {
    use super::*;
    use cgmath::Point3;

    const EPSILON: f64 = 0.000_001;
    const UNIT_MIN: f64 = 0.0;
    const UNIT_MAX: f64 = 1.0;
    const OVERLAP_MIN: f64 = 0.5;
    const SHIFTED_MIN: f64 = 1.0;
    const NEGATIVE_DIRECTION: f64 = -1.0;
    const POSITIVE_DIRECTION: f64 = 1.0;

    fn unit_box() -> Aabb3<f64> {
        Aabb3::new(
            Point3::new(UNIT_MIN, UNIT_MIN, UNIT_MIN),
            Point3::new(UNIT_MAX, UNIT_MAX, UNIT_MAX),
        )
    }

    fn overlapping_box() -> Aabb3<f64> {
        Aabb3::new(
            Point3::new(OVERLAP_MIN, UNIT_MIN, UNIT_MIN),
            Point3::new(SHIFTED_MIN + OVERLAP_MIN, UNIT_MAX, UNIT_MAX),
        )
    }

    #[test]
    fn player_collision_core_detects_overlapping_bounds() {
        assert!(aabb_collides(&unit_box(), &overlapping_box()));
    }

    #[test]
    fn player_collision_core_treats_touching_faces_as_not_colliding() {
        let touching = Aabb3::new(
            Point3::new(SHIFTED_MIN, UNIT_MIN, UNIT_MIN),
            Point3::new(SHIFTED_MIN + UNIT_MAX, UNIT_MAX, UNIT_MAX),
        );

        assert!(!aabb_collides(&unit_box(), &touching));
    }

    #[test]
    fn player_collision_core_moves_positive_x_out_of_obstacle() {
        let moved = move_bounds_out_of(
            overlapping_box(),
            unit_box(),
            Vector3::new(POSITIVE_DIRECTION, 0.0, 0.0),
        );

        assert!((moved.max.x - (UNIT_MIN - COLLISION_SEPARATION)).abs() < EPSILON);
    }

    #[test]
    fn player_collision_core_moves_negative_x_out_of_obstacle() {
        let moved = move_bounds_out_of(
            overlapping_box(),
            unit_box(),
            Vector3::new(NEGATIVE_DIRECTION, 0.0, 0.0),
        );

        assert!((moved.min.x - (UNIT_MAX + COLLISION_SEPARATION)).abs() < EPSILON);
    }
}
