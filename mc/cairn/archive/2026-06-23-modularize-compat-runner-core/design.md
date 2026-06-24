# Design: Modularize the compatibility runner core

## Context

The runner already has some module boundaries (`layout`, `runtime_config`, `scenario_core`, generated manifest data), but `main.rs` still owns many constants and behaviors that pure modules need. This keeps dependency direction inverted and makes targeted tests difficult.

## Decisions

### 1. Preserve public behavior while changing internals

**Choice:** Keep existing binary name, CLI arguments, flake app names, receipt schema versions, scenario names, milestone semantics, and explicit non-claims stable during the modularization pass.

**Rationale:** Reviewers should be able to validate refactor safety with existing dry-run receipts and scenario checks.

### 2. Extract pure core before moving shells

**Choice:** First move scenario definitions, milestone/forbidden-pattern specs, receipt models, receipt validation, and config normalization into pure modules or crates. Leave filesystem, process, Docker/Paper, sockets, clocks, environment, stdout/stderr, and exit-code handling in the shell.

**Rationale:** Pure logic can be tested with plain inputs and deterministic assertions before any orchestration is touched.

### 3. Make dependency direction explicit

**Choice:** `main.rs` may import core modules, but core modules must not import constants or helpers from `main.rs`.

**Rationale:** This prevents shell growth from leaking back into scenario semantics.

### 4. Migrate incrementally with compatibility shims

**Choice:** Allow short-lived re-exports or adapter functions while tests prove equivalence, then remove them once callers are migrated.

**Rationale:** The runner surface is broad; incremental movement reduces review risk.

## Risks / Trade-offs

- Moving too much at once can obscure behavior changes; mitigate with small extraction commits and parity tests.
- New crates can increase flake/Cargo wiring; mitigate by keeping crate boundaries aligned to existing responsibilities.
- Generated scenario tables may need coordinated movement; mitigate by preserving generated file identity until a dedicated generation change updates it.
