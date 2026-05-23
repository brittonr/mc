# Stevenarella ↔ Valence protocol 763 entity metadata evidence — 2026-05-23

## Scope

This records the next bounded probe after the post-login play-state evidence identified an `EntityMetadata` metadata parser panic (`FromUtf8Error`). The change is part of updating our Stevenarella fork for Valence/Minecraft `1.20.1` / protocol `763`.

## Changed

- Added a protocol 763 / 1.20.1 metadata value-type decoder branch for `EntityMetadata`.
- Added protocol 763 packet-shape/mapping fixes encountered while proving the metadata boundary:
  - `0x23` clientbound keep-alive → `KeepAliveClientbound_i64`.
  - `0x24` chunk data without the older `trust_edges` field.
  - `0x3c` player position/look teleport shape.
  - `0x50` player spawn position with angle.
  - `0x12` serverbound keep-alive response.
- Stevenarella fork commit: `b2a6358`.

## Verification

Focused Stevenarella checks passed:

```sh
cargo fmt && cargo fmt --check &&   CARGO_TARGET_DIR=/tmp/stevenarella-target2 cargo test -p steven_protocol protocol::versions::tests -- --nocapture &&   CARGO_TARGET_DIR=/tmp/stevenarella-target2 cargo check -p stevenarella
```

Live Valence `ctf` probe was bounded with `timeout 120s`; status: `exit=124`.

Probe counts:

- `Detected server protocol version 763`: 1
- `MC-COMPAT-MILESTONE login_compression`: 1
- `MC-COMPAT-MILESTONE login_success`: 1
- `MC-COMPAT-MILESTONE join_game_763_shape`: 1
- `MC-COMPAT-MILESTONE join_game`: 2
- `MC-COMPAT-MILESTONE first_chunk_data`: 1
- `MC-COMPAT-MILESTONE render_tick_with_player`: 1
- `FromUtf8Error`: 0
- `UnexpectedEof`: 0
- `panicked at`: 0
- `failed to parse packet`: 0

## What this proves

- The prior `EntityMetadata` `FromUtf8Error` boundary did not recur in this bounded probe.
- The probe reached login compression, login success, join game, first chunk data, and `render_tick_with_player`.
- The probe ended by timeout (`exit=124`) rather than a logged parser panic.

## What this does not prove

- Does not prove full Minecraft 1.20.1 compatibility.
- Does not prove full Stevenarella protocol 763 support.
- Does not prove semantic correctness of every decoded packet.
- Does not prove stable in-world gameplay beyond this bounded probe.

Receipt BLAKE3: `72545f60abeb0a0d46fdb9859486c7924631498689fef60b56271a638ff2db59`
