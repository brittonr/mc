# Proposal: Survival redstone toggle parity rail

## Why

The survival coverage matrix still lists redstone as missing. A minimal lever/button-to-output rail gives the first deterministic block-update coverage for powered state changes while leaving complex circuit timing out of scope.

## What Changes

- Add a bounded protocol-763 `survival-redstone-toggle` survival scenario for one configured input control, one configured powered output block, one on/off toggle sequence, and exact powered-state metrics.
- Add a Stevenarella probe path that can use the configured redstone control, wait for the output state update, optionally toggle back, and emit client milestones for use-item-on-block and observed block-state changes.
- Add paired Paper and Valence fixture instrumentation: Paper and Valence fixtures must build the same tiny redstone fixture, log normalized input/output powered states, and avoid unrelated ticking components.
- Add deterministic checker coverage that rejects missing reference evidence, missing powered-state metrics, mismatched output state, extra unintended output changes, wrong fixture position, and Valence-only evidence.
- Promote only the `redstone` survival coverage matrix row after paired evidence passes.

## Impact

- **Files**: `tools/mc-compat-runner/src/main.rs`, Stevenarella probe code, `valence/examples/survival_compat.rs`, `tools/paper-survival-fixture/`, row checker, survival matrix/current bundle docs, and `docs/evidence/` artifacts.
- **Testing**: runner unit tests, checker positive and negative fixtures, paired Paper/Valence dry-run or live receipts, BLAKE3 evidence manifests, and Cairn validation/gates.
- **Non-claims**: redstone circuit parity, tick-order parity, pistons, observers, comparators, clocks, farms, block-update breadth, full survival compatibility, broad vanilla parity, and production readiness.
