# Maintained Valence CTF 600s soak entrypoint evidence — 2026-05-24

## Scope

This records a live run through the maintained Nix entrypoint, not the earlier manual environment recipe:

```sh
cd /home/brittonr/git/mc
MC_COMPAT_SOAK_RECEIPT=target/mc-compat-soak/no-tee-live.json \
  nix run .#mc-compat-valence-ctf-600s-soak > /tmp/no-tee-live.out 2>&1
```

The entrypoint encodes the protocol-763 Valence CTF multi-client soak shape:

- server backend: `valence`
- Valence example: `ctf`
- Minecraft version: `1.20.1`
- protocol: `763`
- scenario: `multi-client-load-score`
- client timeout: `600s`
- clients: `compatbota`, `compatbotb`

## Receipt

Local ignored receipt:

```text
target/mc-compat-soak/no-tee-live.json
```

BLAKE3:

```text
70310b1a154c8a8e8961b2682330be6c1fde9e2c26053b2c1faaf5e049d0d521  target/mc-compat-soak/no-tee-live.json
```

Parsed receipt summary:

```text
status: pass
mode: run
server protocol/version: 763 / 1.20.1
client timeout: 600
client logs: /tmp/mc-compat-client.compatbota.1779645455550.log; /tmp/mc-compat-client.compatbotb.1779645457550.log
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

Log hygiene scan over the run log, Valence log, and both client logs found zero matches for:

```text
panic | parser | parse | disconnect | decode | protocol_mismatch
```

## Non-claims

This proves that the maintained Nix entrypoint can reproduce the bounded protocol-763 Valence CTF two-client scoring soak locally. It does not claim:

- full CTF semantic correctness;
- broad Minecraft 1.20.1 compatibility;
- public-server or production load safety;
- unbounded long-run stability;
- complete client/server packet coverage.

## Operational note

For this run, redirecting the entrypoint directly to a file was reliable:

```sh
nix run .#mc-compat-valence-ctf-600s-soak > /tmp/no-tee-live.out 2>&1
```

Earlier attempts that piped the long live run through `tee` inside a managed background command exited before receipt completion. Use direct redirection for future long live entrypoint runs unless the runner grows first-class log-file support.
