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

### Requirement: Packet semantic inventory

r[valence_bevy_ecs.packet_semantic_events.inventory] Remaining packet semantic event work MUST inventory selected direct `PacketEvent` consumers, decoded packet types, schedule phases, mutation targets, duplicate decode paths, and current malformed-input behavior before adding adapters.

#### Scenario: Packet semantics are reviewable

r[valence_bevy_ecs.packet_semantic_events.inventory.reviewable]
- GIVEN a remaining packet-derived gameplay, inventory, command, or fixture semantic is selected for promotion
- WHEN reviewers inspect the inventory
- THEN current raw packet readers, packet types, decode sites, event-loop phases, downstream mutations, and diagnostics are recorded
- AND unsupported or low-level raw-packet access remains distinguished from promoted semantics.

### Requirement: Packet semantic event contract

r[valence_bevy_ecs.packet_semantic_events.contract] Each promoted packet semantic MUST define typed event ownership, source client, timing metadata, decoded fields, diagnostics, stale-client behavior, schedule phase, and raw-packet compatibility.

#### Scenario: Ownership avoids double handling

r[valence_bevy_ecs.packet_semantic_events.contract.ownership]
- GIVEN raw and typed packet surfaces coexist for a selected semantic
- WHEN the event contract is reviewed
- THEN it names which adapter emits the typed event and which downstream systems consume it
- AND it states whether raw packet consumers may still observe or act on the same packet.

### Requirement: Packet semantic adapters

r[valence_bevy_ecs.packet_semantic_events.adapters] Packet semantic adapter systems SHOULD decode selected raw packets once and emit typed Bevy events in documented event-loop phases.

#### Scenario: Adapter emits one typed semantic

r[valence_bevy_ecs.packet_semantic_events.adapters.once]
- GIVEN a valid selected packet arrives from a live client
- WHEN the adapter system runs in the documented event-loop phase
- THEN exactly one typed semantic event is emitted for that packet
- AND downstream systems do not need to decode the raw packet body again.

### Requirement: Raw packet compatibility

r[valence_bevy_ecs.packet_semantic_events.compatibility] Remaining packet semantic event work MUST keep raw `PacketEvent` access available for low-level users and preserve selected fixture behavior unless another Cairn changes it.

#### Scenario: Fixture receipts remain comparable

r[valence_bevy_ecs.packet_semantic_events.compatibility.fixtures]
- GIVEN a compatibility fixture migrates from a raw packet reader to a typed semantic event
- WHEN its focused rail runs after migration
- THEN required milestones, forbidden milestones, malformed-input behavior, and non-claim fields remain compatible
- AND no broad compatibility, vanilla parity, or production-readiness claim is added.

### Requirement: Packet semantic event tests

r[valence_bevy_ecs.packet_semantic_events.tests] Packet semantic event work MUST include positive valid-packet tests and negative wrong-id, malformed-payload, partial-decode, stale-client, duplicate-emission, and raw-access regression tests.

#### Scenario: Valid packet emits typed event

r[valence_bevy_ecs.packet_semantic_events.tests.positive]
- GIVEN a valid selected packet event from a live client
- WHEN the adapter runs
- THEN the expected typed event is emitted with documented fields.

#### Scenario: Invalid packet fails closed

r[valence_bevy_ecs.packet_semantic_events.tests.negative]
- GIVEN a wrong-id, malformed-payload, partial-decode, stale-client, or duplicate packet input
- WHEN the adapter runs
- THEN no false gameplay, inventory, command, or fixture action is emitted
- AND diagnostics or rejection behavior match the typed event contract.

### Requirement: Packet semantic event validation

r[valence_bevy_ecs.packet_semantic_events.validation] Packet semantic event work MUST record focused Valence checks, schedule hygiene when registrations change, selected compatibility rails when fixture behavior changes, Cairn gates, Cairn validation, and task-evidence checks before archive.

#### Scenario: Packet semantic closeout is reviewable

r[valence_bevy_ecs.packet_semantic_events.validation.log]
- GIVEN remaining packet semantic event work is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show positive adapter tests, negative malformed-input tests, focused Valence checks, schedule hygiene when applicable, selected mc-compat rails when applicable, Cairn proposal/design/tasks gates, Cairn validation, and task-evidence validation.

### Requirement: Packet compose core

r[valence_bevy_ecs.packet_compose.core] Valence packet composition SHOULD expose pure cores for packet field selection, ordering facts, packet family plans, and shared packet-plan helpers.

#### Scenario: Packet plan is testable without client shell

r[valence_bevy_ecs.packet_compose.core.testable]
- GIVEN internal state summaries and protocol facts
- WHEN the packet compose core processes them
- THEN the packet plan can be tested without client access, layer access, packet writes, Bevy systems, or logging.

### Requirement: Packet compose shell boundary

r[valence_bevy_ecs.packet_compose.shell_boundary] Packet-compose extraction MUST keep client access, layer access, packet writes, Bevy systems, and logging outside pure packet-compose cores.

#### Scenario: Packet side effects remain in shell

r[valence_bevy_ecs.packet_compose.shell_boundary.effects]
- GIVEN the packet compose core returns a packet plan
- WHEN the Valence shell applies that plan
- THEN only the shell reads live clients/layers, writes packets, wires systems, or logs diagnostics.

### Requirement: Packet compose parity

r[valence_bevy_ecs.packet_compose.parity] Packet-compose extraction MUST preserve packet bytes and fields, public APIs, ordering behavior, protocol assumptions, and evidence non-claims.

#### Scenario: Packet behavior remains stable

r[valence_bevy_ecs.packet_compose.parity.stable]
- GIVEN a supported pre-refactor packet composition input
- WHEN the extracted packet core and shell process the same input
- THEN packet fields, ordering, public API behavior, and non-claim boundaries remain equivalent.

### Requirement: Packet compose positive tests

r[valence_bevy_ecs.packet_compose.positive_tests] The change MUST include positive tests for representative packet family plans, field selection, ordering, defaults, and protocol-specific assumptions.

#### Scenario: Supported packet paths pass

r[valence_bevy_ecs.packet_compose.positive_tests.coverage]
- GIVEN representative supported packet compose inputs
- WHEN extracted packet cores process them
- THEN tests prove the expected packet plans are produced.

### Requirement: Packet compose negative tests

r[valence_bevy_ecs.packet_compose.negative_tests] The change MUST include negative tests for missing state, invalid entity or layer facts, unsupported packet variants, malformed inputs, and ordering violations.

#### Scenario: Invalid packet paths fail closed

