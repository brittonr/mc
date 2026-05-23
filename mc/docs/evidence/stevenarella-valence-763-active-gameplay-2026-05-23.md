# Stevenarella → Valence 763 active gameplay probe (2026-05-23)

This records a bounded 180-second Valence `ctf` probe after adding a narrow `MC_COMPAT_ACTIVE_PROBE=1` harness path in our Stevenarella fork. The active probe presses forward+sprint+jump, releases jump, turns right, then stops, and logs when the first `PlayerPositionLook` packet is sent.

## Result

- Status: `bounded_180s_timeout_active_movement_no_logged_runtime_failure`.
- Probe status: `exit=124`. The `124` timeout is expected for the bounded run.
- Stevenarella commit: `05a382b`.
- Valence commit: `c5140b7`.
- Receipt: `stevenarella-valence-763-active-gameplay-2026-05-23.receipt.json`.
- Receipt BLAKE3: `a689ec003bc55ec94619b2c0e406db3a02916f497e37b3ef4f6074e28ea1e9b1`.

## Observed milestones and failures

- `Detected server protocol version 763`: `1`
- `MC-COMPAT-MILESTONE login_compression`: `1`
- `MC-COMPAT-MILESTONE login_success`: `1`
- `MC-COMPAT-MILESTONE join_game_763_shape`: `1`
- `MC-COMPAT-MILESTONE join_game`: `2`
- `MC-COMPAT-MILESTONE first_chunk_data`: `1`
- `MC-COMPAT-MILESTONE render_tick_with_player`: `1`
- `MC-COMPAT-MILESTONE active_probe_input_start`: `1`
- `MC-COMPAT-MILESTONE active_probe_jump_release`: `1`
- `MC-COMPAT-MILESTONE active_probe_input_turn`: `1`
- `MC-COMPAT-MILESTONE active_probe_input_stop`: `1`
- `MC-COMPAT-MILESTONE active_probe_position_look_sent`: `1`
- `UnexpectedEof`: `0`
- `FromUtf8Error`: `0`
- `unknown 1.20.1 metadata type`: `0`
- `failed to parse packet`: `0`
- `panicked at`: `0`
- `Failed to read all of packet`: `0`
- `disconnect`: `0`
- `Disconnect`: `0`

The probe reached protocol/login/join/first-chunk/render milestones, executed active input milestones, and logged `MC-COMPAT-MILESTONE active_probe_position_look_sent` once. No `UnexpectedEof`, `FromUtf8Error`, parser panic, packet parse failure, failed packet read, or disconnect marker was logged in this bounded run.

## Code changes covered

- Added protocol 763 serverbound `0x15 -> PlayerPositionLook` mapping so the active movement packet uses the 1.20.1 wire ID.
- Added observed active-run clientbound mappings for chunk unload/game-state/view-position/view-distance seams.
- Added `MC_COMPAT_ACTIVE_PROBE=1` input milestones in Stevenarella without changing default runtime behavior.

## Non-claims

This is active movement/input evidence only. It does not prove full Minecraft 1.20.1 compatibility, stable in-world gameplay, team selection, combat semantics, inventory semantics, death/respawn behavior, reconnect behavior, full protocol 763 coverage, or semantic parser correctness.
