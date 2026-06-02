# Proposal: Repeated death respawn safety rail

## Why

Repeated-death safety remains outside the bounded lifecycle row. A small repeated cycle rail can prove state cleanup across more than one death/respawn without claiming unbounded lifecycle correctness.

## What Changes

- Add a bounded `death-respawn-repeated-death-safety` row for a configured finite sequence of death and respawn cycles with stable health, entity identity, inventory policy, and no duplicate terminal state.
- Define normalized metrics: cycle index, death cause, respawn request, restored health, entity/session id, inventory policy state, forbidden duplicate deaths, and final playable state.
- Require evidence standard: live receipt with fixed finite cycle count and checker fixtures for stale state and duplicate terminal transitions.
- Add fixture/runner/checker work: fixture causes deterministic repeated deaths and respawns, logging each cycle with stable correlation ids.
- Reject overclaims and bad evidence: missing cycle metric, stale health, duplicate death without respawn, lost entity correlation, unexpected score/capture, or unbounded lifecycle overclaim.
- Update acceptance/current-bundle docs only for this row after evidence passes.

## Impact

- **Area**: death/respawn breadth.
- **Files**: runner/client probes, Valence/Paper or protocol fixtures as applicable, row checker, evidence docs/manifests, acceptance matrix/current bundle, and Cairn specs/tasks.
- **Testing**: positive and negative checker fixtures, row-specific dry-run/live receipts where safe, evidence manifest check, task-evidence gate, and Cairn validation/gates.
- **Non-claims**: unbounded repeated death safety, all death causes, reconnect-during-death, inventory semantics outside configured policy, production readiness, and full lifecycle correctness.
