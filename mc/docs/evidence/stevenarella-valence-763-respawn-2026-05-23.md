# Stevenarella → Valence protocol 763 bounded respawn probe (2026-05-23)

Receipt status: `bounded_two_client_respawn_probe_request_health_restored_respawn_packet_observed_no_logged_runtime_failure`.

Receipt BLAKE3: `8ac373ed45d7b2460d098b7f791f7c8118078e46fa1baabdb0bf937aeecbe491`

## Scope

Bounded two-client headless Stevenarella probe against Valence `ctf` using protocol `763` / Minecraft `1.20.1`.

Probe flags:

- `MC_COMPAT_ACTIVE_PROBE=1`
- `MC_COMPAT_TEAM_PROBE=1`
- `MC_COMPAT_COMBAT_PROBE=1`
- `MC_COMPAT_RESPAWN_PROBE=1`

Roles:

- `RespawnBlue6`: BLUE victim, `MC_COMPAT_COMBAT_PROBE_ROLE=victim`
- `RespawnRed6`: RED attacker, `MC_COMPAT_COMBAT_PROBE_ROLE=attacker`

Stevenarella fork commit: `559c7a6`.
Valence repo unchanged at `c5140b7`.

## Observed bounded milestones

- Both clients detected server protocol version `763`.
- Both clients reached login success, 763 join-game shape, first chunk data, and render tick with player.
- BLUE selected BLUE: `You are on team BLUE!`
- RED selected RED: `You are on team RED!`
- RED sent 10 bounded combat attack packets.
- BLUE victim health decreased through:
  - `update_health health=16.0`
  - `update_health health=12.0`
  - `update_health health=8.0`
  - `update_health health=4.0`
  - `update_health health=0.0`
- BLUE observed death-health milestone: `combat_probe_death_observed health=0.0`.
- BLUE sent respawn request: `respawn_probe_request_sent action_id=0`.
- BLUE observed restored health: `respawn_probe_health_restored health=20.0`.
- BLUE decoded Valence protocol 763 respawn packet shape: `respawn_packet_763_shape dimension_type=minecraft:overworld world=minecraft:overworld portal_cooldown=0`.

No logged `UnexpectedEof`, `FromUtf8Error`, `panicked at`, `failed to parse packet`, `Failed to read all of packet`, `bad packet id`, or `Disconnect` markers were observed in the bounded probe logs.

## Artifact hashes

- BLUE log: `1f95a6a6da6ee626da0c53858b68cbb4906ef50daacea15f356c54d47f8ce047`
- BLUE status: `f67d19a0fed77375c9ee600f70ff255957e708d6c0898a270d40151761401f0a`
- RED log: `a5a500bb9b05fe27a7bc44563cfa8fc39da8e86bbbaee3439ac95f32d98737cb`
- RED status: `f67d19a0fed77375c9ee600f70ff255957e708d6c0898a270d40151761401f0a`

Both status files contain `exit=124`, meaning the bounded `timeout` ended the probes after evidence was collected.

## Non-claims

This proves only bounded Valence `ctf` respawn semantics for the observed path: death-health `0.0`, client respawn request, restored health `20.0`, and decoded protocol 763 respawn packet shape.

It does **not** prove full Minecraft 1.20.1 compatibility, complete protocol 763 coverage, stable in-world gameplay, full combat correctness, inventory correctness, death-message semantics, reconnect/session behavior, or long-term soak.
