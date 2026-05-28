# Proposal: Hotloaded runtime configuration

## Summary

Introduce a typed configuration system for operator-set and harness-set values, with hot reload as the default apply path where safe. Nickel is the source-of-truth configuration language. Steel Scheme is reserved for a narrow, sandboxed extension point only if a later task proves it is needed for dynamic policy logic.

## Motivation

The compatibility work now depends on many values set across runner commands, environment variables, test rails, game examples, evidence paths, protocol/version selection, ports, timings, scenario knobs, client names, and feature toggles. Leaving those values scattered makes review harder, encourages magic numbers, and forces restarts for changes that could be applied safely at runtime.

A config system should make every value we intentionally set visible, typed, documented, and reloadable or explicitly restart-only.

## Scope

- Inventory all operator-set and repo-set runtime values in the compatibility harness, runner, Valence CTF/game rail, and Stevenarella compatibility launch path.
- Define a typed Nickel configuration schema with documented defaults, contracts, merge semantics, and environment-specific overlays.
- Export a normalized runtime snapshot with schema version, source hash, evaluated values, provenance, and redacted secret handling.
- Implement a loader that validates the snapshot before use and rejects unknown, missing, type-invalid, range-invalid, or stale fields.
- Implement hot reload with atomic swap semantics, diff logging, rollback on failed validation/apply, and per-field mutability classes.
- Mark unsafe values as restart-only or next-run-only rather than pretending every value can hotload.
- Add tests and evidence for positive reloads, negative malformed configs, rollback behavior, unknown fields, missing fields, and restart-only changes.
- Document the Steel-vs-Nickel boundary before embedding any Steel Scheme evaluator.

## Out of scope

- Full production config UI.
- Remote config distribution.
- Secret storage beyond redaction and external secret references.
- Replacing domain constants that are protocol facts rather than operator-set values.
- Allowing arbitrary Steel Scheme code to mutate live server state without a typed config boundary.

## Impact

- **Files**: new configuration schema/docs, config loader/hot-reload code, config inventory evidence, tests, and updates to call sites currently setting runtime values directly.
- **Testing**: unit tests for pure normalization/diff/apply-plan logic; integration tests for file reload and rollback; negative tests for malformed and unsafe config changes; Cairn evidence under `docs/evidence/`.
