# Stevenarella ⇄ Valence protocol-763 BLUE flag-score 600s soak — 2026-05-25

## Scope

This records one bounded `600s` live run of the maintained BLUE-team Valence `ctf` soak entrypoint against the owned local Valence fixture.

The run used the maintained flake app added for this scenario:

```sh
MC_COMPAT_BLUE_SOAK_RECEIPT=target/mc-compat-blue-soak/blue-flag-score-600s.json \
  nix run .#mc-compat-valence-ctf-blue-600s-soak \
  > target/mc-compat-blue-soak/blue-flag-score-600s.run.log 2>&1
```

The wrapper fixes the intended default shape unless explicitly overridden:

- `SERVER_PROTOCOL=763`
- `SERVER_VERSION=1.20.1`
- `VALENCE_REV=main`
- `VALENCE_EXAMPLE=ctf`
- `CLIENT_TIMEOUT=600`
- scenario: `blue-flag-score`

## Result

The receipt passed.

```text
receipt: target/mc-compat-blue-soak/blue-flag-score-600s.json
BLAKE3:  b7c861f27ef7ceaf94705a74a5459d3f9df625dada4b14d8715ba8e9c5d921de
status:  pass
client exit: 124
classification: timeout-success-evidence
```

Observed client milestones:

```text
protocol_detected
join_game
render_tick
team_blue
flag_pickup
flag_capture
score_blue_1
```

Observed server milestones:

```text
server_username_seen
server_flag_or_score
```

The receipt also recorded:

```text
duration_secs=600
timeout_secs=600
server.version=1.20.1
server.protocol=763
client.log_path=/tmp/mc-compat-client.compatbot.1779723727754.log
server.log_path=/tmp/mc-compat-valence.log
```

A hygiene scan across the run log, client log, and Valence log found zero matches for:

```text
panic | parser | parse | disconnect | decode_error | protocol_mismatch
```

Port cleanup after the run showed no listener on `:25565`.

## Maintained rail

This slice also adds a deterministic dry-run check for the maintained BLUE soak command shape:

```sh
nix build path:/home/brittonr/git/mc#checks.x86_64-linux.mc-compat-valence-ctf-blue-600s-soak-dry-run --no-link -L
```

The check asserts the dry-run receipt keeps the scenario, protocol/version, timeout/duration, BLUE-team milestones, expected packet-summary surface, and server correlation fields.

## Non-claims

This strengthens the mirrored BLUE scoring path from repeatable short runs to one bounded long run. It still does not claim:

- full CTF semantic correctness;
- full inventory/combat/drop/pickup semantics;
- broad Minecraft 1.20.1 compatibility;
- public-server or production-load safety;
- unbounded long-run stability;
- complete client/server packet coverage.
