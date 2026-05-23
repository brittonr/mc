# Stevenarella protocol 763 observed boundary drain — 2026-05-23

Receipt BLAKE3: `cc62a78238134a339205abaea4c003f82e89597cc89ce9e11c620274974886a2`

Stevenarella fork commit: `4c5e89d` (`git@github.com:brittonr/stevenarella.git` / `master`).

This drains the 14 Cairn-planned, Valence `ctf` observed protocol-763 play/clientbound packet ID boundaries by mapping each wire ID to the closest existing Stevenarella internal packet shape. It does **not** claim full current-Valence client compatibility, full Stevenarella 763 support, semantic correctness, or coverage of every Minecraft 1.20.1 packet.

## Newly mapped boundaries

- `play/clientbound/0x69`: Valence `AdvancementUpdateS2CPacket` → Stevenarella internal `Advancements`
- `play/clientbound/0x58`: Valence `ScoreboardObjectiveUpdateS2CPacket` → Stevenarella internal `ScoreboardObjective`
- `play/clientbound/0x51`: Valence `ScoreboardDisplayS2CPacket` → Stevenarella internal `ScoreboardDisplay`
- `play/clientbound/0x5b`: Valence `ScoreboardPlayerUpdateS2CPacket` → Stevenarella internal `UpdateScore_VarInt`
- `play/clientbound/0x4d`: Valence `UpdateSelectedSlotS2CPacket` → Stevenarella internal `SetCurrentHotbarSlot`
- `play/clientbound/0x14`: Valence `ScreenHandlerSlotUpdateS2CPacket` → Stevenarella internal `WindowSetSlot_State`
- `play/clientbound/0x3a`: Valence `PlayerListS2CPacket` → Stevenarella internal `PlayerInfo`
- `play/clientbound/0x57`: Valence `HealthUpdateS2CPacket` → Stevenarella internal `UpdateHealth`
- `play/clientbound/0x52`: Valence `EntityTrackerUpdateS2CPacket` → Stevenarella internal `EntityMetadata`
- `play/clientbound/0x6a`: Valence `EntityAttributesS2CPacket` → Stevenarella internal `EntityProperties_VarIntVarInt`
- `play/clientbound/0x1c`: Valence `EntityStatusS2CPacket` → Stevenarella internal `EntityStatus`
- `play/clientbound/0x34`: Valence `PlayerAbilitiesS2CPacket` → Stevenarella internal `PlayerAbilities`
- `play/clientbound/0x6e`: Valence `SynchronizeTagsS2CPacket` → Stevenarella internal `Tags_Nested`
- `play/clientbound/0x24`: Valence `ChunkDataS2CPacket` → Stevenarella internal `ChunkData_AndLight`

## Prior mappings still covered

- `play/clientbound/0x28`: `GameJoinS2C` → `JoinGame_WorldNames_IsHard_SimDist`
- `play/clientbound/0x10`: `CommandTreeS2CPacket` → `DeclareCommands`
- `play/clientbound/0x64`: `GameMessageS2CPacket` → `ServerMessage_Position`

## Verification

- `cargo fmt --check`: pass
- `cargo test -p steven_protocol protocol::versions::tests -- --nocapture`: pass (`10 passed`)
- Valence `ctf` trace: pass; captured `40` play packets, all mapped among observed boundary IDs, first unmapped `None`.
- Stevenarella headless probe: pass for bounded protocol-detection smoke (`Detected server protocol version 763`), timed out intentionally with exit 124.

## Non-claims

- Does not prove full current Valence gameplay/client compatibility.
- Does not prove every protocol 763 packet is mapped.
- Does not prove parser-level semantic correctness for the mapped internal packet shapes.
