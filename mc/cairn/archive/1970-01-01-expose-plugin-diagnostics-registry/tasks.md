# Tasks

- [x] [serial] Inventory current Hyperion plugin inspectability gaps, planner diagnostics, app-builder composition facts, plugin flags/resources, dependency decisions, and evidence needs. r[hyperion_game_modes.plugin_diagnostics.inventory]
  - Evidence: `docs/evidence/oracles/2026-06-30/hyperion-valence-integration-inventory.md`, `docs/evidence/run-logs/2026-06-30/hyperion-integration.focused-checks.run.log`, and `docs/evidence/manifests/2026-06-30/hyperion-valence-integration.b3`.
- [x] [depends:inventory] Define the plugin diagnostics registry schema and provenance model for default builders, custom presets, replacements, custom slots, and direct plugin additions. r[hyperion_game_modes.plugin_diagnostics.registry]
  - Evidence: `docs/evidence/run-logs/2026-06-30/hyperion-integration.focused-checks.run.log` and `docs/evidence/manifests/2026-06-30/hyperion-valence-integration.b3`.
- [x] [depends:registry] Wire validated app builders and selected plugin groups to populate diagnostics without changing runtime plugin behavior. r[hyperion_game_modes.plugin_diagnostics.wiring]
  - Evidence: `docs/evidence/run-logs/2026-06-30/hyperion-integration.focused-checks.run.log` and `docs/evidence/manifests/2026-06-30/hyperion-valence-integration.b3`.
- [x] [depends:wiring] Add deterministic test/receipt inspection helpers for selected mode, feature membership, disables, replacements, custom slots, dependencies, contracts, and non-claims. r[hyperion_game_modes.plugin_diagnostics.exposure]
  - Evidence: `docs/evidence/run-logs/2026-06-30/hyperion-integration.focused-checks.run.log` and `docs/evidence/manifests/2026-06-30/hyperion-valence-integration.b3`.
- [x] [depends:exposure] Add positive diagnostics tests and negative stale diagnostics, missing diagnostics, disabled-feature mismatch, replacement mismatch, and direct-provenance tests. r[hyperion_game_modes.plugin_diagnostics.tests]
  - Evidence: `docs/evidence/run-logs/2026-06-30/hyperion-integration.focused-checks.run.log` and `docs/evidence/manifests/2026-06-30/hyperion-valence-integration.b3`.
- [x] [depends:tests] Run focused Hyperion diagnostics checks plus Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, and evidence-manifest checks with promoted logs. r[hyperion_game_modes.plugin_diagnostics.validation]
  - Evidence: `docs/evidence/run-logs/2026-06-30/hyperion-integration.focused-checks.run.log`, `docs/evidence/run-logs/2026-06-30/hyperion-valence-integration.cairn-gates.run.log`, and `docs/evidence/manifests/2026-06-30/hyperion-valence-integration.b3`.
