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

### Requirement: Event-loop phase inventory

r[valence_bevy_ecs.event_loop_phase_sets.inventory] Event-loop phase set work MUST inventory current event-loop schedules, raw packet event production, adapter systems, typed event consumers, diagnostics, cleanup systems, and schedule-impacting checks before changing event-loop schedule wiring.

#### Scenario: Event-loop baseline is reviewable

r[valence_bevy_ecs.event_loop_phase_sets.inventory.reviewable]
- GIVEN event-loop phase set work is selected
- WHEN reviewers inspect the inventory
- THEN raw packet production, raw packet readers, typed adapters, domain consumers, diagnostics, cleanup systems, schedule labels, and existing ordering constraints are recorded
- AND low-level raw access is distinguished from typed event ownership.

### Requirement: Event-loop phase set contract

r[valence_bevy_ecs.event_loop_phase_sets.contract] Event-loop schedules SHOULD expose named `SystemSet`s for raw packet observation, typed adapter emission, domain consumption, diagnostics, and cleanup where those phases exist.

#### Scenario: Event-loop phases are orderable

r[valence_bevy_ecs.event_loop_phase_sets.contract.phases]
- GIVEN a plugin needs to run around event-loop packet processing or typed adapter emission
- WHEN it orders relative to event-loop phase sets
- THEN it can target named sets instead of anonymous schedule order
- AND documentation states which event-loop ordering boundaries are stable and which remain private.

### Requirement: Event-loop phase set wiring

r[valence_bevy_ecs.event_loop_phase_sets.wiring] Event-loop phase set wiring MUST preserve packet/event semantics, raw `PacketEvent` access, typed event timing, diagnostics, and default plugin behavior unless another Cairn changes them.

#### Scenario: Event-loop behavior remains compatible

r[valence_bevy_ecs.event_loop_phase_sets.wiring.preserve]
- GIVEN selected event-loop systems move into named sets
- WHEN focused event-loop and typed event checks run
- THEN raw packet readers, typed adapters, diagnostics, and domain consumers observe compatible ordering and data
- AND default Valence plugin behavior does not change.

### Requirement: Event-loop compatibility boundary

r[valence_bevy_ecs.event_loop_phase_sets.compatibility] Event-loop phase set work MUST document private ordering boundaries and preserve compatibility for raw `PacketEvent` readers and typed event consumers.

#### Scenario: Raw packet access remains compatible

r[valence_bevy_ecs.event_loop_phase_sets.compatibility.raw_access]
- GIVEN a low-level plugin reads raw `PacketEvent`
- WHEN event-loop phase sets are installed
- THEN raw `PacketEvent` remains readable according to the existing event-loop contract
- AND typed adapters document which semantics they own to avoid double handling.

### Requirement: Event-loop phase set tests

r[valence_bevy_ecs.event_loop_phase_sets.tests] Event-loop phase set work MUST include positive schedule tests and negative missing-set, ambiguity, duplicate adapter, raw-access, and disabled-plugin tests.

#### Scenario: Event-loop sets install correctly

r[valence_bevy_ecs.event_loop_phase_sets.tests.positive]
- GIVEN event-loop plugins are added to a minimal test app with required schedules
- WHEN schedules are initialized
- THEN expected event-loop phase sets, raw packet events, typed adapters, and diagnostics surfaces are present.

#### Scenario: Event-loop ordering regression fails clearly

r[valence_bevy_ecs.event_loop_phase_sets.tests.negative]
- GIVEN a missing set, ambiguous ordering, duplicate adapter emission, broken raw access, or disabled plugin configuration
- WHEN schedule or adapter tests run
- THEN the regression fails clearly
- AND no false typed event, duplicate semantic action, stale diagnostic, or raw-access removal occurs.

### Requirement: Event-loop phase set validation

r[valence_bevy_ecs.event_loop_phase_sets.validation] Event-loop phase set work MUST record focused event-loop checks, typed event checks when touched, schedule hygiene, selected compatibility rails when fixture input handling changes, Cairn gates, Cairn validation, and task-evidence checks before archive.

#### Scenario: Event-loop phase closeout is reviewable

