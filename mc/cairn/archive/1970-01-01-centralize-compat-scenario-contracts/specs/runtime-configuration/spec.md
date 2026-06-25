# runtime-configuration Change Spec: Scenario environment contracts

## Requirements

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
