# Tasks

- [x] [serial] Inventory current Valence gameplay scope IDs, plugin contract keys, global resources, event payloads, milestone emitters, cleanup paths, and same-app coexistence risks. r[valence_bevy_ecs.scoped_plugin_instances.inventory]
  - Evidence: `docs/evidence/oracles/2026-06-30/hyperion-valence-integration-inventory.md`, `docs/evidence/run-logs/2026-06-30/valence-integration.focused-checks.run.log`, and `docs/evidence/manifests/2026-06-30/hyperion-valence-integration.b3`.
- [x] [depends:inventory] Define the plugin-instance identity model and ownership rules for arena-, layer-, client-, resource-, and fixture-scoped state. r[valence_bevy_ecs.scoped_plugin_instances.model]
  - Evidence: `docs/evidence/run-logs/2026-06-30/valence-integration.focused-checks.run.log` and `docs/evidence/manifests/2026-06-30/hyperion-valence-integration.b3`.
- [x] [depends:model] Wire selected gameplay systems and contracts to use explicit instance/scope identity for state, events, diagnostics, and milestones where coexistence matters. r[valence_bevy_ecs.scoped_plugin_instances.wiring]
  - Evidence: `docs/evidence/run-logs/2026-06-30/valence-integration.focused-checks.run.log` and `docs/evidence/manifests/2026-06-30/hyperion-valence-integration.b3`.
- [x] [depends:wiring] Preserve current single-primary fixture behavior with compatibility adapters and documented non-claims. r[valence_bevy_ecs.scoped_plugin_instances.compatibility]
  - Evidence: `docs/evidence/oracles/2026-06-30/hyperion-valence-integration-inventory.md`, `docs/evidence/run-logs/2026-06-30/valence-integration.focused-checks.run.log`, and `docs/evidence/manifests/2026-06-30/hyperion-valence-integration.b3`.
- [x] [depends:compatibility] Add positive multi-mode or multi-instance tests and negative wrong-scope, stale-scope, missing-scope, disabled-plugin, and cross-layer mutation tests. r[valence_bevy_ecs.scoped_plugin_instances.tests]
  - Evidence: `docs/evidence/run-logs/2026-06-30/valence-integration.focused-checks.run.log` and `docs/evidence/manifests/2026-06-30/hyperion-valence-integration.b3`.
- [x] [depends:tests] Run focused Valence scope tests, selected compatibility dry-runs if receipts change, Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, and evidence-manifest checks with promoted logs. r[valence_bevy_ecs.scoped_plugin_instances.validation]
  - Evidence: `docs/evidence/run-logs/2026-06-30/valence-integration.focused-checks.run.log`, `docs/evidence/run-logs/2026-06-30/hyperion-valence-integration.cairn-gates.run.log`, and `docs/evidence/manifests/2026-06-30/hyperion-valence-integration.b3`.
