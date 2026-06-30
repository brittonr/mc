# Proposal: Reduce Stevenarella secondary hotspot modules

## Why

Beyond `server/mod.rs`, several Stevenarella `mod.rs` files and root modules remain large enough to slow review: `world/mod.rs`, `model/mod.rs`, `ui/mod.rs`, `ecs/mod.rs`, and `control.rs`. These modules mix public façades, data models, rendering or ECS adaptation, parsing/normalization, and tests. They should become thin façades over focused child modules without changing client behavior.

## What Changes

- Inventory secondary hotspot modules, responsibility groups, public APIs, and baseline tests before extraction.
- Convert large `mod.rs`/root modules into public façades that re-export focused child modules for data models, pure decisions, rendering/adapters, storage, and shell operations.
- Keep deterministic logic in pure helpers and keep renderer, GL, filesystem, network, and global state interactions in shells.
- Preserve public module paths where practical, default client behavior, compat instrumentation boundaries, protocol behavior, and non-claims.
- Add positive and negative tests for extracted pure logic and API/behavior parity.

## Impact

- **Files**: `clients/stevenarella/src/{world,model,ui,ecs}/*`, `clients/stevenarella/src/control.rs`, module tests, docs if public boundaries move, and Cairn artifacts.
- **Testing**: baseline and post-change focused Stevenarella tests for each touched module, affected mc-compat dry-runs if instrumentation behavior changes, Cairn gates, and Cairn validation.
- **Non-claims**: maintainability only; this does not add protocol support, rendering guarantees, gameplay parity, public-server safety, or production readiness evidence.
