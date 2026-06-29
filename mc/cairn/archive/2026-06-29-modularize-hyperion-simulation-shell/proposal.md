# Proposal: Modularize Hyperion simulation shell

## Why

`hyperion/crates/hyperion/src/simulation/mod.rs` is a large Hyperion simulation umbrella that coordinates engine simulation concerns. Hyperion is an independent nested repo, so any modularity work must be Hyperion-local and must not imply Valence adoption.

## What Changes

- In Hyperion, split simulation shell responsibilities into focused modules for system registration, simulation state orchestration, packet-facing adapters, blocks/entities/inventory coordination, and diagnostics.
- Extract pure simulation planning/state transition cores where practical.
- Keep Bevy ECS mutation, packet emission, network/proxy integration, tracing, and scheduling in Hyperion shells.
- Preserve Hyperion public APIs, simulation behavior, performance-sensitive boundaries, and non-claims.

## Impact

- **Files**: Hyperion simulation modules under `hyperion/`, Hyperion tests, optional parent evidence notes, and Cairn artifacts.
- **Testing**: Hyperion baseline/focused tests from `hyperion/`, Cairn gates, and Cairn validation.
- **Non-claims**: Hyperion modularity only; no Valence adoption, Hyperion compatibility, production-scale, or mc-compat evidence claim is promoted.
