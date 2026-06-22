# Design: survival break/place typed-event migration

## Scope

The migration is limited to the existing `survival-break-place-pickup` row. It changes the row from waiver-backed substring fallback to typed-event-ready pass/fail. Existing milestones, wrapper names, receipt fields, current-bundle row, and non-claim language remain stable except for generated migration-state wording.

## Functional core

The pure validation core already evaluates typed-event graphs from in-memory milestone evidence. This change extends the typed-event gate for `Scenario::SurvivalBreakPlacePickup` and adds ordering constraints for the row:

- client break intent precedes break update
- break update precedes pickup observation
- pickup precedes place intent
- place intent precedes place update
- server survival join precedes break
- server break precedes pickup
- server pickup precedes place

Positive fixtures include the complete client/server event graph. Negative fixtures remove a required survival event and reorder server phases to verify fail-closed diagnostics.

## Imperative shell

The shell remains unchanged: wrapper commands, dry-run invocation, receipt paths, and typed-event sidecar writing keep the existing behavior. The only shell-visible effect is that successful live receipts for this row record typed-event pass/fail contribution once client/server evidence is present.

## Validation strategy

- Record baseline runner and manifest checks before edits.
- Update manifest readiness and regenerate generated surfaces.
- Run typed-event focused tests and the full runner test suite.
- Run scenario manifest self-test/check/generated-surface checks.
- Run the survival break/place dry-run wrapper check and evidence manifest check.
- Run Cairn gates, task-evidence validation, archive, accepted-spec validation, and post-archive evidence checks.

## Non-claims

The row remains bounded to one fixed-coordinate survival break/place/pickup scenario with Valence server correlation. It does not claim broad survival parity, all block interactions, long-term world durability, arbitrary inventory interactions, public-server safety, production readiness, or semantic equivalence.
