# mc-compatibility Change Spec: Stevenarella block registry separation

## Requirements

### Requirement: Stevenarella block registry boundaries

r[mc_compatibility.stevenarella_blocks.registry_boundaries] Stevenarella block registry code SHOULD separate generated or declarative block data from hand-authored runtime APIs, helper logic, id-map logic, and public compatibility exports.

#### Scenario: Block data and logic are distinguishable

r[mc_compatibility.stevenarella_blocks.registry_boundaries.ownership]
- GIVEN a block registry change is reviewed
- WHEN maintainers inspect the block crate module tree
- THEN generated block facts and hand-authored runtime logic are owned by distinct modules
- AND reviewers can identify whether the change alters data, logic, or public exports.

### Requirement: Stevenarella block registry parity

r[mc_compatibility.stevenarella_blocks.registry_parity] Block registry separation MUST preserve public block names, numeric IDs, exports, material and collision semantics, `VanillaIDMap` lookup behavior, missing-block fallback behavior, modded-block fallback behavior, and evidence non-claims.

#### Scenario: Existing block lookups remain stable

r[mc_compatibility.stevenarella_blocks.registry_parity.stable]
- GIVEN a supported pre-refactor block lookup, material lookup, collision lookup, or public export use
- WHEN the separated block registry processes the same input
- THEN the returned block, metadata, fallback, and public symbol behavior remain equivalent
- AND no new block support or world compatibility claim is promoted.

### Requirement: Stevenarella block generated freshness

r[mc_compatibility.stevenarella_blocks.generated_freshness] If block data is generated or snapshot-owned, the change SHOULD include a deterministic freshness check that rejects stale checked-in generated data.

#### Scenario: Generated block data is fresh

r[mc_compatibility.stevenarella_blocks.generated_freshness.check]
- GIVEN the generated block data source or generator changes
- WHEN the block registry freshness check runs
- THEN checked-in generated block data is verified against the source
- AND stale generated output is rejected before archive.

### Requirement: Stevenarella block registry positive tests

r[mc_compatibility.stevenarella_blocks.positive_tests] The change MUST include positive tests for representative block id lookups, flat and hierarchical mappings, material access, collision access, and public re-exports.

#### Scenario: Supported block registry paths pass

r[mc_compatibility.stevenarella_blocks.positive_tests.coverage]
- GIVEN representative supported block registry inputs
- WHEN separated block registry modules process them
- THEN tests prove the expected blocks, metadata, collisions, and exported symbols are available.

### Requirement: Stevenarella block registry negative tests

r[mc_compatibility.stevenarella_blocks.negative_tests] The change MUST include negative tests for missing ids, unsupported modded ids, stale generated data, invalid data indices, and unknown fallback paths.

#### Scenario: Invalid block registry paths fail closed

r[mc_compatibility.stevenarella_blocks.negative_tests.fail_closed]
- GIVEN invalid or unsupported block registry inputs
- WHEN separated block registry modules process them
- THEN tests prove the inputs return the current missing-block or diagnostic behavior without panicking or corrupting lookup state.

### Requirement: Stevenarella block registry validation

r[mc_compatibility.stevenarella_blocks.validation] The change MUST record focused block tests, generated freshness checks if added, affected mc-compat dry-runs, Cairn proposal/design/tasks gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[mc_compatibility.stevenarella_blocks.validation.logs]
- GIVEN block registry separation is complete
- WHEN the change is closed
- THEN reviewable logs show positive and negative block-registry tests plus freshness checks when applicable and Cairn gates passing.
