# Proposal: Modularize Valence network session code

## Why

`servers/valence/crates/valence_network/src/lib.rs` and `connect.rs` combine connection setup, status response handling, login/session state, legacy ping behavior, packet IO, and profile/cache interactions. Network behavior is compatibility-sensitive and should be split into pure protocol/session decisions and thin async/socket shells.

## What Changes

- Split network code into focused modules for listener/connect orchestration, status/legacy ping, login/session negotiation, packet IO framing, profile/cache lookup adapters, and pure session decisions.
- Extract pure decisions for state transitions, status response facts, compression thresholds, disconnect reasons, and login/session validation.
- Keep sockets, async tasks, channels, clocks, profile cache IO, and packet reads/writes in shells.
- Preserve existing public APIs, packet/session behavior, status/legacy ping behavior, profile behavior, and non-claims.

## Impact

- **Files**: `servers/valence/crates/valence_network/src/*`, focused network tests, affected examples/checks, and Cairn artifacts.
- **Testing**: baseline Valence network tests, positive and negative session-core tests, affected smoke/dry-runs, Cairn gates, and Cairn validation.
- **Non-claims**: network architecture only; no new protocol compatibility or production-readiness claim is promoted.
