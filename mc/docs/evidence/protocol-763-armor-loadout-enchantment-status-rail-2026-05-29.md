# Protocol-763 armor/loadout/enchantment/status rail evidence — 2026-05-29

## Scope

This rail adds an isolated `armor-loadout-enchantment-status-matrix` scenario path to the mc-compat runner. It reuses the existing bounded armor mitigation probe semantics but names the matrix rail separately so future rows can add evidence without broadening `armor-equipment-mitigation`.

The rail receipt is dry-run fixture evidence only. It does not promote a new live row and does not claim vanilla parity.

## Rail fixture

- Runner scenario: `armor-loadout-enchantment-status-matrix`.
- Receipt: `docs/evidence/protocol-763-armor-loadout-enchantment-status-rail-2026-05-29.receipt.json`.
- Normalized row record: `docs/evidence/protocol-763-armor-loadout-enchantment-status-rail-2026-05-29.record`.
- Run/check log: `docs/evidence/protocol-763-armor-loadout-enchantment-status-rail-2026-05-29.run.log`.

## Required row fields

| Field | Value |
| --- | --- |
| row id | `chest_diamond_none_none_melee` |
| loadout | `armor_loadout_chest_only` |
| equipment slots | `chest=DiamondChestplate` |
| enchantments | `enchantment_none` |
| status effects | `status_effect_none` |
| attack type | `melee` |
| health pre/post | `20000` → `18000` milli-health |
| base/final damage | `4000` → `2000` milli-damage |
| mitigation delta | `2000` milli-damage |
| tolerance | `1` milli-unit |
| reference required | `false` |
| promotion ready | `false` for this dry-run fixture |

## Isolation check

The runner exposes the new scenario name with the same required client/server milestone needles as the existing armor mitigation probe:

- client: `multi_client_count`, `protocol_detected`, `join_game`, `render_tick`, `team_red`, `team_blue`, `remote_player_spawn`, `armor_inventory_slot`, `combat_attack_sent`, `combat_health_update`;
- server: `server_client_a_seen`, `server_client_b_seen`, `server_equipment_state`, `server_combat_damage`, `server_armor_mitigation`.

The existing `armor-equipment-mitigation` scenario name, required milestones, and receipt behavior remain unchanged.

## Non-claims

This rail does not claim all armor permutations, all enchantments, all status effects, exact vanilla balancing, production readiness, full combat correctness, full CTF correctness, or a live row promotion.
