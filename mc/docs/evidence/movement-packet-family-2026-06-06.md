# Movement packet-family evidence (2026-06-06)

## Scope

This row promotes one bounded protocol-763 movement packet-family slice:

- Scenario: `ctf-spawn-team-balance-reset`
- Actor: `compatbota`
- Packet row: `play/serverbound/0x15 Full -> PlayerPositionLook`
- Movement target: `x=-4.0 y=84.0 z=4.0 yaw=0.0 pitch=0.0 on_ground=true`
- Maintained wrapper: `nix run .#mc-compat-valence-movement-packet-family`
- Correlation: Valence assigns `compatbota` to RED after the client enters the RED portal in the same bounded scenario.

The row does not promote `PositionAndOnGround`, `LookAndOnGround`, `OnGroundOnly`, vehicle movement, movement physics, collision, anti-cheat, latency tolerance, malicious-client resilience, all movement variants, full protocol-763 compatibility, public-server safety, or production readiness.

## Evidence basis

| Artifact | Path |
| --- | --- |
| Normalized evidence | `docs/evidence/movement-packet-family-2026-06-06.kv` |
| Row receipt | `docs/evidence/movement-packet-family-2026-06-06.receipt.json` |
| Client log | `docs/evidence/protocol-763-ctf-spawn-team-balance-reset-2026-06-01.client-a.log` |
| Server log | `docs/evidence/protocol-763-ctf-spawn-team-balance-reset-2026-06-01.server.log` |
| Source snapshot | `docs/evidence/movement-packet-family-stevenarella-source-2026-06-06.rs` |
| Checker | `tools/check_movement_packet_family.rs` (validates normalized KV plus `protocol-763-packet-inventory-2026-05-28.tsv`) |

## Normalized facts

| Metric | Value |
| --- | --- |
| `row.id` | `movement-packet-family` |
| `packet.row` | `play/serverbound/0x15 Full -> PlayerPositionLook` |
| `packet.inventory.mapping_status` | `reviewed_override_no_shape_claim` |
| `packet.inventory.parser_shape_status` | `shape_review_missing` |
| `packet.inventory.coverage_status` | `scenario_bounded` |
| `actor.username` | `compatbota` |
| `movement.start` | `0.000,85.000,0.000` |
| `movement.target` | `-4.0,84.0,4.0` |
| `movement.look` | `yaw=0.0,pitch=0.0` |
| `movement.on_ground` | `true` |
| `movement.tolerance` | `exact_logged_values` |
| `child.stevenarella.rev` | `d9caec597041b3443d894701591752d23772e5ae` |
| `child.valence.rev` | `f40d6d6d5aeee300bdaeb406c475e2607ac3b6a7` |

## Log excerpts used by the checker contract

Client evidence in `protocol-763-ctf-spawn-team-balance-reset-2026-06-01.client-a.log` records:

```text
MC-COMPAT-MILESTONE active_probe_position_look_sent x=0.000 y=85.000 z=0.000 on_ground=true
MC-COMPAT-MILESTONE team_probe_enter_red_portal x=-4.0 y=84.0 z=4.0
```

Server evidence in `protocol-763-ctf-spawn-team-balance-reset-2026-06-01.server.log` records:

```text
MC-COMPAT-MILESTONE ctf_spawn_team_assignment username=compatbota team=Red red_count=1 blue_count=0 spawn_x=-40.0 spawn_y=65.0 spawn_z=0.0 slot36=WoodenSword:1 slot37=RedWool:64 correlation_id=team-select-compatbota
```

The source snapshot shows that the team probe writes `packet::play::serverbound::PlayerPositionLook` with the target coordinates and `on_ground=true` for the portal movement.

## Non-claims

No movement physics, collision, anti-cheat, latency tolerance, malicious-client resilience, all movement packet variants, broad parser-shape coverage, full protocol-763 compatibility, broad Minecraft compatibility, public-server safety, or production-readiness claim is made.
