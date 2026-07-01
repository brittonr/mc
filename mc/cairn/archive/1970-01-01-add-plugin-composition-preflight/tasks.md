# Tasks

- [x] [serial] Inventory current Hyperion composition failure paths, assertion points, duplicate-plugin panics, builder mutation order, and existing pure planner diagnostics. r[hyperion_game_modes.composition_preflight.inventory]
  - Evidence: `docs/evidence/oracles/2026-06-30/hyperion-valence-integration-inventory.md`, `docs/evidence/run-logs/2026-06-30/hyperion-integration.focused-checks.run.log`, and `docs/evidence/manifests/2026-06-30/hyperion-valence-integration.b3`.
- [x] [depends:inventory] Define a pure preflight core that validates builder-controlled plugin composition and returns typed diagnostics without Bevy mutation. r[hyperion_game_modes.composition_preflight.core]
  - Evidence: `docs/evidence/run-logs/2026-06-30/hyperion-integration.focused-checks.run.log` and `docs/evidence/manifests/2026-06-30/hyperion-valence-integration.b3`.
- [x] [depends:core] Wire app builders to call preflight before inserting proxy/crypto resources or adding Hyperion core, default gameplay, custom plugins, or mode plugins. r[hyperion_game_modes.composition_preflight.builder]
  - Evidence: `docs/evidence/run-logs/2026-06-30/hyperion-integration.focused-checks.run.log` and `docs/evidence/manifests/2026-06-30/hyperion-valence-integration.b3`.
- [x] [depends:builder] Keep direct plugin diagnostics deterministic and document the fallible-builder versus infallible-Bevy-plugin boundary. r[hyperion_game_modes.composition_preflight.direct]
  - Evidence: `docs/evidence/oracles/2026-06-30/hyperion-valence-integration-inventory.md`, `docs/evidence/run-logs/2026-06-30/hyperion-integration.focused-checks.run.log`, and `docs/evidence/manifests/2026-06-30/hyperion-valence-integration.b3`.
- [x] [depends:direct] Add positive default/custom composition tests and negative duplicate mode, missing dependency, unsupported replacement, duplicate slot/plugin, direct misuse, and partial-app-prevention tests. r[hyperion_game_modes.composition_preflight.tests]
  - Evidence: `docs/evidence/run-logs/2026-06-30/hyperion-integration.focused-checks.run.log` and `docs/evidence/manifests/2026-06-30/hyperion-valence-integration.b3`.
- [x] [depends:tests] Run focused Hyperion preflight and app-builder checks plus Cairn proposal/design/tasks gates, Cairn validation, and evidence-manifest checks with promoted logs. r[hyperion_game_modes.composition_preflight.validation]
  - Evidence: `docs/evidence/run-logs/2026-06-30/hyperion-integration.focused-checks.run.log`, `docs/evidence/run-logs/2026-06-30/hyperion-valence-integration.cairn-gates.run.log`, and `docs/evidence/manifests/2026-06-30/hyperion-valence-integration.b3`.
