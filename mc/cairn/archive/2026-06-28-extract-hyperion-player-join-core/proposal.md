# Proposal: Extract Hyperion player join core

## Why

`hyperion/crates/hyperion/src/egress/player_join/mod.rs` concentrates join egress orchestration, packet/effect selection, state assumptions, and runtime side effects. Join behavior should separate pure join-plan decisions from packet emission and ECS/runtime shells, while staying Hyperion-owned.

## What Changes

- In Hyperion, extract pure player-join plan decisions for initial packets, state summaries, chunk/view setup facts, and join diagnostics.
- Keep ECS access, packet emission, network/proxy integration, tracing, and scheduling in Hyperion shells.
- Preserve Hyperion join behavior, packet order, public APIs, performance-sensitive boundaries, and non-claims.
- Add positive and negative Hyperion-local join-plan tests.

## Impact

- **Files**: Hyperion player-join egress modules under `hyperion/`, Hyperion tests, optional parent evidence notes, and Cairn artifacts.
- **Testing**: Hyperion baseline/focused tests from `hyperion/`, Cairn gates, and Cairn validation.
- **Non-claims**: Hyperion modularity only; no Valence adoption or mc-compat evidence claim is promoted.
