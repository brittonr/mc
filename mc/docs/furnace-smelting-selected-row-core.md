# Furnace smelting selected-row pure core

## Scope

This document records the first implementation slice from `docs/furnace-smelting-behavior-card.md`: a pure deterministic standard-furnace selected-row core.

Target research scope remains Java Edition 1.20.1 / protocol 763, but this package uses local in-memory fixture rows only. It does not use extracted target-version data and does not claim vanilla parity.

## Implemented artifact

- `tools/check_furnace_smelting_core.rs`

The tool contains a pure core plus a thin CLI self-test shell. The core accepts values and returns values. It does not read files, fetch network pages, mutate Bevy world state, emit packets/events, write logs, inspect environment variables, or depend on wall-clock time.

The CLI shell can additionally run `--fixture compat/config/furnace-smelting-selected-row-fixture.ncl` to read the validated selected-row fixture and hand its row values to the pure core for local unit validation. That file read is outside the core boundary and remains local handoff evidence only.

## State model

`FurnaceState` contains:

- `FurnaceKind`, supported only for `standard` in this slice.
- Optional input stack.
- Optional fuel stack.
- Optional output stack.
- `cook_progress_ticks`.
- `remaining_burn_ticks`.
- `recipes_completed`.

`RecipeRow` contains:

- Input item ID.
- Output item ID.
- Output count.
- Cook ticks.

`FuelRow` contains:

- Fuel item ID.
- Burn ticks.

Named constants define the selected-row defaults, including standard furnace cook ticks and maximum stack size. Numeric fixture values are named in the checker so the core and tests do not depend on unexplained magic numbers.

## Transitions and errors

Successful one-tick outcomes return a new state and one `FurnaceTransition`:

- `StartedFuel`
- `AdvancedCooking`
- `ProducedOutput`
- `PausedNoFuel`
- `PausedNoRecipe`
- `PausedOutputBlocked`

Rejected inputs return typed errors:

- `UnsupportedFurnaceKind`
- `MalformedRecipeRow`
- `MalformedFuelRow`

Pause transitions preserve state that must not change, such as fuel on missing recipe and input on blocked output.

## Positive tests

The self-test covers:

- Starting fuel for a valid selected standard-furnace recipe.
- Advancing cooking with remaining burn time without consuming another fuel item.
- Producing output into a compatible non-full stack.
- Completing a cook, consuming one input item, producing exactly the selected output, resetting cook progress, and incrementing completed recipe count.

## Negative tests

The self-test covers:

- Missing recipe returns `PausedNoRecipe` and preserves fuel.
- Missing fuel returns `PausedNoFuel`.
- Wrong output item returns `PausedOutputBlocked` and preserves input.
- Full output stack returns `PausedOutputBlocked`.
- Malformed recipe row returns `MalformedRecipeRow`.
- Unsupported furnace kind returns `UnsupportedFurnaceKind`.

## Non-claims

This package does not claim:

- Valence runtime integration.
- Bevy/ECS shell behavior.
- DefaultPlugins membership changes.
- Broad Minecraft compatibility.
- Broad vanilla parity.
- Broad extracted Java Edition 1.20.1 recipe/fuel data coverage.
- All recipes.
- Smoker or blast-furnace behavior.
- Hopper automation.
- XP behavior.
- Recipe-book synchronization.
- Chunk-unload semantics.
- Public-server safety.
- Production readiness.

## Receipt handoff

`docs/furnace-smelting-selected-row-receipt-handoff.md` distinguishes this local unit core from selected-row receipt evidence. The handoff proves that the validated fixture row matches archived Paper/reference and Valence normalized receipt metrics for RawIron + Coal -> IronIngot timing. `docs/furnace-smelting-valence-shell-contract.md` maps this core to the opt-in Bevy/ECS shell boundary. `docs/furnace-smelting-valence-runtime-shell.md` records the first selected-row Valence shell evidence; this core remains the pure semantic owner and is still not a broad recipe, fuel, or live parity claim.

## Next required evidence before broader claims

Before promoting target-version behavior beyond this local unit core plus receipt handoff, follow-on work must add:

- All-recipe Java Edition 1.20.1 recipe/fuel extraction if breadth is claimed.
- Separate evidence extending `docs/furnace-smelting-valence-runtime-shell.md` before claiming runtime behavior beyond the selected standard-furnace row.
- Separate behavior cards for hoppers, XP, recipe book, smoker, blast furnace, and chunk-unload semantics.
