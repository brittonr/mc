# Design: Valence command manager modules

## Context

Command manager code sits at the boundary between client packets, command graphs, Bevy events, and application handlers. The split should make parse/routing decisions explicit without changing public command APIs.

## Decisions

### 1. Split packet, tree, parse, and event concerns

**Choice:** Create modules for packet adapters, command tree sync, parse core, execution event planning, plugin wiring, and Bevy systems.

**Rationale:** Each concern is independently testable and schedule-sensitive in different ways.

### 2. Extract pure command decisions

**Choice:** Packet-to-command events, tree update requirements, parse outcomes, argument plans, and processed-event plans should be pure over explicit inputs.

**Rationale:** Command behavior can be tested without a Bevy app.

### 3. Preserve schedule and events

**Choice:** Existing event types, event ordering, command tree packets, and plugin behavior remain stable.

**Rationale:** Downstream command handlers depend on these contracts.
