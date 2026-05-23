# Stevenarella → Valence 763 team interaction probe (2026-05-23)

This records a bounded 180-second Valence `ctf` probe after adding a narrow `MC_COMPAT_TEAM_PROBE=1` harness path in our Stevenarella fork. It builds on `MC_COMPAT_ACTIVE_PROBE=1`: the client reaches render, performs active movement, enters/holds the red team portal position, sends a held-item selection, and sends a 1.20.1 use-item packet.

## Result

- Status: `bounded_180s_timeout_team_interaction_probe_no_logged_runtime_failure`.
- Probe status: `exit=124`. The `124` timeout is expected for the bounded run.
- Stevenarella commit: `ca62c2c`.
- Valence commit: `c5140b7`.
- Receipt: `stevenarella-valence-763-team-interaction-2026-05-23.receipt.json`.
- Receipt BLAKE3: `a424f52a04695f4e12338e051ca686b162af2b8b9c8830e1412980d42ac82db3`.
- Probe log BLAKE3: `7878158d32b736b02bf8c1fb277d2c0d15b080f42dcfccf9ef4c550f9a2d9682`.

## Observed milestones and failures

- `Detected server protocol version 763`: `1`
- `MC-COMPAT-MILESTONE login_success`: `1`
- `MC-COMPAT-MILESTONE join_game_763_shape`: `1`
- `MC-COMPAT-MILESTONE first_chunk_data`: `1`
- `MC-COMPAT-MILESTONE render_tick_with_player`: `1`
- `MC-COMPAT-MILESTONE active_probe_position_look_sent`: `1`
- `MC-COMPAT-MILESTONE team_probe_enter_red_portal`: `1`
- `MC-COMPAT-MILESTONE team_probe_hold_red_portal`: `1`
- `MC-COMPAT-MILESTONE team_probe_select_hotbar_slot`: `1`
- `MC-COMPAT-MILESTONE team_probe_use_item_sent`: `1`
- `Received chat message`: `1`
- `You are on team`: `0`
- `UnexpectedEof`: `0`
- `FromUtf8Error`: `0`
- `failed to parse packet`: `0`
- `panicked at`: `0`
- `Failed to read all of packet`: `0`
- `disconnect`: `0`
- `Disconnect`: `0`

The probe reached protocol/login/join/first-chunk/render milestones, sent active movement, sent protocol-763 position/interaction packets for a red-portal team probe, and logged no `UnexpectedEof`, `FromUtf8Error`, parser panic, packet parse failure, failed packet read, or disconnect marker in this bounded run.

## Code changes covered

- Added protocol 763 serverbound mappings for interaction-adjacent packets: `UseEntity_Sneakflag` (`0x10`), `PlayerPosition` (`0x14`), `PlayerPositionLook` (`0x15`), `HeldItemChange` (`0x28`), `PlayerBlockPlacement_insideblock_sequence` (`0x31`), and `UseItem_WithSequence` (`0x32`).
- Added the 1.20.1 sequence packet shapes used by interact-block and interact-item.
- Added `MC_COMPAT_TEAM_PROBE=1` milestones without changing default runtime behavior.

## Non-claims

This is active/team-interaction packet evidence only. It does not prove team selection semantics: the Valence `ctf` confirmation chat `You are on team` was not observed. It also does not prove full Minecraft 1.20.1 compatibility, stable in-world gameplay, combat semantics, inventory semantics, death/respawn behavior, reconnect behavior, full protocol 763 coverage, or semantic parser correctness.
