# valence-hyperion-integration Change Spec: Optional GUI helper plugin

## Requirements

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
