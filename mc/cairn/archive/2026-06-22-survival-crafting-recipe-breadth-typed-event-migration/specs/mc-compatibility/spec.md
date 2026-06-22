# mc-compatibility Change Spec: Survival crafting recipe breadth typed-event migration

## Requirements

### Requirement: Survival crafting recipe breadth typed-event readiness

r[mc_compatibility.survival_crafting_recipe_breadth_typed_event_migration.readiness] The `survival-crafting-recipe-breadth` scenario SHOULD move from waiver-backed substring fallback to `typed-event-ready` only when typed events cover the row's required client milestones, Valence recipe-breadth server milestones, forbidden surfaces, and ordered valid/invalid crafting phases.

#### Scenario: Survival crafting recipe breadth row is typed-event-ready

r[mc_compatibility.survival_crafting_recipe_breadth_typed_event_migration.readiness.complete]
- GIVEN `survival-crafting-recipe-breadth` is marked `typed-event-ready`
- WHEN the scenario manifest and generated runner surfaces are inspected
- THEN the row includes typed-event-ready receipt expectations and no longer uses substring fallback for pass/fail
- AND the manifest still records the existing wrapper, dry-run check, current-bundle row, and non-claim scope.

### Requirement: Survival crafting recipe breadth typed-event gate

r[mc_compatibility.survival_crafting_recipe_breadth_typed_event_migration.gate] The runner MUST include `survival-crafting-recipe-breadth` in the typed-event pass/fail gate so missing or invalid structured recipe events fail before substring fallback can satisfy the row.

#### Scenario: Missing typed recipe evidence fails closed

r[mc_compatibility.survival_crafting_recipe_breadth_typed_event_migration.gate.missing]
- GIVEN a survival recipe-breadth fixture contains legacy substring-compatible milestones but omits a required invalid-rejection typed event
- WHEN typed-event validation evaluates `survival-crafting-recipe-breadth`
- THEN the fixture fails with a structured diagnostic naming the missing event.

#### Scenario: Misordered typed recipe phases fail closed

r[mc_compatibility.survival_crafting_recipe_breadth_typed_event_migration.gate.order]
- GIVEN a survival recipe-breadth fixture contains all required typed recipe events but reorders the shapeless result and grid-clear phases
- WHEN typed-event validation evaluates `survival-crafting-recipe-breadth`
- THEN the fixture fails with an ordering diagnostic instead of passing through substring fallback.

### Requirement: Survival crafting recipe breadth migration evidence

r[mc_compatibility.survival_crafting_recipe_breadth_typed_event_migration.validation] The migration MUST record reviewable evidence for runner fixtures, scenario-manifest checks, generated-surface freshness, dry-run receipt shape, evidence manifests, Cairn gates, and Cairn validation before archive.

#### Scenario: Validation evidence is complete

r[mc_compatibility.survival_crafting_recipe_breadth_typed_event_migration.validation.log]
- GIVEN the survival crafting recipe breadth row is migrated to typed-event-ready
- WHEN reviewers inspect the task evidence
- THEN logs show positive and negative typed-event fixtures, typed-event-ready manifest accounting, generated-surface freshness, the recipe-breadth dry-run wrapper check, evidence manifest validation, Cairn proposal/design/tasks gates, and Cairn validation.
