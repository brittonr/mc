# mc-compatibility Change Spec: Remaining survival breadth typed-event migration

## Requirements

### Requirement: Remaining survival breadth typed-event readiness

r[mc_compatibility.survival_remaining_breadth_typed_event_migration.readiness] The remaining survival breadth scenarios SHOULD move from waiver-backed substring fallback to `typed-event-ready` only when typed events cover each row's required client milestones, Valence server milestones, forbidden surfaces, and row-specific ordered phases.

#### Scenario: Remaining survival breadth rows are typed-event-ready

r[mc_compatibility.survival_remaining_breadth_typed_event_migration.readiness.complete]
- GIVEN the remaining survival breadth rows are marked `typed-event-ready`
- WHEN the scenario manifest and generated runner surfaces are inspected
- THEN `survival-mob-ai-loot-breadth`, `survival-redstone-circuit-breadth`, `survival-biome-dimension-travel`, `survival-world-multichunk-durability`, `survival-container-block-entity-breadth`, and `survival-sign-editing-live` include typed-event-ready receipt expectations
- AND the manifest still records each existing wrapper, dry-run check, current-bundle row, and non-claim scope.

### Requirement: Remaining survival breadth typed-event gates

r[mc_compatibility.survival_remaining_breadth_typed_event_migration.gate] The runner MUST include the remaining survival breadth rows in typed-event pass/fail gates so missing or invalid structured row evidence fails before substring fallback can satisfy a row.

#### Scenario: Missing survival breadth typed evidence fails closed

r[mc_compatibility.survival_remaining_breadth_typed_event_migration.gate.missing]
- GIVEN a remaining survival breadth fixture contains legacy substring-compatible milestones but omits a row-required typed event
- WHEN typed-event validation evaluates that row
- THEN the fixture fails with a structured diagnostic naming the missing event and row.

#### Scenario: Misordered survival breadth phases fail closed

r[mc_compatibility.survival_remaining_breadth_typed_event_migration.gate.order]
- GIVEN a remaining survival breadth fixture contains all required typed events but puts a row postcondition before the required action phase
- WHEN typed-event validation evaluates that row
- THEN the fixture fails with an ordering diagnostic instead of passing through substring fallback.

### Requirement: Remaining survival breadth migration evidence

r[mc_compatibility.survival_remaining_breadth_typed_event_migration.validation] The migration MUST record reviewable evidence for runner fixtures, scenario-manifest checks, generated-surface freshness, dry-run coverage, evidence manifests, Cairn gates, and Cairn validation before archive.

#### Scenario: Validation evidence is complete

r[mc_compatibility.survival_remaining_breadth_typed_event_migration.validation.log]
- GIVEN the remaining survival breadth rows are migrated to typed-event-ready
- WHEN reviewers inspect the task evidence
- THEN logs show positive and negative typed-event fixtures, typed-event-ready manifest accounting, generated-surface freshness, historical survival dry-run coverage, evidence manifest validation, Cairn proposal/design/tasks gates, and Cairn validation.
