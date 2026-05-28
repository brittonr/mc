# Tasks

- [x] [serial] Define the armor/enchantment/status matrix. r[mc_compatibility.prove_armor_enchantment_status_matrix.modifier_matrix]
- [x] [serial] Add positive modifier scenarios. r[mc_compatibility.prove_armor_enchantment_status_matrix.positive_modifier_scenarios]
- [x] [serial] Add negative wrong-loadout or missing-modifier scenarios. r[mc_compatibility.prove_armor_enchantment_status_matrix.negative_modifier_scenarios]
- [x] [serial] Promote only receipt-backed modifier rows. r[mc_compatibility.prove_armor_enchantment_status_matrix.modifier_promotion_gate]

## Progress

- Armor/enchantment/status matrix is documented in `docs/evidence/protocol-763-armor-modifier-matrix-2026-05-27.md`.
- Positive validation promotes one protocol-763 live receipt-backed row: `armor_loadout_chest_only / DiamondChestplate / enchantment_none / status_effect_none / melee`.
- Negative fixtures in `tools/check_armor_modifier_matrix.py --self-test` reject missing armor evidence, protocol mismatch, wrong loadout, missing mitigation attribution, mismatched health delta, vanilla-parity overclaim, and missing live manifest entries.
- `tools/check_armor_modifier_matrix.py` promotes only receipt-backed modifier rows and keeps all armor loadouts, enchantments, status effects, and vanilla parity as non-claims.
