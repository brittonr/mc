# Design: Survival coverage matrix parity sync

## Context

The paired survival reference parity proof promoted exact break/place/pickup metrics using local Paper and Valence receipts. The survival coverage matrix remained at the earlier state, where that row was Valence-only and reference evidence was missing.

## Decisions

### 1. Promote only the existing narrow row

**Choice:** Mark only `break/place/pickup` as paired reference parity covered.

**Rationale:** The promoted evidence covers exact join/render, break, pickup/inventory, and placement metrics for one deterministic row. Crafting, chest/furnace persistence, hunger/food, mob drops, redstone, biome/dimension, and world persistence still lack paired evidence.

### 2. Make the checker fail closed on stale row state

**Choice:** Require the break/place/pickup row to cite both `protocol-763-survival-reference-paper-2026-05-28.receipt.json` and `protocol-763-survival-reference-valence-2026-05-28.receipt.json`, plus the parity doc.

**Rationale:** Future docs should not regress to Valence-only or reference-missing wording after parity has been promoted.

## Risks / Trade-offs

- This does not add new survival breadth; it removes stale metadata so future breadth work starts from correct evidence.
- Full survival compatibility remains blocked by the missing rows.
