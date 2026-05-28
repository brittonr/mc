# Delta: Equipment slot and item matrix proof

## ADDED Requirements

### Requirement: Equipment matrix

r[mc_compatibility.prove_equipment_slot_item_matrix.equipment_matrix] The equipment breadth proof MUST define a matrix of equipment slots, representative item types, empty/non-empty transitions, and update permutations before claiming all equipment update semantics.

#### Scenario: Matrix scopes equipment claims

r[mc_compatibility.prove_equipment_slot_item_matrix.equipment_matrix.scenario]
- GIVEN equipment update breadth is being evaluated
- WHEN the matrix is reviewed
- THEN each claimed slot/item/permutation row records expected server state, expected client observation, evidence status, and non-claim status
- AND untested rows remain non-claims

### Requirement: Positive equipment scenarios

r[mc_compatibility.prove_equipment_slot_item_matrix.positive_equipment_scenarios] Valid equipment matrix rows MUST have positive scenarios that correlate Valence equipment state with Stevenarella remote-entity equipment observations.

#### Scenario: Equipment update is observed for the intended entity

r[mc_compatibility.prove_equipment_slot_item_matrix.positive_equipment_scenarios.scenario]
- GIVEN a valid equipment matrix row is selected
- WHEN the equipment update occurs
- THEN Valence server state and Stevenarella client observation identify the same entity, slot, and item representative
- AND the receipt records no missing equipment milestones for that row

### Requirement: Negative equipment scenarios

r[mc_compatibility.prove_equipment_slot_item_matrix.negative_equipment_scenarios] Equipment verification MUST reject stale, missing, duplicate, wrong-entity, wrong-slot, or wrong-item evidence.

#### Scenario: Mismatched equipment evidence fails

r[mc_compatibility.prove_equipment_slot_item_matrix.negative_equipment_scenarios.scenario]
- GIVEN fixture or live evidence contains mismatched equipment milestones
- WHEN the runner evaluates the equipment row
- THEN the row fails with explicit diagnostics
- AND no acceptance matrix claim is promoted from the mismatched evidence

### Requirement: Equipment promotion gate

r[mc_compatibility.prove_equipment_slot_item_matrix.equipment_promotion_gate] All equipment slots/items/permutations MUST remain a non-claim until required matrix rows have deterministic tests, live receipts, BLAKE3 manifests, and updated evidence indexes.

#### Scenario: Equipment promotion is evidence complete

r[mc_compatibility.prove_equipment_slot_item_matrix.equipment_promotion_gate.scenario]
- GIVEN all equipment update semantics are proposed as covered
- WHEN any required matrix row lacks passing evidence
- THEN the promotion is rejected
- AND the current bundle continues to list equipment breadth as a non-claim
