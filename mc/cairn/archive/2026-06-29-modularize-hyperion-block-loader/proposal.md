# Proposal: Modularize Hyperion block loader

## Why

`hyperion/crates/hyperion/src/simulation/blocks/loader/mod.rs` is a notable Hyperion block-loading concentration point. Block loading should separate parse/validation decisions from storage and runtime side effects, while respecting Hyperion's nested-repo ownership.

## What Changes

- In Hyperion, split block loader code into parsing, validation, palette/section planning, storage update plans, and runtime shell modules.
- Keep file/resource reads, decompression, storage mutation, ECS mutation, and tracing in shells.
- Preserve Hyperion block-loader APIs, world/block behavior, performance-sensitive boundaries, and non-claims.
- Add positive and negative tests for loader parse/validation/planning cores.

## Impact

- **Files**: Hyperion block-loader modules under `hyperion/`, Hyperion tests, optional parent evidence notes, and Cairn artifacts.
- **Testing**: Hyperion baseline/focused tests from `hyperion/`, Cairn gates, and Cairn validation.
- **Non-claims**: Hyperion modularity only; no Valence adoption or compatibility evidence claim is promoted.
