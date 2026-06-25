# Valence Bevy Ecs Specification

## Purpose

Defines the `valence-bevy-ecs` capability.

## Requirements

### Requirement: Schedule hygiene inventory

r[valence_bevy_ecs.schedule_hygiene.inventory] Schedule hygiene work MUST inventory current Valence schedule tooling, named sets, schedule-impacting checks, and evidence gaps before adding gates.

#### Scenario: Schedule tooling is reviewable

r[valence_bevy_ecs.schedule_hygiene.inventory.reviewable]
- GIVEN schedule hygiene work is selected
- WHEN reviewers inspect the inventory
- THEN existing schedule dump tools, named sets, default plugin behavior, ambiguity settings, and validation gaps are recorded
- AND heavy graph evidence is not required for non-schedule-impacting changes.

### Requirement: Schedule evidence policy

r[valence_bevy_ecs.schedule_hygiene.policy] The repository SHOULD define when Bevy schedule evidence is required for new plugins, schedules, system sets, ordering constraints, event-loop phases, and default plugin membership changes.

#### Scenario: Schedule-impacting change triggers evidence

r[valence_bevy_ecs.schedule_hygiene.policy.trigger]
- GIVEN a Cairn changes Bevy plugin registration, schedule labels, system sets, ordering constraints, event-loop phases, or default plugin membership
- WHEN the task plan is reviewed
- THEN the plan includes focused schedule evidence or records why schedule evidence is unnecessary.

### Requirement: Schedule receipts

r[valence_bevy_ecs.schedule_hygiene.receipts] Schedule evidence SHOULD record selected schedule labels, plugin configuration, expected sets/systems, disabled-plugin comparisons when relevant, and command provenance.

#### Scenario: Receipt identifies schedule facts

r[valence_bevy_ecs.schedule_hygiene.receipts.facts]
- GIVEN schedule evidence is produced for a Bevy change
- WHEN reviewers inspect the receipt
- THEN it names the command, schedule label, plugin configuration, expected sets or systems, and disabled/default comparison if applicable
- AND large graph artifacts are optional unless needed for review.

### Requirement: Schedule hygiene tests

r[valence_bevy_ecs.schedule_hygiene.tests] Schedule hygiene gates MUST include positive valid-schedule checks and negative unknown schedule, missing set, unintended default plugin, and ambiguity regression checks where feasible.

#### Scenario: Valid schedule check passes

r[valence_bevy_ecs.schedule_hygiene.tests.positive]
- GIVEN a selected Valence app/plugin configuration is valid
- WHEN the schedule hygiene check runs
- THEN expected schedules, sets, and plugin-owned systems are reported.

#### Scenario: Invalid schedule check fails clearly

r[valence_bevy_ecs.schedule_hygiene.tests.negative]
- GIVEN an unknown schedule, missing expected set, unintended default plugin insertion, or forbidden ambiguity is present in a fixture
- WHEN the schedule hygiene check runs
- THEN it fails with a diagnostic that names the missing or unexpected schedule fact.

### Requirement: Schedule evidence promotion

r[valence_bevy_ecs.schedule_hygiene.evidence] Task-cited schedule artifacts MUST be promoted under reviewable tracked evidence paths and include BLAKE3 manifests when cited by tasks or archive docs.

#### Scenario: Cited schedule evidence is durable

r[valence_bevy_ecs.schedule_hygiene.evidence.durable]
- GIVEN a task cites a schedule dump, schedule check log, or graph artifact
- WHEN evidence validation runs
- THEN the cited artifact is tracked outside transient `target/` output
- AND any manifest digest uses BLAKE3 unless an existing contract requires another algorithm.

### Requirement: Schedule hygiene validation

r[valence_bevy_ecs.schedule_hygiene.validation] Schedule hygiene work MUST record schedule checks, Cairn gates, Cairn validation, and task-evidence checks before archive.

#### Scenario: Schedule hygiene closeout is reviewable

r[valence_bevy_ecs.schedule_hygiene.validation.log]
- GIVEN schedule hygiene work is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show positive and negative schedule hygiene checks, promoted schedule artifacts if cited, Cairn proposal/design/tasks gates, Cairn validation, and task-evidence validation.

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

