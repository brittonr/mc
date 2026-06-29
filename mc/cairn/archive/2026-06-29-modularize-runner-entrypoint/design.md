# Design: Modularize mc-compat runner entrypoint

## Context

The runner already has useful pure modules, but `main.rs` remains the de facto integration surface for unrelated concerns. The next modularity step is to make ownership boundaries explicit without changing public behavior.

## Decisions

### 1. Keep `main.rs` boring

**Choice:** `main.rs` should contain module declarations, `main()`, process-exit mapping, and delegation to an application module.

**Rationale:** New behavior will have to choose an owner instead of defaulting to the entrypoint.

### 2. Extract cohesive owner modules

**Choice:** Move config parsing/defaulting, backend runtime dispatch, mode execution, plan data types, scenario behavior adapters, and receipt/failure-bundle shells into modules named for their responsibility.

**Rationale:** Each module can expose a narrow API and carry its own tests.

### 3. Preserve behavior with adapters first

**Choice:** Extract by moving code behind compatibility-preserving APIs before reshaping internals.

**Rationale:** A large architecture refactor should not also alter CLI, receipt, or scenario semantics.

### 4. Validate positive and negative paths

**Choice:** Keep representative success and fail-closed fixtures for config parsing, mode dispatch, plan generation, receipt/failure-bundle follow-up errors, and unknown arguments.

**Rationale:** Modularity is only safe if the shell still fails in the same places.

## Risks / Trade-offs

- Moving many symbols can create broad import churn; mitigate by extracting one owner module at a time.
- Temporary adapters may look redundant; keep them only until follow-up Cairns reshape internals.
- Existing tests may initially move slower than code; require test relocation before archive.
