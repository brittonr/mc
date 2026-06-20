# mc-compatibility Change Spec: Harness planning core

## Requirements

### Requirement: Harness planning-core contract

r[mc_compatibility.harness_planning_core.contract] Non-trivial mc-compat runner orchestration logic SHOULD be represented by pure planning cores before shell code performs side effects.

#### Scenario: Plan core has no side effects

r[mc_compatibility.harness_planning_core.contract.pure]
- GIVEN runner orchestration logic derives server, client, receipt, artifact, or cleanup intent
- WHEN that logic is moved into the planning core
- THEN it performs no filesystem reads, filesystem writes, process execution, Docker calls, environment mutation, sleeps, clocks, or network probes
- AND it returns deterministic plans or diagnostics from explicit inputs.

### Requirement: Explicit plan structs

r[mc_compatibility.harness_planning_core.plan_structs] The runner SHOULD define focused plan structs for server startup, client sessions, receipt output, artifact collection, and cleanup actions.

#### Scenario: Plan records orchestration intent

r[mc_compatibility.harness_planning_core.plan_structs.intent]
- GIVEN a validated runner config and scenario metadata
- WHEN plan generation runs
- THEN the resulting plans name backend, ports, client sessions, scenario expectations, receipt destinations, artifact paths, cleanup actions, and non-claim context without launching external services.

### Requirement: Thin imperative shell

r[mc_compatibility.harness_planning_core.shell] Side-effecting runner code MUST remain in thin shell functions that consume plans and report outcomes without duplicating planning policy.

#### Scenario: Shell executes rather than decides

r[mc_compatibility.harness_planning_core.shell.boundary]
- GIVEN the shell starts servers, clients, Docker containers, cleanup, or artifact collection
- WHEN it executes a plan
- THEN plan policy has already been computed by the core
- AND shell code owns only I/O, process management, environment mutation, and error plumbing.

### Requirement: Positive planning fixtures

r[mc_compatibility.harness_planning_core.positive_tests] The change MUST include positive tests for representative dry-run, live, matrix, reconnect, multi-client, Paper, Valence, cleanup, and failure-bundle planning paths.

#### Scenario: Supported plans are deterministic

r[mc_compatibility.harness_planning_core.positive_tests.coverage]
- GIVEN representative supported configurations
- WHEN plan generation runs repeatedly
- THEN it returns the same plan data for the same inputs
- AND preserves existing CLI defaults, scenario names, receipt paths, and non-claim boundaries.

### Requirement: Negative planning fixtures

r[mc_compatibility.harness_planning_core.negative_tests] The change MUST include negative tests for invalid backend/config combinations, unsafe public-server inputs, missing receipt destinations, matrix flag conflicts, path hazards, and cleanup hazards.

#### Scenario: Unsafe plan fails before side effects

r[mc_compatibility.harness_planning_core.negative_tests.fail_closed]
- GIVEN a configuration would target an unsafe public server, escape an artifact path, conflict matrix flags, or remove an unsafe cleanup path
- WHEN plan generation runs
- THEN it returns diagnostics before any shell side effect is attempted.

### Requirement: Harness planning-core validation

r[mc_compatibility.harness_planning_core.validation] The change MUST record baseline runner tests before refactor, post-refactor runner tests, plan-core fixtures, scenario-manifest checks, affected dry-run checks, evidence manifest checks, and Cairn gates before archive.

#### Scenario: Validation proves architecture parity

r[mc_compatibility.harness_planning_core.validation.log]
- GIVEN planning core extraction is complete
- WHEN the change is archived
- THEN reviewable logs show baseline and post-refactor runner tests, positive and negative plan fixtures, Cairn proposal/design/tasks gates, and Cairn validation.
