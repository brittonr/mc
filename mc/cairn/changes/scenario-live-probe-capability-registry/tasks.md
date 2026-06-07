# Tasks

- [ ] [serial] Define the scenario live capability data model with scenario id, packet rows, capability kind, backend/client path, evidence mode, required signals, required non-claims, and optional blocker reason. r[mc_compatibility.scenario_live_probe_capabilities.contract]
- [ ] [depends:contract] Implement pure registry validation for duplicate capabilities, unknown scenarios, unknown packet rows, unsupported evidence modes, empty required signals, and missing non-claims. r[mc_compatibility.scenario_live_probe_capabilities.core]
- [ ] [depends:core] Seed registry entries for currently known targeted packet candidates and explicit blocker entries where no deterministic live path exists. r[mc_compatibility.scenario_live_probe_capabilities.seed]
- [ ] [depends:seed] Extend scenario manifest or focused checker coverage so registry drift fails closed. r[mc_compatibility.scenario_live_probe_capabilities.checker]
- [ ] [depends:checker] Add positive and negative tests for valid capabilities, duplicate row/scenario pairs, missing non-claims, unsupported evidence modes, and unknown packet rows. r[mc_compatibility.scenario_live_probe_capabilities.tests]
- [ ] [depends:tests] Document how future live packet rails use the capability registry for selection and blocker reporting. r[mc_compatibility.scenario_live_probe_capabilities.docs]
- [ ] [depends:docs] Run scenario tests/manifest checks, runner dry-runs as applicable, evidence-manifest/task-evidence checks, Cairn gates, sync, archive, and post-archive validation. r[mc_compatibility.scenario_live_probe_capabilities.validation]
