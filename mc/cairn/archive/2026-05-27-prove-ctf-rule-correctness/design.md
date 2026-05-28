# Design: Full CTF rule correctness proof

## Rule inventory

Create a CTF rule ledger with one row per invariant: team selection, own-flag constraints, enemy-flag pickup, capture preconditions, drop/return timing, death handling, reconnect handling, score updates, reset behavior, and invalid action rejection.

## Oracle strategy

Use Valence server milestones for authoritative rule state and Stevenarella client milestones for observed client effects. Every promoted rule needs correlated evidence from both sides when the client should observe a result. Negative cases must prove the forbidden server state or score transition did not happen.

## Scenario strategy

Split the proof into small scenario clusters rather than one monolithic run. Each cluster should have deterministic dry-run fixtures, bounded live run, explicit missing/forbidden milestone diagnostics, and a narrow matrix row.

## Risks

- Long multi-rule scenarios can become flaky. Keep runs short and isolate rule clusters.
- Some CTF rules may be server-only. Mark those with server-only evidence and do not claim client-observed behavior unless Stevenarella evidence exists.
