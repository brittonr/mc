# Stevenarella ⇄ Valence protocol-763 BLUE flag-score probe — 2026-05-24

## Scope

This slice extends the bounded Valence `ctf` compatibility proof from the RED scoring path to the mirrored BLUE-team path.

The runner scenario is:

```sh
SERVER_PROTOCOL=763 SERVER_VERSION=1.20.1 \
VALENCE_REV=main VALENCE_EXAMPLE=ctf \
VALENCE_WORKTREE=/tmp/valence-compat-763 \
VALENCE_TARGET_DIR=/tmp/valence-compat-763-target \
CLIENT_TIMEOUT=180 \
nix run .#mc-compat-smoke -- --run \
  --server-backend valence \
  --scenario blue-flag-score \
  --receipt target/mc-compat-blue-flag-score.json
```

The client-side probe now accepts `MC_COMPAT_FLAG_PROBE_TEAM=blue`; for BLUE it selects the BLUE portal, targets the RED flag position, and returns to the BLUE capture position. The default remains RED for existing scenarios.

## Receipt

Local ignored receipt:

```text
target/mc-compat-blue-flag-score.json
```

BLAKE3:

```text
81729af12fa0b88e8165cb09ce591b5905aa5339c8a1fc71d9c5496e64751bca  target/mc-compat-blue-flag-score.json
```

Parsed receipt summary:

```text
status: pass
scenario: blue-flag-score
client log: /tmp/mc-compat-client.compatbot.1779647687037.log
```

Observed scenario milestones:

```text
protocol_detected
join_game
render_tick
team_blue
flag_pickup
flag_capture
score_blue_1
```

Observed server/correlation milestones:

```text
server_username_seen
server_flag_or_score
```

Log hygiene scan over the runner log, Valence log, and client log found zero matches for:

```text
panic | parser | parse | disconnect | decode_error | protocol_mismatch
```

## Verification

Focused checks run before recording this evidence:

```sh
cargo fmt -- --check                     # stevenarella, in mc devshell
cargo check --bin stevenarella           # stevenarella, in mc devshell
cargo fmt -- --check                     # mc-compat-runner, in mc devshell
cargo test scenario_cli_and_env_parse -- --nocapture
cargo test blue_flag_score_scenario_tracks_mirrored_team_evidence -- --nocapture
nix run .#mc-compat-smoke -- --dry-run --server-backend valence --scenario blue-flag-score --receipt target/mc-compat-blue-flag-score-dry-run.json
```

## Non-claims

This proves the mirrored BLUE scoring path for the owned local Valence `ctf` fixture under protocol `763`. It still does not claim:

- full CTF semantic correctness;
- full inventory/combat/drop/pickup semantics;
- broad Minecraft 1.20.1 compatibility;
- public-server or production-load safety;
- unbounded long-run stability;
- complete client/server packet coverage.
