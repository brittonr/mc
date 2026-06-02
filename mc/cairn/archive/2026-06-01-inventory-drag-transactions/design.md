# Design: Inventory drag transaction rail

## Context

`inventory drag transactions` is still not covered by current promoted evidence, or it is covered only by a historical/oracle workaround. This change creates a narrow lifecycle package so future implementation has an evidence contract before code or docs broaden the claim.

## Decisions

### 1. Contract first

**Choice:** Define scope as one configured drag transaction across a fixed set of slots with exact final item/count distribution.

**Rationale:** The row must be reviewable before receipts are promoted.

### 2. Normalize metrics

**Choice:** Compare window id, state id, drag phase sequence, source stack, target slots, per-slot final counts, carried remainder, and server transaction correlation.

**Rationale:** Logs and pass/fail alone are too coarse for this claim.

### 3. Fail closed

**Choice:** Reject missing phase, out-of-order drag sequence, wrong slot distribution, stale state id, missing server correlation, or all-transaction overclaim.

**Rationale:** Missing evidence should preserve non-claims instead of silently becoming coverage.

### 4. Keep broad claims blocked

**Choice:** Preserve these non-claims: all drag modes, creative inventory, all windows, split/merge outside this row, full inventory semantics, broad protocol coverage, and production readiness.

**Rationale:** This row should reduce one gap only.

## Implementation notes

- Define drag phase contract
- Add phase-order checker fixtures
- Add runner/client drag probe
- Promote only configured drag row

## Risks / Trade-offs

- Drag packets have multi-step state; logs must retain ordered phase evidence.
- Do not conflate drag with ordinary click-slot rows.
