# Design: Survival redstone toggle parity rail

## Context

Existing survival reference parity covers break/place/pickup, crafting table, and chest persistence. The `redstone` row remains missing in the survival coverage matrix, so this change creates a row-scoped paired Paper/Valence rail without strengthening full-survival claims.

## Decisions

### 1. Keep the claim row-scoped

**Choice:** Claim only one configured input control, one configured powered output block, one on/off toggle sequence, and exact powered-state metrics.

**Rationale:** Narrow rows are reviewable and keep full survival compatibility false until every required row has paired evidence.

### 2. Require paired Paper and Valence receipts

**Choice:** Promote the row only when both backends produce reviewable receipts/logs copied under `docs/evidence/`.

**Rationale:** The reference-parity policy rejects Valence-only evidence for survival parity claims.

### 3. Normalize metrics before comparison

**Choice:** The checker compares normalized metrics for input interaction, server powered-state transition, client block/state update for the output, optional return-to-off transition when configured, and matching Paper/Valence powered-state observations.

**Rationale:** Raw log similarity and pass/fail alone are not enough to prove semantic agreement between backends.

### 4. Preserve adjacent non-claims

**Choice:** Matrix and current-bundle updates must keep these non-claims explicit: redstone circuit parity, tick-order parity, pistons, observers, comparators, clocks, farms, block-update breadth, full survival compatibility, broad vanilla parity, and production readiness.

**Rationale:** Each row should reduce one named gap without smuggling in broad vanilla or production claims.

## Risks / Trade-offs

- Redstone tick timing is easy to overclaim; the first row should assert only final bounded state transitions.
- Fixture construction must avoid adjacent blocks that introduce accidental power paths.
