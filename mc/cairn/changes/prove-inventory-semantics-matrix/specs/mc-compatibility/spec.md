# Delta: Inventory semantics matrix proof

## ADDED Requirements

### Requirement: Inventory semantics matrix

r[mc_compatibility.prove_inventory_semantics_matrix.inventory_matrix] The inventory proof MUST define a reviewable matrix of inventory windows, slot classes, click modes, carried-stack states, state-id freshness, and expected outcomes before promoting full inventory semantics.

#### Scenario: Matrix scopes inventory claims

r[mc_compatibility.prove_inventory_semantics_matrix.inventory_matrix.scenario]
- GIVEN inventory semantics are being evaluated
- WHEN the matrix is reviewed
- THEN each row records the interaction shape, expected server outcome, expected client observation, evidence status, and non-claim status
- AND uncovered rows do not contribute to full inventory correctness claims

### Requirement: Positive inventory scenarios

r[mc_compatibility.prove_inventory_semantics_matrix.positive_inventory_scenarios] Valid inventory interactions MUST have positive scenarios that correlate Valence server state changes with Stevenarella client observations.

#### Scenario: Valid interaction preserves expected state

r[mc_compatibility.prove_inventory_semantics_matrix.positive_inventory_scenarios.scenario]
- GIVEN a valid inventory matrix row is selected
- WHEN the scenario performs the interaction
- THEN server before/after inventory state and client slot/window observations match the row expectation
- AND the receipt records no missing milestones for that row

### Requirement: Negative inventory scenarios

r[mc_compatibility.prove_inventory_semantics_matrix.negative_inventory_scenarios] Invalid inventory interactions MUST have negative scenarios that reject stale, malformed, invalid-slot, or invalid-carried-stack transitions without corrupting state.

#### Scenario: Invalid interaction fails closed

r[mc_compatibility.prove_inventory_semantics_matrix.negative_inventory_scenarios.scenario]
- GIVEN an invalid inventory interaction is injected
- WHEN the server and client evidence is evaluated
- THEN the invalid transition is rejected or restored according to the matrix
- AND the receipt fails if server state is corrupted or a forbidden client acceptance milestone appears

### Requirement: Inventory promotion gate

r[mc_compatibility.prove_inventory_semantics_matrix.inventory_promotion_gate] Full inventory semantics MUST remain a non-claim until every required matrix row has passing deterministic tests, live receipt evidence, BLAKE3 manifests, and updated evidence indexes.

#### Scenario: Promotion is row-complete

r[mc_compatibility.prove_inventory_semantics_matrix.inventory_promotion_gate.scenario]
- GIVEN full inventory semantics are proposed as covered
- WHEN any required matrix row lacks passing evidence
- THEN the promotion is rejected
- AND the acceptance matrix keeps full inventory semantics as a non-claim
