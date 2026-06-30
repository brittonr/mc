# Tasks

- [ ] [serial] Inventory `compat/runner/src/lib.rs` responsibilities, CLI/dry-run/receipt/failure-bundle public surfaces, and run baseline runner tests plus maintained dry-runs. r[mc_compatibility.runner_shell_modularization.inventory]
- [ ] [serial] Extract CLI parsing, scenario route compatibility, orchestration, env patch planning, receipt writing, and failure-bundle writing into focused modules while keeping `run_main()` as a thin façade. r[mc_compatibility.runner_shell_modularization.module_boundaries]
- [ ] [serial] Ensure deterministic runner decisions are pure over explicit inputs and side-effecting shells own filesystem, process, Docker, socket, clock, and environment access. r[mc_compatibility.runner_shell_modularization.core_shell]
- [ ] [serial] Preserve CLI flags, aliases, flake app behavior, exit-code behavior, receipt schemas, dry-run text, failure-bundle shape, and non-claim boundaries. r[mc_compatibility.runner_shell_modularization.parity]
- [ ] [serial] Add positive tests for supported parser/planner/env/receipt paths and negative tests for unknown flags, missing values, unsafe paths, invalid config, stale outputs, and failed preflights. r[mc_compatibility.runner_shell_modularization.tests]
- [ ] [serial] Run runner tests, generated-surface checks when touched, affected mc-compat dry-runs, Cairn proposal/design/tasks gates, Cairn validation, and task-evidence checks with reviewable logs before archive. r[mc_compatibility.runner_shell_modularization.validation]
