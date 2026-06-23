# Proposal: Modularize the compatibility runner core

## Why

`compat/runner/src/main.rs` has grown into a large mixed CLI, process-orchestration, scenario, receipt, probing, and validation surface. That makes scenario changes risky, keeps pure logic hard to test without standing up client/server processes, and currently allows scenario code to depend on constants defined by the imperative shell.

## What Changes

- Split compatibility runner logic into explicit functional-core modules or crates for scenario definitions, receipt models, validation, and runtime config normalization.
- Keep process launch, filesystem reads/writes, Paper/Docker handling, logging, and CLI exit behavior in a thin imperative shell.
- Invert dependencies so pure scenario/receipt code does not import from `main.rs`.
- Preserve existing CLI names, flake app names, receipt schemas, scenario semantics, and non-claim boundaries during the first modularization pass.
- Add positive and negative tests around extracted core behavior before deleting compatibility shims.

## Impact

- **Files**: `compat/runner/src/*`, optional `compat/crates/*`, runner tests, flake package paths if crates move, README/docs architecture notes, and Cairn artifacts.
- **Testing**: focused runner unit tests, scenario parsing tests, receipt validation tests, dry-run receipt checks, maintained dry-run aggregate, and Cairn validation/gates.
- **Non-claims**: this is a maintainability/refactor change only; it does not claim new compatibility coverage, live parity, protocol support, or production readiness.
