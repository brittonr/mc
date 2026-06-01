# Design: Full survival compatibility aggregate gate

## Context

Current promoted survival reference parity covers break/place/pickup, crafting table, and chest persistence. Six required rows remain open: furnace persistence, hunger/food, mob drops, redstone, biome/dimension, and world persistence. A full-survival claim must stay false until all row-scoped Cairns have paired Paper/Valence evidence and the docs remove only the matching non-claim text.

## Decisions

### 1. Gate the aggregate, not the individual rows

**Choice:** The aggregate gate reads row states from the survival coverage matrix/current bundle and enforces the complete required row set.

**Rationale:** Individual row Cairns should stay narrow. The aggregate gate prevents accidental broad claims while those rows are still open.

### 2. Name the required row set explicitly

**Choice:** Required rows are break/place/pickup, crafting table, chest persistence, furnace persistence, hunger/food, mob drops, redstone, biome/dimension, and world persistence.

**Rationale:** An explicit row set makes additions/removals reviewable and keeps future full-survival wording tied to concrete evidence.

### 3. Require paired reference evidence for every row

**Choice:** Each row must include Valence evidence, Paper/reference evidence, comparator or row-checker output, BLAKE3 manifest coverage, child revision metadata or an oracle checkpoint, and row-specific non-claim text.

**Rationale:** Valence-only or docs-only evidence is not enough for survival reference parity.

### 4. Fail closed on wording drift

**Choice:** The checker rejects premature `full_survival_compatibility` or equivalent prose while any required row remains missing.

**Rationale:** The matrix can grow over time; broad wording must not escape before evidence catches up.

## Implementation notes

- Parse the survival coverage matrix into a deterministic row map.
- Compare row names against the explicit required set.
- Verify each row has reference-parity covered status, Valence evidence, Paper/reference evidence, comparator/checker evidence, and BLAKE3 manifest linkage.
- Check current bundle and acceptance matrix non-claim text before and after promotion.
- Include positive fixture with all rows covered and negative fixtures for missing row, Valence-only row, missing manifest, and premature full-survival claim.

## Risks / Trade-offs

- The gate should not force implementation of missing rows now; it should fail closed until they land.
- If the required row set changes, the checker fixtures and the matrix prose must change in the same reviewable diff.
