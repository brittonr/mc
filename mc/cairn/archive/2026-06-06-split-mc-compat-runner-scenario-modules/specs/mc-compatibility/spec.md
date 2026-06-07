# mc-compatibility Change Spec: Runner scenario modules

## Requirements

### Requirement: Runner scenario module boundaries

r[mc_compatibility.runner_scenario_modules.boundaries] The runner MUST define explicit module boundaries for scenario core logic before further expanding scenario behavior.

#### Scenario: Scenario core has a small public API

r[mc_compatibility.runner_scenario_modules.boundaries.api]
- GIVEN scenario behavior is maintained in modules
- WHEN other runner code needs a scenario name, alias lookup, milestone list, forbidden pattern, or behavior hook
- THEN it uses the scenario-core API rather than open-coded matches
- AND module exports remain limited to the data and functions needed by runner orchestration.

### Requirement: Pure scenario core extraction

r[mc_compatibility.runner_scenario_modules.scenario_core] Scenario identity, static specs, behavior lookup, and spec validation SHOULD live in pure scenario modules.

#### Scenario: Scenario validation is testable without orchestration

r[mc_compatibility.runner_scenario_modules.scenario_core.pure]
- GIVEN invalid or valid scenario specs are constructed in memory
- WHEN scenario validation tests run
- THEN validation results are produced without starting servers, clients, reading files, writing receipts, or depending on process environment.

### Requirement: Imperative runner shell remains explicit

r[mc_compatibility.runner_scenario_modules.imperative_shell] CLI parsing, backend/client orchestration, environment mutation, log collection, and receipt writing MUST remain in imperative shell code.

#### Scenario: Side effects do not enter scenario validation

r[mc_compatibility.runner_scenario_modules.imperative_shell.side_effects]
- GIVEN scenario validation or behavior lookup is executed
- WHEN tests inspect the scenario-core path
- THEN it performs no filesystem, process, clock, network, or environment side effects
- AND side-effectful runner operations stay in named orchestration code.

### Requirement: Runner surface parity

r[mc_compatibility.runner_scenario_modules.surface_parity] The module split MUST preserve existing compatibility and evidence surfaces unless a separate change explicitly expands them.

#### Scenario: Public runner output remains stable

r[mc_compatibility.runner_scenario_modules.surface_parity.outputs]
- GIVEN the scenario module split is complete
- WHEN existing dry-run, manifest, receipt, and evidence-evaluation paths run
- THEN scenario names, accepted aliases, required milestones, forbidden patterns, receipt fields, non-claim flags, and checker-visible manifest rows match the pre-split behavior.

### Requirement: Runner scenario module tests

r[mc_compatibility.runner_scenario_modules.tests] The change MUST include positive parity tests and negative invalid-definition tests for the extracted modules.

#### Scenario: Module tests prove both success and fail-closed behavior

r[mc_compatibility.runner_scenario_modules.tests.coverage]
- GIVEN the extracted scenario modules expose validation and lookup functions
- WHEN module tests run
- THEN every valid scenario passes parity checks
- AND invalid fixtures for duplicate canonical names, missing aliases, missing milestones, and unsupported behavior defaults fail with explicit diagnostics.

### Requirement: Runner scenario module validation

r[mc_compatibility.runner_scenario_modules.validation] The change MUST record runner tests, manifest checks, dry-run checks, Cairn gates, and Cairn validation before archive.

#### Scenario: Structural split is reviewable

r[mc_compatibility.runner_scenario_modules.validation.logs]
- GIVEN the runner module split is complete
- WHEN the change is archived
- THEN reviewable logs show runner tests, scenario manifest checks, dry-run checks, Cairn proposal/design/tasks gates, and Cairn validation passing.
