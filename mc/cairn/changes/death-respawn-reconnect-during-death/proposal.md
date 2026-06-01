# Proposal: Death respawn reconnect during death rail

## Why

Reconnect during death is a listed lifecycle non-claim. Existing reconnect and death rows are separate and do not prove the combined state machine.

## What Changes

- Add a bounded `death-respawn-reconnect-during-death` row for one death event followed by disconnect before respawn, reconnect, and coherent dead/respawnable or respawned state according to the fixture policy.
- Define normalized metrics: death milestone, disconnect point, reconnect username/session, server retained death state, client post-reconnect state, respawn action, and final health/playable state.
- Require evidence standard: two-session live receipt with state-machine checker and forbidden stale/alive-state checks.
- Add fixture/runner/checker work: fixture kills the client, forces disconnect timing, preserves server state, and logs reconnect/respawn state transitions.
- Reject overclaims and bad evidence: missing disconnect timing, lost death state, duplicate entity/session confusion, unexpected alive state, missing respawn final state, or full reconnect safety overclaim.
- Update acceptance/current-bundle docs only for this row after evidence passes.

## Impact

- **Area**: death/respawn breadth.
- **Files**: runner/client probes, Valence/Paper or protocol fixtures as applicable, row checker, evidence docs/manifests, acceptance matrix/current bundle, and Cairn specs/tasks.
- **Testing**: positive and negative checker fixtures, row-specific dry-run/live receipts where safe, evidence manifest check, task-evidence gate, and Cairn validation/gates.
- **Non-claims**: all reconnect timings, crash recovery, multi-client reconnect races, full death/respawn lifecycle, production readiness, and unbounded reconnect safety.
