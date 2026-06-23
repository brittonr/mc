# valence-hyperion-integration Change Spec: Byte-backed Valence protocol path

## Requirements

### Requirement: Byte-backed protocol audit

r[valence_hyperion_integration.byte_protocol.audit] The integration MUST audit Hyperion's byte-backed protocol usage and Valence's current protocol/event surfaces before adding public byte-backed APIs.

#### Scenario: Required byte behavior is identified

r[valence_hyperion_integration.byte_protocol.audit.recorded]
- GIVEN byte-backed protocol work is selected
- WHEN reviewers inspect the audit notes
- THEN the notes identify required Hyperion fork behavior, affected Valence packet/event APIs, migration risks, and out-of-scope packet-channel/runtime behavior.

### Requirement: Stable byte-backed API

r[valence_hyperion_integration.byte_protocol.api] Valence MUST define stable byte-backed raw-payload and validated text/byte field APIs with explicit ownership, bounds, and validation invariants.

#### Scenario: Invalid text is rejected

r[valence_hyperion_integration.byte_protocol.api.invalid_text]
- GIVEN client packet bytes contain invalid text for a validated string field
- WHEN the byte-backed constructor validates the field
- THEN it returns a deterministic error
- AND no public packet event exposes the invalid field.

### Requirement: Pure packet framing core

r[valence_hyperion_integration.byte_protocol.core] Packet framing, compression decisions, and body validation SHOULD be implemented as pure deterministic cores over in-memory byte buffers, with socket I/O and channel orchestration kept in thin shells.

#### Scenario: Split frame is decoded deterministically

r[valence_hyperion_integration.byte_protocol.core.split_frame]
- GIVEN a valid packet frame split across multiple input buffers
- WHEN the framing core receives the buffers in order
- THEN it returns the same completed packet body as a single-buffer decode
- AND it preserves incomplete-frame state without reading from sockets or global state.

### Requirement: Byte protocol fixture coverage

r[valence_hyperion_integration.byte_protocol.fixtures] Byte-backed protocol work MUST include positive and negative fixtures for valid frames and malformed input boundaries.

#### Scenario: Oversized packet fails closed

r[valence_hyperion_integration.byte_protocol.fixtures.oversized]
- GIVEN packet bytes declare a length beyond the configured packet limit
- WHEN the framing fixture runs
- THEN decoding fails with a deterministic oversized-packet diagnostic
- AND no partial public event is emitted.

### Requirement: Incremental byte protocol migration

r[valence_hyperion_integration.byte_protocol.migration] Selected Valence event-loop packet paths SHOULD migrate behind compatibility shims or feature flags before existing owned packet paths are removed.

#### Scenario: Existing direct mode still decodes packets

r[valence_hyperion_integration.byte_protocol.migration.direct_stable]
- GIVEN byte-backed protocol support is present but default direct networking remains configured
- WHEN existing Valence packet/event tests execute
- THEN packet decoding and event delivery remain compatible with the previous owned path.

### Requirement: Byte protocol validation

r[valence_hyperion_integration.byte_protocol.validation] Byte-backed protocol work MUST record protocol tests, negative malformed-input tests, event-loop regressions, selected mc-compat dry runs, and Cairn gates before archive.

#### Scenario: Byte protocol closeout is reviewable

r[valence_hyperion_integration.byte_protocol.validation.log]
- GIVEN byte-backed protocol work is ready to archive
- WHEN reviewers inspect evidence logs
- THEN logs show positive protocol fixtures, negative malformed-input fixtures, event-loop regressions, selected mc-compat dry runs, Cairn proposal/design/tasks gates, and Cairn validation.
