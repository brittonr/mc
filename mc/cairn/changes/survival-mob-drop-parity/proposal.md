# Proposal: Survival mob drop parity rail

## Why

The survival coverage matrix still lists mob drops as missing. A deterministic spawn/kill/drop/pickup rail adds entity lifecycle coverage without trying to prove broad mob AI or loot-table behavior.

## What Changes

- Add a bounded protocol-763 `survival-mob-drop` survival scenario for one configured mob, one bounded kill interaction, one configured drop stack, one pickup, and exact entity/drop metrics.
- Add a Stevenarella probe path that can approach or face the configured mob, perform the bounded attack sequence, observe the configured drop entity, pick it up, and emit client milestones for attack, death/drop visibility, and pickup/inventory change.
- Add paired Paper and Valence fixture instrumentation: Paper and Valence fixtures must spawn the same low-variance mob at the configured position, constrain health/AI as needed for determinism, and log normalized death/drop/pickup metrics.
- Add deterministic checker coverage that rejects missing reference evidence, missing spawn/death/drop/pickup metrics, mismatched drop item/count, wrong entity identity, uncontrolled extra drops, and Valence-only evidence.
- Promote only the `mob drops` survival coverage matrix row after paired evidence passes.

## Impact

- **Files**: `tools/mc-compat-runner/src/main.rs`, Stevenarella probe code, `valence/examples/survival_compat.rs`, `tools/paper-survival-fixture/`, row checker, survival matrix/current bundle docs, and `docs/evidence/` artifacts.
- **Testing**: runner unit tests, checker positive and negative fixtures, paired Paper/Valence dry-run or live receipts, BLAKE3 evidence manifests, and Cairn validation/gates.
- **Non-claims**: mob AI parity, pathfinding, all entities, all loot tables, combat balancing, experience drops, farms/spawners, full survival compatibility, broad vanilla parity, and production readiness.
