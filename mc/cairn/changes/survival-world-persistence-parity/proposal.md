# Proposal: Survival world persistence parity rail

## Why

The survival coverage matrix still lists world persistence as missing. Existing chest/furnace rows may cover reconnect within a running server, but a restart/reload row is needed before any persisted-world claim can be made.

## What Changes

- Add a bounded protocol-763 `survival-world-persistence-restart` survival scenario for one configured persisted world directory, one configured state mutation, one controlled backend restart or reload, and one post-restart observation of the same state.
- Add a Stevenarella probe path that can perform the configured state mutation, disconnect while the runner restarts or reloads the backend, reconnect, observe the configured persisted state, and emit client milestones for both sessions.
- Add paired Paper and Valence fixture instrumentation: Paper and Valence fixtures must use isolated persisted world storage, deterministic cleanup, and normalized pre/post restart state logs for the same configured mutation.
- Add deterministic checker coverage that rejects missing reference evidence, missing restart logs, missing pre/post metrics, mismatched persisted state, dirty world fixture reuse, stale child revisions, and Valence-only evidence.
- Promote only the `world persistence` survival coverage matrix row after paired evidence passes.

## Impact

- **Files**: `tools/mc-compat-runner/src/main.rs`, Stevenarella probe code, `valence/examples/survival_compat.rs`, `tools/paper-survival-fixture/`, row checker, survival matrix/current bundle docs, and `docs/evidence/` artifacts.
- **Testing**: runner unit tests, checker positive and negative fixtures, paired Paper/Valence dry-run or live receipts, BLAKE3 evidence manifests, and Cairn validation/gates.
- **Non-claims**: long-term durability, crash recovery, multi-chunk persistence, all containers, all block entities, concurrent saves, backups, full survival compatibility, broad vanilla parity, and production readiness.
