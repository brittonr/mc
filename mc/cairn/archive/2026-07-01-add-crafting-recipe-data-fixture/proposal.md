## Candidate decision

Selected: `add-crafting-recipe-data-fixture`

Why now: the crafting behavior card and selected-matrix pure core exist, but the core intentionally uses local in-memory rows only. The next evidence blocker is a Java Edition 1.20.1 / protocol 763 selected recipe-data fixture with malformed-data validation before receipt handoff, Valence shell work, or stronger target-version claims.

Prerequisites satisfied: `docs/crafting-recipe-behavior-card.md` names target-version recipe JSON for `minecraft:chest` and `minecraft:oak_planks` as required before behavior claims; `docs/crafting-recipe-selected-matrix-core.md` names target-version recipe-row extraction, malformed-data validation, selected receipt handoff, and shell work as next required evidence; `cairn/specs/vanilla-composable-plugins/spec.md` records the accepted crafting card and selected-matrix core requirements; archived Cairns `2026-07-01-add-crafting-recipe-behavior-card` and `2026-07-01-add-crafting-recipe-selected-matrix-core` provide the predecessor artifacts.

Deferred alternatives: furnace work is deferred because the furnace chain already has selected fixture, receipt handoff, shell contract, and opt-in runtime shell evidence; crafting receipt handoff is deferred until a validated selected recipe-data fixture exists; Valence crafting shell work is deferred until fixture and handoff evidence exist; all-recipe breadth, data-pack loading, recipe-book behavior, automated crafter behavior, shift-click/drag/split handling, and DefaultPlugins membership are out of scope.

Non-claims: no fixture implementation in this target-hunt package, no all-recipe breadth, no data-pack loading, no recipe-book UI behavior, no automated crafter behavior, no arbitrary collection modes, no Valence runtime integration, no DefaultPlugins membership change, no broad vanilla parity, no public-server safety, and no production readiness.

## Why

The selected-matrix crafting core proves local unit semantics only. It does not prove that the shaped chest or shapeless oak-planks rows came from Java Edition 1.20.1 recipe data, and it does not validate malformed target-data cases before rows reach the core. A dedicated data-fixture Cairn keeps the next implementation small, reviewable, and sequenced before receipt handoff or runtime shell claims.

## What Changes

- Add requirements for a target-scoped selected crafting recipe fixture covering the shaped `minecraft:chest` row, the shapeless `minecraft:oak_planks` row, invalid/no-result fixture data, source/provenance fields, named grid/stack constants, and explicit non-claims.
- Require a focused fixture validator with positive and negative tests for target scope, selected row membership, malformed shaped/shapeless data, duplicate or missing rows, invalid item IDs, zero output counts, unsupported recipe kinds, unsupported collection modes, and overclaim rejection.
- Permit wiring the validated fixture into the existing selected-matrix core checker only as local unit evidence, while preserving non-claims for receipt parity, Valence runtime behavior, all-recipe breadth, and default plugin membership.

## Impact

- **Files**: Future implementation will likely add a Nickel fixture/contract under `compat/config/`, a focused Rust checker under `tools/`, documentation under `docs/`, and promoted evidence under `docs/evidence/`. This package adds only Cairn proposal/design/tasks/spec artifacts.
- **Testing**: Planned validation includes baseline core validation, focused positive and negative fixture checks, optional local core handoff, Cairn validation/gates, task-evidence validation, accepted-spec sync verification, and evidence-manifest checks.
- **Non-claims**: The change does not claim all recipes, Paper/vanilla parity, Valence shell behavior, recipe-book behavior, data-pack loading, automated crafter behavior, public-server safety, or production readiness.
