# Design: survival hunger health-cycle typed-event migration

## Scope

The migration is limited to the existing `survival-hunger-health-cycle` row. It changes the row from waiver-backed substring fallback to typed-event-ready pass/fail while keeping wrapper names, receipt fields, current-bundle row, and non-claims stable.

## Functional core

The pure validation core evaluates typed-event graphs from in-memory milestone evidence. This change extends the typed-event gate for `Scenario::SurvivalHungerHealthCycle` and adds ordering constraints for the row:

- pre-state health/food/saturation observation precedes consume start
- consume start precedes consume finish
- consume finish precedes inventory decrement observation
- inventory decrement precedes final health/food/saturation state
- server pre-state precedes server consume start
- server consume finish precedes server inventory and final state

Positive fixtures include complete client and server event graphs. Negative fixtures remove the final state event and reorder inventory before consume finish to verify fail-closed diagnostics.

## Imperative shell

Wrapper commands, dry-run invocation, receipt paths, and typed-event sidecar writing remain unchanged. The shell only reports the new typed-event contribution status once the pure gate accepts the hunger health-cycle event graph.

## Validation strategy

- Record baseline runner and manifest checks before edits.
- Update manifest readiness and regenerate generated surfaces.
- Run typed-event focused tests and the full runner test suite.
- Run scenario manifest self-test/check/generated-surface checks.
- Run the hunger health-cycle dry-run wrapper check and evidence manifest check.
- Run Cairn gates, task-evidence validation, archive, accepted-spec validation, and post-archive evidence checks.

## Non-claims

The row remains bounded to the configured Bread consumption health-cycle path. It does not claim all foods, exhaustion breadth, starvation loops, potion/effect interactions, offhand consumption, public-server safety, production readiness, or semantic equivalence.
