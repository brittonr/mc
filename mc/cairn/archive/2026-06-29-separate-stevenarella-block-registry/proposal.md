# Proposal: Separate Stevenarella block registry data from runtime APIs

## Why

`clients/stevenarella/blocks/src/lib.rs` is a very large generated-or-macro-expanded block registry surface that also owns runtime mapping APIs, material exports, collision/model metadata, and tests. Mixing generated block facts with hand-authored runtime behavior makes targeted compatibility fixes harder to review and increases the chance of accidental changes to generated data.

## What Changes

- Separate generated block definitions and vanilla id maps from hand-authored runtime APIs such as `WorldAccess`, `VanillaIDMap`, collision helpers, material helpers, and public exports.
- Introduce a deterministic generation or freshness boundary for generated block data if the data is regenerated.
- Preserve public block names, IDs, material/collision semantics, `VanillaIDMap` behavior, modded-block fallback behavior, and non-claims.
- Add positive and negative tests for id lookup, modded fallback, generated-data freshness, collision/material access, and runtime API behavior.

## Impact

- **Files**: `clients/stevenarella/blocks/src/lib.rs`, new block-registry/generated/runtime modules, optional generator/checker surfaces, focused block tests, and Cairn artifacts.
- **Testing**: baseline block/protocol tests, generated freshness checks if added, positive and negative block-registry tests, affected mc-compat dry-runs, Cairn gates, and Cairn validation.
- **Non-claims**: code organization and registry safety only; this does not add new block support or claim full protocol/world compatibility.
