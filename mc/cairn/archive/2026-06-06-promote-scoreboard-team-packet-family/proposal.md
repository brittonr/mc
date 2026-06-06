# Proposal: Promote scoreboard/team packet-family evidence

## Why

CTF rows already exercise teams, scores, and scoreboard-like state, but packet inventory coverage for player list/team/scoreboard packet rows remains narrow or implicit. A dedicated packet-family row can promote one normalized scoreboard/team update without claiming UI parity or all scoreboard semantics.

## What Changes

- Add one bounded scoreboard/team packet-family row using existing CTF team/score fixture context where possible.
- Require client observation or packet fixture evidence plus Valence server correlation for one team/scoreboard update.
- Promote only the configured packet row or rows, keeping all scoreboard UI, all team rules, all objective/display variants, full CTF correctness, full protocol coverage, and production readiness as non-claims.

## Impact

- **Files**: runner metadata or packet fixtures, Valence fixture logging if needed, current bundle/packet inventory docs, checker, evidence artifacts, and Cairn specs/tasks.
- **Testing**: positive/negative checker fixtures, focused scenario or fixture tests, packet inventory/current-bundle checks, evidence manifests, task-evidence gate, and Cairn validation.
