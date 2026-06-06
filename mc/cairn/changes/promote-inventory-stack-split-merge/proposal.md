# Proposal: Promote inventory stack split/merge evidence

## Why

Stack split/merge remains an explicit non-claim in the current protocol-763 Valence evidence bundle. The prior archived `inventory-stack-split-merge` work defined the row contract and generic fixture/checker expectations, but the accepted matrix and current bundle still do not promote a live reviewable row.

## What Changes

- Add a narrow `inventory-stack-split-merge` scenario/evidence promotion for one configured survival inventory stack split and merge-back sequence.
- Require client milestones, Valence server `ClickSlot` correlation, state-id/count metrics, negative checker fixtures, and reviewable evidence under `docs/evidence/` before matrix promotion.
- Promote only this row in the acceptance matrix/current bundle and keep drag transactions, creative inventory, all windows, all click modes, all inventory semantics, broad parser coverage, full protocol compatibility, and production readiness as non-claims.

## Impact

- **Files**: scenario manifest/generated runner metadata, runner/client probes, Valence fixture instrumentation if needed, row checker fixtures, evidence docs/manifests, acceptance matrix/current bundle, Cairn specs/tasks.
- **Testing**: row checker positive/negative fixtures, runner scenario tests, scenario manifest check, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation.
