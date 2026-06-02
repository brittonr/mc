# Proposal: Inventory drag transaction rail

## Why

Drag transactions remain outside current inventory semantics. They need a separate bounded row because their packet/state-id shape differs from ordinary clicks.

## What Changes

- Add `inventory-drag-transactions` as a row-scoped Cairn for one configured drag transaction across a fixed set of slots with exact final item/count distribution.
- Define normalized metrics: window id, state id, drag phase sequence, source stack, target slots, per-slot final counts, carried remainder, and server transaction correlation.
- Require evidence standard: live receipt and checker fixtures for drag phase order, target slots, and final distribution.
- Reject bad evidence and overclaims: missing phase, out-of-order drag sequence, wrong slot distribution, stale state id, missing server correlation, or all-transaction overclaim.
- Update docs only after validation, preserving explicit non-claims.

## Impact

- **Files**: runner/client probes, fixtures/checkers, evidence docs/manifests, acceptance matrix/current bundle, and Cairn specs/tasks as applicable.
- **Validation**: row checker self-tests, row evidence checks, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation.
- **Non-claims**: all drag modes, creative inventory, all windows, split/merge outside this row, full inventory semantics, broad protocol coverage, and production readiness.
