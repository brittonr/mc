# Protocol-763 ROI 01–03 drained receipts index

## Scope

This index makes the already-drained ROI 01–03 receipts discoverable from the maintained protocol-763 evidence matrix without broadening their claims. The source receipts remain the tracked JSON/log files and BLAKE3 manifests under `docs/evidence/`.

## Indexed receipts

| ROI | Seam | Maintained command | Reviewable receipt | BLAKE3 | Scoped claim | Explicit non-claims |
| --- | --- | --- | --- | --- | --- | --- |
| ROI 01 | Armor equipment mitigation | `nix run .#mc-compat-valence-ctf-armor-equipment-mitigation` | `docs/evidence/protocol-763-roi-01-armor-equipment-mitigation-2026-05-27.receipt.json` | `176fdf33d2b8b9047471f577a98f9093904a44ab8da2785baeb80acfc8d97765` | Two deterministic clients with client `armor_inventory_slot` plus Valence `server_equipment_state`, `server_combat_damage`, and `server_armor_mitigation` correlation. | No all armor loadouts, enchantments, status effects, exact vanilla mitigation parity, or full combat correctness. |
| ROI 02 | Equipment update observation | `nix run .#mc-compat-valence-ctf-equipment-update-observation` | `docs/evidence/protocol-763-roi-02-equipment-update-observation-2026-05-27.receipt.json` | `8100dde3ebb3476984235009e277d7e973037b7873b2fdb30c413093e1498d3d` | Two deterministic clients with remote-player spawn, client `entity_equipment_update`, and Valence `server_equipment_update_state` correlation. | No all equipment slots, all item types, packet permutation matrix, or armor damage mitigation proof. |
| ROI 03 | Projectile use/loadout rail | `nix run .#mc-compat-valence-ctf-projectile-hit` | `docs/evidence/protocol-763-roi-03-projectile-hit-2026-05-27.receipt.json` | `22310a0373f86bbff5e6bc116934d092b89f775cf5d539b08d04ff5564ad855b` | Two deterministic clients with protocol/join/render, red team, remote-player spawn, `projectile_use_sent`, `projectile_swing_sent`, and Valence `server_projectile_loadout` correlation. | No projectile travel, collision, damage attribution, all weapon variants, or full projectile physics correctness. |

## Supporting manifests

- `docs/evidence/protocol-763-roi-01-03-reviewable-receipts-2026-05-27.b3` covers ROI 01 and ROI 03 live/dry-run receipts plus the ROI 02 dry-run receipt.
- `docs/evidence/protocol-763-roi-02-equipment-update-observation-2026-05-27.b3` covers the ROI 02 live receipt, run log, regenerated dry-run receipt, and runner verification log.
- `docs/evidence/protocol-763-cairn-drain-review-checkpoint-2026-05-27.md` records the review checkpoint that promoted these receipts out of `target/` and into reviewable evidence.

## Oracle decision

- Question: Are ROI 01–03 drained receipts ready to be indexed as maintained bounded evidence rows?
- Inspected evidence: tracked reviewable receipt copies, BLAKE3 manifests above, archived ROI 01–03 task evidence, and maintained checker output from this index change.
- Decision: Yes, index the rows with narrow claims and explicit non-claims.
- Decision owner: agent; maintainer can request a fresh live rerun for any row before changing its hash.
- Next action: keep future receipt replacements paired with matrix, bundle, residual catalog, and manifest-checker updates.
