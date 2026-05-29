# Delta: Survival coverage reference parity sync

## Requirements

### Requirement: Survival coverage matrix reflects reference parity

r[mc_compatibility.survival_coverage_matrix.reference_parity_synced] The survival coverage matrix MUST mark break/place/pickup as paired Paper/Valence reference parity covered when the promoted parity artifacts are present.

#### Scenario: Break/place/pickup row cites paired evidence

r[mc_compatibility.survival_coverage_matrix.reference_parity_synced.row]
- GIVEN the survival coverage matrix is reviewed
- WHEN the break/place/pickup row is present
- THEN it cites the Paper reference receipt
- AND it cites the Valence paired receipt
- AND it links the survival reference parity evidence doc.

### Requirement: Survival coverage checker blocks stale parity state

r[mc_compatibility.survival_coverage_matrix.reference_parity_gate] The survival coverage checker MUST reject stale break/place/pickup rows that claim Valence-only coverage or missing reference evidence after parity is promoted.

#### Scenario: Stale reference-missing row is rejected

r[mc_compatibility.survival_coverage_matrix.reference_parity_gate.rejects]
- GIVEN the break/place/pickup row still says reference evidence is missing
- WHEN the checker runs
- THEN it fails and names the stale row.

### Requirement: Survival breadth remains scoped

r[mc_compatibility.survival_coverage_matrix.reference_parity_nonclaims] Updating the break/place/pickup row MUST NOT claim broader survival compatibility.

#### Scenario: Missing survival rows remain non-claims

r[mc_compatibility.survival_coverage_matrix.reference_parity_nonclaims.rows]
- GIVEN break/place/pickup parity is marked covered
- WHEN the matrix is reviewed
- THEN crafting, chest persistence, furnace persistence, hunger/food, mob drops, redstone, biome/dimension, and world persistence remain missing non-claim rows.

### Requirement: Survival coverage sync validation

r[mc_compatibility.survival_coverage_matrix.reference_parity_validation] The reference parity sync MUST record checker and Cairn validation output before archive.

#### Scenario: Validation output is reviewable

r[mc_compatibility.survival_coverage_matrix.reference_parity_validation.log]
- GIVEN the matrix and checker are updated
- WHEN the change is archived
- THEN a run log records survival coverage checker, survival parity checker, current bundle, acceptance matrix, evidence manifest, task gate, and Cairn validation output.
