# Tasks

- [x] [serial] Inventory plugin structs deriving `Component`, direct insert/query uses, and any real marker-state needs. r[hyperion_game_modes.plugin_component_derives.inventory]
  - Evidence: `docs/evidence/hyperion-gameplay-composition-inventory-2026-06-30.md`, `docs/evidence/hyperion-gameplay-composition-tests-2026-06-30.run.log`, and BLAKE3 manifest `docs/evidence/hyperion-gameplay-composition-2026-06-30.b3`.
- [x] [depends:inventory] Remove `Component` derives from structs that are only Bevy plugins and not ECS entity state. r[hyperion_game_modes.plugin_component_derives.cleanup]
  - Evidence: `docs/evidence/hyperion-gameplay-composition-tests-2026-06-30.run.log`, `docs/evidence/hyperion-gameplay-format-clippy-2026-06-30.run.log`, and BLAKE3 manifest `docs/evidence/hyperion-gameplay-composition-2026-06-30.b3`.
- [x] [depends:cleanup] Add or keep separate marker components where real ECS state is needed, and document any intentional exception. r[hyperion_game_modes.plugin_component_derives.markers]
  - Evidence: `docs/evidence/hyperion-gameplay-composition-inventory-2026-06-30.md`, `docs/evidence/hyperion-gameplay-composition-tests-2026-06-30.run.log`, and BLAKE3 manifest `docs/evidence/hyperion-gameplay-composition-2026-06-30.b3`.
- [x] [depends:markers] Add positive compile/plugin-install checks and negative checks for accidental plugin-as-component assumptions. r[hyperion_game_modes.plugin_component_derives.tests]
  - Evidence: `docs/evidence/hyperion-gameplay-composition-tests-2026-06-30.run.log`, `docs/evidence/hyperion-gameplay-format-clippy-2026-06-30.run.log`, and BLAKE3 manifest `docs/evidence/hyperion-gameplay-composition-2026-06-30.b3`.
- [x] [depends:tests] Run focused Hyperion compile/tests, marker checks where touched, Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest checks with promoted logs. r[hyperion_game_modes.plugin_component_derives.validation]
  - Evidence: `docs/evidence/hyperion-gameplay-composition-tests-2026-06-30.run.log`, `docs/evidence/hyperion-gameplay-format-clippy-2026-06-30.run.log`, `docs/evidence/hyperion-gameplay-cairn-gates-2026-06-30.run.log`, `docs/evidence/hyperion-gameplay-cairn-validate-2026-06-30.run.log`, and BLAKE3 manifest `docs/evidence/hyperion-gameplay-composition-2026-06-30.b3`.
