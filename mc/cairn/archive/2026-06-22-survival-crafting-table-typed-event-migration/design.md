# Design: survival crafting-table typed-event migration

## Scope

The migration is limited to the existing `survival-crafting-table` row. It changes the row from waiver-backed substring fallback to typed-event-ready pass/fail. Existing milestones, wrapper names, receipt fields, current-bundle row, and non-claim language remain stable except for generated migration-state wording.

## Functional core

The pure validation core already evaluates typed-event graphs from in-memory milestone evidence. This change extends the typed-event gate for `Scenario::SurvivalCraftingTable` and adds ordering constraints for the row:

- client table-open observation precedes input A
- input A precedes input B
- input B precedes result observation
- result observation precedes result collection
- result collection precedes inventory update
- server table-open precedes input A
- server input A precedes input B
- server input B precedes result
- server result precedes collect

Positive fixtures include the complete client/server event graph. Negative fixtures remove a required crafting event and reorder server phases to verify fail-closed diagnostics.

## Imperative shell

The shell remains unchanged: wrapper commands, dry-run invocation, receipt paths, and typed-event sidecar writing keep the existing behavior. The only shell-visible effect is that successful live receipts for this row record typed-event pass/fail contribution once client/server evidence is present.

## Validation strategy

- Record baseline runner and manifest checks before edits.
- Update manifest readiness and regenerate generated surfaces.
- Run typed-event focused tests and the full runner test suite.
- Run scenario manifest self-test/check/generated-surface checks.
- Run the survival crafting-table dry-run wrapper check and evidence manifest check.
- Run Cairn gates, task-evidence validation, archive, accepted-spec validation, and post-archive evidence checks.

## Non-claims

The row remains bounded to one crafting-table recipe interaction with Valence server correlation. It does not claim broad crafting recipe support, recipe-book UI, all containers, arbitrary inventory actions, public-server safety, production readiness, or semantic equivalence.
