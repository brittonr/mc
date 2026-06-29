# valence-bevy-ecs Change Spec: Command manager modules

## Requirements

### Requirement: Command manager boundaries

r[valence_bevy_ecs.command_manager.boundaries] Valence command manager code SHOULD expose cohesive boundaries for plugin wiring, packet adapters, command tree synchronization, parse core, execution event planning, and Bevy systems.

#### Scenario: Command responsibility has one owner

r[valence_bevy_ecs.command_manager.boundaries.ownership]
- GIVEN a command manager responsibility is reviewed
- WHEN maintainers inspect command manager modules
- THEN the responsibility is owned by a focused module
- AND unrelated packet, tree, parse, event, plugin, and system concerns are not reintroduced into one module.

### Requirement: Command manager core

r[valence_bevy_ecs.command_manager.core] Packet-to-command event conversion, command tree update requirements, command parse outcomes, argument parse plans, and processed-event plans SHOULD be pure over explicit inputs.

#### Scenario: Command decision is testable without Bevy

r[valence_bevy_ecs.command_manager.core.testable]
- GIVEN packet, command graph, client scope, or command text summaries
- WHEN the command core processes them
- THEN the decision can be tested without Bevy queries, resources, events, packet sends, or schedule wiring.

### Requirement: Command manager parity

r[valence_bevy_ecs.command_manager.parity] Command manager modularization MUST preserve public command APIs, event shapes, command tree behavior, parsing behavior, schedule behavior, and evidence non-claims.

#### Scenario: Command behavior remains stable

r[valence_bevy_ecs.command_manager.parity.stable]
- GIVEN a supported pre-refactor command manager input
- WHEN the modularized command manager processes the same input
- THEN packet adapter, tree sync, parse, event, schedule, and non-claim behavior remain equivalent.

### Requirement: Command manager positive tests

r[valence_bevy_ecs.command_manager.positive_tests] The change MUST include positive tests for packet adapter events, command tree update decisions, valid command parse, argument parse plans, processed events, and plugin wiring facts.

#### Scenario: Supported command paths pass

r[valence_bevy_ecs.command_manager.positive_tests.coverage]
- GIVEN representative supported command manager inputs
- WHEN extracted command cores process them
- THEN tests prove the expected events, parse results, tree updates, or plans are produced.

### Requirement: Command manager negative tests

r[valence_bevy_ecs.command_manager.negative_tests] The change MUST include negative tests for malformed command packets, unknown commands, invalid arguments, stale command trees, missing scopes, and disabled clients.

#### Scenario: Invalid command paths fail closed

r[valence_bevy_ecs.command_manager.negative_tests.fail_closed]
- GIVEN invalid command manager inputs
- WHEN extracted command cores or shells process them
- THEN tests prove the inputs are rejected, ignored, or diagnosed according to current behavior.

### Requirement: Command manager validation

r[valence_bevy_ecs.command_manager.validation] The change MUST record focused command tests, affected examples/checks, Cairn proposal/design/tasks gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[valence_bevy_ecs.command_manager.validation.logs]
- GIVEN command manager modularization is complete
- WHEN the change is closed
- THEN reviewable logs show positive and negative command tests plus affected checks and Cairn gates passing.
