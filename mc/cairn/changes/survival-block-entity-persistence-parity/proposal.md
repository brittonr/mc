## Why

The current survival durability evidence now covers one ordinary `Dirt` world-persistence row and one bounded crash-recovery row, but it still explicitly excludes all block-entity persistence. A narrow paired Paper/reference and Valence row for one configured sign block entity gives reviewers a higher-value durability seam without claiming broad block-entity, container, or NBT parity.

## What Changes

- Add a bounded `survival-block-entity-persistence-parity` row for one deterministic sign block entity mutation, restart/reconnect observation, and normalized Paper/Valence comparison.
- Extend the survival row parity checker, runner scenario manifest, client rail, Paper fixture, and Valence fixture instrumentation only for this row.
- Promote the row only after paired Paper/reference and Valence receipts, normalized KV evidence, BLAKE3 manifests, and matrix/current-bundle non-claims are present.

## Impact

- **Files**: `tools/check_survival_row_parity.rs`, `tools/mc-compat-runner/src/main.rs`, `tools/mc-compat-runner/src/scenario_manifest_generated.rs`, `config/mc-compat/scenario-manifest.ncl`, `tools/paper-survival-fixture/src/main/java/mc/compat/paper/SurvivalFixturePlugin.java`, `docs/evidence/`, acceptance/current-bundle/survival-coverage docs, and Cairn specs/archive files.
- **Testing**: Checker positive/negative fixtures, runner/scenario-manifest checks, paired Paper and Valence live receipts, evidence-manifest gate, task-evidence gate, Cairn gates, and Cairn validation.
