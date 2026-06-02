# Design: Production readiness envelope gate

## Context

`production readiness aggregate` is still not covered by current promoted evidence, or it is covered only by a historical/oracle workaround. This change creates a narrow lifecycle package so future implementation has an evidence contract before code or docs broaden the claim.

## Decisions

### 1. Contract first

**Choice:** Define scope as an aggregate production-readiness gate requiring owned/public/WAN/adversarial safety rows, telemetry, authorization, redaction, abort criteria, and evidence manifests.

**Rationale:** The row must be reviewable before receipts are promoted.

### 2. Normalize metrics

**Choice:** Compare target scope, authorization, owner, client count, duration, perturbation settings, adversarial model, telemetry, abort criteria, redaction status, and row evidence paths.

**Rationale:** Logs and pass/fail alone are too coarse for this claim.

### 3. Fail closed

**Choice:** Reject missing authorization, missing telemetry, missing oracle checkpoint, missing redaction, unbounded load, public target without approval, or production-readiness overclaim.

**Rationale:** Missing evidence should preserve non-claims instead of silently becoming coverage.

### 4. Keep broad claims blocked

**Choice:** Preserve these non-claims: production readiness until every envelope row passes, public third-party safety without authorization, unbounded load, WAN robustness, adversarial robustness, and security certification.

**Rationale:** This row should reduce one gap only.

## Implementation notes

- Define production readiness row set
- Add aggregate checker fixtures
- Require redacted evidence and human checkpoints
- Block production wording until envelope complete

## Risks / Trade-offs

- This gate must not authorize live public/adversarial tests by itself.
- Production readiness language needs stricter review than local compatibility evidence.
