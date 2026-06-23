# Design: Audit entity metadata diff tracking

## Context

Hyperion registers metadata components, tracks previous values, and encodes deltas. Valence owns generated entity metadata and server update systems. The useful integration is an audit-driven improvement to Valence's existing flow, not a replacement with Hyperion macros.

## Decisions

### 1. Audit first

**Choice:** Record current Valence metadata behavior and compare it to Hyperion's component-diff model before changing code.

**Rationale:** Valence may already cover many cases differently.

### 2. Diff core is pure

**Choice:** Metadata diffing should be expressible as a pure comparison between previous and current metadata snapshots returning encoded or typed update intents.

**Rationale:** Same-tick and default-suppression edge cases need deterministic tests.

### 3. Packet ordering is explicit

**Choice:** Spawn metadata, incremental updates, removals, and despawns must have documented ordering relative to entity lifecycle events.

**Rationale:** Clients can desync when metadata arrives at the wrong time.

### 4. Generated metadata remains source of truth

**Choice:** Do not fork generated metadata definitions. Any helper should consume Valence's generated entity metadata model.

**Rationale:** Generated protocol data should remain authoritative.

## Risks / Trade-offs

- Diff tracking can miss intermediate same-tick changes; define whether final-state or all-transition semantics are intended.
- Additional tracking state can cost memory; measure if broad metadata tracking changes are made.
- Entity protocol details vary by version; keep version assumptions explicit.
