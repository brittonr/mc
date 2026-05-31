# Design: Survival crafting table parity rail

## Context

Existing survival reference parity covers break/place/pickup and chest persistence only. The `crafting` row remains missing in the survival coverage matrix, so this change creates a row-scoped paired Paper/Valence rail without strengthening full-survival claims.

## Decisions

### 1. Keep the claim row-scoped

**Choice:** Claim only one deterministic crafting table, one configured recipe, one configured input stack set, one result stack, and exact inventory/result metrics.

**Rationale:** Narrow rows are reviewable and keep full survival compatibility false until every required row has paired evidence.

### 2. Require paired Paper and Valence receipts

**Choice:** Promote the row only when both backends produce reviewable receipts/logs copied under `docs/evidence/`.

**Rationale:** The reference-parity policy rejects Valence-only evidence for survival parity claims.

### 3. Normalize metrics before comparison

**Choice:** The checker compares normalized metrics for crafting-table open, configured input-slot placement, result-slot availability, result collection, inventory decrement/increment, and matching server-side recipe/result milestones.

**Rationale:** Raw log similarity and pass/fail alone are not enough to prove semantic agreement between backends.

### 4. Preserve adjacent non-claims

**Choice:** Matrix and current-bundle updates must keep these non-claims explicit: full crafting coverage, all recipes, recipe-book behavior, shift-click matrices, all container transaction modes, full survival compatibility, broad vanilla parity, and production readiness.

**Rationale:** Each row should reduce one named gap without smuggling in broad vanilla or production claims.

## Risks / Trade-offs

- Crafting UI slot numbering can differ from player-inventory slot numbering; the contract should name semantic slots instead of relying on raw indexes.
- Recipe-book and shift-click shortcuts are useful later but should not be bundled into the first row.
