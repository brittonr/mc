# Design: survival furnace smelting breadth typed-event migration

## Scope

The migration is limited to the existing `survival-furnace-smelting-breadth` row. It changes the row from waiver-backed substring fallback to typed-event-ready pass/fail while keeping wrapper names, receipt fields, current-bundle row, and non-claims stable.

## Functional core

The pure validation core evaluates typed-event graphs from in-memory milestone evidence. This change extends the typed-event gate for `Scenario::SurvivalFurnaceSmeltingBreadth` and adds ordering constraints for the row:

- furnace-open precedes valid input placement
- valid input placement precedes valid fuel placement
- valid fuel placement precedes burn-progress observation
- burn-progress observation precedes output-available observation
- output-available observation precedes output collection
- collection precedes invalid-fuel attempt
- invalid-fuel attempt precedes invalid-fuel rejection and final state observation

Positive fixtures include complete client and server event graphs. Negative fixtures remove the invalid-fuel rejection and reorder output collection before output availability to verify fail-closed diagnostics.

## Imperative shell

Wrapper commands, dry-run invocation, receipt paths, and typed-event sidecar writing remain unchanged. The shell only reports the new typed-event contribution status once the pure gate accepts the smelting-breadth event graph.

## Validation strategy

- Record baseline runner and manifest checks before edits.
- Update manifest readiness and regenerate generated surfaces.
- Run typed-event focused tests and the full runner test suite.
- Run scenario manifest self-test/check/generated-surface checks.
- Run the furnace smelting breadth dry-run wrapper check and evidence manifest check.
- Run Cairn gates, task-evidence validation, archive, accepted-spec validation, and post-archive evidence checks.

## Non-claims

The row remains bounded to the configured valid smelt and invalid-fuel rejection. It does not claim all smelting recipes, all fuels, long-running timing parity, hopper automation, furnace minecarts, public-server safety, production readiness, or semantic equivalence.
