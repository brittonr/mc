# Resource-pack status fixture row

## Scope

This promotes only the bounded `resource-pack-status` fixture row. The normalized fixture binds one owned-local resource-pack offer/status exchange to `play/clientbound/0x40 ResourcePackSendS2CPacket` and `play/serverbound/0x24 ResourcePackStatusC2SPacket`, with status `declined`, `external_fetch=false`, and no-secrets/no-public-addresses redaction.

## Artifacts

- Normalized KV: `docs/evidence/resource-pack-status-2026-06-06.kv`
- Receipt: `docs/evidence/resource-pack-status-2026-06-06.receipt.json`
- BLAKE3 manifest: `docs/evidence/resource-pack-status-2026-06-06.b3`
- Checker: `tools/check_targeted_packet_promotions.rs`
- Checker run log: `docs/evidence/targeted-packet-promotions-2026-06-06.run.log`

## Checker contract

The checker requires the row id, packet rows, owned-local offer scope, configured status, no-external-fetch guarantee, redaction policy, server correlation, scenario-bounded packet inventory status, positive fixture coverage, negative fixture coverage, and explicit non-claims.

## Explicit non-claims

No asset download/application, trust/security validation, all resource-pack statuses, public-server safety, full protocol-763 compatibility, broad Minecraft compatibility, or production-readiness claim is made.
