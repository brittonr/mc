# Design: Survival crafting recipe breadth parity

## Context

The existing `survival-crafting-table` row proves one stick recipe with paired Paper/reference and Valence evidence. This change defines the next row as finite recipe breadth, not full crafting semantics.

## Decisions

### 1. Keep the recipe matrix finite

**Choice:** The row covers exactly one shaped recipe, one shapeless recipe, one invalid/insufficient-input rejection, and one configured result collection mode.

**Rationale:** A finite matrix is reviewable and lets the checker reject all-recipe overclaims.

### 2. Compare normalized recipe and inventory metrics

**Choice:** Paper and Valence records must normalize recipe id, input slots/items/counts, result slot/item/count, collection mode, final inventory slot/count, and rejection outcome.

**Rationale:** Row parity should be checked over comparable deterministic fields rather than free-form logs.

### 3. Preserve adjacent non-claims

**Choice:** Recipe-book UI, all recipes, arbitrary collection modes, and broad survival remain explicit non-claims.

**Rationale:** The row expands evidence breadth without converting a bounded fixture into an aggregate crafting claim.
