# Survival crafting recipe breadth paired receipts — 2026-06-20

This checkpoint closes the receipt task for `survival-crafting-recipe-breadth`.

Evidence produced:

- Paper live run: `docs/evidence/survival-crafting-recipe-breadth-paper-2026-06-20.receipt.json`, run log, client log, server log, typed events, and normalized key/value evidence.
- Valence live run: `docs/evidence/survival-crafting-recipe-breadth-valence-2026-06-20.receipt.json`, run log, client log, server log, typed events, and normalized key/value evidence.
- Paper fixture jar: `docs/evidence/mc-compat-paper-survival-crafting-breadth-fixture-2026-06-20.jar` (`c5243a9830de707bedaca1084e170283e1a531249165ba38f8ac1662d9888ed2`).
- Row comparator: `tools/check_survival_crafting_recipe_breadth.rs --paper docs/evidence/survival-crafting-recipe-breadth-paper-2026-06-20.kv --valence docs/evidence/survival-crafting-recipe-breadth-valence-2026-06-20.kv` passed with `exit_status=0` in `docs/evidence/survival-crafting-recipe-breadth-checker-2026-06-20.run.log`.

Observed matching normalized metrics:

- Shaped recipe: `minecraft:chest` from eight `OakPlanks` inputs in slots `1,2,3,4,6,7,8,9`, result `Chest x1`, collected by primary click into inventory slot `36`.
- Shapeless recipe: `minecraft:oak_planks` from one `OakLog` input in slot `1`, result `OakPlanks x4`, collected by primary click into inventory slot `37`.
- Invalid/insufficient input: `minecraft:stick_insufficient_input_rejection` with one `OakPlanks` input in slot `1`, result slot empty, rejection outcome `no_result`.
- Both backends recorded client milestones for shaped, shapeless, grid-clear, invalid rejection, and final inventory update, plus server milestones for shaped, shapeless, grid-clear, invalid rejection, and final state.

Child revisions recorded in receipts/evidence:

- Valence: `858a453a1faae8a1f6bebc607266daf284cd7696`.
- Stevenarella: `8e7066e652d9036e050831e90b4d6242fbce866d`.
- Paper backend: `1.20.1` with fixture jar BLAKE3 above; the Paper receipt records clean revision status and server revision `8ad9c8587a3273ec59b0ec4edae0bf790bdf403b`.

Non-claims: this proves only the configured shaped chest recipe, shapeless oak-planks recipe, one invalid/insufficient stick-input rejection, and one primary-click collection mode in the owned local fixture. It does not claim all recipes, recipe-book UI behavior, recipe discovery breadth, arbitrary collection modes, shift-click/drag/split semantics, furnace behavior, hunger/food behavior, mob behavior, redstone behavior, biome/dimension behavior, world persistence, full survival compatibility, broad vanilla parity, public-server safety, production readiness, or semantic equivalence.
