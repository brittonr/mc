# Proposal: Add a static world snapshot loader

## Why

Hyperion's map loading flow and region parsing show a path for fast static-map startup and preloaded block data. Valence has Anvil and chunk support, but minigame servers need a clearly bounded way to load static worlds, validate dimensions/biomes, and optionally feed cached chunk egress without claiming full world-generation behavior.

## What Changes

- Review Hyperion block/region loader ideas and Valence Anvil/layer loading capabilities.
- Define a snapshot loader contract for input paths, region/chunk selection, dimension and biome validation, async boundaries, memory mapping policy, and failure behavior.
- Implement pure manifest/plan validation and chunk snapshot mapping, with filesystem, mmap, and async work in thin shells.
- Add positive and negative fixtures for valid regions, missing files, corrupt NBT, out-of-range sections, dimension mismatch, biome mismatch, partial loads, and cancelled loads.
- Document static-world non-goals and interaction with cached chunk egress.

## Impact

- **Files**: Valence Anvil/layer loading helpers, optional snapshot loader crate/plugin, config/docs, tests/fixtures, and Cairn artifacts.
- **Testing**: manifest/plan tests, corrupt-region fixtures, loader smoke tests, selected chunk/dimension mc-compat dry runs, and Cairn gates/validation.
- **Non-claims**: this does not add terrain generation, full save editing, or Hyperion map-loader parity by default.
