# valence-bevy-ecs Change Spec: Packet compose core

## Requirements

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
