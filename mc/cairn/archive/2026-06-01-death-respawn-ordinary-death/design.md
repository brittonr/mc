# Design: Ordinary death respawn rail

## Context

`ordinary death/respawn` is still not covered by current promoted evidence, or it is covered only by a historical/oracle workaround. This change creates a narrow lifecycle package so future implementation has an evidence contract before code or docs broaden the claim.

## Decisions

### 1. Contract first

**Choice:** Define scope as one ordinary player death outside flag-carrier state followed by respawn request, restored health, and playable post-respawn state.

**Rationale:** The row must be reviewable before receipts are promoted.

### 2. Normalize metrics

**Choice:** Compare death cause, pre-death health, death milestone, respawn request, post-respawn health, post-respawn position, flag-state absence, inventory policy, and server correlation.

**Rationale:** Logs and pass/fail alone are too coarse for this claim.

### 3. Fail closed

**Choice:** Reject flag-carrier-only evidence, missing death cause, missing respawn request, missing restored health, stale flag state, unexpected score/capture, or full lifecycle overclaim.

**Rationale:** Missing evidence should preserve non-claims instead of silently becoming coverage.

### 4. Keep broad claims blocked

**Choice:** Preserve these non-claims: all death causes, inventory drop/reset semantics, reconnect-during-death, invalid-respawn timing, repeated deaths, full CTF correctness, and production readiness.

**Rationale:** This row should reduce one gap only.

## Implementation notes

- Define ordinary-death lifecycle contract
- Add checker positive/negative fixtures
- Add runner/client ordinary-death rail
- Update death/respawn lifecycle doc only for this row

## Risks / Trade-offs

- Ordinary-death fixture must avoid flag state so it is not confused with existing flag-carrier evidence.
- Inventory policy should be explicit even if not promoted.
