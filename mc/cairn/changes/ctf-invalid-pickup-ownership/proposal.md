# Proposal: CTF invalid pickup ownership rail

## Why

The CTF rule ledger lists invalid pickup acceptance as an unpromoted rule family. The harness needs a negative rule rail proving disallowed pickup/ownership transitions stay contained.

## What Changes

- Add a bounded `ctf-invalid-pickup-ownership` row for one configured invalid flag pickup attempt by the wrong team or invalid owner state with no ownership transfer and no score.
- Define normalized metrics: player team, flag identity, pre-owner state, invalid pickup action, post-owner state, score counters, forbidden capture/score patterns, and containment outcome.
- Require evidence standard: live Valence CTF receipt with negative containment checker and BLAKE3-backed logs.
- Add fixture/runner/checker work: CTF fixture places the client in a disallowed pickup state and logs rejected ownership transition milestones.
- Reject overclaims and bad evidence: unexpected owner transfer, unexpected score/capture, missing forbidden-pattern scan, missing server correlation, or broad all-invalid-action claim.
- Update acceptance/current-bundle docs only for this row after evidence passes.

## Impact

- **Area**: CTF rule correctness.
- **Files**: runner/client probes, Valence/Paper or protocol fixtures as applicable, row checker, evidence docs/manifests, acceptance matrix/current bundle, and Cairn specs/tasks.
- **Testing**: positive and negative checker fixtures, row-specific dry-run/live receipts where safe, evidence manifest check, task-evidence gate, and Cairn validation/gates.
- **Non-claims**: all invalid actions, all flag permutations, full CTF correctness, adversarial security, production readiness, and broad Minecraft compatibility.
