# Design: Survival hunger and food parity rail

## Context

Existing survival reference parity covers break/place/pickup, crafting table, and chest persistence. The `hunger/food` row remains missing in the survival coverage matrix, so this change creates a row-scoped paired Paper/Valence rail without strengthening full-survival claims.

## Decisions

### 1. Keep the claim row-scoped

**Choice:** Claim only one deterministic hunger deficit, one configured food item, one consume action, one hunger/saturation delta, and one inventory decrement.

**Rationale:** Narrow rows are reviewable and keep full survival compatibility false until every required row has paired evidence.

### 2. Require paired Paper and Valence receipts

**Choice:** Promote the row only when both backends produce reviewable receipts/logs copied under `docs/evidence/`.

**Rationale:** The reference-parity policy rejects Valence-only evidence for survival parity claims.

### 3. Normalize metrics before comparison

**Choice:** The checker compares normalized metrics for pre-consume hunger/saturation, consume start/finish, item decrement, post-consume hunger/saturation, optional health/regeneration observation when configured, and matching server-side food milestones.

**Rationale:** Raw log similarity and pass/fail alone are not enough to prove semantic agreement between backends.

### 4. Preserve adjacent non-claims

**Choice:** Matrix and current-bundle updates must keep these non-claims explicit: all foods, starvation loops, regeneration balance, potion/status effects, exhaustion math, sprint/jump hunger drain, full survival compatibility, broad vanilla parity, and production readiness.

**Rationale:** Each row should reduce one named gap without smuggling in broad vanilla or production claims.

## Risks / Trade-offs

- Hunger and saturation are stateful; fixtures must reset preconditions explicitly before each session.
- Health regeneration should stay optional unless the row intentionally owns the timing and tolerance model.
