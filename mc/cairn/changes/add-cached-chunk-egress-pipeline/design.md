# Design: Add a cached chunk egress pipeline

## Context

Valence serializes chunks from layer state for clients. Hyperion prepares and stores chunk packet bytes so repeated sends avoid re-encoding. A Valence implementation needs careful cache keys because chunk bytes depend on world height, dimension/biome registries, protocol version, compression threshold, and mutable block/biome/light state.

## Decisions

### 1. Cache immutable snapshots, not live chunks

**Choice:** The cache core consumes explicit chunk snapshots plus render settings and returns packet bytes and metadata.

**Rationale:** Snapshot inputs make cache behavior deterministic and testable.

### 2. Make cache keys reviewable

**Choice:** Cache keys include every setting that can affect client-visible chunk bytes. Use BLAKE3 fingerprints for internal content identity unless an existing protocol contract requires another algorithm.

**Rationale:** Missing key inputs create stale bytes that are hard to debug.

### 3. Separate renderer from storage

**Choice:** The renderer is pure over chunk snapshots. In-memory or on-disk storage, eviction, metrics, and packet flushing live in shells.

**Rationale:** Correctness can be tested independently of cache backend policy.

### 4. Fail closed on uncertainty

**Choice:** Unknown dimension state, changed registries, missing light inputs, compression mismatch, or stale dirty flags must bypass or invalidate cached bytes.

**Rationale:** Sending stale chunks is worse than re-encoding.

## Risks / Trade-offs

- Cache keys can become large; mitigate with structured fields and content fingerprints.
- Cache invalidation can be over-conservative; acceptable before optimizing.
- Benchmarks may not reflect all map shapes; keep performance claims tied to measured fixtures.
