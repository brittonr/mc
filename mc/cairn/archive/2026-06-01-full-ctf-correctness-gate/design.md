# Design: Full CTF correctness aggregate gate

## Context

`full CTF correctness aggregate` is still not covered by current promoted evidence, or it is covered only by a historical/oracle workaround. This change creates a narrow lifecycle package so future implementation has an evidence contract before code or docs broaden the claim.

## Decisions

### 1. Contract first

**Choice:** Define scope as an aggregate checker over CTF rule ledger rows requiring every configured rule family to be covered before full CTF correctness can be claimed.

**Rationale:** The row must be reviewable before receipts are promoted.

### 2. Normalize metrics

**Choice:** Compare rule family, status, receipt path, run log path, BLAKE3 manifest, forbidden-transition checks, negative fixture coverage, and current-bundle label.

**Rationale:** Logs and pass/fail alone are too coarse for this claim.

### 3. Fail closed

**Choice:** Reject missing rule family, missing receipt, missing forbidden scan, missing negative fixture, stale non-claim text, or full CTF overclaim.

**Rationale:** Missing evidence should preserve non-claims instead of silently becoming coverage.

### 4. Keep broad claims blocked

**Choice:** Preserve these non-claims: full CTF correctness until all rule rows pass, production gameplay readiness, public-server safety, and broad Minecraft compatibility.

**Rationale:** This row should reduce one gap only.

## Implementation notes

- Define required CTF rule-family set
- Add aggregate checker negatives
- Wire acceptance/current bundle claim blocks
- Promote aggregate only after all rule rows covered

## Risks / Trade-offs

- CTF rules are fixture-specific; the gate should name the fixture rule set.
- Aggregate docs must not imply production gameplay readiness.
