# valence-hyperion-integration Change Spec: Cached chunk egress pipeline

## Requirements

### Requirement: Chunk cache scope

r[valence_hyperion_integration.chunk_cache.scope] The integration MUST review Hyperion chunk egress/cache behavior and Valence layer/chunk serialization before adding cached chunk egress.

#### Scenario: Cache scope records non-goals

r[valence_hyperion_integration.chunk_cache.scope.non_goals]
- GIVEN cached chunk egress is selected
- WHEN reviewers inspect the scope notes
- THEN they identify cache-eligible chunk states, affected Valence APIs, and non-goals such as world-generation parity or Hyperion map-loader parity.

### Requirement: Chunk cache key contract

r[valence_hyperion_integration.chunk_cache.key] Cached chunk bytes MUST be keyed by every setting and input that can affect client-visible chunk packets, including chunk position, dimension/registry inputs, block/biome/light data, protocol version, and compression behavior.

#### Scenario: Compression change invalidates cache

r[valence_hyperion_integration.chunk_cache.key.compression]
- GIVEN a cached chunk entry was created with one compression setting
- WHEN the server sends the same chunk with a different compression setting
- THEN the cache key does not match the stale entry
- AND bytes are regenerated or a matching entry is selected.

### Requirement: Pure chunk cache core

r[valence_hyperion_integration.chunk_cache.core] Chunk packet rendering for cacheable inputs SHOULD be a deterministic core over chunk snapshots and render settings, with storage, eviction, metrics, and network writes in shells.

#### Scenario: Same snapshot renders same bytes

r[valence_hyperion_integration.chunk_cache.core.deterministic]
- GIVEN identical chunk snapshots and render settings
- WHEN the renderer runs multiple times
- THEN it returns byte-identical packet payloads and identical cache metadata.

### Requirement: Chunk cache fixture coverage

r[valence_hyperion_integration.chunk_cache.fixtures] Cached chunk egress MUST include positive and negative fixtures for cache hits, invalidation, missing inputs, and stale cached bytes.

#### Scenario: Block mutation invalidates entry

r[valence_hyperion_integration.chunk_cache.fixtures.block_mutation]
- GIVEN a chunk cache entry exists for a snapshot
- WHEN a block mutation changes client-visible chunk data
- THEN the old entry is not reused for the mutated snapshot
- AND the fixture fails if stale bytes are emitted.

### Requirement: Optional cached egress wiring

r[valence_hyperion_integration.chunk_cache.wiring] Valence MAY expose cached chunk egress as an optional path, but default uncached semantics MUST remain available and compatible.

#### Scenario: Uncached send remains valid

r[valence_hyperion_integration.chunk_cache.wiring.uncached]
- GIVEN cached egress is disabled
- WHEN existing chunk-send tests or selected mc-compat chunk scenarios run
- THEN Valence sends chunks through the existing uncached path with unchanged semantics.

### Requirement: Chunk cache validation

r[valence_hyperion_integration.chunk_cache.validation] Cached chunk egress work MUST record chunk renderer tests, stale-cache rejection, direct chunk-send regressions, selected mc-compat chunk scenarios, and Cairn gates before archive.

#### Scenario: Chunk cache closeout is reviewable

r[valence_hyperion_integration.chunk_cache.validation.log]
- GIVEN cached chunk egress is ready to archive
- WHEN reviewers inspect evidence logs
- THEN logs show deterministic render fixtures, invalidation fixtures, stale-cache rejection, direct chunk-send regressions, selected mc-compat chunk scenarios, optional benchmark output if performance is claimed, and Cairn validation.
