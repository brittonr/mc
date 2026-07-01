# Proposal: Extract Stevenarella world and dimension functional core

## Why

`clients/stevenarella/src/world/mod.rs` mixes world storage, dimension-codec bounds, chunk section parsing, biome/light/block updates, storage mutation, and protocol-version conditionals. The 1.20.1 compatibility notes already call out dimension-codec bounds as a sensitive area, so world/chunk decisions need smaller pure cores with focused positive and negative tests.

## What Changes

- Extract pure cores for dimension bounds selection, chunk section layout decisions, biome/light data interpretation, block update decisions, and storage update plans.
- Keep byte reading, NBT traversal, world storage mutation, rendering invalidation, and logging in shells that call those cores.
- Preserve existing world behavior, dimension fallback behavior, protocol-version handling, chunk parsing semantics, and evidence non-claims.
- Add positive and negative tests for dimension-codec selection, min-y/height application, chunk-section counts, biome/light boundaries, malformed data, and unsupported dimension inputs.

## Impact

- **Files**: `clients/stevenarella/src/world/mod.rs`, new `clients/stevenarella/src/world/*` modules, focused world tests, affected protocol/compat dry-runs, and Cairn artifacts.
- **Testing**: baseline world/protocol tests, positive and negative world-core tests, affected mc-compat dry-runs, Cairn gates, and Cairn validation.
- **Non-claims**: world/chunk architecture and targeted correctness guards only; this does not claim full protocol 763 support or broad Minecraft compatibility.
