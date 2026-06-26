# Design: Adopt Bevy lifecycle cleanup patterns

## Context

Bevy ECS gives several lifecycle tools: components can own state, queries can target changed/added/removed state, and cleanup systems can run in named schedule sets. Valence also has explicit lifecycle requirements around `Despawned` entities, so cleanup must be classified rather than blindly replaced.

## Decisions

### 1. Cleanup triggers are inventoried

**Choice:** Each targeted cleanup records owner, trigger, current schedule phase, stale-state risk, and whether cleanup must occur before Valence despawn finalization.

**Rationale:** Cleanup timing is correctness-critical.

### 2. Use component lifecycle where ownership is real

**Choice:** State owned by an entity should be removed by component/entity lifecycle or a lifecycle-specific cleanup system.

**Rationale:** This reduces stale indexes and makes ownership queryable.

### 3. Preserve explicit Valence despawn semantics

**Choice:** Cleanup that depends on `Despawned` marker timing stays explicit and gets named cleanup sets where useful.

**Rationale:** Valence needs deinitialization windows before final entity removal.

### 4. Resource/index cleanup is documented

**Choice:** Indexes and external I/O state that remain resources must document their cleanup triggers and stale-entry tests.

**Rationale:** Not all cleanup is component-owned.

## Risks / Trade-offs

- Misclassifying cleanup can delete state too early or too late; negative lifecycle tests are required.
- Bevy removal detection may not fit all Valence despawn timing; explicit `Despawned` cleanup remains valid.
- Named cleanup sets change schedule shape and require schedule hygiene evidence.
