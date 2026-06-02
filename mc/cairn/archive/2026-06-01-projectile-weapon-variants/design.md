# Design: Projectile weapon variants rail

## Context

`projectile weapon variants` is still not covered by current promoted evidence, or it is covered only by a historical/oracle workaround. This change creates a narrow lifecycle package so future implementation has an evidence contract before code or docs broaden the claim.

## Decisions

### 1. Contract first

**Choice:** Define scope as a bounded matrix of configured projectile weapons with use, spawn, hit/miss, damage or no-damage outcome, and per-weapon non-claims.

**Rationale:** The row must be reviewable before receipts are promoted.

### 2. Normalize metrics

**Choice:** Compare weapon id, ammunition/item state, use action, projectile spawn, target identity, hit/miss outcome, damage delta when applicable, and server correlation.

**Rationale:** Logs and pass/fail alone are too coarse for this claim.

### 3. Fail closed

**Choice:** Reject missing weapon id, missing projectile spawn, wrong target/outcome, missing damage/no-damage metric, all-weapons overclaim, or exact vanilla physics overclaim.

**Rationale:** Missing evidence should preserve non-claims instead of silently becoming coverage.

### 4. Keep broad claims blocked

**Choice:** Preserve these non-claims: all projectile weapons, projectile travel physics, exact vanilla projectile parity, enchantments/status effects, combat balancing, and production readiness.

**Rationale:** This row should reduce one gap only.

## Implementation notes

- Define projectile weapon matrix schema
- Add first weapon-variant fixtures
- Add checker positives/negatives
- Promote only listed weapon rows

## Risks / Trade-offs

- Different projectile weapons have different charge/use semantics; each row must name its preconditions.
- Do not use this row to claim continuous projectile travel.
