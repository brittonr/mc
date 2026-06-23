# valence-hyperion-integration Change Spec: Packet encoder buffer reuse

## Requirements

### Requirement: Packet buffer reuse audit

r[valence_hyperion_integration.packet_buffer_reuse.audit] The integration MUST audit Hyperion encoder/buffer reuse patterns and Valence packet encode/flush paths before adding reusable buffers.

#### Scenario: Optimization scope is recorded

r[valence_hyperion_integration.packet_buffer_reuse.audit.scope]
- GIVEN packet buffer reuse work is selected
- WHEN reviewers inspect the audit
- THEN the audit identifies target workloads, affected encode/flush paths, compression boundaries, and protocol semantics that must remain unchanged.

### Requirement: Packet buffer lifecycle contract

r[valence_hyperion_integration.packet_buffer_reuse.contract] Buffer reuse work MUST define buffer lifecycle, compression settings, capacity policy, reset/discard behavior, packet limit behavior, and safety invariants.

#### Scenario: Error resets reusable buffer

r[valence_hyperion_integration.packet_buffer_reuse.contract.error_reset]
- GIVEN packet encoding or compression fails while using a reusable buffer
- WHEN the encoder returns the error
- THEN the buffer is reset or discarded according to the documented policy
- AND stale partial bytes are not reused for later packets.

### Requirement: Packet buffer baseline

r[valence_hyperion_integration.packet_buffer_reuse.baseline] Packet buffer reuse work MUST record baseline allocation or benchmark evidence for selected workloads before implementation.

#### Scenario: Baseline names encode workload

r[valence_hyperion_integration.packet_buffer_reuse.baseline.named]
- GIVEN baseline evidence is recorded
- WHEN reviewers inspect it
- THEN the evidence names packet mix, compression settings, client count or batch size, command, and environment.

### Requirement: Packet buffer reuse implementation

r[valence_hyperion_integration.packet_buffer_reuse.implementation] Valence MAY implement reusable encoder buffers or pools only when invariants and baseline evidence justify the change.

#### Scenario: Default semantics are preserved

r[valence_hyperion_integration.packet_buffer_reuse.implementation.default_semantics]
- GIVEN reusable buffers are enabled internally
- WHEN existing direct-mode packet tests run
- THEN packet bytes, ordering, compression behavior, and error behavior match the previous public semantics.

### Requirement: Packet buffer reuse tests

r[valence_hyperion_integration.packet_buffer_reuse.tests] Buffer reuse work MUST include positive and negative tests for compression, packet limits, error resets, stale bytes, closed clients, and default behavior.

#### Scenario: Oversized packet does not poison pool

r[valence_hyperion_integration.packet_buffer_reuse.tests.oversized]
- GIVEN an oversized packet fails to encode
- WHEN a subsequent valid packet is encoded using the same pool or encoder path
- THEN the valid packet bytes contain no data from the oversized failure
- AND the oversized diagnostic remains deterministic.

### Requirement: Packet buffer reuse validation

r[valence_hyperion_integration.packet_buffer_reuse.validation] Buffer reuse work MUST record encode tests, compression edge tests, direct-mode regressions, benchmark evidence, selected compatibility dry runs, and Cairn gates before archive.

#### Scenario: Buffer reuse closeout is reviewable

r[valence_hyperion_integration.packet_buffer_reuse.validation.log]
- GIVEN packet buffer reuse work is ready to archive
- WHEN reviewers inspect evidence logs
- THEN logs show baseline and final benchmarks, positive encode fixtures, negative error-reset fixtures, compression edge tests, direct-mode regressions, selected dry runs if behavior changed, and Cairn validation.
