# Survival crash-recovery paired receipts — 2026-06-04

This checkpoint closes the receipt task for `survival-crash-recovery-parity`.

Evidence produced:

- Paper live run: `docs/evidence/survival-crash-recovery-paper-2026-06-04.receipt.json`, typed events, pre/post-crash server logs, client session logs, Paper fixture source/jar, and row evidence key/value file.
- Valence live run: `docs/evidence/survival-crash-recovery-valence-2026-06-04.receipt.json`, typed events, pre/post-crash server logs, client session logs, and row evidence key/value file.
- Paper fixture jar: `docs/evidence/mc-compat-paper-survival-crash-recovery-fixture-2026-06-04.jar`.
- Row comparator: `tools/check_survival_row_parity.rs --row survival-crash-recovery-parity` passed for Paper and Valence row evidence in `docs/evidence/survival-crash-recovery-row-parity-2026-06-04.run.log`.
- Revision checkpoint: `docs/evidence/survival-crash-recovery-revision-oracle-2026-06-04.md` records Valence, Stevenarella, and Paper fixture source/fixture jar revisions.

Observed matching normalized metrics:

| Metric | Value |
| --- | --- |
| `pre_crash_mutation` | `block=Dirt position=24,64,0 persisted_before=false persisted_after=true` |
| `crash_stop` | `method=forced_stop storage=isolated graceful=false` |
| `backend_restart` | `method=crash_recovery storage=isolated restart_confirmed=true` |
| `reconnect` | `session=crash_recovery` |
| `post_crash_observation` | `block=Dirt position=24,64,0 persisted=true` |
| `server_recovery_state` | `block=Dirt position=24,64,0 pre_mutation=true crash_stop=true backend_restart=true post_observed=true dirty_reuse=false` |

Scope:

- This promotes only one configured crash-recovery row: one `compatbot` mutation of `Dirt` at `24,64,0`, isolated storage, runner-forced stop, backend restart, reconnect, and post-crash block observation.
- The Paper fixture flushes this configured mutation before the forced stop to make the bounded `persisted_after=true` precondition reviewable; this is not arbitrary crash-consistency proof.
- Non-claims remain: full survival compatibility, long-term durability, arbitrary crash consistency, multi-chunk persistence, all containers, all block entities, concurrent saves, backups, broad vanilla parity, public-server safety, and production readiness.
