# Proposal: Add a cached chunk egress pipeline

## Why

Hyperion keeps chunk packet bytes prepared for fast map delivery, which is valuable for static worlds, minigames, and repeated chunk sends. Valence already owns chunk and layer abstractions; a cached egress path should integrate with those abstractions so servers can avoid repeated chunk serialization without weakening correctness around dimension data, biome data, compression, and invalidation.

## What Changes

- Review Hyperion's chunk sync/cache path and Valence's layer/chunk packet generation.
- Define cache eligibility, cache keys, invalidation, compression behavior, and non-goals.
- Use BLAKE3 for internal cache fingerprints when protocol interoperability does not mandate another hash.
- Implement a deterministic renderer/cache core over chunk snapshots, with filesystem/network work in thin shells.
- Add positive and negative tests for cache hits, invalidation, dimension changes, biome changes, compression changes, and stale cached bytes.

## Impact

- **Files**: Valence layer/chunk code, optional cache helpers, tests/benchmarks, docs, and Cairn artifacts.
- **Testing**: deterministic chunk-render fixtures, stale-cache rejection, direct chunk-send regressions, selected mc-compat chunk scenarios, optional benchmarks, and Cairn gates/validation.
- **Non-claims**: this improves repeated chunk egress only; it does not claim new world-generation semantics or full Hyperion map-loader parity.
