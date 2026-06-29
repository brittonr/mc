# Proposal: Extract Valence inventory click functional core

## Why

`servers/valence/crates/valence_inventory/src/click.rs` couples Bevy packet-event shells, query access, packet validation, drop behavior, key handling, regular click flow, resync behavior, and emitted click-slot events. Inventory click behavior is compatibility-sensitive and should be testable as a deterministic transaction core without Bevy queries or live clients.

## What Changes

- Extract a pure inventory click transaction core that receives explicit inventory, cursor, open-window, mode, slot, and slot-change summaries and returns a transaction decision.
- Keep Bevy queries, client packet resync sends, event writers, inventory mutations, and drop-item events in thin shells.
- Preserve existing packet validation, invalid resync behavior, cursor/drop semantics, key/drop mode behavior, regular click flow behavior, emitted event shapes, and non-claims.
- Add positive and negative tests for valid clicks, outside-window drops, drop-key paths, invalid packets, resync plans, malformed slot changes, and open-inventory failures.

## Impact

- **Files**: `servers/valence/crates/valence_inventory/src/click.rs`, `click/flow.rs`, `click/press.rs`, validation modules, focused tests, affected Valence inventory checks, and Cairn artifacts.
- **Testing**: baseline Valence inventory tests, positive and negative pure-core tests, affected mc-compat inventory dry-runs, Cairn gates, and Cairn validation.
- **Non-claims**: inventory implementation architecture only; this does not add new inventory semantics or compatibility evidence.
