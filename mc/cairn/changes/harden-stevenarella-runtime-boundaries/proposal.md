# Proposal: Harden Stevenarella runtime boundaries

## Why

Stevenarella still has safety and coupling hotspots: `gl/mod.rs` stores a global mutable GL context, `resources.rs` declares an unsafe `Sync` implementation for the resource manager, and `ecs/mod.rs` uses lifetime transmute and raw storage internals. These patterns make rendering, resources, and ECS state difficult to reason about and harder to test in isolation.

## What Changes

- Audit unsafe/global runtime boundaries and document invariants before changing behavior.
- Replace or quarantine the global GL context behind an explicit context/token boundary.
- Split resource manager thread-sharing concerns from resource IO and progress state.
- Encapsulate ECS unsafe storage behind safe generational APIs and invariant tests.
- Preserve observable client behavior and mc-compat rail outputs.

## Impact

- **Files**: `clients/stevenarella/src/gl/mod.rs`, `resources.rs`, `ecs/mod.rs`, render/model/chunk-builder call sites, focused tests and safety docs.
- **Testing**: focused Stevenarella unit tests, compile checks through the mc devshell, selected render/capture or mc-compat checks where affected, positive and negative invariant tests, and Cairn gates.
