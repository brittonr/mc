use bevy_ecs::prelude::Entity;
use valence_protocol::{BlockPos, ChunkPos};

use crate::ChunkView;

const SQUARED_DISTANCE_EXPONENT: u32 = 2;
const PRE_CLIENT_STEP_COUNT: usize = 2;
const POST_CLIENT_STEP_COUNT: usize = 1;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct RadiusSquared(u32);

impl RadiusSquared {
    const fn from_squared(radius_squared: u32) -> Self {
        Self(radius_squared)
    }

    const fn get(self) -> u32 {
        self.0
    }

    #[cfg(test)]
    fn from_radius(radius: i64) -> Option<Self> {
        if radius < 0 {
            return None;
        }

        let radius = u64::try_from(radius).ok()?;
        let radius_squared = radius.checked_mul(radius)?;
        let radius_squared = u32::try_from(radius_squared).ok()?;

        Some(Self::from_squared(radius_squared))
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub(super) enum ChunkLayerUpdatePhase {
    PreClient,
    PostClient,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub(super) enum ChunkLayerUpdateStep {
    UpdateLoadedChunks,
    ReadyMessages,
    UnreadyMessages,
}

const PRE_CLIENT_UPDATE_PLAN: [ChunkLayerUpdateStep; PRE_CLIENT_STEP_COUNT] = [
    ChunkLayerUpdateStep::UpdateLoadedChunks,
    ChunkLayerUpdateStep::ReadyMessages,
];
const POST_CLIENT_UPDATE_PLAN: [ChunkLayerUpdateStep; POST_CLIENT_STEP_COUNT] =
    [ChunkLayerUpdateStep::UnreadyMessages];

pub(crate) fn client_is_not_excluded(client: Entity, except: Entity) -> bool {
    client != except
}

pub(crate) fn view_contains_chunk(view: ChunkView, pos: ChunkPos) -> bool {
    view.contains(pos)
}

pub(crate) fn block_pos_within_radius_squared(
    client_pos: BlockPos,
    center: BlockPos,
    radius_squared: u32,
) -> bool {
    let radius_squared = RadiusSquared::from_squared(radius_squared);
    let dist_squared = (center.x - client_pos.x).pow(SQUARED_DISTANCE_EXPONENT)
        + (center.y - client_pos.y).pow(SQUARED_DISTANCE_EXPONENT)
        + (center.z - client_pos.z).pow(SQUARED_DISTANCE_EXPONENT);

    dist_squared as u32 <= radius_squared.get()
}

pub(super) fn chunk_layer_update_plan(
    phase: ChunkLayerUpdatePhase,
) -> &'static [ChunkLayerUpdateStep] {
    match phase {
        ChunkLayerUpdatePhase::PreClient => &PRE_CLIENT_UPDATE_PLAN,
        ChunkLayerUpdatePhase::PostClient => &POST_CLIENT_UPDATE_PLAN,
    }
}

#[cfg(test)]
fn chunk_list_contains_chunk<I>(chunks: I, pos: ChunkPos) -> bool
where
    I: IntoIterator<Item = ChunkPos>,
{
    chunks.into_iter().any(|chunk| chunk == pos)
}

#[cfg(test)]
fn update_plan_is_valid(phase: ChunkLayerUpdatePhase, plan: &[ChunkLayerUpdateStep]) -> bool {
    plan == chunk_layer_update_plan(phase)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INCLUDED_ENTITY_INDEX: u32 = 1;
    const EXCLUDED_ENTITY_INDEX: u32 = 2;
    const VIEW_CENTER_X: i32 = 8;
    const VIEW_CENTER_Z: i32 = -8;
    const VIEW_DISTANCE: u8 = 2;
    const FAR_CHUNK_X: i32 = 64;
    const FAR_CHUNK_Z: i32 = -64;
    const CENTER_BLOCK_X: i32 = 16;
    const CENTER_BLOCK_Y: i32 = 80;
    const CENTER_BLOCK_Z: i32 = -16;
    const NEAR_BLOCK_X: i32 = 17;
    const INSIDE_RADIUS_SQUARED: u32 = 1;
    const OUTSIDE_RADIUS_SQUARED: u32 = 0;
    const VALID_RADIUS: i64 = 8;
    const VALID_RADIUS_SQUARED: u32 = 64;
    const INVALID_NEGATIVE_RADIUS: i64 = -1;
    const OVERFLOWING_RADIUS: i64 = i64::MAX;
    const EMPTY_VIEW_CHUNK_COUNT: usize = 0;

    fn included_entity() -> Entity {
        Entity::from_raw(INCLUDED_ENTITY_INDEX)
    }

    fn excluded_entity() -> Entity {
        Entity::from_raw(EXCLUDED_ENTITY_INDEX)
    }

    fn center_chunk() -> ChunkPos {
        ChunkPos::new(VIEW_CENTER_X, VIEW_CENTER_Z)
    }

    fn far_chunk() -> ChunkPos {
        ChunkPos::new(FAR_CHUNK_X, FAR_CHUNK_Z)
    }

    fn center_block() -> BlockPos {
        BlockPos::new(CENTER_BLOCK_X, CENTER_BLOCK_Y, CENTER_BLOCK_Z)
    }

    #[test]
    fn view_membership_accepts_chunks_inside_the_view() {
        let view = ChunkView::new(center_chunk(), VIEW_DISTANCE);

        assert!(view_contains_chunk(view, center_chunk()));
    }

    #[test]
    fn view_membership_rejects_chunks_outside_or_empty_views() {
        let view = ChunkView::new(center_chunk(), VIEW_DISTANCE);
        let empty_view: [ChunkPos; EMPTY_VIEW_CHUNK_COUNT] = [];

        assert!(!view_contains_chunk(view, far_chunk()));
        assert!(!chunk_list_contains_chunk(empty_view, center_chunk()));
    }

    #[test]
    fn radius_targeting_accepts_clients_inside_the_radius() {
        assert!(block_pos_within_radius_squared(
            center_block(),
            center_block(),
            INSIDE_RADIUS_SQUARED,
        ));
        assert!(block_pos_within_radius_squared(
            BlockPos::new(NEAR_BLOCK_X, CENTER_BLOCK_Y, CENTER_BLOCK_Z),
            center_block(),
            INSIDE_RADIUS_SQUARED,
        ));
    }

    #[test]
    fn radius_targeting_rejects_out_of_range_and_invalid_radii() {
        assert!(!block_pos_within_radius_squared(
            BlockPos::new(NEAR_BLOCK_X, CENTER_BLOCK_Y, CENTER_BLOCK_Z),
            center_block(),
            OUTSIDE_RADIUS_SQUARED,
        ));
        assert_eq!(
            RadiusSquared::from_radius(VALID_RADIUS).map(RadiusSquared::get),
            Some(VALID_RADIUS_SQUARED)
        );
        assert_eq!(RadiusSquared::from_radius(INVALID_NEGATIVE_RADIUS), None);
        assert_eq!(RadiusSquared::from_radius(OVERFLOWING_RADIUS), None);
    }

    #[test]
    fn exception_filter_allows_non_excluded_clients_only() {
        assert!(client_is_not_excluded(included_entity(), excluded_entity()));
        assert!(!client_is_not_excluded(
            excluded_entity(),
            excluded_entity()
        ));
    }

    #[test]
    fn update_plans_preserve_pre_and_post_client_order() {
        assert_eq!(
            chunk_layer_update_plan(ChunkLayerUpdatePhase::PreClient),
            PRE_CLIENT_UPDATE_PLAN
        );
        assert_eq!(
            chunk_layer_update_plan(ChunkLayerUpdatePhase::PostClient),
            POST_CLIENT_UPDATE_PLAN
        );
    }

    #[test]
    fn update_plans_reject_invalid_order_assumptions() {
        const INVALID_PRE_CLIENT_PLAN: [ChunkLayerUpdateStep; PRE_CLIENT_STEP_COUNT] = [
            ChunkLayerUpdateStep::ReadyMessages,
            ChunkLayerUpdateStep::UpdateLoadedChunks,
        ];

        assert!(!update_plan_is_valid(
            ChunkLayerUpdatePhase::PreClient,
            &INVALID_PRE_CLIENT_PLAN,
        ));
    }
}
