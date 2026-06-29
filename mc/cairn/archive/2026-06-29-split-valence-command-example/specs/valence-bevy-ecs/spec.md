# valence-bevy-ecs Change Spec: Command example split

## Requirements

### Requirement: Command example boundaries

r[valence_bevy_ecs.command_example.boundaries] The Valence command example SHOULD expose clear boundaries for app setup, command definitions, handler logic, fixture/test helpers, and explanatory documentation comments.

#### Scenario: Command example responsibility has one owner

r[valence_bevy_ecs.command_example.boundaries.ownership]
- GIVEN a command example responsibility is reviewed
- WHEN maintainers inspect the example
- THEN the responsibility is owned by a focused module or section
- AND unrelated setup, definition, handler, fixture, and docs concerns are not interleaved without need.

### Requirement: Command example handler core

r[valence_bevy_ecs.command_example.handler_core] Non-trivial command definition and handler decisions in the example SHOULD be pure or return explicit outcomes where practical.

#### Scenario: Command example handler is testable without full app

r[valence_bevy_ecs.command_example.handler_core.testable]
- GIVEN command arguments or fixture summaries
- WHEN the handler core processes them
- THEN the result can be tested without full Bevy app setup, client mutation, packet side effects, or logging.

### Requirement: Command example parity

r[valence_bevy_ecs.command_example.parity] Command example splitting MUST preserve example behavior, documented command API usage, handler outcomes, tests, and evidence non-claims.

#### Scenario: Command example behavior remains stable

r[valence_bevy_ecs.command_example.parity.stable]
- GIVEN a supported pre-refactor command example input
- WHEN the split example processes the same input
- THEN command definitions, handler outcomes, documentation intent, and tests remain equivalent.

### Requirement: Command example positive tests

r[valence_bevy_ecs.command_example.positive_tests] The change MUST include positive tests for representative command definitions, handler outcomes, fixture setup, and documented example paths.

#### Scenario: Supported command example paths pass

r[valence_bevy_ecs.command_example.positive_tests.coverage]
- GIVEN representative supported command example inputs
- WHEN extracted example helpers process them
- THEN tests prove the expected definitions, handler outcomes, and fixtures are produced.

### Requirement: Command example negative tests

r[valence_bevy_ecs.command_example.negative_tests] The change MUST include negative tests for invalid command inputs, missing entities, bad arguments, disabled handlers, and malformed fixture state.

#### Scenario: Invalid command example paths fail closed

r[valence_bevy_ecs.command_example.negative_tests.fail_closed]
- GIVEN invalid command example inputs
- WHEN extracted example helpers process them
- THEN tests prove the inputs are rejected, ignored, or diagnosed according to current behavior.

### Requirement: Command example validation

r[valence_bevy_ecs.command_example.validation] The change MUST record focused command example tests, affected example checks, Cairn proposal/design/tasks gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[valence_bevy_ecs.command_example.validation.logs]
- GIVEN command example splitting is complete
- WHEN the change is closed
- THEN reviewable logs show positive and negative command example tests plus affected checks and Cairn gates passing.
