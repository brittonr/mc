# mc-compatibility Change Spec: Chunk biome data packet promotion

## Requirements

### Requirement: Chunk biome data packet contract

r[mc_compatibility.chunk_biome_data_packet_promotion.contract] The `chunk-biome-data-packet` row MUST define a bounded promotion contract before packet inventory, matrix, or current-bundle coverage is claimed.

#### Scenario: Contract names one packet fixture

r[mc_compatibility.chunk_biome_data_packet_promotion.contract.scope]
- GIVEN the row is prepared for promotion
- WHEN reviewers inspect the contract
- THEN it names the packet row `ChunkBiomeDataS2CPacket`, fixture source, protocol version, payload identity or hash, parser expectations, optional live context receipt, and checker metrics
- AND all biome semantics, all chunk semantics, all worldgen packets, dimension travel, Nether/End behavior, full protocol-763 compatibility, broad Minecraft compatibility, and production readiness remain explicit non-claims.

### Requirement: Chunk biome data packet checker

r[mc_compatibility.chunk_biome_data_packet_promotion.checker] A deterministic Rust checker MUST validate normalized chunk biome data packet evidence before promotion.

#### Scenario: Valid chunk biome evidence passes

r[mc_compatibility.chunk_biome_data_packet_promotion.checker.valid]
- GIVEN normalized evidence names `chunk-biome-data-packet`, the configured packet row, fixture payload identity, parser result, protocol version, and required non-claims
- WHEN the checker evaluates the record
- THEN it passes only if every configured metric is present and internally consistent.

#### Scenario: Weak chunk biome evidence fails closed

r[mc_compatibility.chunk_biome_data_packet_promotion.checker.rejects]
- GIVEN evidence is missing the row id, names the wrong packet, lacks fixture identity, omits parser result, mismatches protocol, or claims broad biome/chunk/worldgen semantics
- WHEN the checker evaluates the record
- THEN it fails and names the missing, unexpected, or mismatched metric.

### Requirement: Chunk biome data packet rail

r[mc_compatibility.chunk_biome_data_packet_promotion.rail] The promotion MUST use isolated packet fixture or live-context rails without changing existing survival, chunk/biome, inventory, CTF, combat, or network semantics.

#### Scenario: Existing chunk/biome rows remain separate

r[mc_compatibility.chunk_biome_data_packet_promotion.rail.isolated]
- GIVEN existing chunk/biome rows cover chunk-delta and overworld environment context
- WHEN chunk biome data evidence is added
- THEN existing rows remain unchanged
- AND the new row records its own packet fixture and checker output.

### Requirement: Chunk biome reviewable artifacts

r[mc_compatibility.chunk_biome_data_packet_promotion.artifacts] Review-critical chunk biome packet artifacts MUST be copied under `docs/evidence/` before promotion.

#### Scenario: Artifacts include checks and fixture source

r[mc_compatibility.chunk_biome_data_packet_promotion.artifacts.reviewable]
- GIVEN the row is ready for promotion
- WHEN reviewers inspect the repository
- THEN fixture payloads or hashes, normalized inputs, checker output, BLAKE3 manifests, optional live receipts, and any oracle checkpoint are present under `docs/evidence/`.

### Requirement: Narrow chunk biome packet matrix promotion

r[mc_compatibility.chunk_biome_data_packet_promotion.matrix] Packet inventory, acceptance matrix, and current bundle docs MUST promote only the configured chunk biome data packet row after checker and evidence gates pass.

#### Scenario: Broader chunk/biome remains a non-claim

r[mc_compatibility.chunk_biome_data_packet_promotion.matrix.nonclaims]
- GIVEN chunk biome data evidence passes
- WHEN docs are updated
- THEN only the configured packet row is marked covered
- AND all broader biome, chunk, worldgen, dimension, full protocol, and production claims remain explicit non-claims.

### Requirement: Chunk biome packet validation evidence

r[mc_compatibility.chunk_biome_data_packet_promotion.validation] The change MUST record checker, packet inventory, manifest, task gate, Cairn gates, and Cairn validation output before archive.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.chunk_biome_data_packet_promotion.validation.log]
- GIVEN the chunk biome data packet row is completed
- WHEN the change is archived
- THEN repo-local evidence logs record checker tests, fixture or runner checks, packet inventory checks, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation.
