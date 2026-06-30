# mc-compatibility Change Spec: mc-compat runner shell modules

## Requirements

### Requirement: Runner shell modularization inventory

r[mc_compatibility.runner_shell_modularization.inventory] Runner shell modularization work MUST inventory current `compat/runner/src/lib.rs` responsibilities, public CLI and dry-run surfaces, receipt schemas, failure-bundle behavior, and baseline validation before extraction.

#### Scenario: Runner shell ownership is reviewable

r[mc_compatibility.runner_shell_modularization.inventory.reviewable]
- GIVEN runner shell modularization is selected
- WHEN reviewers inspect the inventory
- THEN CLI parsing, scenario routing, configuration, orchestration, env patching, receipt writing, failure-bundle writing, backend lifecycle, and public wrapper dependencies are named
- AND baseline validation commands are recorded before core changes.

### Requirement: Runner shell module boundaries

r[mc_compatibility.runner_shell_modularization.module_boundaries] The runner SHOULD keep `lib.rs` as a thin public façade and expose focused modules for CLI parsing, scenario route compatibility, orchestration, environment patch planning, receipt artifact writing, and failure-bundle artifact writing.

#### Scenario: Runner responsibilities have focused owners

r[mc_compatibility.runner_shell_modularization.module_boundaries.focused]
- GIVEN a runner responsibility is reviewed
- WHEN maintainers inspect the runner module tree
- THEN the responsibility is owned by the focused module for its domain
- AND unrelated CLI, environment, receipt, orchestration, and artifact-writing concerns are not reintroduced into one catch-all shell file.

### Requirement: Runner core and shell boundary

r[mc_compatibility.runner_shell_modularization.core_shell] Deterministic runner decisions SHOULD be pure over explicit inputs, while filesystem reads/writes, process execution, Docker lifecycle, sockets, clocks, environment reads, stdout/stderr, and exit-code handling remain in thin shells.

#### Scenario: Runner decisions are testable without side effects

r[mc_compatibility.runner_shell_modularization.core_shell.testable]
- GIVEN explicit CLI arguments, config patches, scenario metadata, and receipt inputs
- WHEN the extracted runner core computes parser, plan, env, receipt, or failure-bundle decisions
- THEN tests can verify the result without touching files, processes, sockets, Docker, clocks, or ambient environment
- AND shells own the side effects.

### Requirement: Runner shell parity

r[mc_compatibility.runner_shell_modularization.parity] Runner shell modularization MUST preserve CLI flags and aliases, flake app behavior, exit-code behavior, receipt schemas, dry-run text, failure-bundle shape, and non-claim boundaries.

#### Scenario: Existing runner command shape remains stable

r[mc_compatibility.runner_shell_modularization.parity.stable]
- GIVEN a supported pre-refactor runner command or wrapper dry-run
- WHEN the modularized runner processes the same input
- THEN the public command shape, dry-run output, receipt schema, failure-bundle fields, and non-claim text remain equivalent
- AND no new live compatibility or semantic parity claim is introduced.

### Requirement: Runner shell modularization tests

r[mc_compatibility.runner_shell_modularization.tests] The change MUST include positive tests for supported parser, planner, environment, receipt, and artifact paths plus negative tests for unknown flags, missing values, unsafe paths, invalid config, stale outputs, and failed preflights.

#### Scenario: Supported runner paths pass

r[mc_compatibility.runner_shell_modularization.tests.positive]
- GIVEN representative supported runner inputs
- WHEN extracted modules process them
- THEN tests prove expected config, plan, env patch, receipt, and artifact decisions are produced.

#### Scenario: Invalid runner paths fail clearly

r[mc_compatibility.runner_shell_modularization.tests.negative]
- GIVEN invalid flags, missing values, unsafe output paths, malformed config, stale generated output, or failed preflight inputs
- WHEN extracted modules process them
- THEN tests prove diagnostics are specific and the runner fails closed without writing misleading evidence.

### Requirement: Runner shell modularization validation

r[mc_compatibility.runner_shell_modularization.validation] The change MUST record runner tests, generated-surface checks when touched, affected mc-compat dry-runs, Cairn proposal/design/tasks gates, Cairn validation, and task-evidence checks before archive.

#### Scenario: Runner shell closeout is reviewable

r[mc_compatibility.runner_shell_modularization.validation.logs]
- GIVEN runner shell modularization is complete
- WHEN the change is closed
- THEN reviewable logs show baseline and post-change runner tests, affected dry-runs, positive and negative regression coverage, Cairn gates, and Cairn validation passing.
