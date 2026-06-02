# Proposal: Survival furnace persistence parity rail

## Why

The survival coverage matrix still lists furnace persistence as missing. Furnace behavior crosses container state, burn progress, recipe output, inventory collection, and reconnect/session persistence, making it the next storage-like survival row after chest persistence.

## What Changes

- Add a bounded protocol-763 `survival-furnace-persistence` survival scenario for one deterministic furnace block, one configured input stack, one fuel stack, one smelted output stack, and one reconnect/reopen observation within the same server process.
- Add a Stevenarella probe path that can open the configured furnace, place input and fuel, wait for the bounded output condition, collect the output, reconnect, reopen, and emit client milestones for the observed slots.
- Add paired Paper and Valence fixture instrumentation: Paper and Valence fixtures must prepare the same furnace position and accelerated deterministic smelt envelope while logging normalized input, fuel, progress, output, and reconnect-state metrics.
- Add deterministic checker coverage that rejects missing reference evidence, missing burn/output/reconnect metrics, mismatched output item/count, stale progress, wrong backend, and Valence-only evidence.
- Promote only the `furnace persistence` survival coverage matrix row after paired evidence passes.

## Impact

- **Files**: `tools/mc-compat-runner/src/main.rs`, Stevenarella probe code, `valence/examples/survival_compat.rs`, `tools/paper-survival-fixture/`, row checker, survival matrix/current bundle docs, and `docs/evidence/` artifacts.
- **Testing**: runner unit tests, checker positive and negative fixtures, paired Paper/Valence dry-run or live receipts, BLAKE3 evidence manifests, and Cairn validation/gates.
- **Non-claims**: all smelting recipes, long-running furnace timing parity, hopper automation, furnace minecarts, server restart/world persistence, full survival compatibility, broad vanilla parity, and production readiness.
