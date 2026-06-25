# Tasks

- [x] [serial] Inventory selected Valence example systems, schedules, resources, events, env toggles, milestone emitters, and non-goals before moving wiring. r[valence_bevy_ecs.gameplay_plugins.inventory]
  - Evidence: `docs/evidence/organize-valence-gameplay-examples-as-bevy-plugins-inventory.md`, `docs/evidence/organize-valence-gameplay-examples-inventory-check.run.log`, `docs/evidence/organize-valence-gameplay-examples-validation.b3`.
- [x] [depends:inventory] Define plugin boundaries and named `SystemSet` contracts for input, rule evaluation, world mutation, presentation, and cleanup phases. r[valence_bevy_ecs.gameplay_plugins.contract]
  - Evidence: `docs/evidence/organize-valence-gameplay-examples-as-bevy-plugins-inventory.md`, `docs/evidence/organize-valence-gameplay-examples-post-plugin-checks.run.log`, `docs/evidence/organize-valence-gameplay-examples-validation.b3`.
- [x] [depends:contract] Move selected `ctf`, `survival_compat`, and `terrain` Bevy app wiring into opt-in example plugins without moving pure fixture decisions into ECS systems. r[valence_bevy_ecs.gameplay_plugins.wiring]
  - Evidence: `docs/evidence/organize-valence-gameplay-examples-post-plugin-checks.run.log`, `docs/evidence/organize-valence-gameplay-examples-validation.b3`.
- [x] [depends:wiring] Preserve example commands, env var contracts, milestone text, selected scenario behavior, and non-claim boundaries. r[valence_bevy_ecs.gameplay_plugins.compatibility]
  - Evidence: `docs/evidence/organize-valence-gameplay-examples-as-bevy-plugins-inventory.md`, `docs/evidence/organize-valence-gameplay-examples-selected-dry-runs.run.log`, `docs/evidence/organize-valence-gameplay-examples-validation.b3`.
- [x] [depends:wiring] Add positive plugin/schedule smoke tests and negative disabled-plugin or ordering regression tests. r[valence_bevy_ecs.gameplay_plugins.tests]
  - Evidence: `docs/evidence/organize-valence-gameplay-examples-post-plugin-checks.run.log`, `docs/evidence/organize-valence-gameplay-examples-validation.b3`.
- [x] [depends:tests] Run focused example checks, selected mc-compat rails if fixture behavior is touched, Cairn proposal/design/tasks gates, Cairn validation, and task-evidence checks with promoted logs. r[valence_bevy_ecs.gameplay_plugins.validation]
  - Evidence: `docs/evidence/organize-valence-gameplay-examples-post-plugin-checks.run.log`, `docs/evidence/organize-valence-gameplay-examples-selected-dry-runs.run.log`, `docs/evidence/organize-valence-gameplay-examples-final-cairn-gates.run.log`, `docs/evidence/organize-valence-gameplay-examples-task-evidence-precloseout.run.log`, `docs/evidence/organize-valence-gameplay-examples-validation.b3`.
