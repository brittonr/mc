# Design: Harden mc-compat receipt schema tests

## Context

The runner writes machine-readable receipts, but some tests prove fields by searching for strings. A structured validator can catch schema regressions that string checks miss while retaining lightweight tests.

## Decisions

### 1. Parse receipts into bounded structs

**Choice:** Add small receipt summary structs for fields under test instead of introducing a broad JSON abstraction for every receipt detail at once.

**Rationale:** Focused structs keep the test core reviewable and avoid overfitting to unrelated JSON formatting.

### 2. Preserve exact non-claim checks

**Choice:** Non-claims, child revision metadata, typed-event artifacts, backend identifiers, and artifact paths become typed assertions with negative fixtures.

**Rationale:** These fields define the evidence boundary and must fail closed when missing or malformed.

### 3. Keep JSON rendering stable

**Choice:** Existing receipt JSON shape should remain backward-compatible unless a separate schema change is explicitly accepted.

**Rationale:** Downstream Cairn/evidence tools consume current receipt shapes.

### 4. Reuse pure validation helpers

**Choice:** Receipt validation should operate on in-memory receipt text or parsed summaries. Filesystem reads and command execution stay in test shells or checker CLIs.

**Rationale:** Positive and negative fixtures can run quickly without launching clients or servers.

## Risks / Trade-offs

- Typed summaries can lag behind full schema complexity; mitigate by focusing on evidence-critical fields first.
- Introducing a JSON parser dependency may affect the tiny runner crate; mitigate by using minimal parsing or existing crate dependencies only when justified.
- Overly strict ordering checks can create churn; mitigate by validating structure and values, not pretty-print order.
