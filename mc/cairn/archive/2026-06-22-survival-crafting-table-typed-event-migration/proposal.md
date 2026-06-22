# Proposal: Migrate survival crafting table to typed-event pass/fail

## Why

`survival-crafting-table` is a maintained survival row with bounded client milestones, Valence crafting-table server correlation, and a dedicated dry-run wrapper. It still relies on waiver-backed substring fallback while nearby inventory and survival break/place rows now fail closed on typed-event evidence.

Moving this row to typed-event-ready makes missing or misordered structured crafting evidence fail before legacy strings can satisfy the row. That improves receipt reviewability without broadening survival crafting claims.

## What Changes

- Mark `survival-crafting-table` as `typed-event-ready` in the scenario manifest and generated surfaces.
- Extend the typed-event pass/fail gate to include `Scenario::SurvivalCraftingTable`.
- Add positive and negative runner fixtures proving required crafting open/input/result/collect/inventory events, server correlation, forbidden surfaces, and ordering are enforced.
- Add manifest readiness fixtures for the crafting-table row.
- Update documentation that names the typed-event-ready scenario set.
- Preserve the existing wrapper, receipt schema, dry-run shape, current-bundle row, and non-claims.

## Impact

- **Files**: `compat/config/scenario-manifest.ncl`, generated scenario surfaces, `compat/runner/src/main.rs`, `tools/check_scenario_manifest.rs`, README/evidence docs, and Cairn lifecycle files.
- **Testing**: focused runner tests, scenario-manifest checks, generated-surface freshness, survival crafting-table dry-run wrapper check, evidence manifest validation, Cairn gates, and Cairn validation.
- **Non-claims**: this changes only the validation basis for one bounded crafting-table row. It does not claim broad crafting recipes, recipe-book UI, all containers, full survival compatibility, public-server safety, production readiness, or semantic equivalence.
