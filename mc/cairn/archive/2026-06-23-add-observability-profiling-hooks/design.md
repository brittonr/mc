# Design: Add optional observability and profiling hooks

## Context

Hyperion's large-event goal makes tracing and profiling important. Valence has logging and can expose spans/metrics around framework operations. The integration should create stable hook points without binding users to a specific profiler or exporter.

## Decisions

### 1. Optional and low overhead by default

**Choice:** Observability hooks are feature/plugin controlled and no-op or minimal when disabled.

**Rationale:** Framework users should not pay for unused instrumentation.

### 2. Stable metric names and labels

**Choice:** Names and labels are documented, bounded, and validated to avoid high-cardinality surprises.

**Rationale:** Metrics become operational contracts once dashboards depend on them.

### 3. Pure classification core

**Choice:** Mapping events or subsystem states to metric/span records is pure. Exporters, profilers, clocks, and sinks are shell adapters.

**Rationale:** Label and redaction behavior can be tested without runtime exporters.

### 4. Redaction is part of the contract

**Choice:** Packet payloads, player identifiers, addresses, and user text are excluded, hashed, or redacted according to documented policy.

**Rationale:** Observability should not leak sensitive data.

## Risks / Trade-offs

- Too many hooks can clutter code; start with high-value subsystem boundaries.
- Metrics can have high cardinality; enforce bounded labels.
- Profiler-specific APIs can create dependencies; keep adapters optional.
