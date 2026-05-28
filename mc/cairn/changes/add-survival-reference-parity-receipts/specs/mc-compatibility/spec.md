# Delta: Survival reference parity receipts

## Requirements

### Requirement: Survival parity metrics

r[mc_compatibility.survival_reference_parity.metrics] The survival parity rail MUST define normalized exact-match metrics before comparing Valence with a reference backend.

#### Scenario: Metrics are explicit

r[mc_compatibility.survival_reference_parity.metrics.explicit]
- GIVEN the break/place/pickup parity rail is evaluated
- WHEN the comparator reads the receipts
- THEN it compares explicit join, break, pickup/inventory, and placement fields
- AND it does not infer parity from raw log similarity alone.

### Requirement: Reference receipt

r[mc_compatibility.survival_reference_parity.reference_receipt] The rail MUST produce a reviewable local reference-server receipt for the same Stevenarella survival probe.

#### Scenario: Reference receipt is reviewable

r[mc_compatibility.survival_reference_parity.reference_receipt.reviewable]
- GIVEN the reference backend run completes
- WHEN evidence is promoted
- THEN the reference receipt and logs are copied under `docs/evidence/`
- AND BLAKE3 hashes are recorded.

### Requirement: Valence receipt

r[mc_compatibility.survival_reference_parity.valence_receipt] The rail MUST produce a matching Valence receipt from committed child revisions.

#### Scenario: Valence receipt is paired

r[mc_compatibility.survival_reference_parity.valence_receipt.paired]
- GIVEN the reference receipt exists
- WHEN the Valence receipt is generated
- THEN it uses the same scenario, username, target coordinates, and normalized metric names
- AND it records committed Valence and Stevenarella child revisions.

### Requirement: Parity comparator

r[mc_compatibility.survival_reference_parity.comparator] A deterministic checker MUST compare the paired receipts and fail on missing or mismatched metrics.

#### Scenario: Mismatch rejects parity

r[mc_compatibility.survival_reference_parity.comparator.rejects]
- GIVEN a paired receipt has a missing or changed normalized metric
- WHEN the comparator runs
- THEN it fails and names the mismatched metric.

### Requirement: Parity non-claims

r[mc_compatibility.survival_reference_parity.nonclaims] The paired break/place/pickup parity row MUST NOT claim full survival compatibility, broad vanilla parity, or production readiness.

#### Scenario: Non-claims remain explicit

r[mc_compatibility.survival_reference_parity.nonclaims.explicit]
- GIVEN the narrow parity row is promoted
- WHEN the evidence doc is reviewed
- THEN full survival compatibility and broad vanilla parity remain explicit non-claims.
