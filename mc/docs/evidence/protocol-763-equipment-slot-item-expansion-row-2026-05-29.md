# Protocol-763 equipment slot/item expansion row evidence — 2026-05-29

## Scope

This evidence binds the existing live Valence equipment update row to the new bounded `equipment-slot-item-matrix-expansion` contract.

Promoted row: `remote_main_hand_slot4_item829_count1_non_empty` / actor `compatbotb` / observer `compatbota` / remote entity `4` / semantic slot `main_hand_remote_entity` / wire slot `slot4` / item id `829` / count `1` / transition `non_empty_update` / order `after_remote_spawn`.

## Evidence set

- Existing live row receipt: `docs/evidence/protocol-763-roi-02-equipment-update-observation-2026-05-27.receipt.json`.
- Existing live receipt BLAKE3: `8100dde3ebb3476984235009e277d7e973037b7873b2fdb30c413093e1498d3d`.
- Existing live run log: `docs/evidence/protocol-763-roi-02-equipment-update-observation-2026-05-27.run.log`.
- Existing row proof: `docs/evidence/protocol-763-equipment-slot-item-matrix-2026-05-27.md`.
- New normalized row record: `docs/evidence/protocol-763-equipment-slot-item-expansion-row-2026-05-29.record`.
- New rail fixture receipt: `docs/evidence/protocol-763-equipment-slot-item-expansion-rail-2026-05-29.receipt.json`.
- New row/rail checker output: `docs/evidence/protocol-763-equipment-slot-item-expansion-row-2026-05-29.run.log` and `docs/evidence/protocol-763-equipment-slot-item-expansion-rail-2026-05-29.run.log`.

## Checked facts

The row checker records these normalized values:

| Field | Value |
| --- | --- |
| row id | `remote_main_hand_slot4_item829_count1_non_empty` |
| actor | `compatbotb` |
| observer | `compatbota` |
| remote entity id | `4` |
| semantic slot | `main_hand_remote_entity` |
| wire slot | `slot4` |
| item id/count | `829` / `1` |
| transition/order | `non_empty_update` / `after_remote_spawn` |
| vanilla/reference required | `false` |

`tools/check_equipment_slot_item_matrix.py` still validates the live receipt/log evidence for this same row. `tools/check_equipment_slot_item_expansion.rs` validates the normalized matrix record and rejects missing fields, wrong slot mapping, missing observer update, item/count mismatch, stale entity id, duplicate update ordering, unpaired reference, and overclaims.

## Decision

This row may be promoted only as bounded Valence-only containment for the exact row above. It remains non-reference evidence and does not claim vanilla parity, all equipment slots/items, or packet permutation coverage.

## Non-claims

No all equipment slots, all item types, all item counts, all transition/order permutations, equipment packet permutations, armor mitigation, enchantment/status effects, exact vanilla/reference parity, production readiness, full equipment semantics, full inventory semantics, full combat correctness, full CTF correctness, or broad protocol coverage claim is made.
