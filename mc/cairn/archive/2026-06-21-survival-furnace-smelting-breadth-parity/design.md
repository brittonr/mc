# Design: Survival furnace smelting breadth parity

## Context

The existing `survival-furnace-persistence` row covers one furnace recipe and persistence observation. This change defines a separate breadth row for a finite smelting/fuel matrix.

## Decisions

### 1. Bound the smelting matrix

**Choice:** The row covers explicitly named recipe/fuel variants plus one configured rejection case.

**Rationale:** Finite variants make parity reviewable without claiming every recipe or fuel.

### 2. Normalize progress and output metrics

**Choice:** Records compare input item/count, fuel item/count, burn state, cook progress checkpoints, output item/count, collection result, and rejection outcome.

**Rationale:** Comparable metrics allow fail-closed checking across Paper and Valence.

### 3. Keep automation and long timing separate

**Choice:** Hopper automation, furnace minecarts, chunk unload/reload, and long-running timing parity stay out of scope.

**Rationale:** Those need dedicated rows with different fixture controls.
