# valence-bevy-ecs Change Spec: Core plugin SystemSet contracts

## Requirements

### Requirement: Core plugin schedule inventory

r[valence_bevy_ecs.core_plugin_sets.inventory] Core plugin SystemSet work MUST inventory selected plugin systems, schedules, tuple ordering, resources, events, feature/default plugin membership, and downstream ordering assumptions before changing schedule wiring.

#### Scenario: Core plugin baseline is reviewable

r[valence_bevy_ecs.core_plugin_sets.inventory.reviewable]
- GIVEN a core plugin is selected for SystemSet contract work
- WHEN reviewers inspect the inventory
- THEN each current system records its schedule label, ordering constraints, resource access, event access, mutation target, feature gate, and default membership status
- AND downstream examples or plugins that depend on ordering are identified.

### Requirement: Core plugin SystemSet contract

r[valence_bevy_ecs.core_plugin_sets.contract] Selected core plugins SHOULD expose minimal named Bevy `SystemSet`s for stable phase-level ordering where downstream composition benefits from them.

#### Scenario: Core phases are orderable

r[valence_bevy_ecs.core_plugin_sets.contract.phases]
- GIVEN a downstream plugin needs to run around selected core plugin behavior
- WHEN it orders relative to a promoted core plugin phase
- THEN it can target a named set instead of relying on anonymous tuple order
- AND documentation states which phases are stable ordering contracts and which internals remain private.

### Requirement: Core plugin SystemSet wiring

r[valence_bevy_ecs.core_plugin_sets.wiring] Core plugin SystemSet wiring MUST preserve existing events, resources, behavior, feature gates, and default plugin membership unless another Cairn changes them.

#### Scenario: Wiring preserves selected plugin behavior

r[valence_bevy_ecs.core_plugin_sets.wiring.preserve]
- GIVEN selected core plugin behavior is moved into named sets
- WHEN focused crate tests and schedule checks run
- THEN selected packet input, state mutation, presentation, and client update behavior remain compatible with the baseline
- AND default Valence plugin membership does not change.

### Requirement: Core plugin schedule compatibility

r[valence_bevy_ecs.core_plugin_sets.compatibility] Core plugin SystemSet work MUST preserve downstream compatibility and record non-claims for unpromoted internal ordering points.

#### Scenario: Private ordering stays private

r[valence_bevy_ecs.core_plugin_sets.compatibility.private]
- GIVEN an internal ordering point is not promoted to a public or crate-visible set
- WHEN reviewers inspect the schedule contract
- THEN the implementation explains why anonymous or private ordering remains sufficient
- AND no gameplay, protocol compatibility, vanilla parity, or production-readiness claim is added.

### Requirement: Core plugin SystemSet tests

r[valence_bevy_ecs.core_plugin_sets.tests] Core plugin SystemSet work MUST include positive schedule/plugin smoke tests and negative disabled-plugin or ordering-regression tests for changed plugins.

#### Scenario: Core plugin installs expected sets

r[valence_bevy_ecs.core_plugin_sets.tests.positive]
- GIVEN a changed core plugin is added to a minimal test app with required dependencies
- WHEN schedules are initialized
- THEN expected resources, events, sets, and schedule labels are present.

#### Scenario: Disabled core plugin is absent

r[valence_bevy_ecs.core_plugin_sets.tests.negative]
- GIVEN the changed core plugin is not added or is disabled through a plugin group comparison
- WHEN schedules and resources are inspected
- THEN plugin-owned resources, events, systems, and sets are absent
- AND no core plugin schedule contract is falsely reported as installed.

### Requirement: Core plugin SystemSet validation

r[valence_bevy_ecs.core_plugin_sets.validation] Core plugin SystemSet work MUST record focused crate checks, Valence schedule hygiene, selected examples when touched, Cairn gates, Cairn validation, and task-evidence checks before archive.

#### Scenario: Core plugin schedule closeout is reviewable

r[valence_bevy_ecs.core_plugin_sets.validation.log]
- GIVEN core plugin SystemSet work is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show focused crate tests, positive and negative schedule tests, Valence schedule hygiene, selected example checks when applicable, Cairn proposal/design/tasks gates, Cairn validation, and task-evidence validation.
