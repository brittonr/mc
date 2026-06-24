# Design: Introduce a typed scenario command router

## Context

Maintained scenario wrappers currently exist as many aliases. They should remain user-friendly, but a typed router can centralize validation and reduce flake/script duplication.

## Decisions

### 1. Add router without removing aliases

**Choice:** Introduce the typed route as an internal and documented command while keeping existing flake app aliases available.

**Rationale:** Users and evidence workflows should not break during migration.

### 2. Validate before side effects

**Choice:** Scenario, backend, receipt path, timeout, live/dry-run, and evidence options are parsed and validated before launching client/server processes or writing receipts.

**Rationale:** Bad command shapes should fail closed without partial artifacts.

### 3. Drive router metadata from manifest where possible

**Choice:** Scenario names, aliases, defaults, maintained status, and dry-run metadata should come from the scenario manifest or generated scenario tables.

**Rationale:** The router should reduce duplication rather than become another table.

### 4. Keep aliases as generated compatibility shims

**Choice:** Existing alias names can be generated or manually wrapped to call the router with explicit typed arguments.

**Rationale:** This preserves public command names while consolidating implementation.

## Risks / Trade-offs

- Router introduction can change error messages; mitigate with focused diagnostics tests.
- Alias parity can drift; mitigate with dry-run comparison checks.
- A generic command can hide scenario-specific caveats; mitigate with scenario metadata and docs links.
