# Survival crash-recovery revision oracle — 2026-06-04

## Question

Which child revisions backed the promoted `survival-crash-recovery-parity` Paper/Valence evidence when the receipts did not machine-record those child revisions?

## Inspected evidence

- `docs/evidence/survival-crash-recovery-revisions-2026-06-04.run.log` records `git -C valence rev-parse HEAD` as `ed1ed0335114c473424312870a77b37de2091a3e` with an empty `git -C valence status --short`.
- The same log records `git -C stevenarella rev-parse HEAD` as `75151ca3cd4ed6673c0daf3dc3579a5069cc4ac3` with an empty `git -C stevenarella status --short`.
- The same log records the Paper fixture source BLAKE3 as `471a8753c1d08c7fb41179858443a4756811587aae83d4b17e1557f89ce1a0d6` and the promoted fixture jar BLAKE3 as `769620b513c73df99136c1a0ba68a19b708a83eee883dd83f9ccfaf5549c9436`.

## Decision

Use Valence child revision `ed1ed0335114c473424312870a77b37de2091a3e`, Stevenarella child revision `75151ca3cd4ed6673c0daf3dc3579a5069cc4ac3`, and Paper fixture source key `paper-fixture-source-471a8753c1d0` for the normalized crash-recovery evidence files.

## Owner

Cairn evidence owner for `survival-crash-recovery-parity`.

## Next action

Keep this checkpoint and the revision run log in `docs/evidence/` with the paired receipts, comparator log, and BLAKE3 manifest.
