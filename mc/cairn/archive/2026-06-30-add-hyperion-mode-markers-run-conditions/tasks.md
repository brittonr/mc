# Tasks

- [x] [serial] Inventory current mode-specific components, observers, systems, active-mode checks, and cleanup responsibilities. r[hyperion_game_modes.mode_markers.inventory]
  - Evidence: `docs/evidence/hyperion-gameplay-composition-inventory-2026-06-30.md`, `docs/evidence/hyperion-gameplay-composition-tests-2026-06-30.run.log`, and BLAKE3 manifest `docs/evidence/hyperion-gameplay-composition-2026-06-30.b3`.
- [x] [depends:inventory] Define marker components/resources and run-condition helpers for Bedwars, Dayz, HardcoreFactions, and shared extension points. r[hyperion_game_modes.mode_markers.contract]
  - Evidence: `docs/evidence/hyperion-gameplay-composition-inventory-2026-06-30.md`, `docs/evidence/hyperion-gameplay-composition-tests-2026-06-30.run.log`, and BLAKE3 manifest `docs/evidence/hyperion-gameplay-composition-2026-06-30.b3`.
- [x] [depends:contract] Refactor mode-specific systems and observers to use marker filters or run conditions while preserving common feature behavior. r[hyperion_game_modes.mode_markers.integration]
  - Evidence: `docs/evidence/hyperion-gameplay-composition-tests-2026-06-30.run.log`, `docs/evidence/hyperion-gameplay-format-clippy-2026-06-30.run.log`, and BLAKE3 manifest `docs/evidence/hyperion-gameplay-composition-2026-06-30.b3`.
- [x] [depends:integration] Add marker-owned cleanup or teardown behavior for scoped temporary state where applicable. r[hyperion_game_modes.mode_markers.cleanup]
  - Evidence: `docs/evidence/hyperion-gameplay-composition-inventory-2026-06-30.md`, `docs/evidence/hyperion-gameplay-composition-tests-2026-06-30.run.log`, and BLAKE3 manifest `docs/evidence/hyperion-gameplay-composition-2026-06-30.b3`.
- [x] [depends:cleanup] Add positive scoped-mutation tests and negative wrong-mode, disabled-plugin, stale-marker, and cleanup-leak tests. r[hyperion_game_modes.mode_markers.tests]
  - Evidence: `docs/evidence/hyperion-gameplay-composition-tests-2026-06-30.run.log`, `docs/evidence/hyperion-gameplay-format-clippy-2026-06-30.run.log`, and BLAKE3 manifest `docs/evidence/hyperion-gameplay-composition-2026-06-30.b3`.
- [x] [depends:tests] Run focused Hyperion ECS/plugin checks, marker cleanup tests, wrong-mode tests, Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest checks with promoted logs. r[hyperion_game_modes.mode_markers.validation]
  - Evidence: `docs/evidence/hyperion-gameplay-composition-tests-2026-06-30.run.log`, `docs/evidence/hyperion-gameplay-format-clippy-2026-06-30.run.log`, `docs/evidence/hyperion-gameplay-cairn-gates-2026-06-30.run.log`, `docs/evidence/hyperion-gameplay-cairn-validate-2026-06-30.run.log`, and BLAKE3 manifest `docs/evidence/hyperion-gameplay-composition-2026-06-30.b3`.
