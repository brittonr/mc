# Tasks

- [ ] [serial] Review Hyperion block/region loader code and Valence Anvil/layer loading capabilities, then define static snapshot scope and non-goals. r[valence_hyperion_integration.world_snapshot_loader.scope]
- [ ] [depends:scope] Define typed loader plan inputs, region selection, resource limits, dimension/biome validation, async boundaries, and partial-load policy. r[valence_hyperion_integration.world_snapshot_loader.contract]
- [ ] [depends:contract] Implement pure plan validation and chunk snapshot normalization over explicit in-memory inputs. r[valence_hyperion_integration.world_snapshot_loader.core]
- [ ] [depends:core] Wire filesystem, mmap if used, async reads, decompression, and layer application as thin adapters. r[valence_hyperion_integration.world_snapshot_loader.adapters]
- [ ] [depends:adapters] Add positive and negative fixtures for valid regions, missing files, corrupt NBT, out-of-range sections, dimension mismatch, biome mismatch, partial loads, and cancellation. r[valence_hyperion_integration.world_snapshot_loader.tests]
- [ ] [depends:tests] Run loader tests, corrupt-region fixtures, loader smoke tests, selected chunk/dimension mc-compat dry runs, Cairn gates, and Cairn validation. r[valence_hyperion_integration.world_snapshot_loader.validation]
