# mc-compatibility Change Spec: Valence network sessions

## Requirements

### Requirement: Valence network session boundaries

r[mc_compatibility.valence_network.session_boundaries] Valence network code SHOULD expose cohesive boundaries for connect and listen orchestration, status and legacy ping handling, login/session negotiation, packet IO framing, profile/cache adapters, and pure session decisions.

#### Scenario: Network responsibility has one owner

r[mc_compatibility.valence_network.session_boundaries.ownership]
- GIVEN a network session responsibility is reviewed
- WHEN maintainers inspect Valence network modules
- THEN the responsibility is owned by a focused module
- AND unrelated socket, packet, status, login, and profile concerns are not reintroduced into one module.

### Requirement: Valence network session core

r[mc_compatibility.valence_network.session_core] Network state transitions, validation, compression choices, status response composition, disconnect classification, and legacy ping classification SHOULD be pure over explicit inputs.

#### Scenario: Network decision is testable without sockets

r[mc_compatibility.valence_network.session_core.testable]
- GIVEN session state summaries and packet or status facts
- WHEN the network core processes them
- THEN the decision can be tested without sockets, async tasks, channels, profile cache IO, or clocks.

### Requirement: Valence network parity

r[mc_compatibility.valence_network.parity] Network modularization MUST preserve public APIs, packet/session behavior, status and legacy ping behavior, profile/cache behavior, async side-effect boundaries, and evidence non-claims.

#### Scenario: Network behavior remains stable

r[mc_compatibility.valence_network.parity.stable]
- GIVEN a supported pre-refactor network input
- WHEN the modularized network code processes the same input
- THEN the session state, packets, status output, profile behavior, and non-claim boundary remain equivalent.

### Requirement: Valence network positive tests

r[mc_compatibility.valence_network.positive_tests] The change MUST include positive tests for status responses, legacy ping classification, login/session transitions, compression decisions, packet framing decisions, and profile adapter outcomes.

#### Scenario: Supported network paths pass

r[mc_compatibility.valence_network.positive_tests.coverage]
- GIVEN representative supported network inputs
- WHEN extracted network cores process them
- THEN tests prove the expected session, status, packet, or profile decisions are produced.

### Requirement: Valence network negative tests

r[mc_compatibility.valence_network.negative_tests] The change MUST include negative tests for malformed handshakes, invalid state transitions, unsupported compression, bad packet frames, missing profiles, closed channels, and timeout classifications.

#### Scenario: Invalid network paths fail closed

r[mc_compatibility.valence_network.negative_tests.fail_closed]
- GIVEN invalid network inputs
- WHEN extracted network cores process them
- THEN tests prove the inputs are rejected, disconnected, or contained according to current behavior.

### Requirement: Valence network validation

r[mc_compatibility.valence_network.validation] The change MUST record focused Valence network tests, affected smoke or dry-run checks, Cairn proposal/design/tasks gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[mc_compatibility.valence_network.validation.logs]
- GIVEN network modularization is complete
- WHEN the change is closed
- THEN reviewable logs show positive and negative network tests plus affected checks and Cairn gates passing.
