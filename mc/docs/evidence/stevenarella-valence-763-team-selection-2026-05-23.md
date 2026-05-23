# Stevenarella → Valence protocol 763 team-selection probe (2026-05-23)

## Summary

A bounded 180-second headless Stevenarella probe against Valence `ctf` now observes the server-side team-selection chat message.

- Stevenarella fork commit: `4c891eb stevenarella: prove 763 ctf team selection`
- Valence example: `ctf`
- Protocol/version: `763` / Minecraft `1.20.1`
- Probe env: `MC_COMPAT_ACTIVE_PROBE=1 MC_COMPAT_TEAM_PROBE=1`
- Probe status: `exit=124` (bounded timeout)
- Team-selection evidence: `Received chat message: You are on team RED!`
- Runtime failures in this probe: none logged (`UnexpectedEof`, `FromUtf8Error`, parse failure, short packet read, bad packet id, panic, disconnect all `0`)

## What changed

The previous team-interaction probe sent the red-portal position/hold/use-item packets but did not observe `You are on team`. A temporary local Valence trace confirmed the server can see the client inside the portal area and that team selection fires, exposing follow-on protocol 763 packet boundaries.

The Stevenarella fork update adds narrow 763 clientbound seams needed after team selection:

- `0x01` `EntitySpawnS2C` → `SpawnObject_VarInt_HeadYaw` (new 1.20.1 object-spawn shape)
- `0x02` `ExperienceOrbSpawnS2C` → `SpawnExperienceOrb`
- `0x03` `PlayerSpawnS2C` → `SpawnPlayer_f64_NoMeta`
- `0x04` `EntityAnimationS2C` → `Animation`
- `0x2b`/`0x2c`/`0x2d`/`0x2e` movement/vehicle packets
- `0x42` head yaw
- `0x53`-`0x56` attach/velocity/equipment/experience packets
- `0x67` item pickup animation
- `0x68` entity position teleport-style shape

## Evidence counts

From `/tmp/stevenarella-763-team-semantic-2026-05-23.log`:

- `Detected server protocol version 763`: `1`
- `MC-COMPAT-MILESTONE login_success`: `1`
- `MC-COMPAT-MILESTONE join_game_763_shape`: `1`
- `MC-COMPAT-MILESTONE first_chunk_data`: `1`
- `MC-COMPAT-MILESTONE render_tick_with_player`: `1`
- `MC-COMPAT-MILESTONE active_probe_position_look_sent`: `1`
- `MC-COMPAT-MILESTONE team_probe_enter_red_portal`: `1`
- `MC-COMPAT-MILESTONE team_probe_hold_red_portal`: `1`
- `MC-COMPAT-MILESTONE team_probe_select_hotbar_slot`: `1`
- `MC-COMPAT-MILESTONE team_probe_use_item_sent`: `1`
- `You are on team`: `1`
- `You are on team RED`: `1`
- `UnexpectedEof`: `0`
- `FromUtf8Error`: `0`
- `failed to parse packet`: `0`
- `Failed to read all of packet`: `0`
- `bad packet id`: `0`
- `panicked at`: `0`
- `Disconnect`: `0`

## Hashes

- Probe log BLAKE3: `b58a72276133d132fb7d8d1d2b090b7a42070c1956ebb9802b77eb4f64cb476e`
- Probe status BLAKE3: `f67d19a0fed77375c9ee600f70ff255957e708d6c0898a270d40151761401f0a`
- Receipt BLAKE3: `56c6154037c1820b7f876cd56a110c8e3babbea8d8122844778111dff4154563`

## Verification

Focused Stevenarella gate passed:

```sh
nix develop path:/home/brittonr/git/mc -c bash -lc 'cargo fmt --check && CARGO_TARGET_DIR=/tmp/stevenarella-target2 cargo test -p steven_protocol protocol::versions::tests -- --nocapture && CARGO_TARGET_DIR=/tmp/stevenarella-target2 cargo check -p stevenarella'
```

The focused protocol test run reported 15 passing tests.

## Non-claims

This evidence proves a bounded team-selection chat observation for the Valence `ctf` red portal in this probe. It does **not** prove full Minecraft 1.20.1 compatibility, full Stevenarella protocol 763 coverage, stable long-term gameplay, complete semantic decode correctness, combat, inventory semantics, death/respawn, reconnect/session behavior, or broad Valence client compatibility.
