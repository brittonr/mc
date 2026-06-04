# Design: Bounded survival crash-recovery parity

## Goal

Promote one narrow survival crash-recovery row only when Paper-reference and Valence evidence agree for the same deterministic mutation: `compatbot` places `Dirt` at `24,64,0`, the backend is stopped ungracefully by the runner, the backend restarts from the same isolated storage, and the client observes the same block state after reconnect. The row is evidence for this configured crash-recovery interaction only.

## Evidence contract

The row id is `survival-crash-recovery-parity`. A valid KV comparator input must name:

- `row=survival-crash-recovery-parity`
- `backend=paper` or `backend=valence`
- clean child revision metadata
- `metric.pre_crash_mutation=block=Dirt position=24,64,0 persisted_before=false persisted_after=true`
- `metric.crash_stop=method=forced_stop storage=isolated graceful=false`
- `metric.backend_restart=method=crash_recovery storage=isolated restart_confirmed=true`
- `metric.reconnect=session=crash_recovery`
- `metric.post_crash_observation=block=Dirt position=24,64,0 persisted=true`
- `metric.server_recovery_state=block=Dirt position=24,64,0 pre_mutation=true crash_stop=true backend_restart=true post_observed=true dirty_reuse=false`

The existing `survival-world-persistence-restart` row remains the graceful restart row. This change does not promote long-term durability, crash consistency across arbitrary write timing, multi-chunk persistence, block entities, concurrent saves, backups, full survival compatibility, broad vanilla parity, public-server safety, or production readiness.

## Functional core

`tools/check_survival_row_parity.rs` remains the pure deterministic comparator core:

- `RowContract` selects allowed metrics for a row id.
- `EvidenceDoc` normalizes one backend's KV input.
- `validate_pair(contract, paper, valence)` rejects missing Paper evidence, Valence-only evidence, stale revisions, missing metrics, and metric mismatches without I/O.

The new crash row extends the contract table and self-test loop. Positive fixtures cover matching Paper/Valence crash evidence. Negative fixtures cover missing metrics, mismatched metrics, stale revisions, unknown child revisions, Valence-only evidence, and unknown row rejection.

## Imperative shell

Runner and fixtures own side effects:

- `tools/mc-compat-runner` exposes `--scenario survival-crash-recovery-parity` and maps it to crash-specific client/server milestone requirements.
- The runner prepares isolated storage, starts the backend, runs one mutation client session, records pre-crash logs, stops the backend ungracefully, marks the second session as post-crash, restarts the backend, runs one reconnect session, and correlates both server log windows.
- Paper and Valence fixture code may reuse existing world-persistence storage setup when the crash row is enabled, but emitted milestones must use crash-specific names so the graceful and crash rows cannot satisfy each other accidentally.
- Promotion copies only reviewable artifacts to `docs/evidence/`, including run logs, receipts, server logs, typed events, normalized KV files, comparator logs, and BLAKE3 manifests.

## Promotion rule

Matrix and bundle docs may mark only `survival-crash-recovery-parity` covered after paired comparator evidence passes. Existing survival rows retain their current scope. Long-term durability, crash consistency breadth, multi-chunk persistence, block entities, concurrent saves, backups, broad survival compatibility, broad vanilla parity, public-server safety, and production readiness remain non-claims.

## Validation

Closeout must record:

- row-parity checker self-test/compile log;
- runner unit tests and scenario manifest check;
- paired Paper/Valence receipts or a reviewable blocker if live backend crash orchestration cannot be safely completed;
- paired comparator output over normalized KV files;
- evidence manifest and task-evidence gates;
- Cairn gates and validation.
