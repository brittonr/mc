# Proposal: Introduce server backend runtime traits

## Why

`tools/mc-compat-runner` carries Valence and Paper behavior behind repeated `match ServerBackend` blocks for naming, ports, start/stop, log access, and matrix execution. That makes every backend-adjacent change touch scattered imperative code and increases the chance that dry-run, cleanup, or receipt behavior drifts between backends.

## What Changes

- Add a small `ServerRuntime` trait boundary for backend name, default port, lifecycle operations, and log access.
- Keep Valence and Paper behavior as explicit implementations with no change to command shape, receipt fields, ports, or evidence semantics.
- Route existing enum parsing and CLI config through a thin dispatch layer so public CLI behavior remains stable while backend-specific code becomes locally testable.
- Add positive and negative tests that prove both backend implementations preserve current behavior and reject unknown backend names.

## Impact

- **Files**: `tools/mc-compat-runner/src/main.rs` and focused runner tests.
- **Testing**: CLI parsing tests, backend default-port/name tests, dry-run lifecycle tests, negative unknown-backend tests, and Cairn validation/gates.