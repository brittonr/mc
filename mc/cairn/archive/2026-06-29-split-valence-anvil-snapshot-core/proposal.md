# Proposal: Split Valence Anvil snapshot core

## Why

`servers/valence/crates/valence_anvil/src/snapshot.rs` combines snapshot data modeling, parsing assumptions, region/chunk lookup behavior, cache or directory interactions, and tests. Anvil loading is format-sensitive and benefits from a pure snapshot plan core with filesystem and compression side effects isolated.

## What Changes

- Split snapshot code into focused modules for snapshot model, region/chunk lookup planning, parsing/validation, cache policy, directory/filesystem shell, and Bevy integration adapters.
- Extract pure decisions for region coordinates, chunk selection, missing-chunk behavior, parse validation, and snapshot update plans.
- Keep filesystem reads, compression/decompression, directory traversal, Bevy asset/resource integration, and logging in shells.
- Preserve public APIs, Anvil format behavior, missing/corrupt region behavior, cache behavior, and non-claims.

## Impact

- **Files**: `servers/valence/crates/valence_anvil/src/snapshot.rs`, related anvil modules, focused tests, and Cairn artifacts.
- **Testing**: baseline Valence Anvil tests, positive and negative snapshot-core tests, affected workspace checks, Cairn gates, and Cairn validation.
- **Non-claims**: Anvil architecture only; no new world-format support or compatibility claim is promoted.
