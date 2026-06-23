# Design: Consolidate evidence checkers into a Rust checker crate

## Context

The current checker set is mostly Rust, with a small shared `checker_framework.rs`, plus legacy Python gates. The flake invokes many single-file scripts. Consolidating should improve reuse without breaking existing review evidence and check names.

## Decisions

### 1. Library first, binaries second

**Choice:** Build a checker library containing pure parsing/validation helpers before migrating individual binaries.

**Rationale:** Shared behavior and tests can stabilize before command wiring changes.

### 2. Preserve external command surfaces

**Choice:** Existing flake check names and checker command names remain available through wrappers or binary aliases during migration.

**Rationale:** Cairn evidence and README instructions should not churn merely because source files moved.

### 3. Every migrated checker gets positive and negative fixtures

**Choice:** A checker is not considered migrated until it has at least one happy-path fixture and at least one failing fixture that proves diagnostics are specific.

**Rationale:** Checkers are evidence gates; false positives are high risk.

### 4. Treat Python as explicit debt

**Choice:** Existing Python gates may remain temporarily, but touched gates must migrate to Rust or receive an explicit Cairn task/waiver explaining why not.

**Rationale:** This matches the workspace preference for Rust/Steel checks without blocking the whole consolidation on legacy gates.

## Risks / Trade-offs

- Moving many checkers can cause noisy diffs; mitigate by migrating in families.
- Binary path changes can break Nix checks; mitigate with wrapper aliases and command-shape tests.
- Shared helpers can become too generic; mitigate by keeping domain-specific validation close to each checker binary.
