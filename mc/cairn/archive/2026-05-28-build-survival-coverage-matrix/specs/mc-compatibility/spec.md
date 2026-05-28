# Delta: Survival coverage matrix

## Requirements

### Requirement: Survival coverage rows

r[mc_compatibility.survival_coverage_matrix.rows] The repo MUST maintain a survival coverage matrix that separates covered survival rails from uncovered survival systems.

#### Scenario: Matrix names uncovered systems

r[mc_compatibility.survival_coverage_matrix.rows.uncovered]
- GIVEN the survival coverage matrix is reviewed
- WHEN a survival system has no live receipt
- THEN the matrix lists it as a non-claim
- AND the matrix includes crafting, furnace, chest, hunger, mob, redstone, biome, dimension, and persistence rows.

### Requirement: Survival row requirements

r[mc_compatibility.survival_coverage_matrix.row_requirements] Each survival matrix row MUST define the minimum evidence required for promotion.

#### Scenario: Row has promotion requirements

r[mc_compatibility.survival_coverage_matrix.row_requirements.present]
- GIVEN a survival row is proposed for promotion
- WHEN the matrix checker evaluates it
- THEN the row names required Valence receipts, reference receipts when parity is claimed, logs, hashes, and child revisions.

### Requirement: Full-survival gate

r[mc_compatibility.survival_coverage_matrix.gate] A deterministic checker MUST block full survival compatibility claims while required rows are missing evidence.

#### Scenario: Full claim is blocked

r[mc_compatibility.survival_coverage_matrix.gate.blocks]
- GIVEN any required survival row is missing live evidence
- WHEN documentation claims full survival compatibility
- THEN the checker fails and names the missing rows.

### Requirement: Survival non-claims

r[mc_compatibility.survival_coverage_matrix.nonclaims] Current evidence docs MUST point full-survival non-claims to the survival coverage matrix.

#### Scenario: Non-claim points to matrix

r[mc_compatibility.survival_coverage_matrix.nonclaims.linked]
- GIVEN the current evidence bundle discusses survival scope
- WHEN full survival compatibility is a non-claim
- THEN it links to the survival coverage matrix or checker output.
