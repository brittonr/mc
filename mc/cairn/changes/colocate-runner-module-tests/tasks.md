# Tasks

- [ ] [serial] Inventory root-level tests by owner module and record baseline runner test results before moving fixtures. r[mc_compatibility.runner_modularity.module_test_colocation]
- [ ] [depends:module_test_colocation] Move config, planning, wire, layout, receipt, evidence, scenario, and client-driver unit tests beside their owner modules. r[mc_compatibility.runner_modularity.module_test_colocation]
- [ ] [depends:module_test_colocation] Create deterministic shared test-support helpers only for fixtures used by multiple owner modules. r[mc_compatibility.runner_modularity.test_support]
- [ ] [depends:test_support] Keep true cross-module runner behavior in explicit integration-style tests with clear fixture setup. r[mc_compatibility.runner_modularity.integration_test_boundary]
- [ ] [depends:integration_test_boundary] Add or preserve positive tests for each moved module family. r[mc_compatibility.runner_modularity.module_test_positive_coverage]
- [ ] [depends:module_test_positive_coverage] Add or preserve negative tests for each moved module family, including invalid config, malformed receipts, missing evidence, bad wire data, unsafe paths, and scenario validation failures. r[mc_compatibility.runner_modularity.module_test_negative_coverage]
- [ ] [depends:module_test_negative_coverage] Run runner tests, integration smoke tests, Cairn gates, and Cairn validation with reviewable logs. r[mc_compatibility.runner_modularity.module_test_validation]
