# valence-bevy-ecs Change Spec: Remaining example plugin organization

## Requirements

### Requirement: Remaining example inventory

r[valence_bevy_ecs.remaining_example_plugins.inventory] Remaining example plugin work MUST inventory selected example systems, schedules, resources, events, env/CLI inputs, pure helpers, visible behavior, and non-goals before refactoring wiring.

#### Scenario: Example baseline is reviewable

r[valence_bevy_ecs.remaining_example_plugins.inventory.reviewable]
- GIVEN a remaining Valence example is selected for plugin organization
- WHEN reviewers inspect the inventory
- THEN each current system, schedule label, resource, event, input contract, pure helper, visible behavior, and evidence boundary is recorded
- AND default Valence behavior, vanilla parity, and production readiness remain explicit non-claims.

### Requirement: Remaining example plugin contract

r[valence_bevy_ecs.remaining_example_plugins.contract] Extracted remaining example plugins SHOULD expose named Bevy `SystemSet`s for stable input, rule evaluation, world mutation, presentation, and cleanup ordering where those phases exist.

#### Scenario: Example phases are orderable

r[valence_bevy_ecs.remaining_example_plugins.contract.phases]
- GIVEN an extracted remaining example plugin registers systems
- WHEN reviewers inspect its schedule contract
- THEN systems are grouped into documented phase sets
- AND user code can order around those sets without depending on anonymous tuple order.

### Requirement: Remaining example plugin wiring

r[valence_bevy_ecs.remaining_example_plugins.wiring] Remaining example plugins MUST keep deterministic gameplay and example decisions outside Bevy ECS access unless the code is only an adapter shell.

#### Scenario: Example plugin remains a shell

r[valence_bevy_ecs.remaining_example_plugins.wiring.shell]
- GIVEN an example decision is migrated during plugin organization
- WHEN the implementation is reviewed
- THEN pure decisions consume explicit inputs and return decisions or mutation requests
- AND Bevy queries, commands, resources, logging, file I/O, and world mutation remain in thin systems.

### Requirement: Remaining example compatibility

r[valence_bevy_ecs.remaining_example_plugins.compatibility] Remaining example plugin organization MUST preserve selected example commands, CLI/env contracts, visible behavior, documentation boundaries, and non-claim scope unless another Cairn changes them.

#### Scenario: Example behavior remains comparable

r[valence_bevy_ecs.remaining_example_plugins.compatibility.behavior]
- GIVEN a selected example runs after plugin organization
- WHEN its focused checks or smoke behavior are compared against the pre-refactor contract
- THEN command names, input contracts, visible behavior, and non-claim fields remain compatible
- AND no default gameplay, vanilla parity, or production-readiness claim is added.

### Requirement: Remaining example plugin tests

r[valence_bevy_ecs.remaining_example_plugins.tests] Remaining example plugin organization MUST include positive plugin/schedule smoke tests and negative disabled-plugin or ordering-regression tests.

#### Scenario: Example plugin smoke passes

r[valence_bevy_ecs.remaining_example_plugins.tests.positive]
- GIVEN an extracted example plugin is added to a minimal test app
- WHEN schedules are initialized
- THEN required resources, events, system sets, and schedule labels are present.

#### Scenario: Disabled example plugin is absent

r[valence_bevy_ecs.remaining_example_plugins.tests.negative]
- GIVEN the extracted example plugin is not added to a minimal test app
- WHEN schedules are inspected or updated
- THEN plugin-owned resources, events, systems, and sets are absent
- AND no example-owned milestone or visible behavior can be emitted by that plugin.

### Requirement: Remaining example plugin validation

r[valence_bevy_ecs.remaining_example_plugins.validation] Remaining example plugin organization MUST record focused example checks, Valence schedule hygiene, selected smoke runs when behavior changes, Cairn gates, Cairn validation, and task-evidence checks before archive.

#### Scenario: Example plugin closeout is reviewable

r[valence_bevy_ecs.remaining_example_plugins.validation.log]
- GIVEN remaining example plugin organization is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show focused example checks, positive and negative plugin tests, schedule hygiene, selected smoke runs when applicable, Cairn proposal/design/tasks gates, Cairn validation, and task-evidence validation.
