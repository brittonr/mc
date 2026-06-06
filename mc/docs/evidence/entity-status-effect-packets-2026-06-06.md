# Entity status-effect packet fixture row

## Scope

This promotes only the bounded `entity-status-effect-packets` fixture row. The normalized fixture binds `play/clientbound/0x6c EntityStatusEffectS2CPacket` and `play/clientbound/0x3f RemoveEntityStatusEffectS2CPacket` to one `compatbot` `minecraft:speed` apply/remove packet observation with amplifier `1` and duration `200` ticks.

## Artifacts

- Normalized KV: `docs/evidence/entity-status-effect-packets-2026-06-06.kv`
- Receipt: `docs/evidence/entity-status-effect-packets-2026-06-06.receipt.json`
- BLAKE3 manifest: `docs/evidence/entity-status-effect-packets-2026-06-06.b3`
- Checker: `tools/check_targeted_packet_promotions.rs`
- Checker run log: `docs/evidence/targeted-packet-promotions-2026-06-06.run.log`

## Checker contract

The checker requires the row id, packet rows, target/effect/amplifier/duration fields, client apply/remove observations, server correlation, scenario-bounded packet inventory status, positive fixture coverage, negative fixture coverage, and explicit non-claims.

## Explicit non-claims

No all-effect, stacking, particle/UI, gameplay-modifier, combat-balancing, survival-parity, full protocol-763 compatibility, broad Minecraft compatibility, public-server safety, or production-readiness claim is made.
