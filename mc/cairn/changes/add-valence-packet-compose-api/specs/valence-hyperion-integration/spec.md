# valence-hyperion-integration Change Spec: Valence packet compose API

## Requirements

### Requirement: Packet compose contract

r[valence_hyperion_integration.packet_compose.contract] Valence SHOULD define a packet compose API contract for bundle construction, route intents, ordering guarantees, errors, and direct-write migration guidance.

#### Scenario: Compose scope is documented

r[valence_hyperion_integration.packet_compose.contract.documented]
- GIVEN the compose API is introduced
- WHEN reviewers inspect the API docs
- THEN docs distinguish packet bundling, route planning, direct flush, future proxy routing, and cases where direct client writes remain appropriate.

### Requirement: Pure packet delivery planner

r[valence_hyperion_integration.packet_compose.planner] Packet delivery planning MUST be a pure deterministic operation over route intents, client visibility inputs, channel/group inputs, exclusions, and bundle metadata.

#### Scenario: Exclusion applies to global route

r[valence_hyperion_integration.packet_compose.planner.global_exclusion]
- GIVEN a global route intent with an excluded client
- WHEN the planner evaluates active clients
- THEN the delivery plan contains every active non-excluded client
- AND the excluded client receives no planned packet.

### Requirement: Direct-mode flush wiring

r[valence_hyperion_integration.packet_compose.direct_flush] The compose API MAY add direct-mode flush wiring, but it MUST NOT change existing direct packet-write behavior for code that does not opt into compose.

#### Scenario: Non-compose packet writes remain stable

r[valence_hyperion_integration.packet_compose.direct_flush.stable]
- GIVEN an existing Valence system writes packets directly to clients
- WHEN compose support is enabled in the workspace
- THEN direct packet writes preserve their previous ordering, encoding, and flush behavior.

### Requirement: Packet compose tests

r[valence_hyperion_integration.packet_compose.tests] Packet compose work MUST include positive and negative tests for ordering, route resolution, exclusions, closed clients, encode failures, invalid route targets, and partial flush errors.

#### Scenario: Closed client reports partial failure

r[valence_hyperion_integration.packet_compose.tests.closed_client]
- GIVEN a delivery plan includes a client that closes before flush
- WHEN the direct flush shell processes the plan
- THEN it reports a structured partial failure for that client
- AND it does not reorder packets for remaining clients.

### Requirement: Packet compose documentation

r[valence_hyperion_integration.packet_compose.docs] Compose API documentation SHOULD include examples for unicast, broadcast, local visibility, grouped delivery, exclusions, and direct-write alternatives.

#### Scenario: Examples avoid proxy overclaiming

r[valence_hyperion_integration.packet_compose.docs.non_overclaiming]
- GIVEN compose examples are published
- WHEN reviewers inspect them
- THEN they do not claim proxy mode or large-scale performance unless those backends have separate evidence.

### Requirement: Packet compose validation

r[valence_hyperion_integration.packet_compose.validation] Packet compose work MUST record planner tests, direct flush regressions, examples or playground smoke tests, selected mc-compat dry runs, and Cairn gates before archive.

#### Scenario: Compose closeout is reviewable

r[valence_hyperion_integration.packet_compose.validation.log]
- GIVEN compose API work is ready to archive
- WHEN reviewers inspect evidence logs
- THEN logs show positive planner tests, negative route/flush tests, direct flush regressions, example smoke output, selected mc-compat dry runs, and Cairn validation.
