# Design: Untangle mc-compat runner module imports

## Context

Wildcard root imports allow modules to compile while depending on implicit state from `main.rs`. Explicit imports make dependencies visible and create pressure to move shared types to stable owner modules.

## Decisions

### 1. Ban production wildcard root imports

**Choice:** Production runner modules should not use `use super::*`; test modules may keep local wildcard imports only where they are scoped to the module under test.

**Rationale:** Production imports are architecture; tests can use local ergonomics without hiding runtime coupling.

### 2. Introduce explicit owner modules for shared types

**Choice:** Move shared types such as config, client evidence, backend runtime, and planning structs into owner modules before replacing imports.

**Rationale:** Explicit imports are only useful when the imported symbols have meaningful homes.

### 3. Add a small boundary guard

**Choice:** Add a focused test or checker that rejects production `use super::*` in runner modules.

**Rationale:** Import coupling tends to regress unless it is mechanically guarded.

### 4. Keep extraction behavior-preserving

**Choice:** The import cleanup must not alter runtime paths, scenario semantics, receipts, or evidence contracts.

**Rationale:** Dependency cleanup should be reviewable as architecture work.

## Risks / Trade-offs

- Explicit imports can create noisy diffs; mitigate by moving owner types first.
- A checker can be too broad; scope it to production modules and allow local test modules.
- Some modules may expose too many symbols initially; tighten APIs after compile parity is restored.
