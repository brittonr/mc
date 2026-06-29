# Proposal: Extract Stevenarella model functional core

## Why

`clients/stevenarella/src/model/mod.rs` combines resource reference parsing, model file path resolution, blockstate variant handling, multipart rules, raw model loading, model geometry, biome/light calculations, and vertex construction. Model compatibility and rendering behavior need smaller cores that can be tested without resource manager and renderer side effects.

## What Changes

- Extract pure cores for resource reference parsing, model path normalization, blockstate variant selection, multipart rule evaluation, model inheritance decisions, biome/light calculations, and vertex planning.
- Keep resource reads, JSON decoding, texture lookup, random source selection, and renderer-facing allocation in shells.
- Preserve current model/resource behavior, blockstate selection semantics, lighting/biome behavior, public types, and non-claims.
- Add positive and negative tests for extracted model cores and malformed model inputs.

## Impact

- **Files**: `clients/stevenarella/src/model/mod.rs`, model submodules, focused tests, affected render checks, and Cairn artifacts.
- **Testing**: baseline model/render tests, positive and negative model-core tests, affected dry-runs, Cairn gates, and Cairn validation.
- **Non-claims**: model architecture only; no new model, resource-pack, or rendering compatibility claim is promoted.
