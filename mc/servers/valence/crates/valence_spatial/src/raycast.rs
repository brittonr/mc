//! Pure raycast and collision ordering helpers.
//!
//! These helpers keep world and ECS access outside the math core. Callers gather
//! entity hitboxes and block collision shapes, then pass explicit candidates into
//! the pure functions here. Equal entity and block distances produce a
//! deterministic [`RaycastCollision::Tie`] instead of depending on query order.

const NO_DISTANCE: f64 = 0.0;
const UNIT_DISTANCE: f64 = 1.0;
const FORWARD_STEP: i32 = 1;
const BACKWARD_STEP: i32 = -1;
const STATIONARY_STEP: i32 = 0;

/// An invalid raycast input rejected by the pure helpers.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RaycastInputError {
    /// The ray origin contains a non-finite coordinate or cannot fit a voxel coordinate.
    InvalidOrigin,
    /// The ray direction contains a non-finite coordinate.
    NonFiniteDirection,
    /// The ray direction has zero length.
    ZeroDirection,
    /// The maximum ray distance is negative or non-finite.
    InvalidMaxDistance,
    /// The voxel traversal bounds have `min > max` on at least one axis.
    InvalidBounds,
    /// A candidate AABB contains non-finite coordinates or has `min > max`.
    InvalidAabb,
}

/// A checked ray with normalized direction and a finite maximum distance.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RaycastRay {
    origin: vek::Vec3<f64>,
    direction: vek::Vec3<f64>,
    max_distance: f64,
}

impl RaycastRay {
    /// Creates a checked ray.
    ///
    /// The direction is normalized on success. Zero-length directions, NaN
    /// coordinates, infinite coordinates, and invalid distance bounds are
    /// rejected before any traversal or intersection work starts.
    pub fn new(
        origin: vek::Vec3<f64>,
        direction: vek::Vec3<f64>,
        max_distance: f64,
    ) -> Result<Self, RaycastInputError> {
        if !vec3_is_finite(origin) {
            return Err(RaycastInputError::InvalidOrigin);
        }
        if !vec3_is_finite(direction) {
            return Err(RaycastInputError::NonFiniteDirection);
        }
        if !max_distance.is_finite() || max_distance < NO_DISTANCE {
            return Err(RaycastInputError::InvalidMaxDistance);
        }

        let direction_length_squared = direction.x.mul_add(
            direction.x,
            direction.y.mul_add(direction.y, direction.z * direction.z),
        );
        if direction_length_squared <= NO_DISTANCE {
            return Err(RaycastInputError::ZeroDirection);
        }

        let direction_length = direction_length_squared.sqrt();
        let direction = vek::Vec3::new(
            direction.x / direction_length,
            direction.y / direction_length,
            direction.z / direction_length,
        );

        Ok(Self {
            origin,
            direction,
            max_distance,
        })
    }

    /// Returns the ray origin.
    pub fn origin(self) -> vek::Vec3<f64> {
        self.origin
    }

    /// Returns the normalized ray direction.
    pub fn direction(self) -> vek::Vec3<f64> {
        self.direction
    }

    /// Returns the maximum distance considered by this ray.
    pub fn max_distance(self) -> f64 {
        self.max_distance
    }

    /// Returns the point at `distance` along the ray.
    pub fn point_at(self, distance: f64) -> vek::Vec3<f64> {
        vek::Vec3::new(
            self.origin.x + self.direction.x * distance,
            self.origin.y + self.direction.y * distance,
            self.origin.z + self.direction.z * distance,
        )
    }

    /// Returns an iterator over voxel cells touched by this ray inside `bounds`.
    ///
    /// Bounds are inclusive. If the origin starts outside the bounds, the
    /// iterator is empty. Ties at voxel corners are resolved in X, then Y, then
    /// Z order so the sequence is deterministic.
    pub fn voxels(self, bounds: VoxelRaycastBounds) -> Result<VoxelRaycast, RaycastInputError> {
        VoxelRaycast::new(self, bounds)
    }
}

