# Design: Projectile, armor, and knockback combat semantics compatibility rail

## Context

This package comes from the 2026-05-25 ROI ranking after protocol-763 Valence CTF evidence had landed through scoring, BLUE/RED soaks, inventory/drop/pickup/block-place/click/open-container, and two-client combat/damage. The goal is to make the next compatibility claims receipt-backed without repeating saturated evidence.

## Decisions

### 1. Select one first mechanic

**Choice:** Do not bundle projectile, armor, and knockback all into the first implementation if one smaller seam can produce durable evidence.

**Rationale:** Keep the slice independently drainable, evidence-backed, and scoped to owned local Valence/Stevenarella compatibility testing.

### 2. Keep behavior public-surface aware

**Choice:** Because this can affect combat semantics and packet coverage, the Cairn package owns requirements before implementation.

**Rationale:** Keep the slice independently drainable, evidence-backed, and scoped to owned local Valence/Stevenarella compatibility testing.

### 3. Preserve non-claims

**Choice:** Receipt must not claim full combat correctness, anti-cheat behavior, or production PvP readiness.

**Rationale:** Keep the slice independently drainable, evidence-backed, and scoped to owned local Valence/Stevenarella compatibility testing.

## Risks / Trade-offs

- May require new Valence example mechanics or Stevenarella packet seams beyond current CTF support.
- Knockback and projectile physics may be harder to make deterministic than damage/health updates.
