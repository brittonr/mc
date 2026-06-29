# Design: Stevenarella ECS core modules

## Context

The client ECS is a small local framework with broad responsibilities. Modularity should improve maintainability while preserving existing APIs and execution behavior.

## Decisions

### 1. Split ECS primitives

**Choice:** Create owners for entity identifiers, component storage, query access, system registration, system execution, and diagnostics.

**Rationale:** ECS state changes and system orchestration have different invariants.

### 2. Extract deterministic decisions

**Choice:** Allocation, lookup, query-shape, system ordering, and diagnostic decisions should be testable without running full client systems.

**Rationale:** ECS invariants need focused positive and negative tests.

### 3. Preserve public API compatibility

**Choice:** Existing ECS APIs remain stable through re-exports or adapters during the split.

**Rationale:** Most client code depends on these APIs.
