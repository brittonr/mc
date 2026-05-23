# Stevenarella / Valence 763 post-login play-state evidence (2026-05-23)

Receipt BLAKE3: `c202e791cd7638fb4ae2ba6237f8ac8bc81deacbd9438ae3c3d9384b86d98582`

## What changed

Updated our Stevenarella fork at `51d22f2` to move protocol 763 past the prior post-`login_success` boundary:

- `play/clientbound/0x28` now uses `JoinGame_WorldNames_IsHard_SimDist_LastDeath_PortalCooldown` for the 1.20.1 join-game packet shape.
- `play/clientbound/0x3a` now uses `PlayerInfo_BitSet` for the 1.20.1 player-info action-bitset packet shape.

## Probe result

Live Valence `ctf` probe under `xvfb-run` reached these milestones:

- `Detected server protocol version 763`: 1
- `MC-COMPAT-MILESTONE login_compression`: 1
- `MC-COMPAT-MILESTONE login_success`: 1
- `MC-COMPAT-MILESTONE join_game_763_shape`: 1
- `MC-COMPAT-MILESTONE join_game`: 2
- `MC-COMPAT-MILESTONE render_tick_with_player`: 1
- `UnexpectedEof`: 0

The prior concrete failure, `UnexpectedEof` after `login_success`, was not observed in this bounded probe.

## New boundary

The next observed runtime boundary is an EntityMetadata metadata parser panic:

```text
panic in EntityMetadata metadata string parser: FromUtf8Error at protocol/src/protocol/mod.rs:281
```

Backtrace attributes this to `Metadata::read_from` during `Conn::read_packet`, so the next ROI is protocol 763 EntityMetadata metadata shape/parser compatibility.

## Verification

- Focused tests/check: `cargo fmt --check`, `cargo test -p steven_protocol protocol::versions::tests -- --nocapture`, and `cargo check -p stevenarella` exited 0.
- Live probe status: `exit=124` (bounded timeout after reaching milestones).
- Backtrace probe status: `exit=124`.

## Non-claims

- Does not prove full Minecraft 1.20.1 compatibility.
- Does not prove full Valence client compatibility.
- Does not prove semantic packet parser correctness.
- Does not prove chunk streaming or stable in-world gameplay success.
