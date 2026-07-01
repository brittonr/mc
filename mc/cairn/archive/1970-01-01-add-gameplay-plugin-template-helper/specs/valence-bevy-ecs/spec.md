# valence-bevy-ecs Change Spec: Gameplay plugin template helper

## Requirements

### Requirement: Gameplay plugin template inventory

r[valence_bevy_ecs.plugin_template_helper.inventory] Gameplay plugin template-helper work MUST inventory duplicated Valence example plugin boilerplate, phase enums, contract resources, metadata registration, schedule wiring, install tests, disabled-plugin tests, and remaining local-only patterns before introducing shared helpers.

#### Scenario: Boilerplate duplication is reviewable

r[valence_bevy_ecs.plugin_template_helper.inventory.reviewable]
- GIVEN gameplay plugin template-helper work is selected
- WHEN reviewers inspect the inventory
- THEN selected examples with duplicated phase, contract, schedule, registration, setup, and test patterns are recorded
- AND examples that should remain local or out of scope are classified explicitly.

### Requirement: Gameplay plugin template helper API

r[valence_bevy_ecs.plugin_template_helper.api] Valence gameplay plugin helpers SHOULD provide explicit reusable functions or types for contract descriptors, phase wiring, metadata registration, and install/disabled test assertions without hiding gameplay logic or ECS mutation.

#### Scenario: Helper registers common metadata explicitly

r[valence_bevy_ecs.plugin_template_helper.api.registration]
- GIVEN a new or migrated example plugin provides a descriptor with plugin identity, schedule phases, owned resources, events, scope model, install mode, and non-claims
- WHEN the helper registers the plugin contract
- THEN shared metadata and phase assertions are installed consistently
- AND gameplay decisions, resource mutation, packet IO, logging, and compatibility milestone emission remain in plugin-owned shell systems or pure cores.

### Requirement: Gameplay plugin template migration

r[valence_bevy_ecs.plugin_template_helper.migration] Selected Valence examples MAY migrate repeated contract and schedule boilerplate to the helper only when behavior, commands, env/CLI inputs, compatibility milestones, and default Valence plugin behavior remain compatible.

#### Scenario: Migrated example behavior remains unchanged

r[valence_bevy_ecs.plugin_template_helper.migration.compatible]
- GIVEN a selected example migrates to the plugin template helper
- WHEN focused example and contract checks run
- THEN the same resources, events, schedule phases, commands, env/CLI contracts, and compatibility milestones remain visible
- AND no default gameplay, vanilla parity, dynamic plugin loading, or production-readiness claim is added.

### Requirement: Gameplay plugin template documentation

r[valence_bevy_ecs.plugin_template_helper.documentation] Gameplay plugin template-helper work MUST document the new-plugin workflow, required contract metadata, required positive and negative tests, migration guidance, and non-claim boundaries.

#### Scenario: New plugin workflow is clear

r[valence_bevy_ecs.plugin_template_helper.documentation.workflow]
- GIVEN a developer wants to add a new compiled Bevy gameplay/example plugin
- WHEN they read the helper documentation or template example
- THEN they can identify the descriptor fields, phase wiring, registration call, install test, disabled-plugin test, and evidence expectations
- AND they understand that runtime-loaded plugins, scripting, sandboxing, default Valence gameplay, and vanilla parity are not promised.

### Requirement: Gameplay plugin template tests

r[valence_bevy_ecs.plugin_template_helper.tests] Gameplay plugin template-helper work MUST include positive helper/plugin install tests and negative missing-contract, disabled-plugin, stale-metadata, duplicate-registration, ordering-regression, and hidden-mutation tests where feasible.

#### Scenario: Helper-installed plugin passes contract checks

r[valence_bevy_ecs.plugin_template_helper.tests.positive]
- GIVEN a selected plugin uses the template helper in a minimal test app
- WHEN helper assertions inspect schedules and resources
- THEN expected contract metadata, phase sets, resources, events, and schedule labels are present.

#### Scenario: Template misuse fails clearly

r[valence_bevy_ecs.plugin_template_helper.tests.negative]
- GIVEN a plugin omits required descriptor fields, registers duplicate metadata, is disabled, declares stale resources/events, or relies on hidden template mutation
- WHEN helper tests run
- THEN the failure names the missing or invalid plugin contract fact
- AND no false installed-plugin contract or hidden gameplay mutation is accepted.

### Requirement: Gameplay plugin template validation

r[valence_bevy_ecs.plugin_template_helper.validation] Gameplay plugin template-helper work MUST record focused helper tests, selected migrated-example checks, positive and negative plugin tests, Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest checks before archive.

#### Scenario: Template helper closeout is reviewable

r[valence_bevy_ecs.plugin_template_helper.validation.log]
- GIVEN gameplay plugin template-helper work is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show inventory, helper API checks, selected migration checks, positive and negative helper tests, selected example checks, Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, and refreshed evidence manifests.
