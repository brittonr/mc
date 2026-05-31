# Design: Refresh RED BLUE scoring soak live evidence

## Context

`RED/BLUE scoring soak freshness` is still not covered by current promoted evidence, or it is covered only by a historical/oracle workaround. This change creates a narrow lifecycle package so future implementation has an evidence contract before code or docs broaden the claim.

## Decisions

### 1. Contract first

**Choice:** Define scope as one fresh live rerun of the maintained RED and BLUE scoring soak rails with copied receipts, run logs, and BLAKE3 manifests.

**Rationale:** The row must be reviewable before receipts are promoted.

### 2. Normalize metrics

**Choice:** Compare scenario status, RED score milestone, BLUE score milestone, server score path milestones, missing milestone lists, forbidden score/capture patterns, child revisions, receipt digests, and run-log digests.

**Rationale:** Logs and pass/fail alone are too coarse for this claim.

### 3. Fail closed

**Choice:** Reject historical target-only evidence, missing copied receipt, digest mismatch, missing child revisions, missing server correlation, unexpected score/capture, or broad CTF overclaim.

**Rationale:** Missing evidence should preserve non-claims instead of silently becoming coverage.

### 4. Keep broad claims blocked

**Choice:** Preserve these non-claims: full CTF correctness, production load, public-server safety, unbounded soak, broad Minecraft compatibility, and unrelated CTF rule rows.

**Rationale:** This row should reduce one gap only.

## Implementation notes

- Rerun maintained RED and BLUE soak commands
- Copy receipts and run logs under docs/evidence
- Replace historical-oracle exception only after fresh evidence passes
- Run evidence freshness promotion gate

## Risks / Trade-offs

- Live soak runs are slower than dry-run checks; retain the historical oracle until fresh evidence is copied and validated.
- Do not cite mutable target paths as promoted evidence.
