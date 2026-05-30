# Protocol-763 armor/loadout/enchantment/status row evidence — 2026-05-29

## Scope

This evidence binds the existing live Valence armor mitigation row to the new bounded `armor-loadout-enchantment-status-matrix` contract.

Promoted row: `chest_diamond_none_none_melee` / `armor_loadout_chest_only` / `chest=DiamondChestplate` / `enchantment_none` / `status_effect_none` / `melee`.

## Evidence set

- Existing live row receipt: `docs/evidence/protocol-763-armor-modifier-matrix-live-2026-05-27.receipt.json`.
- Existing live receipt BLAKE3: `3152241bbbca379405a3806987f0b4dc8e4706b291cecebc1f509d0f96914f07`.
- Existing live row run log: `docs/evidence/protocol-763-armor-modifier-matrix-live-2026-05-27.run.log`.
- Existing live Valence log: `docs/evidence/protocol-763-armor-modifier-matrix-live-2026-05-27.valence.log`.
- Existing live client logs: `docs/evidence/protocol-763-armor-modifier-matrix-live-2026-05-27.client-compatbota.log`, `docs/evidence/protocol-763-armor-modifier-matrix-live-2026-05-27.client-compatbotb.log`.
- Existing live manifest: `docs/evidence/protocol-763-armor-modifier-matrix-live-2026-05-27.b3`.
- New normalized row record: `docs/evidence/protocol-763-armor-loadout-enchantment-status-row-2026-05-29.record`.
- New rail fixture receipt: `docs/evidence/protocol-763-armor-loadout-enchantment-status-rail-2026-05-29.receipt.json`.
- New row/rail checker output: `docs/evidence/protocol-763-armor-loadout-enchantment-status-row-2026-05-29.run.log` and `docs/evidence/protocol-763-armor-loadout-enchantment-status-rail-2026-05-29.run.log`.

## Checked facts

The row checker records these normalized values:

| Field | Value |
| --- | --- |
| row id | `chest_diamond_none_none_melee` |
| loadout | `armor_loadout_chest_only` |
| equipment | `chest=DiamondChestplate` |
| enchantments | `enchantment_none` |
| status effects | `status_effect_none` |
| attack type | `melee` |
| health pre/post | `20000` → `18000` milli-health |
| base/final damage | `4000` → `2000` milli-damage |
| mitigation delta | `2000` milli-damage |
| tolerance | `1` milli-unit |
| vanilla reference required | `false` |

`tools/check_armor_modifier_matrix.py` still validates the live receipt/log/client evidence for this same row. `tools/check_armor_loadout_enchantment_status.rs` validates the normalized matrix record and rejects missing fields, equipment evidence gaps, bad deltas, absent tolerance, unpaired parity, and overclaims.

## Decision

This row may be promoted only as bounded Valence-only containment for the exact row above. It remains non-reference evidence and does not claim vanilla parity.

## Non-claims

No all armor loadouts, all armor materials, all armor slots, enchantment behavior beyond `enchantment_none`, status-effect behavior beyond `status_effect_none`, exact vanilla mitigation parity, modifier stacking, projectile/explosion/fire/fall attacks, production PvP readiness, full combat correctness, full CTF correctness, or broad protocol coverage claim is made.
