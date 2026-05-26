# Stevenarella ⇄ Valence protocol-763 combat/damage receipt (2026-05-25)

## Scope

Bounded two-client Stevenarella probe against the local Valence `ctf` example, protocol `763` / Minecraft `1.20.1`.

This slice proves a maintained combat/damage semantic rail beyond prior team, scoring, respawn, inventory, and open-container evidence. It requires both clients to connect to one owned local Valence server, select opposing teams, observe a remote player, send bounded attack packets, observe victim-side health updates, and correlate the server-side Valence `combat_damage` milestone for the two expected usernames.

## Commands

Live maintained command:

```sh
MC_COMPAT_COMBAT_RECEIPT=target/mc-compat-combat/combat-damage.json \
  nix run .#mc-compat-valence-ctf-combat-damage
```

Dry-run check:

```sh
nix build path:/home/brittonr/git/mc#checks.x86_64-linux.mc-compat-valence-ctf-combat-damage-dry-run --no-link -L
```

Focused checks also run:

```sh
cd tools/mc-compat-runner
nix shell nixpkgs#rustfmt nixpkgs#cargo nixpkgs#rustc nixpkgs#gcc -c bash -lc 'rustfmt src/main.rs && cargo test'

cd valence
nix shell nixpkgs#rustup nixpkgs#gcc nixpkgs#pkg-config nixpkgs#openssl nixpkgs#fontconfig nixpkgs#freetype nixpkgs#libxcb nixpkgs#libxkbcommon nixpkgs#wayland nixpkgs#libGL \
  -c bash -lc 'RUSTC_WRAPPER= CARGO_TARGET_DIR=/tmp/mc-valence-combat-target cargo +nightly fmt -- examples/ctf.rs && RUSTC_WRAPPER= CARGO_TARGET_DIR=/tmp/mc-valence-combat-target cargo +nightly check --example ctf'

nix develop path:/home/brittonr/git/mc -c bash -lc \
  'cd stevenarella && CMAKE_POLICY_VERSION_MINIMUM=3.5 cargo fmt -- src/server/mod.rs && RUSTC_WRAPPER= CARGO_TARGET_DIR=/tmp/stevenarella-combat-target CMAKE_POLICY_VERSION_MINIMUM=3.5 cargo check'
```

## Live receipt

- Receipt: `target/mc-compat-combat/combat-damage.json`
- BLAKE3: `b67962dd5d4fe7242b69fd7c879390e80e13528475d55d7feb5305289f762ac8`
- Stevenarella fork commit: `2447d8e Delay combat probe until victim team selection`
- Valence fork commit: `3835dab Log combat damage compatibility milestones`
- Parent evidence commit: the commit carrying this note
- Status: `pass`
- Server: Valence `ctf`, protocol `763`, `1.20.1`
- Clients: `compatbota`, `compatbotb`
- Live log: `target-mc-compat-combat-live.log`
- Server log: `/tmp/mc-compat-valence.log`

Receipt-required client milestones:

```json
["multi_client_count", "protocol_detected", "join_game", "render_tick", "team_red", "team_blue", "remote_player_spawn", "combat_attack_sent", "combat_health_update"]
```

Receipt-required server milestones:

```json
["server_client_a_seen", "server_client_b_seen", "server_combat_damage"]
```

Packet summary buckets required by the scenario:

```json
["two_client_login", "play_join_game", "use_entity_attack"]
```

Representative client milestones:

```text
MC-COMPAT-MILESTONE team_probe_enter_red_portal x=-4.0 y=84.0 z=4.0
MC-COMPAT-MILESTONE team_probe_enter_blue_portal x=4.0 y=84.0 z=4.0
MC-COMPAT-MILESTONE remote_player_spawn entity_id=8 ... name=compatbotb x=40.000 y=64.536 z=0.000
MC-COMPAT-MILESTONE combat_probe_attack_sent target_id=4 count=1
MC-COMPAT-MILESTONE update_health health=16.0 food=20 saturation=0.0
MC-COMPAT-MILESTONE update_health health=12.0 food=20 saturation=0.0
MC-COMPAT-MILESTONE update_health health=8.0 food=20 saturation=0.0
MC-COMPAT-MILESTONE update_health health=4.0 food=20 saturation=0.0
MC-COMPAT-MILESTONE update_health health=0.0 food=20 saturation=0.0
```

Representative Valence server milestone:

```text
MC-COMPAT-MILESTONE combat_damage attacker=compatbota victim=compatbotb damage=4.0 victim_health_before=20.0 victim_health_after=16.0 attacker_item=WoodenSword
```

## Hygiene

The live client/server logs were scanned for:

```text
panic
unexpected_eof
protocol_mismatch
decode_error
```

No matches were found. Port `25565` was clear after the run.

## What this proves

- Two Stevenarella protocol-763 clients can join one local Valence `ctf` server under the maintained harness.
- Both clients can reach opposing team-selection milestones in the same run.
- The attacker client observes a remote player and sends bounded `UseEntity` attack traffic.
- The victim client receives combat-induced health updates down through death health.
- Valence correlates the attack server-side as `combat_damage attacker=compatbota victim=compatbotb` with item and before/after health values.
- The maintained dry-run gate checks the command shape and required receipt fields without side effects.

## Non-claims

This is still not proof of:

- full Minecraft `1.20.1` combat correctness,
- knockback/armor/projectile/enchantment semantics,
- full death/respawn lifecycle correctness in this maintained combat rail,
- production load safety,
- public-server stress authorization,
- unbounded soak stability,
- broad protocol coverage outside the observed CTF combat path.