r[valence_bevy_ecs.packet_compose.negative_tests.fail_closed]
- GIVEN invalid packet compose inputs
- WHEN extracted packet cores process them
- THEN tests prove the inputs are rejected, defaulted, or contained according to current behavior.

### Requirement: Packet compose validation

r[valence_bevy_ecs.packet_compose.validation] The change MUST record focused packet compose/server tests, affected checks, Cairn proposal/design/tasks gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[valence_bevy_ecs.packet_compose.validation.logs]
- GIVEN packet-compose extraction is complete
- WHEN the change is closed
- THEN reviewable logs show positive and negative packet-compose tests plus affected checks and Cairn gates passing.

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

### Requirement: Fixture component-state inventory

r[valence_bevy_ecs.fixture_component_state.inventory] Fixture component-state work MUST inventory selected fixture/example state keyed by client, entity, username, UUID, container, mob/drop, or visual companion, including owner, lifecycle, cleanup behavior, and consumers before changing ownership.

#### Scenario: Fixture state ownership is visible

r[valence_bevy_ecs.fixture_component_state.inventory.visible]
- GIVEN a fixture or example state value is selected for component migration
- WHEN reviewers inspect the inventory
- THEN its current key space, owner, lifecycle, cleanup behavior, consumer systems, and stale-reference risk are recorded
- AND true globals, indexes, and external identity state are distinguished from entity-owned state.

### Requirement: Fixture component-state classification

r[valence_bevy_ecs.fixture_component_state.classification] Targeted fixture/example state MUST be classified as entity-owned component data, global resource data, pure-core state, index/cache state, or external identity state before migration.

#### Scenario: Component candidate is justified

r[valence_bevy_ecs.fixture_component_state.classification.component]
- GIVEN a state value is keyed by or belongs to a live ECS entity
- WHEN its ownership classification is reviewed
- THEN the classification explains whether the value should migrate to a component or bundle
- AND any decision to keep it in a resource records the cleanup, indexing, or external identity reason.

### Requirement: Fixture component ownership

r[valence_bevy_ecs.fixture_component_state.components] Fixture/example state whose lifecycle is owned by a live ECS entity SHOULD be represented by Bevy components or bundles rather than external entity-keyed collections.

#### Scenario: Entity lifecycle owns fixture state

r[valence_bevy_ecs.fixture_component_state.components.lifecycle]
- GIVEN entity-owned fixture state has migrated to a component or bundle
- WHEN the entity despawns, reconnects, or loses the owning role
- THEN the state is removed through normal component/entity lifecycle or explicit component cleanup
- AND no stale entity-keyed map entry is required for correctness.

### Requirement: Fixture component compatibility

r[valence_bevy_ecs.fixture_component_state.compatibility] Fixture component-state work MUST preserve fixture milestones, env/CLI contracts, selected behavior, cleanup semantics, and compatibility non-claims unless another Cairn changes them.

#### Scenario: Fixture behavior remains comparable

r[valence_bevy_ecs.fixture_component_state.compatibility.receipts]
- GIVEN a selected fixture runs after state migration
- WHEN its focused rail or tests are compared against the baseline
- THEN milestones, forbidden milestones, env/CLI contracts, cleanup behavior, and non-claim fields remain compatible
- AND no broad compatibility, vanilla parity, or production-readiness claim is added.

### Requirement: Fixture component-state tests

r[valence_bevy_ecs.fixture_component_state.tests] Fixture component-state work MUST include positive lifecycle tests and negative stale-entity, despawn, duplicate ownership, reconnect, and plugin-disabled tests for changed state.

#### Scenario: Valid lifecycle is preserved

r[valence_bevy_ecs.fixture_component_state.tests.positive]
- GIVEN a valid client or fixture entity gains migrated component state
- WHEN systems run through the documented lifecycle
- THEN queries observe expected component/resource state and fixture decisions remain compatible.

#### Scenario: Stale fixture state fails closed

r[valence_bevy_ecs.fixture_component_state.tests.negative]
- GIVEN an entity despawns, reconnects, appears in a stale index, or duplicate ownership is attempted
- WHEN migrated systems process state
- THEN stale ownership is ignored, cleaned, or diagnosed deterministically
- AND no false milestone, duplicate ownership, stale mutation, or panic occurs.

### Requirement: Fixture component-state validation

r[valence_bevy_ecs.fixture_component_state.validation] Fixture component-state work MUST record focused example/crate checks, selected compatibility rails when fixture behavior changes, schedule hygiene when plugin wiring changes, Cairn gates, Cairn validation, and task-evidence checks before archive.

#### Scenario: Fixture component-state closeout is reviewable

r[valence_bevy_ecs.fixture_component_state.validation.log]
- GIVEN fixture component-state work is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show positive lifecycle tests, negative stale-state tests, focused example/crate checks, selected mc-compat rails when applicable, schedule hygiene when applicable, Cairn proposal/design/tasks gates, Cairn validation, and task-evidence validation.

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

### Requirement: Gameplay plugin contract inventory

r[valence_bevy_ecs.gameplay_plugin_contracts.inventory] Shared gameplay plugin contract work MUST inventory selected gameplay/example plugin phase sets, contract resources, schedule labels, resources, events, disabled-plugin tests, and private ordering points before changing shared wiring.

#### Scenario: Existing plugin contracts are reviewable

r[valence_bevy_ecs.gameplay_plugin_contracts.inventory.reviewable]
- GIVEN a gameplay or example plugin is selected for shared contract work
- WHEN reviewers inspect the inventory
- THEN current phase names, schedule labels, owned resources, events, plugin install mode, disabled-plugin behavior, and private ordering points are recorded
- AND CTF, survival compatibility, terrain, smaller example plugins, and out-of-scope BedWars/Hyperion boundaries are classified explicitly.

### Requirement: Shared gameplay phase contract

r[valence_bevy_ecs.gameplay_plugin_contracts.phase_contract] Opt-in Valence gameplay plugins SHOULD use a shared phase vocabulary for input, rule evaluation, world mutation, presentation, and cleanup where those phases exist.

#### Scenario: Plugins can order around shared phases

r[valence_bevy_ecs.gameplay_plugin_contracts.phase_contract.orderable]
- GIVEN multiple opt-in gameplay plugins are installed in one app
- WHEN a downstream system needs to run around gameplay input, rule evaluation, world mutation, presentation, or cleanup
- THEN it can target the shared phase contract instead of relying on plugin-local anonymous tuple order
- AND plugin-local subphases remain private unless deliberately promoted.

### Requirement: Gameplay plugin contract metadata

