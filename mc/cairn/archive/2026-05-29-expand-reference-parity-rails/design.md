# Design: Reference parity expansion

## Context

Paired Paper/Valence survival evidence prevents Valence-only receipts from being promoted as vanilla parity. That pattern should become a reusable harness contract instead of a one-off checker.

## Decisions

### 1. Policy before row work

**Choice:** First write a policy that classifies claims as reference-parity-required, Valence-only containment, or non-claim.

**Rationale:** Future rows should not debate evidence standard every time.

### 2. Coordinate with active chest work

**Choice:** Treat `prove-survival-chest-persistence` as the active chest row. This change owns the general parity expansion and subsequent rows, not duplicate chest implementation.

**Rationale:** Avoid competing Cairns for the same evidence row.

### 3. Normalize metrics before comparing

**Choice:** Each parity row compares normalized metrics extracted from receipt/log artifacts, not raw text equality or pass/fail alone.

**Rationale:** Backends can phrase logs differently while still agreeing on semantics; conversely, both can pass while disagreeing on a metric.

### 4. Row-level promotion only

**Choice:** Promote each survival/combat parity row independently and require current-bundle non-claims to remain until coverage is complete.

**Rationale:** Narrow rows are reviewable. Broad parity claims need many rows.

## Risks / Trade-offs

- Paper fixtures may require additional plugin hooks for server-side metrics.
- Some behaviors may be intentionally implementation-specific and should remain Valence-only containment, not parity.
- More paired rails increase live-run cost; dry-run and checker fixtures must remain cheap.
