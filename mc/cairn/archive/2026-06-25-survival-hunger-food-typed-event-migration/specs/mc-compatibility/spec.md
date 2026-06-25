# mc-compatibility Change Spec: Survival hunger/food typed-event migration

## Requirements

### Requirement: Survival hunger/food typed-event readiness

r[mc_compatibility.survival_hunger_food_typed_event_migration.readiness] The `survival-hunger-food` scenario SHOULD move from waiver-backed substring fallback to `typed-event-ready` only when typed events cover the row's required client milestones, Valence server milestones, forbidden surfaces, and ordered phases.

#### Scenario: Survival hunger/food row is typed-event-ready

r[mc_compatibility.survival_hunger_food_typed_event_migration.readiness.complete]
- GIVEN `survival-hunger-food` is marked `typed-event-ready`
- WHEN the scenario manifest and generated runner surfaces are inspected
- THEN `survival-hunger-food` includes the `typed-event-ready` receipt expectation
- AND the manifest still records its existing wrapper, dry-run check, current-bundle row, and non-claim scope.

### Requirement: Survival hunger/food typed-event gate

r[mc_compatibility.survival_hunger_food_typed_event_migration.gate] The runner MUST include `survival-hunger-food` in typed-event pass/fail gates so missing or invalid structured row evidence fails before substring fallback can satisfy the row.

#### Scenario: Missing hunger/food typed evidence fails closed

r[mc_compatibility.survival_hunger_food_typed_event_migration.gate.missing]
- GIVEN a `survival-hunger-food` fixture contains legacy substring-compatible milestones but omits a row-required typed event
- WHEN typed-event validation evaluates that row
- THEN the fixture fails with a structured diagnostic naming the missing event and row.

#### Scenario: Misordered hunger/food phases fail closed

r[mc_compatibility.survival_hunger_food_typed_event_migration.gate.order]
- GIVEN a `survival-hunger-food` fixture contains all required typed events but puts a post-consumption or inventory event before its prerequisite phase
- WHEN typed-event validation evaluates that row
- THEN the fixture fails with an ordering diagnostic instead of passing through substring fallback.

### Requirement: Survival hunger/food migration evidence

r[mc_compatibility.survival_hunger_food_typed_event_migration.validation] The migration MUST record reviewable evidence for runner typed-event fixtures, scenario-manifest checks, generated-surface freshness, fallback accounting, Cairn gates, task evidence, and Cairn validation before archive.

#### Scenario: Validation evidence is complete

r[mc_compatibility.survival_hunger_food_typed_event_migration.validation.log]
- GIVEN `survival-hunger-food` is migrated to typed-event-ready
- WHEN reviewers inspect the task evidence
- THEN logs show positive and negative typed-event fixtures, typed-event-ready manifest accounting, generated-surface freshness, task-evidence validation, Cairn proposal/design/tasks gates, and Cairn validation.
