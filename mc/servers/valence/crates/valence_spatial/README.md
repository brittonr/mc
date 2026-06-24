# `valence_spatial`

An implementation of a [bounding volume hierarchy](https://en.wikipedia.org/wiki/Bounding_volume_hierarchy) (BVH) for fast spatial queries.

## Raycast and collision helpers

The `raycast` module provides pure helpers for checked ray inputs, AABB intersections, deterministic voxel traversal, owner/self exclusion, nearest block/entity hit selection, and entity-vs-block collision ordering. Callers keep ECS and world access in a thin shell: gather entity hitboxes or block collision-shape AABBs, pass explicit candidates into the pure helpers, and apply the returned hit or tie result.

Invalid ray inputs are rejected before traversal: non-finite origins or directions, zero-length directions, negative or non-finite maximum distances, invalid voxel bounds, and invalid AABBs return `RaycastInputError`. If an entity hit and block hit occur at the same distance, `compare_entity_block_hits` returns a deterministic tie instead of depending on query iteration order.

These helpers describe query mechanics only. They do not claim vanilla combat, projectile damage, reach, public-server safety, production readiness, or Hyperion behavior parity without separate reference evidence.
