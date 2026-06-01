# Proposal: Protocol inventory transaction family coverage rail

## Why

Inventory gameplay rows are bounded, and all inventory transactions remain a broad protocol non-claim. This rail adds packet-family coverage for selected inventory transaction shapes.

## What Changes

- Add a bounded `protocol-inventory-transaction-family` row for a named subset of inventory transaction packet rows with reviewed mapping/parser fixtures and bounded live transaction receipts.
- Define normalized metrics: transaction packet name, state/side, wire id, slot/window/state-id fields, parser fixture id, malformed fixture status, live scenario, and receipt digest.
- Require evidence standard: protocol ledger row plus live inventory receipt and negative checker for stale/invalid transaction where supported.
- Add fixture/runner/checker work: protocol fixtures cover selected click/slot/window payloads and live fixture drives the same transaction family.
- Reject overclaims and bad evidence: fallback alias, missing state-id/window metric, missing malformed fixture, missing live receipt, Valence-only gameplay overclaim, or all-transaction claim.
- Update acceptance/current-bundle docs only for this row after evidence passes.

## Impact

- **Area**: broad protocol coverage.
- **Files**: runner/client probes, Valence/Paper or protocol fixtures as applicable, row checker, evidence docs/manifests, acceptance matrix/current bundle, and Cairn specs/tasks.
- **Testing**: positive and negative checker fixtures, row-specific dry-run/live receipts where safe, evidence manifest check, task-evidence gate, and Cairn validation/gates.
- **Non-claims**: all inventory transactions, all windows, drag/split/merge behavior, all-container semantics, full protocol-763 compatibility, and production readiness.
