# Stevenarella → Valence protocol 763 flag-scoring probe (2026-05-23)

## Scope

Bounded single-client headless Stevenarella probe against Valence `ctf` after the prior protocol 763 team-selection/combat/death/respawn/inventory work.

This records a narrow flag pickup/capture/scoring slice after joining RED team. It does not claim full CTF, inventory, combat, reconnect, or long-soak behavior.

## Code under test

- Stevenarella fork commit: `656743f stevenarella: add 763 ctf flag probe`
- Parent mc base before this evidence: `66d7f57 mc: record 763 ctf inventory probe`
- Valence checkout: `c5140b7 valence: add parkour smoke receipts`

## Probe command shape

Valence `ctf` was started from `/home/brittonr/git/mc/valence` in the mc Nix devshell.

Stevenarella was run headlessly with:

```sh
MC_COMPAT_ACTIVE_PROBE=1 \
MC_COMPAT_TEAM_PROBE=1 \
MC_COMPAT_TEAM_PROBE_TEAM=red \
MC_COMPAT_FLAG_PROBE=1 \
xvfb-run -a cargo run --release -- \
  --server 127.0.0.1:25565 \
  --username FlagProbeRed \
  --default-protocol-version 763
```

The client was bounded with `timeout 180s`; status `124` is the expected timeout after the observed milestones.

## Observed milestones

The probe log observed:

- `Detected server protocol version 763`
- `MC-COMPAT-MILESTONE login_success state=play protocol=763 username=FlagProbeRed properties=0`
- `MC-COMPAT-MILESTONE join_game_763_shape dimension_type=minecraft:overworld world=minecraft:overworld portal_cooldown=0`
- `MC-COMPAT-MILESTONE render_tick_with_player`
- `Received chat message: You are on team RED!`
- `MC-COMPAT-MILESTONE flag_probe_move_to_blue_flag x=48.0 y=65.0 z=0.0`
- `MC-COMPAT-MILESTONE flag_probe_dig_blue_flag_sent status=stop_destroy location=46,67,0 sequence=1`
- `Received chat message: You have the flag!`
- `MC-COMPAT-MILESTONE flag_probe_have_flag_chat`
- `MC-COMPAT-MILESTONE flag_probe_move_to_red_capture x=-48.0 y=65.0 z=0.0`
- `Received chat message: You captured the flag!`
- `MC-COMPAT-MILESTONE flag_probe_capture_chat`
- Score chat block:

```text
Scores:
RED: 1
BLUE: 0
```

The probe therefore proves the bounded Stevenarella fork could join RED, send the protocol-763 digging/action packet for the BLUE flag, observe flag possession/capture chat, and observe the Valence score block `RED: 1` / `BLUE: 0` in this scenario.

## Runtime failure scan

The bounded probe log contained zero occurrences of:

- `UnexpectedEof`
- `FromUtf8Error`
- `failed to read packet`
- `Bad packet`
- `panic`
- `disconnect`

## Artifacts

- Receipt: `docs/evidence/stevenarella-valence-763-flag-score-2026-05-23.receipt.json`
- Receipt BLAKE3: `deb1cefa9ce7c5a612f6ef5dc146e97c178fd1d229b5d910fe24a1da7f586cab`
- Source temp log BLAKE3: `5920a63663488757fa7982b05bc58745c70226f3c4fe01be9e86ab606e9f7ddc`
- Source temp status BLAKE3: `16d78a954cb775e521c46341a98c2c6a187ea74a6d018de5efbae328941e87e6`

## Non-claims

This evidence does **not** prove:

- full CTF semantics,
- repeatable scoring under load,
- full combat or inventory semantics,
- reconnect/session behavior,
- stable gameplay or long soak,
- full Minecraft 1.20.1 compatibility,
- complete protocol 763 coverage.
