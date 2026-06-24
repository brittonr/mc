# Design: Unify spatial raycast and collision helpers

## Context

Valence's `valence_spatial` exposes BVH traversal and ray-box intersection. Hyperion's gameplay code combines entity spatial queries with block ray traversal to decide which collision occurs first. The useful integration is a stable API shape and tested math behavior, not a direct copy of Hyperion internals.

## Decisions

### 1. Use Valence-owned types

**Choice:** Prefer existing Valence math/spatial types and adapters over introducing Hyperion geometry types into the public API.

**Rationale:** This keeps the API coherent for Valence users.

### 2. Make collision ordering deterministic

**Choice:** Define how entity hits and block hits are ordered, including ties and starts inside hitboxes.

**Rationale:** Gameplay code should not invent conflicting rules for projectiles, interactions, and reach checks.

### 3. Keep math core pure

**Choice:** Ray traversal, hitbox intersection, and collision comparison are pure functions over positions, shapes, and query results. ECS queries only gather inputs and apply outputs.

**Rationale:** Edge cases can be tested without a world or network.

### 4. Reject invalid ray inputs explicitly

**Choice:** NaN vectors, zero-length directions, invalid bounds, and out-of-range query settings return structured errors or no-hit results according to the documented contract.

**Rationale:** Silent floating-point surprises become gameplay bugs.

## Risks / Trade-offs

- Exact vanilla collision semantics are broader than this API; keep parity claims separate.
- Switching public math types later can be breaking; start with adapters and limited surface area.
- Performance optimizations may need unsafe code; require separate audits before adoption.
