# Proposal: Split Valence observability core

## Why

`servers/valence/crates/valence_server/src/observability.rs` combines metric names, labels, redaction, packet classification, export decisions, Bevy event emission, and tests. Observability is a safety/evidence-adjacent surface, so classification and redaction decisions should be pure and separate from Bevy/export shells.

## What Changes

- Split observability into modules for config, taxonomy, labels/redaction, packet classification, export planning, and Bevy event shells.
- Extract pure metric/label/redaction/classification/export decisions.
- Keep Bevy plugin wiring, event writing, exporter calls, and logging in shells.
- Preserve metric names, labels, redaction policy, packet classification, export outcomes, schedule behavior, and non-claims.

## Impact

- **Files**: `servers/valence/crates/valence_server/src/observability.rs`, observability submodules, focused tests, affected schedule checks, and Cairn artifacts.
- **Testing**: baseline observability/server tests, positive and negative observability-core tests, schedule checks where needed, Cairn gates, and Cairn validation.
- **Non-claims**: observability architecture only; no new telemetry/evidence claim is promoted.