r[valence_bevy_ecs.gameplay_plugin_contracts.metadata] Shared gameplay plugin contracts MUST expose or record minimal metadata for schedule labels, phase order, owned resources, owned events, scope model, installation mode, and non-claim boundaries.

#### Scenario: Contract metadata explains installed behavior

r[valence_bevy_ecs.gameplay_plugin_contracts.metadata.inspectable]
- GIVEN a gameplay plugin is installed in a minimal test app
- WHEN tests or reviewers inspect its contract metadata
- THEN the contract names installed schedules, phase order, owned resources, owned events, expected gameplay scope model, and whether the plugin is default, feature-gated, or explicitly opt-in
- AND non-claims for dynamic plugins, default gameplay, vanilla parity, production readiness, and BedWars/Hyperion scope are visible.

### Requirement: Gameplay plugin contract tests

r[valence_bevy_ecs.gameplay_plugin_contracts.tests] Shared gameplay plugin contract work MUST include reusable positive contract tests and negative disabled-plugin or ordering-regression tests for selected plugins.

#### Scenario: Installed plugin passes shared contract checks

r[valence_bevy_ecs.gameplay_plugin_contracts.tests.positive]
- GIVEN a selected gameplay plugin is added to a minimal test app with required schedules
- WHEN shared contract helpers inspect schedules and resources
- THEN expected phase sets, contract metadata, resources, events, and schedule labels are present.

#### Scenario: Missing plugin fails closed

r[valence_bevy_ecs.gameplay_plugin_contracts.tests.negative]
- GIVEN a selected gameplay plugin is not added or an ordering fixture omits a required phase
- WHEN shared contract helpers inspect the app
- THEN plugin-owned resources, events, systems, sets, and contract metadata are absent or the ordering failure is diagnosed clearly
- AND no false gameplay contract is reported as installed.

### Requirement: Gameplay plugin compatibility boundaries

r[valence_bevy_ecs.gameplay_plugin_contracts.compatibility] Shared gameplay plugin contracts MUST preserve example behavior, command/env/CLI contracts, compatibility milestones, and non-claim scope unless another Cairn changes them.

#### Scenario: Shared contracts do not promote gameplay claims

r[valence_bevy_ecs.gameplay_plugin_contracts.compatibility.non_claims]
- GIVEN an example plugin adopts the shared gameplay contract
- WHEN its behavior and evidence boundaries are reviewed
- THEN existing commands, env/CLI inputs, compatibility milestones, and visible behavior remain comparable
- AND no dynamic plugin loading, default Valence gameplay, BedWars scope, vanilla parity, production readiness, or public-server safety claim is added.

### Requirement: Gameplay plugin contract validation

r[valence_bevy_ecs.gameplay_plugin_contracts.validation] Shared gameplay plugin contract work MUST record focused gameplay/example checks, shared test-helper checks, Valence schedule hygiene, Cairn gates, Cairn validation, task-evidence validation, and evidence manifests before archive.

#### Scenario: Shared contract closeout is reviewable

r[valence_bevy_ecs.gameplay_plugin_contracts.validation.log]
- GIVEN shared gameplay plugin contract work is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show focused gameplay/example checks, positive and negative contract tests, schedule hygiene, Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, and refreshed evidence manifests.

### Requirement: Gameplay arena scoping inventory

r[valence_bevy_ecs.gameplay_arena_scoping.inventory] Gameplay arena scoping work MUST inventory selected CTF and survival compatibility global resources, event payloads, layer/entity assumptions, cleanup paths, milestone emitters, and cross-mode mutation risks before changing ownership.

#### Scenario: Scope risks are visible

r[valence_bevy_ecs.gameplay_arena_scoping.inventory.visible]
- GIVEN a CTF or survival compatibility system is selected for arena scoping
- WHEN reviewers inspect the inventory
- THEN each global resource, event payload, layer or entity query, cleanup path, milestone emitter, and mutation target records whether it is global, arena-owned, layer-owned, client-owned, or fixture-only
- AND risks for same-app CTF plus survival, multiple CTF arenas, multiple survival fixtures, stale arenas, and wrong-layer entities are identified.

### Requirement: Gameplay arena ownership model

r[valence_bevy_ecs.gameplay_arena_scoping.model] Runtime gameplay mode state SHOULD be represented by explicit arena or layer ownership rather than single global mode resources when multiple instances or modes can coexist.

#### Scenario: Arena owns gameplay instance state

r[valence_bevy_ecs.gameplay_arena_scoping.model.owned]
- GIVEN a gameplay mode has score, flag, fixture, container, rule, or presentation state that can vary by arena
- WHEN the state ownership is reviewed
- THEN it is attached to an arena entity, layer-owned component, or explicitly scoped state handle
- AND any remaining global resource records why it is a default, registry, policy, or compatibility shim rather than per-arena state.

### Requirement: Scoped gameplay wiring

r[valence_bevy_ecs.gameplay_arena_scoping.wiring] Gameplay plugin systems MUST filter by explicit scope and mutate only the arenas, layers, clients, entities, resources, and milestones owned by that scope.

#### Scenario: Systems ignore unrelated scope

r[valence_bevy_ecs.gameplay_arena_scoping.wiring.filtered]
- GIVEN CTF and survival compatibility plugins are installed in the same app with distinct arenas or layers
- WHEN scoped systems process input, rules, world mutation, presentation, or cleanup
- THEN each system only reads and mutates data belonging to its matching gameplay scope
- AND wrong-scope, missing-scope, or stale-scope inputs are ignored, cleaned up, or diagnosed deterministically.

### Requirement: Scoped gameplay events and milestones

r[valence_bevy_ecs.gameplay_arena_scoping.events] Gameplay events, diagnostics, and compatibility milestones SHOULD include arena or scope identity when the same semantic can occur in multiple arenas or modes in one app.

#### Scenario: Receipts distinguish arenas

r[valence_bevy_ecs.gameplay_arena_scoping.events.disambiguated]
- GIVEN multiple gameplay arenas can emit the same event or milestone text
- WHEN downstream systems or compatibility receipts observe those events
- THEN the payload or receipt context identifies the owning arena, layer, or gameplay scope
- AND legacy receipt comparability is preserved through documented adapters or explicit evidence non-claims.

### Requirement: Gameplay arena scoping tests

r[valence_bevy_ecs.gameplay_arena_scoping.tests] Gameplay arena scoping work MUST include positive multi-mode or multi-arena tests and negative wrong-scope, stale-scope, missing-scope, disabled-plugin, and cross-layer mutation tests.

#### Scenario: Multiple scoped arenas coexist

