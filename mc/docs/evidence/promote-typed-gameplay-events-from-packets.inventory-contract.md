# Typed packet-derived gameplay events evidence checkpoint

## Question

Which packet-derived gameplay semantic was selected for typed event promotion, and what contract bounds the adapter?

## Inspected evidence

- `servers/valence/crates/valence_server/src/action.rs` previously read raw `PacketEvent` values in `handle_player_action`, decoded `PlayerActionC2s`, updated `ActionSequence`, and emitted `DiggingEvent` for block-destroy actions.
- `servers/valence/crates/valence_server/README.md` now records the selected consumer inventory, previous malformed/stale behavior, and the `PlayerActionEvent` contract.
- Focused tests in `action.rs` cover valid emission plus wrong-id, partial-decode, malformed-payload, stale-client, and duplicate-digging rejection behavior.

## Decision

Promote only `PlayerActionC2s` in this change. The adapter emits `PlayerActionEvent` during `EventLoopPreUpdate` after raw packet ID, full-body decode, and live-client validation. Invalid or stale input emits no typed action and no downstream `DiggingEvent`. Raw `PacketEvent` remains public for low-level and unsupported packet access.

## Owner

Valence `valence_server` action plugin.

## Next action

Keep additional packet families out of this change unless a later Cairn change inventories and contracts them separately. Non-claims remain: no broad Minecraft compatibility, vanilla semantic equivalence, public-server safety, or production-readiness claim.
