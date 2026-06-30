# Proposal: Modularize Stevenarella server runtime and compat probes

## Why

`clients/stevenarella/src/server/mod.rs` has become the main client runtime catch-all: connection/login flow, packet handlers, world and entity updates, chunk/block-entity decoding, inventory/window logic, chat/plugin messages, and many mc-compat probe state machines live in one file. The size makes protocol changes risky and makes pure probe decisions hard to test without standing up the client runtime.

## What Changes

- Inventory the current server runtime responsibilities and packet/probe ownership map before extraction.
- Split protocol handlers into focused server submodules for login/session, world/chunks, entities, inventory/windows, block entities/signs, chat/plugin messages, and common dispatch helpers.
- Move mc-compat probe state into cohesive state structs/modules grouped by CTF, inventory, survival, combat/projectile, and sign/dimension behavior.
- Keep pure probe decisions testable over explicit inputs while the `Server` shell owns packet writes, ECS/world mutation, renderer calls, logging, and connection I/O.
- Preserve public client behavior, typed milestone vocabulary, receipt non-claims, and existing mc-compat scenario semantics.

## Impact

- **Files**: `clients/stevenarella/src/server/mod.rs`, `clients/stevenarella/src/server/*`, new focused server/probe modules, Stevenarella tests, and Cairn artifacts.
- **Testing**: baseline and post-change Stevenarella server/probe tests, affected protocol/compat probe tests, selected mc-compat dry-runs, Cairn gates, and Cairn validation.
- **Non-claims**: maintainability and testability only; this does not add new protocol coverage, gameplay parity, public-server safety, or production readiness evidence.
