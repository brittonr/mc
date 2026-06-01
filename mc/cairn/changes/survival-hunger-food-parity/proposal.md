# Proposal: Survival hunger and food parity rail

## Why

The survival coverage matrix still lists hunger/food as missing. Eating one configured food item gives a focused health-state rail that exercises item use, inventory decrement, hunger/saturation updates, and client-visible state changes.

## What Changes

- Add a bounded protocol-763 `survival-hunger-food` survival scenario for one deterministic hunger deficit, one configured food item, one consume action, one hunger/saturation delta, and one inventory decrement.
- Add a Stevenarella probe path that can start from a fixture-imposed hunger deficit, use the configured food item until consumption completes, and emit client milestones for item use, inventory change, and food/health updates.
- Add paired Paper and Valence fixture instrumentation: Paper and Valence fixtures must set the same hunger/saturation precondition, give the same food item, and log normalized consume, inventory, hunger, saturation, and optional health metrics.
- Add deterministic checker coverage that rejects missing reference evidence, missing hunger/saturation metrics, mismatched food delta, missing inventory decrement, unexpected damage/death, and Valence-only evidence.
- Promote only the `hunger/food` survival coverage matrix row after paired evidence passes.

## Impact

- **Files**: `tools/mc-compat-runner/src/main.rs`, Stevenarella probe code, `valence/examples/survival_compat.rs`, `tools/paper-survival-fixture/`, row checker, survival matrix/current bundle docs, and `docs/evidence/` artifacts.
- **Testing**: runner unit tests, checker positive and negative fixtures, paired Paper/Valence dry-run or live receipts, BLAKE3 evidence manifests, and Cairn validation/gates.
- **Non-claims**: all foods, starvation loops, regeneration balance, potion/status effects, exhaustion math, sprint/jump hunger drain, full survival compatibility, broad vanilla parity, and production readiness.
