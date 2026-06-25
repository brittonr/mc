# valence-bevy-ecs Change Spec: Run conditions for optional systems

## Requirements

### Requirement: Run-condition inventory

r[valence_bevy_ecs.run_conditions.inventory] Run-condition work MUST inventory targeted optional systems, runtime enabled checks, event readers, resources, and disabled-mode behavior before changing scheduling.

#### Scenario: Optional system behavior is visible

r[valence_bevy_ecs.run_conditions.inventory.visible]
- GIVEN an optional plugin or helper system is selected for run-condition work
- WHEN reviewers inspect the inventory
- THEN each targeted system records its enabled config, event readers, resource access, disabled behavior, and runtime toggle expectations
- AND systems with stateful event cursors are identified.

### Requirement: Run-condition disabled contract

r[valence_bevy_ecs.run_conditions.contract] Each targeted optional system MUST define disabled behavior as skip, drain, transform, reject, or explicit in-system guard before adding a Bevy run condition.

#### Scenario: Disabled behavior is intentional

r[valence_bevy_ecs.run_conditions.contract.intentional]
- GIVEN a system is disabled by config or plugin state
- WHEN its disabled contract is reviewed
- THEN the contract states whether input events are unread, drained, transformed, or rejected
- AND the behavior after re-enabling is documented.

### Requirement: Run-condition wiring

r[valence_bevy_ecs.run_conditions.wiring] Optional systems with pure no-op disabled behavior SHOULD use Bevy `run_if` conditions or set-level conditions instead of repeated per-run guards.

#### Scenario: No-op hook is gated by schedule condition

r[valence_bevy_ecs.run_conditions.wiring.no_op]
- GIVEN an optional hook has no disabled cleanup or event-drain obligation
- WHEN the hook is disabled
- THEN Bevy schedule conditions prevent the hook body from running
- AND enabled behavior remains unchanged.

### Requirement: Event reader disabled behavior

r[valence_bevy_ecs.run_conditions.event_readers] Optional systems that read Bevy events MUST preserve documented event cursor behavior when disabled, including explicit drains when skipped readers would accumulate stale events.

#### Scenario: Disabled reader does not replay stale data

r[valence_bevy_ecs.run_conditions.event_readers.no_stale_replay]
- GIVEN an optional event-reading system is disabled while events are produced
- WHEN the system is later enabled
- THEN it observes only events allowed by its disabled contract
- AND stale disabled-period events are not replayed unless the contract explicitly permits replay.

### Requirement: Run-condition tests

r[valence_bevy_ecs.run_conditions.tests] Run-condition work MUST include positive enabled tests and negative disabled, stale-event, and runtime-toggle tests for changed systems.

#### Scenario: Enabled path still emits expected output

r[valence_bevy_ecs.run_conditions.tests.positive]
- GIVEN a changed optional system is enabled
- WHEN its expected inputs are present
- THEN it emits the same events, records, or mutations as before the run-condition change.

#### Scenario: Disabled path follows contract

r[valence_bevy_ecs.run_conditions.tests.negative]
- GIVEN a changed optional system is disabled and receives inputs
- WHEN the app updates and later toggles the system if supported
- THEN disabled outputs, event cursor behavior, and diagnostics match the documented disabled contract.

### Requirement: Run-condition validation

r[valence_bevy_ecs.run_conditions.validation] Run-condition work MUST record focused Valence checks, positive and negative run-condition tests, Cairn gates, Cairn validation, and task-evidence checks before archive.

#### Scenario: Run-condition closeout is reviewable

r[valence_bevy_ecs.run_conditions.validation.log]
- GIVEN run-condition work is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show enabled-path tests, disabled-path tests, stale-event tests, runtime-toggle tests where supported, focused Valence checks, Cairn proposal/design/tasks gates, Cairn validation, and task-evidence validation.
