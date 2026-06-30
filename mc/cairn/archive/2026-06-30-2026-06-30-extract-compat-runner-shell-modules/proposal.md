# Proposal: Extract mc-compat runner shell modules

## Why

`compat/runner/src/lib.rs` is documented as the application shell, but it still owns CLI parsing, scenario router compatibility shims, environment patch construction, scenario behavior env mapping, safety preflights, orchestration, receipt writing, failure bundle writes, and assorted constants. This keeps side-effecting process/filesystem code and pure runner decisions tangled.

## What Changes

- Inventory the current runner shell responsibilities, public CLI surfaces, receipt schemas, and dry-run wrapper dependencies.
- Split `lib.rs` into focused shell modules for CLI/config parsing, scenario route compatibility, orchestration, environment patch construction, receipt artifact writing, failure-bundle artifact writing, and small public façade entrypoints.
- Move deterministic runner decisions into pure cores where they can be tested without filesystem, processes, sockets, clocks, Docker, or environment reads.
- Preserve existing CLI names/flags, flake app behavior, receipt schemas, non-claims, exit-code behavior, failure-bundle shape, and dry-run output.
- Add positive and negative tests around the extracted parser/planner/env/receipt cores and thin shells.

## Impact

- **Files**: `compat/runner/src/lib.rs`, new `compat/runner/src/{cli,orchestration,scenario_route,env_patches,receipt_writer,failure_bundle_shell}.rs` or equivalent modules, runner tests, generated-surface checks if ownership docs change, and Cairn artifacts.
- **Testing**: baseline runner tests and maintained dry-runs before extraction; post-change runner tests, generated-surface checks when touched, affected mc-compat dry-runs, Cairn gates, and Cairn validation.
- **Non-claims**: runner architecture only; this does not create new live compatibility, semantic parity, public-server safety, or production-readiness evidence.
