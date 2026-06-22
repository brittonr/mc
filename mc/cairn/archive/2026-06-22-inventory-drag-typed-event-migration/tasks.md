# Tasks

- [x] [serial] Record the pre-change baseline for runner tests, scenario manifest checks, and generated-surface freshness. r[mc_compatibility.inventory_drag_typed_event_migration.readiness]
  Evidence: docs/evidence/inventory-drag-typed-event-baseline-2026-06-22.run.log; docs/evidence/inventory-drag-typed-event-baseline-2026-06-22.b3
- [x] [depends:readiness] Mark `inventory-drag-transactions` typed-event-ready in the manifest and regenerate scenario surfaces without changing wrapper, dry-run, row, or non-claim scope. r[mc_compatibility.inventory_drag_typed_event_migration.readiness]
  Evidence: docs/evidence/inventory-drag-typed-event-migration-2026-06-22.run.log; docs/evidence/inventory-drag-typed-event-migration-2026-06-22.b3
- [x] [depends:readiness] Add `inventory-drag-transactions` to the typed-event pass/fail gate with positive and negative fixtures for missing events and misordered drag phases. r[mc_compatibility.inventory_drag_typed_event_migration.gate]
  Evidence: docs/evidence/inventory-drag-typed-event-migration-2026-06-22.run.log; docs/evidence/inventory-drag-typed-event-migration-2026-06-22.b3
- [x] [depends:gate] Run focused post-change validation for runner fixtures, scenario-manifest checks, generated surfaces, inventory-drag dry-run receipt shape, and evidence manifests. r[mc_compatibility.inventory_drag_typed_event_migration.validation]
  Evidence: docs/evidence/inventory-drag-typed-event-migration-2026-06-22.run.log; docs/evidence/inventory-drag-typed-event-migration-2026-06-22.b3
- [x] [depends:validation] Run Cairn proposal/design/tasks gates, task-evidence validation for completed implementation tasks, and Cairn validation with reviewable logs. r[mc_compatibility.inventory_drag_typed_event_migration.validation]
  Evidence: docs/evidence/inventory-drag-typed-event-cairn-validation-2026-06-22.run.log; docs/evidence/inventory-drag-typed-event-cairn-validation-2026-06-22.b3
- [x] [depends:validation] Sync/archive the completed change and run accepted-spec post-archive validation with reviewable logs. r[mc_compatibility.inventory_drag_typed_event_migration.validation]
  Evidence: docs/evidence/inventory-drag-typed-event-post-archive-2026-06-22.run.log; docs/evidence/inventory-drag-typed-event-post-archive-2026-06-22.b3