/// Inclusive integer bounds for voxel traversal.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VoxelRaycastBounds {
    min: vek::Vec3<i32>,
    max: vek::Vec3<i32>,
}

impl VoxelRaycastBounds {
    /// Creates inclusive voxel bounds.
    pub fn new(min: vek::Vec3<i32>, max: vek::Vec3<i32>) -> Result<Self, RaycastInputError> {
        if min.x > max.x || min.y > max.y || min.z > max.z {
            return Err(RaycastInputError::InvalidBounds);
        }
        Ok(Self { min, max })
    }

    /// Returns the minimum included voxel coordinate.
    pub fn min(self) -> vek::Vec3<i32> {
        self.min
    }

    /// Returns the maximum included voxel coordinate.
    pub fn max(self) -> vek::Vec3<i32> {
        self.max
    }

    fn contains(self, position: vek::Vec3<i32>) -> bool {
        self.min.x <= position.x
            && position.x <= self.max.x
            && self.min.y <= position.y
            && position.y <= self.max.y
            && self.min.z <= position.z
            && position.z <= self.max.z
    }
}

/// A voxel cell visited by a ray.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RaycastVoxel {
    /// The integer voxel coordinate.
    pub position: vek::Vec3<i32>,
    /// The distance at which the ray entered this voxel.
    pub near: f64,
}

/// Iterator over voxel cells touched by a checked ray.
#[derive(Clone, Debug)]
pub struct VoxelRaycast {
    ray: RaycastRay,
    bounds: VoxelRaycastBounds,
    current: vek::Vec3<i32>,
    step: vek::Vec3<i32>,
    next_boundary: vek::Vec3<f64>,
    boundary_delta: vek::Vec3<f64>,
    current_distance: f64,
    done: bool,
}

impl VoxelRaycast {
    fn new(ray: RaycastRay, bounds: VoxelRaycastBounds) -> Result<Self, RaycastInputError> {
        let current = vek::Vec3::new(
            floor_to_i32(ray.origin.x)?,
            floor_to_i32(ray.origin.y)?,
            floor_to_i32(ray.origin.z)?,
        );
        let step = vek::Vec3::new(
            step_for_direction(ray.direction.x),
            step_for_direction(ray.direction.y),
            step_for_direction(ray.direction.z),
        );
        let next_boundary = vek::Vec3::new(
            next_boundary_distance(ray.origin.x, ray.direction.x, current.x, step.x),
            next_boundary_distance(ray.origin.y, ray.direction.y, current.y, step.y),
            next_boundary_distance(ray.origin.z, ray.direction.z, current.z, step.z),
        );
        let boundary_delta = vek::Vec3::new(
            boundary_delta(ray.direction.x),
            boundary_delta(ray.direction.y),
            boundary_delta(ray.direction.z),
        );

        Ok(Self {
            ray,
            bounds,
            current,
            step,
            next_boundary,
            boundary_delta,
            current_distance: NO_DISTANCE,
            done: false,
        })
    }

    fn advance(&mut self) {
        match next_axis(self.next_boundary) {
            Axis::X => self.advance_x(),
            Axis::Y => self.advance_y(),
            Axis::Z => self.advance_z(),
        }
    }

    fn advance_x(&mut self) {
        self.current_distance = self.next_boundary.x;
        let Some(next_x) = self.current.x.checked_add(self.step.x) else {
            self.done = true;
            return;
        };
        self.current.x = next_x;
        self.next_boundary.x += self.boundary_delta.x;
    }

    fn advance_y(&mut self) {
        self.current_distance = self.next_boundary.y;
        let Some(next_y) = self.current.y.checked_add(self.step.y) else {
            self.done = true;
            return;
        };
        self.current.y = next_y;
        self.next_boundary.y += self.boundary_delta.y;
    }

    fn advance_z(&mut self) {
        self.current_distance = self.next_boundary.z;
        let Some(next_z) = self.current.z.checked_add(self.step.z) else {
            self.done = true;
            return;
        };
        self.current.z = next_z;
        self.next_boundary.z += self.boundary_delta.z;
    }
}

