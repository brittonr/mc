# Survival world-persistence paired receipts — 2026-06-02

This checkpoint closes the receipt task for `survival-world-persistence-restart`.

Evidence produced:

- Paper live run: `docs/evidence/survival-world-persistence-paper-2026-06-02.receipt.json`, run log, typed events, pre/post restart server logs, Paper fixture source/jar, and row evidence key/value file.
- Valence live run: `docs/evidence/survival-world-persistence-valence-2026-06-02.receipt.json`, run log, typed events, pre/post restart server logs, and row evidence key/value file.
- Paper fixture jar: `docs/evidence/mc-compat-paper-survival-world-persistence-fixture-2026-06-02.jar`.
- Row comparator: `tools/check_survival_row_parity.rs --row survival-world-persistence-restart` passed for Paper and Valence row evidence.

Observed matching normalized metrics:

- `pre_restart_mutation=block=Dirt position=24,64,0 persisted_before=false persisted_after=true`
- `clean_shutdown=storage=isolated marker=written`
- `backend_restart=method=controlled_reload storage=isolated`
- `reconnect=session=restart`
- `post_restart_observation=block=Dirt position=24,64,0 persisted=true`
- `server_persistence_state=block=Dirt position=24,64,0 pre_mutation=true clean_shutdown=true backend_restart=true post_observed=true dirty_reuse=false`

Child revisions recorded for this checkpoint:

- Valence: `d6fddd786eaf3da44826285a4c1f7155ffd3988e`
- Stevenarella: `0a745b04c9bc0589f7c6a5f3f5edf3b4b8014a17`
- Paper fixture source: `paper-fixture-source-669c6607`

Non-claims: this proves one configured persisted world directory, one configured `Dirt` mutation at `24,64,0`, one controlled backend restart/reload, and one post-restart observation of the same state on both Paper and Valence. It does not claim long-term durability, crash recovery, multi-chunk persistence, all containers, all block entities, concurrent saves, backups, full survival compatibility, broad vanilla parity, production readiness, or public-server safety.
