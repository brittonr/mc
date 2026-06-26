# Design: Scope gameplay plugins to arenas and layers

## Context

A plugin can be installed once while many gameplay instances exist at runtime. CTF and survival compatibility currently demonstrate the plugin shell pattern, but they are not yet modeled as arena-owned instances. Composition requires a separate runtime scope seam: install plugin code once, spawn scoped arenas many times, and ensure systems mutate only entities in their owned scope.

## Decisions

### 1. Install plugins once, spawn arenas many times

**Choice:** Treat `CtfGameplayPlugin` and `SurvivalCompatibilityPlugin` as system-registration plugins, not per-arena instances. Runtime instances are arena/scope entities or layer-owned components.

**Rationale:** Bevy plugins are a wiring mechanism. Runtime multiplicity belongs in ECS data.

### 2. Scope state by arena or layer owner

**Choice:** Move per-mode mutable state that can vary by arena into arena-owned components/resources referenced by an explicit scope. Global resources may remain only for truly global policy or defaults.

**Rationale:** One global game-mode resource prevents two CTF arenas, CTF plus survival, or independent layers from coexisting predictably.

### 3. Make systems fail closed on missing or mismatched scope

**Choice:** Gameplay systems check scope membership before reading events or mutating clients, entities, layers, scores, flags, containers, or milestone state.

**Rationale:** Cross-layer mutation is the main composability hazard.

### 4. Scope observable events and milestones

**Choice:** Gameplay events and compatibility milestones include arena or scope identity when the same event type can arise from multiple arenas.

**Rationale:** Receipts and downstream systems need to distinguish same-mode and mixed-mode activity in one app.

## Risks / Trade-offs

- Arena scoping touches many queries and event payloads; focused tests must distinguish behavior preservation from intentional metadata additions.
- Some current fixture milestones may need compatibility adapters to keep old receipts comparable.
- Over-scoping can add ceremony to tiny examples; this change targets CTF and survival compatibility first.
