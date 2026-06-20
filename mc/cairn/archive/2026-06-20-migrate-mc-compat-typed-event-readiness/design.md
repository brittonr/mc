# Design: Migrate mc-compat scenarios to typed-event readiness

## Context

The runner already emits and validates typed event evidence for representative scenarios, but the manifest still identifies all rows as substring fallback. The next step is to make typed-event readiness a row-level contract and stop accepting fallback for new maintained rows without review.

## Decisions

### 1. Define row-level readiness

**Choice:** A scenario is `typed-event-ready` only when required client milestones, required server milestones, and forbidden patterns have typed-event equivalents or explicit typed-event derivation rules.

**Rationale:** Partial conversion can otherwise hide gaps behind the migration label.

### 2. Gate new fallback usage

**Choice:** The scenario-manifest checker will reject newly maintained fallback rows unless waiver metadata explains the blocker and next action.

**Rationale:** Fallback should shrink over time instead of remaining the default.

### 3. Preserve legacy log compatibility

**Choice:** Typed-event-ready rows may keep substring fallback as a diagnostic aid, but pass/fail decisions should prefer typed-event evidence where available.

**Rationale:** Existing logs and evidence remain readable while new evidence becomes structured.

### 4. Keep typed-event validation pure

**Choice:** Readiness classification and missing-event diagnostics should be computed from in-memory scenario specs and typed-event fixtures. File reads and receipt promotion remain shell responsibilities.

**Rationale:** The migration policy needs fast positive and negative tests without live clients.

## Risks / Trade-offs

- Typed-event names can drift from legacy milestone names; mitigate with parity fixtures for every migrated row.
- A row may appear ready while a server-side event is derived indirectly; mitigate by requiring derivation rules in the readiness contract.
- Tight gating can block urgent rows; mitigate with explicit waiver metadata rather than silent fallback.
