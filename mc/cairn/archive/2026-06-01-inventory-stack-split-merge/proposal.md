# Proposal: Inventory stack split merge rail

## Why

Full inventory semantics remains a non-claim. Stack splitting and merging are explicitly outside the current bounded inventory/drop/click/open-container rows.

## What Changes

- Add `inventory-stack-split-merge` as a row-scoped Cairn for one configured item stack split into two stacks and merged back under one window/state-id sequence.
- Define normalized metrics: initial slot/item/count, split action, carried stack count, destination slot/count, merge action, final slot counts, state id, and server click-slot correlation.
- Require evidence standard: live Valence receipt with client and server inventory metrics plus checker negative fixtures for wrong counts/state ids.
- Reject bad evidence and overclaims: missing split/merge metrics, wrong carried count, wrong final count, missing state-id correlation, unexpected item id, or full inventory overclaim.
- Update docs only after validation, preserving explicit non-claims.

## Impact

- **Files**: runner/client probes, fixtures/checkers, evidence docs/manifests, acceptance matrix/current bundle, and Cairn specs/tasks as applicable.
- **Validation**: row checker self-tests, row evidence checks, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation.
- **Non-claims**: all inventory transactions, drag actions, creative mode, all windows, all item lifecycle correctness, broad protocol coverage, and production readiness.
