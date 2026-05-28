# Tasks

## Phase: inventory

- [ ] [serial] Build a runtime value inventory for compatibility harness, runner, Valence CTF/game rail, and Stevenarella launch path. r[runtime_configuration.hotload_runtime_configuration.config_inventory]
- [ ] [serial] Classify each inventory row as hot, next-run, restart-only, or fixed-protocol-fact. r[runtime_configuration.hotload_runtime_configuration.reload_mutability]

## Phase: schema

- [ ] [serial] Add Nickel schema/contracts with documented defaults and environment overlays. r[runtime_configuration.hotload_runtime_configuration.nickel_schema]
- [ ] [serial] Add normalized snapshot export with schema version, BLAKE3 hash, provenance, and redaction metadata. r[runtime_configuration.hotload_runtime_configuration.normalized_snapshot]
- [ ] [serial] Document Steel Scheme boundary and keep Steel disabled unless a typed/sandboxed proof is added. r[runtime_configuration.hotload_runtime_configuration.steel_boundary]

## Phase: implementation

- [ ] [parallel] Implement pure config normalization, validation, diff, and apply-plan core. r[runtime_configuration.hotload_runtime_configuration.loader_validation]
- [ ] [parallel] Implement thin runtime shell for loading snapshots, watching or receiving reload requests, logging diffs, and invoking apply handlers. r[runtime_configuration.hotload_runtime_configuration.hot_reload]
- [ ] [serial] Migrate direct runtime value setters to read from typed config or documented fixed-protocol constants. r[runtime_configuration.hotload_runtime_configuration.config_inventory]

## Phase: verification

- [ ] [parallel] Add positive tests for defaults, overlays, hot reload, next-run/restart-only reporting, and redaction. r[runtime_configuration.hotload_runtime_configuration.reload_tests]
- [ ] [parallel] Add negative tests for unknown fields, missing fields, wrong types, invalid ranges, malformed snapshots, hot-apply failure rollback, and restart-only hot-apply attempts. r[runtime_configuration.hotload_runtime_configuration.reload_tests]
- [ ] [serial] Add checker/evidence tying inventory, Nickel schema, exported snapshot, tests, and migrated call sites together. r[runtime_configuration.hotload_runtime_configuration.reload_evidence]
- [ ] [serial] Run Cairn gates, validation, and archive only after completed tasks match implemented/evidenced behavior. r[runtime_configuration.hotload_runtime_configuration.reload_evidence]
