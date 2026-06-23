# Tasks

- [ ] [serial] Review Hyperion chunk egress/cache code and Valence layer/chunk serialization, then define cache eligibility and non-goals. r[valence_hyperion_integration.chunk_cache.scope]
- [ ] [depends:scope] Specify cache keys, BLAKE3 content fingerprints, invalidation inputs, compression settings, and dimension/biome dependencies. r[valence_hyperion_integration.chunk_cache.key]
- [ ] [depends:key] Implement a deterministic chunk snapshot renderer/cache core with storage and network flushing kept in shells. r[valence_hyperion_integration.chunk_cache.core]
- [ ] [depends:core] Add positive and negative fixtures for cache hits, block changes, biome changes, registry/dimension changes, compression changes, missing inputs, and stale bytes. r[valence_hyperion_integration.chunk_cache.fixtures]
- [ ] [depends:fixtures] Wire an optional cached egress path and docs without changing default uncached semantics. r[valence_hyperion_integration.chunk_cache.wiring]
- [ ] [depends:wiring] Run chunk tests, stale-cache rejection, direct chunk-send regressions, selected mc-compat chunk scenarios, optional benchmarks, Cairn gates, and Cairn validation. r[valence_hyperion_integration.chunk_cache.validation]
