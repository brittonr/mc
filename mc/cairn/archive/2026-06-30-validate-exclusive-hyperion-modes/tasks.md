# Tasks

- [x] [serial] Inventory current single-mode selection paths, direct plugin-add risks, and additive feature plugin surfaces. r[hyperion_game_modes.exclusive_modes.inventory]
  - Evidence: `docs/evidence/hyperion-gameplay-composition-inventory-2026-06-30.md`, `docs/evidence/hyperion-gameplay-composition-tests-2026-06-30.run.log`, and BLAKE3 manifest `docs/evidence/hyperion-gameplay-composition-2026-06-30.b3`.
- [x] [depends:inventory] Define a pure exclusivity validator that classifies exclusive world modes separately from additive gameplay features. r[hyperion_game_modes.exclusive_modes.validator]
  - Evidence: `docs/evidence/hyperion-gameplay-composition-tests-2026-06-30.run.log`, `docs/evidence/hyperion-gameplay-format-clippy-2026-06-30.run.log`, and BLAKE3 manifest `docs/evidence/hyperion-gameplay-composition-2026-06-30.b3`.
- [x] [depends:validator] Wire preset builders and mode plugin setup to reject or diagnose multiple exclusive modes before ambiguous app behavior occurs. r[hyperion_game_modes.exclusive_modes.integration]
  - Evidence: `docs/evidence/hyperion-gameplay-composition-tests-2026-06-30.run.log`, `docs/evidence/hyperion-gameplay-format-clippy-2026-06-30.run.log`, and BLAKE3 manifest `docs/evidence/hyperion-gameplay-composition-2026-06-30.b3`.
- [x] [depends:integration] Document the single-exclusive-mode contract and multi-mode/multi-world non-claims. r[hyperion_game_modes.exclusive_modes.documentation]
  - Evidence: `docs/evidence/hyperion-gameplay-composition-inventory-2026-06-30.md`, `docs/evidence/hyperion-gameplay-composition-tests-2026-06-30.run.log`, and BLAKE3 manifest `docs/evidence/hyperion-gameplay-composition-2026-06-30.b3`.
- [x] [depends:documentation] Add positive one-mode-plus-features tests and negative duplicate exclusive mode tests for builder and direct-plugin paths where practical. r[hyperion_game_modes.exclusive_modes.tests]
  - Evidence: `docs/evidence/hyperion-gameplay-composition-tests-2026-06-30.run.log`, `docs/evidence/hyperion-gameplay-format-clippy-2026-06-30.run.log`, and BLAKE3 manifest `docs/evidence/hyperion-gameplay-composition-2026-06-30.b3`.
- [x] [depends:tests] Run pure exclusivity tests, Bevy app composition tests, duplicate-mode diagnostics, Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest checks with promoted logs. r[hyperion_game_modes.exclusive_modes.validation]
  - Evidence: `docs/evidence/hyperion-gameplay-composition-tests-2026-06-30.run.log`, `docs/evidence/hyperion-gameplay-format-clippy-2026-06-30.run.log`, `docs/evidence/hyperion-gameplay-cairn-gates-2026-06-30.run.log`, `docs/evidence/hyperion-gameplay-cairn-validate-2026-06-30.run.log`, and BLAKE3 manifest `docs/evidence/hyperion-gameplay-composition-2026-06-30.b3`.
