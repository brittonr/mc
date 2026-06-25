# valence-bevy-ecs Change Spec: Typed packet-derived gameplay events

## Requirements

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
