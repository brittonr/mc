# Proposal: Valence status response resource

## Why

Hyperion recently made server ping response data resource-owned. Valence has status handling, but compatibility tests would be more deterministic if MOTD/version/sample/favicon-style response data were a configurable resource with focused tests.

## What Changes

- Introduce or expose a Valence status-response resource used by status ping handling.
- Allow examples/tests to configure MOTD/version/player sample fields deterministically.
- Add tests proving default and configured status responses.
- Extend smoke receipts or status probes to assert configured response data when used.

## Impact

- **Files**: `valence/crates/valence_server/src/status.rs`, public exports if needed, example setup, and focused status tests.
- **Testing**: Valence unit/integration tests for default/configured status response plus existing status-only probe receipt checks.
