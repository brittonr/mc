# valence-hyperion-integration Change Spec: Valence tick scheduler

## Requirements

### Requirement: Tick scheduler scope

r[valence_hyperion_integration.tick_scheduler.scope] The integration MUST review Hyperion scheduler behavior and Valence tick/timer patterns before adding a scheduler utility.

#### Scenario: Scheduler scope is bounded

r[valence_hyperion_integration.tick_scheduler.scope.bounded]
- GIVEN tick scheduler work is selected
- WHEN reviewers inspect the scope notes
- THEN the notes identify adopted concepts, affected Valence surfaces, gameplay examples, and non-goals such as async task scheduling or wall-clock timers.

### Requirement: Tick scheduler contract

r[valence_hyperion_integration.tick_scheduler.contract] The scheduler MUST define scheduling, peeking, draining, equal-key ordering, clearing, optional cancellation, and error behavior.

#### Scenario: Not-due work remains queued

r[valence_hyperion_integration.tick_scheduler.contract.not_due]
- GIVEN scheduled work has a key greater than the drain limit
- WHEN due work is drained
- THEN the not-due work remains queued
- AND subsequent peeking reports the earliest remaining work.

### Requirement: Pure scheduler core

r[valence_hyperion_integration.tick_scheduler.core] Scheduler queue operations MUST be pure deterministic operations over explicit keys and values, with ECS systems and tick resources kept in thin shells.

#### Scenario: Core has no implicit time

r[valence_hyperion_integration.tick_scheduler.core.no_time]
- GIVEN the scheduler core drains work
- WHEN it decides whether work is due
- THEN it uses only the explicit drain limit supplied by the caller
- AND it does not read wall-clock time, runtime state, or global tick resources.

### Requirement: Scheduler fixture coverage

r[valence_hyperion_integration.tick_scheduler.fixtures] Scheduler work MUST include positive and negative fixtures for queue boundaries and ordering behavior.

#### Scenario: Empty queue drains cleanly

r[valence_hyperion_integration.tick_scheduler.fixtures.empty]
- GIVEN the scheduler queue is empty
- WHEN due work is drained
- THEN the result is empty
- AND no panic, underflow, or stale item is reported.

### Requirement: Optional scheduler wiring

r[valence_hyperion_integration.tick_scheduler.wiring] Valence MAY expose the scheduler through an optional plugin or utility shell, but gameplay policy MUST remain outside the core scheduler.

#### Scenario: Plugin disabled has no timer effect

r[valence_hyperion_integration.tick_scheduler.wiring.disabled]
- GIVEN the scheduler plugin is not enabled
- WHEN existing Valence gameplay tests run
- THEN no scheduled gameplay behavior is inserted or drained by default.

### Requirement: Scheduler validation

r[valence_hyperion_integration.tick_scheduler.validation] Scheduler work MUST record pure scheduler tests, plugin smoke tests, example checks, and Cairn gates before archive.

#### Scenario: Scheduler closeout is reviewable

r[valence_hyperion_integration.tick_scheduler.validation.log]
- GIVEN scheduler work is ready to archive
- WHEN reviewers inspect evidence logs
- THEN logs show positive scheduler tests, negative boundary tests, plugin-disabled checks, example timer output if examples changed, and Cairn validation.
