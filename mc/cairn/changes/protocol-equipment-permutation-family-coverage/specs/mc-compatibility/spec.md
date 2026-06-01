# Delta: Protocol equipment permutation family coverage rail

## Requirements

### Requirement: Contract

r[mc_compatibility.protocol_equipment_permutation_family_coverage.contract] The `equipment permutation packet family` row MUST define a bounded deterministic evidence contract before producing or promoting receipts.

#### Scenario: Contract names exact scope

r[mc_compatibility.protocol_equipment_permutation_family_coverage.contract.scope]
- GIVEN `protocol-equipment-permutation-family` work starts
- WHEN the evidence contract is reviewed
- THEN it names a named subset of equipment update packet permutations with reviewed parser fixtures and remote observer receipts
- AND it states that all equipment permutations, armor mitigation, combat balancing, all item types, full protocol-763 compatibility, and production readiness remain non-claims.

### Requirement: Normalized checker

r[mc_compatibility.protocol_equipment_permutation_family_coverage.checker] A deterministic checker MUST compare normalized metrics before the `equipment permutation packet family` row is promoted.

#### Scenario: Missing or mismatched evidence fails closed

r[mc_compatibility.protocol_equipment_permutation_family_coverage.checker.rejects]
- GIVEN evidence is missing or mismatches equipment packet name, wire id, entity id, slot, item id, count, parser fixture id, live observer receipt, and digest
- WHEN the checker runs
- THEN it fails and names the missing or mismatched metric.

#### Scenario: Evidence standard is enforced

r[mc_compatibility.protocol_equipment_permutation_family_coverage.checker.standard]
- GIVEN the row requires protocol ledger row with parser fixtures plus equipment observer live evidence and explicit gameplay non-claims
- WHEN evidence lacks that standard
- THEN promotion fails before any matrix or current-bundle claim is recorded.

### Requirement: Runner or fixture rail

r[mc_compatibility.protocol_equipment_permutation_family_coverage.rail] The harness MUST expose a `protocol-equipment-permutation-family` rail or fixture set that records required client/server/protocol milestones without changing existing row semantics.

#### Scenario: Rail is isolated

r[mc_compatibility.protocol_equipment_permutation_family_coverage.rail.isolated]
- GIVEN existing maintained scenarios and evidence rows
- WHEN `protocol-equipment-permutation-family` is added
- THEN existing milestone requirements and claims remain unchanged
- AND the new row has explicit required milestones or fixture outputs.

### Requirement: Reviewable evidence

r[mc_compatibility.protocol_equipment_permutation_family_coverage.evidence] `equipment permutation packet family` evidence MUST be reviewable locally before promotion.

#### Scenario: Evidence artifacts are durable

r[mc_compatibility.protocol_equipment_permutation_family_coverage.evidence.reviewable]
- GIVEN the `equipment permutation packet family` row is proposed for promotion
- WHEN reviewers inspect the repo
- THEN required receipts, logs, checker output, and BLAKE3 manifests are present under `docs/evidence/`
- AND child revisions, target ownership, authorization, or oracle checkpoints are recorded when applicable.

### Requirement: Matrix promotion

r[mc_compatibility.protocol_equipment_permutation_family_coverage.matrix] Acceptance matrix and current-bundle docs MUST promote only the `equipment permutation packet family` row after required evidence passes.

#### Scenario: Broader claims remain false

r[mc_compatibility.protocol_equipment_permutation_family_coverage.matrix.nonclaims]
- GIVEN `equipment permutation packet family` evidence passes
- WHEN matrix/current-bundle docs are updated
- THEN only the configured `equipment permutation packet family` row is marked covered
- AND all equipment permutations, armor mitigation, combat balancing, all item types, full protocol-763 compatibility, and production readiness remain explicit non-claims.

### Requirement: Validation

r[mc_compatibility.protocol_equipment_permutation_family_coverage.validation] The change MUST record checker, manifest, task gate, and Cairn validation output before archive.

#### Scenario: Validation output is reviewable

r[mc_compatibility.protocol_equipment_permutation_family_coverage.validation.log]
- GIVEN the `equipment permutation packet family` row is completed
- WHEN the change is archived
- THEN repo-local logs record the row checker, acceptance/current-bundle checks, evidence manifest check, task-evidence gate, and Cairn validation/gates.
