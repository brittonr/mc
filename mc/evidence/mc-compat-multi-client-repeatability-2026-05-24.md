# MC compat multi-client repeatability evidence — 2026-05-24

## Scope

This note records a local, owned Valence-backed repeatability proof for the `multi-client-load-score` scenario after the protocol-763 scoring evidence landed.

This is evidence that the bounded scenario repeated successfully under the recorded inputs. It is not a claim of full Minecraft correctness, semantic equivalence, public-server load safety, or unbounded soak stability.

## Command shape

Each run used the same command shape with a distinct receipt path:

```sh
SERVER_PROTOCOL=763 SERVER_VERSION=1.20.1 VALENCE_REV=main \
VALENCE_EXAMPLE=ctf VALENCE_WORKTREE=/tmp/valence-compat-763 \
VALENCE_TARGET_DIR=/tmp/valence-compat-763-target CLIENT_TIMEOUT=240 \
nix run .#mc-compat-smoke -- --run --server-backend valence \
  --scenario multi-client-load-score \
  --receipt target/mc-compat-repeat/multi-client-load-score-run-${i}.json
```

## Result

Pass count: **3/3** live runs.

| Run | Receipt | Status | BLAKE3 |
| --- | --- | --- | --- |
| 1 | `target/mc-compat-repeat/multi-client-load-score-run-1.json` | pass | `61fb94fb14e30bc17b34171fef92f5102e2dfc30b3d828d6884622ea3a35979c` |
| 2 | `target/mc-compat-repeat/multi-client-load-score-run-2.json` | pass | `1b22574370d437d36f6883f8ccd1bcdf704af77954d303285b121345c49400d4` |
| 3 | `target/mc-compat-repeat/multi-client-load-score-run-3.json` | pass | `b641eadbe4395002cbd536a0b08bf1449157cc299e300f25070eb0fb54f7e812` |

All three receipts observed these scenario milestones:

- `multi_client_count`
- `protocol_detected`
- `join_game`
- `render_tick`
- `team_red`
- `flag_pickup`
- `flag_capture`
- `score_red_1`

All three receipts observed these server/correlation milestones:

- `server_client_a_seen`
- `server_client_b_seen`
- `server_flag_or_score`

## Log hygiene check

The client logs referenced by the three receipts were scanned for:

- `panic`
- parser/parse failure strings
- disconnects / connection resets
- decode errors

Result: **0 hits**.

## Repository state at closeout

After the repeatability run, the working trees were clean/aligned:

- parent `/home/brittonr/git`: `main...origin/main`
- `stevenarella`: `master...fork/master`

The receipt JSON files remain generated artifacts under ignored `target/`; this tracked note preserves their paths, hashes, milestone summary, and log-hygiene result.
