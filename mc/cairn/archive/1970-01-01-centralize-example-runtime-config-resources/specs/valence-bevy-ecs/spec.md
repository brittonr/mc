# valence-bevy-ecs Change Spec: Runtime config resources

## Requirements

### Requirement: Runtime config inventory

r[valence_bevy_ecs.runtime_config_resources.inventory] Runtime config resource work MUST inventory selected example env/CLI/config reads, filesystem path inputs, default values, reload triggers, runtime-toggle expectations, and milestone effects before changing ownership.

#### Scenario: Config inputs are reviewable

r[valence_bevy_ecs.runtime_config_resources.inventory.reviewable]
- GIVEN an example or fixture runtime input is selected for resource ownership
- WHEN reviewers inspect the inventory
- THEN each env var, CLI input, path input, default, reload trigger, runtime-toggle expectation, and milestone impact is recorded
- AND inputs that must remain dynamically polled are distinguished from inputs safe to load once.

### Requirement: Runtime config resource contract

r[valence_bevy_ecs.runtime_config_resources.contract] Selected runtime configuration SHOULD be represented by typed Bevy resources backed by pure parser contracts over explicit inputs.

#### Scenario: Parser core is testable

r[valence_bevy_ecs.runtime_config_resources.contract.parser]
- GIVEN selected runtime configuration is parsed
- WHEN pure parser tests invoke the parser with explicit input values
- THEN the parser returns typed configuration or typed errors without reading environment variables, filesystem state, clocks, Bevy resources, or global state.

### Requirement: Runtime config resource wiring

r[valence_bevy_ecs.runtime_config_resources.wiring] Systems selected for runtime config resource work SHOULD consume typed config resources or explicit reload events instead of reading process environment directly.

#### Scenario: Systems consume explicit config

r[valence_bevy_ecs.runtime_config_resources.wiring.resources]
- GIVEN a selected system needs runtime policy
- WHEN the system runs after migration
- THEN it reads the relevant typed resource or reload event
- AND direct environment access remains in startup/reload shell code only.

### Requirement: Runtime config compatibility

r[valence_bevy_ecs.runtime_config_resources.compatibility] Runtime config resource work MUST preserve selected env var names, CLI inputs, default behavior, reload semantics, milestone text, and non-claim boundaries unless another Cairn changes them.

#### Scenario: Config receipts remain comparable

r[valence_bevy_ecs.runtime_config_resources.compatibility.receipts]
- GIVEN a selected compatibility fixture runs after config resource migration
- WHEN its receipts or logs are compared against the pre-migration contract
- THEN required milestones, forbidden milestones, input names, defaults, and reload behavior remain compatible
- AND no production configuration management, broad compatibility, or vanilla parity claim is added.

### Requirement: Runtime config resource tests

r[valence_bevy_ecs.runtime_config_resources.tests] Runtime config resource work MUST include positive config parser/resource tests and negative missing, malformed, conflicting, reload-stale, and disabled-plugin tests for changed inputs.

#### Scenario: Valid config installs resources

r[valence_bevy_ecs.runtime_config_resources.tests.positive]
- GIVEN valid selected runtime inputs
- WHEN parser and plugin/resource installation tests run
- THEN typed resources contain expected values and selected systems observe those values.

#### Scenario: Invalid config fails clearly

r[valence_bevy_ecs.runtime_config_resources.tests.negative]
- GIVEN missing required input, malformed values, conflicting options, stale reload requests, or disabled plugin configuration
- WHEN parser and app tests run
- THEN typed errors, disabled behavior, or diagnostics match the config contract
- AND no false milestone, panic, or stale resource mutation occurs.

### Requirement: Runtime config resource validation

r[valence_bevy_ecs.runtime_config_resources.validation] Runtime config resource work MUST record focused example checks, selected compatibility rails when fixture behavior changes, schedule hygiene when plugin/run-condition wiring changes, Cairn gates, Cairn validation, and task-evidence checks before archive.

#### Scenario: Runtime config closeout is reviewable

r[valence_bevy_ecs.runtime_config_resources.validation.log]
- GIVEN runtime config resource work is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show pure parser tests, positive and negative resource tests, focused example checks, selected mc-compat rails when applicable, schedule hygiene when applicable, Cairn proposal/design/tasks gates, Cairn validation, and task-evidence validation.
