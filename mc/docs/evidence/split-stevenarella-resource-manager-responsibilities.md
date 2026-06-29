# split-stevenarella-resource-manager responsibility map

## Question

Which Stevenarella resource manager responsibilities moved out of the former monolithic `clients/stevenarella/src/resources.rs` shell?

## Inspected evidence

- `clients/stevenarella/src/resources.rs` now keeps public API orchestration for `Manager`, `ManagerUI`, downloads, UI progress shell wiring, and compatibility-preserving calls.
- `clients/stevenarella/src/resources/identifiers.rs` owns resource identifiers and legacy-to-modern resource aliases.
- `clients/stevenarella/src/resources/paths.rs` owns contained relative path decisions.
- `clients/stevenarella/src/resources/lookup.rs` owns lookup candidate ordering, pack precedence, and duplicate entry detection.
- `clients/stevenarella/src/resources/cache.rs` owns asset object cache key path and URL derivation.
- `clients/stevenarella/src/resources/archive.rs` owns archive entry acceptance and output path containment.
- `clients/stevenarella/src/resources/io_shell.rs` owns filesystem/internal/object pack adapters, asset commit IO, and progress read adapters.
- `clients/stevenarella/src/resources/pack_discovery.rs` owns downloaded pack insertion policy.
- `clients/stevenarella/src/resources/state.rs` owns shared manager, progress, and pack trait state.

## Decision

The split keeps rendering/model/UI callers on the existing `crate::resources` public API while moving resource decisions into focused modules that can be tested without filesystem, archive, download, lock, or logging side effects. The change remains a resource architecture claim only; it does not promote broad Minecraft compatibility, public-server safety, production readiness, or full gameplay correctness.

## Owner

Stevenarella client subtree: `clients/stevenarella/`.

## Next action

Use focused `cargo test resources`, affected model/render tests, Cairn gates, Cairn validation, and evidence manifest checks before archive.
