# Design: Migrate anticheat statistics to components

## Context

The optional anticheat statistics plugin samples packet and movement activity into per-player metric windows. The current resource owns a map from `Entity` to `PlayerAnticheatStatistics`, which requires explicit stale-key hygiene. Because each metric window belongs to one live client entity, component ownership is a better ECS fit. The plugin-local tick remains global state and can stay in a resource.

## Decisions

### 1. Separate entity state from global state

**Choice:** Per-player metric windows migrate to a component. Plugin-local tick/configuration remains a resource.

**Rationale:** Component state follows entity lifecycle, while tick/config state is global plugin policy.

### 2. Sampling systems query components

**Choice:** Packet and movement samples update a client-owned statistics component after validating the event's client entity still has the component.

**Rationale:** Stale events should fail closed without map entries surviving despawn or reconnect boundaries.

### 3. Plugin remains advisory and opt-in

**Choice:** The plugin continues to emit observations only when explicitly added and does not enforce movement, packet cadence, or rotation policy.

**Rationale:** Storage migration should not change product behavior or safety claims.

### 4. Lifecycle tests cover stale ownership

**Choice:** Tests cover added clients, despawned clients, reconnect-like new entities, duplicate component ownership, and disabled plugin behavior.

**Rationale:** The value of component migration is lifecycle correctness, not just API shape.

## Risks / Trade-offs

- Component migration can change when state is initialized; tests must cover first-sample behavior.
- Event readers can observe events for stale clients; systems must validate component presence before mutation.
- Public accessors on the resource may need compatibility shims or a documented migration path.
