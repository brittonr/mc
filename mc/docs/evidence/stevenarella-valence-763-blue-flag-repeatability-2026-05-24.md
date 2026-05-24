# Stevenarella ⇄ Valence protocol-763 BLUE flag-score repeatability — 2026-05-24

## Scope

This records three sequential live runs of the mirrored BLUE-team `blue-flag-score` scenario against the owned local Valence `ctf` fixture.

Each run used:

```sh
SERVER_PROTOCOL=763 SERVER_VERSION=1.20.1 \
VALENCE_REV=main VALENCE_EXAMPLE=ctf \
VALENCE_WORKTREE=/tmp/valence-compat-763 \
VALENCE_TARGET_DIR=/tmp/valence-compat-763-target \
CLIENT_TIMEOUT=180 \
nix run .#mc-compat-smoke -- --run \
  --server-backend valence \
  --scenario blue-flag-score \
  --receipt target/mc-compat-blue-repeat/blue-flag-score-run-N.json
```

Valence was stopped between runs with `nix run .#mc-compat-smoke -- --stop`.

## Results

All runs passed: `3/3`.

```text
run  status  observed scenario milestones                                      observed server milestones                  BLAKE3
1    pass    protocol_detected,join_game,render_tick,team_blue,flag_pickup,flag_capture,score_blue_1  server_username_seen,server_flag_or_score  99b0b32da606e1456d325a28c1b05e5511c2c89fd08e3296519413f8cc873529
2    pass    protocol_detected,join_game,render_tick,team_blue,flag_pickup,flag_capture,score_blue_1  server_username_seen,server_flag_or_score  0fa8d1877165169c33e831003c9cc8125c80d52de0130ead1fb33771d2a446cf
3    pass    protocol_detected,join_game,render_tick,team_blue,flag_pickup,flag_capture,score_blue_1  server_username_seen,server_flag_or_score  6f4fdca35ce9b13023414b65f323aacf72c295a35e84effbd59bcbe9121534e1
```

Local ignored receipts:

```text
target/mc-compat-blue-repeat/blue-flag-score-run-1.json
target/mc-compat-blue-repeat/blue-flag-score-run-2.json
target/mc-compat-blue-repeat/blue-flag-score-run-3.json
```

Client logs recorded in receipts:

```text
/tmp/mc-compat-client.compatbot.1779648062262.log
/tmp/mc-compat-client.compatbot.1779648244743.log
/tmp/mc-compat-client.compatbot.1779648427278.log
```

A hygiene scan across the run logs, final Valence log, and client logs found zero matches for:

```text
panic | parser | parse | disconnect | decode_error | protocol_mismatch
```

Port cleanup after the final stop showed no listener on `:25565`.

## Non-claims

This strengthens the BLUE mirrored scoring path from a one-off receipt to repeatable local evidence. It still does not claim:

- full CTF semantic correctness;
- full inventory/combat/drop/pickup semantics;
- broad Minecraft 1.20.1 compatibility;
- public-server or production-load safety;
- unbounded long-run stability;
- complete client/server packet coverage.
