# Tasks

- [ ] [serial] Inventory active transition paths and all current code/doc/check references to them. r[repository_layout.transition_path_retirement.inventory]
- [ ] [depends:inventory] Define canonical role paths and update active docs/commands to use those paths. r[repository_layout.transition_path_retirement.canonical_paths]
- [ ] [depends:canonical_paths] Remove or deprecate transition-path resolver support with actionable diagnostics for old paths and duplicate roots. r[repository_layout.transition_path_retirement.resolver]
- [ ] [depends:resolver] Add positive tests for canonical roots and negative tests for legacy-only roots, duplicate roots, nested Git roots, and missing roots. r[repository_layout.transition_path_retirement.tests]
- [ ] [depends:tests] Update historical/evidence references only where needed to avoid active-doc confusion while preserving archive meaning. r[repository_layout.transition_path_retirement.docs]
- [ ] [depends:docs] Run layout tests, missing-checkout diagnostics, runner dry-runs, Cairn gates, and Cairn validation with reviewable logs. r[repository_layout.transition_path_retirement.validation]
