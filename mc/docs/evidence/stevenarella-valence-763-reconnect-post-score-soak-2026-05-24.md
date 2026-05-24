# Stevenarella / Valence 763 reconnect post-score soak evidence (2026-05-24)

Receipt BLAKE3: `0525a258bd82da5472261ac14fdf22316dc5d120baf10030f134aa0b6c70f945`

## Result

`bounded_same_username_reconnect_then_600s_second_session_after_score_probe_no_logged_runtime_failure`

A single Valence `ctf` server stayed running while the same Stevenarella username (`steve763soak`) connected, selected RED, captured/scored once, was ended by bounded timeout, waited 30 seconds, then connected again with the same username. The second session reached login/join/render/team/flag-score milestones before its bounded `600s` timeout, and the process stayed alive until that timeout without logged runtime-failure markers.

## Commits

- Parent before evidence: `5cf5ee2 mc: record 763 reconnect flag scoring probe`
- Stevenarella: `6be4515 stevenarella: count repeated ctf score events`
- Valence: `c5140b7 valence: add parkour smoke receipts`

## Probe

- Server: Valence `ctf` example on `127.0.0.1:25565`
- Client command shape: `timeout <seconds>s xvfb-run -a env ... cargo run --release -- --server 127.0.0.1:25565 --username steve763soak`
- Environment: `MC_COMPAT_ACTIVE_PROBE=1 MC_COMPAT_TEAM_PROBE=1 MC_COMPAT_TEAM_PROBE_TEAM=red MC_COMPAT_FLAG_PROBE=1 MC_COMPAT_FLAG_PROBE_REPEAT=1`
- Inter-session gap: `30s`
- First session bound: `150s`; second soak session bound: `600s`
- Both status files: `exit=124`; the bounded timeout is expected after evidence milestones.

## Session observations

### first

- Bound: `150s`
- Status: `exit=124`
- Log BLAKE3: `43068498b3363593c50c6cfcb77b8cb583f8948afd2a38ec583d09e77dec2d0a`
- Status BLAKE3: `f67d19a0fed77375c9ee600f70ff255957e708d6c0898a270d40151761401f0a`
- `Detected server protocol version 763`: `1`
- `MC-COMPAT-MILESTONE login_success`: `1`
- `MC-COMPAT-MILESTONE join_game`: `2`
- `MC-COMPAT-MILESTONE render_tick_with_player`: `1`
- `MC-COMPAT-MILESTONE active_probe_position_look_sent`: `1`
- `Received chat message: You are on team RED!`: `1`
- `MC-COMPAT-MILESTONE flag_probe_have_flag_chat`: `1`
- `MC-COMPAT-MILESTONE flag_probe_capture_chat`: `1`
- `MC-COMPAT-MILESTONE flag_probe_score_chat`: `1`
- `MC-COMPAT-MILESTONE flag_probe_repeat_target_reached`: `1`
- Score block: `Scores:
RED: 1
BLUE: 0`
- Failure marker counts: `{'UnexpectedEof': 0, 'FromUtf8Error': 0, 'failed to read packet': 0, 'Bad packet': 0, 'panic': 0, 'disconnect': 0}`

### second-soak

- Bound: `600s`
- Status: `exit=124`
- Log BLAKE3: `6eb3023f9a8c2094063d0fb141b046a63705d4d3a3eeae1d6c3038b4cf0b7910`
- Status BLAKE3: `f67d19a0fed77375c9ee600f70ff255957e708d6c0898a270d40151761401f0a`
- `Detected server protocol version 763`: `1`
- `MC-COMPAT-MILESTONE login_success`: `1`
- `MC-COMPAT-MILESTONE join_game`: `2`
- `MC-COMPAT-MILESTONE render_tick_with_player`: `1`
- `MC-COMPAT-MILESTONE active_probe_position_look_sent`: `1`
- `Received chat message: You are on team RED!`: `1`
- `MC-COMPAT-MILESTONE flag_probe_have_flag_chat`: `1`
- `MC-COMPAT-MILESTONE flag_probe_capture_chat`: `1`
- `MC-COMPAT-MILESTONE flag_probe_score_chat`: `1`
- `MC-COMPAT-MILESTONE flag_probe_repeat_target_reached`: `1`
- Score block: `Scores:
RED: 2
BLUE: 0`
- Failure marker counts: `{'UnexpectedEof': 0, 'FromUtf8Error': 0, 'failed to read packet': 0, 'Bad packet': 0, 'panic': 0, 'disconnect': 0}`

## What this proves

- Bounded same-username reconnect/session restart against a continuous Valence `ctf` server can reach protocol 763 login, join, render, active movement, RED team selection, flag pickup, capture, and score before the second session's `600s` bounded timeout.
- The second client process stayed alive until the expected `timeout` boundary after reaching score evidence, with no logged `UnexpectedEof`, `FromUtf8Error`, `failed to read packet`, `Bad packet`, `panic`, or `disconnect` marker.

## What this does not prove

- Long soak stability beyond the bounded `600s` second session.
- Full CTF/gameplay semantics.
- Full Minecraft 1.20.1 compatibility or complete protocol 763 coverage.
