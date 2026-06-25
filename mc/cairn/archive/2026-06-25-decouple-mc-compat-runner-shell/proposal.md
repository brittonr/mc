# Proposal: Decouple mc-compat runner shell

## Why

`compat/runner/src/main.rs` still owns CLI parsing, backend lifecycle, client orchestration, MCP control, planning, evidence rendering, JSON helpers, and tests in one large compilation unit. That shape makes compatibility-preserving changes harder to review and weakens the documented functional-core / imperative-shell boundary.

## What Changes

- Inventory the current runner responsibilities and define stable module boundaries.
- Move pure planning, evidence evaluation, typed-event graphing, and receipt shaping out of the shell before moving process or filesystem code.
- Split Valence/Paper backend lifecycle and client-driving code into shell modules with narrow inputs and outputs.
- Keep CLI flags, environment variables, receipt schemas, scenario semantics, dry-run output, and non-claim boundaries unchanged unless a separate Cairn updates them.
- Add positive parity tests and negative fail-closed tests for moved cores and migrated shells.

## Impact

- **Files**: `compat/runner/src/main.rs`, new modules under `compat/runner/src/`, runner tests, generated scenario/evidence surfaces if imports change, docs/evidence receipts after implementation.
- **Testing**: runner unit tests, dry-run parity checks, receipt validation checks, scenario manifest freshness checks, relevant Cairn gates, and task-evidence validation before archive.
