# Proposal: Colocate mc-compat runner module tests

## Why

A very large test module in `main.rs` mixes config, planning, wire, layout, evidence, receipt, scenario, and client-driver tests. That makes it harder to see which module owns a behavior and slows modular refactors because tests are not near the code they protect.

## What Changes

- Move unit tests from the root entrypoint into the modules that own the tested behavior.
- Keep only true integration-style runner tests at the crate root or under `tests/`.
- Introduce deterministic shared test-support helpers for fixtures that are used by more than one module.
- Preserve positive and negative coverage during the move.

## Impact

- **Files**: `compat/runner/src/main.rs`, module-local test modules, optional `test_support` module, integration tests, and Cairn artifacts.
- **Testing**: baseline test inventory, module-local positive/negative tests, integration test smoke, runner tests, Cairn gates, and Cairn validation.
- **Non-claims**: test organization only; no new compatibility evidence or gameplay behavior changes.
