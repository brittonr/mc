# Proposal: Automate evidence promotion

## Why

Promoting evidence is currently a manual multi-step process: run a rail, copy receipts/logs from `target/`, compute BLAKE3, update manifests, update acceptance matrix, update current bundle, run freshness gates, and avoid stale child-repo metadata. Review history shows this is easy to get wrong. A first-party promotion tool should make the safe path the default.

## What Changes

- Add a repo-owned evidence promotion command or Rust tool with dry-run and apply modes.
- Capture receipt, run log, client/server logs, child-repo revision metadata, BLAKE3 manifests, matrix row updates, and current-bundle updates from one declared rail run.
- Validate required artifacts before mutation and fail closed if evidence is incomplete or non-claim text would be weakened.
- Keep manual oracle checkpoints available only for explicitly historical or non-machine-recorded evidence.

## Impact

- **Files**: `tools/`, `flake.nix`, `docs/evidence/`, README, acceptance/current-bundle checkers.
- **Testing**: positive/negative promotion fixtures, dry-run no-mutation tests, missing-artifact failure tests, stale-hash failure tests, matrix non-claim preservation tests.
- **Non-claims**: promotion automation manages evidence hygiene only; it does not decide that a compatibility claim is true.
