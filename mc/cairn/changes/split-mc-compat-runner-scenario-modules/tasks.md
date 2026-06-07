# Tasks

- [ ] [serial] Define runner module boundaries, public scenario-core API, and no-public-surface-change scope. r[mc_compatibility.runner_scenario_modules.boundaries]
- [ ] [depends:boundaries] Extract scenario identity, static specs, behavior kinds, behavior lookup, and validation into pure scenario modules. r[mc_compatibility.runner_scenario_modules.scenario_core]
- [ ] [depends:scenario_core] Keep CLI parsing, backend/client orchestration, environment mutation, log collection, and receipt writing in imperative shell code. r[mc_compatibility.runner_scenario_modules.imperative_shell]
- [ ] [depends:imperative_shell] Preserve scenario names, aliases, dry-run output, manifest surfaces, receipt fields, and non-claim flags across the split. r[mc_compatibility.runner_scenario_modules.surface_parity]
- [ ] [depends:surface_parity] Add positive module parity tests and negative invalid-spec tests for duplicated canonical names, missing aliases, missing milestones, and unsupported behavior defaults. r[mc_compatibility.runner_scenario_modules.tests]
- [ ] [depends:tests] Run runner tests, scenario manifest checks, dry-run checks, Cairn gates, and Cairn validation with reviewable logs. r[mc_compatibility.runner_scenario_modules.validation]
