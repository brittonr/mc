# Proposal: Equipment slot item matrix expansion rail

## Why

Current equipment evidence observes one main-hand remote entity update. Other equipment slots, item types, and packet permutations remain non-claims.

## What Changes

- Add a bounded `equipment-slot-item-matrix-expansion` row for a bounded matrix of configured equipment slots, item ids, counts, and remote observer update expectations.
- Define normalized metrics: actor identity, observer identity, slot, item id, item count, update order, remote entity id, and client/server correlation ids.
- Require evidence standard: matrix checker with per-row client/server correlation and no broad slot/item claim outside listed rows.
- Add fixture/runner/checker work: fixtures equip deterministic items into configured slots while a remote Stevenarella observer records equipment updates.
- Reject overclaims and bad evidence: missing slot/item fields, wrong slot mapping, missing observer update, item/count mismatch, stale entity id, or all-equipment overclaim.
- Update acceptance/current-bundle docs only for this row after evidence passes.

## Impact

- **Area**: residual combat breadth.
- **Files**: runner/client probes, Valence/Paper or protocol fixtures as applicable, row checker, evidence docs/manifests, acceptance matrix/current bundle, and Cairn specs/tasks.
- **Testing**: positive and negative checker fixtures, row-specific dry-run/live receipts where safe, evidence manifest check, task-evidence gate, and Cairn validation/gates.
- **Non-claims**: all equipment slots/items, equipment packet permutations, armor mitigation, enchantment/status effects, and production readiness.
