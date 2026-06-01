# Proposal: Creative mode inventory rail

## Why

Creative-mode inventory remains an explicit non-claim. It uses different permissions and packet paths than survival inventory interactions.

## What Changes

- Add `inventory-creative-mode` as a row-scoped Cairn for one configured creative inventory action under an owned local fixture with explicit permission, item id/count, and resulting slot state.
- Define normalized metrics: game mode, permission state, creative action type, item id, item count, target slot, client observation, server inventory state, and forbidden survival-only assumptions.
- Require evidence standard: live receipt with explicit creative-mode fixture and checker fixtures for missing permission or wrong item state.
- Reject bad evidence and overclaims: survival-mode evidence, missing permission metric, wrong item/count, missing server state, unexpected survival semantics, or all-creative-inventory overclaim.
- Update docs only after validation, preserving explicit non-claims.

## Impact

- **Files**: runner/client probes, fixtures/checkers, evidence docs/manifests, acceptance matrix/current bundle, and Cairn specs/tasks as applicable.
- **Validation**: row checker self-tests, row evidence checks, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation.
- **Non-claims**: all creative actions, operator/admin safety, public-server creative permissions, all inventory transactions, production readiness, and broad protocol coverage.
