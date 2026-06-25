# Design: Replace entity-keyed state with components where appropriate

## Context

Bevy works best when state that belongs to an entity is stored on that entity. Valence examples already use components for many player and world concepts, but compatibility fixture shells still have resource maps for some entity-owned state. A targeted migration can reduce stale-state hazards without forcing all state into ECS.

## Decisions

### 1. Classify before moving

**Choice:** Inventory every targeted entity-keyed or player-keyed collection and classify it as component state, global resource, pure core state, index/cache, or intentionally external identity state.

**Rationale:** Some maps are correct because they index teams, portals, layers, or external usernames rather than entity-owned mutable state.

### 2. Components own entity lifecycle state

**Choice:** State that should disappear or change with an entity moves to a component or bundle attached to that entity.

**Rationale:** Bevy then owns cleanup and query visibility, and tests can assert lifecycle behavior directly.

### 3. Resources remain for global policy and indexes

**Choice:** Resources remain appropriate for global policy, schedule queues, registries, team/layer maps, and caches whose lifecycle is not one ECS entity.

**Rationale:** Moving global state into singleton entities would obscure ownership and complicate APIs.

### 4. Pure cores stay Bevy-free

**Choice:** Deterministic fixture cores continue to accept snapshots and return decisions. ECS adapters translate components/resources into those snapshots.

**Rationale:** This preserves functional-core testability.

## Risks / Trade-offs

- Component migrations can alter query ordering; tests must avoid relying on unspecified Bevy iteration order.
- Reconnect behavior may use stable usernames or UUIDs intentionally; keep external identity state explicit.
- Resource indexes may still need cleanup systems; document any index that mirrors components.
