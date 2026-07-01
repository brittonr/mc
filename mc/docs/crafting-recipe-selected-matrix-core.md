# Crafting recipe selected-matrix pure core

## Scope

This document records the first implementation slice from `docs/crafting-recipe-behavior-card.md`: a pure deterministic selected-matrix crafting recipe core.

Target research scope remains Java Edition 1.20.1 / protocol 763, but this package uses local in-memory selected rows only. It does not use extracted target-version recipe JSON and does not claim vanilla parity.

## Implemented artifact

- `tools/check_crafting_recipe_core.rs`

The tool contains a pure core plus a thin CLI self-test shell. The core accepts in-memory values and returns values. It does not read files, fetch network pages, mutate Bevy world state, emit packets/events, write logs, inspect environment variables, parse data packs, or depend on wall-clock time.

## State model

`CraftingGrid` contains the selected 3×3 grid slots as optional item stacks.

`RecipeMatrix` contains the selected in-memory rows:

- Shaped `minecraft:chest` row: eight `minecraft:oak_planks` in the 3×3 ring produce `minecraft:chest` count `1`.
- Shapeless `minecraft:oak_planks` row: one `minecraft:oak_log` produces `minecraft:oak_planks` count `4`.
- Invalid stick-input rejection is represented as a selected no-result probe, `minecraft:stick_insufficient_input_rejection`, over one `minecraft:oak_planks` input in slot `1`.

`OutputSlotState` contains the current result-slot stack. `CollectionRequest` is limited to primary-click collection into a named target inventory slot. `CraftingLimits` names the maximum stack and result-slot stack limits.

## Decisions, transitions, and diagnostics

Successful primary-click matching returns `CraftingDecision::Matched` with:

- matched recipe id;
- output stack;
- consumed grid state;
- proposed inventory delta.

Rejected states return one of:

- `CraftingDecision::NoResult` for the invalid selected probe while preserving grid and target inventory state;
- `CraftingDecision::OutputBlocked` for incompatible or full result-slot state while preserving grid and target inventory state;
- typed `CraftingError` diagnostics for missing selected rows, duplicate recipe ids, malformed shaped rows, malformed shapeless rows, invalid item ids, zero output counts, missing target data, unsupported recipe kinds, unsupported collection modes, or incompatible target inventory capacity.

## Positive tests

The self-test covers:

- shaped chest matching and primary-click collection into an empty target slot;
- shapeless oak-planks matching and primary-click collection into an empty target slot;
- primary-click collection merging with compatible inventory capacity.

## Negative tests

The self-test covers:

- insufficient stick-input/no-result rejection;
- blocked output preserving grid and inventory state;
- missing selected data;
- duplicate recipe ids;
- malformed shaped rows;
- malformed shapeless rows;
- invalid item ids;
- zero output counts;
- missing target data;
- unsupported recipe kinds;
- unsupported collection modes: shift-click, drag, split, recipe-book UI, and automated crafter;
- incompatible target inventory capacity.

## Evidence

Focused evidence for the original selected-matrix core package is local unit evidence only:

- `docs/evidence/crafting-recipe-selected-matrix-core-inventory-2026-07-01.md`
- `docs/evidence/crafting-recipe-selected-matrix-core-baseline-2026-07-01.run.log`
- `docs/evidence/crafting-recipe-selected-matrix-core-focused-validation-2026-07-01.run.log`
- `docs/evidence/crafting-recipe-selected-matrix-core-2026-07-01.b3`

The selected-matrix data fixture is now documented in `docs/crafting-recipe-selected-matrix-data-fixture.md`. Its validation and local core handoff remain selected-fixture/unit evidence only and do not promote receipt parity or Valence runtime behavior.

## Non-claims

This package does not claim:

- target-version recipe JSON extraction;
- data-pack loading;
- recipe-book UI behavior;
- recipe discovery, advancement, or `doLimitedCrafting` behavior;
- automated crafter behavior;
- arbitrary collection modes;
- shift-click, drag, or split breadth;
- Valence runtime integration;
- Bevy/ECS shell behavior;
- `DefaultPlugins` membership changes;
- all-recipe breadth;
- broad Minecraft compatibility;
- broad vanilla parity;
- public-server safety;
- production readiness.

## Fixture handoff

`docs/crafting-recipe-selected-matrix-data-fixture.md` defines the selected Java Edition 1.20.1 / protocol 763 fixture rows for shaped chest, shapeless oak-planks, and invalid/no-result rejection. `tools/check_crafting_recipe_core.rs --self-test --fixture compat/config/crafting-recipe-selected-matrix-fixture.ncl` feeds those validated rows into the pure core as local unit evidence only.

## Next required evidence before broader claims

Before promoting target-version or runtime behavior beyond this local selected-matrix unit core plus fixture handoff, follow-on work must add selected receipt handoff, a Valence shell contract, opt-in runtime shell tests, and separate scopes for all-recipe breadth, data-pack loading, recipe-book behavior, automated crafter behavior, or additional collection modes.
