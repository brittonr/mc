# Proposal: Ordinary death respawn rail

## Why

Death/respawn coverage currently uses flag-carrier CTF death. Ordinary non-flag death remains a lifecycle non-claim and needs its own bounded row.

## What Changes

- Add `death-respawn-ordinary-death` as a row-scoped Cairn for one ordinary player death outside flag-carrier state followed by respawn request, restored health, and playable post-respawn state.
- Define normalized metrics: death cause, pre-death health, death milestone, respawn request, post-respawn health, post-respawn position, flag-state absence, inventory policy, and server correlation.
- Require evidence standard: live receipt/log bundle with client/server lifecycle milestones and checker fixtures for missing/out-of-order respawn.
- Reject bad evidence and overclaims: flag-carrier-only evidence, missing death cause, missing respawn request, missing restored health, stale flag state, unexpected score/capture, or full lifecycle overclaim.
- Update docs only after validation, preserving explicit non-claims.

## Impact

- **Files**: runner/client probes, fixtures/checkers, evidence docs/manifests, acceptance matrix/current bundle, and Cairn specs/tasks as applicable.
- **Validation**: row checker self-tests, row evidence checks, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation.
- **Non-claims**: all death causes, inventory drop/reset semantics, reconnect-during-death, invalid-respawn timing, repeated deaths, full CTF correctness, and production readiness.
