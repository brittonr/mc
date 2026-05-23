# Stevenarella / Valence protocol 763 combat/death probe — 2026-05-23

## Scope

Bounded two-client headless Stevenarella probe against the Valence `ctf` example using protocol 763 / Minecraft 1.20.1.

This records a narrow combat/death semantic proof after prior team-selection proof. It does **not** prove full Minecraft 1.20.1 compatibility, full Stevenarella protocol 763 support, stable gameplay, full combat correctness, inventory correctness, respawn semantics, reconnect behavior, or long-term soak.

## Setup

- Valence example: `ctf`
- Valence protocol: `763`
- Connection mode: offline
- Stevenarella commit: `2804c81`
- Victim client: `CombatBlue2`, `MC_COMPAT_ACTIVE_PROBE=1 MC_COMPAT_TEAM_PROBE=1 MC_COMPAT_TEAM_PROBE_TEAM=blue MC_COMPAT_COMBAT_PROBE=1 MC_COMPAT_COMBAT_PROBE_ROLE=victim`
- Attacker client: `CombatRed2`, `MC_COMPAT_ACTIVE_PROBE=1 MC_COMPAT_TEAM_PROBE=1 MC_COMPAT_TEAM_PROBE_TEAM=red MC_COMPAT_COMBAT_PROBE=1 MC_COMPAT_COMBAT_PROBE_ROLE=attacker`
- Bound: `timeout 180s` for each client

## Result

The bounded probe observed:

- both clients detected server protocol version 763;
- both clients reached login success, 1.20.1 join-game shape, first chunk data, and render tick with player;
- blue victim selected the BLUE team (`You are on team BLUE`);
- red attacker selected the RED team (`You are on team RED`);
- red attacker observed remote player spawns, moved near the blue spawn, and sent six `UseEntity` attack packets;
- blue victim observed health updates including `update_health health=16.0`, `update_health health=12.0`, `update_health health=8.0`, `update_health health=4.0`, and `update_health health=0.0`;
- blue victim logged `MC-COMPAT-MILESTONE combat_probe_death_observed` twice;
- no `UnexpectedEof`, `FromUtf8Error`, panic, bad packet id, short read, failed packet parse, or disconnect marker was logged in either probe log.

Observed next boundary: no death-message semantic was observed in this probe (`combat_probe_death_message = 0`), and no respawn semantic was claimed.

## Artifact hashes

- Red attacker log `/tmp/stevenarella-763-combat-red-2026-05-23.log`: `4e223dd9243508d1dfca0a7def6d5de350154f0ba9befec83849cf8b1fed77e5`
- Red attacker status `/tmp/stevenarella-763-combat-red-2026-05-23.status`: `f67d19a0fed77375c9ee600f70ff255957e708d6c0898a270d40151761401f0a`
- Blue victim log `/tmp/stevenarella-763-combat-blue-2026-05-23.log`: `1e68587edce7e2353dfb4f5d4276f6144417de68228b5c17b9ffbd270959aaa6`
- Blue victim status `/tmp/stevenarella-763-combat-blue-2026-05-23.status`: `f67d19a0fed77375c9ee600f70ff255957e708d6c0898a270d40151761401f0a`

## Verification

Focused Stevenarella verification exited 0:

```sh
nix develop path:/home/brittonr/git/mc -c bash -lc 'cargo fmt --check && CARGO_TARGET_DIR=/tmp/stevenarella-target2 cargo test -p steven_protocol protocol::versions::tests -- --nocapture && CARGO_TARGET_DIR=/tmp/stevenarella-target2 cargo check -p stevenarella'
```

## Non-claims

This evidence claims only bounded Valence `ctf` two-client combat/death health semantics for the recorded run. It does not claim full protocol support, stable gameplay, full combat correctness, inventory correctness, respawn behavior, reconnect behavior, or long-term soak.

Receipt BLAKE3: `51060b62454eae5c878dbfcdd2899d457f49eb3d6496a7a72e173e17b157c15d`
