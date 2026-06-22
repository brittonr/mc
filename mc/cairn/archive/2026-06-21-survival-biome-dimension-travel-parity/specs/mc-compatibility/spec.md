# Delta: Survival biome and dimension travel parity

## Requirements

### Requirement: Biome dimension travel contract

r[mc_compatibility.survival_biome_dimension_travel_parity.contract] The `survival-biome-dimension-travel-parity` row MUST define a bounded dimension-transition contract before promotion.

#### Scenario: Contract names finite travel scope

r[mc_compatibility.survival_biome_dimension_travel_parity.contract.scope]
- GIVEN biome/dimension travel work starts
- WHEN the contract is reviewed
- THEN it names starting environment, trigger action, target environment, transition state, position bounds, post-transition observation, and normalized server/client metrics
- AND all biomes, biome lookup breadth, all dimensions, portal mechanics breadth, world generation, full survival compatibility, and broad vanilla parity remain non-claims.

### Requirement: Biome dimension travel checker

r[mc_compatibility.survival_biome_dimension_travel_parity.checker] A deterministic checker MUST compare paired Paper/reference and Valence dimension-travel metrics before promotion.

#### Scenario: Weak travel evidence fails closed

r[mc_compatibility.survival_biome_dimension_travel_parity.checker.rejects]
- GIVEN evidence is missing the Paper record, contains only Valence evidence, omits environment identifiers, mismatches transition state, reports stale child revisions, or claims all dimension behavior
- WHEN the checker evaluates the row
- THEN it fails with diagnostics naming the invalid travel metric.

### Requirement: Isolated biome dimension travel rail

r[mc_compatibility.survival_biome_dimension_travel_parity.rail] The harness MUST expose an isolated dimension-travel rail without changing the existing overworld join-state row.

#### Scenario: Existing environment row remains unchanged

r[mc_compatibility.survival_biome_dimension_travel_parity.rail.isolated]
- GIVEN the existing overworld join-state row is promoted
- WHEN the dimension-travel rail is added
- THEN existing row milestones and non-claims remain unchanged
- AND the new row records its own transition metrics.

### Requirement: Reviewable biome dimension travel receipts

r[mc_compatibility.survival_biome_dimension_travel_parity.receipts] Paired dimension-travel receipts and logs MUST be copied under `docs/evidence/` with child revision metadata and BLAKE3 manifests.

#### Scenario: Receipts are reviewable

r[mc_compatibility.survival_biome_dimension_travel_parity.receipts.reviewable]
- GIVEN the row is ready for review
- WHEN reviewers inspect `docs/evidence/`
- THEN Paper/reference and Valence receipts, client logs, server logs, comparator output, and manifests are present.

### Requirement: Narrow biome dimension travel promotion

r[mc_compatibility.survival_biome_dimension_travel_parity.promotion] Matrix and bundle docs MUST promote only the bounded dimension-travel row after paired evidence passes.

#### Scenario: Broader dimension behavior remains a non-claim

r[mc_compatibility.survival_biome_dimension_travel_parity.promotion.nonclaims]
- GIVEN paired dimension-travel evidence passes
- WHEN docs are updated
- THEN only the configured dimension-travel row is marked covered
- AND all biomes, all dimensions, portal mechanics breadth, world generation, full survival compatibility, broad vanilla parity, public-server safety, and production readiness remain explicit non-claims.

### Requirement: Biome dimension travel validation evidence

r[mc_compatibility.survival_biome_dimension_travel_parity.validation] The change MUST record checker, comparator, manifest, task gate, Cairn gates, and Cairn validation output before archive.

#### Scenario: Validation output is reviewable

r[mc_compatibility.survival_biome_dimension_travel_parity.validation.log]
- GIVEN the row is completed
- WHEN the change is archived
- THEN repo-local evidence logs record checker self-tests, paired comparator, scenario checks, evidence manifests, task-evidence gate, Cairn gates, and Cairn validation.
