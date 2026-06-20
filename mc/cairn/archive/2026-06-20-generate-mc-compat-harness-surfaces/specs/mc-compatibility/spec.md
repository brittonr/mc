# mc-compatibility Change Spec: Generated harness surfaces

## Requirements

### Requirement: Generated surface contract

r[mc_compatibility.generated_harness_surfaces.contract] Scenario-derived harness surfaces SHOULD be generated from the typed scenario manifest when the generated output is stable, bounded, and reviewable.

#### Scenario: Runtime remains Rust-owned

r[mc_compatibility.generated_harness_surfaces.contract.runtime]
- GIVEN generated harness surfaces exist
- WHEN `tools/mc-compat-runner` starts
- THEN it consumes checked-in Rust or static artifacts
- AND it does not evaluate Nickel at runtime.

### Requirement: Pure generator core

r[mc_compatibility.generated_harness_surfaces.generator] The generator MUST separate pure manifest parsing/rendering from the imperative shell that reads and writes repository files.

#### Scenario: Invalid manifest fixture fails closed

r[mc_compatibility.generated_harness_surfaces.generator.negative]
- GIVEN a manifest fixture has a missing required field, duplicate generated name, unsupported migration state, or unsafe output path
- WHEN the generator core evaluates the fixture
- THEN it returns deterministic diagnostics and emits no partial generated artifact.

### Requirement: Generated Rust scenario tables

r[mc_compatibility.generated_harness_surfaces.rust_tables] The checked-in Rust scenario tables SHOULD be generated from the manifest while preserving scenario names, aliases, milestone IDs, forbidden-pattern IDs, behavior metadata, and receipt semantics.

#### Scenario: Generated Rust preserves scenario parity

r[mc_compatibility.generated_harness_surfaces.rust_tables.parity]
- GIVEN generated Rust tables replace or refresh manual tables
- WHEN runner parity tests enumerate all scenarios
- THEN parsed names, aliases, client milestones, server milestones, forbidden patterns, dry-run metadata, and migration states match the manifest.

### Requirement: Generated documentation blocks

r[mc_compatibility.generated_harness_surfaces.docs_blocks] Documentation or index output MAY be generated only inside clearly delimited machine-owned blocks while human-authored evidence interpretation remains outside generated sections.

#### Scenario: Generated block is bounded

r[mc_compatibility.generated_harness_surfaces.docs_blocks.review]
- GIVEN a README or evidence index contains generated scenario commands
- WHEN a reviewer inspects the file
- THEN generated content is bounded by ownership markers
- AND prose outside the markers is not overwritten by the generator.

### Requirement: Generated output freshness

r[mc_compatibility.generated_harness_surfaces.freshness] The repository MUST include a check that regenerates manifest-derived outputs and fails when checked-in generated artifacts are stale.

#### Scenario: Stale generated artifact fails Nix check

r[mc_compatibility.generated_harness_surfaces.freshness.drift]
- GIVEN the scenario manifest changes without refreshing generated outputs
- WHEN the generated-output freshness check runs
- THEN it reports the stale artifact path and fails before evidence can be promoted.

### Requirement: Generated surface validation

r[mc_compatibility.generated_harness_surfaces.validation] The change MUST record generator fixtures, runner tests, scenario-manifest checks, generated-output freshness checks, maintained dry-run aggregate output, evidence manifest checks, and Cairn gates before archive.

#### Scenario: Validation proves generation safety

r[mc_compatibility.generated_harness_surfaces.validation.log]
- GIVEN generated surfaces are introduced
- WHEN the change is archived
- THEN reviewable logs show positive and negative generator fixtures, stale-output rejection, runner parity tests, Cairn proposal/design/tasks gates, and Cairn validation.