impl Iterator for VoxelRaycast {
    type Item = RaycastVoxel;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        if !self.bounds.contains(self.current) {
            self.done = true;
            return None;
        }
        if self.current_distance > self.ray.max_distance {
            self.done = true;
            return None;
        }

        let voxel = RaycastVoxel {
            position: self.current,
            near: self.current_distance,
        };
        self.advance();
        Some(voxel)
    }
}

/// The entry and exit distances for an AABB hit.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RaycastAabbHit {
    /// Distance from the ray origin to the nearest intersection point.
    pub near: f64,
    /// Distance from the ray origin to the exit intersection point.
    pub far: f64,
}

/// Intersects a checked ray with an axis-aligned bounding box.
pub fn intersect_aabb(
    ray: RaycastRay,
    aabb: vek::Aabb<f64>,
) -> Result<Option<RaycastAabbHit>, RaycastInputError> {
    validate_aabb(aabb)?;

    let Some((x_near, x_far)) =
        axis_interval(ray.origin.x, ray.direction.x, aabb.min.x, aabb.max.x)
    else {
        return Ok(None);
    };
    let Some((y_near, y_far)) =
        axis_interval(ray.origin.y, ray.direction.y, aabb.min.y, aabb.max.y)
    else {
        return Ok(None);
    };
    let Some((z_near, z_far)) =
        axis_interval(ray.origin.z, ray.direction.z, aabb.min.z, aabb.max.z)
    else {
        return Ok(None);
    };

    let near = x_near.max(y_near).max(z_near).max(NO_DISTANCE);
    let far = x_far.min(y_far).min(z_far);

    if near <= far && near <= ray.max_distance {
        Ok(Some(RaycastAabbHit { near, far }))
    } else {
        Ok(None)
    }
}

/// An entity AABB candidate supplied by a caller-owned shell.
#[derive(Clone, Copy, Debug)]
pub struct EntityRaycastCandidate<'a, O> {
    /// The entity or caller-owned object associated with this hitbox.
    pub object: &'a O,
    /// The world-space entity hitbox.
    pub aabb: vek::Aabb<f64>,
    /// Whether this candidate is intentionally excluded, such as an owner/self hitbox.
    pub excluded: bool,
}

/// An entity raycast hit.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EntityRaycastHit<'a, O> {
    /// The entity or caller-owned object that was hit.
    pub object: &'a O,
    /// Distance from the ray origin to the nearest hitbox intersection.
    pub near: f64,
    /// Distance from the ray origin to the hitbox exit point.
    pub far: f64,
}

/// Returns the nearest non-excluded entity hit.
pub fn first_entity_hit<'a, O, I>(
    ray: RaycastRay,
    candidates: I,
) -> Result<Option<EntityRaycastHit<'a, O>>, RaycastInputError>
where
    I: IntoIterator<Item = EntityRaycastCandidate<'a, O>>,
{
    let mut closest = None;

    for candidate in candidates {
        if candidate.excluded {
            continue;
        }
        let Some(hit) = intersect_aabb(ray, candidate.aabb)? else {
            continue;
        };
        if is_closer_entity_hit(hit.near, closest.as_ref()) {
            closest = Some(EntityRaycastHit {
                object: candidate.object,
                near: hit.near,
                far: hit.far,
            });
        }
    }

    Ok(closest)
}

/// A block collision-shape candidate supplied by a caller-owned shell.
#[derive(Clone, Copy, Debug)]
pub struct BlockRaycastCandidate<B> {
    /// Caller-owned block identifier or metadata.
    pub block: B,
    /// The world-space collision shape AABB.
    pub aabb: vek::Aabb<f64>,
}

/// A block raycast hit.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BlockRaycastHit<B> {
    /// Caller-owned block identifier or metadata.
    pub block: B,
    /// Distance from the ray origin to the nearest block shape intersection.
    pub near: f64,
    /// Distance from the ray origin to the block shape exit point.
    pub far: f64,
}

