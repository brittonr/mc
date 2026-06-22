# Design: survival furnace persistence typed-event migration

## Scope

The migration is limited to the existing `survival-furnace-persistence` row. It changes the row from waiver-backed substring fallback to typed-event-ready pass/fail while keeping wrapper names, receipt fields, current-bundle row, and non-claims stable.

## Functional core

The pure validation core evaluates typed-event graphs from in-memory milestone evidence. This change extends the typed-event gate for `Scenario::SurvivalFurnacePersistence` and adds ordering constraints for the row:

- furnace-open precedes input placement
- input placement precedes fuel placement
- fuel placement precedes burn-progress observation
- burn-progress observation precedes output-available observation
- output-available observation precedes output collection
- output collection precedes reconnect or second-session start
- second-session reopen precedes persisted-state confirmation

Positive fixtures include complete client and server event graphs. Negative fixtures remove a required output/state event and reorder reconnect before collection to verify fail-closed diagnostics.

## Imperative shell

Wrapper commands, dry-run invocation, receipt paths, and typed-event sidecar writing remain unchanged. The shell only reports the new typed-event contribution status once the pure gate accepts the furnace event graph.

## Validation strategy

- Record baseline runner and manifest checks before edits.
- Update manifest readiness and regenerate generated surfaces.
- Run typed-event focused tests and the full runner test suite.
- Run scenario manifest self-test/check/generated-surface checks.
- Run the furnace persistence dry-run wrapper check and evidence manifest check.
- Run Cairn gates, task-evidence validation, archive, accepted-spec validation, and post-archive evidence checks.

## Non-claims

The row remains bounded to one furnace persistence interaction with paired Paper/reference and Valence evidence. It does not claim all smelting recipes, fuel exhaustion breadth, hopper automation, furnace minecarts, public-server safety, production readiness, or semantic equivalence.
