# Entity status-effect live rail contract — 2026-06-07

## Contract

- Targeted row: `entity-status-effect-packets`.
- Target entity: `compatbot`.
- Effect: `minecraft:speed`.
- Amplifier: `1`.
- Duration: `200` ticks.
- Packet rows: `play/clientbound/0x6c EntityStatusEffectS2CPacket` and `play/clientbound/0x3f RemoveEntityStatusEffectS2CPacket`.
- Expected observations: client apply and remove observations plus one server correlation for the configured effect transition.
- Backend path: `status-effect-rail-missing` until a maintained owned-local status-effect rail exists.
- Client path: `stevenarella-effect-observation-candidate` until it emits maintained effect apply/remove telemetry.

## Current decision

The existing deterministic fixture remains valid for the targeted packet row, but the scenario capability registry records `entity-status-effect-packets` as `targeted-packet-live-blocker` with evidence mode `fixture-bounded-blocker`. The blocker reason is: combat rail does not apply and remove a bounded status effect.

## Non-claims

This change does not claim all effects, stacking, particles/UI, gameplay modifiers, combat balancing, survival parity, public-server safety, production readiness, broad Minecraft compatibility, or full protocol 763 compatibility.

## Owner / next action

Owner: local Cairn drain agent. Next action: leave matrix/current-bundle/packet-inventory status fixture-bounded, record blocker evidence, and require a future isolated status-effect receipt before live promotion.
