# Design: Creative mode inventory rail

## Context

`creative-mode inventory` is still not covered by current promoted evidence, or it is covered only by a historical/oracle workaround. This change creates a narrow lifecycle package so future implementation has an evidence contract before code or docs broaden the claim.

## Decisions

### 1. Contract first

**Choice:** Define scope as one configured creative inventory action under an owned local fixture with explicit permission, item id/count, and resulting slot state.

**Rationale:** The row must be reviewable before receipts are promoted.

### 2. Normalize metrics

**Choice:** Compare game mode, permission state, creative action type, item id, item count, target slot, client observation, server inventory state, and forbidden survival-only assumptions.

**Rationale:** Logs and pass/fail alone are too coarse for this claim.

### 3. Fail closed

**Choice:** Reject survival-mode evidence, missing permission metric, wrong item/count, missing server state, unexpected survival semantics, or all-creative-inventory overclaim.

**Rationale:** Missing evidence should preserve non-claims instead of silently becoming coverage.

### 4. Keep broad claims blocked

**Choice:** Preserve these non-claims: all creative actions, operator/admin safety, public-server creative permissions, all inventory transactions, production readiness, and broad protocol coverage.

**Rationale:** This row should reduce one gap only.

## Implementation notes

- Define creative permission contract
- Add checker negative fixtures
- Add creative fixture/probe
- Promote only configured creative action

## Risks / Trade-offs

- Creative mode can imply elevated permission; keep target owned-local and never public by default.
- Do not weaken public-server safety preflights.
