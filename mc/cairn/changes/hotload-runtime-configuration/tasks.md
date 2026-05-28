# Tasks

## Phase: inventory

- [ ] [serial] Build a runtime value inventory for compatibility harness, runner, Valence CTF/game rail, and Stevenarella launch path. r[runtime_configuration.hotload_runtime_configuration.config_inventory]
- [ ] [serial] Classify each inventory row as hot, next-run, restart-only, or fixed-protocol-fact. r[runtime_configuration.hotload_runtime_configuration.reload_mutability]

## Phase: Steel config/policy contract

- [ ] [serial] Add Steel module contract docs and sample modules with documented defaults, explicit exports, sandbox profile, and environment overlays. r[runtime_configuration.hotload_runtime_configuration.steel_module_contract]
- [ ] [serial] Add Rust-owned typed contracts for Steel exports and policy hook inputs/outputs. r[runtime_configuration.hotload_runtime_configuration.typed_steel_boundary]
- [ ] [serial] Add normalized snapshot export with schema version, Steel module hash, evaluated exports, provenance, sandbox metadata, and redaction metadata. r[runtime_configuration.hotload_runtime_configuration.normalized_snapshot]

## Phase: implementation

- [ ] [parallel] Implement pure config normalization, validation, diff, and apply-plan core. r[runtime_configuration.hotload_runtime_configuration.loader_validation]
- [ ] [parallel] Implement isolated Steel compile/evaluate shell with bounded host capabilities. r[runtime_configuration.hotload_runtime_configuration.steel_sandbox]
- [ ] [parallel] Implement thin runtime shell for loading snapshots, watching or receiving reload requests, logging diffs, and invoking apply handlers. r[runtime_configuration.hotload_runtime_configuration.hot_reload]
- [ ] [serial] Migrate direct runtime value setters to read from typed Steel-backed config or documented fixed-protocol constants. r[runtime_configuration.hotload_runtime_configuration.config_inventory]
- [ ] [serial] Add an arrow-damage hotload policy path as the representative gameplay-rule migration. r[runtime_configuration.hotload_runtime_configuration.arrow_damage_policy]

## Phase: verification

- [ ] [parallel] Add positive tests for defaults, module edits, hot reload, arrow-damage policy decisions, next-run/restart-only reporting, and redaction. r[runtime_configuration.hotload_runtime_configuration.reload_tests]
- [ ] [parallel] Add negative tests for unknown exports, missing exports, wrong types, invalid ranges, malformed Steel modules, sandbox violations, nondeterministic policy attempts, hot-apply failure rollback, and restart-only hot-apply attempts. r[runtime_configuration.hotload_runtime_configuration.reload_tests]
- [ ] [serial] Add checker/evidence tying inventory, Steel module contract, typed boundary, exported snapshot, tests, and migrated call sites together. r[runtime_configuration.hotload_runtime_configuration.reload_evidence]
- [ ] [serial] Run Cairn gates, validation, and archive only after completed tasks match implemented/evidenced behavior. r[runtime_configuration.hotload_runtime_configuration.reload_evidence]
