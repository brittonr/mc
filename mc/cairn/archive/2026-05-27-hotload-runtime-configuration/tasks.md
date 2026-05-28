# Tasks

## Phase: inventory

- [x] [serial] Build a runtime value inventory for compatibility harness, runner, Valence CTF/game rail, and Stevenarella launch path. r[runtime_configuration.hotload_runtime_configuration.config_inventory]
- [x] [serial] Classify each inventory row as hot, next-run, restart-only, or fixed-protocol-fact. r[runtime_configuration.hotload_runtime_configuration.reload_mutability]

## Phase: Steel config/policy contract

- [x] [serial] Add Steel module contract docs and sample modules with documented defaults, explicit exports, sandbox profile, and environment overlays. r[runtime_configuration.hotload_runtime_configuration.steel_module_contract]
- [x] [serial] Add Rust-owned typed contracts for Steel exports and policy hook inputs/outputs. r[runtime_configuration.hotload_runtime_configuration.typed_steel_boundary]
- [x] [serial] Add normalized snapshot export with schema version, Steel module hash, evaluated exports, provenance, sandbox metadata, and redaction metadata. r[runtime_configuration.hotload_runtime_configuration.normalized_snapshot]

## Phase: implementation

- [x] [parallel] Implement pure config normalization, validation, diff, and apply-plan core. r[runtime_configuration.hotload_runtime_configuration.loader_validation]
- [x] [parallel] Implement isolated Steel compile/evaluate shell with bounded host capabilities. r[runtime_configuration.hotload_runtime_configuration.steel_sandbox]
- [x] [parallel] Implement thin runtime shell for loading snapshots, watching or receiving reload requests, logging diffs, and invoking apply handlers. r[runtime_configuration.hotload_runtime_configuration.hot_reload]
- [x] [serial] Migrate direct runtime value setters to read from typed Steel-backed config or documented fixed-protocol constants. r[runtime_configuration.hotload_runtime_configuration.config_inventory]
- [x] [serial] Add an arrow-damage hotload policy path as the representative gameplay-rule migration. r[runtime_configuration.hotload_runtime_configuration.arrow_damage_policy]

## Phase: verification

- [x] [parallel] Add positive tests for defaults, module edits, hot reload, arrow-damage policy decisions, next-run/restart-only reporting, and redaction. r[runtime_configuration.hotload_runtime_configuration.reload_tests]
- [x] [parallel] Add negative tests for unknown exports, missing exports, wrong types, invalid ranges, malformed Steel modules, sandbox violations, nondeterministic policy attempts, hot-apply failure rollback, and restart-only hot-apply attempts. r[runtime_configuration.hotload_runtime_configuration.reload_tests]
- [x] [serial] Add checker/evidence tying inventory, Steel module contract, typed boundary, exported snapshot, tests, and migrated call sites together. r[runtime_configuration.hotload_runtime_configuration.reload_evidence]
- [x] [serial] Run Cairn gates, validation, and archive only after completed tasks match implemented/evidenced behavior. r[runtime_configuration.hotload_runtime_configuration.reload_evidence]

## Progress

- Inventory, mutability classes, Steel module contract, typed boundary, and normalized snapshot are evidenced by:
  - `docs/evidence/runtime-config-inventory-2026-05-27.tsv`
  - `config/mc-compat/steel/default.scm`
  - `docs/evidence/steel-runtime-config-contract-2026-05-27.md`
  - `docs/evidence/steel-runtime-config-default.snapshot.json`
  - `tools/check_runtime_steel_config.rs`
- `tools/check_runtime_steel_config.rs --self-test` includes positive and negative fixtures for missing inventory rows, invalid mutability, missing Steel exports, forbidden sandbox tokens, invalid arrow-damage policy shape, and snapshot hash mismatch.
- `tools/mc-compat-runner/src/runtime_config.rs` implements pure Steel-export normalization, validation, diff/apply-plan generation, mutability enforcement, restricted Steel literal evaluation, sandbox token rejection, unknown export rejection, and bounded arrow-damage policy evaluation. Unit tests cover valid exports, missing/wrong-type exports, range/sandbox rejection, arrow damage clamping, hot/next-run/restart-only separation, and fixed-protocol-fact rejection.
- `mc-compat-runner` accepts `--steel-config` / `MC_COMPAT_STEEL_CONFIG` for startup-time restricted Steel config loading, with env and later CLI flags still taking precedence.
- `RuntimeConfigController::reload_with` receives reload candidates, computes diffs/apply plans, invokes hot apply handlers, rejects restart-only hot application, and preserves the previous snapshot on validation/apply failure. This chooses explicit reload requests rather than a filesystem watcher for the first shell.
- Steel-backed startup migration now covers the module-exported runner/server/client/Valence/evidence fields; direct env/CLI setters remain documented precedence overrides.
- The projectile damage dry-run/evidence rail uses the Steel-backed arrow-damage policy to derive expected damage and victim health, giving a representative gameplay-rule migration without claiming Valence server-side Steel integration.
- Positive and negative tests cover defaults, module edits, hot reload, arrow-damage decisions, next-run/restart-only reporting, redaction, unknown/missing/wrong-type/range-invalid exports, malformed policy shape, sandbox/nondeterminism rejection, hot-apply rollback, and restart-only hot-apply rejection.
- No filesystem watcher, Valence server-side Steel integration, remote config distribution, or full production rollout is claimed.
