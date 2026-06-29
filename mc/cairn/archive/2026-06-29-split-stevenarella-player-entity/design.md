# Design: Stevenarella player entity split

## Context

Player entity code mixes pure state decisions with ECS and render side effects. The split should preserve public player APIs while making movement, collision, and model decisions independently testable.

## Decisions

### 1. Split by player concern

**Choice:** Extract construction, model state, rendering shell, movement state, collision core, and ECS system wiring into separate modules.

**Rationale:** These concerns change independently and need focused fixtures.

### 2. Keep pure decisions small

**Choice:** Movement updates, collision checks, model-part visibility, and local/remote player state transitions should be pure over explicit summaries.

**Rationale:** Tests should not require a renderer or live ECS manager.

### 3. Preserve shell behavior

**Choice:** ECS mutation, renderer calls, resource access, and network-facing behavior remain in shells.

**Rationale:** Refactoring must not change visible client behavior.
