# Tasks

- [x] [serial] Inventory current preset fields, planner outputs, builder install order, replacement behavior, and name-only custom plugin gaps. r[hyperion_game_modes.preset_plugin_slots.inventory]
  - Evidence: `docs/evidence/oracles/2026-06-30/hyperion-valence-integration-inventory.md`, `docs/evidence/run-logs/2026-06-30/hyperion-integration.focused-checks.run.log`, and `docs/evidence/manifests/2026-06-30/hyperion-valence-integration.b3`.
- [x] [depends:inventory] Define the typed plugin-slot contract and pure semantic plan outputs for feature replacement and custom plugin additions. r[hyperion_game_modes.preset_plugin_slots.contract]
  - Evidence: `docs/evidence/run-logs/2026-06-30/hyperion-integration.focused-checks.run.log` and `docs/evidence/manifests/2026-06-30/hyperion-valence-integration.b3`.
- [x] [depends:contract] Wire the app-builder shell to install validated replacement and custom plugin slots without mutating `App` before validation succeeds. r[hyperion_game_modes.preset_plugin_slots.app_shell]
  - Evidence: `docs/evidence/run-logs/2026-06-30/hyperion-integration.focused-checks.run.log` and `docs/evidence/manifests/2026-06-30/hyperion-valence-integration.b3`.
- [x] [depends:app_shell] Preserve existing default entrypoints and document compiled-plugin, no-hot-load, and no-sandbox non-claims. r[hyperion_game_modes.preset_plugin_slots.compatibility]
  - Evidence: `docs/evidence/oracles/2026-06-30/hyperion-valence-integration-inventory.md`, `docs/evidence/run-logs/2026-06-30/hyperion-integration.focused-checks.run.log`, and `docs/evidence/manifests/2026-06-30/hyperion-valence-integration.b3`.
- [x] [depends:compatibility] Add positive executable replacement/custom-slot tests and negative missing-slot, duplicate-slot, dependency, and partial-app-prevention tests. r[hyperion_game_modes.preset_plugin_slots.tests]
  - Evidence: `docs/evidence/run-logs/2026-06-30/hyperion-integration.focused-checks.run.log` and `docs/evidence/manifests/2026-06-30/hyperion-valence-integration.b3`.
- [x] [depends:tests] Run focused Hyperion preset/app-builder checks plus Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, and evidence-manifest checks with promoted logs. r[hyperion_game_modes.preset_plugin_slots.validation]
  - Evidence: `docs/evidence/run-logs/2026-06-30/hyperion-integration.focused-checks.run.log`, `docs/evidence/run-logs/2026-06-30/hyperion-valence-integration.cairn-gates.run.log`, and `docs/evidence/manifests/2026-06-30/hyperion-valence-integration.b3`.
