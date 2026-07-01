## Why

The furnace smelting behavior card now bounds the first standard-furnace selected-row slice. The next step is a pure deterministic rule core with positive and negative tests, without adding a Bevy/ECS shell or making vanilla parity claims.

## What Changes

- Add `tools/check_furnace_smelting_core.rs` containing the selected-row furnace smelting pure core and self-tests.
- Add `docs/furnace-smelting-selected-row-core.md` documenting the implemented local semantics, inputs/outputs, tests, and non-claims.
- Extend the `vanilla-composable-plugins` spec with requirements for selected-row core behavior, test coverage, and closeout evidence.

## Impact

- **Files**: `tools/check_furnace_smelting_core.rs`, `docs/furnace-smelting-selected-row-core.md`, accepted spec updates, archived Cairn package, and evidence logs.
- **Testing**: Baseline validation before core work, focused core self-tests with positive and negative cases, Cairn gates/validation, task-evidence validation, evidence-manifest checks, and flake evidence checks.
- **Non-claims**: No Valence Bevy/ECS shell, no default plugin membership change, no extracted-data breadth, no Paper/vanilla parity, no all recipes, no smoker/blast-furnace behavior, no hoppers/XP/recipe-book/chunk-unload behavior, no public-server safety, and no production readiness.
