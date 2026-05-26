# Delta: Protocol-763 compatibility acceptance matrix and gap checklist

## Requirements

### Requirement: Rows Cover Landed Evidence

r[mc_compatibility.acceptance_matrix.rows_cover_landed_evidence] The system MUST list the landed protocol-763 Stevenarella-Valence evidence seams with receipt paths, commands, commit identifiers, BLAKE3 hashes when available, and scoped claims.

#### Scenario: Rows Cover Landed Evidence evidence is required

r[mc_compatibility.acceptance_matrix.rows_cover_landed_evidence.scenario]
- GIVEN the `Protocol-763 compatibility acceptance matrix and gap checklist` change is being drained
- WHEN the implementation and evidence are reviewed
- THEN the evidence satisfies `mc_compatibility.acceptance_matrix.rows_cover_landed_evidence`
- AND the receipt or documentation states scoped non-claims where the proof is bounded

### Requirement: Remaining Gaps Explicit

r[mc_compatibility.acceptance_matrix.remaining_gaps_explicit] The system MUST list remaining compatibility gaps and non-claims, including CTF edge cases, latency tolerance, projectile/armor/knockback combat, production load, and broad protocol coverage.

#### Scenario: Remaining Gaps Explicit evidence is required

r[mc_compatibility.acceptance_matrix.remaining_gaps_explicit.scenario]
- GIVEN the `Protocol-763 compatibility acceptance matrix and gap checklist` change is being drained
- WHEN the implementation and evidence are reviewed
- THEN the evidence satisfies `mc_compatibility.acceptance_matrix.remaining_gaps_explicit`
- AND the receipt or documentation states scoped non-claims where the proof is bounded

### Requirement: Machine Check

r[mc_compatibility.acceptance_matrix.machine_check] The system MUST include a deterministic cheap check that validates the matrix has required rows and required claim/non-claim fields.

#### Scenario: Machine Check evidence is required

r[mc_compatibility.acceptance_matrix.machine_check.scenario]
- GIVEN the `Protocol-763 compatibility acceptance matrix and gap checklist` change is being drained
- WHEN the implementation and evidence are reviewed
- THEN the evidence satisfies `mc_compatibility.acceptance_matrix.machine_check`
- AND the receipt or documentation states scoped non-claims where the proof is bounded

### Requirement: Roi Order Preserved

r[mc_compatibility.acceptance_matrix.roi_order_preserved] The system MUST preserve the current ROI ordering so future drain work starts with flag-carrier death/return unless live state changes.

#### Scenario: Roi Order Preserved evidence is required

r[mc_compatibility.acceptance_matrix.roi_order_preserved.scenario]
- GIVEN the `Protocol-763 compatibility acceptance matrix and gap checklist` change is being drained
- WHEN the implementation and evidence are reviewed
- THEN the evidence satisfies `mc_compatibility.acceptance_matrix.roi_order_preserved`
- AND the receipt or documentation states scoped non-claims where the proof is bounded
