# Tasks

- [x] [serial] Record the pre-change baseline for runner tests, scenario manifest checks, and generated-surface freshness. r[mc_compatibility.survival_crafting_table_typed_event_migration.readiness]
  Evidence: docs/evidence/survival-crafting-table-typed-event-baseline-2026-06-22.run.log; docs/evidence/survival-crafting-table-typed-event-baseline-2026-06-22.b3
- [x] [depends:readiness] Mark `survival-crafting-table` typed-event-ready in the manifest and regenerate scenario surfaces without changing wrapper, dry-run, row, or non-claim scope. r[mc_compatibility.survival_crafting_table_typed_event_migration.readiness]
  Evidence: docs/evidence/survival-crafting-table-typed-event-migration-2026-06-22.run.log; docs/evidence/survival-crafting-table-typed-event-migration-2026-06-22.b3
- [x] [depends:readiness] Add `survival-crafting-table` to the typed-event pass/fail gate with positive and negative fixtures for missing events and misordered crafting phases. r[mc_compatibility.survival_crafting_table_typed_event_migration.gate]
  Evidence: docs/evidence/survival-crafting-table-typed-event-migration-2026-06-22.run.log; docs/evidence/survival-crafting-table-typed-event-migration-2026-06-22.b3
- [x] [depends:gate] Run focused post-change validation for runner fixtures, scenario-manifest checks, generated surfaces, survival crafting-table dry-run receipt shape, and evidence manifests. r[mc_compatibility.survival_crafting_table_typed_event_migration.validation]
  Evidence: docs/evidence/survival-crafting-table-typed-event-migration-2026-06-22.run.log; docs/evidence/survival-crafting-table-typed-event-migration-2026-06-22.b3
- [ ] [depends:validation] Run Cairn proposal/design/tasks gates, task-evidence validation for completed implementation tasks, and Cairn validation with reviewable logs. r[mc_compatibility.survival_crafting_table_typed_event_migration.validation]
  Evidence: TBD
- [ ] [depends:validation] Sync/archive the completed change and run accepted-spec post-archive validation with reviewable logs. r[mc_compatibility.survival_crafting_table_typed_event_migration.validation]
  Evidence: TBD