### Requirement: Scheduler adoption inventory

r[valence_bevy_ecs.scheduler_adoption.inventory] Valence scheduler adoption work MUST inventory selected tick counters, modulo checks, cooldowns, delayed resets, temporary world changes, despawn timers, and fixture deadlines before replacing them.

#### Scenario: Timing behavior is reviewable

r[valence_bevy_ecs.scheduler_adoption.inventory.reviewable]
- GIVEN an example, fixture, or plugin is selected for scheduler adoption
- WHEN reviewers inspect the inventory
- THEN each timing behavior records current owner, tick source, due condition, mutation target, cancellation behavior, and evidence impact
- AND behaviors that are not explicit tick-keyed delayed work are identified.

### Requirement: Scheduler adoption classification

r[valence_bevy_ecs.scheduler_adoption.classification] Each selected timing behavior MUST be classified as scheduler-suitable, immediate state, wall-clock or async work, or intentionally custom policy before migration.

#### Scenario: Unsuitable behavior is not forced into scheduler

r[valence_bevy_ecs.scheduler_adoption.classification.unsuitable]
- GIVEN a timing behavior depends on wall-clock measurement, background completion, or continuous per-tick state
- WHEN scheduler adoption is planned
- THEN the behavior is left outside `TickScheduler` or separately scoped
- AND the rationale is recorded.

### Requirement: Scheduler adoption wiring

r[valence_bevy_ecs.scheduler_adoption.wiring] Scheduler-suitable gameplay delays SHOULD use `ServerTickScheduler` resources or typed scheduled events instead of ad hoc counters or delayed maps.

#### Scenario: Due work drains through typed shell

r[valence_bevy_ecs.scheduler_adoption.wiring.drain]
- GIVEN work is scheduled for an explicit server tick
- WHEN the scheduler shell drains due work
- THEN it emits or applies a typed domain request
- AND world mutation systems validate entity liveness and current ownership before applying it.

### Requirement: Scheduler policy boundary

r[valence_bevy_ecs.scheduler_adoption.policy] Gameplay durations, cooldown rules, despawn targets, block restoration choices, and compatibility milestone decisions MUST remain outside the generic scheduler core.

#### Scenario: Scheduler does not own gameplay policy

r[valence_bevy_ecs.scheduler_adoption.policy.boundary]
- GIVEN a cooldown or delayed fixture action uses the scheduler
- WHEN reviewers inspect the implementation
- THEN policy values and rule decisions are supplied by gameplay or fixture code
- AND the scheduler core only stores, orders, cancels, clears, and drains explicit work.

### Requirement: Scheduler adoption tests

r[valence_bevy_ecs.scheduler_adoption.tests] Scheduler adoption MUST include positive due-work tests and negative cancellation, stale entity, duplicate event, invalid tick, and plugin-disabled tests for migrated behavior.

#### Scenario: Due work applies once

r[valence_bevy_ecs.scheduler_adoption.tests.positive]
- GIVEN valid work is scheduled for a future tick
- WHEN the server reaches that tick
- THEN the typed work is observed exactly once in documented order.

#### Scenario: Stale delayed work fails closed

r[valence_bevy_ecs.scheduler_adoption.tests.negative]
- GIVEN scheduled work targets an entity or state that despawned, disconnected, reconnected, or changed ownership
- WHEN the due work drains
- THEN mutation is skipped or diagnosed deterministically
- AND no duplicate milestone, panic, or stale world mutation occurs.

### Requirement: Scheduler adoption validation

r[valence_bevy_ecs.scheduler_adoption.validation] Scheduler adoption MUST record focused scheduler/example checks, selected compatibility rails when timing changes, Cairn gates, Cairn validation, and task-evidence checks before archive.

#### Scenario: Scheduler adoption closeout is reviewable

r[valence_bevy_ecs.scheduler_adoption.validation.log]
- GIVEN scheduler adoption is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show positive and negative scheduler adoption tests, focused Valence/example checks, selected mc-compat dry-runs if fixture timing changed, Cairn proposal/design/tasks gates, Cairn validation, and task-evidence validation.
