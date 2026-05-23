# Proposal: EntityStatusS2CPacket protocol 763 boundary

## Why

Protocol 763 Stevenarella currently advances past handshake, join-game, command-tree, and game-message seams, then the Valence ctf trace exposes `0x1c` as `EntityStatusS2CPacket`. The inherited 758 fallback is inherited 758 Explosion_VarInt, so this boundary needs its own reviewed mapping before claiming broader current-Valence compatibility.

## What Changes

- Inspect Stevenarella packet DTOs and Valence protocol 763 packet shape for `EntityStatusS2CPacket`.
- Add the narrow protocol 763 translator override for `play/clientbound/0x1c` only when a compatible internal packet shape exists; otherwise add the minimal internal DTO/parser needed for this boundary.
- Add regression tests proving `0x1c` maps to the reviewed internal semantic and no longer resolves through the inherited 758 fallback.
- Rerun the Valence `ctf` trace/probe and record the next boundary as evidence without claiming full protocol 763 support.

## Impact

- **Files**: `stevenarella/protocol/src/protocol/versions/v1_20_1.rs`, `stevenarella/protocol/src/protocol/versions.rs`, and possibly `stevenarella/protocol/src/protocol/packet.rs` if no compatible internal packet exists.
- **Testing**: `cargo fmt --check`, focused `steven_protocol` version tests, Valence `ctf` trace/probe, and a parent `mc` evidence receipt/check.
