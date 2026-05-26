# Valence CTF flag-carrier death/return compatibility evidence

Date: 2026-05-26

## Scope

Bounded protocol-763 Stevenarella ⇄ Valence CTF evidence for a two-client flag-carrier death edge:

- `compatbota` joins RED and attacks.
- `compatbotb` joins BLUE, picks up the RED flag, dies before capture, sends respawn, and observes restored health.
- Valence correlates both clients, flag pickup, lethal flag-carrier transition, and flag return/reset.

This receipt does **not** claim full CTF correctness, broad Minecraft compatibility, unbounded soak behavior, or production load behavior.

## Maintained rail

```sh
nix run .#mc-compat-valence-ctf-flag-carrier-death-return
nix run .#mc-compat-valence-ctf-flag-carrier-death-return -- --dry-run
```

The dry-run gate is:

```sh
nix build path:/home/brittonr/git/mc#checks.x86_64-linux.mc-compat-valence-ctf-flag-carrier-death-return-dry-run --no-link -L
```

## Live receipt

- Receipt: `target/mc-compat-flag-carrier-death/flag-carrier-death-return.json`
- BLAKE3: `d4202d7f04245dd385f16f9a174b84fa59a837fd75a8f9ba7db3cc7adaf692a4`
- Command: `nix run .#mc-compat-valence-ctf-flag-carrier-death-return`
- Status: `pass`

Observed client milestones included:

- `multi_client_count`
- `protocol_detected`
- `join_game`
- `render_tick`
- `team_red`
- `team_blue`
- `flag_pickup`
- `remote_player_spawn`
- `combat_attack_sent`
- `combat_death_observed`
- `respawn_request_sent`
- `respawn_health_restored`

Observed server milestones included:

- `server_client_a_seen`
- `server_client_b_seen`
- `server_flag_pickup`
- `server_flag_carrier_death`
- `server_flag_return`

Forbidden accidental-success patterns were clean:

- `unexpected_flag_capture`
- `unexpected_flag_capture_milestone`
- `unexpected_red_score`
- `unexpected_blue_score`

## Verification

- Runner unit tests: `28 passed; 0 failed`
- Valence compile gate: `cargo +nightly check --example ctf` passed
- Stevenarella compile gate: `cargo check` passed in the repository dev shell
- Dry-run Nix gate: `mc-compat-valence-ctf-flag-carrier-death-return-dry-run` passed
