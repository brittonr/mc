# Design: survival crafting recipe breadth typed-event migration

## Scope

The migration is limited to the existing `survival-crafting-recipe-breadth` row. It changes the row from waiver-backed substring fallback to typed-event-ready pass/fail while keeping wrapper names, receipt fields, current-bundle row, and non-claims stable.

## Functional core

The pure validation core evaluates typed-event graphs from in-memory milestone evidence. This change extends the typed-event gate for `Scenario::SurvivalCraftingRecipeBreadth` and adds ordering constraints for the row:

- shaped-recipe input precedes shaped result observation
- shaped result observation precedes shaped collection
- shapeless-recipe input/result follows the shaped recipe checkpoint
- grid clear follows shapeless result observation
- invalid-input attempt follows the valid recipe and grid-clear checkpoints
- invalid-input attempt precedes invalid rejection and final state observation

Positive fixtures include complete client and server event graphs. Negative fixtures remove the invalid-rejection event and reorder shapeless/grid-clear phases to verify fail-closed diagnostics.

## Imperative shell

Wrapper commands, dry-run invocation, receipt paths, and typed-event sidecar writing remain unchanged. The shell only reports the new typed-event contribution status once the pure gate accepts the recipe-breadth event graph.

## Validation strategy

- Record baseline runner and manifest checks before edits.
- Update manifest readiness and regenerate generated surfaces.
- Run typed-event focused tests and the full runner test suite.
- Run scenario manifest self-test/check/generated-surface checks.
- Run the crafting recipe breadth dry-run wrapper check and evidence manifest check.
- Run Cairn gates, task-evidence validation, archive, accepted-spec validation, and post-archive evidence checks.

## Non-claims

The row remains bounded to the configured shaped recipe, shapeless recipe, invalid-input rejection, and primary-click collection path. It does not claim all recipes, recipe-book UI, arbitrary collection modes, shift-click/drag/split semantics, public-server safety, production readiness, or semantic equivalence.
