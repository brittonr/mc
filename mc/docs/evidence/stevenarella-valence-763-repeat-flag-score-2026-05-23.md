# Stevenarella → Valence protocol 763 repeated flag-scoring probe (2026-05-23)

## Scope

Bounded single-client headless Stevenarella probe against Valence `ctf` after the protocol 763 flag-scoring proof.

This records a narrow repeatability slice: the same client joins RED, captures the BLUE flag twice, and observes two Valence score updates in one bounded run. It does not claim load, reconnect, long-soak, or full CTF semantics.

## Code under test

- Stevenarella fork commit: `6be4515 stevenarella: count repeated ctf score events`
- Parent mc base before this evidence: `45437ec mc: record 763 ctf flag scoring probe`
- Valence checkout: `c5140b7 valence: add parkour smoke receipts`

## Probe command shape

Valence `ctf` was started from `/home/brittonr/git/mc/valence` in the mc Nix devshell.

Stevenarella was run headlessly with:

```sh
MC_COMPAT_ACTIVE_PROBE=1 \
MC_COMPAT_TEAM_PROBE=1 \
MC_COMPAT_TEAM_PROBE_TEAM=red \
MC_COMPAT_FLAG_PROBE=1 \
MC_COMPAT_FLAG_PROBE_REPEAT=2 \
xvfb-run -a cargo run --release -- \
  --server 127.0.0.1:25565 \
  --username FlagRepeatRed \
  --default-protocol-version 763
```

The client was bounded with `timeout 240s`; status `124` is the expected timeout after the observed milestones.

## Observed milestones

The probe log observed:

- `Detected server protocol version 763`
- `MC-COMPAT-MILESTONE login_success state=play protocol=763 username=FlagRepeatRed properties=0`
- `MC-COMPAT-MILESTONE join_game_763_shape dimension_type=minecraft:overworld world=minecraft:overworld portal_cooldown=0`
- `MC-COMPAT-MILESTONE render_tick_with_player`
- `Received chat message: You are on team RED!`
- `MC-COMPAT-MILESTONE flag_probe_move_to_blue_flag x=48.0 y=65.0 z=0.0 cycle=1`
- `MC-COMPAT-MILESTONE flag_probe_dig_blue_flag_sent status=stop_destroy location=46,67,0 sequence=1 cycle=1`
- `MC-COMPAT-MILESTONE flag_probe_have_flag_chat count=1 target=2`
- `MC-COMPAT-MILESTONE flag_probe_capture_chat count=1 target=2`
- `MC-COMPAT-MILESTONE flag_probe_score_chat count=1 target=2 message=Scores:` with score block `RED: 1` / `BLUE: 0`
- `MC-COMPAT-MILESTONE flag_probe_move_to_blue_flag x=48.0 y=65.0 z=0.0 cycle=2`
- `MC-COMPAT-MILESTONE flag_probe_dig_blue_flag_sent status=stop_destroy location=46,67,0 sequence=2 cycle=2`
- `MC-COMPAT-MILESTONE flag_probe_have_flag_chat count=2 target=2`
- `MC-COMPAT-MILESTONE flag_probe_capture_chat count=2 target=2`
- `MC-COMPAT-MILESTONE flag_probe_score_chat count=2 target=2 message=Scores:` with score block `RED: 2` / `BLUE: 0`
- `MC-COMPAT-MILESTONE flag_probe_repeat_target_reached count=2 target=2`

The probe therefore proves two bounded Stevenarella fork flag pickup/capture/scoring cycles against Valence `ctf` in this scenario.

## Runtime failure scan

The bounded probe log contained zero occurrences of:

- `UnexpectedEof`
- `FromUtf8Error`
- `failed to read packet`
- `Bad packet`
- `panic`
- `disconnect`

## Artifacts

- Receipt: `docs/evidence/stevenarella-valence-763-repeat-flag-score-2026-05-23.receipt.json`
- Receipt BLAKE3: `d1488626d8e39132f8abfb41b0355e6ed8d9fda7ae0c56276e1a02a3bf3df5fb`
- Source temp log BLAKE3: `4bc2564994572b36b4226082761d9258255a8ce5ffd4555739a51a913366bca5`
- Source temp status BLAKE3: `16d78a954cb775e521c46341a98c2c6a187ea74a6d018de5efbae328941e87e6`

## Non-claims

This evidence does **not** prove:

- repeatable scoring under load,
- full CTF semantics,
- full combat or inventory semantics,
- reconnect/session behavior,
- stable gameplay or long soak,
- full Minecraft 1.20.1 compatibility,
- complete protocol 763 coverage.