/// Returns the nearest block hit from explicit world-space candidates.
pub fn first_block_hit<B, I>(
    ray: RaycastRay,
    candidates: I,
) -> Result<Option<BlockRaycastHit<B>>, RaycastInputError>
where
    I: IntoIterator<Item = BlockRaycastCandidate<B>>,
{
    let mut closest = None;

    for candidate in candidates {
        let Some(hit) = intersect_aabb(ray, candidate.aabb)? else {
            continue;
        };
        if is_closer_block_hit(hit.near, closest.as_ref()) {
            closest = Some(BlockRaycastHit {
                block: candidate.block,
                near: hit.near,
                far: hit.far,
            });
        }
    }

    Ok(closest)
}

/// Walks voxels and returns the nearest block hit from per-voxel candidates.
///
/// The closure is the thin shell boundary: it can query a chunk layer or test
/// fixture for shapes in the visited voxel, while this function owns traversal,
/// validation, and nearest-hit ordering.
pub fn first_block_hit_in_voxels<B, I, F>(
    ray: RaycastRay,
    bounds: VoxelRaycastBounds,
    mut candidates_for_voxel: F,
) -> Result<Option<BlockRaycastHit<B>>, RaycastInputError>
where
    I: IntoIterator<Item = BlockRaycastCandidate<B>>,
    F: FnMut(RaycastVoxel) -> I,
{
    let mut closest = None;

    for voxel in ray.voxels(bounds)? {
        if is_voxel_past_block_hit(voxel.near, closest.as_ref()) {
            break;
        }
        for candidate in candidates_for_voxel(voxel) {
            let Some(hit) = intersect_aabb(ray, candidate.aabb)? else {
                continue;
            };
            if is_closer_block_hit(hit.near, closest.as_ref()) {
                closest = Some(BlockRaycastHit {
                    block: candidate.block,
                    near: hit.near,
                    far: hit.far,
                });
            }
        }
    }

    Ok(closest)
}

/// Deterministic result of comparing entity and block hits.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RaycastCollision<E, B> {
    /// The entity hit is closer than the block hit or is the only hit.
    Entity(E),
    /// The block hit is closer than the entity hit or is the only hit.
    Block(B),
    /// The nearest entity and block hits occur at the same distance.
    Tie { entity: E, block: B },
}

/// Compares optional entity and block hits without iteration-order dependence.
pub fn compare_entity_block_hits<'a, O, B>(
    entity: Option<EntityRaycastHit<'a, O>>,
    block: Option<BlockRaycastHit<B>>,
) -> Option<RaycastCollision<EntityRaycastHit<'a, O>, BlockRaycastHit<B>>> {
    match (entity, block) {
        (Some(entity), Some(block)) => Some(compare_present_hits(entity, block)),
        (Some(entity), None) => Some(RaycastCollision::Entity(entity)),
        (None, Some(block)) => Some(RaycastCollision::Block(block)),
        (None, None) => None,
    }
}

fn compare_present_hits<'a, O, B>(
    entity: EntityRaycastHit<'a, O>,
    block: BlockRaycastHit<B>,
) -> RaycastCollision<EntityRaycastHit<'a, O>, BlockRaycastHit<B>> {
    if entity.near < block.near {
        return RaycastCollision::Entity(entity);
    }
    if block.near < entity.near {
        return RaycastCollision::Block(block);
    }
    RaycastCollision::Tie { entity, block }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Axis {
    X,
    Y,
    Z,
}

fn vec3_is_finite(vec: vek::Vec3<f64>) -> bool {
    vec.x.is_finite() && vec.y.is_finite() && vec.z.is_finite()
}

fn floor_to_i32(value: f64) -> Result<i32, RaycastInputError> {
    let floored = value.floor();
    if floored < f64::from(i32::MIN) || floored > f64::from(i32::MAX) {
        return Err(RaycastInputError::InvalidOrigin);
    }
    Ok(floored as i32)
}

