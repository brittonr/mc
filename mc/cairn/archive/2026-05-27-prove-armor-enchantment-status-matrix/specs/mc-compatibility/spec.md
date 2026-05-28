# Delta: Armor, enchantment, and status modifier proof

## ADDED Requirements

### Requirement: Modifier matrix

r[mc_compatibility.prove_armor_enchantment_status_matrix.modifier_matrix] The armor/modifier proof MUST define a matrix of armor loadouts, armor materials, enchantment representatives, status-effect representatives, attack types, and expected damage deltas before promoting modifier breadth claims.

#### Scenario: Matrix scopes modifier claims

r[mc_compatibility.prove_armor_enchantment_status_matrix.modifier_matrix.scenario]
- GIVEN armor, enchantment, or status-effect behavior is being evaluated
- WHEN the matrix is reviewed
- THEN each row records loadout, modifiers, attack type, expected server calculation, expected client health observation, evidence status, and non-claim status
- AND untested combinations remain non-claims

### Requirement: Positive modifier scenarios

r[mc_compatibility.prove_armor_enchantment_status_matrix.positive_modifier_scenarios] Selected armor/enchantment/status rows MUST have positive scenarios that correlate Valence modifier calculations with Stevenarella victim health observations.

#### Scenario: Modifier row has correlated damage evidence

r[mc_compatibility.prove_armor_enchantment_status_matrix.positive_modifier_scenarios.scenario]
- GIVEN a modifier matrix row is selected
- WHEN a bounded combat event occurs
- THEN server evidence records loadout, modifiers, raw damage, mitigated damage, and victim health delta
- AND Stevenarella records the matching victim health update

### Requirement: Negative modifier scenarios

r[mc_compatibility.prove_armor_enchantment_status_matrix.negative_modifier_scenarios] Modifier verification MUST reject wrong loadout, stale equipment, missing modifier attribution, or mismatched health evidence.

#### Scenario: Weak modifier evidence fails

r[mc_compatibility.prove_armor_enchantment_status_matrix.negative_modifier_scenarios.scenario]
- GIVEN evidence lacks the selected loadout or modifier attribution
- WHEN the row is evaluated
- THEN the row fails with explicit diagnostics
- AND no modifier breadth claim is promoted

### Requirement: Modifier promotion gate

r[mc_compatibility.prove_armor_enchantment_status_matrix.modifier_promotion_gate] Armor loadout, enchantment, and status-effect breadth MUST remain a non-claim until required rows have passing tests, live receipts, BLAKE3 manifests, and updated evidence indexes.

#### Scenario: Modifier promotion is evidence complete

r[mc_compatibility.prove_armor_enchantment_status_matrix.modifier_promotion_gate.scenario]
- GIVEN modifier breadth is proposed as covered
- WHEN any required matrix row lacks passing evidence or a named oracle
- THEN the promotion is rejected
- AND exact vanilla parity remains a separate non-claim unless proven by its own oracle
