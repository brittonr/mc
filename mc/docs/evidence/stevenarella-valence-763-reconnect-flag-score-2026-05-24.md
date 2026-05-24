# Stevenarella / Valence 763 reconnect flag-score evidence (2026-05-24)

Receipt BLAKE3: `5bf9ec0d77277227ad1d4912435c2934ba5672b22caa31a94ecbeaa1aa46b5ff`

## Result

`bounded_same_username_two_session_reconnect_ctf_flag_score_observed_no_logged_runtime_failure`

A single Valence `ctf` server stayed running while the same Stevenarella username (`steve763rejoin`) connected, selected RED, captured/scored once, was ended by bounded timeout, waited 30 seconds, then connected again with the same username and repeated RED selection plus flag capture/score.

## Commits

- Parent before evidence: `d477123 mc: record 763 repeated flag scoring probe`
- Stevenarella: `6be4515 stevenarella: count repeated ctf score events`
- Valence: `c5140b7 valence: add parkour smoke receipts`

## Probe

- Server: Valence `ctf` example on `127.0.0.1:25565`
- Client command shape: `timeout 150s xvfb-run -a env ... cargo run --release -- --server 127.0.0.1:25565 --username steve763rejoin`
- Environment: `MC_COMPAT_ACTIVE_PROBE=1 MC_COMPAT_TEAM_PROBE=1 MC_COMPAT_TEAM_PROBE_TEAM=red MC_COMPAT_FLAG_PROBE=1 MC_COMPAT_FLAG_PROBE_REPEAT=1`
- Inter-session gap: `30s`
- Both status files: `exit=124`; the bounded timeout is expected after evidence milestones.

## Session observations

### first

- Log BLAKE3: `9bf9f7fb0b8ff3b1427db2fbdf9b7f7d6b2cceab2c82973effd9fe70145443bf`
- Status BLAKE3: `f67d19a0fed77375c9ee600f70ff255957e708d6c0898a270d40151761401f0a`
- Timeout status: `124`
- `Detected server protocol version 763`: `1`
- `MC-COMPAT-MILESTONE login_success`: `1`
- `MC-COMPAT-MILESTONE join_game`: `2`
- `MC-COMPAT-MILESTONE render_tick_with_player`: `1`
- `Received chat message: You are on team RED!`: `1`
- `MC-COMPAT-MILESTONE flag_probe_have_flag_chat`: `1`
- `MC-COMPAT-MILESTONE flag_probe_capture_chat`: `1`
- `MC-COMPAT-MILESTONE flag_probe_score_chat`: `1`
- `MC-COMPAT-MILESTONE flag_probe_repeat_target_reached`: `1`
- Score block: `Scores:
RED: 3
BLUE: 0`
- Failure marker counts: `{'Bad packet': 0, 'FromUtf8Error': 0, 'UnexpectedEof': 0, 'disconnect': 0, 'failed to read packet': 0, 'panic': 0}`

### second

- Log BLAKE3: `768f8351aa636989cbb41e3abebe708ce64ca3f7b74d536cfa5432c61117fb23`
- Status BLAKE3: `f67d19a0fed77375c9ee600f70ff255957e708d6c0898a270d40151761401f0a`
- Timeout status: `124`
- `Detected server protocol version 763`: `1`
- `MC-COMPAT-MILESTONE login_success`: `1`
- `MC-COMPAT-MILESTONE join_game`: `2`
- `MC-COMPAT-MILESTONE render_tick_with_player`: `1`
- `Received chat message: You are on team RED!`: `1`
- `MC-COMPAT-MILESTONE flag_probe_have_flag_chat`: `1`
- `MC-COMPAT-MILESTONE flag_probe_capture_chat`: `1`
- `MC-COMPAT-MILESTONE flag_probe_score_chat`: `1`
- `MC-COMPAT-MILESTONE flag_probe_repeat_target_reached`: `1`
- Score block: `Scores:
RED: 4
BLUE: 0`
- Failure marker counts: `{'Bad packet': 0, 'FromUtf8Error': 0, 'UnexpectedEof': 0, 'disconnect': 0, 'failed to read packet': 0, 'panic': 0}`

## What this proves

- Bounded same-username reconnect/session restart against a continuous Valence `ctf` server can reach protocol 763 login, join, render, RED team selection, flag pickup, capture, and score in both sessions.
- No `UnexpectedEof`, `FromUtf8Error`, `failed to read packet`, `Bad packet`, `panic`, or `disconnect` marker appeared in either client log.

## What this does not prove

- Long soak stability.
- Full CTF/gameplay semantics.
- Full Minecraft 1.20.1 compatibility or complete protocol 763 coverage.
