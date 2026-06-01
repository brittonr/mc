# Protocol-763 chunk/biome packet-family row — 2026-06-01

## Scope

This checkpoint promotes only a narrow `chunk/biome packet family` row for one already-reviewed protocol-763 raw packet mapping plus one live environment receipt pair:

- `play/clientbound/0x43 ChunkDeltaUpdateS2CPacket -> ChunkDeltaUpdateRaw`
- live environment evidence from paired Paper and Valence `survival-biome-dimension-state` receipts.

`ChunkBiomeDataS2CPacket`, all biome payload semantics, all chunk semantics, dimension travel, Nether/End behavior, and world persistence remain non-claims.

## Evidence basis

| Evidence | Path | BLAKE3 / status |
| --- | --- | --- |
| Row receipt | `docs/evidence/protocol-chunk-biome-family-2026-06-01.receipt.json` | `d0eafb817292c04fbab78b019c74568b4cd4bcd6eb32e4f718b8f5d3909faaf1` |
| Row contract kv | `docs/evidence/protocol-chunk-biome-family-2026-06-01.kv` | in this row manifest |
| Parser ledger | `docs/evidence/protocol-763-broad-coverage-ledger-2026-05-28.md` | `fa6e946063ea721d447cefbd6867ee934c99b253ee27cfec742e460728c2e3cb` |
| Packet inventory | `docs/evidence/protocol-763-packet-inventory-2026-05-28.tsv` | `20de60f18fe7484e5ad393e07ce7ad8814e6f0150cda12a0440d8be9d55ecbf6` |
| Parser fixture snapshot | `docs/evidence/protocol-763-broad-parser-fixtures-stevenarella-2026-05-28.md` | `d4e47d929a9f0a8bba15abe5c72160c1cc64a2dcb1ee517957ada7399d542266` |
| Parser oracle | `docs/evidence/protocol-763-broad-parser-fixture-oracle-2026-05-28.md` | `fc92fc5a65cee89b7de66616822671a2bd1a9fac51a624e7f4977c19be7fcc51` |
| Parser gate log | `docs/evidence/protocol-763-broad-coverage-ledger-gate-2026-05-28.run.log` | `aa712d34bdd843831918f9aa38fad0253b0a82862b873ae64e7773f3f1513475` |
| Survival biome/dimension evidence doc | `docs/evidence/survival-biome-dimension-receipts-2026-06-01.md` | `7dae1beb78bfa36e812b46528d59254869addf6f32b01cce4ad3bb6f7d53e181` |
| Paper biome/dimension receipt | `docs/evidence/survival-biome-dimension-paper-2026-06-01.receipt.json` | `4c8f59af896af4c3c6f0733a5069350441fc95f02fb8282eaac4097a906c7207` |
| Valence biome/dimension receipt | `docs/evidence/survival-biome-dimension-valence-2026-06-01.receipt.json` | `a7b534142458f0178d02215e0d6c53542ceca807970524800a96178f3dbb3928` |

## Normalized metrics

| Metric | Value |
| --- | --- |
| `packet_name` | `ChunkDeltaUpdateS2CPacket` |
| `wire_id` | `play/clientbound/0x43` |
| `chunk_position` | parser fixture raw chunk-delta payload plus live join environment only |
| `biome_id_or_environment_id` | `minecraft:overworld` |
| `parser_fixture_id` | `protocol_763_high_risk_raw_parser_fixtures_accept_payloads` |
| `live_receipt_path` | `docs/evidence/protocol-chunk-biome-family-2026-06-01.receipt.json` |
| `malformed_fixture_status_where_supported` | semantic malformed rejection remains a non-claim for byte-opaque raw consumer |

## Decision

The row can be promoted as a bounded protocol-family slice because the packet inventory already marks `ChunkDeltaUpdateS2CPacket` as `broad_covered`, the parser snapshot/oracle prove raw byte preservation for `ChunkDeltaUpdateRaw`, and the survival biome/dimension receipts provide a live Paper/Valence environment path with matching `minecraft:overworld` normalized identifiers.

This does not prove `ChunkBiomeDataS2CPacket`, chunk-delta semantics, biome lookup semantics, all worldgen packets, dimension travel, Nether/End behavior, world persistence, full protocol-763 compatibility, full Minecraft compatibility, or production readiness.

## Non-claims

No all-worldgen-packets, all-chunks, all-biomes, chunk-delta semantics, chunk-biome-data packet coverage, biome lookup semantics, dimension travel, Nether/End behavior, world persistence, full protocol-763 compatibility, full Minecraft compatibility, full survival compatibility, broad vanilla parity, or production readiness claim is made.
