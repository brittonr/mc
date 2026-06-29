# Proposal: Extract Valence packet compose core

## Why

`servers/valence/crates/valence_server/src/packet_compose.rs` concentrates packet composition rules that bridge server state and protocol packets. Packet composition is compatibility-sensitive and should separate deterministic packet-plan decisions from client/layer shells.

## What Changes

- Extract pure packet composition cores for selected packet families and shared packet-plan helpers.
- Keep client access, layer access, packet writes, Bevy systems, and logging in shells.
- Preserve packet bytes/fields, public APIs, ordering behavior, protocol assumptions, and non-claims.
- Add positive and negative tests for packet-plan decisions and malformed inputs.

## Impact

- **Files**: `servers/valence/crates/valence_server/src/packet_compose.rs`, packet composition submodules, focused tests, affected protocol/example checks, and Cairn artifacts.
- **Testing**: baseline packet compose tests, positive and negative packet-core tests, affected dry-runs, Cairn gates, and Cairn validation.
- **Non-claims**: packet composition architecture only; no new protocol support or compatibility evidence is promoted.
