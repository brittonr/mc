# mc-compatibility Change Spec: Survival hunger health-cycle typed-event migration

## Requirements

### Requirement: Survival hunger health-cycle typed-event readiness

r[mc_compatibility.survival_hunger_health_cycle_typed_event_migration.readiness] The `survival-hunger-health-cycle` scenario SHOULD move from waiver-backed substring fallback to `typed-event-ready` only when typed events cover the row's required client milestones, Valence hunger/health server milestones, forbidden surfaces, and ordered consume phases.

#### Scenario: Survival hunger health-cycle row is typed-event-ready

r[mc_compatibility.survival_hunger_health_cycle_typed_event_migration.readiness.complete]
- GIVEN `survival-hunger-health-cycle` is marked `typed-event-ready`
- WHEN the scenario manifest and generated runner surfaces are inspected
- THEN the row includes typed-event-ready receipt expectations and no longer uses substring fallback for pass/fail
- AND the manifest still records the existing wrapper, dry-run check, current-bundle row, and non-claim scope.

### Requirement: Survival hunger health-cycle typed-event gate

r[mc_compatibility.survival_hunger_health_cycle_typed_event_migration.gate] The runner MUST include `survival-hunger-health-cycle` in the typed-event pass/fail gate so missing or invalid structured hunger/health events fail before substring fallback can satisfy the row.

#### Scenario: Missing typed hunger health-cycle evidence fails closed

r[mc_compatibility.survival_hunger_health_cycle_typed_event_migration.gate.missing]
- GIVEN a survival hunger health-cycle fixture contains legacy substring-compatible milestones but omits a required final health/food/saturation typed event
- WHEN typed-event validation evaluates `survival-hunger-health-cycle`
- THEN the fixture fails with a structured diagnostic naming the missing event.

#### Scenario: Misordered typed hunger health-cycle phases fail closed

r[mc_compatibility.survival_hunger_health_cycle_typed_event_migration.gate.order]
- GIVEN a survival hunger health-cycle fixture contains all required typed events but puts inventory decrement before consume finish
- WHEN typed-event validation evaluates `survival-hunger-health-cycle`
- THEN the fixture fails with an ordering diagnostic instead of passing through substring fallback.

### Requirement: Survival hunger health-cycle migration evidence

r[mc_compatibility.survival_hunger_health_cycle_typed_event_migration.validation] The migration MUST record reviewable evidence for runner fixtures, scenario-manifest checks, generated-surface freshness, dry-run receipt shape, evidence manifests, Cairn gates, and Cairn validation before archive.

#### Scenario: Validation evidence is complete

r[mc_compatibility.survival_hunger_health_cycle_typed_event_migration.validation.log]
- GIVEN the survival hunger health-cycle row is migrated to typed-event-ready
- WHEN reviewers inspect the task evidence
- THEN logs show positive and negative typed-event fixtures, typed-event-ready manifest accounting, generated-surface freshness, the hunger health-cycle dry-run wrapper check, evidence manifest validation, Cairn proposal/design/tasks gates, and Cairn validation.
