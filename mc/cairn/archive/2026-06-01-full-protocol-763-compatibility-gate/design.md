# Design: Full protocol 763 compatibility aggregate gate

## Context

`full protocol-763 compatibility aggregate` is still not covered by current promoted evidence, or it is covered only by a historical/oracle workaround. This change creates a narrow lifecycle package so future implementation has an evidence contract before code or docs broaden the claim.

## Decisions

### 1. Contract first

**Choice:** Define scope as an aggregate checker over protocol-763 packet inventory requiring every required packet-family row to have mapping, parser fixtures, live evidence, owner, and next action before full protocol claim promotion.

**Rationale:** The row must be reviewable before receipts are promoted.

### 2. Normalize metrics

**Choice:** Compare packet row count, family status, mapping status, parser fixture id, malformed fixture status, live receipt path, owner, next action, and digest.

**Rationale:** Logs and pass/fail alone are too coarse for this claim.

### 3. Fail closed

**Choice:** Reject fallback alias, missing parser fixture, missing live receipt, missing owner, missing next action, stale packet inventory, or full protocol overclaim.

**Rationale:** Missing evidence should preserve non-claims instead of silently becoming coverage.

### 4. Keep broad claims blocked

**Choice:** Preserve these non-claims: full protocol-763 compatibility, full Minecraft compatibility, all gameplay semantics, production readiness, and security robustness until all required rows pass.

**Rationale:** This row should reduce one gap only.

## Implementation notes

- Define packet-family completion criteria
- Add aggregate checker fixtures
- Wire broad coverage ledger/current bundle claim blocks
- Promote aggregate only after ledger complete

## Risks / Trade-offs

- Exhaustive packet coverage may be too large; aggregate criteria should separate packet-family coverage from full per-packet coverage if needed.
- Raw byte-preservation rows must not become semantic parser claims.
