# Proposal: Add event-loop phase SystemSets

## Why

Valence has event-loop schedules for raw packet processing and game-facing event handling, but many adapter systems register directly into `EventLoopPreUpdate`, `EventLoopUpdate`, or `EventLoopPostUpdate`. As typed packet events and diagnostics grow, named phase sets inside those schedules would make ordering around raw packet observation, typed adapter emission, domain consumption, diagnostics, and cleanup more explicit.

## What Changes

- Inventory current event-loop schedules, packet event production, adapter systems, diagnostics, typed event consumers, and schedule-impacting checks.
- Define named event-loop phase `SystemSet`s for raw packet observation, typed adapter emission, domain consumption, diagnostics, and cleanup where those phases exist.
- Move selected adapter and diagnostic systems into named sets while preserving packet/event semantics and raw access.
- Document ordering boundaries that remain private.
- Add positive schedule tests and negative missing-set, ambiguity, duplicate adapter, raw-access, and disabled-plugin tests.

## Impact

- **Files**: `servers/valence/crates/valence_server/src/event/loop.rs`, selected packet adapter modules, observability/anticheat systems if scoped, schedule hygiene evidence under `docs/evidence/`.
- **Testing**: focused event-loop tests, typed event tests where touched, Valence schedule hygiene, selected compatibility rails when fixture input handling changes, Cairn gates, Cairn validation, and task-evidence validation.
- **Non-claims**: this does not remove raw `PacketEvent`, change networking transport, or require every packet semantic to become typed.
