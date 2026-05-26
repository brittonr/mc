# Delta: Projectile, armor, and knockback combat semantics compatibility rail

## Requirements

### Requirement: First Mechanic Selected

r[mc_compatibility.combat_mechanics.first_mechanic_selected] The system MUST select one bounded first mechanic from projectile, armor, or knockback and document why it is independently drainable.

#### Scenario: First Mechanic Selected evidence is required

r[mc_compatibility.combat_mechanics.first_mechanic_selected.scenario]
- GIVEN the `Projectile, armor, and knockback combat semantics compatibility rail` change is being drained
- WHEN the implementation and evidence are reviewed
- THEN the evidence satisfies `mc_compatibility.combat_mechanics.first_mechanic_selected`
- AND the receipt or documentation states scoped non-claims where the proof is bounded

### Requirement: Server Client Correlation

r[mc_compatibility.combat_mechanics.server_client_correlation] The system MUST correlate server-side combat-mechanic events with client-side Stevenarella observations for the selected mechanic.

#### Scenario: Server Client Correlation evidence is required

r[mc_compatibility.combat_mechanics.server_client_correlation.scenario]
- GIVEN the `Projectile, armor, and knockback combat semantics compatibility rail` change is being drained
- WHEN the implementation and evidence are reviewed
- THEN the evidence satisfies `mc_compatibility.combat_mechanics.server_client_correlation`
- AND the receipt or documentation states scoped non-claims where the proof is bounded

### Requirement: Protocol Boundaries Recorded

r[mc_compatibility.combat_mechanics.protocol_boundaries_recorded] The system MUST record any newly exposed protocol packet mappings or parser gaps before claiming the mechanic receipt passes.

#### Scenario: Protocol Boundaries Recorded evidence is required

r[mc_compatibility.combat_mechanics.protocol_boundaries_recorded.scenario]
- GIVEN the `Projectile, armor, and knockback combat semantics compatibility rail` change is being drained
- WHEN the implementation and evidence are reviewed
- THEN the evidence satisfies `mc_compatibility.combat_mechanics.protocol_boundaries_recorded`
- AND the receipt or documentation states scoped non-claims where the proof is bounded

### Requirement: Receipt Gate

r[mc_compatibility.combat_mechanics.receipt_gate] The system MUST provide focused tests, a deterministic dry-run gate, live evidence, BLAKE3, hygiene scan, and non-claims for full combat correctness.

#### Scenario: Receipt Gate evidence is required

r[mc_compatibility.combat_mechanics.receipt_gate.scenario]
- GIVEN the `Projectile, armor, and knockback combat semantics compatibility rail` change is being drained
- WHEN the implementation and evidence are reviewed
- THEN the evidence satisfies `mc_compatibility.combat_mechanics.receipt_gate`
- AND the receipt or documentation states scoped non-claims where the proof is bounded
