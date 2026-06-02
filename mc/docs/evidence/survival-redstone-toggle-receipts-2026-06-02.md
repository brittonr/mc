# Survival redstone-toggle paired receipts — 2026-06-02

This checkpoint closes the receipt task for `survival-redstone-toggle`.

Evidence produced:

- Paper live run: `docs/evidence/survival-redstone-toggle-paper-2026-06-02.receipt.json`, client/server logs, typed events, Paper fixture source/jar, and row evidence key/value file.
- Valence live run: `docs/evidence/survival-redstone-toggle-valence-2026-06-02.receipt.json`, client/server logs, typed events, and row evidence key/value file.
- Paper fixture jar: `docs/evidence/mc-compat-paper-survival-redstone-toggle-fixture-2026-06-02.jar`.
- Row comparator: `tools/check_survival_row_parity.rs --row survival-redstone-toggle` passed for Paper and Valence row evidence.

Observed matching normalized metrics:

- `input_interaction=control=Lever position=20,64,0 powered_before=false powered_after=true`
- `powered_on=output=RedstoneLamp position=21,64,0 powered=true`
- `client_state_update=output=RedstoneLamp position=21,64,0 powered_on=true powered_off=true raw_on=7417 raw_off=7418`
- `powered_off=output=RedstoneLamp position=21,64,0 powered=false`
- `server_power_state=control=Lever output=RedstoneLamp on_seen=true off_seen=true unintended_outputs=false`

Child revisions recorded for review:

- Valence: `469558f87501a979fbcc8a56e21716c43bfec356`
- Stevenarella: `357021defa46ec678221a1590b6e56e9b3239aa8`
- Paper fixture source: `paper-fixture-source-f22cb60b`

Non-claims: this proves one deterministic Lever input, one Redstone Lamp output, one powered-on observation, one returned-off observation, and matching client/server state milestones. It does not claim general redstone circuit parity, tick-order parity, pistons, observers, comparators, clocks, farms, broad block-update breadth, public-server safety, load behavior, production readiness, or broad vanilla survival parity.
