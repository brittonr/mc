# `valence_anvil`

Support for Minecraft's [anvil file format](https://minecraft.wiki/w/Anvil_file_format).

## Static world snapshots

The `snapshot` module provides a bounded loader for static world snapshots and controlled reloads. Callers build a typed `SnapshotPlanInput` with a dimension root, selected regions or chunks, resource limits, expected dimension bounds, allowed biome identifiers, missing-region policy, partial-load policy, and adapter policy. Plan validation is pure and runs before filesystem access.

Chunk normalization is also pure over in-memory NBT plus an explicit biome map. It rejects corrupt chunk shapes, out-of-range sections, dimension height mismatches, and biome mismatches before a shell can apply chunks to a `ChunkLayer`. Filesystem discovery, Anvil decompression, cancellation checks, and layer insertion stay in shell functions.

This is not terrain generation, arbitrary save editing, Hyperion loader parity, production startup readiness, public-server safety, or broad Minecraft compatibility. Snapshot application produces `UnloadedChunk` values; cached chunk egress remains a separate `ChunkLayer` send-path setting and is not enabled by the loader.
