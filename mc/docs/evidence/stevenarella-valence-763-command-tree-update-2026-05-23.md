# Stevenarella / Valence protocol 763 command-tree update — 2026-05-23

Receipt status: `pass`
Receipt BLAKE3: `c7ec4f1d98c7af0ecc01bd235828cdd63831af88485c68100e08e33f4eb5c0eb`

## What changed
- Stevenarella fork commit: `119f7d52dbce8a690de8935ec69dd5c870cbdd81`
- Fork remote: `git@github.com:brittonr/stevenarella.git` (`master`)
- Protocol `763` now maps `play/clientbound/0x10` to Stevenarella `DeclareCommands`.
- This replaces the inherited protocol-758 interpretation of `0x10` as `ClearTitles` for the protocol-763 path.

## Evidence
- Valence target: protocol `763` / Minecraft `1.20.1`, `ctf` example in offline mode.
- Direct trace artifact: `/tmp/valence-763-after-command-tree-update-trace.json`
- Direct trace BLAKE3: `f4e4451ddb1aecfe7958044f78e72880292176b04316f93102e992e7671a338c`
- Stevenarella probe log: `/tmp/stevenarella-763-after-command-tree-update.log`
- Stevenarella probe log BLAKE3: `5edcd9614681f17c8a5cf84f18fc5056a55e13a67f5b94a911669c16622b56c6`
- Probe classification: timeout-success evidence after matching `Detected server protocol version 763`.

## Packet boundary result
Captured sequence after login:

1. `login/clientbound/0x03` — `SetCompressionS2C`
2. `login/clientbound/0x02` — `LoginSuccessS2C`
3. `play/clientbound/0x28` — prior mapped boundary: `GameJoinS2C` → `JoinGame_WorldNames_IsHard_SimDist`
4. `play/clientbound/0x10` — updated boundary: `CommandTreeS2CPacket` → `DeclareCommands`
5. `play/clientbound/0x64` — next unpatched mismatch: Valence `GameMessageS2CPacket`, inherited Stevenarella 758 table `EntityProperties_VarIntVarInt`.

## Verification
- `cargo fmt --check`: pass
- `cargo test -p steven_protocol protocol::versions::tests -- --nocapture`: pass (`6 passed`)
- `git diff --check`: pass before Stevenarella commit

## Contract
This evidence claims the protocol-763 `0x10` command-tree packet-id mapping is now updated in the Stevenarella fork. It does **not** claim full 1.20.1 client compatibility. The next focused update is `play/clientbound/0x64` (`GameMessageS2CPacket` vs inherited `EntityProperties_VarIntVarInt`).
