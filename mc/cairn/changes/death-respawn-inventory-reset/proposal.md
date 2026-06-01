# Proposal: Death respawn inventory reset rail

## Why

Current lifecycle evidence covers one flag-carrier death and respawn health restore, but inventory reset/drop semantics remain non-claims.

## What Changes

- Add a bounded `death-respawn-inventory-reset` row for one configured death event with pre-death inventory, death/drop or reset policy, respawn, and post-respawn inventory state.
- Define normalized metrics: pre-death inventory slots, death cause, drop/reset policy, dropped item ids/counts, respawn inventory slots, and server correlation milestones.
- Require evidence standard: live receipt with explicit inventory policy and negative mismatch checks.
- Add fixture/runner/checker work: fixture gives deterministic inventory, causes bounded death, records drops/reset, and respawns the client for post-state observation.
- Reject overclaims and bad evidence: missing pre/post inventory metrics, wrong drop/reset state, missing respawn, unexpected score/capture side effect, or full lifecycle overclaim.
- Update acceptance/current-bundle docs only for this row after evidence passes.

## Impact

- **Area**: death/respawn breadth.
- **Files**: runner/client probes, Valence/Paper or protocol fixtures as applicable, row checker, evidence docs/manifests, acceptance matrix/current bundle, and Cairn specs/tasks.
- **Testing**: positive and negative checker fixtures, row-specific dry-run/live receipts where safe, evidence manifest check, task-evidence gate, and Cairn validation/gates.
- **Non-claims**: all death causes, all inventory policies, XP drops, item despawn timing, full death/respawn lifecycle, full CTF correctness, and production readiness.
