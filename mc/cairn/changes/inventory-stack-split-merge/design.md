# Design: Inventory stack split merge rail

## Context

`inventory stack split/merge` is still not covered by current promoted evidence, or it is covered only by a historical/oracle workaround. This change creates a narrow lifecycle package so future implementation has an evidence contract before code or docs broaden the claim.

## Decisions

### 1. Contract first

**Choice:** Define scope as one configured item stack split into two stacks and merged back under one window/state-id sequence.

**Rationale:** The row must be reviewable before receipts are promoted.

### 2. Normalize metrics

**Choice:** Compare initial slot/item/count, split action, carried stack count, destination slot/count, merge action, final slot counts, state id, and server click-slot correlation.

**Rationale:** Logs and pass/fail alone are too coarse for this claim.

### 3. Fail closed

**Choice:** Reject missing split/merge metrics, wrong carried count, wrong final count, missing state-id correlation, unexpected item id, or full inventory overclaim.

**Rationale:** Missing evidence should preserve non-claims instead of silently becoming coverage.

### 4. Keep broad claims blocked

**Choice:** Preserve these non-claims: all inventory transactions, drag actions, creative mode, all windows, all item lifecycle correctness, broad protocol coverage, and production readiness.

**Rationale:** This row should reduce one gap only.

## Implementation notes

- Define split/merge contract
- Add checker fixtures for count/state-id mismatches
- Add runner/client split/merge probe
- Update inventory matrix row only

## Risks / Trade-offs

- Slot indexes can shift by window type; use semantic slot labels with raw ids.
- State-id freshness must be explicit to avoid accepting stale clicks.
