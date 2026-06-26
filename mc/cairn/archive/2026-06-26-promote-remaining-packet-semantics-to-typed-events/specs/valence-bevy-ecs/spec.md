# valence-bevy-ecs Change Spec: Remaining packet semantic typed events

## Requirements

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
