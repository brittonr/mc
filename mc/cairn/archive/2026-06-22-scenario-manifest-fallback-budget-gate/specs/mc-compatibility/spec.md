# mc-compatibility Change Spec: Scenario manifest fallback budget gate

## Requirements

### Requirement: Scenario manifest fallback baseline

r[mc_compatibility.scenario_manifest_fallback_budget_gate.baseline] The system MUST record a deterministic baseline for scenario rows that still use substring fallback, including row identity and waiver metadata that names owner, reason, non-claim, and next action.

#### Scenario: Existing fallback rows are accounted for

r[mc_compatibility.scenario_manifest_fallback_budget_gate.baseline.accounted]
- GIVEN the scenario manifest contains existing substring-fallback rows
- WHEN the fallback budget gate evaluates the manifest
- THEN each existing fallback row is either present in the checked baseline with waiver metadata or reported as unapproved fallback debt.

### Requirement: Scenario manifest fallback gate

r[mc_compatibility.scenario_manifest_fallback_budget_gate.gate] The system MUST fail closed when the scenario manifest adds unapproved substring fallback rows, removes required waiver metadata, or regresses a typed-event-ready row back to substring fallback.

#### Scenario: New fallback row fails closed

r[mc_compatibility.scenario_manifest_fallback_budget_gate.gate.new_fallback]
- GIVEN a scenario manifest adds a row with `migration_state` set to `substring-fallback`
- WHEN the row is not in the approved fallback baseline with complete waiver metadata
- THEN the fallback budget gate fails with a diagnostic naming the row.

#### Scenario: Typed-event regression fails closed

r[mc_compatibility.scenario_manifest_fallback_budget_gate.gate.regression]
- GIVEN a scenario row is recorded as typed-event-ready in the baseline
- WHEN the manifest changes that row back to substring fallback
- THEN the fallback budget gate fails with a typed-event regression diagnostic.

#### Scenario: Fallback removal is reported as progress

r[mc_compatibility.scenario_manifest_fallback_budget_gate.gate.removal]
- GIVEN a row is removed from substring fallback by a typed-event migration
- WHEN the fallback budget gate evaluates the manifest
- THEN the gate passes and reports the row as fallback debt removed.

### Requirement: Scenario manifest fallback gate integration

r[mc_compatibility.scenario_manifest_fallback_budget_gate.integration] The fallback budget gate MUST run as part of the focused mc-compat validation surface without changing scenario behavior, wrapper selection, or evidence claims.

#### Scenario: Focused validation includes fallback accounting

r[mc_compatibility.scenario_manifest_fallback_budget_gate.integration.focused]
- GIVEN focused mc-compat validation runs
- WHEN scenario manifest checks execute
- THEN fallback budget accounting runs against the checked-in manifest surfaces and reports approved, removed, new, and regressed rows.

### Requirement: Scenario manifest fallback gate documentation and evidence

r[mc_compatibility.scenario_manifest_fallback_budget_gate.docs] The change MUST document the fallback budget report shape and preserve explicit non-claims for unmigrated rows.

#### Scenario: Documentation keeps fallback rows non-claiming

r[mc_compatibility.scenario_manifest_fallback_budget_gate.docs.non_claiming]
- GIVEN fallback budget documentation is updated
- WHEN reviewers read the fallback report description
- THEN it states that fallback accounting does not prove typed-event coverage, live compatibility, semantic equivalence, public-server safety, or production readiness.

### Requirement: Scenario manifest fallback gate validation

r[mc_compatibility.scenario_manifest_fallback_budget_gate.validation] The change MUST record reviewable evidence for positive and negative gate fixtures, generated-surface freshness, evidence manifests, Cairn gates, and Cairn validation before archive.

#### Scenario: Validation evidence is complete

r[mc_compatibility.scenario_manifest_fallback_budget_gate.validation.log]
- GIVEN the fallback budget gate is implemented
- WHEN reviewers inspect the task evidence
- THEN logs show positive and negative gate fixtures, generated-surface freshness, evidence manifest validation, Cairn proposal/design/tasks gates, and Cairn validation.
