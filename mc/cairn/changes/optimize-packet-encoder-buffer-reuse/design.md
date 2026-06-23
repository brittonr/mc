# Design: Optimize packet encoder buffer reuse

## Context

Packet egress can allocate repeatedly when encoding many packets. Hyperion keeps reusable buffers and compression-aware encoder paths. Valence should evaluate similar reuse while preserving correctness for compression boundaries, packet limits, and client flush ordering.

## Decisions

### 1. Correctness before pooling

**Choice:** Define packet encode and compression invariants before adding reuse or pooling.

**Rationale:** Buffer reuse bugs can leak stale bytes or corrupt packets.

### 2. Pure encode planning where possible

**Choice:** Decisions about compression threshold, packet length prefixing, capacity reservation, and flush grouping are expressed as deterministic helpers. Actual buffer mutation remains carefully isolated.

**Rationale:** Edge cases can be tested without sockets.

### 3. Reuse survives errors safely

**Choice:** Any encoder or pool that observes an encode/compression error must reset or discard affected buffers before reuse.

**Rationale:** Error paths are where stale bytes commonly leak.

### 4. Performance claims require measurement

**Choice:** Allocation or latency improvements are claimed only with before/after evidence for named workloads.

**Rationale:** Buffer pools can improve one workload while hurting another.

## Risks / Trade-offs

- Pooling increases statefulness; keep lifecycle small and test resets.
- Compression settings can differ per connection; include settings in buffer state validation.
- Over-retained capacity can waste memory; document shrink policy.
