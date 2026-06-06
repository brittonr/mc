# Protocol-763 broad coverage ledger — 2026-05-28

## Scope

This checkpoint drains the broad protocol-763 coverage Cairn by refreshing packet inventory from current Valence and Stevenarella sources, tightening the promotion checker, and keeping broad claims blocked.

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
| survival_break_place_pickup | Covered only by one bounded survival fixture row | Dedicated break/place/pickup receipt. |
| survival_reference_packet_acceptance | Four packet families promoted; remaining rows bounded/non-claim | Paper 1.20.1 reference rail plus Stevenarella parser fixtures cover command_tree_raw, chunk_delta_raw, recipe_book_raw, and custom_payload_brand. Other observed mappings stay scenario-bounded until parser fixtures exist. |
| reconnect_flag_state | Covered only by one bounded reconnect row | Reconnect flag-state receipt. |
| movement_packet_family | Covered only by one bounded movement packet-family row | `movement-packet-family` normalized evidence covers one `Full -> PlayerPositionLook` RED-portal transition; movement physics and all movement variants remain non-claims. |

## Uncovered protocol surfaces

| Surface | Status | Gap reason |
| --- | --- | --- |
| all_packets_all_states | Non-claim | `docs/evidence/protocol-763-packet-inventory-2026-05-28.tsv` lists 175 Valence protocol-763 packet rows, but most rows lack parser fixtures and live receipts. |
| all_entity_metadata_variants | Non-claim | Only scenario-observed metadata/decode paths are covered. |
| all_inventory_transactions | Non-claim | Inventory matrix keeps many transactions non-claims. |
| all_equipment_permutations | Non-claim | Equipment matrix covers one row only. |
| all_biomes_chunks_commands_recipes_advancements | Non-claim | Paper survival reference accepts representative command/recipe/chunk-related packet families, but no full feature semantics are promoted. |
| full_survival_compatibility | Non-claim | Only break/place/pickup parity is covered; crafting, furnace, chest, hunger, mob, redstone, biome, dimension, and persistence rows remain missing. |
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
| Survival break/place/pickup | `a88fe547bfe2dd43fff3ac5bd967f0ebf5a3c539403211dd029865293130090b` |

## Packet inventory

`docs/evidence/protocol-763-packet-inventory-2026-05-28.tsv` is generated from `valence/crates/valence_generated/extracted/packets.json` and `stevenarella/protocol/src/protocol/versions/v1_20_1.rs`. It records 175 Valence protocol-763 packet rows with state, side, wire id, Valence packet name, Stevenarella mapping status, parser-shape status, scenario evidence, owner, and next action.

Status vocabulary:

- fallback_alias_non_claim: row falls through inherited mapping and cannot support coverage promotion.
- reviewed_override_no_shape_claim: row has a Stevenarella 1.20.1 override but still needs parser-shape fixtures before broad promotion.
- shape_review_missing: parser shape has not been independently proven for broad coverage.
- scenario_bounded: maintained scenario evidence touches this packet family, but only for the named scenario surface.
- broad_covered: row has reviewed mapping, parser-shape fixture, and receipt/log evidence. Current rows using it are `CommandTreeS2CPacket`, `ChunkDeltaUpdateS2CPacket`, `SynchronizeRecipesS2CPacket`, and `CustomPayloadC2SPacket`.
- non_claim: no packet-family coverage claim is made.

## Mapping/parser fixture policy

`tools/check_protocol_coverage_ledger.py --self-test` models the required promotion gate for newly promoted packet families:

- positive fixture: reviewed mapping, reviewed parser shape, no fallback alias, no malformed shape acceptance, live receipt present;
- fallback_alias_rejected: inherited/fallback aliases cannot satisfy new packet-family coverage;
- malformed_shape_rejected: malformed payload/shape acceptance blocks promotion;
- missing owner, missing next action, missing parser-shape fixture, or missing live receipt blocks promotion.

