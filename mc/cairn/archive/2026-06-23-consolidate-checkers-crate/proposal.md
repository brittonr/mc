# Proposal: Consolidate evidence checkers into a Rust checker crate

## Why

`tools/` contains many standalone checkers plus shared Rust helper code and legacy Python gates. The loose script layout makes shared diagnostics, fixtures, binary wiring, and positive/negative tests harder to keep consistent. A repo-owned Rust checker crate can preserve existing gate names while giving checkers a common pure validation core.

## What Changes

- Create a Rust checker package or workspace member for evidence/checker binaries.
- Move shared key-value parsing, diagnostics, claim-boundary helpers, BLAKE3/evidence helpers, and fixture harnesses into a library module.
- Migrate Rust standalone checkers into `src/bin` or equivalent binary targets while preserving invoked command names through flake wrappers.
- Keep legacy Python checkers only as explicit migration debt; migrate touched Python gates before extending them.
- Add positive and negative fixtures for each migrated checker.

## Impact

- **Files**: `tools/*.rs`, legacy `tools/*.py`, new checker crate files, flake check/wrapper wiring, docs/evidence gate references, README/architecture notes, and Cairn artifacts.
- **Testing**: checker unit tests, valid fixture tests, invalid fixture tests, selected flake checks, evidence-manifest/task-evidence gates if touched, and Cairn validation/gates.
- **Non-claims**: this is checker maintainability only; it does not create new live compatibility evidence or broaden any compatibility claim.
