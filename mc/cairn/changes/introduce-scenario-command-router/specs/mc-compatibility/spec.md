# mc-compatibility Change Spec: Scenario command router

## Requirements

### Requirement: Typed scenario router contract

r[mc_compatibility.scenario_command_router.contract] The compatibility harness SHOULD provide a typed scenario command router that accepts explicit scenario, backend, dry-run/live, receipt, timeout, and evidence options.

#### Scenario: Router command is explicit

r[mc_compatibility.scenario_command_router.contract.explicit]
- GIVEN a user invokes the scenario router
- WHEN the command is parsed
- THEN the selected scenario, backend, dry-run/live mode, receipt path, timeout, and evidence options are represented in a typed route request
- AND broad compatibility, production, or semantic-equivalence claims remain false unless a separate evidence contract allows them.

### Requirement: Pure route planning

r[mc_compatibility.scenario_command_router.plan] Scenario router planning MUST validate typed inputs before launching processes, opening sockets, writing receipts, or mutating files.

#### Scenario: Invalid route fails before side effects

r[mc_compatibility.scenario_command_router.plan.invalid]
- GIVEN a route request has an unknown scenario, unsupported backend, unsafe receipt path, invalid timeout, or blocked live option
- WHEN route planning runs
- THEN deterministic diagnostics are returned
- AND no client, server, Paper container, receipt write, or evidence mutation occurs.

### Requirement: Alias parity

r[mc_compatibility.scenario_command_router.alias_parity] Existing maintained flake aliases MUST preserve their public names and dry-run command shapes while routing through the typed scenario command unless explicitly deprecated by a separate change.

#### Scenario: Alias dry-run matches router plan

r[mc_compatibility.scenario_command_router.alias_parity.dry_run]
- GIVEN an existing maintained flake alias is routed through the typed command
- WHEN its dry-run is executed
- THEN the route plan names the same scenario, backend defaults, receipt defaults, timeout behavior, and non-claim boundaries as the pre-router alias.

### Requirement: Router tests

r[mc_compatibility.scenario_command_router.tests] The router MUST include positive and negative tests for scenario lookup, backend validation, receipt path validation, live/dry-run constraints, and alias parity.

#### Scenario: Negative router fixtures fail closed

r[mc_compatibility.scenario_command_router.tests.negative]
- GIVEN router fixtures contain an unknown scenario, invalid backend, path escape, missing required option, or overclaiming flag
- WHEN router tests run
- THEN each invalid fixture fails with a specific diagnostic
- AND no invalid route is executed.

### Requirement: Router documentation

r[mc_compatibility.scenario_command_router.docs] Command documentation SHOULD describe the typed router and keep generated alias/index docs fresh when aliases route through it.

#### Scenario: Router docs match generated aliases

r[mc_compatibility.scenario_command_router.docs.fresh]
- GIVEN alias wrappers are generated or routed through the typed command
- WHEN docs freshness checks run
- THEN command docs match the generated alias metadata
- AND scenario-specific caveats remain human-authored or linked.

### Requirement: Router validation

r[mc_compatibility.scenario_command_router.validation] Router introduction MUST record router tests, alias parity dry-runs, maintained dry-run aggregate output, and Cairn gates before archive.

#### Scenario: Router closeout is reviewable

r[mc_compatibility.scenario_command_router.validation.log]
- GIVEN scenario aliases route through the typed command
- WHEN the change is archived
- THEN reviewable logs show positive router tests, negative router tests, alias parity dry-runs, maintained dry-run aggregate output, Cairn proposal/design/tasks gates, and Cairn validation.
