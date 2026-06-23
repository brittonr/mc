# Valence Hyperion Integration Specification

## Purpose

Defines the `valence-hyperion-integration` capability.

## Requirements

### Requirement: Hyperion integration inventory

r[valence_hyperion_integration.boundaries.inventory] Future Hyperion-to-Valence integration work MUST classify inspected Hyperion sources as adopt, port, reference, or reject before implementation.

#### Scenario: Source classification precedes code changes

r[valence_hyperion_integration.boundaries.inventory.precedes]
- GIVEN a future integration Cairn proposes using Hyperion code or concepts
- WHEN reviewers inspect its design
- THEN each relevant source is classified as adopt, port, reference, or reject
- AND the classification explains ownership, safety, and API impact.

### Requirement: Forbidden Valence core merges

r[valence_hyperion_integration.boundaries.forbidden_core] Integration work MUST NOT merge Bedwars-specific game logic, replace Valence's runtime with Hyperion's runtime wholesale, add custom combat as Valence core behavior, or import unaudited nightly/unsafe-heavy code directly into Valence core.

#### Scenario: Forbidden source is rejected

r[valence_hyperion_integration.boundaries.forbidden_core.rejected]
- GIVEN an inspected Hyperion source is Bedwars-specific, runtime-replacement scope, custom combat core behavior, or unaudited nightly/unsafe-heavy implementation
- WHEN the integration inventory is evaluated
- THEN the source is classified as reject or reference-only
- AND no Valence core task depends on copying it directly.

### Requirement: Optional gameplay plugin boundary

r[valence_hyperion_integration.boundaries.optional_plugins] Gameplay semantics inspired by Hyperion MAY be implemented only as optional plugins or examples unless separate accepted Valence scope and reference evidence justify core behavior.

#### Scenario: Combat remains optional without reference evidence

r[valence_hyperion_integration.boundaries.optional_plugins.combat]
- GIVEN Hyperion combat behavior is considered for Valence
- WHEN no separate vanilla/reference evidence proves the intended core behavior
- THEN the work is scoped as an optional plugin, example, or reference-only note
- AND Valence core behavior remains unchanged.

### Requirement: Integration review gate

r[valence_hyperion_integration.boundaries.review_gate] Future Hyperion integration Cairns SHOULD cite the boundary inventory and non-claim checklist before archive.

#### Scenario: Non-claim checklist is present

r[valence_hyperion_integration.boundaries.review_gate.non_claims]
- GIVEN an integration Cairn is ready to archive
- WHEN reviewers inspect its proposal and evidence
- THEN production-scale, vanilla-parity, Hyperion-compatibility, default-behavior, and safety claims are each either supported by evidence or explicitly left as non-claims.

### Requirement: Boundary fixtures

r[valence_hyperion_integration.boundaries.fixtures] Boundary work SHOULD include positive and negative checklist examples or fixtures for allowed reference use and forbidden direct imports.

#### Scenario: Reference-only use passes

r[valence_hyperion_integration.boundaries.fixtures.reference_only]
- GIVEN a future Cairn uses Hyperion code only as design reference
- WHEN the boundary checklist is evaluated
- THEN it passes if no copied code or unsupported behavior claim is present
- AND it records the referenced source and resulting Valence-owned design.

### Requirement: Boundary validation

r[valence_hyperion_integration.boundaries.validation] Boundary work MUST record inventory/checklist validation, negative forbidden-import examples, and Cairn gates before archive.

#### Scenario: Boundary closeout is reviewable

r[valence_hyperion_integration.boundaries.validation.log]
- GIVEN the boundary change is ready to archive
- WHEN reviewers inspect evidence logs
- THEN logs show inventory/checklist validation, positive reference-only examples, negative forbidden-import examples, Cairn proposal/design/tasks gates, and Cairn validation.

### Requirement: Anti-cheat statistics scope

r[valence_hyperion_integration.anticheat_stats.scope] The integration MUST audit Hyperion statistics behavior and Valence event sources before adding an anti-cheat statistics plugin.

#### Scenario: Metric scope is bounded

r[valence_hyperion_integration.anticheat_stats.scope.bounded]
- GIVEN anti-cheat statistics work is selected
- WHEN reviewers inspect the scope notes
- THEN the selected metrics, event sources, sampling windows, non-goals, and no-default-enforcement boundary are recorded.

### Requirement: Stable statistics core

r[valence_hyperion_integration.anticheat_stats.core] Statistics calculations MUST be implemented first as a pure stable Rust core over explicit samples and sample-window settings.

#### Scenario: Empty sample window is handled

r[valence_hyperion_integration.anticheat_stats.core.empty]
- GIVEN the statistics core receives an empty sample window
- WHEN it computes selected metrics
- THEN it returns the documented empty-window result
- AND it does not panic, divide by zero, read clocks, or mutate global state.

### Requirement: Statistics fixture coverage

r[valence_hyperion_integration.anticheat_stats.fixtures] Anti-cheat statistics work MUST include positive and negative fixtures for normal samples and invalid/boundary inputs.

#### Scenario: Invalid sample window fails closed