r[valence_bevy_ecs.gameplay_arena_scoping.tests.positive]
- GIVEN CTF and survival compatibility plugins or multiple instances of one mode are installed in one app with distinct arenas
- WHEN valid scoped events and systems run
- THEN each arena updates only its own state, emits scope-identifiable observations, and preserves selected fixture behavior.

#### Scenario: Invalid scope fails closed

r[valence_bevy_ecs.gameplay_arena_scoping.tests.negative]
- GIVEN an event, client, entity, container, flag, score, or cleanup target has a missing, stale, disabled-plugin, or wrong-mode scope
- WHEN gameplay systems process it
- THEN no unrelated arena mutates, no false milestone is emitted, stale ownership is cleaned or ignored deterministically, and no panic occurs.

### Requirement: Gameplay arena scoping validation

r[valence_bevy_ecs.gameplay_arena_scoping.validation] Gameplay arena scoping work MUST record focused CTF/survival checks, selected compatibility rails when touched, Valence schedule hygiene, Cairn gates, Cairn validation, task-evidence validation, and evidence manifests before archive.

#### Scenario: Arena scoping closeout is reviewable

r[valence_bevy_ecs.gameplay_arena_scoping.validation.log]
- GIVEN gameplay arena scoping work is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show focused CTF and survival checks, positive and negative scoping tests, selected compatibility rails if fixture behavior or receipts changed, schedule hygiene, Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, and refreshed evidence manifests.

### Requirement: Gameplay config source inventory

r[valence_bevy_ecs.gameplay_config_sources.inventory] Gameplay config source work MUST inventory selected env, CLI, file, default, runtime refresh, validation, test, and receipt-facing input contracts before moving config ownership.

#### Scenario: Config inputs are visible

r[valence_bevy_ecs.gameplay_config_sources.inventory.visible]
- GIVEN a CTF, survival compatibility, terrain, or selected example config path is selected for source separation
- WHEN reviewers inspect the inventory
- THEN each env variable, CLI flag, default value, runtime refresh system, parser, validation rule, test assumption, and receipt-facing input contract is recorded
- AND process-global state, arena-scoped state, and fixture-only toggles are classified explicitly.

### Requirement: Typed gameplay config cores

r[valence_bevy_ecs.gameplay_config_sources.typed] Gameplay config parsing and validation SHOULD be pure deterministic cores over explicit inputs that return typed config values or typed errors.

#### Scenario: Config validation is testable without process state

r[valence_bevy_ecs.gameplay_config_sources.typed.pure]
- GIVEN explicit config inputs are provided by a test, env adapter, CLI adapter, or default provider
- WHEN the config parser validates them
- THEN it returns a typed config value or typed diagnostic without reading environment variables, files, clocks, ECS state, logging, or global mutable state
- AND malformed, missing, non-finite, out-of-range, or contradictory inputs fail closed.

### Requirement: Gameplay config source boundary

r[valence_bevy_ecs.gameplay_config_sources.source_boundary] Gameplay plugin systems MUST consume typed config resources or arena-owned config components rather than reading process environment, CLI state, or files directly during gameplay phases.

#### Scenario: Source adapters own side effects

r[valence_bevy_ecs.gameplay_config_sources.source_boundary.adapters]
- GIVEN an env, CLI, or file source is used for a gameplay example or compatibility fixture
- WHEN the source is read
- THEN a source adapter or startup shell performs the I/O and writes typed config into the documented resource or arena scope
- AND gameplay systems read only the typed config surface.

### Requirement: Explicit gameplay config reload

r[valence_bevy_ecs.gameplay_config_sources.reload] Runtime gameplay config reloads MUST be explicit and scoped when config can affect multiple arenas, modes, or fixture instances.

#### Scenario: Reload affects intended scope only

r[valence_bevy_ecs.gameplay_config_sources.reload.scoped]
- GIVEN a runtime config reload is requested while multiple gameplay scopes may exist
- WHEN reload systems apply the new typed config
- THEN only the requested arena, mode, default profile, or explicitly global config changes
- AND stale or wrong-scope reload requests are ignored or diagnosed deterministically.

### Requirement: Gameplay config source tests

r[valence_bevy_ecs.gameplay_config_sources.tests] Gameplay config source separation MUST include positive typed-config/default/source-adapter tests and negative malformed, missing, stale, wrong-scope, disabled-source, and process-env-isolation tests.

#### Scenario: Valid config reaches gameplay systems

r[valence_bevy_ecs.gameplay_config_sources.tests.positive]
- GIVEN valid explicit inputs, defaults, or source-adapter values are supplied
- WHEN selected gameplay plugins run
- THEN systems observe the expected typed config in the documented scope and preserve selected fixture or example behavior.

#### Scenario: Invalid config fails closed

r[valence_bevy_ecs.gameplay_config_sources.tests.negative]
- GIVEN malformed, missing, stale, wrong-scope, disabled-source, or process-env-mutated inputs are present
- WHEN parsers, source adapters, reload systems, or gameplay systems run
- THEN invalid config is rejected, scoped defaults are preserved when appropriate, unrelated arenas are unchanged, and no panic or false milestone occurs.

### Requirement: Gameplay config source validation

r[valence_bevy_ecs.gameplay_config_sources.validation] Gameplay config source work MUST record focused config checks, selected CTF/survival/terrain/example checks, schedule hygiene when wiring changes, Cairn gates, Cairn validation, task-evidence validation, and evidence manifests before archive.

#### Scenario: Config source closeout is reviewable

r[valence_bevy_ecs.gameplay_config_sources.validation.log]
- GIVEN gameplay config source separation is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show pure config tests, source-adapter tests, positive and negative reload/scope tests, selected gameplay/example checks, schedule hygiene when applicable, Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, and refreshed evidence manifests.

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

### Requirement: Anticheat state inventory

r[valence_bevy_ecs.anticheat_component_state.inventory] Anticheat component-state work MUST inventory current statistic owners, resource fields, per-player map entries, event readers, initialization, cleanup behavior, and public accessors before changing ownership.

#### Scenario: Anticheat ownership baseline is visible

r[valence_bevy_ecs.anticheat_component_state.inventory.visible]
- GIVEN anticheat statistics state is selected for component migration
- WHEN reviewers inspect the inventory
- THEN each current state field records its owner, key space, lifecycle, cleanup behavior, consumer systems, and public accessor impact
- AND stale-entity risks and disabled-plugin behavior are identified.

### Requirement: Anticheat state classification

r[valence_bevy_ecs.anticheat_component_state.classification] Anticheat statistics state MUST be classified as entity-owned component data, global resource state, pure core state, or intentional index/cache state before migration.

#### Scenario: Component candidate is justified

