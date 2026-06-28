# Tasks

- [ ] [serial] Inventory production `use super::*` imports and identify the owning module for each dependency. r[mc_compatibility.runner_modularity.explicit_imports]
- [ ] [depends:explicit_imports] Move shared root-owned types into explicit owner modules so production modules can import stable symbols. r[mc_compatibility.runner_modularity.shared_type_ownership]
- [ ] [depends:shared_type_ownership] Replace production wildcard root imports with explicit imports and keep module APIs narrow. r[mc_compatibility.runner_modularity.explicit_imports]
- [ ] [depends:explicit_imports] Add a positive boundary fixture that allows explicit production imports and scoped test-module imports. r[mc_compatibility.runner_modularity.import_boundary_positive_tests]
- [ ] [depends:import_boundary_positive_tests] Add a negative boundary fixture that rejects production `use super::*` regressions. r[mc_compatibility.runner_modularity.import_boundary_negative_tests]
- [ ] [depends:import_boundary_negative_tests] Run runner tests, import-boundary checks, Cairn gates, and Cairn validation with reviewable logs. r[mc_compatibility.runner_modularity.import_boundary_validation]
