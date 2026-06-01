# Proposal: Inventory extra window types rail

## Why

Open-container evidence currently covers one bounded chest/container path. Other window types remain non-claims and should be added one row at a time.

## What Changes

- Add `inventory-extra-window-types` as a row-scoped Cairn for one additional configured window type with open, click/transfer, close, and final inventory/window state metrics.
- Define normalized metrics: window type, window id, opened title/type, slot mapping, action item/count, final window slot state, final player inventory state, and server correlation.
- Require evidence standard: paired or Valence-scoped receipt with window-type checker and explicit non-claims for other windows.
- Reject bad evidence and overclaims: missing window type, wrong slot mapping, missing open/close, wrong final item state, Valence-only vanilla parity, or all-window overclaim.
- Update docs only after validation, preserving explicit non-claims.

## Impact

- **Files**: runner/client probes, fixtures/checkers, evidence docs/manifests, acceptance matrix/current bundle, and Cairn specs/tasks as applicable.
- **Validation**: row checker self-tests, row evidence checks, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation.
- **Non-claims**: all window types, crafting/furnace/chest rows already scoped elsewhere, all container transactions, all inventory semantics, and production readiness.