r[valence_bevy_ecs.anticheat_component_state.classification.component]
- GIVEN a statistic value is keyed by a live client entity
- WHEN its ownership classification is reviewed
- THEN the classification explains whether the value should migrate to a component
- AND any remaining resource-owned collection records the cleanup or indexing reason.

### Requirement: Anticheat component ownership

r[valence_bevy_ecs.anticheat_component_state.components] Per-client anticheat statistics whose lifecycle is owned by a live client entity SHOULD be represented by Bevy components rather than external entity-keyed resource maps.

#### Scenario: Despawn removes per-client statistics

r[valence_bevy_ecs.anticheat_component_state.components.despawn]
- GIVEN anticheat statistics have migrated to a client-owned component
- WHEN the client entity despawns or loses the owning role
- THEN the per-client statistics are removed through normal component/entity lifecycle
- AND no stale entity-keyed map entry is required for correctness.

### Requirement: Anticheat compatibility boundary

r[valence_bevy_ecs.anticheat_component_state.compatibility] Anticheat component-state work MUST preserve advisory-only plugin behavior, emitted event shape, explicit opt-in registration, disabled-plugin behavior, and no-enforcement non-claims.

#### Scenario: Plugin behavior remains advisory

r[valence_bevy_ecs.anticheat_component_state.compatibility.advisory]
- GIVEN anticheat statistics storage migrates to components
- WHEN the plugin samples packet or movement activity
- THEN it emits the same advisory observation semantics for valid clients
- AND it does not add enforcement, public-server safety, production cheat detection, or default plugin membership claims.

### Requirement: Anticheat component-state tests

r[valence_bevy_ecs.anticheat_component_state.tests] Anticheat component-state work MUST include positive lifecycle tests and negative stale-entity, despawn, duplicate ownership, reconnect, and plugin-disabled tests.

#### Scenario: Valid client lifecycle records statistics

r[valence_bevy_ecs.anticheat_component_state.tests.positive]
- GIVEN the anticheat statistics plugin is enabled and a valid client emits packet or movement input
- WHEN sampling systems run
- THEN the client's statistics component updates and the expected advisory event is emitted.

#### Scenario: Stale client input fails closed

r[valence_bevy_ecs.anticheat_component_state.tests.negative]
- GIVEN an event targets a despawned, disconnected, reconnected, missing-component, or duplicate-ownership client state
- WHEN sampling systems process the input
- THEN stale ownership is ignored, cleaned, or diagnosed deterministically
- AND no false observation, stale mutation, or panic occurs.

### Requirement: Anticheat component-state validation

r[valence_bevy_ecs.anticheat_component_state.validation] Anticheat component-state work MUST record focused anticheat/Valence checks, schedule hygiene when plugin wiring changes, Cairn gates, Cairn validation, and task-evidence checks before archive.

#### Scenario: Anticheat component-state closeout is reviewable

r[valence_bevy_ecs.anticheat_component_state.validation.log]
- GIVEN anticheat component-state work is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show positive lifecycle tests, negative stale-state tests, focused anticheat/Valence checks, schedule hygiene when applicable, Cairn proposal/design/tasks gates, Cairn validation, and task-evidence validation.

### Requirement: Lifecycle cleanup inventory

r[valence_bevy_ecs.lifecycle_cleanup.inventory] Lifecycle cleanup work MUST inventory selected cleanup paths, owners, triggers, stale-state risks, schedule phases, Valence despawn timing, and evidence impact before changing cleanup ownership.

#### Scenario: Cleanup baseline is reviewable

r[valence_bevy_ecs.lifecycle_cleanup.inventory.reviewable]
- GIVEN a cleanup path is selected for lifecycle work
- WHEN reviewers inspect the inventory
- THEN owner, trigger, current schedule phase, stale-state risk, Valence despawn timing, mutation target, and evidence impact are recorded
- AND cleanup that must run before final entity removal is identified.

### Requirement: Lifecycle cleanup classification

r[valence_bevy_ecs.lifecycle_cleanup.classification] Each targeted cleanup path MUST be classified as component lifecycle, explicit `Despawned` marker cleanup, removal detection, resource/index cleanup, or external I/O cleanup before migration.

#### Scenario: Cleanup classification is justified

r[valence_bevy_ecs.lifecycle_cleanup.classification.justified]
- GIVEN a cleanup path is reviewed
- WHEN its cleanup ownership is classified
- THEN the classification explains why the cleanup belongs to component lifecycle, explicit marker handling, removal detection, resource/index cleanup, or external I/O cleanup
- AND unsuitable lifecycle patterns are rejected with rationale.

### Requirement: Lifecycle cleanup wiring

r[valence_bevy_ecs.lifecycle_cleanup.wiring] Selected cleanup SHOULD use Bevy component lifecycle, removal/change detection, or named cleanup sets where this preserves Valence despawn semantics.

#### Scenario: Cleanup runs in documented phase

r[valence_bevy_ecs.lifecycle_cleanup.wiring.phase]
- GIVEN selected cleanup migrates to a lifecycle pattern or named cleanup set
- WHEN the owning entity or component enters the documented cleanup condition
- THEN cleanup runs in the documented schedule phase
- AND required Valence deinitialization windows before final despawn are preserved.

### Requirement: Explicit cleanup documentation

r[valence_bevy_ecs.lifecycle_cleanup.resources] Cleanup that remains explicit in resources or indexes MUST document owner, trigger, stale-entry handling, and the reason it is not component-owned.

#### Scenario: Resource cleanup remains intentional

r[valence_bevy_ecs.lifecycle_cleanup.resources.intentional]
- GIVEN cleanup remains resource/index-owned after review
- WHEN reviewers inspect the implementation
- THEN key space, lifecycle, cleanup trigger, stale-entry handling, and reason for not using component lifecycle are documented
- AND stale entries are impossible or covered by negative tests.

### Requirement: Lifecycle cleanup tests

r[valence_bevy_ecs.lifecycle_cleanup.tests] Lifecycle cleanup work MUST include positive cleanup tests and negative stale entity, duplicate cleanup, missing owner, reconnect, and plugin-disabled tests for changed paths.

#### Scenario: Valid cleanup removes owned state

r[valence_bevy_ecs.lifecycle_cleanup.tests.positive]
- GIVEN an entity or resource enters the documented cleanup condition
- WHEN cleanup systems run
- THEN owned state is removed or finalized exactly once in the documented phase.

#### Scenario: Invalid cleanup fails closed

