# Design: Valence inventory click transaction core

## Context

Valence inventory click handling already has helper modules for flow and press behavior, but the top-level handler still mixes Bevy access, validation, mutation, and event emission. A pure transaction core can make inventory semantics easier to test and safer to reuse in compatibility fixtures.

## Decisions

### 1. Introduce explicit transaction inputs and decisions

**Choice:** Define core input summaries for client inventory, optional open inventory, cursor item, click mode, slot index, and slot changes. The core returns decisions such as apply transaction, drop cursor, resync invalid, emit click event, or ignore.

**Rationale:** Inventory click behavior can be tested without Bevy ECS queries.

### 2. Keep mutation in shells

**Choice:** The pure core computes the plan. Existing shells apply inventory mutations, send packet resyncs, write events, and emit drop-item events.

**Rationale:** Side effects remain localized and reviewable.

### 3. Reuse existing validators

**Choice:** Existing packet validation logic remains authoritative but should be callable from the transaction core through explicit inputs.

**Rationale:** The refactor should not rewrite semantics and validation at the same time.

### 4. Preserve event and resync semantics

**Choice:** Emitted `ClickSlotEvent` shapes, invalid resync behavior, outside-window drop behavior, and drop-key behavior remain stable.

**Rationale:** Compatibility rails consume inventory behavior and evidence markers.

## Risks / Trade-offs

- Borrow lifetimes around Bevy queries may require adapter structs; keep adapters thin and test the pure core separately.
- Existing `flow` and `press` modules may already mutate state; extract planning incrementally before changing mutation paths.
- Invalid packet handling is safety relevant; add negative tests before moving shells.
