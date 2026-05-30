# Delta: Equipment slot item matrix expansion rail

## Requirements

### Requirement: Contract

r[mc_compatibility.equipment_slot_item_matrix_expansion.contract] The `equipment slot/item matrix` row MUST define a bounded deterministic evidence contract before producing or promoting receipts.

#### Scenario: Contract names exact scope

r[mc_compatibility.equipment_slot_item_matrix_expansion.contract.scope]
- GIVEN `equipment-slot-item-matrix-expansion` work starts
- WHEN the evidence contract is reviewed
- THEN it names a bounded matrix of configured equipment slots, item ids, counts, and remote observer update expectations
- AND it states that all equipment slots/items, equipment packet permutations, armor mitigation, enchantment/status effects, and production readiness remain non-claims.

### Requirement: Normalized checker

r[mc_compatibility.equipment_slot_item_matrix_expansion.checker] A deterministic checker MUST compare normalized metrics before the `equipment slot/item matrix` row is promoted.

#### Scenario: Missing or mismatched evidence fails closed

r[mc_compatibility.equipment_slot_item_matrix_expansion.checker.rejects]
- GIVEN evidence is missing or mismatches actor identity, observer identity, slot, item id, item count, update order, remote entity id, and client/server correlation ids
- WHEN the checker runs
- THEN it fails and names the missing or mismatched metric.

#### Scenario: Evidence standard is enforced

r[mc_compatibility.equipment_slot_item_matrix_expansion.checker.standard]
- GIVEN the row requires matrix checker with per-row client/server correlation and no broad slot/item claim outside listed rows
- WHEN evidence lacks that standard
- THEN promotion fails before any matrix or current-bundle claim is recorded.

### Requirement: Runner or fixture rail

r[mc_compatibility.equipment_slot_item_matrix_expansion.rail] The harness MUST expose a `equipment-slot-item-matrix-expansion` rail or fixture set that records required client/server/protocol milestones without changing existing row semantics.

#### Scenario: Rail is isolated

r[mc_compatibility.equipment_slot_item_matrix_expansion.rail.isolated]
- GIVEN existing maintained scenarios and evidence rows
- WHEN `equipment-slot-item-matrix-expansion` is added
- THEN existing milestone requirements and claims remain unchanged
- AND the new row has explicit required milestones or fixture outputs.

### Requirement: Reviewable evidence

r[mc_compatibility.equipment_slot_item_matrix_expansion.evidence] `equipment slot/item matrix` evidence MUST be reviewable locally before promotion.

#### Scenario: Evidence artifacts are durable

r[mc_compatibility.equipment_slot_item_matrix_expansion.evidence.reviewable]
- GIVEN the `equipment slot/item matrix` row is proposed for promotion
- WHEN reviewers inspect the repo
- THEN required receipts, logs, checker output, and BLAKE3 manifests are present under `docs/evidence/`
- AND child revisions, target ownership, authorization, or oracle checkpoints are recorded when applicable.

### Requirement: Matrix promotion

r[mc_compatibility.equipment_slot_item_matrix_expansion.matrix] Acceptance matrix and current-bundle docs MUST promote only the `equipment slot/item matrix` row after required evidence passes.

#### Scenario: Broader claims remain false

r[mc_compatibility.equipment_slot_item_matrix_expansion.matrix.nonclaims]
- GIVEN `equipment slot/item matrix` evidence passes
- WHEN matrix/current-bundle docs are updated
- THEN only the configured `equipment slot/item matrix` row is marked covered
- AND all equipment slots/items, equipment packet permutations, armor mitigation, enchantment/status effects, and production readiness remain explicit non-claims.

### Requirement: Validation

r[mc_compatibility.equipment_slot_item_matrix_expansion.validation] The change MUST record checker, manifest, task gate, and Cairn validation output before archive.

#### Scenario: Validation output is reviewable

r[mc_compatibility.equipment_slot_item_matrix_expansion.validation.log]
- GIVEN the `equipment slot/item matrix` row is completed
- WHEN the change is archived
- THEN repo-local logs record the row checker, acceptance/current-bundle checks, evidence manifest check, task-evidence gate, and Cairn validation/gates.
