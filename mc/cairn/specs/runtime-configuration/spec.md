# Runtime Configuration Specification

## Purpose

Defines the `runtime-configuration` capability.

## Requirements

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

### Requirement: Steel value decoder trait contract

r[runtime_configuration.steel_value_decoder_traits.contract] Runtime configuration code MUST define a bounded Steel value decoder trait contract before replacing repeated typed export helper functions.

#### Scenario: Decoder scope is limited to existing values

r[runtime_configuration.steel_value_decoder_traits.contract.scope]
- GIVEN the runtime loader decodes restricted Steel-compatible literal exports
- WHEN reviewers inspect the decoder contract
- THEN it lists the accepted target Rust types and existing `SteelValue` variants
- AND it does not add new Steel syntax, new sandbox capabilities, new exports, new policy hooks, or new snapshot schema fields.

### Requirement: Pure decoder core

r[runtime_configuration.steel_value_decoder_traits.core] Steel value decoder implementations MUST be pure deterministic conversions from `SteelValue` references to explicit Rust target types.

#### Scenario: Decoder conversion has no side effects

r[runtime_configuration.steel_value_decoder_traits.core.pure]
- GIVEN a decoder receives a `SteelValue` and an expected target type
- WHEN conversion runs for string, string-list, u32, or f64
- THEN it returns the converted value or a typed mismatch
- AND it does not read files, evaluate Steel code, spawn processes, inspect environment, use clocks, perform network access, or mutate runtime state.

### Requirement: Runtime config migration

r[runtime_configuration.steel_value_decoder_traits.migration] Runtime config normalization SHOULD use one generic typed required-export helper once decoder parity tests exist.

#### Scenario: Domain validation remains separate

r[runtime_configuration.steel_value_decoder_traits.migration.validation]
- GIVEN a required export decodes successfully
- WHEN runtime config normalization continues
- THEN schema-version checks, backend parsing, port bounds, timeout bounds, arrow damage range checks, mutability classification, sandbox checks, and snapshot provenance checks remain separate from the decoder trait.

### Requirement: Decoder tests

r[runtime_configuration.steel_value_decoder_traits.tests] The decoder refactor MUST include positive and negative tests for every supported decoded type and failure mode.

#### Scenario: Valid typed exports pass

r[runtime_configuration.steel_value_decoder_traits.tests.positive]
- GIVEN valid string, string-list, u32, and f64 exports are present in the parsed literal export map
- WHEN the generic required-export helper decodes them
- THEN each value is returned with the same value that the pre-refactor helper returned.

#### Scenario: Invalid typed exports fail closed

r[runtime_configuration.steel_value_decoder_traits.tests.negative]
- GIVEN an export is missing, has the wrong `SteelValue` variant, is malformed before decoding, is unsupported, or decodes to a value that later violates domain range checks
- WHEN loader tests run
- THEN diagnostics preserve the configured path and expected type
- AND no invalid candidate snapshot becomes active.

### Requirement: Runtime config regression coverage

r[runtime_configuration.steel_value_decoder_traits.regression] Existing sandbox, mutability, snapshot, and reload-planning behavior MUST remain covered after decoder migration.

#### Scenario: Existing invalid module behavior is preserved

r[runtime_configuration.steel_value_decoder_traits.regression.invalid]
- GIVEN a Steel module uses forbidden capabilities, unknown exports, missing required exports, malformed literals, wrong types, invalid ranges, fixed-protocol changes, or apply failures
- WHEN runtime config tests and checkers run
- THEN the previous snapshot or default policy remains authoritative
- AND diagnostics remain reviewable and non-secret.

### Requirement: Decoder validation

r[runtime_configuration.steel_value_decoder_traits.validation] The change MUST record runtime config tests, runtime Steel config checker output, and Cairn gates before archive.

#### Scenario: Decoder closeout is reviewable

r[runtime_configuration.steel_value_decoder_traits.validation.log]
- GIVEN Steel value decoder traits are implemented
- WHEN the change is archived
- THEN successful logs show decoder positive tests, decoder negative tests, runtime config regression tests, runtime Steel config checker output, Cairn proposal/design/tasks gates, and Cairn validation.

### Requirement: Valence combat-loop arrow policy

r[runtime_configuration.valence_combat_loop_steel_policy.arrow_damage_live] Valence CTF projectile-probe combat MUST source arrow damage from the latest Rust-validated policy snapshot after atomic publish succeeds. For this change, the Valence side accepts a restricted Steel-compatible literal module subset and does not claim full Steel evaluator execution.

#### Scenario: Published policy drives Valence projectile combat

r[runtime_configuration.valence_combat_loop_steel_policy.arrow_damage_live.scenario]
- GIVEN a valid Steel-compatible arrow-damage policy snapshot is published
- WHEN the Valence CTF projectile-probe combat paths apply projectile damage
- THEN both the combat-event path and the projectile-interaction path use the published policy decision
- AND milestone/evidence output records the policy id, snapshot generation or hash, damage, clamped flag, and victim health delta
- AND the old projectile damage constant is used only as the default policy input before an operator override is published

### Requirement: Atomic Valence policy publish

r[runtime_configuration.valence_combat_loop_steel_policy.atomic_publish] Valence policy reload MUST publish a candidate arrow policy only after restricted Steel-compatible literal normalization, capability-token rejection, typed validation, decision validation, and apply preparation all succeed.

#### Scenario: Invalid reload preserves active combat policy