r[valence_bevy_ecs.event_loop_phase_sets.validation.log]
- GIVEN event-loop phase set work is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show focused event-loop checks, typed event checks when applicable, positive and negative schedule tests, schedule hygiene, selected mc-compat rails when applicable, Cairn proposal/design/tasks gates, Cairn validation, and task-evidence validation.

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

### Requirement: Entity state inventory

r[valence_bevy_ecs.entity_state.inventory] Valence Bevy state migration work MUST inventory selected entity-keyed, player-keyed, and collection-backed runtime state before changing ownership.

#### Scenario: State ownership inputs are visible

r[valence_bevy_ecs.entity_state.inventory.visible]
- GIVEN an example or fixture shell is selected for state migration
- WHEN reviewers inspect the inventory
- THEN each targeted `Entity`, player, UUID, username, or collection key is recorded with current owner, lifecycle, cleanup behavior, and consumer systems
- AND maps that are team, layer, registry, or external identity indexes are distinguished from entity-owned state.

### Requirement: Entity state classification

r[valence_bevy_ecs.entity_state.classification] Targeted runtime state MUST be classified as entity-owned component state, global resource state, pure core state, index/cache state, or external identity state before migration.

#### Scenario: Component candidate is justified

r[valence_bevy_ecs.entity_state.classification.component_candidate]
- GIVEN a state value is keyed by a live ECS entity
- WHEN its ownership classification is reviewed
- THEN the classification explains whether the value should be stored as a component
- AND any decision to keep it in a resource records the cleanup or indexing reason.

### Requirement: Entity state components

r[valence_bevy_ecs.entity_state.components] State whose lifecycle is owned by a live ECS entity SHOULD be represented by Bevy components or bundles rather than external entity-keyed collections.

#### Scenario: Despawn cleans entity-owned state

r[valence_bevy_ecs.entity_state.components.despawn]
- GIVEN entity-owned state has migrated to a component
- WHEN the entity is despawned or loses the owning role
- THEN the state is removed through normal component/entity lifecycle
- AND no stale entity-keyed map entry is required for correctness.

### Requirement: Resource ownership remains explicit

r[valence_bevy_ecs.entity_state.resources] Global policy, team/layer maps, registries, deterministic pure-core state, and intentional indexes MAY remain resources when ownership is documented.

#### Scenario: Resource is intentionally global

r[valence_bevy_ecs.entity_state.resources.global]
- GIVEN a collection remains a resource after state migration
- WHEN reviewers inspect it
- THEN its key space, lifecycle, cleanup behavior, and reason for not being component-owned are documented
- AND stale entity entries are either impossible or covered by cleanup tests.

### Requirement: Entity state tests

r[valence_bevy_ecs.entity_state.tests] Entity state migration MUST include positive lifecycle tests and negative stale-entity, despawn, duplicate ownership, and reconnect tests for changed state.

#### Scenario: Valid lifecycle is preserved

r[valence_bevy_ecs.entity_state.tests.positive]
- GIVEN a valid player or fixture entity gains the migrated state
- WHEN systems run through the documented lifecycle
- THEN queries observe the expected component/resource state and fixture decisions remain compatible.

#### Scenario: Stale entity fails closed

r[valence_bevy_ecs.entity_state.tests.negative]
- GIVEN an entity despawns, reconnects, or appears in a stale index
- WHEN migrated systems process the state
- THEN stale ownership is ignored, cleaned, or diagnosed deterministically
- AND no false milestone, duplicate ownership, or panic occurs.

### Requirement: Entity state validation

r[valence_bevy_ecs.entity_state.validation] Entity state migration MUST record focused checks, selected compatibility rails when touched, Cairn gates, Cairn validation, and task-evidence checks before archive.

#### Scenario: Entity state closeout is reviewable

r[valence_bevy_ecs.entity_state.validation.log]
- GIVEN entity state migration is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show positive lifecycle tests, negative stale-state tests, focused Valence/example checks, selected mc-compat dry-runs if fixture behavior changed, Cairn proposal/design/tasks gates, Cairn validation, and task-evidence validation.

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

