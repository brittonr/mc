# Delta: Current-head representative live refresh

## Requirements

### Requirement: Representative Receipt

r[mc_compatibility.roi_05_current_head_live_refresh.representative_receipt] The repo MUST refresh one representative maintained live protocol-763 receipt at current HEAD and record its BLAKE3 in tracked evidence.

#### Scenario: Representative Receipt evidence is required

r[mc_compatibility.roi_05_current_head_live_refresh.representative_receipt.scenario]
- GIVEN `Current-head representative live refresh` is drained
- WHEN the maintained compatibility evidence is reviewed
- THEN `representative_receipt` is satisfied by tracked evidence or a deterministic check
- AND scoped non-claims are preserved where relevant

### Requirement: Bundle Alignment

r[mc_compatibility.roi_05_current_head_live_refresh.bundle_alignment] The current evidence bundle or companion note MUST identify the refreshed receipt, payload commit, maintained command, and scoped non-claims without moving unrelated historical evidence.

#### Scenario: Bundle Alignment evidence is required

r[mc_compatibility.roi_05_current_head_live_refresh.bundle_alignment.scenario]
- GIVEN `Current-head representative live refresh` is drained
- WHEN the maintained compatibility evidence is reviewed
- THEN `bundle_alignment` is satisfied by tracked evidence or a deterministic check
- AND scoped non-claims are preserved where relevant

### Requirement: Verification

r[mc_compatibility.roi_05_current_head_live_refresh.verification] The refresh MUST run the relevant maintained app/check plus acceptance-matrix/current-bundle validation and leave parent/child repos clean.

#### Scenario: Verification evidence is required

r[mc_compatibility.roi_05_current_head_live_refresh.verification.scenario]
- GIVEN `Current-head representative live refresh` is drained
- WHEN the maintained compatibility evidence is reviewed
- THEN `verification` is satisfied by tracked evidence or a deterministic check
- AND scoped non-claims are preserved where relevant
