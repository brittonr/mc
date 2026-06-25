# valence-bevy-ecs Change Spec: Bevy task orchestration for worldgen

## Requirements

### Requirement: Worldgen task inventory

r[valence_bevy_ecs.worldgen_tasks.inventory] Worldgen task work MUST inventory current terrain background work, thread/channel ownership, task inputs, completion handling, shutdown assumptions, and dependency impact before changing orchestration.

#### Scenario: Background work is reviewable

r[valence_bevy_ecs.worldgen_tasks.inventory.reviewable]
- GIVEN the terrain example is selected for Bevy task orchestration
- WHEN reviewers inspect the inventory
- THEN current thread creation, channel usage, pending request state, completion polling, world mutation, and shutdown assumptions are recorded
- AND production worldgen, persistence, and universal runtime behavior remain explicit non-claims.

### Requirement: Worldgen task boundary

r[valence_bevy_ecs.worldgen_tasks.boundary] Chunk generation logic MUST be a pure core over explicit generation inputs, with task spawning, polling, logging, and ECS mutation in Bevy shell systems.

#### Scenario: Generation core has no ECS access

r[valence_bevy_ecs.worldgen_tasks.boundary.core]
- GIVEN chunk generation is extracted for task orchestration
- WHEN the pure generator is invoked in tests
- THEN it produces a chunk result or typed error from explicit inputs
- AND it does not access Bevy queries, commands, resources, threads, channels, clocks, or global state.

### Requirement: Worldgen task wiring

r[valence_bevy_ecs.worldgen_tasks.wiring] Terrain background work SHOULD use Bevy task-pool integration or a documented Valence-owned task resource instead of raw per-example thread spawning.

#### Scenario: Task shell owns orchestration

r[valence_bevy_ecs.worldgen_tasks.wiring.shell]
- GIVEN a chunk request is queued
- WHEN the Bevy task shell runs
- THEN it spawns or schedules background generation through the documented task abstraction
- AND polling systems apply only completed, validated results.

### Requirement: Worldgen stale work safety

r[valence_bevy_ecs.worldgen_tasks.stale_work] Completed worldgen work MUST validate current request and view state before mutating chunk layers.

#### Scenario: Stale completion is ignored

r[valence_bevy_ecs.worldgen_tasks.stale_work.ignored]
- GIVEN a chunk generation task completes after the chunk is no longer requested
- WHEN the completion is polled
- THEN the result is ignored or recorded as stale
- AND no unrequested chunk is inserted into the layer.

### Requirement: Worldgen task tests

r[valence_bevy_ecs.worldgen_tasks.tests] Worldgen task work MUST include positive chunk completion tests and negative cancellation, duplicate request, stale view, worker failure, and shutdown tests.

#### Scenario: Valid completion mutates requested chunk

r[valence_bevy_ecs.worldgen_tasks.tests.positive]
- GIVEN a valid chunk request remains pending until generation completes
- WHEN the task shell polls the completion
- THEN the expected chunk is inserted once and the pending state is cleared.

#### Scenario: Invalid completion fails closed

r[valence_bevy_ecs.worldgen_tasks.tests.negative]
- GIVEN generation fails, duplicates a request, completes after a view change, or is observed during shutdown
- WHEN the task shell handles the completion
- THEN it reports or ignores the result deterministically
- AND no duplicate insertion, panic, or stale mutation occurs.

### Requirement: Worldgen task validation

r[valence_bevy_ecs.worldgen_tasks.validation] Worldgen task work MUST record focused terrain/example checks, positive and negative task tests, Cairn gates, Cairn validation, and task-evidence checks before archive.

#### Scenario: Worldgen task closeout is reviewable

r[valence_bevy_ecs.worldgen_tasks.validation.log]
- GIVEN worldgen task orchestration is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show pure generation tests, task shell tests, stale-work tests, focused terrain/example checks, Cairn proposal/design/tasks gates, Cairn validation, and task-evidence validation.
