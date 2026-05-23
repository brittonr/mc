# Design: PlayerAbilitiesS2CPacket protocol 763 boundary

## Context

The parent `mc` evidence chain records incremental Stevenarella fork updates for protocol 763. Already-landed seams are:

- `0x28` `GameJoinS2C` -> `JoinGame_WorldNames_IsHard_SimDist`
- `0x10` `CommandTreeS2CPacket` -> `DeclareCommands`
- `0x64` `GameMessageS2CPacket` -> `ServerMessage_Position`

The latest trace `/tmp/valence-763-after-game-message-update-trace.json` observed `0x34` as Valence `PlayerAbilitiesS2CPacket`. Its inherited fallback is inherited 758 CombatEventEnter. This package covers only `0x34` / `PlayerAbilitiesS2CPacket`.

## Decisions

### 1. Keep the update boundary-first

**Choice:** Implement only the `0x34` clientbound play mapping/parser needed for `PlayerAbilitiesS2CPacket` before moving to later trace packets.

**Rationale:** The fork has been advancing one deterministic protocol seam at a time, with each seam backed by tests, a trace, and a parent evidence gate.

### 2. Prefer existing internal semantics before new DTOs

**Choice:** Inspect `packet.rs` for a compatible internal semantic. Add a new internal packet/parser only if reusing an existing semantic would misdescribe the protocol 763 packet shape.

**Rationale:** ID remaps are safe only when the old internal shape is compatible with the Valence 763 packet. Semantic mismatch must fail closed into a new DTO rather than overclaiming compatibility.

### 3. Evidence remains non-overclaiming

**Choice:** The evidence receipt must claim only the `0x34` mapping update and must keep full-current-Valence and full-Stevenarella-763 support flags false.

**Rationale:** Later packets remain unresolved; the trace-derived boundary is an incremental fork update, not a full compatibility release.

## Risks / Trade-offs

- `PlayerAbilitiesS2CPacket` may require a packet-shape update rather than a simple ID remap.
- Advancing this boundary can reveal a later, larger parser gap such as chunk data or tags.
- Evidence derived from the `ctf` example proves that observed login/play path, not every possible server behavior.
