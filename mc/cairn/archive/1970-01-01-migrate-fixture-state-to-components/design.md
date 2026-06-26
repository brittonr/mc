# Design: Migrate selected fixture state to components

## Context

Valence compatibility fixtures often model specific client or entity lifecycles. When that state is stored globally, systems need explicit cleanup and stale-key checks. Bevy components can encode ownership directly when the lifecycle belongs to an ECS entity. Not all state should move: team maps, layer registries, global fixture policy, pure-core accumulators, and external identity indexes can remain resources when documented.

## Decisions

### 1. Classify state before migration

**Choice:** Each candidate records key space, owner, lifecycle, cleanup behavior, consumers, and stale-reference risk.

**Rationale:** Component migration is only correct when the ECS entity owns the state lifecycle.

### 2. Components own entity-lifecycle state

**Choice:** State attached to a live client, container, mob, drop, clone, or visual companion migrates to components or bundles where feasible.

**Rationale:** Components naturally disappear with entities and make queries express ownership.

### 3. Resources remain for true globals

**Choice:** Team/layer maps, global fixture policy, registries, pure-core state, and external identity indexes remain resources with documented cleanup or indexing rationale.

**Rationale:** Forcing all state into components would harm clarity.

### 4. Compatibility behavior is preserved

**Choice:** Fixture milestones and evidence boundaries remain stable unless another Cairn changes them.

**Rationale:** State migration should not widen compatibility claims.

## Risks / Trade-offs

- Component migration can change initialization timing; tests must cover added/reconnected entities.
- Some resource maps are intentional indexes and should not be removed blindly.
- Queries can become more complex; use bundles and focused helper functions to keep systems readable.
