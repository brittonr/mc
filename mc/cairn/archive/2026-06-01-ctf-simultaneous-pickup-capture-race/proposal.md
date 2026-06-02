# Proposal: CTF simultaneous pickup capture race rail

## Why

The CTF rule ledger lists simultaneous pickup/capture races as unpromoted. A deterministic multi-client race fixture is needed before race safety can be claimed.

## What Changes

- Add a bounded `ctf-simultaneous-pickup-capture-race` row for one configured two-client race window with deterministic ordering oracle and exactly one accepted state transition.
- Define normalized metrics: client identities, team roles, action timestamps or ordered milestones, accepted transition, rejected transition, final flag state, final score, and race-window bounds.
- Require evidence standard: multi-client live receipt with deterministic ordering metadata and negative forbidden-transition checks.
- Add fixture/runner/checker work: CTF fixture coordinates two clients near flags/portals and logs ordered race decisions from the server authority.
- Reject overclaims and bad evidence: missing order evidence, double accept, inconsistent final flag state, unexpected score, missing client/server correlation, or all-race overclaim.
- Update acceptance/current-bundle docs only for this row after evidence passes.

## Impact

- **Area**: CTF rule correctness.
- **Files**: runner/client probes, Valence/Paper or protocol fixtures as applicable, row checker, evidence docs/manifests, acceptance matrix/current bundle, and Cairn specs/tasks.
- **Testing**: positive and negative checker fixtures, row-specific dry-run/live receipts where safe, evidence manifest check, task-evidence gate, and Cairn validation/gates.
- **Non-claims**: all race conditions, network adversarial safety, unbounded concurrency, full CTF correctness, production readiness, and broad Minecraft compatibility.