r[valence_bevy_ecs.lifecycle_cleanup.tests.negative]
- GIVEN stale entity, duplicate cleanup, missing owner, reconnect, or plugin-disabled conditions
- WHEN cleanup systems run
- THEN cleanup is skipped, diagnosed, or applied deterministically according to the contract
- AND no false milestone, stale mutation, double removal, or panic occurs.

### Requirement: Lifecycle cleanup validation

r[valence_bevy_ecs.lifecycle_cleanup.validation] Lifecycle cleanup work MUST record focused lifecycle checks, selected examples/compatibility rails when behavior changes, schedule hygiene when cleanup sets change, Cairn gates, Cairn validation, and task-evidence checks before archive.

#### Scenario: Lifecycle cleanup closeout is reviewable

r[valence_bevy_ecs.lifecycle_cleanup.validation.log]
- GIVEN lifecycle cleanup work is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show positive cleanup tests, negative stale-cleanup tests, focused lifecycle checks, selected examples or mc-compat rails when applicable, schedule hygiene when applicable, Cairn proposal/design/tasks gates, Cairn validation, and task-evidence validation.

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

### Requirement: Runtime config inventory

r[valence_bevy_ecs.runtime_config_resources.inventory] Runtime config resource work MUST inventory selected example env/CLI/config reads, filesystem path inputs, default values, reload triggers, runtime-toggle expectations, and milestone effects before changing ownership.

#### Scenario: Config inputs are reviewable

r[valence_bevy_ecs.runtime_config_resources.inventory.reviewable]
- GIVEN an example or fixture runtime input is selected for resource ownership
- WHEN reviewers inspect the inventory
- THEN each env var, CLI input, path input, default, reload trigger, runtime-toggle expectation, and milestone impact is recorded
- AND inputs that must remain dynamically polled are distinguished from inputs safe to load once.

### Requirement: Runtime config resource contract

r[valence_bevy_ecs.runtime_config_resources.contract] Selected runtime configuration SHOULD be represented by typed Bevy resources backed by pure parser contracts over explicit inputs.

#### Scenario: Parser core is testable

r[valence_bevy_ecs.runtime_config_resources.contract.parser]
- GIVEN selected runtime configuration is parsed
- WHEN pure parser tests invoke the parser with explicit input values
- THEN the parser returns typed configuration or typed errors without reading environment variables, filesystem state, clocks, Bevy resources, or global state.

### Requirement: Runtime config resource wiring

r[valence_bevy_ecs.runtime_config_resources.wiring] Systems selected for runtime config resource work SHOULD consume typed config resources or explicit reload events instead of reading process environment directly.

#### Scenario: Systems consume explicit config

r[valence_bevy_ecs.runtime_config_resources.wiring.resources]
- GIVEN a selected system needs runtime policy
- WHEN the system runs after migration
- THEN it reads the relevant typed resource or reload event
- AND direct environment access remains in startup/reload shell code only.

### Requirement: Runtime config compatibility

r[valence_bevy_ecs.runtime_config_resources.compatibility] Runtime config resource work MUST preserve selected env var names, CLI inputs, default behavior, reload semantics, milestone text, and non-claim boundaries unless another Cairn changes them.

#### Scenario: Config receipts remain comparable

r[valence_bevy_ecs.runtime_config_resources.compatibility.receipts]
- GIVEN a selected compatibility fixture runs after config resource migration
- WHEN its receipts or logs are compared against the pre-migration contract
- THEN required milestones, forbidden milestones, input names, defaults, and reload behavior remain compatible
- AND no production configuration management, broad compatibility, or vanilla parity claim is added.

### Requirement: Runtime config resource tests

r[valence_bevy_ecs.runtime_config_resources.tests] Runtime config resource work MUST include positive config parser/resource tests and negative missing, malformed, conflicting, reload-stale, and disabled-plugin tests for changed inputs.

#### Scenario: Valid config installs resources

r[valence_bevy_ecs.runtime_config_resources.tests.positive]
- GIVEN valid selected runtime inputs
- WHEN parser and plugin/resource installation tests run
- THEN typed resources contain expected values and selected systems observe those values.

#### Scenario: Invalid config fails clearly

r[valence_bevy_ecs.runtime_config_resources.tests.negative]
- GIVEN missing required input, malformed values, conflicting options, stale reload requests, or disabled plugin configuration
- WHEN parser and app tests run
- THEN typed errors, disabled behavior, or diagnostics match the config contract
- AND no false milestone, panic, or stale resource mutation occurs.

### Requirement: Runtime config resource validation

r[valence_bevy_ecs.runtime_config_resources.validation] Runtime config resource work MUST record focused example checks, selected compatibility rails when fixture behavior changes, schedule hygiene when plugin/run-condition wiring changes, Cairn gates, Cairn validation, and task-evidence checks before archive.

#### Scenario: Runtime config closeout is reviewable

r[valence_bevy_ecs.runtime_config_resources.validation.log]
- GIVEN runtime config resource work is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show pure parser tests, positive and negative resource tests, focused example checks, selected mc-compat rails when applicable, schedule hygiene when applicable, Cairn proposal/design/tasks gates, Cairn validation, and task-evidence validation.

### Requirement: Entity hierarchy inventory

r[valence_bevy_ecs.entity_hierarchy.inventory] Entity hierarchy work MUST inventory selected entity relationships, owners, child lifecycles, traversal needs, cleanup behavior, schedule impact, and evidence impact before changing relationship ownership.

#### Scenario: Relationship baseline is reviewable

r[valence_bevy_ecs.entity_hierarchy.inventory.reviewable]
- GIVEN an entity relationship is selected for hierarchy or relationship-component review
- WHEN reviewers inspect the inventory
- THEN owner, child lifecycle, traversal need, cleanup behavior, current representation, schedule impact, and evidence impact are recorded
- AND tree-like ownership is distinguished from graph, index, layer, protocol ID, or external identity relationships.

### Requirement: Entity hierarchy classification

r[valence_bevy_ecs.entity_hierarchy.classification] Each targeted entity relationship MUST be classified as hierarchy-suitable, explicit relationship component, resource/index, external identity, or intentionally independent entities before migration.

#### Scenario: Hierarchy candidate is justified

r[valence_bevy_ecs.entity_hierarchy.classification.hierarchy]
- GIVEN a relationship is considered for Bevy hierarchy
- WHEN its classification is reviewed
- THEN the classification explains the owner, child lifecycle, traversal need, and cleanup behavior that make hierarchy suitable
- AND unsuitable relationships are left as components, resources, indexes, or independent entities with rationale.

### Requirement: Entity hierarchy wiring

r[valence_bevy_ecs.entity_hierarchy.wiring] Bevy hierarchy or explicit relationship components SHOULD be adopted only where ownership/traversal semantics are real and documented.

