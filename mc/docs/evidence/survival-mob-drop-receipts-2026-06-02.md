# Survival mob-drop paired receipts — 2026-06-02

This checkpoint closes the receipt task for `survival-mob-drop`.

Evidence produced:

- Paper live run: `docs/evidence/survival-mob-drop-paper-2026-06-02.receipt.json`, run log, typed events, Paper fixture source/jar, and row evidence key/value file.
- Valence live run: `docs/evidence/survival-mob-drop-valence-2026-06-02.receipt.json`, run log, typed events, and row evidence key/value file.
- Paper fixture jar: `docs/evidence/mc-compat-paper-survival-mob-drop-fixture-2026-06-02.jar`.
- Row comparator: `tools/check_survival_row_parity.rs --row survival-mob-drop` passed for Paper and Valence row evidence.

Observed matching normalized metrics:

- `mob_spawn=mob=IronGolem position=16.5,65.0,2.5`
- `client_attack=mob=IronGolem action=InteractEntity_Attack target=fixture_mob`
- `server_death=mob=IronGolem killed_by=compatbot`
- `drop_spawn=item=IronIngot count=1 position=16.5,65.0,2.5`
- `pickup=item=IronIngot count=1 collector=compatbot`
- `inventory_increment=slot=36 item=IronIngot count_before=0 count_after=1`
- `server_drop_state=mob=IronGolem drop=IronIngot drop_count=1 pickup=true inventory_slot=36 inventory_count=1 death=true`

Child revisions recorded in receipts:

- Valence: `eba9c8a01095f0e243beee899375182c25033fc8`
- Stevenarella: `7122e47b6ab25a33f89094ff8aa4ad5e17873ef6`
- Paper fixture source: `paper-fixture-source-7b71051d`

Non-claims: this proves one deterministic Iron Golem fixture, one client attack, one server-side death, one Iron Ingot drop, one pickup, and one hotbar inventory increment to slot 36. It does not claim vanilla mob loot tables in general, random drop distributions, all hostile/passive mobs, all pickup races, public-server safety, load behavior, or broad vanilla survival parity.