r[runtime_configuration.valence_combat_loop_steel_policy.atomic_publish.scenario]
- GIVEN an active Valence arrow policy snapshot is serving combat decisions
- WHEN a candidate policy module is malformed, capability-token-invalid, type-invalid, range-invalid, or fails representative decision validation
- THEN reload returns diagnostics without publishing the candidate
- AND subsequent Valence projectile-probe combat uses the previous active snapshot
- AND evidence records the rejection reason without leaking secret-like values

### Requirement: Valence policy evidence coverage

r[runtime_configuration.valence_combat_loop_steel_policy.evidence] A Valence combat-loop Steel-compatible policy migration MUST NOT be marked complete until tests, checker output, and reviewable live or live-equivalent evidence prove the server call sites use the published policy.

#### Scenario: Evidence ties config to live call sites

r[runtime_configuration.valence_combat_loop_steel_policy.evidence.scenario]
- GIVEN tasks claim Valence combat-loop arrow damage is Steel-managed
- WHEN the checker reviews inventory, Steel-compatible exports, typed Rust boundary, Valence call-site list, milestone receipt, and BLAKE3 evidence manifest
- THEN each artifact names the same `combat.arrow.*` config paths, hot mutability class, and Valence projectile-probe consumers
- AND unmatched call sites or missing receipts keep the task incomplete

### Requirement: Scenario contract inventory

r[runtime_configuration.scenario_env_contracts.inventory] The repository MUST inventory compatibility scenario names, aliases, probe environment variables, fixture toggles, milestone labels, receipt identifiers, and their consumers before centralizing them.

#### Scenario: Contract consumers are known

r[runtime_configuration.scenario_env_contracts.inventory.consumers]
- GIVEN a scenario contract value is used by the runner, Stevenarella, Valence fixtures, generated manifests, or evidence checks
- WHEN the inventory is reviewed
- THEN the value records owner, current spelling, consumer path, compatibility status, and migration action
- AND historical aliases are separated from canonical contract values.

### Requirement: Typed scenario contract source

r[runtime_configuration.scenario_env_contracts.contract] Compatibility scenario identifiers SHOULD be authored in a typed source of truth that records schema version, canonical names, aliases, env vars, fixture ownership, milestone IDs, and non-claim boundaries.

#### Scenario: Contract is explicit

r[runtime_configuration.scenario_env_contracts.contract.explicit]
- GIVEN maintainers add or update a scenario contract value
- WHEN the typed contract is evaluated or generated
- THEN required fields, duplicate names, duplicate env vars, unknown fixture owners, and unsupported aliases are validated
- AND compatibility aliases remain explicit rather than inferred from ad hoc strings.

### Requirement: Generated runtime surface

r[runtime_configuration.scenario_env_contracts.generated_surface] Runtime Rust code MUST consume checked-in generated/static contract data or validated constants rather than evaluating Nickel at runtime.

#### Scenario: Runtime remains Nickel-free

r[runtime_configuration.scenario_env_contracts.generated_surface.runtime_free]
- GIVEN runner, Stevenarella, or Valence fixture code uses scenario contract values
- WHEN the code is built or checked
- THEN it uses generated Rust/static data or constants whose freshness is validated
- AND runtime code does not read Nickel files, invoke Nickel, or perform schema generation during normal compatibility runs.

### Requirement: Consumer migration

r[runtime_configuration.scenario_env_contracts.consumer_migration] Scenario contract consumers SHOULD migrate from duplicated string constants to the shared contract surface or a drift checker in small compatibility-preserving slices.

#### Scenario: Consumer behavior is preserved

r[runtime_configuration.scenario_env_contracts.consumer_migration.parity]
- GIVEN a runner, Stevenarella, or Valence consumer is migrated
- WHEN existing scenario dry-runs and focused consumer tests run
- THEN the same env vars are set/read, the same milestones are required/emitted, and the same receipt fields are produced
- AND no new compatibility or public-server claim is introduced.

### Requirement: Scenario contract tests

r[runtime_configuration.scenario_env_contracts.tests] The scenario contract system MUST include positive aligned-contract fixtures and negative drift fixtures.

#### Scenario: Valid contract passes

r[runtime_configuration.scenario_env_contracts.tests.positive]
- GIVEN canonical scenarios, env vars, fixture toggles, aliases, and milestones are declared and consumed consistently
- WHEN contract freshness and drift checks run
- THEN generated/static outputs match the source contract and all consumers pass.

#### Scenario: Drift fails closed

r[runtime_configuration.scenario_env_contracts.tests.negative]
- GIVEN a consumer uses a missing env var, duplicate identifier, stale alias, mismatched milestone, undeclared fixture toggle, or retired contract value
- WHEN drift checks run
- THEN diagnostics identify the value and consumer
- AND the repository cannot archive the migration as complete until drift is fixed or explicitly waived.

### Requirement: Scenario contract validation

r[runtime_configuration.scenario_env_contracts.validation] Scenario contract work MUST record contract freshness checks, positive and negative drift fixtures, affected component tests, selected dry-runs, Cairn gates, and task-evidence checks before archive.

#### Scenario: Contract closeout is reviewable

r[runtime_configuration.scenario_env_contracts.validation.log]
- GIVEN scenario contract centralization is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show contract generation/freshness, positive and negative drift fixtures, affected runner/client/server checks, selected dry-runs, Cairn proposal/design/tasks gates, Cairn validation, and task-evidence validation.
