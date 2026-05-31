# Survival crafting table fixture task oracle — 2026-05-31

## Question

Does the `survival-crafting-table-parity` fixture task now have bounded Paper and Valence server-side instrumentation for the scoped crafting scenario?

## Inspected evidence

- Valence child commit `5c97914` (`add survival crafting fixture milestones`) adds `MC_COMPAT_SURVIVAL_CRAFTING_FIXTURE` to `examples/survival_compat.rs` for table `4,64,0`, window `1`, input slots `1` and `4`, result slot `0`, inventory slot `36`, `OakPlanks` inputs, `Stick x4` result, and recipe label `minecraft:stick`.
- `docs/evidence/protocol-763-survival-crafting-table-fixtures-valence-source-2026-05-31.patch` records the Valence source delta and focused positive/negative helper tests.
- `docs/evidence/protocol-763-survival-crafting-table-fixtures-paper-source-2026-05-31.patch` records the Paper fixture delta for `MC_COMPAT_SURVIVAL_CRAFTING_FIXTURE`, scheduled workbench opening, input/result/collect milestones, and fixture jar packaging.
- `docs/evidence/protocol-763-survival-crafting-table-fixtures-2026-05-31.run.log` records Valence focused tests, Paper fixture `javac` compilation against Paper API `1.20.1-R0.1-SNAPSHOT`, Paper fixture jar creation, and a Valence scenario dry-run pinned to `VALENCE_REV=5c97914`.
- `docs/evidence/mc-compat-paper-survival-crafting-fixture-2026-05-31.jar` is the row-specific compiled Paper fixture artifact for later Paper receipt work.
- `docs/evidence/protocol-763-survival-crafting-table-fixtures-valence-dry-run-2026-05-31.receipt.json` is the copied dry-run receipt; no review-critical receipt remains only under `target/`.

## Decision

Mark only the fixture instrumentation task complete. This evidence does not claim paired live Paper/Valence receipts, matrix promotion, broad crafting parity, recipe-book behavior, shift-click behavior, production readiness, or final Cairn archive readiness.

## Owner

Compatibility evidence owner: `mc` maintainers.

## Next action

Run live paired Paper and Valence receipts using the row-scoped fixtures, copy receipts/logs into `docs/evidence/`, and run the paired crafting checker before matrix promotion.