### Requirement: Inventory schedule inventory

r[valence_bevy_ecs.inventory_sets.inventory] Inventory SystemSet work MUST inventory current `InventoryPlugin` systems, schedules, tuple ordering, resources, events, feature/default plugin membership, and downstream ordering dependencies before changing schedule wiring.

#### Scenario: Inventory schedule baseline is reviewable

r[valence_bevy_ecs.inventory_sets.inventory.reviewable]
- GIVEN inventory schedule work is selected
- WHEN reviewers inspect the inventory
- THEN each current inventory system records its schedule label, ordering constraints, resource access, event access, and mutation target
- AND feature gates, default plugin membership, and downstream ordering assumptions are recorded.

### Requirement: Inventory SystemSet contract

r[valence_bevy_ecs.inventory_sets.contract] `InventoryPlugin` SHOULD expose named Bevy `SystemSet`s for stable packet input, model mutation, viewer/window synchronization, presentation or flush preparation, and cleanup ordering where those phases exist.

#### Scenario: Inventory phases are orderable

r[valence_bevy_ecs.inventory_sets.contract.phases]
- GIVEN a user plugin needs to run around inventory behavior
- WHEN it orders relative to inventory schedule phases
- THEN it can target named inventory sets instead of relying on anonymous tuple order
- AND the set documentation states which phases are public ordering contracts.

### Requirement: Inventory SystemSet wiring

r[valence_bevy_ecs.inventory_sets.wiring] Inventory SystemSet wiring MUST preserve existing inventory events, resources, packet behavior, feature gates, and default plugin membership unless another Cairn changes them.

#### Scenario: Wiring preserves behavior

r[valence_bevy_ecs.inventory_sets.wiring.preserve]
- GIVEN existing inventory behavior is moved into named sets
- WHEN focused inventory tests and schedule checks run
- THEN selected packet input, inventory mutation, viewer/window synchronization, and packet flush preparation remain compatible with the baseline
- AND default Valence plugin membership does not change.

### Requirement: Inventory schedule compatibility

r[valence_bevy_ecs.inventory_sets.compatibility] Inventory SystemSet work MUST document downstream compatibility boundaries and schedule non-claims for ordering points that remain intentionally private.

#### Scenario: Non-claims are explicit

r[valence_bevy_ecs.inventory_sets.compatibility.non_claims]
- GIVEN an inventory ordering point is not promoted to a public or crate-visible set
- WHEN reviewers inspect the schedule contract
- THEN the implementation explains why anonymous or private ordering remains sufficient
- AND no broad inventory compatibility, vanilla parity, or production-readiness claim is added.

### Requirement: Inventory SystemSet tests

r[valence_bevy_ecs.inventory_sets.tests] Inventory SystemSet work MUST include positive schedule/plugin smoke tests and negative disabled-plugin or ordering-regression tests.

#### Scenario: Inventory plugin installs expected sets

r[valence_bevy_ecs.inventory_sets.tests.positive]
- GIVEN `InventoryPlugin` is added to a minimal test app with required dependencies
- WHEN schedules are initialized
- THEN expected inventory events, resources, sets, and schedule labels are present.

#### Scenario: Disabled inventory plugin is absent

r[valence_bevy_ecs.inventory_sets.tests.negative]
- GIVEN the inventory plugin is not added or is disabled through a plugin group comparison
- WHEN schedules and resources are inspected
- THEN inventory-owned resources, events, systems, and sets are absent
- AND no inventory schedule contract is falsely reported as installed.

### Requirement: Inventory SystemSet validation

r[valence_bevy_ecs.inventory_sets.validation] Inventory SystemSet work MUST record focused inventory checks, Valence schedule hygiene, selected example checks when touched, Cairn gates, Cairn validation, and task-evidence checks before archive.

#### Scenario: Inventory schedule closeout is reviewable

r[valence_bevy_ecs.inventory_sets.validation.log]
- GIVEN inventory SystemSet work is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show focused inventory tests, positive and negative schedule tests, Valence schedule hygiene, selected example checks when applicable, Cairn proposal/design/tasks gates, Cairn validation, and task-evidence validation.

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
