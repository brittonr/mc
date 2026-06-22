# Design: survival chest persistence typed-event migration

## Scope

The migration is limited to the existing `survival-chest-persistence` row. It changes the row from waiver-backed substring fallback to typed-event-ready pass/fail. Existing milestones, wrapper names, receipt fields, current-bundle row, and non-claim language remain stable except for generated migration-state wording.

## Functional core

The pure validation core evaluates typed-event graphs from in-memory milestone evidence. This change extends the typed-event gate for `Scenario::SurvivalChestPersistence` and adds ordering constraints for the row:

- client chest-open observation precedes store observation
- store observation precedes close observation
- close observation precedes reconnect or second-session start
- second-session reopen precedes persisted-slot observation
- server chest-open precedes server store
- server store precedes server close
- server close precedes server reopen
- server reopen precedes server persisted-state confirmation

Positive fixtures include the complete client/server event graph. Negative fixtures remove a required persisted-state event and reorder the reconnect/reopen phases to verify fail-closed diagnostics.

## Imperative shell

The shell remains unchanged: wrapper commands, dry-run invocation, receipt paths, and typed-event sidecar writing keep the existing behavior. The only shell-visible effect is that successful live receipts for this row record typed-event pass/fail contribution once client/server evidence is present.

## Validation strategy

- Record baseline runner and manifest checks before edits.
- Update manifest readiness and regenerate generated surfaces.
- Run typed-event focused tests and the full runner test suite.
- Run scenario manifest self-test/check/generated-surface checks.
- Run the survival chest dry-run wrapper check and evidence manifest check.
- Run Cairn gates, task-evidence validation, archive, accepted-spec validation, and post-archive evidence checks.

## Non-claims

The row remains bounded to one chest persistence interaction with paired Paper/reference and Valence evidence. It does not claim all-container behavior, all item transfers, restart/world persistence breadth, crash recovery, public-server safety, production readiness, or semantic equivalence.
