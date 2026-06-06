# Design: Promote chunk biome data packet evidence

## Context

The chunk/biome family currently promotes chunk-delta raw evidence and an overworld environment receipt pair, but `ChunkBiomeDataS2CPacket` is still explicitly out of scope. This change should add one exact packet row without implying semantic biome lookup correctness.

## Decisions

### 1. Prefer parser fixture plus bounded live context

**Choice:** Use deterministic raw packet fixture evidence for the packet shape and pair it with a bounded live join/chunk/render context when feasible.

**Rationale:** Biome data is packet-shape heavy; a parser fixture is more reviewable than inferring coverage from gameplay.

### 2. Keep biome semantics out of scope

**Choice:** Validate packet acceptance/shape only, not biome lookup, rendering, generation, or dimension travel semantics.

**Rationale:** Those are broader systems requiring separate evidence.

### 3. Update the packet inventory exactly

**Choice:** Mark only `ChunkBiomeDataS2CPacket` for the configured fixture scope.

**Rationale:** The row should not widen `ChunkDataS2CPacket`, chunk-delta, or worldgen claims.

## Risks / Trade-offs

- Producing a realistic fixture may require extracting bytes from Paper or Valence logs.
- A fixture-only row must clearly avoid gameplay semantic claims.
- Chunk/biome docs already have related rows; wording must avoid double-counting evidence.
