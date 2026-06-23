# Design: Upstream a byte-backed Valence protocol path

## Context

Hyperion's ingress docs describe copying client packets into shared byte storage once, then decoding packet events that can live long enough for ECS processing. The local Hyperion checkout currently depends on forked Valence crates with byte-oriented protocol changes. Valence should make any adopted subset explicit and stable.

## Decisions

### 1. Audit before API commitment

**Choice:** First document which forked byte-backed behavior Hyperion actually needs and which Valence packet/event APIs would change.

**Rationale:** The fork contains more than one concern. Upstreaming should avoid accidental public API churn.

### 2. Separate framing core from I/O shell

**Choice:** Keep VarInt length framing, compression decision logic, and packet-body validation as pure deterministic functions over in-memory buffers. Tokio tasks, channels, and backpressure stay in `valence_network` shells.

**Rationale:** Protocol edge cases need fixture tests without sockets or async runtimes.

### 3. Validate byte-backed field invariants

**Choice:** Byte-backed strings and raw payloads must expose constructors that prove UTF validity, bounds, and packet ownership before values enter public events.

**Rationale:** Borrow-like ergonomics must not let invalid client data outlive validation.

### 4. Migrate incrementally

**Choice:** Introduce byte-backed packet/event types behind compatibility shims or features before removing existing owned paths.

**Rationale:** Existing Valence consumers need a clear migration path.

## Risks / Trade-offs

- Public protocol APIs may become more complex; mitigate with narrow type aliases and docs.
- Zero-copy goals can invite unsafe code; mitigate by preferring stable safe abstractions and requiring explicit audits for any unsafe block.
- Compression/framing bugs are security-sensitive; mitigate with malformed-input fixtures and byte limits.
