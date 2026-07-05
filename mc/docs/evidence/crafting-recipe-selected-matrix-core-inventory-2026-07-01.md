# Crafting recipe selected-matrix core inventory — 2026-07-01

This inventory satisfies `r[vanilla_composable_plugins.crafting_recipe_selected_matrix_core.inventory]` before implementing the selected-matrix pure core. It records accepted prerequisites, predecessor evidence, selected rows, local assumptions, and unresolved target-version data gaps.

## Accepted behavior card and requirements

- `docs/crafting-recipe-behavior-card.md` is the accepted behavior card for the first crafting recipe slice.
- Accepted `cairn/specs/vanilla-composable-plugins/spec.md` requires pure deterministic cores with thin Bevy/ECS shells, positive and negative tests, evidence that does not overclaim, and explicit stop conditions before broader vanilla behavior claims.
- Accepted `cairn/specs/mc-compatibility/spec.md` contains predecessor `survival-crafting-recipe-breadth` requirements and typed-event migration requirements. Those requirements are compatibility/evidence rails, not a reusable plugin-core implementation.

## Selected matrix rows

The core slice is bounded to the behavior-card matrix:

- Shaped row: `minecraft:chest` from eight `minecraft:oak_planks` inputs in the selected 3×3 ring slots `1,2,3,4,6,7,8,9`, expected output `minecraft:chest` count `1`, primary-click collection into inventory slot `36`.
- Shapeless row: `minecraft:oak_planks` from one `minecraft:oak_log` input in slot `1`, expected output `minecraft:oak_planks` count `4`, primary-click collection into inventory slot `37`.
- Invalid/no-result row: `minecraft:stick_insufficient_input_rejection` vocabulary from one `minecraft:oak_planks` input in slot `1`, expected empty result/no-result diagnostic, no collection.

## Predecessor evidence

- Archived `2026-07-01-add-crafting-recipe-behavior-card` accepts the behavior-card contract and non-claims.
- Archived `2026-06-20-survival-crafting-recipe-breadth-parity` and `docs/evidence/survival-crafting-recipe-breadth-receipts-2026-06-20.md` provide paired Paper/reference and Valence row evidence for shaped chest, shapeless oak-planks, invalid/no-result rejection, and primary-click collection.
- Archived `2026-06-22-survival-crafting-recipe-breadth-typed-event-migration` provides typed-event readiness evidence for the same compatibility row.

These artifacts are predecessor vocabulary and row evidence only. They do not prove a reusable pure recipe core, a target-version data extractor, Valence runtime shell behavior, all-recipe breadth, recipe-book behavior, automated crafter behavior, public-server safety, or production readiness.

## Local fixture-core assumptions

Existing `tools/check_survival_crafting_recipe_breadth.rs` normalizes the predecessor row with named metrics for the selected recipe ids, input slots, result counts, primary-click collection mode, final inventory slots, and non-claims. The new core may reuse those row names and constants as in-memory selected rows, but must not treat the receipt comparator as plugin-core evidence.

The selected-matrix core must keep recipe matching and rejection pure over in-memory values. It must not read files, fetch network pages, inspect environment variables, mutate Bevy world state, emit packets/events, write logs, parse data packs, or depend on wall-clock time.

## Unresolved target-version prerequisites

Before stronger target-version or runtime claims, follow-on work still needs:

- Java Edition 1.20.1 / protocol 763 recipe JSON extraction for selected shaped and shapeless rows;
- malformed target-data fixtures and validation;
- receipt handoff that compares selected rows against Paper/reference and Valence evidence through a dedicated checker;
- `docs/crafting-recipe-valence-shell-contract.md` as the Valence Bevy/ECS shell contract and opt-in runtime shell evidence;
- dedicated scope for all-recipe breadth, data-pack loading, recipe-book behavior, automated crafter behavior, and arbitrary collection modes.
