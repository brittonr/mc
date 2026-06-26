# valence-bevy-ecs Change Spec: Optional fixture run conditions

## Requirements

### Requirement: Fixture run-condition inventory

r[valence_bevy_ecs.fixture_run_conditions.inventory] Optional fixture run-condition work MUST inventory targeted optional systems, runtime enabled checks, event readers, resources, disabled behavior, and re-enable expectations before changing scheduling.

#### Scenario: Optional fixture behavior is visible

r[valence_bevy_ecs.fixture_run_conditions.inventory.visible]
- GIVEN an optional fixture or probe system is selected for run-condition work
- WHEN reviewers inspect the inventory
- THEN its enabled configuration, event readers, resource access, disabled behavior, runtime-toggle expectations, and compatibility milestones are recorded
- AND systems with stateful event cursors are identified.

### Requirement: Fixture disabled contract

r[valence_bevy_ecs.fixture_run_conditions.contract] Each targeted optional fixture system MUST define disabled behavior as skip, drain, transform, reject, or explicit in-system guard before adding a Bevy run condition.

#### Scenario: Disabled fixture behavior is intentional

r[valence_bevy_ecs.fixture_run_conditions.contract.intentional]
- GIVEN a fixture system is disabled by configuration or missing resource
- WHEN its disabled contract is reviewed
- THEN the contract states whether input events are unread, drained, transformed, rejected, or handled by an explicit guard
- AND behavior after re-enabling is documented.

### Requirement: Fixture run-condition wiring

r[valence_bevy_ecs.fixture_run_conditions.wiring] Optional fixture systems with pure no-op disabled behavior SHOULD use Bevy `run_if` conditions or set-level conditions instead of repeated per-run guards.

#### Scenario: No-op fixture hook is schedule gated

r[valence_bevy_ecs.fixture_run_conditions.wiring.no_op]
- GIVEN an optional fixture hook has no disabled cleanup, diagnostics, state mutation, or event-drain obligation
- WHEN the hook is disabled
- THEN Bevy schedule conditions prevent the hook body from running
- AND enabled behavior remains unchanged.

### Requirement: Fixture event-reader behavior

r[valence_bevy_ecs.fixture_run_conditions.event_readers] Optional fixture systems that read Bevy events MUST preserve documented event cursor behavior when disabled, including explicit drains when skipped readers would accumulate stale events.

#### Scenario: Disabled fixture reader does not replay stale events

r[valence_bevy_ecs.fixture_run_conditions.event_readers.no_stale_replay]
- GIVEN an optional event-reading fixture system is disabled while events are produced
- WHEN the system is later enabled
- THEN it observes only events allowed by its disabled contract
- AND stale disabled-period events are not replayed unless the contract explicitly permits replay.

### Requirement: Fixture run-condition tests

r[valence_bevy_ecs.fixture_run_conditions.tests] Optional fixture run-condition work MUST include positive enabled tests and negative disabled, stale-event, and runtime-toggle tests for changed systems.

#### Scenario: Enabled fixture path remains compatible

r[valence_bevy_ecs.fixture_run_conditions.tests.positive]
- GIVEN a changed fixture system is enabled
- WHEN its expected inputs are present
- THEN it emits the same events, records, mutations, or compatibility milestones as before the run-condition change.

#### Scenario: Disabled fixture path follows contract

r[valence_bevy_ecs.fixture_run_conditions.tests.negative]
- GIVEN a changed fixture system is disabled and receives inputs
- WHEN the app updates and later toggles the system if supported
- THEN disabled outputs, event cursor behavior, stale-state handling, and diagnostics match the documented disabled contract.

### Requirement: Fixture run-condition validation

r[valence_bevy_ecs.fixture_run_conditions.validation] Optional fixture run-condition work MUST record focused example checks, selected compatibility rails when fixture behavior changes, Valence schedule hygiene when schedule conditions change, Cairn gates, Cairn validation, and task-evidence checks before archive.

#### Scenario: Fixture run-condition closeout is reviewable

r[valence_bevy_ecs.fixture_run_conditions.validation.log]
- GIVEN optional fixture run-condition work is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show enabled-path tests, disabled-path tests, stale-event tests, runtime-toggle tests where supported, focused example checks, selected mc-compat rails when applicable, schedule hygiene when applicable, Cairn proposal/design/tasks gates, Cairn validation, and task-evidence validation.
