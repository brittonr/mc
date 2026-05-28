# Protocol-763 broad coverage ledger — 2026-05-27

## Scope

This checkpoint drains the broad protocol-763 coverage Cairn by making current coverage reviewable and by keeping broad claims blocked.

full protocol-763 compatibility remains a non-claim.
full Minecraft compatibility remains a non-claim.

owner: agent
next_action: add packet-family rows only when mapping/parser fixtures and bounded live receipts exist.

## Covered protocol surfaces

| Surface | Status | Evidence basis |
| --- | --- | --- |
| status_login_play_join | Covered only by bounded scenario receipts | status/login/play join milestones across maintained rows. |
| ctf_score_path | Covered only by bounded CTF score receipts/oracle | RED/BLUE scoring soak historical oracle. |
| inventory_drop_pickup_click_container_block_place | Covered only by bounded inventory rows | Inventory/drop, pickup, click, open-container, block-place receipts. |
| combat_damage_knockback_armor_projectile | Covered only by bounded combat rows | Combat damage, knockback, armor, equipment, projectile use/loadout, projectile damage attribution. |
| reconnect_flag_state | Covered only by one bounded reconnect row | Reconnect flag-state receipt. |

## Uncovered protocol surfaces

| Surface | Status | Gap reason |
| --- | --- | --- |
| all_packets_all_states | Non-claim | No exhaustive packet/state ledger or receipts. |
| all_entity_metadata_variants | Non-claim | Only scenario-observed metadata/decode paths are covered. |
| all_inventory_transactions | Non-claim | Inventory matrix keeps many transactions non-claims. |
| all_equipment_permutations | Non-claim | Equipment matrix covers one row only. |
| all_biomes_chunks_commands_recipes_advancements | Non-claim | No receipt-backed coverage rows. |
| all_vanilla_combat_parity | Non-claim | No paired reference oracle receipts. |

## Maintained evidence seams

| Seam | BLAKE3 |
| --- | --- |
| RED/BLUE scoring soak | `b7c861f27ef7ceaf94705a74a5459d3f9df625dada4b14d8715ba8e9c5d921de` |
| Inventory/drop | `4aeb08172b35edd03d57169c63a4942ca149c783fbc51539702922ac246a0e46` |
| Block placement / use-item-on-block | `9feec3b967b3fd5cb011139eda524c32c73123323823b3ebef7bd93062e1c122` |
| Pickup semantics | `bcac4aab63857cf0d3b6dd148455324e7f0368dd3e57cfd26841ae7fc1b5ffe8` |
| Player-inventory click/container click | `c75381feed1d98cd33d584ab9b8efdfe849d85eb3d1bb6cc23a23578cc8d7f7d` |
| Open-container semantics | `b7913ddd1f000981f411f7f14331b67820761c1d317c528fbf8a5070c139d3f3` |
| Two-client combat/damage | `b67962dd5d4fe7242b69fd7c879390e80e13528475d55d7feb5305289f762ac8` |
| Flag-carrier death/return | `d4202d7f04245dd385f16f9a174b84fa59a837fd75a8f9ba7db3cc7adaf692a4` |
| Reconnect flag-state | `4d848af56b25ad4b3c466863bac5b2052adbbc1c59e2b2164bfb2a696c225cb3` |
| Latency/jitter tolerance | `a4a407fb1ac3aceae06faeacb794891ff8411c8ac86470c651c89b37b6c7f33d` |
| Combat knockback | `a5d0ba5ea6155a99b58f245a03195da05b4925d7bd151b5b3f67503ae7a4cf09` |
| Armor equipment mitigation | `176fdf33d2b8b9047471f577a98f9093904a44ab8da2785baeb80acfc8d97765` |
| Equipment update observation | `8100dde3ebb3476984235009e277d7e973037b7873b2fdb30c413093e1498d3d` |
| Projectile use/loadout rail | `22310a0373f86bbff5e6bc116934d092b89f775cf5d539b08d04ff5564ad855b` |
| Projectile damage attribution | `cf84fcb81ae557ecfbd2ff0b1f8b94af7bf07eaa85c20b1cde442929e3e3e529` |

## Mapping/parser fixture policy

`tools/check_protocol_coverage_ledger.py --self-test` models the required promotion gate for newly promoted packet families:

- positive fixture: reviewed mapping, reviewed parser shape, no fallback alias, no malformed shape acceptance, live receipt present;
- fallback_alias_rejected: inherited/fallback aliases cannot satisfy new packet-family coverage;
- malformed_shape_rejected: malformed payload/shape acceptance blocks promotion;
- missing live receipt blocks promotion.

## Live scenario gate policy

A live scenario family can cover only the protocol surface named in its receipt and ledger row. Scenario receipts must name missing milestones, scenario family, protocol `763`, and reviewable evidence path. Unrelated packet families remain non-claims.

## Non-overclaiming gate

Broad protocol coverage, full protocol-763 compatibility, and full Minecraft compatibility stay blocked until every required ledger row has:

- mapping/parser fixtures;
- protocol-763 live receipt evidence;
- run logs and BLAKE3 manifests under `docs/evidence/`;
- acceptance matrix and current bundle entries;
- explicit owner and next action.

## Decision

- Question: Can current evidence support broad protocol-763 compatibility?
- Inspected evidence: acceptance matrix, current bundle, all maintained seam digests, row-specific checkers, and coverage ledger fixtures.
- Decision: No. Current evidence supports bounded scenario seams only. Full protocol-763 compatibility remains a non-claim and full Minecraft compatibility remains a non-claim.
- Owner: agent.
- Next action: generate per-packet-family mapping/parser fixtures before adding broad coverage rows.

## Non-claims

No full protocol-763 compatibility, full Minecraft compatibility, all packets, all play states, all entity metadata, all inventory transactions, all equipment permutations, all chunk/biome/command/recipe/advancement features, exact vanilla combat parity, production readiness, public-server safety, or unbounded soak claim is made.
