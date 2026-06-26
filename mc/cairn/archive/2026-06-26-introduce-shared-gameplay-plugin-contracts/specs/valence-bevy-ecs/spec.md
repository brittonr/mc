# valence-bevy-ecs Change Spec: Shared gameplay plugin composition contracts

## Requirements

### Requirement: Gameplay plugin contract inventory

r[valence_bevy_ecs.gameplay_plugin_contracts.inventory] Shared gameplay plugin contract work MUST inventory selected gameplay/example plugin phase sets, contract resources, schedule labels, resources, events, disabled-plugin tests, and private ordering points before changing shared wiring.

#### Scenario: Existing plugin contracts are reviewable

r[valence_bevy_ecs.gameplay_plugin_contracts.inventory.reviewable]
- GIVEN a gameplay or example plugin is selected for shared contract work
- WHEN reviewers inspect the inventory
- THEN current phase names, schedule labels, owned resources, events, plugin install mode, disabled-plugin behavior, and private ordering points are recorded
- AND CTF, survival compatibility, terrain, smaller example plugins, and out-of-scope BedWars/Hyperion boundaries are classified explicitly.

### Requirement: Shared gameplay phase contract

r[valence_bevy_ecs.gameplay_plugin_contracts.phase_contract] Opt-in Valence gameplay plugins SHOULD use a shared phase vocabulary for input, rule evaluation, world mutation, presentation, and cleanup where those phases exist.

#### Scenario: Plugins can order around shared phases

r[valence_bevy_ecs.gameplay_plugin_contracts.phase_contract.orderable]
- GIVEN multiple opt-in gameplay plugins are installed in one app
- WHEN a downstream system needs to run around gameplay input, rule evaluation, world mutation, presentation, or cleanup
- THEN it can target the shared phase contract instead of relying on plugin-local anonymous tuple order
- AND plugin-local subphases remain private unless deliberately promoted.

### Requirement: Gameplay plugin contract metadata

r[valence_bevy_ecs.gameplay_plugin_contracts.metadata] Shared gameplay plugin contracts MUST expose or record minimal metadata for schedule labels, phase order, owned resources, owned events, scope model, installation mode, and non-claim boundaries.

#### Scenario: Contract metadata explains installed behavior

r[valence_bevy_ecs.gameplay_plugin_contracts.metadata.inspectable]
- GIVEN a gameplay plugin is installed in a minimal test app
- WHEN tests or reviewers inspect its contract metadata
- THEN the contract names installed schedules, phase order, owned resources, owned events, expected gameplay scope model, and whether the plugin is default, feature-gated, or explicitly opt-in
- AND non-claims for dynamic plugins, default gameplay, vanilla parity, production readiness, and BedWars/Hyperion scope are visible.

### Requirement: Gameplay plugin contract tests

r[valence_bevy_ecs.gameplay_plugin_contracts.tests] Shared gameplay plugin contract work MUST include reusable positive contract tests and negative disabled-plugin or ordering-regression tests for selected plugins.

#### Scenario: Installed plugin passes shared contract checks

r[valence_bevy_ecs.gameplay_plugin_contracts.tests.positive]
- GIVEN a selected gameplay plugin is added to a minimal test app with required schedules
- WHEN shared contract helpers inspect schedules and resources
- THEN expected phase sets, contract metadata, resources, events, and schedule labels are present.

#### Scenario: Missing plugin fails closed

r[valence_bevy_ecs.gameplay_plugin_contracts.tests.negative]
- GIVEN a selected gameplay plugin is not added or an ordering fixture omits a required phase
- WHEN shared contract helpers inspect the app
- THEN plugin-owned resources, events, systems, sets, and contract metadata are absent or the ordering failure is diagnosed clearly
- AND no false gameplay contract is reported as installed.

### Requirement: Gameplay plugin compatibility boundaries

r[valence_bevy_ecs.gameplay_plugin_contracts.compatibility] Shared gameplay plugin contracts MUST preserve example behavior, command/env/CLI contracts, compatibility milestones, and non-claim scope unless another Cairn changes them.

#### Scenario: Shared contracts do not promote gameplay claims

r[valence_bevy_ecs.gameplay_plugin_contracts.compatibility.non_claims]
- GIVEN an example plugin adopts the shared gameplay contract
- WHEN its behavior and evidence boundaries are reviewed
- THEN existing commands, env/CLI inputs, compatibility milestones, and visible behavior remain comparable
- AND no dynamic plugin loading, default Valence gameplay, BedWars scope, vanilla parity, production readiness, or public-server safety claim is added.

### Requirement: Gameplay plugin contract validation

r[valence_bevy_ecs.gameplay_plugin_contracts.validation] Shared gameplay plugin contract work MUST record focused gameplay/example checks, shared test-helper checks, Valence schedule hygiene, Cairn gates, Cairn validation, task-evidence validation, and evidence manifests before archive.

#### Scenario: Shared contract closeout is reviewable

r[valence_bevy_ecs.gameplay_plugin_contracts.validation.log]
- GIVEN shared gameplay plugin contract work is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show focused gameplay/example checks, positive and negative contract tests, schedule hygiene, Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, and refreshed evidence manifests.
