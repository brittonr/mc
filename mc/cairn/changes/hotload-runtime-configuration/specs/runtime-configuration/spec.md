# Delta: Hotloaded runtime configuration

## ADDED Requirements

### Requirement: Runtime value inventory

r[runtime_configuration.hotload_runtime_configuration.config_inventory] The configuration system MUST maintain a reviewable inventory of every operator-set or repo-set runtime value in the targeted compatibility rails before migrating those values behind config.

#### Scenario: Inventory records ownership and migration state

r[runtime_configuration.hotload_runtime_configuration.config_inventory.scenario]
- GIVEN a runtime value is set by a compatibility command, environment variable, runner default, game rail, or launch path
- WHEN the inventory is reviewed
- THEN the row records name, owner, source location, current default, type, contract, runtime consumer, migration status, and evidence path
- AND protocol facts are documented separately from operator-configurable values

### Requirement: Reload mutability classification

r[runtime_configuration.hotload_runtime_configuration.reload_mutability] Each inventory row MUST declare whether it is hot, next-run, restart-only, or fixed-protocol-fact before any reload behavior is implemented.

#### Scenario: Unsafe live mutation is blocked

r[runtime_configuration.hotload_runtime_configuration.reload_mutability.scenario]
- GIVEN a candidate config changes a next-run, restart-only, or fixed-protocol-fact row
- WHEN the reload plan is generated
- THEN the change is reported without mutating live state
- AND only rows classified as hot can appear in the live apply plan

### Requirement: Nickel schema

r[runtime_configuration.hotload_runtime_configuration.nickel_schema] The configuration source of truth MUST be a typed Nickel schema with documented defaults, contracts, and mergeable overlays.

#### Scenario: Nickel rejects invalid config before runtime load

r[runtime_configuration.hotload_runtime_configuration.nickel_schema.scenario]
- GIVEN an operator edits config or an overlay
- WHEN Nickel typecheck/export runs
- THEN unknown fields, missing required fields, wrong types, and invalid contract values fail before the runtime snapshot is accepted
- AND defaults remain visible in the schema documentation

### Requirement: Normalized runtime snapshot

r[runtime_configuration.hotload_runtime_configuration.normalized_snapshot] Runtime code MUST load a normalized snapshot that records schema version, evaluated values, BLAKE3 source hash, provenance, and redaction metadata.

#### Scenario: Snapshot provenance is reviewable

r[runtime_configuration.hotload_runtime_configuration.normalized_snapshot.scenario]
- GIVEN a process loads config
- WHEN it records startup or reload evidence
- THEN the evidence names the snapshot schema version, BLAKE3 hash, source path or overlay identifier, generation command, and redacted fields
- AND secret-like values are not emitted in cleartext logs

### Requirement: Loader validation

r[runtime_configuration.hotload_runtime_configuration.loader_validation] The loader MUST validate snapshots into typed domain config through a pure functional core before any caller observes the new config.

#### Scenario: Invalid snapshot fails closed

r[runtime_configuration.hotload_runtime_configuration.loader_validation.scenario]
- GIVEN a malformed, stale, unknown-field, missing-field, wrong-type, or range-invalid snapshot
- WHEN the loader validates it
- THEN validation returns explicit diagnostics
- AND the previously active config remains authoritative

### Requirement: Atomic hot reload

r[runtime_configuration.hotload_runtime_configuration.hot_reload] Hot reload MUST validate before swap, apply through explicit handlers, and preserve the old snapshot if any validation or apply step fails.

#### Scenario: Reload is all-or-nothing

r[runtime_configuration.hotload_runtime_configuration.hot_reload.scenario]
- GIVEN a candidate config changes hot fields
- WHEN reload is requested
- THEN the system computes a redacted diff and apply plan
- AND publishes the candidate snapshot only after every hot apply handler succeeds
- AND rolls back to the previous snapshot with diagnostics if any handler fails

### Requirement: Steel boundary

r[runtime_configuration.hotload_runtime_configuration.steel_boundary] Steel Scheme MUST NOT be allowed to mutate runtime config or live state unless a separate proof defines sandboxing, deterministic evaluation, capability limits, and typed output validation.

#### Scenario: Steel-derived values pass the typed boundary

r[runtime_configuration.hotload_runtime_configuration.steel_boundary.scenario]
- GIVEN a future Steel policy hook is proposed
- WHEN it produces config-derived values
- THEN those values pass the same typed Nickel/snapshot boundary as static config
- AND failures keep the previous runtime snapshot active

### Requirement: Reload test and evidence coverage

r[runtime_configuration.hotload_runtime_configuration.reload_tests] The implementation MUST include positive and negative tests for config normalization, reload planning, hot application, rollback, redaction, and mutability enforcement.

#### Scenario: Verification covers success and failure paths

r[runtime_configuration.hotload_runtime_configuration.reload_tests.scenario]
- GIVEN the reload test suite runs
- WHEN positive and negative cases execute
- THEN valid defaults, overlays, hot reloads, next-run reporting, restart-only reporting, unknown fields, missing fields, wrong types, invalid ranges, malformed snapshots, apply failures, and redaction are all checked
- AND review-critical receipts or run logs are copied under `docs/evidence/`

### Requirement: Evidence-backed migration

r[runtime_configuration.hotload_runtime_configuration.reload_evidence] A value MUST NOT be marked migrated until inventory, schema, runtime consumer, tests, and evidence all agree on its config path and reload mutability.

#### Scenario: Migration claim matches evidence

r[runtime_configuration.hotload_runtime_configuration.reload_evidence.scenario]
- GIVEN a task claims a runtime value is config-managed
- WHEN the checker reviews the inventory, schema, code annotations or call-site list, and evidence
- THEN the same config path and mutability class appear in all artifacts
- AND unmatched rows remain incomplete tasks rather than archived claims
