# mc-compatibility Change Spec: Stevenarella world core

## Requirements

### Requirement: Stevenarella dimension and chunk core

r[mc_compatibility.stevenarella_world.dimension_chunk_core] Stevenarella SHOULD expose pure deterministic cores for dimension bounds selection, chunk-section layout, biome and light interpretation, block update decisions, and storage update planning.

#### Scenario: Dimension and chunk decisions are explicit

r[mc_compatibility.stevenarella_world.dimension_chunk_core.explicit]
- GIVEN join-game dimension facts, dimension-codec facts, protocol-version facts, or chunk payload summaries
- WHEN world/chunk logic needs bounds or layout decisions
- THEN the decisions are produced by pure core functions
- AND storage mutation or rendering side effects are not required to inspect the decision.

### Requirement: Stevenarella world shell boundary

r[mc_compatibility.stevenarella_world.world_shell_boundary] Stevenarella world-core extraction MUST keep byte reading, NBT traversal, packet variant handling, world storage mutation, render invalidation, and logging outside the pure world core.

#### Scenario: World side effects remain in shell

r[mc_compatibility.stevenarella_world.world_shell_boundary.effects]
- GIVEN the world core returns a dimension, chunk, biome, light, or block update plan
- WHEN the Stevenarella world shell applies that plan
- THEN only the shell reads raw packet bytes, traverses NBT, mutates world storage, invalidates rendering, or logs diagnostics
- AND the core remains testable with in-memory summaries.

### Requirement: Stevenarella world parity

r[mc_compatibility.stevenarella_world.world_parity] World-core extraction MUST preserve existing world behavior, dimension fallback behavior, protocol-version handling, chunk parsing semantics, and evidence non-claims.

#### Scenario: Existing world behavior remains stable

r[mc_compatibility.stevenarella_world.world_parity.stable]
- GIVEN a supported pre-refactor world or chunk input
- WHEN the extracted world core and shell process the same input
- THEN the selected bounds, storage updates, parsing outcomes, and non-claim boundaries remain equivalent
- AND no full protocol 763 or broad compatibility claim is promoted.

### Requirement: Stevenarella world positive tests

r[mc_compatibility.stevenarella_world.world_positive_tests] The change MUST include positive tests for selected dimension-codec bounds, min-y and height application, section-count derivation, biome and light payload acceptance, and block update plans.

#### Scenario: Supported world inputs pass

r[mc_compatibility.stevenarella_world.world_positive_tests.coverage]
- GIVEN representative supported world, dimension, and chunk inputs
- WHEN extracted world cores process them
- THEN tests prove the expected bounds, layouts, and update plans are produced.

### Requirement: Stevenarella world negative tests

r[mc_compatibility.stevenarella_world.world_negative_tests] The change MUST include negative tests for missing dimension type, invalid min-y or height, truncated chunk data, inconsistent section counts, malformed biome or light data, and unsupported dimension inputs.

#### Scenario: Invalid world inputs fail closed

r[mc_compatibility.stevenarella_world.world_negative_tests.fail_closed]
- GIVEN malformed or unsupported world, dimension, or chunk inputs
- WHEN extracted world cores process them
- THEN tests prove the inputs are rejected, defaulted, or contained according to current behavior without corrupting storage plans.

### Requirement: Stevenarella world validation

r[mc_compatibility.stevenarella_world.world_validation] The change MUST record focused Stevenarella world/protocol tests, affected mc-compat dry-runs, Cairn proposal/design/tasks gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[mc_compatibility.stevenarella_world.world_validation.logs]
- GIVEN world-core extraction is complete
- WHEN the change is closed
- THEN reviewable logs show positive and negative world-core tests plus affected dry-runs and Cairn gates passing.
