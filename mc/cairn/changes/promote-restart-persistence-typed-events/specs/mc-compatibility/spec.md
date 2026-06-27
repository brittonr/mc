# mc-compatibility Change Spec: Restart persistence typed events

## Requirements

### Requirement: Restart persistence fallback inventory

r[mc_compatibility.restart_persistence_typed_events.inventory] The change MUST inventory selected restart persistence rows, current substring fallback behavior, existing evidence, session count, and non-claim boundaries before migration.

#### Scenario: Selected rows are scoped

r[mc_compatibility.restart_persistence_typed_events.inventory.scoped]
- GIVEN restart persistence typed-event work begins
- WHEN reviewers inspect the inventory
- THEN it names the selected scenarios, current fallback evidence, required sessions, existing receipts, and explicit non-claims.

### Requirement: Restart persistence typed contract

r[mc_compatibility.restart_persistence_typed_events.contract] Selected restart persistence rows MUST define typed milestones for pre-boundary mutation, restart or crash boundary, reconnect when required, post-boundary client observation, and server restored state.

#### Scenario: Boundary sequence is explicit

r[mc_compatibility.restart_persistence_typed_events.contract.sequence]
- GIVEN a selected restart persistence scenario runs
- WHEN its receipt is evaluated
- THEN the typed milestones identify the pre-boundary mutation, boundary event, reconnect requirement, post-boundary observation, and restored server state
- AND the receipt keeps arbitrary durability and full survival compatibility as non-claims.

### Requirement: Pure restart persistence validator

r[mc_compatibility.restart_persistence_typed_events.validator] Restart persistence validation MUST be a pure deterministic core over normalized receipt events and MUST fail closed for missing, unordered, duplicate, mismatched, or stale milestones.

#### Scenario: Complete restart persistence receipt passes

r[mc_compatibility.restart_persistence_typed_events.validator.positive]
- GIVEN a selected scenario receipt contains complete ordered client and server milestones with matching restored state
- WHEN the validator evaluates the receipt
- THEN validation passes with stable diagnostics.

#### Scenario: Weak restart persistence receipt fails

r[mc_compatibility.restart_persistence_typed_events.validator.negative]
- GIVEN a selected scenario receipt is missing the boundary milestone, reconnect milestone, post-boundary observation, or matching restored server state
- WHEN the validator evaluates the receipt
- THEN validation fails and names the missing or mismatched milestone.

### Requirement: Restart persistence typed wiring

r[mc_compatibility.restart_persistence_typed_events.wiring] Runner, client, and server fixture shells MUST emit the selected typed milestones without changing maintained wrapper names, scenario names, or bounded live claims.

#### Scenario: Substring fallback is no longer required

r[mc_compatibility.restart_persistence_typed_events.wiring.no_substring]
- GIVEN typed milestone emission is wired for a selected row
- WHEN the scenario validation runs
- THEN promotion checks depend on typed receipt fields rather than substring log matching
- AND raw logs remain review evidence only.

### Requirement: Restart persistence manifest migration

r[mc_compatibility.restart_persistence_typed_events.manifest] Scenario manifest migration states, fallback budget baseline, and generated surfaces MUST update only after typed validation for the selected rows passes.

#### Scenario: Manifest reflects typed readiness

r[mc_compatibility.restart_persistence_typed_events.manifest.ready]
- GIVEN typed validation passes for a selected row
- WHEN generated scenario surfaces are refreshed
- THEN the row is marked typed-event-ready and removed from approved fallback debt
- AND non-claim fields remain visible.

### Requirement: Restart persistence closeout

r[mc_compatibility.restart_persistence_typed_events.closeout] The change MUST record focused receipt checks, scenario manifest checks, generated-surface freshness, evidence manifests, task-evidence validation, Cairn gates, and Cairn validation before archive.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.restart_persistence_typed_events.closeout.log]
- GIVEN selected restart persistence rows have migrated
- WHEN reviewers inspect task evidence
- THEN logs show positive and negative typed validation fixtures, focused scenario checks, scenario manifest checks, generated-surface freshness, evidence manifest validation, task-evidence validation, Cairn gates, and Cairn validation.
