# Proposal: Survival crafting table parity rail

## Why

The survival coverage matrix still lists crafting as missing. A small crafting-table recipe is the highest-ROI next survival row because it exercises container opening, recipe placement, result-slot transfer, inventory mutation, and backend recipe semantics without claiming broad survival parity.

## What Changes

- Add a bounded protocol-763 `survival-crafting-table` survival scenario for one deterministic crafting table, one configured recipe, one configured input stack set, one result stack, and exact inventory/result metrics.
- Add a Stevenarella probe path that can open the configured crafting table, place the configured input stacks, collect the configured result stack, and emit client milestones for each transition.
- Add paired Paper and Valence fixture instrumentation: Paper and Valence fixtures must prepare the same crafting table position, recipe inputs, and expected result while logging normalized recipe and inventory outcomes.
- Add deterministic checker coverage that rejects missing reference evidence, missing recipe/result metrics, mismatched result item/count, mismatched consumed inputs, stale child revisions, and Valence-only evidence.
- Promote only the `crafting` survival coverage matrix row after paired evidence passes.

## Impact

- **Files**: `tools/mc-compat-runner/src/main.rs`, Stevenarella probe code, `valence/examples/survival_compat.rs`, `tools/paper-survival-fixture/`, row checker, survival matrix/current bundle docs, and `docs/evidence/` artifacts.
- **Testing**: runner unit tests, checker positive and negative fixtures, paired Paper/Valence dry-run or live receipts, BLAKE3 evidence manifests, and Cairn validation/gates.
- **Non-claims**: full crafting coverage, all recipes, recipe-book behavior, shift-click matrices, all container transaction modes, full survival compatibility, broad vanilla parity, and production readiness.
