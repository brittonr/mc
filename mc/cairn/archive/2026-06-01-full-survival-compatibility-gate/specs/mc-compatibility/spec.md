# Delta: Full survival compatibility aggregate gate

## Requirements

### Requirement: Contract

r[mc_compatibility.full_survival_compatibility_gate.contract] The `full survival compatibility` aggregate gate MUST define the complete required survival row set before any broad survival claim is promoted.

#### Scenario: Contract names exact row set

r[mc_compatibility.full_survival_compatibility_gate.contract.scope]
- GIVEN `full-survival-compatibility-gate` work starts
- WHEN the evidence contract is reviewed
- THEN it names these required rows: break/place/pickup, crafting table, chest persistence, furnace persistence, hunger/food, mob drops, redstone, biome/dimension, and world persistence
- AND it states that full survival compatibility, broad vanilla parity, production readiness, broad Minecraft compatibility, unbounded restart/durability, and uncovered survival mechanics remain non-claims until every row passes.

### Requirement: Aggregate checker

r[mc_compatibility.full_survival_compatibility_gate.checker] A deterministic aggregate checker MUST reject full-survival promotion unless every required row has paired reference-parity evidence.

#### Scenario: All rows covered passes

r[mc_compatibility.full_survival_compatibility_gate.checker.passes]
- GIVEN every required row is marked reference-parity covered
- AND each row has Valence evidence, Paper/reference evidence, comparator or row-checker output, BLAKE3 manifest linkage, child revision metadata or an oracle checkpoint, and explicit row-scoped non-claims
- WHEN the aggregate checker runs
- THEN it permits full-survival promotion.

#### Scenario: Missing or incomplete rows fail closed

r[mc_compatibility.full_survival_compatibility_gate.checker.rejects]
- GIVEN any required row is missing, Valence-only, missing Paper/reference evidence, missing comparator/checker evidence, missing manifest linkage, missing child revision metadata or oracle evidence, or still marked missing
- WHEN the aggregate checker runs
- THEN it fails and names the row and missing field.

#### Scenario: Premature broad wording fails closed

r[mc_compatibility.full_survival_compatibility_gate.checker.nonclaims]
- GIVEN any required row remains missing
- WHEN acceptance matrix or current-bundle text claims full survival compatibility or equivalent broad wording
- THEN the checker fails and names the offending text location.

### Requirement: Evidence standard

r[mc_compatibility.full_survival_compatibility_gate.evidence_standard] Full-survival promotion MUST use the aggregate checker as the deterministic evidence standard.

#### Scenario: Evidence standard is required

r[mc_compatibility.full_survival_compatibility_gate.evidence_standard.enforced]
- GIVEN full-survival wording is proposed
- WHEN checker output is absent or not copied under `docs/evidence/`
- THEN promotion fails before docs change.

### Requirement: Rail isolation

r[mc_compatibility.full_survival_compatibility_gate.rail] The aggregate gate MUST NOT change row-specific scenario semantics.

#### Scenario: Existing rows stay isolated

r[mc_compatibility.full_survival_compatibility_gate.rail.isolated]
- GIVEN existing survival rows and active row Cairns
- WHEN the aggregate gate is added
- THEN row-specific scenario milestone requirements remain unchanged
- AND the aggregate checker only reads evidence docs and manifests.

### Requirement: Reviewable artifacts

r[mc_compatibility.full_survival_compatibility_gate.artifacts] Review-critical aggregate gate artifacts MUST be copied under `docs/evidence/` before archive.

#### Scenario: Artifacts are durable

r[mc_compatibility.full_survival_compatibility_gate.artifacts.reviewable]
- GIVEN the aggregate gate is completed
- WHEN reviewers inspect the repo
- THEN checker output, matrix/current-bundle check logs, Cairn gate logs, Cairn validation logs, BLAKE3 manifests, and any required oracle checkpoints are present under `docs/evidence/`.

### Requirement: Matrix and bundle labels

r[mc_compatibility.full_survival_compatibility_gate.matrix] Acceptance matrix and current bundle MUST promote full-survival wording only after the aggregate checker passes.

#### Scenario: Broader claims remain false until complete

r[mc_compatibility.full_survival_compatibility_gate.matrix.nonclaims]
- GIVEN any required row remains missing
- WHEN docs are updated
- THEN full survival compatibility remains an explicit non-claim
- AND broad vanilla parity, production readiness, broad Minecraft compatibility, unbounded restart/durability, and uncovered survival mechanics remain explicit non-claims.

### Requirement: Validation

r[mc_compatibility.full_survival_compatibility_gate.validation] The change MUST record checker, manifest, task gate, and Cairn validation output before archive.

#### Scenario: Validation output is reviewable

r[mc_compatibility.full_survival_compatibility_gate.validation.log]
- GIVEN the aggregate gate is archived
- WHEN validation is reviewed
- THEN repo-local logs show aggregate checker self-tests, matrix/current-bundle checks, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation.
