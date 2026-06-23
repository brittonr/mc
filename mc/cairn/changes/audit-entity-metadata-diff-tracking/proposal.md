# Proposal: Audit entity metadata diff tracking

## Why

Hyperion tracks metadata components and encodes changed values into compact metadata updates. Valence already has entity metadata support, but a focused audit could identify cleaner diff tracking, fewer redundant updates, and better tests around default metadata, despawns, and packet ordering.

## What Changes

- Compare Hyperion metadata tracking with Valence entity metadata update flow.
- Define invariants for default metadata, changed metadata, same-tick mutations, spawn packets, despawn cleanup, and encoded update ordering.
- Implement improvements only where the audit shows Valence gaps or measurable duplication.
- Add positive and negative tests for unchanged metadata, changed metadata, multiple changes per tick, default suppression, despawn, invalid metadata indices, and client-visible packet parity.
- Record non-claims around entity behavior and vanilla parity.

## Impact

- **Files**: `valence_entity`, `valence_server` entity update code, metadata tests/fixtures, docs or notes, and Cairn artifacts.
- **Testing**: metadata unit tests, packet encode fixtures, client-visible update regressions, selected entity mc-compat scenarios, and Cairn gates/validation.
- **Non-claims**: this does not add new entity types or claim full vanilla entity behavior parity.
