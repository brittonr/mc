# Design: Colocate mc-compat runner module tests

## Context

Tests should document ownership. A root-level test module that validates every subsystem makes ownership ambiguous and creates friction when modules move.

## Decisions

### 1. Move tests beside owner modules

**Choice:** Config tests live with config, planning tests with planning, wire tests with wire, layout tests with layout, receipt tests with receipt modules, evidence tests with evidence modules, and client-driver tests with client-driver/core modules.

**Rationale:** Local tests make module contracts visible.

### 2. Keep integration tests separate

**Choice:** Tests that exercise cross-module runner behavior remain in a crate-root integration area with explicit fixture setup.

**Rationale:** Cross-module behavior is valuable but should not obscure unit ownership.

### 3. Add deterministic test support

**Choice:** Shared fixtures should live in a small test-support module with no hidden environment mutation or persistent filesystem assumptions.

**Rationale:** Moved tests should not duplicate brittle helpers.

### 4. Preserve positive and negative coverage

**Choice:** Each moved test family must retain happy-path and fail-closed fixtures before the old root test block is removed.

**Rationale:** Test moves are behavior-preserving only when coverage remains visible.

## Risks / Trade-offs

- Moving tests can break access to private helpers; prefer testing public module contracts or adding narrow test-only helpers.
- Shared fixtures can become a dumping ground; keep helpers scoped and deterministic.
- Large moves can make review harder; migrate by test family.
