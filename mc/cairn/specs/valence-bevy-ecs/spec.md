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

### Requirement: Gameplay plugin inventory

r[valence_bevy_ecs.gameplay_plugins.inventory] Valence gameplay example plugin work MUST inventory selected example Bevy systems, schedules, resources, events, env toggles, compatibility milestones, and non-goals before refactoring wiring.

#### Scenario: Example wiring is reviewable

r[valence_bevy_ecs.gameplay_plugins.inventory.reviewable]
- GIVEN a Valence gameplay or compatibility example is selected for plugin organization
- WHEN reviewers inspect the inventory
- THEN each current system, schedule label, resource, event, env toggle, milestone emitter, and evidence boundary is classified
- AND production gameplay, vanilla parity, and default Valence behavior remain explicit non-claims unless separately scoped.

### Requirement: Gameplay plugin contract

r[valence_bevy_ecs.gameplay_plugins.contract] Extracted gameplay example plugins SHOULD expose named Bevy `SystemSet`s for stable input, rule evaluation, world mutation, presentation, and cleanup ordering.

#### Scenario: Sets describe schedule phases

r[valence_bevy_ecs.gameplay_plugins.contract.phases]
- GIVEN an extracted example plugin registers systems
- WHEN reviewers inspect its schedule contract
- THEN the systems are grouped into documented phase sets
- AND user code can order around those sets without depending on anonymous tuple order.

### Requirement: Gameplay plugin wiring

r[valence_bevy_ecs.gameplay_plugins.wiring] Example plugins MUST keep deterministic gameplay and compatibility decisions outside Bevy ECS access unless the code is only an adapter shell.

#### Scenario: Plugin remains a shell

r[valence_bevy_ecs.gameplay_plugins.wiring.shell]
- GIVEN a CTF, survival, or terrain decision is migrated during plugin organization
- WHEN the implementation is reviewed
- THEN pure decisions consume explicit inputs and return decisions or mutation requests
- AND Bevy queries, commands, resources, logging, file I/O, and world mutation remain in thin systems.

### Requirement: Gameplay compatibility preservation

r[valence_bevy_ecs.gameplay_plugins.compatibility] Plugin organization MUST preserve selected example commands, env var contracts, milestone text, scenario behavior, and evidence non-claim boundaries unless another Cairn changes them.

#### Scenario: Fixture receipts remain comparable

r[valence_bevy_ecs.gameplay_plugins.compatibility.receipts]
- GIVEN selected compatibility scenarios run after plugin organization
- WHEN their receipts and logs are compared against the pre-refactor contract
- THEN required milestones, forbidden milestones, env toggles, and non-claim fields remain compatible
- AND no default Valence gameplay, production-readiness, or vanilla-parity claim is added.

### Requirement: Gameplay plugin tests

r[valence_bevy_ecs.gameplay_plugins.tests] Gameplay plugin organization MUST include positive plugin/schedule smoke tests and negative disabled-plugin or ordering regression tests.

#### Scenario: Positive plugin smoke passes

r[valence_bevy_ecs.gameplay_plugins.tests.positive]
- GIVEN an extracted example plugin is added to a minimal test app
- WHEN the app schedules are initialized
- THEN required resources, events, system sets, and schedule labels are present.

#### Scenario: Disabled plugin does not install behavior

r[valence_bevy_ecs.gameplay_plugins.tests.negative]
- GIVEN the extracted plugin is not added to a minimal test app
- WHEN the app schedules are inspected or updated
- THEN plugin-owned resources, events, and gameplay systems are absent
- AND no compatibility milestone can be emitted by that plugin.

### Requirement: Gameplay plugin validation

r[valence_bevy_ecs.gameplay_plugins.validation] Gameplay plugin organization MUST record focused example checks, selected compatibility rails when touched, Cairn gates, Cairn validation, and task-evidence checks before archive.

#### Scenario: Plugin closeout is reviewable

