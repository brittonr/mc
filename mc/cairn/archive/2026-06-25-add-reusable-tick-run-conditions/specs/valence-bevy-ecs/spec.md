# valence-bevy-ecs Change Spec: Reusable tick run conditions

## Requirements

### Requirement: Tick run-condition inventory

r[valence_bevy_ecs.tick_run_conditions.inventory] Tick run-condition work MUST inventory selected periodic systems, tick sources, interval constants, current modulo behavior, event readers, mutation targets, and evidence impact before changing scheduling.

#### Scenario: Periodic behavior is reviewable

r[valence_bevy_ecs.tick_run_conditions.inventory.reviewable]
- GIVEN a periodic system is selected for tick run-condition work
- WHEN reviewers inspect the inventory
- THEN its tick source, interval value, current due condition, event readers, mutation target, and evidence impact are recorded
- AND unexplained numeric intervals are replaced or planned as named constants or config values.

### Requirement: Tick run-condition classification

r[valence_bevy_ecs.tick_run_conditions.classification] Each selected periodic behavior MUST be classified as pure periodic no-op, delayed due-work, wall-clock measurement, async completion, or event-reader drain behavior before adding run conditions.

#### Scenario: Unsuitable timing behavior is excluded

r[valence_bevy_ecs.tick_run_conditions.classification.unsuitable]
- GIVEN a timing behavior depends on delayed due-work, wall-clock measurement, async completion, or event-reader draining
- WHEN tick run-condition adoption is planned
- THEN the behavior is left outside reusable tick run conditions or separately scoped
- AND the rationale is recorded.

### Requirement: Tick run-condition contract

r[valence_bevy_ecs.tick_run_conditions.contract] Reusable tick-cadence run conditions MUST define current tick source, interval units, phase alignment, invalid-interval behavior, and tick-rate-change behavior.

#### Scenario: Cadence condition is deterministic

r[valence_bevy_ecs.tick_run_conditions.contract.deterministic]
- GIVEN a tick-cadence run condition is evaluated with explicit tick and interval inputs
- WHEN pure tests exercise due and not-due ticks
- THEN the condition returns deterministic results
- AND zero, negative, overflow, or otherwise invalid intervals fail closed or produce typed errors according to the contract.

### Requirement: Tick run-condition wiring

r[valence_bevy_ecs.tick_run_conditions.wiring] Selected systems with pure periodic no-op disabled behavior SHOULD use Bevy `run_if` conditions or set-level conditions instead of inline modulo guards.

#### Scenario: Periodic no-op body is skipped by schedule

r[valence_bevy_ecs.tick_run_conditions.wiring.no_op]
- GIVEN a selected periodic system has no disabled cleanup, diagnostics, state mutation, or event-drain obligation
- WHEN the current tick is not due
- THEN Bevy schedule conditions prevent the system body from running
- AND due-tick behavior remains compatible with the baseline.

### Requirement: Tick run-condition tests

r[valence_bevy_ecs.tick_run_conditions.tests] Tick run-condition work MUST include positive cadence tests and negative invalid interval, disabled plugin, stale event-reader, tick-rate-change, and behavior-preservation tests for changed systems.

#### Scenario: Due ticks run expected systems

r[valence_bevy_ecs.tick_run_conditions.tests.positive]
- GIVEN a valid cadence condition and a due tick
- WHEN the app updates
- THEN the selected system runs and emits the same mutation or presentation behavior as before migration.

#### Scenario: Not-due and invalid ticks fail closed

r[valence_bevy_ecs.tick_run_conditions.tests.negative]
- GIVEN a not-due tick, invalid interval, disabled plugin, event-reader candidate, or tick-rate-change case
- WHEN cadence tests run
- THEN behavior matches the documented contract
- AND no stale event replay, false mutation, panic, or hidden timing policy change occurs.

### Requirement: Tick run-condition validation

r[valence_bevy_ecs.tick_run_conditions.validation] Tick run-condition work MUST record focused example/helper checks, schedule hygiene when conditions change, Cairn gates, Cairn validation, and task-evidence checks before archive.

#### Scenario: Tick run-condition closeout is reviewable

r[valence_bevy_ecs.tick_run_conditions.validation.log]
- GIVEN tick run-condition work is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show pure cadence tests, positive and negative changed-system tests, focused example/helper checks, schedule hygiene when applicable, Cairn proposal/design/tasks gates, Cairn validation, and task-evidence validation.
