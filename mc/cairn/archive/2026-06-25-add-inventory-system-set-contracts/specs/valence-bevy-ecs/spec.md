# valence-bevy-ecs Change Spec: Inventory SystemSet contracts

## Requirements

### Requirement: Inventory schedule inventory

r[valence_bevy_ecs.inventory_sets.inventory] Inventory SystemSet work MUST inventory current `InventoryPlugin` systems, schedules, tuple ordering, resources, events, feature/default plugin membership, and downstream ordering dependencies before changing schedule wiring.

#### Scenario: Inventory schedule baseline is reviewable

r[valence_bevy_ecs.inventory_sets.inventory.reviewable]
- GIVEN inventory schedule work is selected
- WHEN reviewers inspect the inventory
- THEN each current inventory system records its schedule label, ordering constraints, resource access, event access, and mutation target
- AND feature gates, default plugin membership, and downstream ordering assumptions are recorded.

### Requirement: Inventory SystemSet contract

r[valence_bevy_ecs.inventory_sets.contract] `InventoryPlugin` SHOULD expose named Bevy `SystemSet`s for stable packet input, model mutation, viewer/window synchronization, presentation or flush preparation, and cleanup ordering where those phases exist.

#### Scenario: Inventory phases are orderable

r[valence_bevy_ecs.inventory_sets.contract.phases]
- GIVEN a user plugin needs to run around inventory behavior
- WHEN it orders relative to inventory schedule phases
- THEN it can target named inventory sets instead of relying on anonymous tuple order
- AND the set documentation states which phases are public ordering contracts.

### Requirement: Inventory SystemSet wiring

r[valence_bevy_ecs.inventory_sets.wiring] Inventory SystemSet wiring MUST preserve existing inventory events, resources, packet behavior, feature gates, and default plugin membership unless another Cairn changes them.

#### Scenario: Wiring preserves behavior

r[valence_bevy_ecs.inventory_sets.wiring.preserve]
- GIVEN existing inventory behavior is moved into named sets
- WHEN focused inventory tests and schedule checks run
- THEN selected packet input, inventory mutation, viewer/window synchronization, and packet flush preparation remain compatible with the baseline
- AND default Valence plugin membership does not change.

### Requirement: Inventory schedule compatibility

r[valence_bevy_ecs.inventory_sets.compatibility] Inventory SystemSet work MUST document downstream compatibility boundaries and schedule non-claims for ordering points that remain intentionally private.

#### Scenario: Non-claims are explicit

r[valence_bevy_ecs.inventory_sets.compatibility.non_claims]
- GIVEN an inventory ordering point is not promoted to a public or crate-visible set
- WHEN reviewers inspect the schedule contract
- THEN the implementation explains why anonymous or private ordering remains sufficient
- AND no broad inventory compatibility, vanilla parity, or production-readiness claim is added.

### Requirement: Inventory SystemSet tests

r[valence_bevy_ecs.inventory_sets.tests] Inventory SystemSet work MUST include positive schedule/plugin smoke tests and negative disabled-plugin or ordering-regression tests.

#### Scenario: Inventory plugin installs expected sets

r[valence_bevy_ecs.inventory_sets.tests.positive]
- GIVEN `InventoryPlugin` is added to a minimal test app with required dependencies
- WHEN schedules are initialized
- THEN expected inventory events, resources, sets, and schedule labels are present.

#### Scenario: Disabled inventory plugin is absent

r[valence_bevy_ecs.inventory_sets.tests.negative]
- GIVEN the inventory plugin is not added or is disabled through a plugin group comparison
- WHEN schedules and resources are inspected
- THEN inventory-owned resources, events, systems, and sets are absent
- AND no inventory schedule contract is falsely reported as installed.

### Requirement: Inventory SystemSet validation

r[valence_bevy_ecs.inventory_sets.validation] Inventory SystemSet work MUST record focused inventory checks, Valence schedule hygiene, selected example checks when touched, Cairn gates, Cairn validation, and task-evidence checks before archive.

#### Scenario: Inventory schedule closeout is reviewable

r[valence_bevy_ecs.inventory_sets.validation.log]
- GIVEN inventory SystemSet work is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show focused inventory tests, positive and negative schedule tests, Valence schedule hygiene, selected example checks when applicable, Cairn proposal/design/tasks gates, Cairn validation, and task-evidence validation.
