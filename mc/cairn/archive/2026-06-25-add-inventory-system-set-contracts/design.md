# Design: Add inventory SystemSet contracts

## Context

Valence already uses named sets for several schedule-sensitive systems and examples. Inventory is a core plugin with multiple event-loop and post-update systems, but its ordering is currently expressed as tuple composition plus local `before` and `after` constraints. Named sets would expose stable composition points while preserving the existing functional core and imperative ECS shell split.

## Decisions

### 1. Inventory existing schedule shape first

**Choice:** Record current schedules, tuple groups, ordering constraints, events, resources, and default plugin inclusion before refactoring.

**Rationale:** Named sets are schedule-impacting and need a baseline to avoid accidental reordering.

### 2. Sets describe phases, not policy

**Choice:** Define sets around packet input, inventory model mutation, viewer/window synchronization, presentation or flush preparation, and cleanup where the current plugin has that behavior.

**Rationale:** Sets should help user plugins order around inventory without moving inventory rules into global policy.

### 3. Preserve default behavior

**Choice:** Keep `InventoryPlugin` default membership, feature gates, event names, and packet semantics unchanged.

**Rationale:** This is an orchestration cleanup, not an inventory behavior change.

### 4. Tests assert both presence and absence

**Choice:** Add positive tests proving the plugin installs expected sets/events and negative tests proving disabled plugin configurations do not install inventory-owned behavior.

**Rationale:** Schedule contracts are only useful if regressions fail clearly.

## Risks / Trade-offs

- Public set names become part of the ordering surface and should be minimal.
- Reordering can be subtle; schedule hygiene and focused inventory tests are required.
- Overly granular sets can make the API noisy; prefer phase-level sets over one set per system.
