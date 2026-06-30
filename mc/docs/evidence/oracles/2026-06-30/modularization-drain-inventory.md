# Modularization drain inventory — 2026-06-30

## Question

Which active Cairn modularization packages still require source edits, and which are already satisfied by focused modules that need validation-only closeout?

## Inspected evidence

- `compat/runner/src/lib.rs` now remains the public runner façade for `run_main()` while scenario-route CLI parsing and router path safety live in `compat/runner/src/scenario_route.rs`.
- `compat/runner/src/scenario_core.rs` keeps the stable scenario API, while focused scenario family classification and targeted-packet family coverage live in `compat/runner/src/scenario_families.rs`.
- Existing runner modules already own side-effect/pure-core boundaries for backend lifecycle, client driving, env patch composition, evidence bundles, receipts, planning, runtime config, and wire parsing.
- `flake.nix` imports focused Nix aggregators under `nix/packages.nix`, `nix/apps.nix`, `nix/checks.nix`, and `nix/devshells.nix`. The existing `mc-flake-output-inventory` check compares current package/app/check/devshell names to `docs/evidence/split-root-flake-modules-baseline-output-inventory.json` with explicit allowed check additions.
- `clients/stevenarella/src/server/` already has focused modules for login, session, chunks, world state, entities, inventory, plugin messages, dispatch, target helpers, and probe cores. Positive and negative tests are present in those modules.
- `clients/stevenarella/src/world/` already has tracked `biome.rs` and `storage.rs`; a pre-existing untracked `clients/stevenarella/src/world/core.rs` and archive directory were present before this drain pass and were preserved rather than rewritten.
- `servers/valence/examples/ctf.rs` and `servers/valence/examples/survival_compat.rs` remain example/plugin shells. Pure fixture decisions live under `servers/valence/examples/fixture_core/ctf.rs`, `servers/valence/examples/fixture_core/ctf/*`, `servers/valence/examples/fixture_core/survival.rs`, and `servers/valence/examples/fixture_core/survival/*` with positive and negative tests.
- Pre-existing non-Cairn modifications in `servers/valence/crates/valence_network/src/profile_cache.rs` and `servers/valence/crates/valence_network/src/proxy_broadcast.rs` were inspected and left untouched.

## Decision

- Source edits were required for the runner/scenario active changes because parser and family ownership were still centralized. The drain pass extracted scenario-route parsing into `scenario_route.rs` and added explicit family ownership/coverage in `scenario_families.rs` without changing public CLI names, receipt schema, or scenario names.
- The Nix, Stevenarella server runtime, Stevenarella hotspot, and Valence fixture active changes are treated as validation closeouts over already-present focused modules. Their tasks should cite fresh focused validation logs plus this inventory rather than claim new gameplay or compatibility behavior.
- No active change broadens live compatibility, semantic equivalence, public-server safety, production readiness, or full gameplay correctness claims.

## Owner

mc Cairn drain owner for the 2026-06-30 modularization queue.

## Next action

Use focused runner, generated-surface, Nix output-inventory, Stevenarella, Valence fixture, Cairn gate/validation, and task-evidence logs as closeout evidence before syncing and archiving the active changes.
