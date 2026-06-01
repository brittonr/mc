# Proposal: Protocol equipment permutation family coverage rail

## Why

All equipment permutations remain broad protocol non-claims. This rail adds mapping/parser and live evidence for selected equipment packet permutations separate from gameplay mitigation claims.

## What Changes

- Add a bounded `protocol-equipment-permutation-family` row for a named subset of equipment update packet permutations with reviewed parser fixtures and remote observer receipts.
- Define normalized metrics: equipment packet name, wire id, entity id, slot, item id, count, parser fixture id, live observer receipt, and digest.
- Require evidence standard: protocol ledger row with parser fixtures plus equipment observer live evidence and explicit gameplay non-claims.
- Add fixture/runner/checker work: protocol tests cover selected equipment payloads and live fixture causes remote observer equipment updates for those slots/items.
- Reject overclaims and bad evidence: missing parser fixture, wrong slot mapping, stale entity id, missing live observer receipt, or all-equipment claim.
- Update acceptance/current-bundle docs only for this row after evidence passes.

## Impact

- **Area**: broad protocol coverage.
- **Files**: runner/client probes, Valence/Paper or protocol fixtures as applicable, row checker, evidence docs/manifests, acceptance matrix/current bundle, and Cairn specs/tasks.
- **Testing**: positive and negative checker fixtures, row-specific dry-run/live receipts where safe, evidence manifest check, task-evidence gate, and Cairn validation/gates.
- **Non-claims**: all equipment permutations, armor mitigation, combat balancing, all item types, full protocol-763 compatibility, and production readiness.
