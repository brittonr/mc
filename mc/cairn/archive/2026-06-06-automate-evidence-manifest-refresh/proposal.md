# Proposal: Automate evidence manifest refresh

## Why

Evidence manifests under `docs/evidence/*.b3` are updated by hand during Cairn drains. When a cited log, accepted spec, archive task, or nested manifest changes, digest updates can cascade across several files. Manual refresh is slow and easy to get wrong; stale rows have already caused avoidable evidence-manifest and task-evidence failures after otherwise valid work.

## What Changes

- Add a repo-owned deterministic manifest refresh/check tool for `docs/evidence/*.b3` files.
- Support a check-only mode for CI and a mutating refresh mode for local evidence preparation.
- Preserve non-existent-file rows for review instead of silently deleting or rewriting them.
- Wire the helper into the existing evidence-manifest workflow so future Cairn drains can refresh manifests without ad hoc shell loops.

## Impact

- **Files**: a new tool under `tools/`, `flake.nix` check/app wiring, evidence workflow docs, and focused tests.
- **Testing**: positive fixture refresh, stale-manifest detection, missing-file preservation, outside-root rejection, malformed-row rejection, and existing evidence-manifest/task-evidence checks.