#### Scenario: Relationship wiring matches classification

r[valence_bevy_ecs.entity_hierarchy.wiring.classified]
- GIVEN a relationship migrates to hierarchy or an explicit relationship component
- WHEN systems query or clean up the relationship
- THEN the implementation follows the documented owner, child lifecycle, traversal, and cleanup contract
- AND arbitrary indexes or protocol IDs are not hidden behind hierarchy.

### Requirement: Entity hierarchy compatibility

r[valence_bevy_ecs.entity_hierarchy.compatibility] Entity hierarchy work MUST preserve cleanup behavior, fixture/example milestones, layer/entity ID semantics, and non-claim boundaries unless another Cairn changes them.

#### Scenario: Relationship behavior remains comparable

r[valence_bevy_ecs.entity_hierarchy.compatibility.behavior]
- GIVEN a selected relationship is migrated
- WHEN focused tests or rails compare behavior against the baseline
- THEN cleanup behavior, fixture/example milestones, layer membership, entity ID semantics, and non-claim fields remain compatible
- AND no automatic recursive cleanup, broad compatibility, vanilla parity, or production-readiness claim is added.

### Requirement: Entity hierarchy tests

r[valence_bevy_ecs.entity_hierarchy.tests] Entity hierarchy work MUST include positive relationship lifecycle/traversal tests and negative stale parent, orphan child, duplicate parent, cycle/invalid relationship, and plugin-disabled tests for changed relationships.

#### Scenario: Valid relationship lifecycle works

r[valence_bevy_ecs.entity_hierarchy.tests.positive]
- GIVEN a valid parent/child or explicit relationship is created
- WHEN traversal and cleanup systems run
- THEN expected owners, children, and cleanup effects are observed exactly once.

#### Scenario: Invalid relationship fails closed

r[valence_bevy_ecs.entity_hierarchy.tests.negative]
- GIVEN a stale parent, orphan child, duplicate parent, cycle/invalid relationship, or disabled plugin condition
- WHEN relationship systems run
- THEN invalid ownership is rejected, cleaned, or diagnosed deterministically
- AND no stale mutation, unintended recursive despawn, false milestone, or panic occurs.

### Requirement: Entity hierarchy validation

r[valence_bevy_ecs.entity_hierarchy.validation] Entity hierarchy work MUST record focused relationship checks, selected examples/compatibility rails when behavior changes, schedule hygiene when plugin/schedule wiring changes, Cairn gates, Cairn validation, and task-evidence checks before archive.

#### Scenario: Entity hierarchy closeout is reviewable

r[valence_bevy_ecs.entity_hierarchy.validation.log]
- GIVEN entity hierarchy work is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show positive relationship tests, negative invalid-relationship tests, focused relationship checks, selected examples or mc-compat rails when applicable, schedule hygiene when applicable, Cairn proposal/design/tasks gates, Cairn validation, and task-evidence validation.

### Requirement: Remaining example inventory

r[valence_bevy_ecs.remaining_example_plugins.inventory] Remaining example plugin work MUST inventory selected example systems, schedules, resources, events, env/CLI inputs, pure helpers, visible behavior, and non-goals before refactoring wiring.

#### Scenario: Example baseline is reviewable

r[valence_bevy_ecs.remaining_example_plugins.inventory.reviewable]
- GIVEN a remaining Valence example is selected for plugin organization
- WHEN reviewers inspect the inventory
- THEN each current system, schedule label, resource, event, input contract, pure helper, visible behavior, and evidence boundary is recorded
- AND default Valence behavior, vanilla parity, and production readiness remain explicit non-claims.

### Requirement: Remaining example plugin contract

r[valence_bevy_ecs.remaining_example_plugins.contract] Extracted remaining example plugins SHOULD expose named Bevy `SystemSet`s for stable input, rule evaluation, world mutation, presentation, and cleanup ordering where those phases exist.

#### Scenario: Example phases are orderable

r[valence_bevy_ecs.remaining_example_plugins.contract.phases]
- GIVEN an extracted remaining example plugin registers systems
- WHEN reviewers inspect its schedule contract
- THEN systems are grouped into documented phase sets
- AND user code can order around those sets without depending on anonymous tuple order.

### Requirement: Remaining example plugin wiring

r[valence_bevy_ecs.remaining_example_plugins.wiring] Remaining example plugins MUST keep deterministic gameplay and example decisions outside Bevy ECS access unless the code is only an adapter shell.

#### Scenario: Example plugin remains a shell

r[valence_bevy_ecs.remaining_example_plugins.wiring.shell]
- GIVEN an example decision is migrated during plugin organization
- WHEN the implementation is reviewed
- THEN pure decisions consume explicit inputs and return decisions or mutation requests
- AND Bevy queries, commands, resources, logging, file I/O, and world mutation remain in thin systems.

### Requirement: Remaining example compatibility

r[valence_bevy_ecs.remaining_example_plugins.compatibility] Remaining example plugin organization MUST preserve selected example commands, CLI/env contracts, visible behavior, documentation boundaries, and non-claim scope unless another Cairn changes them.

#### Scenario: Example behavior remains comparable

r[valence_bevy_ecs.remaining_example_plugins.compatibility.behavior]
- GIVEN a selected example runs after plugin organization
- WHEN its focused checks or smoke behavior are compared against the pre-refactor contract
- THEN command names, input contracts, visible behavior, and non-claim fields remain compatible
- AND no default gameplay, vanilla parity, or production-readiness claim is added.

### Requirement: Remaining example plugin tests

r[valence_bevy_ecs.remaining_example_plugins.tests] Remaining example plugin organization MUST include positive plugin/schedule smoke tests and negative disabled-plugin or ordering-regression tests.

#### Scenario: Example plugin smoke passes

r[valence_bevy_ecs.remaining_example_plugins.tests.positive]
- GIVEN an extracted example plugin is added to a minimal test app
- WHEN schedules are initialized
- THEN required resources, events, system sets, and schedule labels are present.

#### Scenario: Disabled example plugin is absent

r[valence_bevy_ecs.remaining_example_plugins.tests.negative]
- GIVEN the extracted example plugin is not added to a minimal test app
- WHEN schedules are inspected or updated
- THEN plugin-owned resources, events, systems, and sets are absent
- AND no example-owned milestone or visible behavior can be emitted by that plugin.

### Requirement: Remaining example plugin validation

r[valence_bevy_ecs.remaining_example_plugins.validation] Remaining example plugin organization MUST record focused example checks, Valence schedule hygiene, selected smoke runs when behavior changes, Cairn gates, Cairn validation, and task-evidence checks before archive.

