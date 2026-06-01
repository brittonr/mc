# Proposal: CTF spawn team balance reset rail

## Why

The CTF rule ledger lists spawn/team balance/resource reset as unpromoted. Current rows do not prove team assignment balance, respawn resources, or post-score reset invariants.

## What Changes

- Add a bounded `ctf-spawn-team-balance-reset` row for one configured join/team-selection/reset sequence with bounded team counts, spawn locations, inventory/resource state, and reset milestones.
- Define normalized metrics: team counts, selected teams, spawn coordinates, initial resources, post-score or post-death reset state, inventory/resource ids, and server correlation ids.
- Require evidence standard: live CTF receipt with team/spawn/resource matrix row and negative checker for imbalance or stale resources.
- Add fixture/runner/checker work: CTF fixture starts deterministic clients, controls team selection, and logs spawn/resource reset state after the configured trigger.
- Reject overclaims and bad evidence: team imbalance outside bounds, wrong spawn, stale inventory/resource state, missing reset milestone, or full CTF overclaim.
- Update acceptance/current-bundle docs only for this row after evidence passes.

## Impact

- **Area**: CTF rule correctness.
- **Files**: runner/client probes, Valence/Paper or protocol fixtures as applicable, row checker, evidence docs/manifests, acceptance matrix/current bundle, and Cairn specs/tasks.
- **Testing**: positive and negative checker fixtures, row-specific dry-run/live receipts where safe, evidence manifest check, task-evidence gate, and Cairn validation/gates.
- **Non-claims**: all team balancing algorithms, all maps, all resource loadouts, all reset triggers, production gameplay readiness, and full CTF correctness.
