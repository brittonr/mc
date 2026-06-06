# Chunk biome data packet fixture row

## Scope

This promotes only the bounded `chunk-biome-data-packet` fixture row. The normalized fixture binds `play/clientbound/0x0d ChunkBiomeDataS2CPacket` to one protocol-763 byte-preservation fixture named `single-overworld-column-biome-palette`.

## Artifacts

- Normalized KV: `docs/evidence/chunk-biome-data-packet-2026-06-06.kv`
- Receipt: `docs/evidence/chunk-biome-data-packet-2026-06-06.receipt.json`
- BLAKE3 manifest: `docs/evidence/chunk-biome-data-packet-2026-06-06.b3`
- Checker: `tools/check_targeted_packet_promotions.rs`
- Checker run log: `docs/evidence/targeted-packet-promotions-2026-06-06.run.log`

## Checker contract

The checker requires the row id, packet row, protocol version, fixture identity, parser expectation, scenario-bounded packet inventory status, positive fixture coverage, negative fixture coverage, and explicit non-claims.

## Explicit non-claims

No all-biome-semantics, all-chunk-semantics, all-worldgen-packet, dimension-travel, Nether/End behavior, full protocol-763 compatibility, broad Minecraft compatibility, public-server safety, or production-readiness claim is made.
