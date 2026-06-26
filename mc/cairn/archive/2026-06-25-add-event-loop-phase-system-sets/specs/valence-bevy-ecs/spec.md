# valence-bevy-ecs Change Spec: Event-loop phase SystemSets

## Requirements

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
