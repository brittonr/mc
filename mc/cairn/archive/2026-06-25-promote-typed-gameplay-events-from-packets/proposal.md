# Proposal: Promote typed gameplay events from packet boundaries

## Why

Valence exposes raw `PacketEvent` at the event-loop boundary, which is useful for low-level protocol access. Gameplay and compatibility fixtures often need stable semantics rather than repeated packet decoding in many systems. Promoting selected packet-derived semantics into typed Bevy events can make system ordering clearer, reduce duplicate decode logic, and improve negative malformed-packet behavior.

## What Changes

- Inventory selected systems that read `PacketEvent` directly and decode the same packet types or gameplay semantics.
- Define typed event contracts for selected packet-derived actions, including source client, packet timestamp needs, decode diagnostics, and malformed-data behavior.
- Add packet-boundary adapter systems that read `PacketEvent` once and emit typed domain events in the documented event-loop phase.
- Keep raw `PacketEvent` available for low-level users and unsupported packet types.
- Add positive valid-packet tests and negative wrong-id, partial-decode, malformed-payload, stale-client, and duplicate-emission tests.

## Impact

- **Files**: `servers/valence/crates/valence_server/src/event/loop.rs`, selected interaction/inventory/action systems, examples/fixtures, tests, docs/evidence after implementation.
- **Testing**: typed event adapter tests, malformed packet tests, existing event-loop regressions, selected compatibility rails if fixture input handling changes, Cairn gates, and Cairn validation.
- **Non-claims**: this does not remove direct packet access or require every packet type to get a typed event.
