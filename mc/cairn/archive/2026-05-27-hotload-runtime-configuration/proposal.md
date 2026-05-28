# Proposal: Steel-first hotloaded runtime configuration

## Summary

Introduce a typed Steel Scheme configuration and policy system for operator-set and harness-set values, with hot reload as the default apply path where safe. Steel becomes the editable runtime source for both plain values and small dynamic rules such as arrow-damage formulas. Rust remains the authority for types, mutability classes, sandbox limits, validation, and final apply decisions.

## Motivation

The compatibility work now depends on many values set across runner commands, environment variables, test rails, game examples, evidence paths, protocol/version selection, ports, timings, scenario knobs, client names, feature toggles, and gameplay tuning such as projectile damage. Leaving those values scattered makes review harder, encourages magic numbers, and forces restarts for changes that could be applied safely at runtime.

Steel is a better fit than a static config language when a value is really policy logic. For example, arrow damage may depend on projectile context, team state, equipment, range, or scenario mode. A hotloaded Scheme module can express that logic directly, while the Rust boundary can keep the output typed, bounded, deterministic, and rollback-safe.

## Scope

- Inventory all operator-set and repo-set runtime values in the compatibility harness, runner, Valence CTF/game rail, and Stevenarella compatibility launch path.
- Define a Steel module interface for runtime config and policy hooks with documented defaults, contracts, host-provided context types, and explicit exports.
- Define Rust-owned typed contracts for every Steel export and reject unknown, missing, type-invalid, range-invalid, stale, nondeterministic, or capability-invalid fields/functions.
- Export a normalized runtime snapshot with schema version, Steel module hash, evaluated values/policy exports, provenance, sandbox settings, and redacted secret handling.
- Implement hot reload with validate-before-swap semantics, compilation/evaluation in isolation, diff logging, rollback on failed validation/apply, and per-field mutability classes.
- Mark unsafe values as restart-only or next-run-only rather than pretending every value can hotload.
- Add tests and evidence for positive reloads, negative malformed Steel modules, sandbox violations, rollback behavior, unknown exports, missing exports, type/range failures, and restart-only changes.
- Keep Nickel out of the initial implementation path except as a possible future static export/input format if a later proof needs it.

## Out of scope

- Full production config UI.
- Remote config distribution.
- Secret storage beyond redaction and external secret references.
- Replacing domain constants that are protocol facts rather than operator-set values.
- Allowing arbitrary Steel code to mutate live server state, perform I/O, access ambient time/randomness, or bypass the typed Rust boundary.

## Impact

- **Files**: new Steel config/policy modules, Rust typed boundary and sandbox loader, config inventory evidence, tests, and updates to call sites currently setting runtime values directly.
- **Testing**: unit tests for pure normalization/diff/apply-plan logic; sandbox tests for Steel module loading and deterministic policy output; integration tests for file reload and rollback; negative tests for malformed modules and unsafe config changes; Cairn evidence under `docs/evidence/`.
