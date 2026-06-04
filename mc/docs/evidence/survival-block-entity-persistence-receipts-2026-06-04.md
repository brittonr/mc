# Survival sign block-entity persistence paired receipts — 2026-06-04

This checkpoint closes the receipt task for `survival-block-entity-persistence-parity`.

Evidence produced:

- Paper/reference live run: `docs/evidence/survival-block-entity-persistence-paper-2026-06-04.receipt.json`, typed events, pre/post-restart server logs, client session logs, Paper fixture source/jar, and row evidence key/value file.
- Valence live run: `docs/evidence/survival-block-entity-persistence-valence-2026-06-04.receipt.json`, typed events, pre/post-restart server logs, client session logs, and row evidence key/value file.
- Paper fixture jar: `docs/evidence/mc-compat-paper-survival-block-entity-fixture-2026-06-04.jar`.
- Row comparator: `tools/check_survival_row_parity.rs --row survival-block-entity-persistence-parity` passed for Paper and Valence row evidence in `docs/evidence/survival-block-entity-persistence-row-parity-2026-06-04.run.log`.

Observed matching normalized metrics:

| Metric | Value |
| --- | --- |
| `actor` | `compatbot` |
| `block_entity_kind` | `Sign` |
| `position` | `28,64,0` |
| `text_payload` | `MC\|Compat\|Sign\|Persist` |
| `pre_restart_observation` | `sign_text_visible` |
| `clean_shutdown` | `graceful` |
| `backend_restart` | `controlled_reload` |
| `reconnect` | `restart` |
| `post_restart_observation` | `sign_text_visible` |
| `server_persistence_state` | `persisted` |

Scope:

- This promotes only one configured sign block entity: actor `compatbot`, kind `Sign`, position `28,64,0`, text payload `MC|Compat|Sign|Persist`, controlled restart, reconnect, and post-restart observation.
- The Paper fixture writes an `OAK_SIGN` and lets the mounted Paper world persist the sign NBT across the controlled restart; the Valence fixture writes the same sign payload and uses isolated marker storage for the bounded row.
- Non-claims remain: all block-entity parity, arbitrary NBT parity, sign editing UI parity, multi-chunk persistence, broad survival compatibility, public-server safety, and production readiness.