r[valence_bevy_ecs.gameplay_plugins.validation.log]
- GIVEN gameplay plugin organization is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show focused example checks, positive and negative plugin tests, selected mc-compat dry-runs if fixture behavior changed, Cairn proposal/design/tasks gates, Cairn validation, and task-evidence validation.

### Requirement: Typed event inventory

r[valence_bevy_ecs.typed_events.inventory] Typed event work MUST inventory selected direct `PacketEvent` consumers, decoded packet types, duplicate decode paths, emitted gameplay semantics, and current malformed-input behavior before adding adapters.

#### Scenario: Packet consumers are reviewable

r[valence_bevy_ecs.typed_events.inventory.reviewable]
- GIVEN a packet-derived gameplay semantic is selected for typed event promotion
- WHEN reviewers inspect the inventory
- THEN current raw packet readers, decode sites, event-loop phases, mutation targets, and error handling are recorded
- AND unsupported or low-level packet access remains distinguished from typed gameplay semantics.

### Requirement: Typed event contract

r[valence_bevy_ecs.typed_events.contract] Each promoted packet-derived event MUST define source client, timing metadata, decoded fields, diagnostic behavior, schedule phase, and malformed-input semantics.

#### Scenario: Malformed packet contract is explicit

r[valence_bevy_ecs.typed_events.contract.malformed]
- GIVEN a selected packet fails ID match, decode, full-consumption, or client-liveness validation
- WHEN the typed event adapter evaluates it
- THEN the contract states whether no event, rejection event, or diagnostic is emitted
- AND gameplay mutation does not proceed from invalid input.

### Requirement: Typed event adapters

r[valence_bevy_ecs.typed_events.adapters] Packet-boundary adapter systems SHOULD decode selected raw packets once and emit typed Bevy events in documented event-loop phases.

#### Scenario: Adapter emits one semantic event

r[valence_bevy_ecs.typed_events.adapters.once]
- GIVEN a valid selected packet arrives from a live client
- WHEN the adapter system runs in the documented phase
- THEN exactly one typed semantic event is emitted for that packet
- AND downstream gameplay systems do not need to decode the raw packet body again.

### Requirement: Typed event compatibility

r[valence_bevy_ecs.typed_events.compatibility] Typed event promotion MUST keep raw `PacketEvent` access available for low-level users and preserve selected fixture behavior unless another Cairn changes it.

#### Scenario: Raw packet access remains available

r[valence_bevy_ecs.typed_events.compatibility.raw_access]
- GIVEN a plugin needs unsupported or low-level packet access
- WHEN typed event adapters are installed
- THEN raw `PacketEvent` remains readable according to the existing event-loop contract
- AND typed adapters document which semantics they own to avoid double handling.

### Requirement: Typed event tests

r[valence_bevy_ecs.typed_events.tests] Typed event promotion MUST include positive valid-packet tests and negative wrong-id, partial-decode, malformed-payload, stale-client, and duplicate-emission tests.

#### Scenario: Valid packet emits typed event

r[valence_bevy_ecs.typed_events.tests.positive]
- GIVEN a valid selected packet event from a live client
- WHEN the adapter runs
- THEN the expected typed event is emitted with documented fields.

#### Scenario: Invalid packet emits no false action

r[valence_bevy_ecs.typed_events.tests.negative]
- GIVEN a wrong-id, partial-decode, malformed-payload, stale-client, or duplicate packet input
- WHEN the adapter runs
- THEN no false gameplay action is emitted
- AND diagnostics or rejection events match the typed event contract.

### Requirement: Typed event validation

r[valence_bevy_ecs.typed_events.validation] Typed event promotion MUST record focused event-loop/interaction checks, selected compatibility rails when touched, Cairn gates, Cairn validation, and task-evidence checks before archive.

#### Scenario: Typed event closeout is reviewable

r[valence_bevy_ecs.typed_events.validation.log]
- GIVEN typed event promotion is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show positive adapter tests, negative malformed-input tests, focused Valence checks, selected mc-compat dry-runs if fixture input handling changed, Cairn proposal/design/tasks gates, Cairn validation, and task-evidence validation.
