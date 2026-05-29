# Protocol-763 armor/loadout/enchantment/status matrix contract — 2026-05-29

## Scope

This contract defines the bounded `armor-loadout-enchantment-status-matrix` row before any runner, fixture, parser, or matrix promotion work starts.

Covered claim, once live evidence exists: one configured protocol-763 Valence CTF armor modifier matrix evaluates a finite list of explicitly named rows. Each row records the victim loadout, equipment slots, enchantment representatives, status-effect representatives, attack type, expected pre/post health, damage delta, mitigation delta, tolerance, and non-claim labels.

Current landed evidence still covers only `armor_loadout_chest_only / DiamondChestplate / enchantment_none / status_effect_none / melee`. This contract does not promote new rows by itself.

## Candidate matrix rows

| Row id | Loadout | Equipment slots | Enchantments | Status effects | Attack type | Promotion label |
| --- | --- | --- | --- | --- | --- | --- |
| `chest_diamond_none_none_melee` | `armor_loadout_chest_only` | `chest=DiamondChestplate` | `enchantment_none` | `status_effect_none` | `melee` | existing bounded Valence-only containment |
| `full_diamond_none_none_melee` | `armor_loadout_full_diamond` | `head=DiamondHelmet; chest=DiamondChestplate; legs=DiamondLeggings; feet=DiamondBoots` | `enchantment_none` | `status_effect_none` | `melee` | future live row |
| `chest_diamond_protection_i_none_melee` | `armor_loadout_chest_only` | `chest=DiamondChestplate` | `protection_i` | `status_effect_none` | `melee` | future live row |
| `chest_diamond_none_resistance_i_melee` | `armor_loadout_chest_only` | `chest=DiamondChestplate` | `enchantment_none` | `resistance_i` | `melee` | future live row |

## Required normalized metrics

| Metric | Meaning |
| --- | --- |
| `row.id` | Stable matrix row id from the contract table. |
| `row.promotion_label` | Either existing bounded Valence-only containment or future live row. |
| `victim.loadout_id` | Victim armor loadout identifier. |
| `victim.equipment_slots` | Complete slot→item list for the row. |
| `victim.enchantments` | Complete enchantment id/level list or `enchantment_none`. |
| `victim.status_effects` | Complete status-effect id/amplifier list or `status_effect_none`. |
| `attack.type` | Attack representative, initially `melee`. |
| `health.pre` | Victim health before damage. |
| `health.post` | Victim health after damage. |
| `damage.base` | Damage before armor/enchantment/status modifiers. |
| `damage.final` | Damage after armor/enchantment/status modifiers. |
| `mitigation.delta` | Difference between base and final damage. |
| `tolerance.absolute` | Absolute comparison tolerance for damage and health deltas. |
| `reference.required` | `true` only for any row claiming vanilla parity. |
| `reference.receipt` | Reference receipt path when `reference.required=true`; otherwise `none`. |
| `valence.receipt` | Valence live receipt path for promoted rows. |
| `server.milestone.equipment_state` | Server observed victim equipment state for row. |
| `server.milestone.armor_mitigation` | Server observed armor/enchantment/status mitigation calculation. |
| `client.milestone.health_update` | Victim client observed health update after attack. |

## Checker contract

A row checker must pass positive fixtures and reject:

- `missing_matrix_row_field` for any absent required metric;
- `missing_equipment_evidence` when server/client logs do not record the row equipment state;
- `mismatched_damage_delta` when `health.pre - damage.final != health.post` within tolerance;
- `absent_tolerance` when the row lacks an explicit tolerance;
- `unpaired_vanilla_parity` when a vanilla-parity row lacks paired reference evidence;
- `all_loadout_overclaim` when docs claim all armor permutations, all enchantments, all status effects, or exact vanilla balancing outside listed rows.

## Promotion requirements

A future row may be promoted only when it has:

- a live protocol-763 Valence receipt;
- server log evidence for equipment state and modifier calculation;
- victim client health-update evidence;
- a BLAKE3 manifest over receipt, logs, checker output, and docs;
- positive and negative checker fixture output;
- current bundle and acceptance matrix text that names only the promoted row;
- paired reference evidence before any vanilla parity label.

## Non-claims

This contract does not claim all armor permutations, all armor slots, all item materials, all enchantments, all enchantment levels, all status effects, modifier stacking, projectile/explosion/fire/fall attacks, exact vanilla mitigation parity, production PvP readiness, full combat correctness, full CTF correctness, or broad protocol coverage.
