# Design: Invalid respawn timing rail

## Context

`invalid respawn timing` is still not covered by current promoted evidence, or it is covered only by a historical/oracle workaround. This change creates a narrow lifecycle package so future implementation has an evidence contract before code or docs broaden the claim.

## Decisions

### 1. Contract first

**Choice:** Define scope as one invalid respawn attempt before the fixture allows respawn plus one valid respawn path after the configured state transition.

**Rationale:** The row must be reviewable before receipts are promoted.

### 2. Normalize metrics

**Choice:** Compare pre-death state, invalid respawn attempt timing, containment result, death state retained, valid respawn request, restored health, duplicate-respawn guard, and server correlation.

**Rationale:** Logs and pass/fail alone are too coarse for this claim.

### 3. Fail closed

**Choice:** Reject missing invalid attempt evidence, premature alive state, duplicate entity/session, missing valid respawn final state, or full lifecycle overclaim.

**Rationale:** Missing evidence should preserve non-claims instead of silently becoming coverage.

### 4. Keep broad claims blocked

**Choice:** Preserve these non-claims: all respawn timing races, reconnect-during-death, repeated deaths, crash recovery, production readiness, and unbounded lifecycle correctness.

**Rationale:** This row should reduce one gap only.

## Implementation notes

- Define invalid timing state machine
- Add negative checker fixtures
- Add runner/client invalid respawn attempt
- Promote only configured timing row

## Risks / Trade-offs

- A negative rail must prove the invalid attempt occurred.
- Allowed post-invalid state must be named before implementation.
