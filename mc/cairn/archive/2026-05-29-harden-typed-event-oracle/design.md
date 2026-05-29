# Design: Typed event oracle

## Context

Current scenario evaluation mostly scans combined process output. That makes the runner easy to extend, but logs are not a stable evidence API. A typed event stream gives the harness a reviewable contract without requiring packet capture or full semantic decoding.

## Decisions

### 1. Additive event stream first

**Choice:** Emit typed JSONL events next to existing logs, then migrate scenario oracles one rail at a time.

**Rationale:** Existing receipts and evidence stay reviewable while typed events are introduced. Historical evidence does not need to be rewritten.

### 2. Pure event-graph core

**Choice:** Parse event lines into typed structs and evaluate scenarios with pure functions over in-memory events.

**Rationale:** Tests can exercise the oracle without starting Minecraft, Docker, Xvfb, or child processes.

### 3. Correlation before breadth

**Choice:** The initial schema requires scenario name, session id, username, source, monotonic sequence, event kind, and typed fields. Raw packet payloads remain excluded unless a separate redacted capture contract is added.

**Rationale:** The highest-value gap is proving that the right client/server events belong to the same bounded run.

### 4. Timeline hash in receipts

**Choice:** Receipts record the event schema, event-log path, and BLAKE3 hash of the normalized timeline.

**Rationale:** Reviewers can verify that promoted receipts correspond to immutable local artifacts.

## Risks / Trade-offs

- Probe emitters in child repos must be updated carefully so typed events do not diverge from existing text milestones.
- Event schema churn can break old receipts; versioning and migration fallback are required.
- The schema should stay narrow enough to avoid becoming a second packet protocol.
