# Delta: Survival biome and dimension state parity rail

## Requirements

### Requirement: Contract

r[mc_compatibility.survival_biome_dimension.contract] The `biome/dimension` row MUST define a bounded deterministic evidence contract before producing receipts.

#### Scenario: Contract names exact scope

r[mc_compatibility.survival_biome_dimension.contract.scope]
- GIVEN `survival-biome-dimension-state` work starts
- WHEN the evidence contract is reviewed
- THEN it names one configured environment-state observation, either a fixed biome sample or a bounded fixture-driven dimension transition, with explicit normalized state fields
- AND it states that world-generation parity, all biomes, all dimensions, portal mechanics breadth, lighting/weather parity, structure generation, full survival compatibility, broad vanilla parity, and production readiness remain non-claims.

### Requirement: Checker

r[mc_compatibility.survival_biome_dimension.checker] A deterministic checker MUST reject `biome/dimension` promotion unless paired Paper and Valence evidence contains matching required metrics.

#### Scenario: Missing or mismatched metrics fail closed

r[mc_compatibility.survival_biome_dimension.checker.rejects]
- GIVEN a receipt pair is missing or mismatches spawn environment, biome or dimension identifier, client-observed environment update, server authoritative environment state, and matching Paper/Valence normalized identifiers
- WHEN the checker runs
- THEN it fails and names the missing or mismatched metric.

#### Scenario: Valence-only evidence is rejected

r[mc_compatibility.survival_biome_dimension.checker.paired]
- GIVEN only a Valence receipt is present
- WHEN the checker runs for `biome/dimension` reference parity
- THEN it refuses to promote the row and names the missing Paper/reference artifact.

### Requirement: Runner rail

r[mc_compatibility.survival_biome_dimension.runner] The compatibility runner MUST expose a `survival-biome-dimension-state` scenario that records client and server milestones without changing existing scenario semantics.

#### Scenario: Scenario is isolated

r[mc_compatibility.survival_biome_dimension.runner.isolated]
- GIVEN existing survival and CTF scenarios
- WHEN `survival-biome-dimension-state` is added
- THEN existing scenario milestone requirements and tests remain unchanged
- AND the new scenario has explicit required client and server milestones.

### Requirement: Fixtures

r[mc_compatibility.survival_biome_dimension.fixtures] Paper and Valence fixtures MUST emit reviewable server-side milestones for `biome/dimension` normalized metrics.

#### Scenario: Server milestones correlate with client milestones

r[mc_compatibility.survival_biome_dimension.fixtures.correlate]
- GIVEN the client completes the configured `biome/dimension` action
- WHEN both backend logs are reviewed
- THEN Paper and Valence report matching normalized metrics for the configured row
- AND each milestone is scoped to the configured username, scenario, and fixture state.

### Requirement: Receipts

r[mc_compatibility.survival_biome_dimension.receipts] `biome/dimension` evidence MUST include paired Paper and Valence receipts/logs copied under `docs/evidence/` with committed revision metadata or an oracle checkpoint.

#### Scenario: Evidence is reviewable locally

r[mc_compatibility.survival_biome_dimension.receipts.reviewable]
- GIVEN a `biome/dimension` row is proposed for promotion
- WHEN reviewers inspect the repo
- THEN all required receipts, client logs, server logs, run logs, and BLAKE3 manifests are present under `docs/evidence/`.

### Requirement: Matrix promotion

r[mc_compatibility.survival_biome_dimension.matrix] The survival coverage matrix MUST promote only the `biome/dimension` row after paired evidence passes.

#### Scenario: Broader survival claims remain non-claims

r[mc_compatibility.survival_biome_dimension.matrix.nonclaims]
- GIVEN `biome/dimension` evidence passes
- WHEN the matrix and current bundle are updated
- THEN only the `biome/dimension` row is marked reference-parity covered
- AND world-generation parity, all biomes, all dimensions, portal mechanics breadth, lighting/weather parity, structure generation, full survival compatibility, broad vanilla parity, and production readiness remain explicit non-claims.

### Requirement: Validation

r[mc_compatibility.survival_biome_dimension.validation] The change MUST record checker, evidence manifest, task gate, and Cairn validation output before archive.

#### Scenario: Validation output is reviewable

r[mc_compatibility.survival_biome_dimension.validation.log]
- GIVEN the `biome/dimension` row is promoted
- WHEN the change is archived
- THEN repo-local logs record the row checker, survival coverage checker, acceptance matrix, current bundle, evidence manifests, task gate, and Cairn validation output.
