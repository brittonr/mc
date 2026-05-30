# Protocol-763 equipment slot/item expansion contract — 2026-05-29

## Scope

This contract defines the bounded `equipment-slot-item-matrix-expansion` row before runner, fixture, parser, or matrix promotion work starts.

Covered claim, once evidence exists: a finite list of explicitly named protocol-763 Valence CTF equipment observer rows records actor identity, observer identity, semantic slot, wire/protocol slot key, item id, item count, update order, remote entity id, client milestone, server milestone, and non-claim labels.

Current landed evidence still covers only `main_hand_remote_entity / slot4 / item id 829 / count 1 / non_empty_update`. This contract does not promote new rows by itself.

## Candidate matrix rows

| Row id | Actor | Observer | Semantic slot | Wire slot | Item representative | Count | Transition | Promotion label |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `remote_main_hand_slot4_item829_count1_non_empty` | `compatbotb` | `compatbota` | `main_hand_remote_entity` | `slot4` | `item_id_829` | `1` | `non_empty_update` | existing bounded Valence-only containment |
| `remote_chest_slot6_diamond_chestplate_count1_non_empty` | `compatbotb` | `compatbota` | `chest_remote_entity` | `slot6` | `DiamondChestplate` | `1` | `non_empty_update` | future live row |
| `remote_offhand_slot5_shield_count1_non_empty` | `compatbotb` | `compatbota` | `offhand_remote_entity` | `slot5` | `Shield` | `1` | `non_empty_update` | future live row |
| `remote_main_hand_slot4_empty_count0_clear` | `compatbotb` | `compatbota` | `main_hand_remote_entity` | `slot4` | `item_empty` | `0` | `non_empty_to_empty_update` | future live row |

## Required normalized metrics

| Metric | Meaning |
| --- | --- |
| `row.id` | Stable row id from the contract table. |
| `row.promotion_label` | Existing bounded containment or future live row. |
| `actor.username` | Player whose equipment changed. |
| `observer.username` | Remote client that observed the equipment update. |
| `entity.remote_id` | Remote entity id observed by the client. |
| `slot.semantic` | Human-readable slot representative. |
| `slot.wire` | Protocol/client slot key observed in the packet/update. |
| `item.id` | Numeric item id or stable item label for the representative. |
| `item.count` | Observed stack count. |
| `transition.kind` | Non-empty update, clear update, repeated update, or other named transition. |
| `update.order` | Explicit ordering relation such as `after_remote_spawn`. |
| `client.milestone.remote_spawn` | Client evidence that binds the remote entity id before equipment observation. |
| `client.milestone.entity_equipment_update` | Client evidence for the equipment update row. |
| `server.milestone.equipment_update_state` | Server evidence for actor/slot/item state. |
| `valence.receipt` | Valence live receipt path for promoted rows. |
| `reference.required` | `true` only for a row claiming vanilla/reference parity. |
| `reference.receipt` | Reference receipt path when `reference.required=true`; otherwise `none`. |

## Checker contract

A row checker must pass positive fixtures and reject:

- `missing_equipment_row_field` for any absent required metric;
- `wrong_slot_mapping` when semantic and wire slot do not match the row contract;
- `missing_observer_update` when the client lacks remote spawn or equipment-update evidence;
- `item_count_mismatch` when item id or count differs from the configured row;
- `stale_entity_id` when the equipment update does not bind to the latest remote spawn entity id;
- `duplicate_update_order` when a single-update row contains repeated or out-of-order updates;
- `unpaired_equipment_reference` when a reference/parity row lacks paired reference evidence;
- `all_equipment_overclaim` when docs claim all slots, all item types, all packet permutations, armor mitigation, enchantment/status effects, production readiness, or full equipment semantics outside listed rows.

## Promotion requirements

A future row may be promoted only when it has:

- a live protocol-763 Valence receipt;
- remote observer client log evidence for remote spawn and equipment update;
- server log evidence for actor/slot/item state;
- a BLAKE3 manifest over receipt, logs, checker output, and docs;
- positive and negative checker fixture output;
- current bundle and acceptance matrix text that names only the promoted row;
- paired reference evidence before any vanilla/reference parity label.

## Non-claims

This contract does not claim all equipment slots, all item types, all item counts, all transition/order permutations, all equipment packets, armor mitigation, enchantment/status effects, exact vanilla/reference parity, production readiness, full combat correctness, full inventory semantics, full CTF correctness, or broad protocol coverage.
