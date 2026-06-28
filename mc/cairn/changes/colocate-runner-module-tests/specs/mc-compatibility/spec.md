# mc-compatibility Change Spec: Runner module test colocation

## Requirements

### Requirement: Module test colocation

r[mc_compatibility.runner_modularity.module_test_colocation] Runner unit tests SHOULD live beside the module that owns the behavior under test instead of accumulating in the root entrypoint.

#### Scenario: Unit test documents owner

r[mc_compatibility.runner_modularity.module_test_colocation.owner]
- GIVEN a unit test validates config, planning, wire, layout, receipt, evidence, scenario, or client-driver behavior
- WHEN tests are organized after the move
- THEN the test is located with the module that owns that behavior
- AND the root entrypoint no longer owns unrelated unit test families.

### Requirement: Shared test support

r[mc_compatibility.runner_modularity.test_support] Shared runner test fixtures SHOULD live in deterministic test-support helpers with explicit inputs and no hidden global environment mutation.

#### Scenario: Shared fixture is deterministic

r[mc_compatibility.runner_modularity.test_support.deterministic]
- GIVEN multiple module tests need the same fixture
- WHEN the fixture helper is invoked
- THEN it derives its output from explicit inputs
- AND it does not rely on hidden process environment state.

### Requirement: Integration test boundary

r[mc_compatibility.runner_modularity.integration_test_boundary] Cross-module runner tests MAY remain at crate-root or integration-test scope when they validate behavior that intentionally spans multiple owner modules.

#### Scenario: Integration test names boundary

r[mc_compatibility.runner_modularity.integration_test_boundary.cross_module]
- GIVEN a test validates behavior across config, planning, execution, receipts, and evidence
- WHEN the test remains outside a single owner module
- THEN the test setup names the cross-module boundary it covers
- AND unit-level assertions remain in owner modules where practical.

### Requirement: Module-test positive coverage

r[mc_compatibility.runner_modularity.module_test_positive_coverage] The move MUST preserve or add positive tests for every moved module family.

#### Scenario: Positive coverage remains visible

r[mc_compatibility.runner_modularity.module_test_positive_coverage.visible]
- GIVEN a module test family moves from the root entrypoint
- WHEN tests run after the move
- THEN the module still has happy-path coverage for its primary contract.

### Requirement: Module-test negative coverage

r[mc_compatibility.runner_modularity.module_test_negative_coverage] The move MUST preserve or add negative tests for every moved module family, including invalid config, malformed receipts, missing evidence, bad wire data, unsafe paths, and scenario validation failures.

#### Scenario: Negative coverage remains visible

r[mc_compatibility.runner_modularity.module_test_negative_coverage.visible]
- GIVEN a module test family moves from the root entrypoint
- WHEN tests run after the move
- THEN the module still has fail-closed coverage for malformed or unsafe inputs.

### Requirement: Module-test validation

r[mc_compatibility.runner_modularity.module_test_validation] The change MUST record runner tests, integration smoke tests, Cairn gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[mc_compatibility.runner_modularity.module_test_validation.logs]
- GIVEN module-test colocation is complete
- WHEN the change is closed
- THEN reviewable logs show moved positive and negative coverage plus Cairn validation passing.