fn step_for_direction(direction: f64) -> i32 {
    if direction > NO_DISTANCE {
        return FORWARD_STEP;
    }
    if direction < NO_DISTANCE {
        return BACKWARD_STEP;
    }
    STATIONARY_STEP
}

fn next_boundary_distance(origin: f64, direction: f64, current: i32, step: i32) -> f64 {
    match step {
        FORWARD_STEP => (f64::from(current) + UNIT_DISTANCE - origin) / direction,
        BACKWARD_STEP => (f64::from(current) - origin) / direction,
        STATIONARY_STEP => f64::INFINITY,
        _ => unreachable!("step_for_direction only returns known steps"),
    }
}

fn boundary_delta(direction: f64) -> f64 {
    if direction == NO_DISTANCE {
        f64::INFINITY
    } else {
        UNIT_DISTANCE / direction.abs()
    }
}

fn next_axis(next_boundary: vek::Vec3<f64>) -> Axis {
    if next_boundary.x <= next_boundary.y && next_boundary.x <= next_boundary.z {
        return Axis::X;
    }
    if next_boundary.y <= next_boundary.z {
        return Axis::Y;
    }
    Axis::Z
}

fn validate_aabb(aabb: vek::Aabb<f64>) -> Result<(), RaycastInputError> {
    if !vec3_is_finite(aabb.min) || !vec3_is_finite(aabb.max) {
        return Err(RaycastInputError::InvalidAabb);
    }
    if aabb.min.x > aabb.max.x || aabb.min.y > aabb.max.y || aabb.min.z > aabb.max.z {
        return Err(RaycastInputError::InvalidAabb);
    }
    Ok(())
}

fn axis_interval(origin: f64, direction: f64, min: f64, max: f64) -> Option<(f64, f64)> {
    if direction == NO_DISTANCE {
        if origin < min || origin > max {
            None
        } else {
            Some((f64::NEG_INFINITY, f64::INFINITY))
        }
    } else {
        let first = (min - origin) / direction;
        let second = (max - origin) / direction;
        Some((first.min(second), first.max(second)))
    }
}

fn is_closer_entity_hit<O>(near: f64, closest: Option<&EntityRaycastHit<'_, O>>) -> bool {
    match closest {
        Some(closest) => near < closest.near,
        None => true,
    }
}

fn is_closer_block_hit<B>(near: f64, closest: Option<&BlockRaycastHit<B>>) -> bool {
    match closest {
        Some(closest) => near < closest.near,
        None => true,
    }
}

