# Proposal: Split Stevenarella resource manager

## Why

`clients/stevenarella/src/resources.rs` combines resource pack discovery, lookup, cache behavior, IO, archive handling, and resource manager state. Resource decisions are used by rendering, models, UI, and server behavior, so path/lookup/cache rules should be testable without filesystem or network side effects.

## What Changes

- Split resources into modules for path normalization, pack discovery, lookup/indexing, cache policy, archive access, IO shell, and shared manager state.
- Extract pure decisions for resource identifiers, lookup precedence, cache keys, path containment, and pack selection.
- Keep filesystem reads, archive IO, downloads, resource manager locks, and logging in shells.
- Preserve public resource APIs, lookup precedence, cache behavior, path safety, and non-claims.

## Impact

- **Files**: `clients/stevenarella/src/resources.rs`, resource submodules, focused tests, affected model/render checks, and Cairn artifacts.
- **Testing**: baseline resource tests, positive and negative resource-core tests, affected dry-runs, Cairn gates, and Cairn validation.
- **Non-claims**: resource architecture only; no new resource-pack compatibility claim is promoted.
