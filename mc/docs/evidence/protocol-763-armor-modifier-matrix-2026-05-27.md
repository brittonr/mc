# Protocol-763 armor/enchantment/status modifier matrix proof — 2026-05-27

## Scope

This checkpoint drains the armor, enchantment, and status modifier Cairn by defining the one modifier row currently promoted by live evidence and by keeping untested modifier breadth explicit.

It proves one bounded Valence CTF armor mitigation row: armor_loadout_chest_only with DiamondChestplate, enchantment_none, status_effect_none, melee attack, server mitigation calculation, and Stevenarella victim health observation. It does not claim all armor loadouts, enchantments, status effects, exact vanilla mitigation parity, or modifier stacking. All armor loadouts, enchantments, and status-effect modifiers remain a non-claim.

## Promoted matrix row

| Seam | armor loadout | material/item | enchantment representative | status-effect representative | attack type | expected evidence | Receipt | BLAKE3 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Armor equipment mitigation | armor_loadout_chest_only | DiamondChestplate | enchantment_none | status_effect_none | melee | `armor_inventory_slot`, `server_equipment_state`, `server_combat_damage`, `server_armor_mitigation`, `combat_health_update` | `docs/evidence/protocol-763-roi-01-armor-equipment-mitigation-2026-05-27.receipt.json` | `176fdf33d2b8b9047471f577a98f9093904a44ab8da2785baeb80acfc8d97765` |

## Fresh live corroboration

The matrix row is corroborated by a fresh live run:

- Receipt: `docs/evidence/protocol-763-armor-modifier-matrix-live-2026-05-27.receipt.json`.
- Run log: `docs/evidence/protocol-763-armor-modifier-matrix-live-2026-05-27.run.log`.
- Valence log: `docs/evidence/protocol-763-armor-modifier-matrix-live-2026-05-27.valence.log`.
- Client logs: `docs/evidence/protocol-763-armor-modifier-matrix-live-2026-05-27.client-compatbota.log`, `docs/evidence/protocol-763-armor-modifier-matrix-live-2026-05-27.client-compatbotb.log`.
- BLAKE3 manifest: `docs/evidence/protocol-763-armor-modifier-matrix-live-2026-05-27.b3`.

The checker validates that the Valence log contains `armor_equipment_state username=compatbotb slot=chest item=DiamondChestplate` and a `combat_armor_mitigation` calculation where final damage is lower than base damage and the health delta is internally consistent. It also validates that the victim client log contains the mitigated `update_health health=18.0` observation.

## Non-promoted matrix cells

| Modifier dimension | Status | Reason |
| --- | --- | --- |
| helmet/leggings/boots/full armor loadouts | Non-claim | No live receipt-backed row. |
| leather/iron/gold/netherite representative materials | Non-claim | Only DiamondChestplate is observed. |
| protection/projectile/fire/blast enchantments | Non-claim | enchantment_none only. |
| strength/resistance/weakness/status effects | Non-claim | status_effect_none only. |
| projectile/explosion/fire/fall attack types | Non-claim | melee only. |
| exact vanilla parity/tolerance | Non-claim | No vanilla reference oracle is included. |

## Positive validation

`tools/check_armor_modifier_matrix.py` requires:

- matrix receipt and fresh live receipt are `mode=run`, `dry_run=false`, `status=pass`;
- server protocol is `763`;
- scenario is `armor-equipment-mitigation`;
- client milestones include `armor_inventory_slot`, `combat_attack_sent`, and `combat_health_update`;
- server milestones include `server_equipment_state`, `server_combat_damage`, and `server_armor_mitigation`;
- run log records DiamondChestplate loadout and a coherent mitigation calculation;
- matrix/current-bundle digest entries exist;
- BLAKE3 manifest covers the fresh live receipt and run log.

## Negative fixtures

`tools/check_armor_modifier_matrix.py --self-test` rejects:

- missing armor inventory milestone;
- protocol mismatch (`758` instead of `763`);
- wrong loadout;
- missing mitigation attribution;
- mismatched health delta;
- wrong_loadout_accepted;
- missing_modifier_attribution_accepted;
- mismatched_health_delta_accepted;
- stale_equipment_state_accepted;
- vanilla_parity_claim_without_oracle;
- missing live manifest entries.

## Promotion gate

Only the `armor_loadout_chest_only / DiamondChestplate / enchantment_none / status_effect_none / melee` row is promoted. Future armor/enchantment/status rows must provide live protocol-763 receipts, run logs, BLAKE3 manifests, matrix/current-bundle updates, and negative checker fixtures before promotion.

## Decision

- Question: Can the existing armor mitigation row be promoted as a modifier matrix row without implying modifier breadth or vanilla parity?
- Inspected evidence: indexed armor mitigation receipt, fresh live receipt/log/manifest, acceptance matrix, current bundle, and checker fixtures.
- Decision: Yes. Promote one bounded row and keep all armor loadouts, enchantments, and status-effect modifiers remain a non-claim.
- Owner: agent.
- Next action: add separate rows for armor materials/loadouts, enchantments, status effects, and vanilla parity only with dedicated live evidence and a named oracle.

## Non-claims

No all armor loadouts, all armor materials, all armor slots, enchantment behavior, status-effect behavior, exact vanilla mitigation parity, modifier stacking, projectile/explosion/fire/fall attacks, production PvP readiness, full combat correctness, full CTF correctness, or broad protocol coverage claim is made.