#### Scenario: Example plugin closeout is reviewable

r[valence_bevy_ecs.remaining_example_plugins.validation.log]
- GIVEN remaining example plugin organization is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show focused example checks, positive and negative plugin tests, schedule hygiene, selected smoke runs when applicable, Cairn proposal/design/tasks gates, Cairn validation, and task-evidence validation.

### Requirement: Chunk layer boundaries

r[valence_bevy_ecs.chunk_layer.boundaries] Valence chunk layer code SHOULD expose cohesive boundaries for storage, entry APIs, view and radius targeting, packet writer adapters, local messages, layer trait integration, and update systems.

#### Scenario: Chunk responsibility has one owner

r[valence_bevy_ecs.chunk_layer.boundaries.ownership]
- GIVEN a chunk layer responsibility is reviewed
- WHEN maintainers inspect chunk layer modules
- THEN the responsibility is owned by a focused module
- AND unrelated storage, targeting, writer, message, layer, and system concerns are not reintroduced into one module.

### Requirement: Chunk layer core

r[valence_bevy_ecs.chunk_layer.core] Chunk layer view membership, radius targeting, exception filtering, entry state transitions, and update-plan selection SHOULD be pure over explicit inputs.

#### Scenario: Chunk targeting is testable without clients

r[valence_bevy_ecs.chunk_layer.core.testable]
- GIVEN chunk, view, radius, client, or entry summaries
- WHEN the chunk layer core processes them
- THEN the result can be tested without packet writes, Bevy queries, layer mutation, or schedule systems.

### Requirement: Chunk layer parity

r[valence_bevy_ecs.chunk_layer.parity] Chunk layer modularization MUST preserve public chunk APIs, packet targeting behavior, update ordering, layer semantics, and evidence non-claims.

#### Scenario: Chunk behavior remains stable

r[valence_bevy_ecs.chunk_layer.parity.stable]
- GIVEN a supported pre-refactor chunk layer input
- WHEN the modularized chunk layer processes the same input
- THEN storage, targeting, entry, update, and non-claim behavior remain equivalent.

### Requirement: Chunk layer positive tests

r[valence_bevy_ecs.chunk_layer.positive_tests] The change MUST include positive tests for view targeting, radius targeting, exception filtering, occupied and vacant entries, local messages, and update plans.

#### Scenario: Supported chunk paths pass

r[valence_bevy_ecs.chunk_layer.positive_tests.coverage]
- GIVEN representative supported chunk layer inputs
- WHEN extracted chunk layer cores process them
- THEN tests prove the expected targeting, entry, message, or update decisions are produced.

### Requirement: Chunk layer negative tests

r[valence_bevy_ecs.chunk_layer.negative_tests] The change MUST include negative tests for invalid radii, missing chunks, stale entries, excluded clients, empty views, and invalid update order assumptions.

#### Scenario: Invalid chunk paths fail closed

r[valence_bevy_ecs.chunk_layer.negative_tests.fail_closed]
- GIVEN invalid chunk layer inputs
- WHEN extracted chunk layer cores process them
- THEN tests prove the inputs are rejected, ignored, or contained according to current behavior.

### Requirement: Chunk layer validation

r[valence_bevy_ecs.chunk_layer.validation] The change MUST record focused chunk/layer tests, affected schedule checks, Cairn proposal/design/tasks gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[valence_bevy_ecs.chunk_layer.validation.logs]
- GIVEN chunk layer modularization is complete
- WHEN the change is closed
- THEN reviewable logs show positive and negative chunk tests plus affected schedule checks and Cairn gates passing.

### Requirement: Command manager boundaries

r[valence_bevy_ecs.command_manager.boundaries] Valence command manager code SHOULD expose cohesive boundaries for plugin wiring, packet adapters, command tree synchronization, parse core, execution event planning, and Bevy systems.

#### Scenario: Command responsibility has one owner

r[valence_bevy_ecs.command_manager.boundaries.ownership]
- GIVEN a command manager responsibility is reviewed
- WHEN maintainers inspect command manager modules
- THEN the responsibility is owned by a focused module
- AND unrelated packet, tree, parse, event, plugin, and system concerns are not reintroduced into one module.

### Requirement: Command manager core

r[valence_bevy_ecs.command_manager.core] Packet-to-command event conversion, command tree update requirements, command parse outcomes, argument parse plans, and processed-event plans SHOULD be pure over explicit inputs.

#### Scenario: Command decision is testable without Bevy

r[valence_bevy_ecs.command_manager.core.testable]
- GIVEN packet, command graph, client scope, or command text summaries
- WHEN the command core processes them
- THEN the decision can be tested without Bevy queries, resources, events, packet sends, or schedule wiring.

### Requirement: Command manager parity

r[valence_bevy_ecs.command_manager.parity] Command manager modularization MUST preserve public command APIs, event shapes, command tree behavior, parsing behavior, schedule behavior, and evidence non-claims.

#### Scenario: Command behavior remains stable

r[valence_bevy_ecs.command_manager.parity.stable]
- GIVEN a supported pre-refactor command manager input
- WHEN the modularized command manager processes the same input
- THEN packet adapter, tree sync, parse, event, schedule, and non-claim behavior remain equivalent.

### Requirement: Command manager positive tests

r[valence_bevy_ecs.command_manager.positive_tests] The change MUST include positive tests for packet adapter events, command tree update decisions, valid command parse, argument parse plans, processed events, and plugin wiring facts.

#### Scenario: Supported command paths pass

r[valence_bevy_ecs.command_manager.positive_tests.coverage]
- GIVEN representative supported command manager inputs
- WHEN extracted command cores process them
- THEN tests prove the expected events, parse results, tree updates, or plans are produced.

### Requirement: Command manager negative tests

r[valence_bevy_ecs.command_manager.negative_tests] The change MUST include negative tests for malformed command packets, unknown commands, invalid arguments, stale command trees, missing scopes, and disabled clients.

#### Scenario: Invalid command paths fail closed

r[valence_bevy_ecs.command_manager.negative_tests.fail_closed]
- GIVEN invalid command manager inputs
- WHEN extracted command cores or shells process them
- THEN tests prove the inputs are rejected, ignored, or diagnosed according to current behavior.

### Requirement: Command manager validation

r[valence_bevy_ecs.command_manager.validation] The change MUST record focused command tests, affected examples/checks, Cairn proposal/design/tasks gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[valence_bevy_ecs.command_manager.validation.logs]
- GIVEN command manager modularization is complete
- WHEN the change is closed
- THEN reviewable logs show positive and negative command tests plus affected checks and Cairn gates passing.
