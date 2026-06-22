# Proposal: Migrate survival crafting recipe breadth to typed-event pass/fail

## Why

`survival-crafting-recipe-breadth` already has paired Paper/Valence receipts, a dedicated wrapper, and checker-backed normalized evidence for shaped crafting, shapeless crafting, invalid-input rejection, and collection. The row still advertises `substring-fallback`, leaving a mismatch between its evidence quality and manifest migration state.

Moving this row to typed-event-ready makes the structured recipe sequence the pass/fail source without expanding recipe breadth or survival claims.

## What Changes

- Mark `survival-crafting-recipe-breadth` as `typed-event-ready` in the scenario manifest and generated surfaces.
- Extend the typed-event pass/fail gate to include `Scenario::SurvivalCraftingRecipeBreadth`.
- Add positive and negative runner fixtures for shaped recipe, shapeless recipe, grid clear, invalid rejection, collection, server correlation, forbidden surfaces, and ordering.
- Add manifest readiness fixtures for the crafting recipe breadth row.
- Update documentation that names the typed-event-ready scenario set.
- Preserve the existing wrapper, receipt schema, dry-run shape, current-bundle row, and non-claims.

## Impact

- **Files**: `compat/config/scenario-manifest.ncl`, generated scenario surfaces, `compat/runner/src/main.rs`, `tools/check_scenario_manifest.rs`, README/evidence docs, and Cairn lifecycle files.
- **Testing**: focused runner tests, scenario-manifest checks, generated-surface freshness, crafting recipe breadth dry-run wrapper check, evidence manifest validation, Cairn gates, and Cairn validation.
- **Non-claims**: this changes only the validation basis for one bounded recipe-breadth row. It does not claim all recipes, recipe-book UI behavior, arbitrary collection modes, shift-click/drag/split semantics, full survival compatibility, public-server safety, production readiness, or semantic equivalence.
