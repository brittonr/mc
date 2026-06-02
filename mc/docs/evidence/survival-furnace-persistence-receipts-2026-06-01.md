# Survival furnace persistence paired receipts — 2026-06-01

This checkpoint closes the receipt task for `survival-furnace-persistence`.

Evidence produced:

- Paper live run: `docs/evidence/survival-furnace-persistence-paper-2026-06-01.receipt.json`, run log, client log, server log, typed events, and row evidence key/value file.
- Valence live run: `docs/evidence/survival-furnace-persistence-valence-2026-06-01.receipt.json`, run log, client log, server log, typed events, and row evidence key/value file.
- Paper fixture jar: `docs/evidence/mc-compat-paper-survival-furnace-fixture-2026-06-01.jar`.
- Row comparator: `tools/check_survival_row_parity.rs --row survival-furnace-persistence` passed for Paper and Valence row evidence.

Observed matching normalized metrics:

- `furnace_open=window=1`
- `input_insert=RawIron`
- `fuel_insert=Coal`
- `burn_progress_start=started`
- `output_available=IronIngot`
- `output_collect=IronIngot`
- `reconnect_reopen=window=1`
- `server_state=collected=true;session_persistent=true`

Child revisions recorded in receipts:

- Valence: `61a1f773d8967b0d7d22719771277bba3eb73f37`
- Stevenarella: `d9caec597041b3443d894701591752d23772e5ae`
- Paper fixture source BLAKE3 prefix: `ac722a39`

Non-claims: this proves one deterministic furnace block, one raw-iron input, one coal fuel item, one accelerated output observation, one output collection, and one reconnect/reopen observation within the same server process. It does not claim all smelting recipes, long-running furnace timing parity, hopper automation, furnace minecarts, server restart/world persistence, full survival compatibility, broad vanilla parity, public-server safety, load behavior, or production readiness.
