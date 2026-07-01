# Tasks

- [x] [serial] Inventory current Valence schedule graph checks, schedule evidence requirements, selected plugin configurations, DOT usage, and review-critical schedule facts. r[valence_bevy_ecs.structured_schedule_receipts.inventory]
  - Evidence: `docs/evidence/oracles/2026-06-30/hyperion-valence-integration-inventory.md`, `docs/evidence/run-logs/2026-06-30/valence-integration.focused-checks.run.log`, and `docs/evidence/manifests/2026-06-30/hyperion-valence-integration.b3`.
- [x] [depends:inventory] Define the deterministic structured schedule receipt schema and pure fact-checking/normalization boundary. r[valence_bevy_ecs.structured_schedule_receipts.schema]
  - Evidence: `docs/evidence/run-logs/2026-06-30/valence-integration.focused-checks.run.log` and `docs/evidence/manifests/2026-06-30/hyperion-valence-integration.b3`.
- [x] [depends:schema] Add shell helpers to collect selected Bevy schedule facts and render receipts without requiring full graph dumps. r[valence_bevy_ecs.structured_schedule_receipts.collection]
  - Evidence: `docs/evidence/run-logs/2026-06-30/valence-integration.focused-checks.run.log` and `docs/evidence/manifests/2026-06-30/hyperion-valence-integration.b3`.
- [x] [depends:collection] Wire selected core plugin schedule tests or tools to validate structured receipts for default and disabled-plugin configurations. r[valence_bevy_ecs.structured_schedule_receipts.wiring]
  - Evidence: `docs/evidence/run-logs/2026-06-30/valence-integration.focused-checks.run.log` and `docs/evidence/manifests/2026-06-30/hyperion-valence-integration.b3`.
- [x] [depends:wiring] Add positive receipt tests and negative unknown schedule, missing set, unexpected system/plugin, ambiguity, and determinism tests. r[valence_bevy_ecs.structured_schedule_receipts.tests]
  - Evidence: `docs/evidence/run-logs/2026-06-30/valence-integration.focused-checks.run.log` and `docs/evidence/manifests/2026-06-30/hyperion-valence-integration.b3`.
- [x] [depends:tests] Run focused Valence schedule receipt checks plus Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, and evidence-manifest checks with promoted logs. r[valence_bevy_ecs.structured_schedule_receipts.validation]
  - Evidence: `docs/evidence/run-logs/2026-06-30/valence-integration.focused-checks.run.log`, `docs/evidence/run-logs/2026-06-30/hyperion-valence-integration.cairn-gates.run.log`, and `docs/evidence/manifests/2026-06-30/hyperion-valence-integration.b3`.
