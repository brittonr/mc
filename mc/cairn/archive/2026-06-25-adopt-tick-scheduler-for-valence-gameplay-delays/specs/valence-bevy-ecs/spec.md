# valence-bevy-ecs Change Spec: Tick scheduler adoption

## Requirements

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
