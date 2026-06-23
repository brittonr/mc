# Design: Retire legacy layout transition paths

## Context

The resolver supports role paths such as `clients/stevenarella` and legacy transition paths such as direct root-level component names. Dual support makes ambiguity checks necessary and keeps old names alive in docs and tests.

## Decisions

### 1. Inventory before removal

**Choice:** Search code, docs, flake wrappers, evidence notes, and tests for transition-path references before removing resolver support.

**Rationale:** Some references may be review evidence or historical docs and should be updated, archived, or explicitly left as history.

### 2. Canonical role paths win

**Choice:** Active commands and current docs should use role-based paths. Legacy paths should fail with actionable diagnostics unless a temporary compatibility shim is explicitly retained.

**Rationale:** One canonical path per role keeps mental model and tooling simple.

### 3. Preserve historical evidence meaning

**Choice:** Archived evidence may mention old paths, but active task/spec docs should cite current canonical paths or explain historical context.

**Rationale:** Evidence should stay reviewable without pretending old paths are active.

### 4. Fail closed on duplicate roots

**Choice:** If both a canonical root and legacy transition root exist, validation should report ambiguity instead of guessing.

**Rationale:** Silent selection can run tests against the wrong tree.

## Risks / Trade-offs

- Removing compatibility too early can break local workflows; mitigate with dry-run diagnostics and a migration note.
- Updating path references can churn evidence docs; mitigate by limiting edits to active docs unless historical clarity is needed.
- Resolver simplification can mask missing checkouts; retain missing-checkout tests.