r[valence_hyperion_integration.anticheat_stats.fixtures.invalid_window]
- GIVEN a metric config has an invalid sample window
- WHEN the fixture validator runs
- THEN it returns a deterministic diagnostic
- AND the plugin does not emit a misleading score for that metric.

### Requirement: Optional statistics plugin

r[valence_hyperion_integration.anticheat_stats.plugin] Valence MAY expose an optional statistics plugin that samples explicit event streams and emits observations, but it MUST NOT enforce kicks, bans, or gameplay mutations by default.

#### Scenario: Plugin disabled has no effect

r[valence_hyperion_integration.anticheat_stats.plugin.disabled]
- GIVEN the statistics plugin is not enabled
- WHEN existing Valence gameplay and networking tests run
- THEN no anti-cheat components, metrics, or enforcement behavior are added.

### Requirement: Statistics documentation

r[valence_hyperion_integration.anticheat_stats.docs] Statistics plugin documentation SHOULD describe metric meanings, false-positive risks, data retention, and non-claims.

#### Scenario: Docs warn about enforcement limits

r[valence_hyperion_integration.anticheat_stats.docs.limits]
- GIVEN statistics docs are published
- WHEN reviewers inspect them
- THEN they state that metrics are advisory signals unless a separate policy plugin consumes them.

### Requirement: Statistics validation

r[valence_hyperion_integration.anticheat_stats.validation] Anti-cheat statistics work MUST record statistics tests, negative fixtures, plugin-off regressions, sampling smoke tests, and Cairn gates before archive.

#### Scenario: Statistics closeout is reviewable

r[valence_hyperion_integration.anticheat_stats.validation.log]
- GIVEN statistics plugin work is ready to archive
- WHEN reviewers inspect evidence logs
- THEN logs show pure statistics tests, negative invalid-input tests, plugin-off regressions, sampling smoke tests, docs checks if present, and Cairn validation.

### Requirement: GUI helper scope

r[valence_hyperion_integration.gui_helper.scope] The integration MUST review Hyperion GUI helper concepts and Valence inventory/window behavior before adding a GUI helper plugin.

#### Scenario: Inventory ownership remains clear

r[valence_hyperion_integration.gui_helper.scope.inventory_ownership]
- GIVEN GUI helper work is selected
- WHEN reviewers inspect the scope notes
- THEN the notes identify which behavior remains owned by `valence_inventory` and which behavior belongs to the optional GUI helper.

### Requirement: GUI model contract

r[valence_hyperion_integration.gui_helper.model] The GUI helper SHOULD define an explicit model for windows, slots, readonly behavior, click outcomes, close events, and lifecycle cleanup.

#### Scenario: Readonly slot rejects mutation

r[valence_hyperion_integration.gui_helper.model.readonly]
- GIVEN a GUI slot is marked readonly
- WHEN a client click attempts to mutate that slot
- THEN the model returns the documented rejection or action result
- AND no inventory mutation is planned for that slot.

### Requirement: Pure GUI transitions

r[valence_hyperion_integration.gui_helper.core] GUI state transitions SHOULD be pure deterministic helpers over explicit model inputs, with ECS/event shells applying packets, commands, or inventory mutations.

#### Scenario: Stale window click is rejected

r[valence_hyperion_integration.gui_helper.core.stale_window]
- GIVEN a click references a stale or closed GUI window
- WHEN the GUI transition helper evaluates it
- THEN it returns a deterministic stale-window result
- AND no click action or inventory mutation is emitted.

### Requirement: GUI helper tests

r[valence_hyperion_integration.gui_helper.tests] GUI helper work MUST include positive and negative tests for open, click, readonly slots, stale window IDs, invalid slots, close events, disconnect cleanup, and plugin-disabled behavior.

#### Scenario: Disconnect cleans up viewer state

r[valence_hyperion_integration.gui_helper.tests.disconnect]
- GIVEN a client is viewing a GUI
- WHEN the client disconnects
- THEN viewer state is removed or marked closed
- AND later clicks from that window are rejected.

### Requirement: GUI helper docs

r[valence_hyperion_integration.gui_helper.docs] GUI helper documentation SHOULD show common menu examples and avoid claiming full vanilla container parity.

#### Scenario: Docs preserve inventory non-claims

r[valence_hyperion_integration.gui_helper.docs.non_claims]
- GIVEN GUI helper docs are published
- WHEN reviewers inspect them
- THEN they state that the helper builds on existing inventory semantics
- AND they do not claim untested vanilla container parity.

### Requirement: GUI helper validation

r[valence_hyperion_integration.gui_helper.validation] GUI helper work MUST record GUI tests, inventory integration tests, example smoke tests, selected inventory mc-compat dry runs, and Cairn gates before archive.

#### Scenario: GUI helper closeout is reviewable

r[valence_hyperion_integration.gui_helper.validation.log]
- GIVEN GUI helper work is ready to archive
- WHEN reviewers inspect evidence logs
- THEN logs show GUI model tests, negative stale/invalid-click tests, inventory integration tests, example smoke output, selected inventory mc-compat dry runs, and Cairn validation.
