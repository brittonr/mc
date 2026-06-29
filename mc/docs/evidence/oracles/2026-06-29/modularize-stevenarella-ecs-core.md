# Modularize Stevenarella ECS core checkpoint

## Question

Does the Stevenarella ECS refactor keep each ECS responsibility owned by a focused module while preserving the existing public ECS surface and claim boundary?

## Inspected evidence

- `clients/stevenarella/src/ecs/mod.rs` now keeps `Manager` as the imperative shell and re-exports `Entity`, `Key`, `Filter`, and `System`.
- `clients/stevenarella/src/ecs/entity.rs` owns entity identifiers, world-entity constants, entity state, generation matching, and deterministic allocation decisions.
- `clients/stevenarella/src/ecs/components.rs` owns component keys and component-memory storage.
- `clients/stevenarella/src/ecs/query.rs` owns filters, entity/filter matching, and query-shape checks.
- `clients/stevenarella/src/ecs/registration.rs` owns system registration surfaces.
- `clients/stevenarella/src/ecs/execution.rs` owns deterministic system ordering and add/remove trigger decisions.
- `clients/stevenarella/src/ecs/diagnostics.rs` owns component mutation diagnostics and legacy panic messages.
- Focused validation is recorded in `docs/evidence/run-logs/2026-06-29/modularize-stevenarella-ecs-core.post-storage-cleanup-ecs-tests.run.log`.

## Decision

The refactor satisfies the ECS architecture scope: allocation, lookup, query-shape, system-ordering, and diagnostic decisions have focused deterministic cores where practical, while mutation and client integration remain in the `Manager` shell. Public ECS names remain available from `ecs::`.

## Owner

Stevenarella core client subtree: `clients/stevenarella/`.

## Next action

Keep the change scoped to ECS modularity. Do not promote broad Minecraft compatibility, gameplay correctness, production readiness, public-server safety, full CTF correctness, or full survival correctness claims from this evidence.
