# Design: Survival mob drop parity rail

## Context

Existing survival reference parity covers break/place/pickup, crafting table, and chest persistence. The `mob drops` row remains missing in the survival coverage matrix, so this change creates a row-scoped paired Paper/Valence rail without strengthening full-survival claims.

## Decisions

### 1. Keep the claim row-scoped

**Choice:** Claim only one configured mob, one bounded kill interaction, one configured drop stack, one pickup, and exact entity/drop metrics.

**Rationale:** Narrow rows are reviewable and keep full survival compatibility false until every required row has paired evidence.

### 2. Require paired Paper and Valence receipts

**Choice:** Promote the row only when both backends produce reviewable receipts/logs copied under `docs/evidence/`.

**Rationale:** The reference-parity policy rejects Valence-only evidence for survival parity claims.

### 3. Normalize metrics before comparison

**Choice:** The checker compares normalized metrics for mob spawn, target acquisition or fixed placement, client attack, server damage/death, drop spawn item/count, client collect/pickup observation, inventory increment, and matching server-side drop milestones.

**Rationale:** Raw log similarity and pass/fail alone are not enough to prove semantic agreement between backends.

### 4. Preserve adjacent non-claims

**Choice:** Matrix and current-bundle updates must keep these non-claims explicit: mob AI parity, pathfinding, all entities, all loot tables, combat balancing, experience drops, farms/spawners, full survival compatibility, broad vanilla parity, and production readiness.

**Rationale:** Each row should reduce one named gap without smuggling in broad vanilla or production claims.

## Risks / Trade-offs

- Uncontrolled AI movement can make the rail flaky; the first row should constrain position and health.
- Loot randomness must be replaced by a configured deterministic drop contract.
