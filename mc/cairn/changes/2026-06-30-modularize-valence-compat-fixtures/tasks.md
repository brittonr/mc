# Tasks

- [ ] [serial] Read Valence local workflow docs, inventory CTF/survival fixture responsibilities, plugin/schedule contracts, scenario env contracts, and baseline checks. r[valence_bevy_ecs.compat_fixture_modularization.inventory]
- [ ] [serial] Extract CTF and survival fixture logic into focused modules while keeping example entrypoints as thin app/plugin shells. r[valence_bevy_ecs.compat_fixture_modularization.module_boundaries]
- [ ] [serial] Separate pure fixture decisions from Bevy shells for score/flag/inventory/survival/config/probe logic. r[valence_bevy_ecs.compat_fixture_modularization.core_shell]
- [ ] [serial] Preserve explicit opt-in plugin behavior, schedule contracts, scenario env vars, typed milestone vocabulary, fixture behavior, runner receipt shapes, and non-claim boundaries. r[valence_bevy_ecs.compat_fixture_modularization.parity]
- [ ] [serial] Add positive tests for extracted fixture contracts and negative tests for invalid env/config, missing schedule facts, disabled plugins, stale fixture state, invalid probe transitions, and overclaim markers. r[valence_bevy_ecs.compat_fixture_modularization.tests]
- [ ] [serial] Run affected Valence tests, schedule hygiene, affected mc-compat dry-runs or live rails required by tasks, Cairn proposal/design/tasks gates, Cairn validation, and task-evidence checks with reviewable logs before archive. r[valence_bevy_ecs.compat_fixture_modularization.validation]
