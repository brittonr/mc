# Design: Promote typed gameplay events from packet boundaries

## Context

`PacketEvent` is the right low-level boundary between networking and game logic, but packet decoding scattered across gameplay systems makes behavior harder to audit. Valence already has typed events for several interactions; this change targets remaining repeated or fixture-critical raw packet decoding.

## Decisions

### 1. Inventory direct packet consumers

**Choice:** Record selected `PacketEvent` readers, decoded packet types, emitted semantics, duplicate decode paths, and error handling.

**Rationale:** Typed events should be introduced where they reduce duplication or clarify gameplay semantics.

### 2. Adapter systems own decoding

**Choice:** Packet adapter systems read raw events in a documented event-loop phase, decode selected packet types, and emit typed events or diagnostics.

**Rationale:** Gameplay systems should consume domain events and stay independent from raw packet bytes when possible.

### 3. Raw packet access remains public

**Choice:** `PacketEvent` stays available for low-level users, experimental plugins, and unsupported packets.

**Rationale:** Valence's flexibility includes direct protocol access.

### 4. Malformed input fails closed

**Choice:** Wrong packet IDs, partial decodes, malformed payloads, and stale clients produce no typed action or an explicit diagnostic according to the event contract.

**Rationale:** Typed events should prevent duplicated error-prone decode handling.

## Risks / Trade-offs

- Typed events can hide protocol detail needed by advanced users; keep raw access intact.
- Event timing can shift behavior; schedule adapters in existing event-loop phases and test ordering.
- Emitting both raw and typed events can double-handle actions if systems are not migrated carefully; document ownership per semantic.
