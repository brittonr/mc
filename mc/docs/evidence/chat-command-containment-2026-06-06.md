# Chat/command containment fixture row

## Scope

This promotes only the bounded `chat-command-containment` fixture row. The normalized fixture binds one harmless owned-local payload to `play/serverbound/0x05 ChatMessageC2SPacket` and `play/serverbound/0x04 CommandExecutionC2SPacket` with a no-secrets/no-public-addresses redaction policy.

## Artifacts

- Normalized KV: `docs/evidence/chat-command-containment-2026-06-06.kv`
- Receipt: `docs/evidence/chat-command-containment-2026-06-06.receipt.json`
- BLAKE3 manifest: `docs/evidence/chat-command-containment-2026-06-06.b3`
- Checker: `tools/check_targeted_packet_promotions.rs`
- Checker run log: `docs/evidence/targeted-packet-promotions-2026-06-06.run.log`

## Checker contract

The checker requires the row id, both packet rows, owned-local scope, harmless payload identity, server containment correlation, redaction policy, scenario-bounded packet inventory status, positive fixture coverage, negative fixture coverage, and explicit non-claims.

## Explicit non-claims

No chat-signing/security, all-command, command-permission, moderation, adversarial-resilience, public-server safety, full protocol-763 compatibility, broad Minecraft compatibility, or production-readiness claim is made.
