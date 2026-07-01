# hyperion-game-modes Change Spec: Plugin diagnostics registry

## Requirements

### Requirement: Plugin diagnostics inventory

r[hyperion_game_modes.plugin_diagnostics.inventory] Plugin diagnostics registry work MUST inventory current Hyperion plugin inspectability gaps, preset planner diagnostics, app-builder composition facts, plugin flags/resources, dependency decisions, contract metadata, direct-plugin behavior, and evidence needs before adding a diagnostics registry.

#### Scenario: Diagnostics gaps are reviewable

r[hyperion_game_modes.plugin_diagnostics.inventory.reviewable]
- GIVEN plugin diagnostics registry work is selected
- WHEN reviewers inspect the inventory
- THEN current tests, resources, plugin-added flags, preset plan fields, disabled/replacement/custom intent data, dependency checks, and missing inspection surfaces are recorded
- AND runtime admin protocols, metrics backends, hot reload, and sandboxing are identified as non-claims.

### Requirement: Plugin diagnostics registry

r[hyperion_game_modes.plugin_diagnostics.registry] Hyperion SHOULD expose a deterministic plugin diagnostics registry that records selected mode, default gameplay inclusion, installed features, disabled features, replacement features, custom plugin slots, dependency decisions, contract metadata, provenance, and non-claim boundaries for validated composition paths.

#### Scenario: Registry explains builder composition

r[hyperion_game_modes.plugin_diagnostics.registry.builder]
- GIVEN a Hyperion app is built through a validated preset or default builder
- WHEN tests or receipts inspect the diagnostics registry
- THEN they can identify selected mode, default gameplay status, installed feature set, disabled features, replacement slots, custom slots, dependency decisions, contract metadata, and provenance
- AND diagnostics are derived from the validated plan and shell install results rather than hidden global state.

### Requirement: Plugin diagnostics wiring

r[hyperion_game_modes.plugin_diagnostics.wiring] Hyperion app builders and selected plugin groups MUST populate diagnostics without changing default gameplay, mode plugin behavior, proxy setup, command registration, or direct plugin semantics unless another Cairn changes them.

#### Scenario: Diagnostics are side-effect-light

r[hyperion_game_modes.plugin_diagnostics.wiring.compatible]
- GIVEN diagnostics registry wiring is introduced
- WHEN existing default app builders and selected custom preset builders run
- THEN runtime plugin behavior, selected mode, default feature composition, proxy resources, and command registration remain compatible
- AND diagnostics population does not install extra gameplay systems, read files, start networking, or require live proxy startup.

### Requirement: Plugin diagnostics exposure

r[hyperion_game_modes.plugin_diagnostics.exposure] Plugin diagnostics SHOULD be inspectable through deterministic test helpers or optional receipt/log rendering that does not require a live server or proxy.

#### Scenario: Diagnostics are receipt-friendly

r[hyperion_game_modes.plugin_diagnostics.exposure.receipt]
- GIVEN a focused composition check builds a Hyperion app without running a live server
- WHEN diagnostics are rendered for evidence
- THEN the output names selected mode, features, disabled/replaced/custom slots, dependencies, contracts, provenance, and non-claims in a deterministic order
- AND optional hashes use BLAKE3 unless an existing receipt contract requires another algorithm.

### Requirement: Plugin diagnostics tests

r[hyperion_game_modes.plugin_diagnostics.tests] Plugin diagnostics registry work MUST include positive diagnostics tests and negative stale diagnostics, missing diagnostics, disabled-feature mismatch, replacement mismatch, dependency mismatch, direct-provenance, and no-live-server tests.

#### Scenario: Valid diagnostics match composition

r[hyperion_game_modes.plugin_diagnostics.tests.positive]
- GIVEN supported default and custom Hyperion compositions are built through validated builders
- WHEN diagnostics tests inspect the registry
- THEN registry facts match planner outputs and selected installed plugin/resource facts.

#### Scenario: Stale diagnostics fail clearly

r[hyperion_game_modes.plugin_diagnostics.tests.negative]
- GIVEN a fixture omits diagnostics, declares a feature installed when it is disabled, records the wrong selected mode, loses a replacement/custom slot, or reports unsupported provenance
- WHEN diagnostics tests run
- THEN the failure names the stale or missing diagnostic fact
- AND no live proxy startup, panic-string parsing, or hidden fallback is required.

### Requirement: Plugin diagnostics validation

r[hyperion_game_modes.plugin_diagnostics.validation] Plugin diagnostics registry work MUST record focused Hyperion diagnostics tests, preset builder checks, disabled/replacement/custom-slot checks, negative stale diagnostics tests, Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest checks before archive.

#### Scenario: Diagnostics closeout is reviewable

r[hyperion_game_modes.plugin_diagnostics.validation.log]
- GIVEN plugin diagnostics registry work is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show inventory, registry schema checks, positive and negative diagnostics tests, default/custom preset checks, no-live-server evidence, Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, and refreshed evidence manifests.
