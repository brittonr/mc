# valence-bevy-ecs Change Spec: Gameplay example plugin organization

## Requirements

### Requirement: Gameplay plugin inventory

r[valence_bevy_ecs.gameplay_plugins.inventory] Valence gameplay example plugin work MUST inventory selected example Bevy systems, schedules, resources, events, env toggles, compatibility milestones, and non-goals before refactoring wiring.

#### Scenario: Example wiring is reviewable

r[valence_bevy_ecs.gameplay_plugins.inventory.reviewable]
- GIVEN a Valence gameplay or compatibility example is selected for plugin organization
- WHEN reviewers inspect the inventory
- THEN each current system, schedule label, resource, event, env toggle, milestone emitter, and evidence boundary is classified
- AND production gameplay, vanilla parity, and default Valence behavior remain explicit non-claims unless separately scoped.

### Requirement: Gameplay plugin contract

r[valence_bevy_ecs.gameplay_plugins.contract] Extracted gameplay example plugins SHOULD expose named Bevy `SystemSet`s for stable input, rule evaluation, world mutation, presentation, and cleanup ordering.

#### Scenario: Sets describe schedule phases

r[valence_bevy_ecs.gameplay_plugins.contract.phases]
- GIVEN an extracted example plugin registers systems
- WHEN reviewers inspect its schedule contract
- THEN the systems are grouped into documented phase sets
- AND user code can order around those sets without depending on anonymous tuple order.

### Requirement: Gameplay plugin wiring

r[valence_bevy_ecs.gameplay_plugins.wiring] Example plugins MUST keep deterministic gameplay and compatibility decisions outside Bevy ECS access unless the code is only an adapter shell.

#### Scenario: Plugin remains a shell

r[valence_bevy_ecs.gameplay_plugins.wiring.shell]
- GIVEN a CTF, survival, or terrain decision is migrated during plugin organization
- WHEN the implementation is reviewed
- THEN pure decisions consume explicit inputs and return decisions or mutation requests
- AND Bevy queries, commands, resources, logging, file I/O, and world mutation remain in thin systems.

### Requirement: Gameplay compatibility preservation

r[valence_bevy_ecs.gameplay_plugins.compatibility] Plugin organization MUST preserve selected example commands, env var contracts, milestone text, scenario behavior, and evidence non-claim boundaries unless another Cairn changes them.

#### Scenario: Fixture receipts remain comparable

r[valence_bevy_ecs.gameplay_plugins.compatibility.receipts]
- GIVEN selected compatibility scenarios run after plugin organization
- WHEN their receipts and logs are compared against the pre-refactor contract
- THEN required milestones, forbidden milestones, env toggles, and non-claim fields remain compatible
- AND no default Valence gameplay, production-readiness, or vanilla-parity claim is added.

### Requirement: Gameplay plugin tests

r[valence_bevy_ecs.gameplay_plugins.tests] Gameplay plugin organization MUST include positive plugin/schedule smoke tests and negative disabled-plugin or ordering regression tests.

#### Scenario: Positive plugin smoke passes

r[valence_bevy_ecs.gameplay_plugins.tests.positive]
- GIVEN an extracted example plugin is added to a minimal test app
- WHEN the app schedules are initialized
- THEN required resources, events, system sets, and schedule labels are present.

#### Scenario: Disabled plugin does not install behavior

r[valence_bevy_ecs.gameplay_plugins.tests.negative]
- GIVEN the extracted plugin is not added to a minimal test app
- WHEN the app schedules are inspected or updated
- THEN plugin-owned resources, events, and gameplay systems are absent
- AND no compatibility milestone can be emitted by that plugin.

### Requirement: Gameplay plugin validation

r[valence_bevy_ecs.gameplay_plugins.validation] Gameplay plugin organization MUST record focused example checks, selected compatibility rails when touched, Cairn gates, Cairn validation, and task-evidence checks before archive.

#### Scenario: Plugin closeout is reviewable

r[valence_bevy_ecs.gameplay_plugins.validation.log]
- GIVEN gameplay plugin organization is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show focused example checks, positive and negative plugin tests, selected mc-compat dry-runs if fixture behavior changed, Cairn proposal/design/tasks gates, Cairn validation, and task-evidence validation.
