# Survival hunger/food paired receipts — 2026-06-02

This checkpoint closes the receipt task for `survival-hunger-food`.

Evidence produced:

- Paper live run: `docs/evidence/survival-hunger-food-paper-2026-06-02.receipt.json`, run log, client log, server log, typed events, and row evidence key/value file.
- Valence live run: `docs/evidence/survival-hunger-food-valence-2026-06-02.receipt.json`, run log, client log, server log, typed events, and row evidence key/value file.
- Paper fixture jar: `docs/evidence/mc-compat-paper-survival-hunger-food-fixture-2026-06-02.jar`.
- Row comparator: `tools/check_survival_row_parity.rs --row survival-hunger-food` passed for Paper and Valence row evidence.

Observed matching normalized metrics:

- `pre_consume_food=health=20.0 food=15 saturation=0.0 item=Bread count=1 slot=36`
- `consume_start=item=Bread slot=36 food_before=15 saturation_before=0.0`
- `consume_finish=item=Bread slot=36 food_after=20 saturation_after=6.0`
- `item_decrement=slot=36 item=Bread count_before=1 count_after=0`
- `post_consume_food=health=20.0 food=20`
- `saturation_update=before=0.0 after=6.0`
- `server_food_state=health=20.0 food_before=15 food_after=20 saturation_before=0.0 saturation_after=6.0 unexpected_damage=false death=false`

Child revisions recorded in receipts:

- Valence: `573b8f0f33db3ab2ff7d01c3762dc3f3c72cad02`
- Stevenarella: `e996816fe3fca74a96f5ff7996c2fa4dad310bb4`
- Paper fixture source: `paper-fixture-source-a9592778`

Non-claims: this proves one deterministic hunger deficit, one Bread item in inventory slot 36, one main-hand consume action, one food update from 15 to 20, one saturation update from 0.0 to 6.0, and one inventory decrement from Bread x1 to empty. It does not claim all foods, all hunger mechanics, natural exhaustion, regeneration/starvation, potion effects, offhand consumption, public-server safety, load behavior, or broad vanilla survival parity.
