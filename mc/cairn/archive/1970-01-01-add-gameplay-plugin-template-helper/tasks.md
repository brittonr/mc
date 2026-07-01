# Tasks

- [x] [serial] Inventory duplicated Valence gameplay/example plugin boilerplate, contract resources, phase setup, schedule registration, install tests, and disabled-plugin gaps. r[valence_bevy_ecs.plugin_template_helper.inventory]
  - Evidence: `docs/evidence/oracles/2026-06-30/hyperion-valence-integration-inventory.md`, `docs/evidence/run-logs/2026-06-30/valence-integration.focused-checks.run.log`, and `docs/evidence/manifests/2026-06-30/hyperion-valence-integration.b3`.
- [x] [depends:inventory] Define the reusable plugin template/helper API for contract descriptors, phase wiring, registration, and test assertions. r[valence_bevy_ecs.plugin_template_helper.api]
  - Evidence: `docs/evidence/run-logs/2026-06-30/valence-integration.focused-checks.run.log` and `docs/evidence/manifests/2026-06-30/hyperion-valence-integration.b3`.
- [x] [depends:api] Migrate selected representative examples to the helper while keeping gameplay logic, compatibility milestones, and visible behavior unchanged. r[valence_bevy_ecs.plugin_template_helper.migration]
  - Evidence: `docs/evidence/run-logs/2026-06-30/valence-integration.focused-checks.run.log` and `docs/evidence/manifests/2026-06-30/hyperion-valence-integration.b3`.
- [x] [depends:migration] Document the new-plugin workflow, required positive and negative tests, and non-claims for dynamic/runtime plugin loading. r[valence_bevy_ecs.plugin_template_helper.documentation]
  - Evidence: `docs/evidence/oracles/2026-06-30/hyperion-valence-integration-inventory.md`, `docs/evidence/run-logs/2026-06-30/valence-integration.focused-checks.run.log`, and `docs/evidence/manifests/2026-06-30/hyperion-valence-integration.b3`.
- [x] [depends:documentation] Add positive helper/plugin install tests and negative missing-contract, disabled-plugin, stale-metadata, duplicate-registration, and ordering-regression tests. r[valence_bevy_ecs.plugin_template_helper.tests]
  - Evidence: `docs/evidence/run-logs/2026-06-30/valence-integration.focused-checks.run.log` and `docs/evidence/manifests/2026-06-30/hyperion-valence-integration.b3`.
- [x] [depends:tests] Run selected Valence example/helper checks plus Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, and evidence-manifest checks with promoted logs. r[valence_bevy_ecs.plugin_template_helper.validation]
  - Evidence: `docs/evidence/run-logs/2026-06-30/valence-integration.focused-checks.run.log`, `docs/evidence/run-logs/2026-06-30/hyperion-valence-integration.cairn-gates.run.log`, and `docs/evidence/manifests/2026-06-30/hyperion-valence-integration.b3`.
