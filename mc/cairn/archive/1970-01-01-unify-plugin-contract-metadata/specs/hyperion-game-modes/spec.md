# hyperion-game-modes Change Spec: Unified plugin contract metadata

## Requirements

### Requirement: Plugin contract metadata inventory

r[hyperion_game_modes.plugin_contract_metadata.inventory] Hyperion plugin contract metadata work MUST inventory current gameplay feature inventory entries, default gameplay group membership, preset builder metadata, mode plugin setup, dependencies, owned resources/events where known, scope assumptions, and Valence comparison fields before changing public metadata surfaces.

#### Scenario: Hyperion metadata baseline is reviewable

r[hyperion_game_modes.plugin_contract_metadata.inventory.reviewable]
- GIVEN unified plugin contract metadata work is selected
- WHEN reviewers inspect the Hyperion inventory
- THEN `DEFAULT_GAMEPLAY_INVENTORY`, public feature handles, mode-only plugins, preset builder plan fields, dependency notes, and non-claim boundaries are recorded
- AND Hyperion-only, Valence-comparable, private, and out-of-scope dynamic-plugin fields are classified explicitly.

### Requirement: Plugin contract metadata vocabulary

r[hyperion_game_modes.plugin_contract_metadata.vocabulary] Hyperion gameplay/plugin contracts SHOULD use a minimal shared vocabulary for plugin identity, install mode, scope model, schedule labels, phase order, dependencies, owned resources, owned events, compatibility boundaries, and non-claims where those concepts apply.

#### Scenario: Hyperion contract fields answer common plugin questions

r[hyperion_game_modes.plugin_contract_metadata.vocabulary.common]
- GIVEN a selected Hyperion gameplay feature, mode plugin, or preset-composed plugin exposes contract metadata
- WHEN downstream tests or reviewers inspect the contract
- THEN they can identify what plugin was installed, what dependencies are required, what schedule or resource/event ownership is declared, what scope assumptions apply, and what behavior is not claimed
- AND mode-local internals remain private unless deliberately promoted.

### Requirement: Plugin contract metadata adapters

r[hyperion_game_modes.plugin_contract_metadata.adapters] Hyperion MUST expose unified contract metadata through Hyperion-local helper types or adapters without introducing a hard dependency on Valence example helpers.

#### Scenario: Hyperion adapter stays engine-local

r[hyperion_game_modes.plugin_contract_metadata.adapters.local]
- GIVEN unified metadata is added to Hyperion gameplay groups or mode composition helpers
- WHEN the implementation is reviewed
- THEN Hyperion contract data is produced from Hyperion-local feature inventories, mode plugins, preset plans, schedules, resources, and events
- AND Valence comparison remains documentation or test vocabulary rather than a required runtime dependency.

### Requirement: Plugin contract metadata compatibility

r[hyperion_game_modes.plugin_contract_metadata.compatibility] Unified Hyperion plugin contract metadata MUST preserve default gameplay composition, mode-only behavior, preset builder compatibility, command/proxy setup, and existing non-claim boundaries unless another Cairn changes them.

#### Scenario: Metadata does not promote runtime plugin claims

r[hyperion_game_modes.plugin_contract_metadata.compatibility.non_claims]
- GIVEN a Hyperion gameplay feature or mode plugin adopts unified metadata
- WHEN receipts, docs, or tests inspect the plugin contract
- THEN existing default app behavior and compatibility evidence remain comparable
- AND no runtime-loaded plugin, hot reload, scripting, sandboxing, multi-world mode, Valence dependency, or public-server safety claim is added.

### Requirement: Plugin contract metadata tests

r[hyperion_game_modes.plugin_contract_metadata.tests] Hyperion plugin contract metadata work MUST include positive contract-inspection tests and negative missing-contract, stale-contract, disabled-feature, dependency, and unintended-mode-installation tests for selected plugins.

#### Scenario: Installed Hyperion plugin metadata matches wiring

r[hyperion_game_modes.plugin_contract_metadata.tests.positive]
- GIVEN a selected Hyperion gameplay feature group, mode plugin, or preset-composed app is built in a focused test
- WHEN contract helpers inspect the app or plan
- THEN expected metadata, dependencies, selected mode state, feature membership, owned resources/events where declared, and non-claim fields are present.

#### Scenario: Missing or stale Hyperion metadata fails clearly

r[hyperion_game_modes.plugin_contract_metadata.tests.negative]
- GIVEN a selected feature is disabled, a mode plugin is absent, a dependency is missing, or declared metadata no longer matches installed behavior
- WHEN contract tests run
- THEN the failure names the missing or stale contract fact
- AND no false installed-feature or installed-mode contract is reported.

### Requirement: Plugin contract metadata validation

r[hyperion_game_modes.plugin_contract_metadata.validation] Hyperion plugin contract metadata work MUST record focused Hyperion gameplay inventory checks, preset/mode contract checks, cross-vocabulary review notes, Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest checks before archive.

#### Scenario: Hyperion metadata closeout is reviewable

r[hyperion_game_modes.plugin_contract_metadata.validation.log]
- GIVEN unified plugin contract metadata work is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show inventory, adapter checks, positive and negative metadata tests, selected preset/mode checks, Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, and refreshed evidence manifests.
