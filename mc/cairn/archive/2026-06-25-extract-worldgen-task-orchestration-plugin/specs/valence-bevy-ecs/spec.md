# valence-bevy-ecs Change Spec: Worldgen task orchestration plugin

## Requirements

### Requirement: Worldgen task plugin inventory

r[valence_bevy_ecs.worldgen_task_plugin.inventory] Worldgen task plugin work MUST inventory current terrain background work, task inputs, pending request state, polling order, cancellation, stale-work validation, mutation targets, shutdown assumptions, and dependency impact before changing orchestration.

#### Scenario: Task orchestration baseline is reviewable

r[valence_bevy_ecs.worldgen_task_plugin.inventory.reviewable]
- GIVEN terrain worldgen task orchestration is selected for extraction
- WHEN reviewers inspect the inventory
- THEN current request collection, task pool usage, queued/generating state, polling order, completion handling, stale validation, mutation targets, and shutdown assumptions are recorded
- AND production worldgen, persistence, vanilla parity, and universal runtime behavior remain explicit non-claims.

### Requirement: Worldgen task boundary

r[valence_bevy_ecs.worldgen_task_plugin.boundary] Worldgen task orchestration MUST keep chunk generation logic as a pure core over explicit inputs, with task spawning, polling, logging, and ECS mutation in Bevy shell systems.

#### Scenario: Generator has no ECS access

r[valence_bevy_ecs.worldgen_task_plugin.boundary.core]
- GIVEN chunk generation is invoked through the task orchestration shell
- WHEN the pure generator is tested directly
- THEN it produces a typed chunk result or typed error from explicit inputs
- AND it does not access Bevy queries, commands, resources, threads, channels, clocks, logging, or global state.

### Requirement: Worldgen task plugin wiring

r[valence_bevy_ecs.worldgen_task_plugin.wiring] Terrain background work SHOULD use an opt-in Bevy task resource/plugin shell that owns spawning, polling, cancellation, and completion routing without giving generators ECS access.

#### Scenario: Task shell owns lifecycle

r[valence_bevy_ecs.worldgen_task_plugin.wiring.shell]
- GIVEN a valid chunk request is queued
- WHEN the task shell runs
- THEN it schedules background generation through the documented task abstraction
- AND polling systems emit or apply only completed, validated results.

### Requirement: Worldgen stale-work safety

r[valence_bevy_ecs.worldgen_task_plugin.stale_work] Worldgen task plugin work MUST validate current request ownership, view state, already-loaded state, and shutdown state before mutating chunk layers from completed work.

#### Scenario: Stale completion is ignored

r[valence_bevy_ecs.worldgen_task_plugin.stale_work.ignored]
- GIVEN a chunk generation task completes after the chunk is no longer requested, already loaded, cancelled, or observed during shutdown
- WHEN the completion is handled
- THEN the result is ignored or diagnosed deterministically
- AND no duplicate or unrequested chunk is inserted into the layer.

### Requirement: Worldgen task plugin tests

r[valence_bevy_ecs.worldgen_task_plugin.tests] Worldgen task plugin work MUST include positive completion tests and negative duplicate request, stale view, worker failure, cancellation, shutdown, and plugin-disabled tests.

#### Scenario: Valid completion applies once

r[valence_bevy_ecs.worldgen_task_plugin.tests.positive]
- GIVEN a valid chunk request remains current until generation completes
- WHEN the task shell polls the completion
- THEN the expected completion is emitted or inserted once and pending state is cleared.

#### Scenario: Invalid completion fails closed

r[valence_bevy_ecs.worldgen_task_plugin.tests.negative]
- GIVEN generation fails, duplicates a request, completes after a view change, is cancelled, targets already-loaded state, or is observed during shutdown
- WHEN the task shell handles the completion
- THEN it reports or ignores the result deterministically
- AND no duplicate insertion, stale mutation, false completion event, or panic occurs.

### Requirement: Worldgen task plugin validation

r[valence_bevy_ecs.worldgen_task_plugin.validation] Worldgen task plugin work MUST record pure generation tests, task shell tests, terrain example checks, schedule hygiene when plugin wiring changes, Cairn gates, Cairn validation, and task-evidence checks before archive.

#### Scenario: Worldgen task plugin closeout is reviewable

r[valence_bevy_ecs.worldgen_task_plugin.validation.log]
- GIVEN worldgen task orchestration plugin work is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show pure generation tests, positive and negative task-shell tests, stale-work tests, terrain example checks, schedule hygiene when applicable, Cairn proposal/design/tasks gates, Cairn validation, and task-evidence validation.
