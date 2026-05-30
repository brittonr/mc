# Protocol-763 equipment slot/item expansion rail evidence — 2026-05-29

## Scope

This rail adds an isolated `equipment-slot-item-matrix-expansion` scenario path to the mc-compat runner. It reuses the existing bounded equipment-update observer probe semantics but names the expansion rail separately so future slot/item rows can add evidence without broadening `equipment-update-observation`.

The rail receipt is dry-run fixture evidence only. It does not promote a new live row and does not claim all equipment semantics or vanilla/reference parity.

## Rail fixture

- Runner scenario: `equipment-slot-item-matrix-expansion`.
- Receipt: `docs/evidence/protocol-763-equipment-slot-item-expansion-rail-2026-05-29.receipt.json`.
- Normalized row record: `docs/evidence/protocol-763-equipment-slot-item-expansion-rail-2026-05-29.record`.
- Run/check log: `docs/evidence/protocol-763-equipment-slot-item-expansion-rail-2026-05-29.run.log`.

## Required row fields

| Field | Value |
| --- | --- |
| row id | `remote_main_hand_slot4_item829_count1_non_empty` |
| actor | `compatbotb` |
| observer | `compatbota` |
| remote entity id | `4` |
| semantic slot | `main_hand_remote_entity` |
| wire slot | `slot4` |
| item id | `829` |
| item count | `1` |
| transition | `non_empty_update` |
| update order | `after_remote_spawn` |
| reference required | `false` |
| promotion ready | `false` for this dry-run fixture |

## Isolation check

The runner exposes the new scenario name with the same required client/server milestone needles as the existing equipment update probe:

- client: `multi_client_count`, `protocol_detected`, `join_game`, `render_tick`, `team_red`, `team_blue`, `remote_player_spawn`, `entity_equipment_update`;
- server: `server_client_a_seen`, `server_client_b_seen`, `server_equipment_update_state`.

The existing `equipment-update-observation` scenario name, required milestones, and receipt behavior remain unchanged.

## Non-claims

This rail does not claim all equipment slots, all item types, all transition/order permutations, equipment packet permutations, armor mitigation, enchantment/status effects, production readiness, full equipment semantics, full inventory semantics, full CTF correctness, broad protocol coverage, or a live row promotion.
