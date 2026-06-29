# Proposal: Split Stevenarella player entity responsibilities

## Why

`clients/stevenarella/src/entity/player.rs` combines player creation, player model state, rendering, movement, collision checks, and ECS system wiring. Player behavior is central to manual and compatibility checks, so rendering and movement decisions should be easier to test without full ECS/render setup.

## What Changes

- Split player entity code into focused modules for construction, model state, rendering, movement input/state, collision decisions, and ECS system shells.
- Extract pure movement, collision, and model-state decisions over explicit inputs.
- Keep ECS mutation, renderer calls, resource lookups, and packet/network interactions in shells.
- Preserve existing player behavior, model visibility, movement/collision semantics, public APIs, and non-claims.

## Impact

- **Files**: `clients/stevenarella/src/entity/player.rs`, new player/entity modules, focused tests, affected client checks, and Cairn artifacts.
- **Testing**: baseline player/entity tests, positive and negative player-core tests, affected dry-runs, Cairn gates, and Cairn validation.
- **Non-claims**: client player architecture only; no new gameplay, rendering, movement, or compatibility claim is promoted.
