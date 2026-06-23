# Proposal: Retire legacy layout transition paths

## Why

`compat/runner/src/layout.rs` still accepts both role-based paths and older transition paths. That was useful during migration, but long-term dual layouts add ambiguity and increase the chance that tooling probes the wrong component root. Once current users are migrated, the workspace should prefer one canonical path per role.

## What Changes

- Inventory every transition path currently accepted by layout resolution and any command, doc, or evidence reference that still uses it.
- Choose canonical role-based paths for client, server, compat runner/config, fixtures, and related component roots.
- Replace transition-path support with explicit diagnostics or documented compatibility shims where needed.
- Add tests that reject ambiguous duplicate roots and prove canonical roots still resolve.

## Impact

- **Files**: `compat/runner/src/layout.rs`, layout tests, README/docs/AGENTS path references, flake wrappers if paths are embedded, evidence docs if stale paths are cited, and Cairn artifacts.
- **Testing**: layout resolver positive/negative tests, missing-checkout diagnostics, dry-run runner checks, Cairn validation/gates.
- **Non-claims**: this simplifies layout resolution only; it does not change scenario semantics, receipt schemas, or compatibility evidence claims.
