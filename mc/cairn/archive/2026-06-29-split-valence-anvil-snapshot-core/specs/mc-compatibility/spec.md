# mc-compatibility Change Spec: Valence Anvil snapshot core

## Requirements

### Requirement: Valence Anvil snapshot boundaries

r[mc_compatibility.valence_anvil.snapshot_boundaries] Valence Anvil snapshot code SHOULD expose cohesive boundaries for snapshot model types, region and chunk lookup planning, parsing and validation, cache policy, directory/filesystem shell, and Bevy integration adapters.

#### Scenario: Snapshot responsibility has one owner

r[mc_compatibility.valence_anvil.snapshot_boundaries.ownership]
- GIVEN an Anvil snapshot responsibility is reviewed
- WHEN maintainers inspect Anvil modules
- THEN the responsibility is owned by a focused module
- AND unrelated model, parse, cache, filesystem, and Bevy concerns are not reintroduced into one module.

### Requirement: Valence Anvil snapshot core

r[mc_compatibility.valence_anvil.snapshot_core] Region coordinate calculation, chunk selection, missing or corrupt classification, parse validation, and snapshot update planning SHOULD be pure over explicit inputs.

#### Scenario: Snapshot decision is testable without filesystem

r[mc_compatibility.valence_anvil.snapshot_core.testable]
- GIVEN snapshot, region, chunk, parse, or cache summaries
- WHEN the snapshot core processes them
- THEN the decision can be tested without filesystem, compression, directory traversal, Bevy resources, or logging.

### Requirement: Valence Anvil parity

r[mc_compatibility.valence_anvil.parity] Snapshot-core splitting MUST preserve public APIs, Anvil format behavior, missing/corrupt region behavior, cache behavior, Bevy integration behavior, and evidence non-claims.

#### Scenario: Anvil snapshot behavior remains stable

r[mc_compatibility.valence_anvil.parity.stable]
- GIVEN a supported pre-refactor Anvil snapshot input
- WHEN the split snapshot core and shell process the same input
- THEN lookup behavior, parse behavior, cache behavior, public APIs, and non-claim boundaries remain equivalent.

### Requirement: Valence Anvil positive tests

r[mc_compatibility.valence_anvil.positive_tests] The change MUST include positive tests for region coordinate mapping, chunk lookup, valid parse summaries, missing chunk behavior, cache plans, and snapshot update plans.

#### Scenario: Supported Anvil paths pass

r[mc_compatibility.valence_anvil.positive_tests.coverage]
- GIVEN representative supported Anvil snapshot inputs
- WHEN extracted snapshot cores process them
- THEN tests prove the expected lookup, classification, parse, cache, or update decisions are produced.

### Requirement: Valence Anvil negative tests

r[mc_compatibility.valence_anvil.negative_tests] The change MUST include negative tests for invalid coordinates, missing regions, corrupt chunks, malformed parse summaries, stale cache entries, and unavailable directories.

#### Scenario: Invalid Anvil paths fail closed

r[mc_compatibility.valence_anvil.negative_tests.fail_closed]
- GIVEN invalid or unavailable Anvil snapshot inputs
- WHEN extracted snapshot cores or shells process them
- THEN tests prove the inputs are rejected, classified, or contained according to current behavior.

### Requirement: Valence Anvil validation

r[mc_compatibility.valence_anvil.validation] The change MUST record focused Valence Anvil tests, affected workspace checks, Cairn proposal/design/tasks gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[mc_compatibility.valence_anvil.validation.logs]
- GIVEN Anvil snapshot splitting is complete
- WHEN the change is closed
- THEN reviewable logs show positive and negative Anvil tests plus affected checks and Cairn gates passing.
