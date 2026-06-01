# Design: Survival biome and dimension state parity rail

## Context

Existing survival reference parity covers break/place/pickup, crafting table, and chest persistence. The `biome/dimension` row remains missing in the survival coverage matrix, so this change creates a row-scoped paired Paper/Valence rail without strengthening full-survival claims.

## Decisions

### 1. Keep the claim row-scoped

**Choice:** Claim only one configured environment-state observation, either a fixed biome sample or a bounded fixture-driven dimension transition, with explicit normalized state fields.

**Rationale:** Narrow rows are reviewable and keep full survival compatibility false until every required row has paired evidence.

### 2. Require paired Paper and Valence receipts

**Choice:** Promote the row only when both backends produce reviewable receipts/logs copied under `docs/evidence/`.

**Rationale:** The reference-parity policy rejects Valence-only evidence for survival parity claims.

### 3. Normalize metrics before comparison

**Choice:** The checker compares normalized metrics for spawn environment, biome or dimension identifier, client-observed environment update, server authoritative environment state, and matching Paper/Valence normalized identifiers.

**Rationale:** Raw log similarity and pass/fail alone are not enough to prove semantic agreement between backends.

### 4. Preserve adjacent non-claims

**Choice:** Matrix and current-bundle updates must keep these non-claims explicit: world-generation parity, all biomes, all dimensions, portal mechanics breadth, lighting/weather parity, structure generation, full survival compatibility, broad vanilla parity, and production readiness.

**Rationale:** Each row should reduce one named gap without smuggling in broad vanilla or production claims.

## Risks / Trade-offs

- Biome and dimension semantics are broader than one row; the contract must choose and name the exact subrail before implementation.
- World generation should be fixture-controlled so matching identifiers are not mistaken for terrain parity.
