# mc-compatibility Change Spec: Survival crafting-table typed-event migration

## Requirements

### Requirement: Survival crafting-table typed-event readiness

r[mc_compatibility.survival_crafting_table_typed_event_migration.readiness] The `survival-crafting-table` scenario SHOULD move from waiver-backed substring fallback to `typed-event-ready` only when typed events cover the row's required client milestones, Valence crafting-table server milestones, forbidden surfaces, and ordered crafting phases.

#### Scenario: Survival crafting-table row is typed-event-ready

r[mc_compatibility.survival_crafting_table_typed_event_migration.readiness.complete]
- GIVEN `survival-crafting-table` is marked `typed-event-ready`
- WHEN the scenario manifest and generated runner surfaces are inspected
- THEN the row includes typed-event-ready receipt expectations and no longer uses substring fallback for pass/fail
- AND the manifest still records the existing wrapper, dry-run check, current-bundle row, and non-claim scope.

### Requirement: Survival crafting-table typed-event gate

r[mc_compatibility.survival_crafting_table_typed_event_migration.gate] The runner MUST include `survival-crafting-table` in the typed-event pass/fail gate so missing or invalid structured crafting events fail before substring fallback can satisfy the row.

#### Scenario: Missing typed crafting evidence fails closed

r[mc_compatibility.survival_crafting_table_typed_event_migration.gate.missing]
- GIVEN a survival crafting-table fixture contains legacy substring-compatible milestones but omits a required typed crafting event
- WHEN typed-event validation evaluates `survival-crafting-table`
- THEN the fixture fails with a structured diagnostic naming the missing event.

#### Scenario: Misordered typed crafting phases fail closed

r[mc_compatibility.survival_crafting_table_typed_event_migration.gate.order]
- GIVEN a survival crafting-table fixture contains all required typed crafting events but puts the server collect phase before the required result phase
- WHEN typed-event validation evaluates `survival-crafting-table`
- THEN the fixture fails with an ordering diagnostic instead of passing through substring fallback.

### Requirement: Survival crafting-table migration evidence

r[mc_compatibility.survival_crafting_table_typed_event_migration.validation] The migration MUST record reviewable evidence for runner fixtures, scenario-manifest checks, generated-surface freshness, dry-run receipt shape, evidence manifests, Cairn gates, and Cairn validation before archive.

#### Scenario: Validation evidence is complete

r[mc_compatibility.survival_crafting_table_typed_event_migration.validation.log]
- GIVEN the survival crafting-table row is migrated to typed-event-ready
- WHEN reviewers inspect the task evidence
- THEN logs show positive and negative typed-event fixtures, typed-event-ready manifest accounting, generated-surface freshness, the survival crafting-table dry-run wrapper check, evidence manifest validation, Cairn proposal/design/tasks gates, and Cairn validation.
