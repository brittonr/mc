# mc-compatibility Change Spec: Scenario fallback budget ratchet

## Requirements

### Requirement: Fallback ratchet inventory

r[mc_compatibility.scenario_fallback_budget_ratchet.inventory] The system MUST inventory current scenario manifest migration states against the checked fallback-budget baseline before changing approved fallback rows.

#### Scenario: Stale fallback approvals are visible

r[mc_compatibility.scenario_fallback_budget_ratchet.inventory.visible]
- GIVEN the scenario manifest and fallback baseline are evaluated together
- WHEN a baseline fallback entry no longer corresponds to a current substring-fallback row
- THEN the inventory names the row as migrated or stale fallback approval
- AND it keeps current unapproved fallback rows separate from migrated rows.

### Requirement: Ratcheted fallback baseline

r[mc_compatibility.scenario_fallback_budget_ratchet.baseline] The fallback-budget baseline MUST approve only current substring-fallback rows with complete owner, reason, non-claim, and next-action metadata, while preserving regression protection for rows that have migrated to typed-event-ready.

#### Scenario: Migrated rows leave the approved fallback list

r[mc_compatibility.scenario_fallback_budget_ratchet.baseline.migrated]
- GIVEN a row is typed-event-ready in the current manifest
- WHEN the fallback baseline is ratcheted
- THEN the row is not listed as an approved fallback row
- AND future movement back to substring fallback remains a fail-closed regression unless explicitly re-waived.

### Requirement: Ratchet gate behavior

r[mc_compatibility.scenario_fallback_budget_ratchet.gate] The scenario manifest checker MUST fail closed for unapproved current fallback rows, incomplete waiver metadata, and typed-event-ready regression after the ratchet.

#### Scenario: Removed fallback row cannot silently return

r[mc_compatibility.scenario_fallback_budget_ratchet.gate.removed_returns]
- GIVEN a migrated row was removed from the approved fallback list
- WHEN the manifest changes that row back to substring fallback without a new complete waiver
- THEN the checker fails with a diagnostic naming the row.

#### Scenario: Current fallback waiver remains required

r[mc_compatibility.scenario_fallback_budget_ratchet.gate.waiver]
- GIVEN a current fallback row remains approved
- WHEN the baseline lacks owner, reason, non-claim, or next-action metadata for that row
- THEN the checker fails with a missing-waiver diagnostic.

### Requirement: Ratchet generated surfaces and docs

r[mc_compatibility.scenario_fallback_budget_ratchet.surfaces] Generated scenario surfaces and fallback-budget documentation MUST be refreshed so reviewers can see the current fallback set and the non-claim boundary.

#### Scenario: Documentation describes accounting only

r[mc_compatibility.scenario_fallback_budget_ratchet.surfaces.docs]
- GIVEN fallback-budget docs or generated indexes are reviewed
- WHEN the ratcheted fallback set is displayed
- THEN the docs state that fallback accounting does not prove typed-event coverage, live compatibility, semantic equivalence, public-server safety, or production readiness.

### Requirement: Ratchet validation evidence

r[mc_compatibility.scenario_fallback_budget_ratchet.validation] The change MUST record reviewable evidence for checker fixtures, generated-surface freshness, evidence manifests, task-evidence validation, Cairn gates, and Cairn validation before archive.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.scenario_fallback_budget_ratchet.validation.log]
- GIVEN the fallback budget has been ratcheted
- WHEN reviewers inspect the task evidence
- THEN logs show positive and negative checker fixtures, generated-surface freshness, evidence manifest validation, Cairn proposal/design/tasks gates, task-evidence validation, and Cairn validation.
