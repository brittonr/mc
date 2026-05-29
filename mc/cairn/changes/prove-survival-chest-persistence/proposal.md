# Proposal: Prove survival chest persistence row

## Why

The survival coverage matrix still lists `chest persistence` as missing. Chest storage is a high-value survival behavior because it crosses player inventory, container windows, block state, and reconnect/session continuity. It is narrower and lower-risk than full crafting or world-persistence work, but it reduces an explicit survival breadth gap.

## What Changes

- Add a bounded protocol-763 `survival-chest-persistence` rail for one deterministic chest, one stored item stack, one container slot, and one reconnect/reopen cycle.
- Produce paired Valence and Paper reference receipts/logs under `docs/evidence/` with normalized chest metrics.
- Add a deterministic checker that rejects missing/mismatched chest metrics and Valence-only evidence.
- Promote only the `chest persistence` row in the survival coverage matrix when the paired evidence passes.
- Keep full survival compatibility, all-container coverage, restart/world persistence, hopper/redstone inventory behavior, and broader vanilla parity as non-claims.

## Impact

- **Files**: `tools/mc-compat-runner/src/main.rs`, `stevenarella/src/server/mod.rs`, `valence/examples/survival_compat.rs`, `tools/paper-survival-fixture/src/main/java/mc/compat/paper/SurvivalFixturePlugin.java`, checker/evidence docs/manifests, Cairn specs/tasks.
- **Testing**: runner unit tests, Valence example tests, survival chest checker positive/negative fixtures, paired Paper+Valence receipts, evidence manifest check, Cairn validation.
