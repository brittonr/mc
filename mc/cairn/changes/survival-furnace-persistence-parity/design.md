# Design: Survival furnace persistence parity rail

## Context

Existing survival reference parity covers break/place/pickup, crafting table, and chest persistence. The `furnace persistence` row remains missing in the survival coverage matrix, so this change creates a row-scoped paired Paper/Valence rail without strengthening full-survival claims.

## Decisions

### 1. Keep the claim row-scoped

**Choice:** Claim only one deterministic furnace block, one configured input stack, one fuel stack, one smelted output stack, and one reconnect/reopen observation within the same server process.

**Rationale:** Narrow rows are reviewable and keep full survival compatibility false until every required row has paired evidence.

### 2. Require paired Paper and Valence receipts

**Choice:** Promote the row only when both backends produce reviewable receipts/logs copied under `docs/evidence/`.

**Rationale:** The reference-parity policy rejects Valence-only evidence for survival parity claims.

### 3. Normalize metrics before comparison

**Choice:** The checker compares normalized metrics for furnace open, input insert, fuel insert, burn/progress start, output availability, output collection, reconnect/reopen observation, and matching server-side furnace state milestones.

**Rationale:** Raw log similarity and pass/fail alone are not enough to prove semantic agreement between backends.

### 4. Preserve adjacent non-claims

**Choice:** Matrix and current-bundle updates must keep these non-claims explicit: all smelting recipes, long-running furnace timing parity, hopper automation, furnace minecarts, server restart/world persistence, full survival compatibility, broad vanilla parity, and production readiness.

**Rationale:** Each row should reduce one named gap without smuggling in broad vanilla or production claims.

## Risks / Trade-offs

- Wall-clock furnace timing can be flaky; the fixture should expose a bounded deterministic completion condition instead of sleeping blindly.
- Reconnect/session persistence must not be described as server restart/world persistence.
