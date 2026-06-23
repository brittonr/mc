# Design: Add an optional GUI helper plugin

## Context

Hyperion's GUI helper spawns inventory-backed entities and associates slots with callbacks. Valence's inventory crate already models inventory windows, cursor state, click handling, readonly slots, and client synchronization. A helper should sit above Valence inventory rather than reimplementing it.

## Decisions

### 1. Build on Valence inventory

**Choice:** The GUI helper composes existing `valence_inventory` windows, slots, and events.

**Rationale:** Inventory packet semantics should remain in one crate.

### 2. Model click outcomes explicitly

**Choice:** Slot click handling returns explicit actions or events rather than arbitrary hidden state mutation in callbacks.

**Rationale:** GUI behavior becomes testable and easier to reason about.

### 3. Lifecycle cleanup is required

**Choice:** Close events, disconnects, despawns, and stale window IDs must clean up viewer state and reject late clicks.

**Rationale:** GUI helpers otherwise create common desync bugs.

### 4. Optional ergonomic layer

**Choice:** The helper is optional and examples-focused; lower-level inventory APIs remain available.

**Rationale:** Advanced users should not be forced through menu abstractions.

## Risks / Trade-offs

- Callback ergonomics can hide side effects; mitigate with explicit actions and docs.
- Inventory semantics are broad; keep vanilla parity claims in inventory-specific Cairns.
- Helper APIs can ossify too early; start narrow with common menu flows.
