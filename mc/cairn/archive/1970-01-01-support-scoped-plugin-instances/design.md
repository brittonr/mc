# Design: Support scoped plugin instances

## Context

Valence examples use `GameplayScope`, `GameplayArenaId`, and shared contract metadata, but several defaults are static primary IDs and plugin contracts are keyed by plugin name. A plugin can be installed once and understood once, but multiple arenas or multiple instances need a stronger identity boundary.

## Decisions

### 1. Introduce explicit plugin instance identity

**Choice:** Model a plugin instance as a stable scope identity composed from gameplay mode, arena/layer ownership, and plugin role where needed.

**Rationale:** Plugin name alone cannot distinguish two CTF arenas or two survival fixture instances.

### 2. Scope mutable gameplay state by owner

**Choice:** State that can vary by arena or plugin instance should move to arena entities, layer-owned components, or explicit instance-keyed resources. Remaining globals must document why they are policy/default/registry state.

**Rationale:** This prevents cross-arena mutation and makes ownership reviewable.

### 3. Preserve compatibility through adapters

**Choice:** Existing single-primary fixture entrypoints can continue using default instance IDs and adapters that emit the same legacy milestone text where required.

**Rationale:** mc-compat receipts should stay comparable unless a separate change broadens receipt schema.

### 4. Make wrong-scope behavior fail closed

**Choice:** Systems should ignore, clean up, or diagnose missing, stale, wrong-mode, and wrong-arena scope inputs without panics or cross-scope mutation.

**Rationale:** Scoped plugins are only safe if invalid scope data cannot mutate unrelated instances.

## Risks / Trade-offs

- Moving global resources to instance-owned state can be invasive; start with selected CTF/survival/terrain paths that already have scope helpers.
- Event payload changes may affect compatibility receipts; adapters or explicit evidence non-claims are required.
- Multiple instances may need deterministic IDs in tests; use named constants and typed wrappers rather than raw strings scattered through systems.
