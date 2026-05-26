# Stevenarella ⇄ Valence protocol-763 combat/knockback receipt (2026-05-26)

## Scope

Bounded two-client Stevenarella probe against the local Valence `ctf` example, protocol `763` / Minecraft `1.20.1`.

This slice extends the maintained combat rail from damage/health into knockback. It requires both clients to connect to one owned local Valence server, select opposing teams, observe a remote player, send bounded attack packets, observe victim-side health and non-zero velocity updates, and correlate Valence `combat_damage` plus `combat_knockback` server milestones for the two expected usernames.

## Commands

Live maintained command:

```sh
nix run .#mc-compat-valence-ctf-combat-knockback
```

Dry-run check:

```sh
nix build path:/home/brittonr/git/mc#checks.x86_64-linux.mc-compat-valence-ctf-combat-knockback-dry-run --no-link -L
```

Focused checks also run:

```sh
cd tools/mc-compat-runner
nix shell nixpkgs#cargo nixpkgs#rustc nixpkgs#gcc nixpkgs#rustfmt -c bash -lc 'cargo fmt && cargo test'

cd valence
nix shell nixpkgs#rustup nixpkgs#gcc nixpkgs#pkg-config nixpkgs#openssl nixpkgs#fontconfig nixpkgs#freetype nixpkgs#libxcb nixpkgs#libxkbcommon nixpkgs#wayland nixpkgs#libGL \
  -c bash -lc 'RUSTC_WRAPPER= CARGO_TARGET_DIR=/tmp/mc-valence-knockback-target cargo +nightly check --example ctf'

nix develop path:/home/brittonr/git/mc -c bash -lc \
  'cd stevenarella && RUSTC_WRAPPER= cargo fmt -- src/server/mod.rs && RUSTC_WRAPPER= CARGO_TARGET_DIR=/tmp/stevenarella-knockback-target CMAKE_POLICY_VERSION_MINIMUM=3.5 cargo check'
```

## Live receipt

- Receipt: `target/mc-compat-knockback/combat-knockback.json`
- BLAKE3: `a5d0ba5ea6155a99b58f245a03195da05b4925d7bd151b5b3f67503ae7a4cf09`
- Stevenarella fork commit: `3884f0a Observe combat velocity updates`
- Valence fork commit: `e401e76 Log combat knockback milestones`
- Parent evidence commit: the commit carrying this note
- Status: `pass`
- Server: Valence `ctf`, protocol `763`, `1.20.1`
- Clients: `compatbota`, `compatbotb`
- Server log: `/tmp/mc-compat-valence.log`
- Client logs: `/tmp/mc-compat-client.compatbota.1779805961665.log`, `/tmp/mc-compat-client.compatbotb.1779805963666.log`

Receipt-required client milestones:

```json
["multi_client_count", "protocol_detected", "join_game", "render_tick", "team_red", "team_blue", "remote_player_spawn", "combat_attack_sent", "combat_health_update", "combat_velocity_update"]
```

Receipt-required server milestones:

```json
["server_client_a_seen", "server_client_b_seen", "server_combat_damage", "server_combat_knockback"]
```

Packet summary buckets required by the scenario:

```json
["two_client_login", "play_join_game", "use_entity_attack", "entity_velocity"]
```

Representative client milestones:

```text
MC-COMPAT-MILESTONE combat_probe_velocity_observed entity_id=0 vx=3200 vy=2572 vz=0
MC-COMPAT-MILESTONE update_health health=16.0 food=20 saturation=0.0
MC-COMPAT-MILESTONE update_health health=12.0 food=20 saturation=0.0
MC-COMPAT-MILESTONE update_health health=8.0 food=20 saturation=0.0
MC-COMPAT-MILESTONE update_health health=4.0 food=20 saturation=0.0
MC-COMPAT-MILESTONE update_health health=0.0 food=20 saturation=0.0
```

Representative Valence server milestones:

```text
MC-COMPAT-MILESTONE combat_knockback attacker=compatbota victim=compatbotb vx=8.000 vy=6.432 vz=0.000 bonus=false
MC-COMPAT-MILESTONE combat_damage attacker=compatbota victim=compatbotb damage=4.0 victim_health_before=20.0 victim_health_after=16.0 attacker_item=WoodenSword
```

## Hygiene

The live client/server logs from the receipt were scanned for:

```text
panic
UnexpectedEof
protocol mismatch
decode error
Decode
thread .* panicked
```

No matches were found in the current receipt logs.

## What this proves

- Two Stevenarella protocol-763 clients can join one local Valence `ctf` server under the maintained harness.
- Both clients can reach opposing team-selection milestones in the same run.
- The attacker client observes a remote player and sends bounded `UseEntity` attack traffic.
- The victim client receives combat-induced health updates and non-zero `EntityVelocity` evidence.
- Valence correlates the attack server-side as both `combat_damage` and `combat_knockback attacker=compatbota victim=compatbotb` with deterministic velocity components.
- The maintained dry-run gate checks the command shape and required two-client receipt fields without side effects.

## Non-claims

This is still not proof of:

- full Minecraft `1.20.1` combat correctness,
- projectile mechanics,
- armor/enchantment mitigation semantics,
- exact vanilla knockback balancing,
- full death/respawn lifecycle correctness in this maintained combat rail,
- production load safety,
- public-server stress authorization,
- unbounded soak stability,
- broad protocol coverage outside the observed CTF combat path.
