# Proposal: Full protocol 763 compatibility aggregate gate

## Why

Full protocol-763 compatibility remains blocked while most packet rows lack parser fixtures and live receipts. An aggregate gate should make the blocking condition machine-checkable.

## What Changes

- Add `full-protocol-763-compatibility-gate` as a row-scoped Cairn for an aggregate checker over protocol-763 packet inventory requiring every required packet-family row to have mapping, parser fixtures, live evidence, owner, and next action before full protocol claim promotion.
- Define normalized metrics: packet row count, family status, mapping status, parser fixture id, malformed fixture status, live receipt path, owner, next action, and digest.
- Require evidence standard: protocol ledger aggregate checker and negative fixtures for missing parser/live evidence or fallback aliases.
- Reject bad evidence and overclaims: fallback alias, missing parser fixture, missing live receipt, missing owner, missing next action, stale packet inventory, or full protocol overclaim.
- Update docs only after validation, preserving explicit non-claims.

## Impact

- **Files**: runner/client probes, fixtures/checkers, evidence docs/manifests, acceptance matrix/current bundle, and Cairn specs/tasks as applicable.
- **Validation**: row checker self-tests, row evidence checks, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation.
- **Non-claims**: full protocol-763 compatibility, full Minecraft compatibility, all gameplay semantics, production readiness, and security robustness until all required rows pass.
