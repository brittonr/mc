# Proposal: Promote inventory drag transaction evidence

## Why

Drag transactions remain an explicit non-claim in the protocol-763 Valence evidence bundle. The prior archived `inventory-drag-transactions` work described row-level contract expectations, but the maintained matrix and current bundle still do not promote a live reviewable drag transaction row.

## What Changes

- Add a narrow `inventory-drag-transactions` scenario/evidence promotion for one configured player-inventory left-drag transaction.
- Require client drag-phase milestones, Valence server `ClickSlot`/quick-craft correlation, state-id and slot-count metrics, negative checker fixtures, and reviewable evidence under `docs/evidence/` before matrix promotion.
- Promote only this configured drag row in the acceptance matrix/current bundle and keep all drag modes, creative inventory, all windows, all click modes, all inventory semantics, broad parser coverage, full protocol compatibility, public-server safety, and production readiness as non-claims.

## Impact

- **Files**: scenario manifest/generated runner metadata, runner/client probes, Valence fixture instrumentation if needed, row checker fixtures, evidence docs/manifests, acceptance matrix/current bundle, Cairn specs/tasks.
- **Testing**: row checker positive/negative fixtures, runner scenario tests, scenario manifest check, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation.
