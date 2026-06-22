# mc-compatibility Change Spec: Inventory drag typed-event migration

## Requirements

### Requirement: Inventory drag typed-event readiness

r[mc_compatibility.inventory_drag_typed_event_migration.readiness] The `inventory-drag-transactions` scenario SHOULD move from waiver-backed substring fallback to `typed-event-ready` only when typed events cover the row's required client milestones, Valence quick-craft server milestones, forbidden surfaces, and ordered drag phases.

#### Scenario: Inventory drag row is typed-event-ready

r[mc_compatibility.inventory_drag_typed_event_migration.readiness.complete]
- GIVEN `inventory-drag-transactions` is marked `typed-event-ready`
- WHEN the scenario manifest and generated runner surfaces are inspected
- THEN the row includes typed-event-ready receipt expectations and no longer uses substring fallback for pass/fail
- AND the manifest still records the existing wrapper, dry-run check, current-bundle row, and non-claim scope.

### Requirement: Inventory drag typed-event gate

r[mc_compatibility.inventory_drag_typed_event_migration.gate] The runner MUST include `inventory-drag-transactions` in the typed-event pass/fail gate so missing or invalid structured drag events fail before substring fallback can satisfy the row.

#### Scenario: Missing typed drag evidence fails closed

r[mc_compatibility.inventory_drag_typed_event_migration.gate.missing]
- GIVEN a drag receipt fixture contains legacy substring-compatible milestones but omits a required typed drag event
- WHEN typed-event validation evaluates `inventory-drag-transactions`
- THEN the fixture fails with a structured diagnostic naming the missing event.

#### Scenario: Misordered typed drag phases fail closed

r[mc_compatibility.inventory_drag_typed_event_migration.gate.order]
- GIVEN a drag receipt fixture contains all required typed drag events but puts the quick-craft end before a required target phase
- WHEN typed-event validation evaluates `inventory-drag-transactions`
- THEN the fixture fails with an ordering diagnostic instead of passing through substring fallback.

### Requirement: Inventory drag migration evidence

r[mc_compatibility.inventory_drag_typed_event_migration.validation] The migration MUST record reviewable evidence for runner fixtures, scenario-manifest checks, generated-surface freshness, dry-run receipt shape, evidence manifests, Cairn gates, and Cairn validation before archive.

#### Scenario: Validation evidence is complete

r[mc_compatibility.inventory_drag_typed_event_migration.validation.log]
- GIVEN the inventory drag row is migrated to typed-event-ready
- WHEN reviewers inspect the task evidence
- THEN logs show positive and negative typed-event fixtures, typed-event-ready manifest accounting, generated-surface freshness, the inventory-drag dry-run wrapper check, evidence manifest validation, Cairn proposal/design/tasks gates, and Cairn validation.
