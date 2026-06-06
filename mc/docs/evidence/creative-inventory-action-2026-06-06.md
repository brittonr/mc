# Creative inventory action fixture row

## Scope

This promotes only the bounded `creative-inventory-action` fixture row. The normalized fixture binds `play/serverbound/0x2b CreativeInventoryActionC2SPacket` to one creative-mode precondition and one `compatbot` hotbar mutation for `minecraft:stone` count `64` in semantic slot `hotbar_0` / wire slot `36`.

## Artifacts

- Normalized KV: `docs/evidence/creative-inventory-action-2026-06-06.kv`
- Receipt: `docs/evidence/creative-inventory-action-2026-06-06.receipt.json`
- BLAKE3 manifest: `docs/evidence/creative-inventory-action-2026-06-06.b3`
- Checker: `tools/check_targeted_packet_promotions.rs`
- Checker run log: `docs/evidence/targeted-packet-promotions-2026-06-06.run.log`

## Checker contract

The checker requires the row id, packet row, creative-mode precondition, actor, slot/item/count fields, server acceptance metric, final slot-state metric, scenario-bounded packet inventory status, positive fixture coverage, negative fixture coverage, and explicit non-claims.

## Explicit non-claims

No all-creative-inventory-semantics, all-slot, all-item, game-mode-transition, pick-block, public-server safety, full protocol-763 compatibility, broad Minecraft compatibility, or production-readiness claim is made.
