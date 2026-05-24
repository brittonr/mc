# Strict-decode maintained Valence CTF 600s soak — 2026-05-24

## Scope

This records the first maintained-entrypoint protocol-763 Valence CTF multi-client soak after removing the scenario-scoped `MC_COMPAT_IGNORE_DECODE_ERRORS=1` escape hatch from `multi-client-load-score` client runs.

Command:

```sh
cd /home/brittonr/git/mc
MC_COMPAT_SOAK_RECEIPT=target/mc-compat-soak/strict-decode-maintained-600s.json \
  nix run .#mc-compat-valence-ctf-600s-soak > /tmp/strict-decode-maintained-600s.out 2>&1
```

The entrypoint encoded:

- backend: Valence
- example: `ctf`
- Minecraft version/protocol: `1.20.1` / `763`
- scenario: `multi-client-load-score`
- clients: `compatbota`, `compatbotb`
- scoring client timeout: `600s`
- passive peer timeout: runner-managed short peer timeout
- decode-error mode: strict/default Stevenarella behavior, no `MC_COMPAT_IGNORE_DECODE_ERRORS` env injected by the runner

## Receipt

Local ignored receipt:

```text
target/mc-compat-soak/strict-decode-maintained-600s.json
```

BLAKE3:

```text
85061e95562955bbd575dfabc3ef527af1987d09e5f1141d3e4d618eba2af09a  target/mc-compat-soak/strict-decode-maintained-600s.json
```

Parsed receipt summary:

```text
status: pass
mode: run
server protocol/version: 763 / 1.20.1
client timeout: 600
client logs: /tmp/mc-compat-client.compatbota.1779646464010.log; /tmp/mc-compat-client.compatbotb.1779646466010.log
```

Observed scenario milestones:

```text
multi_client_count
protocol_detected
join_game
render_tick
team_red
flag_pickup
flag_capture
score_red_1
```

Observed server/correlation milestones:

```text
server_client_a_seen
server_client_b_seen
server_flag_or_score
```

Log hygiene scan over the maintained-entrypoint run log, Valence log, and both client logs found zero matches for:

```text
panic | parser | parse | disconnect | decode_error | protocol_mismatch
```

A preceding strict-decode 240s `mc-compat-smoke --scenario multi-client-load-score` probe also passed with the same milestone set and receipt hash:

```text
b439ae769e43a8c38c4d8c4001c003303d03cb31c9033be5b7442000beff2e36  target/mc-compat-soak/strict-decode-multi-client-240s.json
```

## Non-claims

This strengthens the bounded two-client Valence CTF evidence by proving the scenario no longer needs decode-error suppression. It still does not claim:

- full CTF semantic correctness;
- broad Minecraft 1.20.1 compatibility;
- public-server or production load safety;
- unbounded long-run stability;
- complete client/server packet coverage.
