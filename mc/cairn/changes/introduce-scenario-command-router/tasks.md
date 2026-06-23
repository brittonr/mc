# Tasks

- [ ] [serial] Define the typed scenario router CLI contract, accepted arguments, defaults, and non-claim boundaries. r[mc_compatibility.scenario_command_router.contract]
- [ ] [depends:contract] Implement pure route planning for scenario/backend/receipt/timeout/live-dry-run inputs before side effects. r[mc_compatibility.scenario_command_router.plan]
- [ ] [depends:plan] Route selected existing flake aliases through the typed command while preserving app names and dry-run command shapes. r[mc_compatibility.scenario_command_router.alias_parity]
- [ ] [depends:alias_parity] Add positive and negative tests for known scenarios, invalid scenarios, invalid backends, unsafe receipt paths, and blocked live options. r[mc_compatibility.scenario_command_router.tests]
- [ ] [depends:tests] Update generated command docs/indexes if wrapper command shapes change. r[mc_compatibility.scenario_command_router.docs]
- [ ] [depends:docs] Run CLI/router tests, alias parity dry-runs, maintained dry-run aggregate, Cairn gates, and Cairn validation with reviewable logs. r[mc_compatibility.scenario_command_router.validation]
