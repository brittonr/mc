## Why

Targeted packet live promotion currently relies on manual reasoning about which runner scenarios can prove which packet rows. A scenario live-probe capability registry would make those capabilities explicit, reviewable, and testable before more packet rows are promoted.

## What Changes

- Add a pure registry that maps scenario capabilities to packet rows, required live signals, backend/client paths, evidence mode, and non-claims.
- Expose lookup/validation helpers that fail closed for unknown rows, duplicate capabilities, unsupported evidence modes, or missing non-claims.
- Use the registry to guide future live-promotion selection and blocker reporting.
- Keep registry metadata separate from runner process orchestration.

## Impact

- **Files**: `tools/mc-compat-runner/src/scenario_core.rs` or a new scenario capability module, scenario manifest/checker tooling, `docs/evidence/**`, Cairn specs/tasks.
- **Testing**: Registry positive/negative unit tests, scenario manifest checks, runner dry-runs as relevant, evidence-manifest/task-evidence checks, Cairn gates and validation.
