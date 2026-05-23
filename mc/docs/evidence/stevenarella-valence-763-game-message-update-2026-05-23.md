# Stevenarella / Valence protocol 763 game-message update — 2026-05-23

Receipt status: `pass`
Receipt BLAKE3: `6578b9bd647ca3150e924c012a998181f84bddd10a8054ef498a93d42cd21bee`

## What changed

Updated the Stevenarella fork's incremental protocol `763` translator for `play/clientbound/0x64`:

- Valence 763 semantic: `GameMessageS2CPacket`
- Stevenarella 763 internal packet: `ServerMessage_Position`
- Previous inherited 758 alias: `EntityProperties_VarIntVarInt`

Prior protocol-763 mappings remain covered:

- `play/clientbound/0x28`: `GameJoinS2CPacket` → `JoinGame_WorldNames_IsHard_SimDist`
- `play/clientbound/0x10`: `CommandTreeS2CPacket` → `DeclareCommands`

Stevenarella fork commit: `cd9b59d3de0c8fd08a04a72fcd9439b678480ce0` (`Add protocol 763 game message mapping`).

## Trace result

A direct offline Valence `ctf` trace for protocol `763` observed login packets `0x03`, `0x02`, then play packets beginning:

`0x28`, `0x10`, `0x64`, `0x69`, `0x58`, `0x51`

After the `0x64` update, the next boundary is `play/clientbound/0x69`:

- Valence 763 semantic: `AdvancementUpdateS2CPacket`
- Inherited Stevenarella 758 alias: no mapping / fallback internal `null`
- Status: `unmapped_by_758_fallback`

Trace artifact BLAKE3: `6fa76c598e7eae48d0d7a0e6146bae8fa5f319bbb97de5833d1548c76e9725d2`

## Verification

- `cargo fmt --check`: `pass`
- `cargo test -p steven_protocol protocol::versions::tests -- --nocapture`: `pass` (`8` tests)
- Headless Stevenarella probe matched: `Detected server protocol version 763`
- Probe log BLAKE3: `9950c7806dfc913d02f8415a1621afa87fe29f0a01280a3e56d62b5787ac572f`

## Contract / non-claims

This evidence claims the narrow protocol `763` game-message ID mapping only. It does **not** claim full current Valence client compatibility, full Stevenarella protocol `763` support, or semantic correctness of all decoded packet fields.
