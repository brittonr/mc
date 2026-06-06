# Design: Promote creative inventory action evidence

## Context

Inventory evidence has grown to cover survival click, stack split/merge, and drag semantics, but creative inventory remains out of scope. A single creative action can cover the serverbound packet boundary while avoiding broad creative-mode claims.

## Decisions

### 1. Use one slot and one item

**Choice:** Configure one creative-mode actor to set or clear one deterministic slot with one deterministic item/count.

**Rationale:** Creative inventory has broad special behavior. One exact slot/item makes the row reviewable.

### 2. Require game-mode and server mutation evidence

**Choice:** Evidence must record creative game mode, client action, Valence server acceptance, final slot state, and child revisions.

**Rationale:** A packet send alone is insufficient; reviewers need server-side mutation correlation.

### 3. Keep survival inventory rows unchanged

**Choice:** Add a dedicated creative rail and keep existing inventory rows scoped to their current survival/player-inventory semantics.

**Rationale:** Creative actions have different invariants and should not pollute existing rows.

## Risks / Trade-offs

- Creative mode setup may require fixture-specific permissions/game-mode handling.
- Slot numbering can be ambiguous; the row must name semantic and wire slot meanings.
- This row does not cover creative pick-block, search tabs, all items, or all mutation modes.
