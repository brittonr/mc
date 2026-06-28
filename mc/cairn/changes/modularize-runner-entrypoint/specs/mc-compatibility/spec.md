# mc-compatibility Change Spec: Modular runner entrypoint

## Requirements

### Requirement: Runner entrypoint boundary

r[mc_compatibility.runner_modularity.entrypoint_boundary] The mc-compat runner MUST keep its entrypoint thin by limiting `main.rs` to module wiring, process exit translation, and delegation into focused runner modules.

#### Scenario: Entrypoint delegates behavior

r[mc_compatibility.runner_modularity.entrypoint_boundary.delegates]
- GIVEN the runner starts from `main.rs`
- WHEN configuration, mode dispatch, backend lifecycle, scenario behavior, planning, receipt writing, or failure-bundle behavior is needed
- THEN `main.rs` delegates that behavior to an owning module
- AND `main.rs` does not own non-trivial policy logic for those responsibilities.

### Requirement: Runner module ownership

r[mc_compatibility.runner_modularity.entrypoint_modules] Extracted runner modules SHOULD have cohesive ownership boundaries and narrow public APIs for config, app dispatch, backend runtime, scenario behavior, planning, receipts, and failure bundles.

#### Scenario: Module owns one responsibility family

r[mc_compatibility.runner_modularity.entrypoint_modules.ownership]
- GIVEN a runner responsibility is extracted from `main.rs`
- WHEN the new module API is reviewed
- THEN the module exposes only the types and functions needed by neighboring modules
- AND side-effecting shell code remains separate from pure planning or evidence logic.

### Requirement: Entrypoint extraction preserves public behavior

r[mc_compatibility.runner_modularity.entrypoint_parity] Entrypoint modularization MUST preserve existing CLI flags, environment variables, receipt schemas, scenario names, scenario semantics, dry-run/live behavior, and evidence non-claims.

#### Scenario: Public surfaces remain stable

r[mc_compatibility.runner_modularity.entrypoint_parity.stable]
- GIVEN a supported pre-refactor runner invocation
- WHEN the modularized runner receives the same inputs
- THEN it produces the same mode selection, scenario selection, receipt shape, and non-claim boundaries
- AND it does not promote new compatibility evidence.

### Requirement: Entrypoint positive tests

r[mc_compatibility.runner_modularity.entrypoint_positive_tests] The change MUST include positive tests for representative dry-run, run, build-client, status, cleanup, matrix, receipt, and failure-bundle paths.

#### Scenario: Supported paths still pass

r[mc_compatibility.runner_modularity.entrypoint_positive_tests.coverage]
- GIVEN representative supported runner inputs
- WHEN the modularized entrypoint delegates into owner modules
- THEN tests prove the supported paths still produce the expected plans, receipts, and outcomes.

### Requirement: Entrypoint negative tests

r[mc_compatibility.runner_modularity.entrypoint_negative_tests] The change MUST include negative tests for unknown arguments, missing option values, unsafe cleanup/path plans, receipt/failure-bundle follow-up failures, and invalid mode combinations.

#### Scenario: Invalid paths fail closed

r[mc_compatibility.runner_modularity.entrypoint_negative_tests.fail_closed]
- GIVEN an invalid runner input or unsafe plan
- WHEN the modularized entrypoint delegates into owner modules
- THEN tests prove the runner returns the expected diagnostic before unintended side effects occur.

### Requirement: Entrypoint modularization validation

r[mc_compatibility.runner_modularity.entrypoint_validation] The change MUST record focused runner tests, dry-run smoke checks, scenario manifest checks, Cairn gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[mc_compatibility.runner_modularity.entrypoint_validation.logs]
- GIVEN entrypoint modularization is complete
- WHEN the change is closed
- THEN reviewable evidence logs show positive and negative tests plus Cairn proposal, design, tasks, and validation gates passing.
