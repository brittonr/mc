# Block-entity update breadth fixture row

## Scope

This promotes only the bounded `block-entity-update-breadth` fixture row. The normalized fixture binds `play/clientbound/0x08 BlockEntityUpdateS2CPacket` to one non-sign `Chest` payload at `32,64,0` with payload identity `items=1;custom_name=Compat Chest`.

## Artifacts

- Normalized KV: `docs/evidence/block-entity-update-breadth-2026-06-06.kv`
- Receipt: `docs/evidence/block-entity-update-breadth-2026-06-06.receipt.json`
- BLAKE3 manifest: `docs/evidence/block-entity-update-breadth-2026-06-06.b3`
- Checker: `tools/check_targeted_packet_promotions.rs`
- Checker run log: `docs/evidence/targeted-packet-promotions-2026-06-06.run.log`

## Checker contract

The checker requires the row id, packet row, fixture payload identity, backend evidence marker, scenario-bounded packet inventory status, positive fixture coverage, negative fixture coverage, and explicit non-claims.

## Explicit non-claims

No all-block-entity, arbitrary NBT parity, persistence breadth, sign editing, full protocol-763 compatibility, broad Minecraft compatibility, public-server safety, or production-readiness claim is made.
