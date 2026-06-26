# Design: Promote remaining packet semantics to typed Bevy events

## Context

Raw `PacketEvent` is the correct low-level protocol boundary, but packet decoding inside multiple gameplay systems makes schedule ownership, malformed-input behavior, and fixture compatibility harder to review. `PlayerActionEvent` shows the desired shape: a thin adapter decodes once in the event-loop phase, emits a domain event, and leaves policy in downstream systems.

## Decisions

### 1. Inventory before promotion

**Choice:** Record selected raw packet readers, packet types, decoded fields, current diagnostics, downstream mutations, and compatibility fixture milestones before adding events.

**Rationale:** Typed events should be added where they clarify stable semantics rather than hiding useful protocol details.

### 2. Adapter systems own decoding

**Choice:** Each promoted semantic gets an adapter system that reads `PacketEvent`, validates the packet ID/body and live client when required, and emits one typed event or documented diagnostic behavior.

**Rationale:** Downstream systems should not duplicate packet-body decoding or accidentally diverge on malformed input.

### 3. Event ownership is explicit

**Choice:** The contract names which systems own each typed event and which code paths may continue using raw packets.

**Rationale:** Raw and typed events can coexist only if double handling is visible and testable.

### 4. Compatibility fixtures migrate last

**Choice:** Core typed events land before fixture systems move away from raw packet readers.

**Rationale:** Fixture milestones must remain comparable, and migration should be reversible if the event contract exposes a compatibility gap.

## Risks / Trade-offs

- Event timing can shift behavior; adapters must stay in existing event-loop phases and schedule hygiene must cover ordering changes.
- Typed events can obscure low-level packet details; raw access remains public and documented.
- Migrations can double-handle an action if raw consumers are not audited; ownership and negative duplicate-emission tests mitigate this.
