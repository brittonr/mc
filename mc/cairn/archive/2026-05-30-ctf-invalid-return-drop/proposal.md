# Proposal: CTF invalid return/drop rail

## Why

The CTF rule ledger lists invalid return/drop acceptance as an unpromoted rule family. A bounded negative rail should prove invalid return/drop attempts do not corrupt flag state.

## What Changes

- Add a bounded `ctf-invalid-return-drop` row for one configured invalid flag return or drop attempt with unchanged flag state and no unexpected score.
- Define normalized metrics: flag identity, actor team, pre-state, invalid return/drop action, post-state, score counters, forbidden transitions, and server containment milestone.
- Require evidence standard: live negative CTF receipt with client/server attempted-action evidence and forbidden-transition scan.
- Add fixture/runner/checker work: CTF fixture creates a disallowed return/drop state and logs both attempted action and rejected state transition.
- Reject overclaims and bad evidence: missing attempted-action evidence, flag state mutation, unexpected score/capture, missing server containment, or all-return/drop overclaim.
- Update acceptance/current-bundle docs only for this row after evidence passes.

## Impact

- **Area**: CTF rule correctness.
- **Files**: runner/client probes, Valence/Paper or protocol fixtures as applicable, row checker, evidence docs/manifests, acceptance matrix/current bundle, and Cairn specs/tasks.
- **Testing**: positive and negative checker fixtures, row-specific dry-run/live receipts where safe, evidence manifest check, task-evidence gate, and Cairn validation/gates.
- **Non-claims**: all invalid return/drop permutations, full CTF correctness, adversarial security, production readiness, and broad Minecraft compatibility.
