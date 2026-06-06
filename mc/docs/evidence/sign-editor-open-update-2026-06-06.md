# Sign editor open/update fixture row

## Scope

This promotes only the bounded `sign-editor-open-update` fixture row. The normalized fixture binds `play/clientbound/0x31 SignEditorOpenS2CPacket` and `play/serverbound/0x2e UpdateSignC2SPacket` to one `compatbot` sign edit at `28,64,0` with payload `MC|Compat|Sign|Edit`.

## Artifacts

- Normalized KV: `docs/evidence/sign-editor-open-update-2026-06-06.kv`
- Receipt: `docs/evidence/sign-editor-open-update-2026-06-06.receipt.json`
- BLAKE3 manifest: `docs/evidence/sign-editor-open-update-2026-06-06.b3`
- Checker: `tools/check_targeted_packet_promotions.rs`
- Checker run log: `docs/evidence/targeted-packet-promotions-2026-06-06.run.log`

## Checker contract

The checker requires the row id, packet rows, actor, sign position, initial state, submitted payload, client open/update metrics, server acceptance metric, scenario-bounded packet inventory status, positive fixture coverage, negative fixture coverage, and explicit non-claims.

## Explicit non-claims

No sign editing UI behavior, all sign variants, all text formats, arbitrary NBT semantics, all block entities, public-server safety, full protocol-763 compatibility, broad Minecraft compatibility, or production-readiness claim is made.
