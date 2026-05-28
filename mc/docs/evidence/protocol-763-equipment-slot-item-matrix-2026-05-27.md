# Protocol-763 equipment slot/item matrix proof — 2026-05-27

## Scope

This checkpoint drains the equipment slot/item matrix Cairn by defining the one equipment-update row currently promoted by live evidence and by making every other equipment slot, item class, transition, and packet permutation an explicit non-claim.

It proves one bounded remote-player equipment observation row. It does not claim all equipment slots, all item representatives, empty/non-empty transition coverage, repeated update ordering, armor mitigation, enchantment/status effects, exact vanilla combat parity, or full inventory semantics. All equipment slots/items/permutations remain a non-claim.

## Matrix rows

| Seam | slot key | item representative | transition | update ordering | client observation | server evidence | Receipt | BLAKE3 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Equipment update observation | main_hand_remote_entity / slot4 | item id 829 / count 1 | non_empty_update | single ordered update after remote spawn | `entity_equipment_update`, run-log marker `equipment_probe_entity_equipment entity_id=4 entries=1 slots=slot4:id=829:count=1` | `server_equipment_update_state` | `docs/evidence/protocol-763-roi-02-equipment-update-observation-2026-05-27.receipt.json` | `8100dde3ebb3476984235009e277d7e973037b7873b2fdb30c413093e1498d3d` |

## Non-promoted matrix cells

| Slot/item/permutation | Status | Reason |
| --- | --- | --- |
| head/chest/legs/feet armor equipment update packets | Non-claim | Armor mitigation row observes inventory armor and server mitigation, not remote client equipment breadth. |
| off-hand equipment | Non-claim | No live receipt-backed client/server row. |
| empty-to-non-empty, non-empty-to-empty, repeated updates | Non-claim | No row with update sequence proof. |
| all item representatives | Non-claim | Only item id `829` count `1` is observed in the promoted row. |
| packet ordering/permutation matrix | Non-claim | Only one ordered update after remote spawn is observed. |

## Positive validation

`tools/check_equipment_slot_item_matrix.py` requires:

- `mode=run`, `dry_run=false`, `status=pass`;
- server protocol `763`;
- scenario `equipment-update-observation`;
- no missing client/server milestones;
- no forbidden matches;
- client milestones including `remote_player_spawn` and `entity_equipment_update`;
- server milestones including `server_equipment_update_state`;
- run-log correlation for remote spawn entity `4` and exactly one matching `slot4:id=829:count=1` equipment update;
- matrix/current-bundle digest agreement;
- scoped non-claims for full CTF correctness, broad Minecraft compatibility, unbounded soak, and production load.

## Negative fixtures

`tools/check_equipment_slot_item_matrix.py --self-test` rejects:

- missing client equipment milestone;
- protocol mismatch (`758` instead of `763`);
- wrong remote entity;
- wrong slot;
- duplicate equipment update;
- wrong_entity_equipment_accepted;
- wrong_slot_equipment_accepted;
- wrong_item_equipment_accepted;
- duplicate_equipment_update_accepted;
- stale_equipment_update_accepted;
- missing non-claim/model text.

## Promotion gate

Only the receipt-backed `main_hand_remote_entity / slot4 / item id 829 / count 1 / non_empty_update` row is promoted. Any future equipment row must provide protocol-763 live receipt evidence, run log, BLAKE3 manifest, matrix/current-bundle entries, and positive/negative checker coverage before promotion.

## Decision

- Question: Can the equipment update observation receipt be promoted as a slot/item matrix row without claiming all equipment semantics?
- Inspected evidence: `docs/evidence/protocol-763-roi-02-equipment-update-observation-2026-05-27.receipt.json`, `docs/evidence/protocol-763-roi-02-equipment-update-observation-2026-05-27.run.log`, acceptance matrix, current bundle, and checker fixtures.
- Decision: Yes. Promote one bounded row and keep all equipment slots/items/permutations remain a non-claim.
- Owner: agent.
- Next action: add separate rows for armor slots, off-hand, empty transitions, repeated updates, and item classes only with live receipts and manifests.

## Non-claims

No all equipment slots, all item types, packet permutation matrix, empty/non-empty breadth, repeated updates, off-hand behavior, armor slot update breadth, enchantment/status effects, armor mitigation, exact vanilla combat parity, full inventory semantics, production load, full CTF correctness, or broad protocol coverage claim is made.
