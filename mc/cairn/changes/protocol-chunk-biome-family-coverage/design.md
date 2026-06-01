# Design: Protocol chunk and biome family coverage rail

## Context

The `chunk/biome packet family` row is currently a non-claim or bounded-gap item in the compatibility evidence set. This change creates a row-scoped Cairn so future implementation cannot silently broaden existing evidence.

## Decisions

### 1. Keep the row narrow

**Choice:** Claim only selected chunk/biome packet rows with reviewed parser fixtures and one live fixture proving client receipt of configured environment data.

**Rationale:** Narrow rows are reviewable and keep broad compatibility claims false until every required row has evidence.

### 2. Compare normalized metrics

**Choice:** Checkers compare packet name, wire id, chunk position, biome id or environment id, parser fixture id, live receipt path, and malformed fixture status where supported.

**Rationale:** Raw logs and pass/fail alone do not prove the intended compatibility claim.

### 3. Fail closed on evidence gaps

**Choice:** Reject missing parser fixture, missing live observation, terrain/worldgen overclaim, fallback alias, or full chunk/biome claim.

**Rationale:** Missing reference, telemetry, parser, or correlation data must block promotion instead of becoming implicit coverage.

### 4. Preserve adjacent non-claims

**Choice:** Docs and receipts keep these non-claims explicit: world generation parity, all chunks, all biomes, lighting/weather, structures, full protocol-763 compatibility, full survival compatibility, and production readiness.

**Rationale:** Each Cairn reduces one named gap without smuggling in full protocol, gameplay, production, or security claims.

## Risks / Trade-offs

- Raw byte preservation is not semantic chunk correctness.
- Biome/dimension gameplay rail should remain separate from protocol-family coverage.
