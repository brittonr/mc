# valence-hyperion-integration Change Spec: Static world snapshot loader

## Requirements

### Requirement: World snapshot loader scope

r[valence_hyperion_integration.world_snapshot_loader.scope] The integration MUST review Hyperion block/region loading and Valence Anvil/layer loading before adding a static world snapshot loader.

#### Scenario: Static-world non-goals are recorded

r[valence_hyperion_integration.world_snapshot_loader.scope.non_goals]
- GIVEN snapshot loader work is selected
- WHEN reviewers inspect the scope notes
- THEN the notes distinguish static snapshots, controlled reloads, terrain generation, save editing, and Hyperion loader parity claims.

### Requirement: Loader contract

r[valence_hyperion_integration.world_snapshot_loader.contract] The loader MUST define typed plan inputs, region selection, resource limits, dimension and biome validation, async boundaries, and partial-load policy.

#### Scenario: Dimension mismatch rejects snapshot

r[valence_hyperion_integration.world_snapshot_loader.contract.dimension_mismatch]
- GIVEN a snapshot declares or implies dimension bounds incompatible with the target Valence layer
- WHEN the loader validates the plan or chunk data
- THEN it rejects the snapshot with a deterministic diagnostic
- AND no client-visible layer mutation is applied.

### Requirement: Pure loader core

r[valence_hyperion_integration.world_snapshot_loader.core] Loader plan validation and chunk snapshot normalization SHOULD be pure deterministic logic over explicit in-memory inputs.

#### Scenario: Corrupt chunk input reports error

r[valence_hyperion_integration.world_snapshot_loader.core.corrupt_chunk]
- GIVEN a chunk input has malformed or missing required NBT fields
- WHEN the normalization core evaluates it
- THEN it returns the documented parse diagnostic
- AND it does not read files, spawn tasks, or mutate layers.

### Requirement: Loader adapters

r[valence_hyperion_integration.world_snapshot_loader.adapters] Filesystem discovery, memory mapping, async reads, decompression, and layer mutation MUST remain shell adapters around the loader core.

#### Scenario: Missing region file fails by policy

r[valence_hyperion_integration.world_snapshot_loader.adapters.missing_region]
- GIVEN the plan requires a region file that is absent
- WHEN the filesystem adapter runs
- THEN it follows the documented missing-file policy
- AND records whether the load failed, skipped, or produced a partial result.

### Requirement: Loader tests

r[valence_hyperion_integration.world_snapshot_loader.tests] Snapshot loader work MUST include positive and negative tests for valid regions, missing files, corrupt NBT, out-of-range sections, dimension mismatch, biome mismatch, partial loads, and cancellation.

#### Scenario: Partial load policy is deterministic

r[valence_hyperion_integration.world_snapshot_loader.tests.partial]
- GIVEN one selected region loads and another selected region fails
- WHEN partial loads are configured
- THEN the loader produces the documented partial result or rollback result deterministically.

### Requirement: Loader validation

r[valence_hyperion_integration.world_snapshot_loader.validation] Snapshot loader work MUST record loader tests, corrupt-region fixtures, smoke tests, selected chunk/dimension compatibility dry runs, and Cairn gates before archive.

#### Scenario: Loader closeout is reviewable

r[valence_hyperion_integration.world_snapshot_loader.validation.log]
- GIVEN snapshot loader work is ready to archive
- WHEN reviewers inspect evidence logs
- THEN logs show plan validation tests, corrupt-input fixtures, adapter failure fixtures, loader smoke output, selected chunk/dimension dry runs if behavior changes, and Cairn validation.
