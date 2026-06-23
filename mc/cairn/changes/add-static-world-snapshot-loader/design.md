# Design: Add a static world snapshot loader

## Context

Hyperion loads map data into a block resource for a static event server. Valence has existing world/chunk data structures and Anvil support. A Valence snapshot loader should focus on static, reviewable inputs and produce explicit chunk snapshots or layer mutations.

## Decisions

### 1. Loader plan is pure

**Choice:** Parse and validate a loader plan from typed inputs before touching files. The plan names worlds, dimensions, region ranges, expected registries, and output layer behavior.

**Rationale:** Invalid configurations should fail before filesystem or async work begins.

### 2. Filesystem and mmap are shells

**Choice:** Region file discovery, memory mapping, async reads, and decompression live in shell adapters. Chunk normalization and validation use explicit in-memory inputs.

**Rationale:** Corrupt input fixtures should be testable without live directories.

### 3. Dimension and biome validation is mandatory

**Choice:** Loaded snapshots must validate expected dimension bounds and biome identifiers before being accepted for client-visible chunks.

**Rationale:** Dimension mismatches are a known source of client/rendering bugs.

### 4. Static-world scope is explicit

**Choice:** The first loader targets static snapshots and controlled reloads, not full world generation or arbitrary save editing.

**Rationale:** Minigame maps need fast load, not a complete world engine.

## Risks / Trade-offs

- Large worlds can consume memory; require range selection and resource limits.
- Mmap behavior differs by platform; keep it optional or adapter-specific.
- Partial loads can be useful but risky; define acceptance and rollback policy.
