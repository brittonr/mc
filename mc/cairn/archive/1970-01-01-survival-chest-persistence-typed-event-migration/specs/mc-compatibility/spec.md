# mc-compatibility Change Spec: Survival chest persistence typed-event migration

## Requirements

### Requirement: Survival chest persistence typed-event readiness

r[mc_compatibility.survival_chest_persistence_typed_event_migration.readiness] The `survival-chest-persistence` scenario SHOULD move from waiver-backed substring fallback to `typed-event-ready` only when typed events cover the row's required client milestones, Valence chest server milestones, forbidden surfaces, and ordered two-session persistence phases.

#### Scenario: Survival chest persistence row is typed-event-ready

r[mc_compatibility.survival_chest_persistence_typed_event_migration.readiness.complete]
- GIVEN `survival-chest-persistence` is marked `typed-event-ready`
- WHEN the scenario manifest and generated runner surfaces are inspected
- THEN the row includes typed-event-ready receipt expectations and no longer uses substring fallback for pass/fail
- AND the manifest still records the existing wrapper, dry-run check, current-bundle row, and non-claim scope.

### Requirement: Survival chest persistence typed-event gate

r[mc_compatibility.survival_chest_persistence_typed_event_migration.gate] The runner MUST include `survival-chest-persistence` in the typed-event pass/fail gate so missing or invalid structured chest events fail before substring fallback can satisfy the row.

#### Scenario: Missing typed chest evidence fails closed

r[mc_compatibility.survival_chest_persistence_typed_event_migration.gate.missing]
- GIVEN a survival chest fixture contains legacy substring-compatible milestones but omits a required persisted-state typed event
- WHEN typed-event validation evaluates `survival-chest-persistence`
- THEN the fixture fails with a structured diagnostic naming the missing event.

#### Scenario: Misordered two-session chest phases fail closed

r[mc_compatibility.survival_chest_persistence_typed_event_migration.gate.order]
- GIVEN a survival chest fixture contains all required typed chest events but puts second-session reopen before the required first-session close phase
- WHEN typed-event validation evaluates `survival-chest-persistence`
- THEN the fixture fails with an ordering diagnostic instead of passing through substring fallback.

### Requirement: Survival chest persistence migration evidence

r[mc_compatibility.survival_chest_persistence_typed_event_migration.validation] The migration MUST record reviewable evidence for runner fixtures, scenario-manifest checks, generated-surface freshness, dry-run receipt shape, evidence manifests, Cairn gates, and Cairn validation before archive.

#### Scenario: Validation evidence is complete

r[mc_compatibility.survival_chest_persistence_typed_event_migration.validation.log]
- GIVEN the survival chest persistence row is migrated to typed-event-ready
- WHEN reviewers inspect the task evidence
- THEN logs show positive and negative typed-event fixtures, typed-event-ready manifest accounting, generated-surface freshness, the survival chest dry-run wrapper check, evidence manifest validation, Cairn proposal/design/tasks gates, and Cairn validation.
