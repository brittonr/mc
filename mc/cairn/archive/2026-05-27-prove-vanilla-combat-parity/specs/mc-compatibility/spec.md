# Delta: Vanilla combat parity proof

## ADDED Requirements

### Requirement: Reference oracle

r[mc_compatibility.prove_vanilla_combat_parity.reference_oracle] Any vanilla combat parity proof MUST name the reference oracle, version, configuration, and limitations before accepting parity evidence.

#### Scenario: Oracle is pinned and reviewable

r[mc_compatibility.prove_vanilla_combat_parity.reference_oracle.scenario]
- GIVEN a vanilla parity claim is proposed
- WHEN the proof records its reference oracle
- THEN it identifies the oracle implementation, version, configuration, evidence path, decision owner, and known limitations
- AND it rejects Valence-only evidence as a parity oracle

### Requirement: Parity metrics

r[mc_compatibility.prove_vanilla_combat_parity.parity_metrics] The parity proof MUST define named metrics and tolerances for each claimed combat behavior before comparing Valence and reference evidence.

#### Scenario: Metrics define comparison boundaries

r[mc_compatibility.prove_vanilla_combat_parity.parity_metrics.scenario]
- GIVEN a combat parity row is selected
- WHEN the comparison is evaluated
- THEN the row names the metric, tolerance, unit, reference value, Valence value, and rationale
- AND metrics without tolerances remain non-claims

### Requirement: Parity fixtures

r[mc_compatibility.prove_vanilla_combat_parity.parity_fixtures] The parity comparison logic MUST include positive and negative fixtures for equal-within-tolerance, out-of-tolerance, missing-reference, wrong-version, and Valence-only evidence.

#### Scenario: Weak parity evidence fails closed

r[mc_compatibility.prove_vanilla_combat_parity.parity_fixtures.scenario]
- GIVEN parity fixtures are executed
- WHEN evidence is out of tolerance, lacks a reference run, uses the wrong version, or contains only Valence data
- THEN the fixture fails with explicit diagnostics
- AND no parity claim is promoted

### Requirement: Parity promotion gate

r[mc_compatibility.prove_vanilla_combat_parity.parity_promotion_gate] Exact vanilla combat parity MUST remain a non-claim until paired reference and Valence receipts satisfy the metric/tolerance gate and are tracked with BLAKE3 manifests.

#### Scenario: Parity promotion requires paired evidence

r[mc_compatibility.prove_vanilla_combat_parity.parity_promotion_gate.scenario]
- GIVEN a parity row is proposed for the acceptance matrix
- WHEN paired reference and Valence evidence is missing or stale
- THEN promotion is rejected
- AND the current bundle keeps exact vanilla parity as a non-claim
