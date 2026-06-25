# Proposal: Isolate runner JSON and evidence rendering

## Why

The compatibility runner currently keeps many hand-rolled JSON parsing/rendering helpers inside `compat/runner/src/main.rs` alongside orchestration. Evidence schemas, receipt rendering, failure bundles, typed-event artifacts, and low-level string parsing are therefore coupled to the shell and harder to test for malformed input and overclaim rejection.

## What Changes

- Inventory runner JSON consumers, receipt schemas, evidence artifact types, and manual parser/render helpers.
- Define typed evidence structs and a dedicated JSON/evidence module boundary.
- Move parsing/rendering helpers out of the runner shell without changing checked receipt schemas.
- Add strict positive and negative tests for escaping, malformed values, missing fields, invalid paths, stale overclaims, and schema compatibility.
- Keep broad compatibility claims and non-claim fields unchanged.

## Impact

- **Files**: `compat/runner/src/main.rs`, new `compat/runner/src/evidence/*` or `json.rs`, receipt/failure-bundle tests, docs/evidence schemas if documented.
- **Testing**: runner tests, receipt validation tests, failure-bundle tests, selected dry-runs, evidence manifest/task-evidence checks, and Cairn gates.
