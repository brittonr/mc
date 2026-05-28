# Tasks

- [x] [serial] Define the equipment slot/item matrix. r[mc_compatibility.prove_equipment_slot_item_matrix.equipment_matrix]
- [x] [serial] Add positive equipment observation scenarios. r[mc_compatibility.prove_equipment_slot_item_matrix.positive_equipment_scenarios]
- [x] [serial] Add negative stale/mismatched equipment scenarios. r[mc_compatibility.prove_equipment_slot_item_matrix.negative_equipment_scenarios]
- [x] [serial] Promote only receipt-backed equipment rows. r[mc_compatibility.prove_equipment_slot_item_matrix.equipment_promotion_gate]

## Progress

- Equipment slot/item matrix is documented in `docs/evidence/protocol-763-equipment-slot-item-matrix-2026-05-27.md`.
- Positive validation promotes one protocol-763 live receipt-backed row: `main_hand_remote_entity / slot4 / item id 829 / count 1 / non_empty_update`.
- Negative fixtures in `tools/check_equipment_slot_item_matrix.py --self-test` reject missing milestones, protocol mismatch, wrong entity, wrong slot, duplicate update, mismatched equipment markers, and missing non-claim/model text.
- `tools/check_equipment_slot_item_matrix.py` promotes only receipt-backed equipment rows and keeps all equipment slots/items/permutations as non-claims.
