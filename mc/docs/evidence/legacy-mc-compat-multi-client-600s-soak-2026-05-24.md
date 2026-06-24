# MC compat multi-client 600s soak evidence — 2026-05-24

## Scope

This note records a local, owned Valence-backed bounded soak proof for the `multi-client-load-score` scenario on protocol 763 / Minecraft 1.20.1.

This is evidence that the bounded scenario ran for the configured client timeout and still produced the expected multi-client scoring and server-correlation milestones. It is not a claim of full Minecraft correctness, semantic equivalence, public-server load safety, production load, or unbounded soak stability.

## Command

```sh
SERVER_PROTOCOL=763 SERVER_VERSION=1.20.1 VALENCE_REV=main \
VALENCE_EXAMPLE=ctf VALENCE_WORKTREE=/tmp/valence-compat-763 \
VALENCE_TARGET_DIR=/tmp/valence-compat-763-target CLIENT_TIMEOUT=600 \
nix run .#mc-compat-smoke -- --run --server-backend valence \
  --scenario multi-client-load-score \
  --receipt target/mc-compat-soak/multi-client-load-score-600s-2026-05-24.json
```

## Result

- Receipt: `target/mc-compat-soak/multi-client-load-score-600s-2026-05-24.json`
- Status: `pass`
- Classification: `multi-client-load-evidence`
- Configured client timeout: `600` seconds
- BLAKE3: `c2240f09565bc020e562d2a5ddba364c199e66973d191f1076bb58158e972eb8`

Observed scenario milestones:

- `multi_client_count`
- `protocol_detected`
- `join_game`
- `render_tick`
- `team_red`
- `flag_pickup`
- `flag_capture`
- `score_red_1`

Observed server/correlation milestones:

- `server_client_a_seen`
- `server_client_b_seen`
- `server_flag_or_score`

Client logs recorded in the receipt:

- `/tmp/mc-compat-client.compatbota.1779643320405.log`
- `/tmp/mc-compat-client.compatbotb.1779643322406.log`

## Log hygiene check

The receipt client logs were scanned for:

- `panic`
- parser/parse failure strings
- disconnects / connection resets
- decode errors
- `unexpected_eof`
- `protocol_mismatch`

Result: **0 hits**.

## Repository state at closeout

Before writing this note, the working trees were clean/aligned:

- parent `/home/brittonr/git`: `main...origin/main`
- `stevenarella`: `master...fork/master`

The receipt JSON remains a generated artifact under ignored `target/`; this tracked note preserves its path, hash, timeout, milestone summary, and log-hygiene result.
