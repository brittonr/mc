# Proposal: Promote remaining packet semantics to typed Bevy events

## Why

Valence already benefits from typed Bevy events for player actions, but several core and fixture paths still read raw `PacketEvent` values and decode packets inside gameplay systems. Inventory hotbar/click/creative actions, command execution, GUI/container close handling, and selected survival compatibility adapters would be easier to audit if packet decoding happened once at the event-loop boundary and downstream systems consumed typed semantics.

## What Changes

- Inventory selected remaining `PacketEvent` consumers in Valence core crates and compatibility examples, including decoded packet types, schedule phases, mutation targets, and malformed-input behavior.
- Define typed event ownership contracts for selected packet semantics, including source client, timestamp needs, decoded fields, stale-client behavior, and raw-packet compatibility.
- Add adapter systems that decode selected packets once and emit typed Bevy events in documented event-loop phases.
- Migrate selected downstream gameplay or fixture systems to consume typed events while keeping direct `PacketEvent` available for low-level users.
- Add positive valid-packet tests and negative wrong-id, malformed-payload, partial-decode, stale-client, duplicate-emission, and raw-access regression tests.

## Impact

- **Files**: `servers/valence/crates/valence_inventory/src/`, `servers/valence/crates/valence_command/src/`, selected `servers/valence/crates/valence_server/src/` packet adapters, `servers/valence/examples/survival_compat.rs`, tests, and promoted evidence under `docs/evidence/`.
- **Testing**: focused typed-event adapter tests, inventory/command tests, survival fixture tests when touched, Valence schedule hygiene when event-loop registrations change, Cairn gates, Cairn validation, and task-evidence validation.
- **Non-claims**: this does not remove raw `PacketEvent`, promote every packet type, change public low-level protocol access, or claim broad Minecraft compatibility.
