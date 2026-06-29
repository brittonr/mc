# Proposal: Modularize mc-compat runner entrypoint

## Why

`compat/runner/src/main.rs` still owns too many responsibilities: process exit translation, config parsing, mode dispatch, backend runtime definitions, scenario behavior adapters, planning data types, receipt helpers, failure-bundle helpers, and a large test module. That makes focused refactors harder and encourages new logic to accumulate in the entrypoint.

## What Changes

- Keep `main.rs` as a thin entrypoint that declares modules, translates runner errors into process exits, and delegates to a focused application shell.
- Move config construction, backend runtime definitions, mode dispatch, planning data types, receipt/failure-bundle shells, and scenario behavior adapters into cohesive modules.
- Preserve existing CLI flags, environment variables, receipt schemas, scenario semantics, non-claims, and dry-run/live behavior.
- Add positive and negative parity tests around the extraction boundaries before removing old entrypoint responsibilities.

## Impact

- **Files**: `compat/runner/src/main.rs`, new runner modules, module-local tests, Cairn artifacts, and possibly documentation that describes runner architecture.
- **Testing**: baseline runner tests before extraction, focused module tests after extraction, dry-run smoke checks, scenario manifest checks, Cairn gates, and Cairn validation.
- **Non-claims**: architecture and composability only; this does not promote new Minecraft compatibility evidence or change gameplay semantics.
