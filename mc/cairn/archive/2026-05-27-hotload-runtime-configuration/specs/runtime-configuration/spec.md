# Delta: Steel-first hotloaded runtime configuration

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

### Requirement: Steel module contract

r[runtime_configuration.hotload_runtime_configuration.steel_module_contract] The editable runtime configuration source MUST be a Steel module with explicit exports, documented defaults, version metadata, and a declared sandbox profile.

#### Scenario: Steel module exports are explicit

r[runtime_configuration.hotload_runtime_configuration.steel_module_contract.scenario]
- GIVEN an operator edits a Steel config module
- WHEN the module is loaded for evaluation
- THEN only documented config and policy exports are accepted
- AND unknown exports, missing required exports, malformed modules, or sandbox profile mismatches are rejected before runtime apply

### Requirement: Typed Steel boundary

r[runtime_configuration.hotload_runtime_configuration.typed_steel_boundary] Steel values and policy results MUST pass Rust-owned typed contracts before becoming runtime config or gameplay decisions.

#### Scenario: Policy output is typed and bounded

r[runtime_configuration.hotload_runtime_configuration.typed_steel_boundary.scenario]
- GIVEN a Steel policy hook returns a decision such as projectile damage
- WHEN Rust validates the result
- THEN the result matches the hook schema, type bounds, range bounds, and mutability policy
- AND invalid output keeps the previous runtime snapshot or previous policy decision active

### Requirement: Normalized runtime snapshot

r[runtime_configuration.hotload_runtime_configuration.normalized_snapshot] Runtime code MUST load a normalized snapshot that records schema version, evaluated Steel exports, BLAKE3 source/evaluation hash, provenance, sandbox metadata, and redaction metadata.

#### Scenario: Snapshot provenance is reviewable

r[runtime_configuration.hotload_runtime_configuration.normalized_snapshot.scenario]
- GIVEN a process loads config
- WHEN it records startup or reload evidence
- THEN the evidence names the snapshot schema version, Steel module hash, evaluated snapshot hash, source path or overlay identifier, generation command, sandbox profile, and redacted fields
- AND secret-like values are not emitted in cleartext logs

### Requirement: Loader validation

r[runtime_configuration.hotload_runtime_configuration.loader_validation] The loader MUST validate Steel-derived snapshots into typed domain config through a pure functional core before any caller observes the new config.

#### Scenario: Invalid snapshot fails closed

r[runtime_configuration.hotload_runtime_configuration.loader_validation.scenario]
- GIVEN a malformed, stale, unknown-export, missing-export, wrong-type, range-invalid, or capability-invalid snapshot
- WHEN the loader validates it
- THEN validation returns explicit diagnostics
- AND the previously active config remains authoritative

### Requirement: Steel sandbox

r[runtime_configuration.hotload_runtime_configuration.steel_sandbox] Steel evaluation MUST run in an isolated sandbox with no ambient filesystem, network, process, wall-clock, randomness, or live-state mutation capabilities.

#### Scenario: Sandbox violations fail reload

r[runtime_configuration.hotload_runtime_configuration.steel_sandbox.scenario]
- GIVEN a Steel module attempts to use a forbidden capability or nondeterministic input
- WHEN the candidate module is evaluated
- THEN evaluation fails with diagnostics
- AND no config snapshot or policy hook from that module is published

### Requirement: Atomic hot reload

r[runtime_configuration.hotload_runtime_configuration.hot_reload] Hot reload MUST validate before swap, apply through explicit handlers, and preserve the old snapshot if any validation or apply step fails.

#### Scenario: Reload is all-or-nothing

r[runtime_configuration.hotload_runtime_configuration.hot_reload.scenario]
- GIVEN a candidate Steel module changes hot fields
- WHEN reload is requested
- THEN the system compiles/evaluates the module in isolation, computes a redacted diff and apply plan, and applies only hot fields
- AND publishes the candidate snapshot only after every hot apply handler succeeds
- AND rolls back to the previous snapshot with diagnostics if any handler fails

### Requirement: Arrow damage policy

r[runtime_configuration.hotload_runtime_configuration.arrow_damage_policy] Arrow damage MUST be the representative gameplay-rule policy path for the compatibility runner Steel hotloading slice, using an explicit host-provided context and typed Rust-validated decision output. This requirement does not claim Valence/server combat-loop integration.

#### Scenario: Arrow damage hotloads safely

r[runtime_configuration.hotload_runtime_configuration.arrow_damage_policy.scenario]
- GIVEN the Steel arrow-damage policy is edited
- WHEN the policy is hotloaded
- THEN the candidate policy is validated against its context schema, decision schema, sandbox profile, and range bounds
- AND the compatibility runner's projectile-damage evidence rail uses the new policy only after atomic publish succeeds

### Requirement: Reload test and evidence coverage

r[runtime_configuration.hotload_runtime_configuration.reload_tests] The implementation MUST include positive and negative tests for Steel module loading, config normalization, policy decisions, reload planning, hot application, rollback, redaction, and mutability enforcement.

#### Scenario: Verification covers success and failure paths

r[runtime_configuration.hotload_runtime_configuration.reload_tests.scenario]
- GIVEN the reload test suite runs
- WHEN positive and negative cases execute
- THEN valid defaults, module edits, hot reloads, arrow-damage decisions, next-run reporting, restart-only reporting, unknown exports, missing exports, wrong types, invalid ranges, malformed modules, sandbox violations, nondeterminism attempts, apply failures, and redaction are all checked
- AND review-critical receipts or run logs are copied under `docs/evidence/`

### Requirement: Evidence-backed migration

r[runtime_configuration.hotload_runtime_configuration.reload_evidence] A value MUST NOT be marked migrated until inventory, Steel module contract, typed boundary, runtime consumer, tests, and evidence all agree on its config path and reload mutability.

#### Scenario: Migration claim matches evidence

r[runtime_configuration.hotload_runtime_configuration.reload_evidence.scenario]
- GIVEN a task claims a runtime value is Steel-config-managed
- WHEN the checker reviews the inventory, module contract, typed boundary, code annotations or call-site list, and evidence
- THEN the same config path and mutability class appear in all artifacts
- AND unmatched rows remain incomplete tasks rather than archived claims
