# Design: Add Hyperion mode markers and run conditions

## Context

Mode-specific behavior should be addressable by ECS facts: marker components for players/entities/resources and run conditions for systems. A global active-mode resource is useful for selected-world identity, but by itself it is a weak isolation boundary.

## Decisions

### 1. Marker-first ownership

**Choice:** Mode plugins insert explicit marker components and mode-owned resources for players, arenas, or state they own.

**Rationale:** Queries and cleanup can then target exact ownership instead of inferring from a global mode flag.

### 2. Run conditions for mode-owned systems

**Choice:** Systems that should run only for a mode use named run conditions or marker-filtered queries.

**Rationale:** Disabled-plugin and wrong-mode behavior becomes testable and visible in schedules.

### 3. Cleanup is part of ownership

**Choice:** Marker-owned temporary state includes cleanup expectations for disconnect, reset, or mode teardown where scoped.

**Rationale:** Composability fails if markers prevent mutation but leave stale state behind.

### 4. Preserve common feature behavior

**Choice:** Shared gameplay mechanics remain mode-neutral unless a mode-specific marker or policy explicitly gates them.

**Rationale:** The goal is isolating mode-specific behavior, not disabling common mechanics unexpectedly.

## Risks / Trade-offs

- Over-gating can suppress expected default gameplay; tests must prove default presets still behave.
- Some existing systems may mix common and mode-specific logic and need splitting.
- Run-condition names should avoid promising stronger ordering than this Cairn changes.
