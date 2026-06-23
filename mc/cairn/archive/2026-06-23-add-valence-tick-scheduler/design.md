# Design: Add a Valence tick scheduler

## Context

Hyperion's `Scheduled<K, V>` is a minimal min-heap over ordered keys. Valence can adopt the concept as a stable utility for tick-driven gameplay without inheriting Hyperion-specific runtime assumptions.

## Decisions

### 1. Tick keys are explicit inputs

**Choice:** The scheduler core receives explicit ordered tick keys and a drain limit from callers. It does not read clocks or global tick resources.

**Rationale:** Deterministic tests should exercise scheduling without a Bevy app or runtime.

### 2. Pure core, ECS shell

**Choice:** The core owns queue operations. A plugin or helper system reads the Valence tick resource and drains due work.

**Rationale:** Gameplay policy remains outside the data structure.

### 3. Deterministic equal-key behavior

**Choice:** Equal-key ordering is documented and tested, either as stable insertion order or explicitly arbitrary but deterministic within one queue implementation.

**Rationale:** Same-tick events are common and should not surprise plugin authors.

### 4. Cancellation is explicit or deferred

**Choice:** The first implementation either includes explicit cancellation handles or documents cancellation as a non-goal with clear clear/drain behavior.

**Rationale:** Hidden cancellation semantics complicate simple scheduler usage.

## Risks / Trade-offs

- A generic scheduler can become a task runtime; keep scope to tick-keyed data.
- Stable equal-key ordering can require extra sequence data; justify with tests if added.
- Different plugins may need different policies; keep the core small and composable.