fn is_voxel_past_block_hit<B>(voxel_near: f64, closest: Option<&BlockRaycastHit<B>>) -> bool {
    match closest {
        Some(closest) => closest.near < voxel_near,
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RAY_DISTANCE: f64 = 8.0;
    const SHORT_RAY_DISTANCE: f64 = 2.0;
    const ORIGIN_AXIS_X: f64 = 0.5;
    const ORIGIN_AXIS_Y: f64 = 0.5;
    const ORIGIN_AXIS_Z: f64 = 0.5;
    const UNIT_BOX_MIN: f64 = 0.0;
    const UNIT_BOX_MAX: f64 = 1.0;
    const SECOND_BOX_MIN_X: f64 = 2.0;
    const SECOND_BOX_MAX_X: f64 = 3.0;
    const HIT_BOUNDARY_X: f64 = 1.0;
    const HIT_EXIT_X: f64 = 2.0;
    const NEGATIVE_HALF: f64 = -0.5;
    const POSITIVE_HALF: f64 = 0.5;
    const FAR_BLOCK_MIN_X: f64 = 4.0;
    const FAR_BLOCK_MAX_X: f64 = 5.0;
    const AXIS_BOUND_MIN: i32 = 0;
    const FIRST_STEP_VOXEL: i32 = 1;
    const SECOND_STEP_VOXEL: i32 = 2;
    const THIRD_STEP_VOXEL: i32 = 3;
    const AXIS_BOUND_MAX: i32 = 4;
    const DIAGONAL_BOUND_MAX: i32 = 3;
    const BLOCK_ID: &str = "stone";
    const OWNER_ID: &str = "owner";
    const TARGET_ID: &str = "target";

    #[test]
    fn axis_aligned_voxel_traversal_visits_expected_cells() {
        let ray = axis_ray(RAY_DISTANCE);
        let bounds = VoxelRaycastBounds::new(
            vek::Vec3::new(AXIS_BOUND_MIN, AXIS_BOUND_MIN, AXIS_BOUND_MIN),
            vek::Vec3::new(AXIS_BOUND_MAX, AXIS_BOUND_MIN, AXIS_BOUND_MIN),
        )
        .unwrap();

        let voxels = ray
            .voxels(bounds)
            .unwrap()
            .map(|voxel| voxel.position)
            .collect::<Vec<_>>();

        assert_eq!(
            voxels,
            vec![
                vek::Vec3::new(0, 0, 0),
                vek::Vec3::new(FIRST_STEP_VOXEL, 0, 0),
                vek::Vec3::new(SECOND_STEP_VOXEL, 0, 0),
                vek::Vec3::new(THIRD_STEP_VOXEL, 0, 0),
                vek::Vec3::new(AXIS_BOUND_MAX, 0, 0),
            ]
        );
    }

    #[test]
    fn diagonal_voxel_traversal_uses_deterministic_axis_ties() {
        let ray = RaycastRay::new(
            origin(),
            vek::Vec3::new(UNIT_BOX_MAX, UNIT_BOX_MAX, UNIT_BOX_MIN),
            SHORT_RAY_DISTANCE,
        )
        .unwrap();
        let bounds = VoxelRaycastBounds::new(
            vek::Vec3::new(AXIS_BOUND_MIN, AXIS_BOUND_MIN, AXIS_BOUND_MIN),
            vek::Vec3::new(DIAGONAL_BOUND_MAX, DIAGONAL_BOUND_MAX, AXIS_BOUND_MIN),
        )
        .unwrap();

        let voxels = ray
            .voxels(bounds)
            .unwrap()
            .map(|voxel| voxel.position)
            .collect::<Vec<_>>();

        assert_eq!(
            voxels,
            vec![
                vek::Vec3::new(0, 0, 0),
                vek::Vec3::new(FIRST_STEP_VOXEL, 0, 0),
                vek::Vec3::new(FIRST_STEP_VOXEL, FIRST_STEP_VOXEL, 0),
            ]
        );
    }

    #[test]
    fn inside_hitbox_start_reports_zero_near_distance() {
        let hit = intersect_aabb(axis_ray(RAY_DISTANCE), unit_aabb())
            .unwrap()
            .unwrap();

        assert_eq!(hit.near, NO_DISTANCE);
        assert_eq!(hit.far, POSITIVE_HALF);
    }

    #[test]
    fn boundary_hit_reports_entry_distance() {
        let hit = intersect_aabb(
            RaycastRay::new(
                vek::Vec3::new(UNIT_BOX_MIN, ORIGIN_AXIS_Y, ORIGIN_AXIS_Z),
                vek::Vec3::new(UNIT_BOX_MAX, UNIT_BOX_MIN, UNIT_BOX_MIN),
                RAY_DISTANCE,
            )
            .unwrap(),
            vek::Aabb {
                min: vek::Vec3::new(HIT_BOUNDARY_X, UNIT_BOX_MIN, UNIT_BOX_MIN),
                max: vek::Vec3::new(HIT_EXIT_X, UNIT_BOX_MAX, UNIT_BOX_MAX),
            },
        )
        .unwrap()
        .unwrap();

        assert_eq!(hit.near, HIT_BOUNDARY_X);
        assert_eq!(hit.far, HIT_EXIT_X);
    }

    #[test]
    fn invalid_rays_are_rejected() {
        assert_eq!(
            RaycastRay::new(
                vek::Vec3::new(f64::NAN, UNIT_BOX_MIN, UNIT_BOX_MIN),
                vek::Vec3::new(UNIT_BOX_MAX, UNIT_BOX_MIN, UNIT_BOX_MIN),
                RAY_DISTANCE,
            ),
            Err(RaycastInputError::InvalidOrigin)
        );
        assert_eq!(
            RaycastRay::new(
                origin(),
                vek::Vec3::new(UNIT_BOX_MAX, f64::NAN, UNIT_BOX_MIN),
                RAY_DISTANCE
            ),
            Err(RaycastInputError::NonFiniteDirection)
        );
        assert_eq!(
            RaycastRay::new(
                origin(),
                vek::Vec3::new(UNIT_BOX_MIN, UNIT_BOX_MIN, UNIT_BOX_MIN),
                RAY_DISTANCE
            ),
            Err(RaycastInputError::ZeroDirection)
        );
        assert_eq!(
            RaycastRay::new(
                origin(),
                vek::Vec3::new(UNIT_BOX_MAX, UNIT_BOX_MIN, UNIT_BOX_MIN),
                f64::INFINITY,
            ),
            Err(RaycastInputError::InvalidMaxDistance)
        );
    }

    #[test]
    fn invalid_bounds_and_aabbs_are_rejected() {
        assert_eq!(
            VoxelRaycastBounds::new(vek::Vec3::new(1, 0, 0), vek::Vec3::new(0, 0, 0)),
            Err(RaycastInputError::InvalidBounds)
        );
        assert_eq!(
            intersect_aabb(
                axis_ray(RAY_DISTANCE),
                vek::Aabb {
                    min: vek::Vec3::new(UNIT_BOX_MAX, UNIT_BOX_MIN, UNIT_BOX_MIN),
                    max: vek::Vec3::new(UNIT_BOX_MIN, UNIT_BOX_MAX, UNIT_BOX_MAX),
                },
            ),
            Err(RaycastInputError::InvalidAabb)
        );
    }

    #[test]
    fn owner_exclusion_keeps_other_entity_hits_eligible() {
        let candidates = [
            EntityRaycastCandidate {
                object: &OWNER_ID,
                aabb: unit_aabb(),
                excluded: true,
            },
            EntityRaycastCandidate {
                object: &TARGET_ID,
                aabb: vek::Aabb {
                    min: vek::Vec3::new(SECOND_BOX_MIN_X, UNIT_BOX_MIN, UNIT_BOX_MIN),
                    max: vek::Vec3::new(SECOND_BOX_MAX_X, UNIT_BOX_MAX, UNIT_BOX_MAX),
                },
                excluded: false,
            },
        ];

        let hit = first_entity_hit(axis_ray(RAY_DISTANCE), candidates)
            .unwrap()
            .unwrap();

        assert_eq!(*hit.object, TARGET_ID);
        assert_eq!(hit.near, SECOND_BOX_MIN_X - ORIGIN_AXIS_X);
    }

    #[test]
    fn block_entity_tie_returns_tie_result() {
        let entity = first_entity_hit(
            RaycastRay::new(
                vek::Vec3::new(UNIT_BOX_MIN, UNIT_BOX_MIN, UNIT_BOX_MIN),
                vek::Vec3::new(UNIT_BOX_MAX, UNIT_BOX_MIN, UNIT_BOX_MIN),
                RAY_DISTANCE,
            )
            .unwrap(),
            [EntityRaycastCandidate {
                object: &TARGET_ID,
                aabb: tie_aabb(),
                excluded: false,
            }],
        )
        .unwrap();
        let block = first_block_hit(
            RaycastRay::new(
                vek::Vec3::new(UNIT_BOX_MIN, UNIT_BOX_MIN, UNIT_BOX_MIN),
                vek::Vec3::new(UNIT_BOX_MAX, UNIT_BOX_MIN, UNIT_BOX_MIN),
                RAY_DISTANCE,
            )
            .unwrap(),
            [BlockRaycastCandidate {
                block: BLOCK_ID,
                aabb: tie_aabb(),
            }],
        )
        .unwrap();

        let collision = compare_entity_block_hits(entity, block).unwrap();

        match collision {
            RaycastCollision::Tie { entity, block } => {
                assert_eq!(*entity.object, TARGET_ID);
                assert_eq!(block.block, BLOCK_ID);
            }
            RaycastCollision::Entity(_) | RaycastCollision::Block(_) => {
                panic!("equal distances must produce a tie")
            }
        }
    }

    #[test]
    fn closer_hit_wins_entity_block_comparison() {
        let entity = Some(EntityRaycastHit {
            object: &TARGET_ID,
            near: HIT_BOUNDARY_X,
            far: HIT_EXIT_X,
        });
        let block = Some(BlockRaycastHit {
            block: BLOCK_ID,
            near: SECOND_BOX_MIN_X,
            far: SECOND_BOX_MAX_X,
        });

        assert!(matches!(
            compare_entity_block_hits(entity, block),
            Some(RaycastCollision::Entity(_))
        ));

        let entity = Some(EntityRaycastHit {
            object: &TARGET_ID,
            near: FAR_BLOCK_MIN_X,
            far: FAR_BLOCK_MAX_X,
        });
        let block = Some(BlockRaycastHit {
            block: BLOCK_ID,
            near: HIT_BOUNDARY_X,
            far: HIT_EXIT_X,
        });

        assert!(matches!(
            compare_entity_block_hits(entity, block),
            Some(RaycastCollision::Block(_))
        ));
    }

    #[test]
    fn voxel_block_shell_returns_nearest_shape() {
        let bounds = VoxelRaycastBounds::new(
            vek::Vec3::new(AXIS_BOUND_MIN, AXIS_BOUND_MIN, AXIS_BOUND_MIN),
            vek::Vec3::new(AXIS_BOUND_MAX, AXIS_BOUND_MIN, AXIS_BOUND_MIN),
        )
        .unwrap();

        let hit = first_block_hit_in_voxels(axis_ray(RAY_DISTANCE), bounds, |voxel| {
            if voxel.position.x == SECOND_BOX_MIN_X as i32 {
                vec![BlockRaycastCandidate {
                    block: BLOCK_ID,
                    aabb: vek::Aabb {
                        min: vek::Vec3::new(SECOND_BOX_MIN_X, UNIT_BOX_MIN, UNIT_BOX_MIN),
                        max: vek::Vec3::new(SECOND_BOX_MAX_X, UNIT_BOX_MAX, UNIT_BOX_MAX),
                    },
                }]
            } else {
                Vec::new()
            }
        })
        .unwrap()
        .unwrap();

        assert_eq!(hit.block, BLOCK_ID);
        assert_eq!(hit.near, SECOND_BOX_MIN_X - ORIGIN_AXIS_X);
    }

    fn axis_ray(max_distance: f64) -> RaycastRay {
        RaycastRay::new(
            origin(),
            vek::Vec3::new(UNIT_BOX_MAX, UNIT_BOX_MIN, UNIT_BOX_MIN),
            max_distance,
        )
        .unwrap()
    }

    fn origin() -> vek::Vec3<f64> {
        vek::Vec3::new(ORIGIN_AXIS_X, ORIGIN_AXIS_Y, ORIGIN_AXIS_Z)
    }

    fn unit_aabb() -> vek::Aabb<f64> {
        vek::Aabb {
            min: vek::Vec3::new(UNIT_BOX_MIN, UNIT_BOX_MIN, UNIT_BOX_MIN),
            max: vek::Vec3::new(UNIT_BOX_MAX, UNIT_BOX_MAX, UNIT_BOX_MAX),
        }
    }

    fn tie_aabb() -> vek::Aabb<f64> {
        vek::Aabb {
            min: vek::Vec3::new(HIT_BOUNDARY_X, NEGATIVE_HALF, NEGATIVE_HALF),
            max: vek::Vec3::new(HIT_EXIT_X, POSITIVE_HALF, POSITIVE_HALF),
        }
    }
}
