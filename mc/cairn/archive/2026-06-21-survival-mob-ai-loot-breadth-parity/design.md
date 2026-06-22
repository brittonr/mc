# Design: Survival mob AI and loot breadth parity

## Context

The promoted mob-drop row covers one Iron Golem fixture. This change defines a separate finite mob matrix for a hostile/passive breadth slice.

## Decisions

### 1. Keep mob selection explicit

**Choice:** The row names exact mob types, spawn positions, expected interaction, and deterministic loot outputs.

**Rationale:** Explicit fixtures avoid broad mob-AI or loot-table claims.

### 2. Normalize behavior milestones

**Choice:** Records compare mob identity, spawn state, bounded movement/targeting or damage milestone, death/drop state, pickup actor, and inventory delta.

**Rationale:** These fields let the checker compare Paper and Valence without relying on free-form logs.

### 3. Avoid random distribution claims

**Choice:** Loot is deterministic fixture evidence only; random distribution and all loot tables remain out of scope.

**Rationale:** Distribution parity needs a separate statistical design.
