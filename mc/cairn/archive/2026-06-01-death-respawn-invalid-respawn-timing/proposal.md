# Proposal: Invalid respawn timing rail

## Why

Invalid respawn timing remains outside the death/respawn lifecycle evidence. A negative rail should prove early/duplicate respawn attempts are contained for one configured state.

## What Changes

- Add `death-respawn-invalid-respawn-timing` as a row-scoped Cairn for one invalid respawn attempt before the fixture allows respawn plus one valid respawn path after the configured state transition.
- Define normalized metrics: pre-death state, invalid respawn attempt timing, containment result, death state retained, valid respawn request, restored health, duplicate-respawn guard, and server correlation.
- Require evidence standard: negative live receipt with attempted-action evidence, containment milestones, and forbidden premature-alive patterns.
- Reject bad evidence and overclaims: missing invalid attempt evidence, premature alive state, duplicate entity/session, missing valid respawn final state, or full lifecycle overclaim.
- Update docs only after validation, preserving explicit non-claims.

## Impact

- **Files**: runner/client probes, fixtures/checkers, evidence docs/manifests, acceptance matrix/current bundle, and Cairn specs/tasks as applicable.
- **Validation**: row checker self-tests, row evidence checks, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation.
- **Non-claims**: all respawn timing races, reconnect-during-death, repeated deaths, crash recovery, production readiness, and unbounded lifecycle correctness.
