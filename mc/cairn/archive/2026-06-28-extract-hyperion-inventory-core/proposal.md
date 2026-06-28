# Proposal: Extract Hyperion inventory functional core

## Why

`hyperion/crates/hyperion/src/simulation/inventory.rs` is the largest Hyperion engine file in this workspace scan. Inventory simulation is domain logic that should be testable as pure transitions, but Hyperion remains an independent nested repository, so any work must respect Hyperion ownership and not imply Valence adoption.

## What Changes

- In Hyperion, inventory simulation SHOULD be split into pure transaction/state cores and thin ECS/runtime shells.
- Classify the work as Hyperion-owned unless a separate Valence integration Cairn explicitly ports or references the design.
- Preserve Hyperion public APIs, simulation behavior, packet-facing behavior, and performance-sensitive boundaries.
- Add positive and negative Hyperion inventory transition tests.
- Record nested-repo validation from the Hyperion root.

## Impact

- **Files**: Hyperion inventory simulation modules under `hyperion/`, Hyperion tests, optional parent evidence notes, and Cairn artifacts.
- **Testing**: Hyperion baseline/focused tests from `hyperion/`, Cairn gates, and Cairn validation.
- **Non-claims**: Hyperion modularity only; no Valence adoption, vanilla parity, production-safety, or mc-compat evidence claim is promoted.
