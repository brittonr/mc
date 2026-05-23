# Stevenarella ↔ Valence protocol 763 extended gameplay probe — 2026-05-23

## Scope

This records a longer bounded Stevenarella → Valence `ctf` probe after the protocol 763 `EntityMetadata` parser work. No code changed in this slice; the goal was to continue through the compatibility effort by looking for the next concrete runtime boundary.

## Baseline

- Stevenarella fork commit: `b2a6358`.
- Parent evidence baseline gate: `stevenarella-valence-763-entity-metadata-evidence`.
- Valence example: `ctf`, protocol `763` / Minecraft `1.20.1`.

## Probe

Command shape:

```sh
timeout 300s xvfb-run -a env   RUSTC_WRAPPER=   CARGO_TARGET_DIR=/tmp/stevenarella-target2   LIBGL_ALWAYS_SOFTWARE=1   RUST_BACKTRACE=1   cargo run -- --server 127.0.0.1
```

Status: `exit=124`.

Probe counts:

- `Detected server protocol version 763`: 1
- `MC-COMPAT-MILESTONE login_compression`: 1
- `MC-COMPAT-MILESTONE login_success`: 1
- `MC-COMPAT-MILESTONE join_game_763_shape`: 1
- `MC-COMPAT-MILESTONE join_game`: 2
- `MC-COMPAT-MILESTONE first_chunk_data`: 1
- `MC-COMPAT-MILESTONE render_tick_with_player`: 1
- `UnexpectedEof`: 0
- `FromUtf8Error`: 0
- `failed to parse packet`: 0
- `panicked at`: 0
- `Disconnect`: 0

## What this proves

- The client survived a bounded 300-second Valence `ctf` probe without logged parser failure, panic, EOF failure, or disconnect.
- Protocol 763 detection, login success, join-game, first chunk data, and `render_tick_with_player` were reached.
- No next concrete runtime boundary appeared in this longer passive probe.

## What this does not prove

- Does not prove full Minecraft 1.20.1 compatibility.
- Does not prove full Stevenarella protocol 763 support.
- Does not prove semantic correctness of every decoded packet.
- Does not prove stable in-world gameplay beyond this bounded passive probe.
- Does not exercise active player input, team selection, combat, chunk traversal, inventory interactions, or reconnect/session behavior.

Receipt BLAKE3: `da6abd016dea91a5417d6f1bb092b91c12d2bf686ed9de1c535cabf1570f80ab`
