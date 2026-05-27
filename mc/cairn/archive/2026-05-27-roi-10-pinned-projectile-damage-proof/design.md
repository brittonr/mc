# Design: Pinned projectile damage proof

## Dependency strategy

Use the local nested `valence/` repository as the pinned server instrumentation source for the next proof. The checkpoint records exact commit `e5d18ad04010d92881267ac1ea43922ae91821f5` (`ctf: add projectile hit compat probe`) and requires a clean Valence worktree before live evidence. Live commands must pass the exact revision, not `HEAD`, and logs/receipts must record the resolved revision.

## Causality strategy

Treat projectile damage attribution as a causal sequence, not a set membership check. The runner should parse ordered client and server milestone lines and require this sequence for the same bounded two-client scenario:

1. client projectile use is sent;
2. client projectile swing is sent;
3. server records projectile use with probe sequence/damage for a concrete attacker/victim pair;
4. server records projectile hit for the same attacker/victim and health delta;
5. the victim client records the post-hit health value named by the server hit milestone.

A receipt can pass only when the ordered proof exists. Missing, mismatched, or out-of-order milestones must fail with explicit missing/forbidden/order diagnostics.

## Evidence strategy

Keep ROI 08 demoted until this change produces:

- deterministic dry-run receipt and log;
- runner unit tests covering positive and negative ordering cases;
- live receipt/log under `docs/evidence/`;
- BLAKE3 sidecars covering all review-critical artifacts;
- updated matrix, bundle, residual docs, and checkers re-promoting the row.

## Risks

- Server and client logs may not share timestamps. Mitigate by using the run log merge order plus exact scenario marker names, and document the proof boundary as ordered log evidence rather than physics proof.
- The Valence commit may move or be unavailable to another reviewer. Mitigate by recording the exact nested repository commit and requiring a clean checkout status in evidence.
