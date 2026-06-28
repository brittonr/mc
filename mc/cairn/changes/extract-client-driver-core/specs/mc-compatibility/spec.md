# mc-compatibility Change Spec: Client-driver functional core

## Requirements

### Requirement: Client-driver pure core

r[mc_compatibility.runner_modularity.client_driver_core] Non-trivial client-driver decisions SHOULD be implemented as pure deterministic cores over explicit config, run records, logs, and scenario metadata.

#### Scenario: Client-driver logic is testable without live processes

r[mc_compatibility.runner_modularity.client_driver_core.pure]
- GIVEN client-driver logic derives run plans, combines logs, evaluates scenario evidence, or classifies outcomes
- WHEN that logic is invoked by tests
- THEN it can run without Xvfb, process spawning, filesystem reads, server restarts, sleeps, or stdout/stderr.

### Requirement: Client run planning

r[mc_compatibility.runner_modularity.client_run_planning] The runner SHOULD derive client usernames, session counts, timeouts, log strategies, restart needs, and dry-run evidence modes through pure client run planning.

#### Scenario: Run plan is deterministic

r[mc_compatibility.runner_modularity.client_run_planning.deterministic]
- GIVEN the same config and scenario metadata
- WHEN client run planning runs repeatedly
- THEN it returns the same client run plan each time.

### Requirement: Client evidence classification

r[mc_compatibility.runner_modularity.client_evidence_classification] Client evidence classification MUST preserve existing classification strings, evidence fields, scenario evaluation, server-correlation behavior, projectile checks, and non-claim boundaries.

#### Scenario: Classification preserves evidence contract

r[mc_compatibility.runner_modularity.client_evidence_classification.parity]
- GIVEN client run records and logs equivalent to pre-refactor behavior
- WHEN the pure classification core evaluates them
- THEN it returns the same classification, evidence fields, pass/fail result, and diagnostics as the pre-refactor runner.

### Requirement: Client-driver shell boundary

r[mc_compatibility.runner_modularity.client_driver_shell] Client-driver shell code MUST own process execution, timeout handling, filesystem log access, restart transitions, stdout/stderr, and error plumbing without duplicating pure evidence policy.

#### Scenario: Shell produces run records

r[mc_compatibility.runner_modularity.client_driver_shell.records]
- GIVEN a live client scenario is executed
- WHEN shell code finishes a client process or restart transition
- THEN it produces run records for the pure core
- AND shell code does not recompute evidence classification policy.

### Requirement: Client-driver positive tests

r[mc_compatibility.runner_modularity.client_driver_positive_tests] The change MUST include positive tests for dry-run evidence, successful single-client, reconnect, multi-client, projectile, and timeout-success classifications.

#### Scenario: Supported classifications pass

r[mc_compatibility.runner_modularity.client_driver_positive_tests.coverage]
- GIVEN representative successful run records and logs
- WHEN the pure client-driver core evaluates them
- THEN it produces the expected passing classifications and evidence fields.

### Requirement: Client-driver negative tests

r[mc_compatibility.runner_modularity.client_driver_negative_tests] The change MUST include negative tests for missing milestones, forbidden markers, bad exit codes, missing server correlation, projectile order failures, and restart-state failures.

#### Scenario: Bad evidence fails closed

r[mc_compatibility.runner_modularity.client_driver_negative_tests.fail_closed]
- GIVEN malformed or incomplete run records and logs
- WHEN the pure client-driver core evaluates them
- THEN it rejects the evidence with actionable diagnostics before receipts claim success.

### Requirement: Client-driver validation

r[mc_compatibility.runner_modularity.client_driver_validation] The change MUST record focused client-driver tests, runner tests, dry-run smoke checks, Cairn gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[mc_compatibility.runner_modularity.client_driver_validation.logs]
- GIVEN client-driver core extraction is complete
- WHEN the change is closed
- THEN reviewable logs show core parity, positive and negative fixtures, dry-run smoke checks, and Cairn validation passing.
