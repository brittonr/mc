# Proposal: CTF score limit win condition rail

## Why

The CTF rule ledger lists score limit and win condition as an unpromoted rule family. Current scoring rows prove individual scores, not match completion behavior.

## What Changes

- Add a bounded `ctf-score-limit-win-condition` row for one bounded match reaching a configured score limit and emitting the configured win/end state exactly once.
- Define normalized metrics: score limit, team scores before final capture, final capture actor/team, win team, end-state milestone, duplicate-win guard, and post-win forbidden score changes.
- Require evidence standard: live CTF receipt with score-limit contract, no duplicate win, and matrix/current-bundle non-claims preserved.
- Add fixture/runner/checker work: CTF fixture starts near the score limit, drives one final deterministic capture, and logs win-condition milestones.
- Reject overclaims and bad evidence: missing prelimit state, missing final capture, wrong winning team, duplicate win, post-win score mutation, or full CTF overclaim.
- Update acceptance/current-bundle docs only for this row after evidence passes.

## Impact

- **Area**: CTF rule correctness.
- **Files**: runner/client probes, Valence/Paper or protocol fixtures as applicable, row checker, evidence docs/manifests, acceptance matrix/current bundle, and Cairn specs/tasks.
- **Testing**: positive and negative checker fixtures, row-specific dry-run/live receipts where safe, evidence manifest check, task-evidence gate, and Cairn validation/gates.
- **Non-claims**: all match settings, overtime/tiebreakers, scoreboard UI parity, all scoring races, production gameplay readiness, and full CTF correctness.
