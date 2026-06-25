# mc-compatibility Change Spec: Runner shell decoupling

## Requirements

### Requirement: Runner responsibility inventory

r[mc_compatibility.runner_shell_decoupling.inventory] The compatibility runner MUST inventory `main.rs` responsibilities, side effects, compatibility surfaces, and candidate module owners before large extraction work begins.

#### Scenario: Responsibility clusters are reviewable

r[mc_compatibility.runner_shell_decoupling.inventory.reviewable]
- GIVEN runner shell decoupling is selected
- WHEN reviewers inspect the change design or inventory
- THEN CLI parsing, config loading, planning, backend lifecycle, client driving, MCP control, evidence evaluation, receipt rendering, typed-event graphing, JSON/wire helpers, tests, and side effects are each classified
- AND compatibility surfaces that must not drift are named.

### Requirement: Runner module boundary contract

r[mc_compatibility.runner_shell_decoupling.module_boundaries] The runner SHOULD define crate-private modules around stable responsibility boundaries before moving behavior.

#### Scenario: Module APIs are narrow

r[mc_compatibility.runner_shell_decoupling.module_boundaries.narrow]
- GIVEN a runner responsibility is moved out of `main.rs`
- WHEN reviewers inspect the new module API
- THEN the API accepts explicit inputs and returns explicit results or diagnostics
- AND it does not expose unrelated helper state or require callers to know raw CLI parsing details unless it is the CLI module.

### Requirement: Pure runner cores

r[mc_compatibility.runner_shell_decoupling.pure_cores] Planning, scenario/evidence evaluation, typed-event graphing, and receipt shaping MUST be pure deterministic cores over in-memory inputs.

#### Scenario: Pure core has no shell effects

r[mc_compatibility.runner_shell_decoupling.pure_cores.no_effects]
- GIVEN a moved runner core evaluates a plan, scenario, typed-event graph, receipt, or failure bundle
- WHEN the core executes in a unit test
- THEN it returns deterministic data or diagnostics
- AND it does not read files, inspect environment, spawn processes, open sockets, use clocks, write stdout/stderr, or mutate repository state.

### Requirement: Explicit shell modules

r[mc_compatibility.runner_shell_decoupling.shell_modules] Backend lifecycle, client driving, MCP process control, filesystem, socket, and command execution code MUST remain in explicit shell modules or the top-level shell.

#### Scenario: Shell work is plan-driven

r[mc_compatibility.runner_shell_decoupling.shell_modules.plan_driven]
- GIVEN a backend, client, MCP, or artifact shell performs side effects
- WHEN it is invoked by the runner
- THEN it receives a validated plan/config input and returns typed evidence or diagnostics
- AND raw parsing, evidence-policy decisions, and unrelated scenario semantics are not reimplemented inside the shell.

### Requirement: Runner compatibility preservation

r[mc_compatibility.runner_shell_decoupling.compatibility] The decoupling MUST preserve CLI behavior, scenario aliases, generated manifests, receipt schemas, dry-run output, milestone matching, and non-claim boundaries unless a separate Cairn explicitly changes them.

#### Scenario: Existing surfaces remain stable

r[mc_compatibility.runner_shell_decoupling.compatibility.stable]
- GIVEN the runner split is implemented
- WHEN existing dry-run, receipt, scenario manifest, and compare-receipt checks run
- THEN user-visible names, aliases, fields, non-claims, and diagnostics remain compatible with the pre-split contract
- AND any intentional drift is rejected unless backed by another accepted change.

### Requirement: Runner split tests

r[mc_compatibility.runner_shell_decoupling.tests] The runner split MUST include positive parity tests and negative fail-closed tests for moved cores and migrated shells.

#### Scenario: Positive parity is covered

r[mc_compatibility.runner_shell_decoupling.tests.positive]
- GIVEN valid existing runner configurations, scenarios, receipts, typed-event logs, and failure-bundle inputs
- WHEN moved cores and migrated shells are tested
- THEN outputs match the pre-split behavior or documented compatibility contract.

#### Scenario: Invalid inputs fail closed

r[mc_compatibility.runner_shell_decoupling.tests.negative]
- GIVEN malformed plans, invalid artifact paths, unsupported scenario/receipt combinations, malformed typed events, missing milestones, forbidden milestones, or backend/client shell failures
- WHEN the moved modules evaluate them
- THEN deterministic diagnostics are returned
- AND no false successful evidence or overbroad compatibility claim is emitted.

### Requirement: Runner decoupling validation

r[mc_compatibility.runner_shell_decoupling.validation] Runner decoupling work MUST record focused runner tests, scenario manifest checks, receipt validation, selected dry-runs, Cairn gates, and task-evidence checks before archive.

#### Scenario: Runner closeout is reviewable

r[mc_compatibility.runner_shell_decoupling.validation.log]
- GIVEN the runner split is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show positive and negative runner tests, compatibility-preserving dry-runs, receipt/schema checks, scenario manifest freshness, Cairn proposal/design/tasks gates, Cairn validation, and task-evidence validation.
