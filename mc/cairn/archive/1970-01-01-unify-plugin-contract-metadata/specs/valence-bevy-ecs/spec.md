# valence-bevy-ecs Change Spec: Unified plugin contract metadata

## Requirements

### Requirement: Plugin contract metadata inventory

r[valence_bevy_ecs.plugin_contract_metadata.inventory] Valence plugin contract metadata work MUST inventory current gameplay/example contract helpers, schedule labels, phase sets, owned resources, owned events, install modes, scope models, non-claims, disabled-plugin behavior, and Hyperion comparison fields before changing shared contract wiring.

#### Scenario: Valence metadata baseline is reviewable

r[valence_bevy_ecs.plugin_contract_metadata.inventory.reviewable]
- GIVEN unified plugin contract metadata work is selected
- WHEN reviewers inspect the inventory
- THEN `GameplayPluginContract`, `GameplayPluginContracts`, selected CTF, survival, terrain, and smaller example contract fields are recorded
- AND Valence-only, Hyperion-comparable, private, and out-of-scope dynamic-plugin fields are classified explicitly.

### Requirement: Plugin contract metadata vocabulary

r[valence_bevy_ecs.plugin_contract_metadata.vocabulary] Valence gameplay/plugin contracts SHOULD use a minimal shared vocabulary for plugin identity, install mode, scope model, schedule labels, phase order, dependencies, owned resources, owned events, compatibility boundaries, and non-claims where those concepts apply.

#### Scenario: Contract fields answer common plugin questions

r[valence_bevy_ecs.plugin_contract_metadata.vocabulary.common]
- GIVEN a selected Valence gameplay plugin exposes contract metadata
- WHEN downstream tests or reviewers inspect the contract
- THEN they can identify what plugin was installed, what phases are orderable, what schedules are affected, what resources/events are owned, what scope model applies, and what behavior is not claimed
- AND plugin-local subphases remain private unless deliberately promoted.

### Requirement: Plugin contract metadata adapters

r[valence_bevy_ecs.plugin_contract_metadata.adapters] Valence MUST expose unified contract metadata through Valence-local helper types or adapters without introducing a hard dependency on Hyperion crates.

#### Scenario: Valence adapter stays engine-local

r[valence_bevy_ecs.plugin_contract_metadata.adapters.local]
- GIVEN unified metadata is added to Valence examples or helpers
- WHEN the implementation is reviewed
- THEN Valence contract data is produced from Valence-local schedules, resources, events, scopes, and examples
- AND Hyperion comparison remains documentation or test vocabulary rather than a required runtime dependency.

### Requirement: Plugin contract metadata compatibility

r[valence_bevy_ecs.plugin_contract_metadata.compatibility] Unified Valence plugin contract metadata MUST preserve selected example behavior, commands, env/CLI contracts, compatibility milestones, disabled-plugin behavior, and existing non-claim boundaries unless another Cairn changes them.

#### Scenario: Metadata does not promote behavior claims

r[valence_bevy_ecs.plugin_contract_metadata.compatibility.non_claims]
- GIVEN a Valence example plugin adopts unified metadata
- WHEN receipts, docs, or tests inspect the plugin contract
- THEN existing behavior and compatibility evidence remain comparable
- AND no dynamic plugin loading, default Valence gameplay, vanilla parity, production readiness, Hyperion runtime dependency, or public-server safety claim is added.

### Requirement: Plugin contract metadata tests

r[valence_bevy_ecs.plugin_contract_metadata.tests] Valence plugin contract metadata work MUST include positive contract-inspection tests and negative missing-contract, stale-contract, disabled-plugin, and ordering-regression tests for selected plugins.

#### Scenario: Installed Valence plugin metadata matches wiring

r[valence_bevy_ecs.plugin_contract_metadata.tests.positive]
- GIVEN a selected Valence gameplay or example plugin is installed in a minimal test app
- WHEN contract helpers inspect the app
- THEN expected metadata, phase sets, schedule labels, resources, events, and scope model are present.

#### Scenario: Missing or stale Valence metadata fails clearly

r[valence_bevy_ecs.plugin_contract_metadata.tests.negative]
- GIVEN a selected plugin is missing, disabled, lacks required metadata, or declares an ordering/resource/event fact that is not installed
- WHEN contract tests run
- THEN the failure names the missing or stale contract fact
- AND no false installed-plugin contract is reported.

### Requirement: Plugin contract metadata validation

r[valence_bevy_ecs.plugin_contract_metadata.validation] Valence plugin contract metadata work MUST record focused Valence contract checks, selected example checks, cross-vocabulary review notes, Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest checks before archive.

#### Scenario: Valence metadata closeout is reviewable

r[valence_bevy_ecs.plugin_contract_metadata.validation.log]
- GIVEN unified plugin contract metadata work is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show inventory, adapter checks, positive and negative metadata tests, selected example checks, Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, and refreshed evidence manifests.
