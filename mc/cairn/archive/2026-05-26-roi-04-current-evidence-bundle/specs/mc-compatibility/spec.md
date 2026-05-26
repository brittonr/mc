# Delta: Current protocol-763 evidence bundle index

## Requirements

### Requirement: Bundle Index

r[mc_compatibility.roi_04_current_evidence_bundle.bundle_index] The docs MUST provide a current evidence bundle index for the protocol-763 Valence CTF seams.

#### Scenario: Bundle Index evidence is required

r[mc_compatibility.roi_04_current_evidence_bundle.bundle_index.scenario]
- GIVEN `Current protocol-763 evidence bundle index` is drained
- WHEN the evidence and checks are reviewed
- THEN `bundle_index` is satisfied by a tracked artifact or deterministic check
- AND scoped non-claims are preserved where relevant

### Requirement: Bundle Check

r[mc_compatibility.roi_04_current_evidence_bundle.bundle_check] A deterministic checker MUST validate that the bundle references the current matrix seams and hashes.

#### Scenario: Bundle Check evidence is required

r[mc_compatibility.roi_04_current_evidence_bundle.bundle_check.scenario]
- GIVEN `Current protocol-763 evidence bundle index` is drained
- WHEN the evidence and checks are reviewed
- THEN `bundle_check` is satisfied by a tracked artifact or deterministic check
- AND scoped non-claims are preserved where relevant
