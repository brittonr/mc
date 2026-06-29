# Proposal: Modularize Stevenarella ECS core

## Why

`clients/stevenarella/src/ecs/mod.rs` owns entity allocation, component storage, system registration/execution, queries, and tests in one module. ECS behavior underpins most client state, so storage and scheduling decisions should be separated from mutation-heavy shells.

## What Changes

- Split ECS code into modules for entity IDs, component storage, query access, system registration, system execution, and diagnostics.
- Extract pure allocation, lookup, scheduling-order, and query-shape decisions where practical.
- Preserve public ECS APIs, execution order, component behavior, borrow/error behavior, and non-claims.

## Impact

- **Files**: `clients/stevenarella/src/ecs/mod.rs`, ECS submodules, focused tests, and Cairn artifacts.
- **Testing**: baseline ECS tests, positive and negative ECS-core tests, affected client checks, Cairn gates, and Cairn validation.
- **Non-claims**: ECS architecture only; no gameplay or compatibility claim is promoted.