Highest-risk parser-shape fixtures are real Stevenarella protocol tests in commit `ba3ce751f04b4fecefe516e06dff3e40363d2e72`: `protocol_763_high_risk_raw_parser_fixtures_accept_payloads`, `protocol_763_custom_payload_parser_fixture_accepts_brand_payload`, and `protocol_763_custom_payload_parser_fixture_rejects_malformed_channel`. Reviewable source snapshot: `docs/evidence/protocol-763-broad-parser-fixtures-stevenarella-2026-05-28.md`. Human oracle checkpoint: `docs/evidence/protocol-763-broad-parser-fixture-oracle-2026-05-28.md`. The checker validates that snapshot, verifies it matches `stevenarella/protocol/src/protocol/versions.rs`, and validates the oracle before deriving any `parser_shape_reviewed` row. These fixtures prove raw command/chunk-delta/recipe byte preservation and custom-payload positive plus malformed-channel rejection. Raw command/chunk-delta/recipe malformed semantic rejection is impossible for the current byte-opaque raw consumers and remains a non-claim until semantic decoders exist. They do not claim full command, chunk, recipe, or plugin-message semantics.

## Live scenario gate policy

A live scenario family can cover only the protocol surface named in its receipt and ledger row. Scenario receipts must name missing milestones, scenario family, protocol `763`, and reviewable evidence path. Unrelated packet families remain non-claims.

Representative live evidence for this refresh:

- Parser fixture source snapshot: `docs/evidence/protocol-763-broad-parser-fixtures-stevenarella-2026-05-28.md`.
- Parser fixture oracle checkpoint: `docs/evidence/protocol-763-broad-parser-fixture-oracle-2026-05-28.md`.
- Parser fixture test log: `docs/evidence/protocol-763-broad-coverage-ledger-gate-2026-05-28.run.log`.
- Paper reference receipt: `docs/evidence/protocol-763-survival-reference-paper-2026-05-28.receipt.json`.
- Paper client log: `docs/evidence/protocol-763-survival-reference-paper-2026-05-28.client.log`.
- Paper server log: `docs/evidence/protocol-763-survival-reference-paper-2026-05-28.server.log`.
- Pair manifest: `docs/evidence/protocol-763-survival-reference-pair-2026-05-28.b3`.

## Non-overclaiming gate

Broad protocol coverage, full protocol-763 compatibility, and full Minecraft compatibility stay blocked until every required ledger row has:

- mapping/parser fixtures;
- protocol-763 live receipt evidence;
- run logs and BLAKE3 manifests under `docs/evidence/`;
- acceptance matrix and current bundle entries;
- explicit owner and next action.

## Validation evidence

- Run log: `docs/evidence/protocol-763-broad-coverage-ledger-gate-2026-05-28.run.log`.
- BLAKE3 manifest: `docs/evidence/protocol-763-broad-coverage-ledger-gate-2026-05-28.b3`.
- Checker output: `protocol coverage ledger ok: 16 bounded seams, 175 packet rows, 4 broad packet rows, broad protocol claims blocked`.

## Decision

- Question: Can current evidence support broad protocol-763 compatibility?
- Inspected evidence: acceptance matrix, current bundle, all maintained seam digests, `docs/evidence/protocol-763-packet-inventory-2026-05-28.tsv`, row-specific checkers, Paper/Valence survival reference receipts, Stevenarella parser fixture source snapshot, and parser fixture oracle checkpoint.
- Decision: No. Current evidence supports bounded scenario seams and four narrow parser-fixture-backed packet rows only. Full protocol-763 compatibility remains a non-claim and full Minecraft compatibility remains a non-claim.
- Owner: agent.
- Next action: generate per-packet-family mapping/parser fixtures before adding broad coverage rows.

## Non-claims

No full protocol-763 compatibility, full Minecraft compatibility, full survival compatibility, vanilla parity, all packets, all play states, all entity metadata, all inventory transactions, all equipment permutations, all chunk/biome/command/recipe/advancement features, crafting/furnace/chest/mob/redstone coverage, exact vanilla combat parity, production readiness, public-server safety, or unbounded soak claim is made.
