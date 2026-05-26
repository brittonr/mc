# Delta: Scenario triage receipt blocks

## Requirements

### Requirement: Triage Block

r[mc_compatibility.roi_04_scenario_triage_receipts.triage_block] Scenario receipts MUST include a deterministic triage block that identifies first missing client/server milestone, first forbidden pattern/source, relevant log paths, and suggested boundary when a scenario fails or dry-runs.

#### Scenario: Triage Block evidence is required

r[mc_compatibility.roi_04_scenario_triage_receipts.triage_block.scenario]
- GIVEN `Scenario triage receipt blocks` is drained
- WHEN the maintained compatibility evidence is reviewed
- THEN `triage_block` is satisfied by tracked evidence or a deterministic check
- AND scoped non-claims are preserved where relevant

### Requirement: Triage Tests

r[mc_compatibility.roi_04_scenario_triage_receipts.triage_tests] Runner tests MUST cover at least one missing-milestone/forbidden-pattern fixture so triage fields cannot silently regress.

#### Scenario: Triage Tests evidence is required

r[mc_compatibility.roi_04_scenario_triage_receipts.triage_tests.scenario]
- GIVEN `Scenario triage receipt blocks` is drained
- WHEN the maintained compatibility evidence is reviewed
- THEN `triage_tests` is satisfied by tracked evidence or a deterministic check
- AND scoped non-claims are preserved where relevant

### Requirement: Docs Check

r[mc_compatibility.roi_04_scenario_triage_receipts.docs_check] README or evidence docs MUST document the triage fields and a deterministic check MUST validate the receipt contract.

#### Scenario: Docs Check evidence is required

r[mc_compatibility.roi_04_scenario_triage_receipts.docs_check.scenario]
- GIVEN `Scenario triage receipt blocks` is drained
- WHEN the maintained compatibility evidence is reviewed
- THEN `docs_check` is satisfied by tracked evidence or a deterministic check
- AND scoped non-claims are preserved where relevant
