# Design: Stevenarella resource manager split

## Context

Resource handling is a boundary between untrusted paths/data and rendering/model consumers. The split should make path and lookup rules explicit while preserving public API behavior.

## Decisions

### 1. Split by resource responsibility

**Choice:** Create modules for identifiers/paths, pack discovery, lookup/indexing, cache policy, archive access, IO shell, and shared manager state.

**Rationale:** Path safety and IO behavior need separate tests.

### 2. Extract pure resource decisions

**Choice:** Identifier parsing, lookup precedence, cache key derivation, path containment, and pack selection should be pure over explicit inputs.

**Rationale:** These rules can be tested without filesystem or archive IO.

### 3. Preserve public APIs

**Choice:** Existing resource manager calls remain stable through adapters or re-exports.

**Rationale:** Render/model/UI call sites should not need broad migration.
