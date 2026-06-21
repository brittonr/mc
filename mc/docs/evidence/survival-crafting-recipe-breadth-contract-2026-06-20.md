# Survival crafting recipe breadth contract — 2026-06-20

## Scope

This contract defines the bounded `survival-crafting-recipe-breadth-parity` row before live evidence promotion. It expands beyond the existing `survival-crafting-table` stick row, but remains finite and row-scoped.

The row covers exactly:

- one shaped recipe: `minecraft:chest` from eight `OakPlanks` inputs;
- one shapeless recipe: `minecraft:oak_planks` from one `OakLog` input;
- one invalid or insufficient-input rejection: `minecraft:stick_insufficient_input_rejection` with one `OakPlanks` input and no result;
- one collection mode: `primary_click` result collection into a configured inventory slot.

## Normalized metrics

Paired Paper/reference and Valence evidence must provide the following key/value metrics for each backend:

- `matrix.version=2026-06-20`
- shaped recipe id, input slots/items/counts, result slot/item/count, collection mode, and final inventory slot/item/count
- shapeless recipe id, input slots/items/counts, result slot/item/count, collection mode, and final inventory slot/item/count
- invalid rejection recipe id, input slots/items/counts, result slot/item/count, and rejection outcome
- explicit non-claim metrics for all recipes, recipe-book UI, arbitrary collection modes, full survival compatibility, and broad vanilla parity

The deterministic checker is `tools/check_survival_crafting_recipe_breadth.rs`. It requires paired `paper` and `valence` records, clean child revision metadata, exact metric values, cross-backend agreement, and fail-closed broad-overclaim rejection.

## Non-claims

This contract is not live promotion evidence. It does not claim all recipes, recipe-book UI behavior, recipe discovery breadth, arbitrary collection modes, shift-click/drag/split semantics, furnace behavior, hunger/food behavior, mob behavior, redstone behavior, biome/dimension behavior, world persistence, full survival compatibility, broad vanilla parity, public-server safety, production readiness, or semantic equivalence.
