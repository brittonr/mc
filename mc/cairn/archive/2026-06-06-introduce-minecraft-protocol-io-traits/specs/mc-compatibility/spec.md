# mc-compatibility Change Spec: Minecraft protocol I/O traits

## Requirements

### Requirement: Minecraft protocol I/O trait contract

r[mc_compatibility.minecraft_protocol_io_traits.contract] The runner MUST define a narrow Minecraft protocol I/O trait contract before replacing direct packet, string, or VarInt helpers.

#### Scenario: Contract is runner-local

r[mc_compatibility.minecraft_protocol_io_traits.contract.scope]
- GIVEN protocol helper traits are introduced
- WHEN reviewers inspect the contract
- THEN the scope is limited to runner-local status/query and packet-framing helpers
- AND it does not claim a complete Minecraft protocol implementation, Valence protocol parity, public-server compatibility, or new packet coverage.

### Requirement: Pure protocol helper core

r[mc_compatibility.minecraft_protocol_io_traits.core] VarInt, string, and packet framing helpers MUST separate deterministic wire-format logic from network I/O.

#### Scenario: In-memory protocol helpers are deterministic

r[mc_compatibility.minecraft_protocol_io_traits.core.pure]
- GIVEN in-memory byte buffers and explicit string or packet inputs
- WHEN VarInt, string, and packet helper functions run
- THEN they produce or parse the documented bytes deterministically
- AND they do not open sockets, read files, spawn processes, inspect environment, use clocks, or mutate external state.

#### Scenario: Wire-format constants are named

r[mc_compatibility.minecraft_protocol_io_traits.core.constants]
- GIVEN VarInt encoding or decoding uses masks, continuation bits, shift widths, or maximum byte counts
- WHEN reviewers inspect the helper implementation
- THEN those numeric values are named constants
- AND tests cover boundary values that depend on them.

### Requirement: Protocol helper migration

r[mc_compatibility.minecraft_protocol_io_traits.migration] Existing runner status/query and packet-write call sites MUST migrate to protocol I/O traits without changing wire bytes or readiness behavior.

#### Scenario: Status behavior remains stable

r[mc_compatibility.minecraft_protocol_io_traits.migration.parity]
- GIVEN the runner waits for a server status response or writes a protocol packet
- WHEN the migrated helper path executes
- THEN packet framing, VarInt encoding, string encoding, timeout/error mapping, and success diagnostics match the pre-refactor contract.

### Requirement: Protocol helper tests

r[mc_compatibility.minecraft_protocol_io_traits.tests] The protocol helper refactor MUST include positive and negative tests that run against in-memory readers and writers.

#### Scenario: Valid wire-format fixtures pass

r[mc_compatibility.minecraft_protocol_io_traits.tests.positive]
- GIVEN valid VarInt values, strings, packet IDs, and payloads
- WHEN protocol helper tests run through in-memory cursors
- THEN values round-trip and packet bytes match expected status/query fixture bytes.

#### Scenario: Invalid wire-format fixtures fail closed

r[mc_compatibility.minecraft_protocol_io_traits.tests.negative]
- GIVEN input ends early, a packet is truncated, a VarInt exceeds the supported maximum byte count, or a string length is invalid for the fixture
- WHEN protocol helper tests run
- THEN the helper returns an explicit error
- AND no caller treats malformed input as a successful status or packet decode.

### Requirement: Protocol I/O evidence

r[mc_compatibility.minecraft_protocol_io_traits.evidence] Review-critical protocol I/O logs MUST be copied under `docs/evidence/` when task closeout cites status/query behavior beyond unit tests.

#### Scenario: Evidence scope is narrow

r[mc_compatibility.minecraft_protocol_io_traits.evidence.reviewable]
- GIVEN closeout cites protocol I/O behavior
- WHEN reviewers inspect evidence
- THEN logs identify the checked status/query or dry-run path and state that no complete protocol, Valence parity, or public-server compatibility claim is made.

### Requirement: Protocol I/O validation

r[mc_compatibility.minecraft_protocol_io_traits.validation] The change MUST record protocol helper tests, relevant runner/status regression checks, and Cairn gates before archive.

#### Scenario: Protocol I/O closeout is reviewable

r[mc_compatibility.minecraft_protocol_io_traits.validation.log]
- GIVEN Minecraft protocol I/O traits are implemented
- WHEN the change is archived
- THEN successful logs show positive protocol helper tests, negative malformed-input tests, relevant runner/status regression checks, Cairn proposal/design/tasks gates, and Cairn validation.