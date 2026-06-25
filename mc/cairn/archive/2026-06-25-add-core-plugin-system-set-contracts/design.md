# Design: Add core plugin SystemSet contracts

## Context

Valence exposes Bevy plugin composition as a primary extension point. Some core plugins already publish sets, while others rely on tuple order or local `before`/`after` constraints. A focused pass can make the most useful plugin phases orderable while avoiding unnecessary API surface.

## Decisions

### 1. Inventory before exposing sets

**Choice:** Record current systems, schedules, resources, events, ordering constraints, feature gates, and downstream examples before adding any set names.

**Rationale:** Set names become ordering contracts and should reflect actual stable phases.

### 2. Promote phase-level sets only

**Choice:** Add sets for meaningful phases such as command input/dispatch/tree sync, equipment init/change broadcast, advancement input/cache/client sync, or scoreboard update phases.

**Rationale:** One set per system is noisy and can freeze internals unnecessarily.

### 3. Preserve behavior and membership

**Choice:** Keep event names, resources, feature gates, default plugin group membership, and system semantics unchanged.

**Rationale:** This is schedule API work, not behavior work.

### 4. Document private internals

**Choice:** If a plugin has ordering that should remain private, document that non-claim rather than exposing a set.

**Rationale:** Users need stable boundaries, not every internal detail.

## Risks / Trade-offs

- Public set names increase API surface and require maintenance.
- Reordering bugs can be subtle; schedule hygiene and focused tests are mandatory.
- Some plugins may not warrant public sets yet; inventory should classify them as out of scope.
