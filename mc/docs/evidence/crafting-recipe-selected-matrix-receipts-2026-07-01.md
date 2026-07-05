# Crafting recipe selected-matrix receipt handoff — 2026-07-01

This checkpoint records the receipt-handoff evidence for `verify-crafting-recipe-selected-matrix-receipts`.

## Inputs

- Fixture: `compat/config/crafting-recipe-selected-matrix-fixture.ncl`.
- Paper/reference normalized evidence: `docs/evidence/survival-crafting-recipe-breadth-paper-2026-06-20.kv`.
- Valence normalized evidence: `docs/evidence/survival-crafting-recipe-breadth-valence-2026-06-20.kv`.
- Contract: `docs/crafting-recipe-selected-matrix-receipt-handoff.md`.
- Checker: `tools/check_crafting_recipe_receipt_handoff.rs`.

## Decision

The archived `survival-crafting-recipe-breadth` Paper/reference and Valence receipt inputs are reused for selected-matrix handoff because the checker validates that they match the selected Java Edition 1.20.1 / protocol 763 fixture rows.

The checker summary in `docs/evidence/crafting-recipe-selected-matrix-receipts-checker-2026-07-01.run.log` records:

- shaped `minecraft:chest` from eight `minecraft:oak_planks` inputs in slots `1,2,3,4,6,7,8,9`, output `minecraft:chest` count `1`, collected into target inventory slot `36`;
- shapeless `minecraft:oak_planks` from one `minecraft:oak_log` input in slot `1`, output `minecraft:oak_planks` count `4`, collected into target inventory slot `37`;
- invalid `minecraft:stick_insufficient_input_rejection` with one `minecraft:oak_planks` input in slot `1`, empty result, `no_result` diagnostic;
- primary-click collection only;
- Paper/reference child evidence `paper-1.20.1-fixture-jar-b3:c5243a9830de707bedaca1084e170283e1a531249165ba38f8ac1662d9888ed2`;
- Valence child evidence `858a453` from the normalized input;
- retained receipt non-claim metrics for all-recipes, recipe-book UI, arbitrary collection modes, full-survival compatibility, and broad vanilla parity.

## Evidence

- Baseline fixture/core validation: `docs/evidence/crafting-recipe-selected-matrix-receipts-baseline-2026-07-01.run.log` (`exit_status=0`).
- Checker self-test and archived receipt handoff: `docs/evidence/crafting-recipe-selected-matrix-receipts-checker-2026-07-01.run.log` (`exit_status=0`).
- BLAKE3 manifest: `docs/evidence/crafting-recipe-selected-matrix-receipts-2026-07-01.b3`.

## Next prerequisite

Before any Valence runtime shell implementation can claim selected-matrix crafting behavior, the next prerequisite is `docs/crafting-recipe-valence-shell-contract.md`, which defines the opt-in Bevy/ECS shell boundary, plugin ownership, schedule contract, data-loading boundaries, disabled-plugin behavior, and validation requirements.

## Non-claims

This handoff proves only that the selected fixture/core inputs match the archived bounded Paper/reference and Valence receipt metrics for the finite selected matrix. It does not add a new live run and does not claim all recipes, arbitrary collection modes, shift-click/drag/split handling, data-pack loading, recipe-book behavior, recipe discovery or advancement behavior, automated crafter behavior, Valence runtime integration, default plugin membership, broad vanilla parity, broad Minecraft compatibility, public-server safety, production readiness, full survival correctness, or semantic equivalence.
