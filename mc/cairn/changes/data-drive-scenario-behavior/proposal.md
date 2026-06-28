# Proposal: Data-drive mc-compat scenario behavior

## Why

Scenario behavior is spread across static scenario specs, large match statements, typed-event graph edges, runner env wiring, and receipt selection logic. Adding or changing one scenario currently risks missing one of those surfaces.

## What Changes

- Promote scenario behavior facts into explicit metadata attached to `ScenarioSpec` or generated scenario surfaces.
- Represent run strategy, client/server milestone families, typed-event graph edges, env intents, non-claims, and evidence selectors as data where practical.
- Keep specialized code paths only for behavior that cannot be represented declaratively.
- Add positive and negative metadata validation so incomplete scenario rows fail before runtime.

## Impact

- **Files**: `compat/runner/src/scenario_core.rs`, generated scenario surfaces, `evidence_core.rs`, scenario behavior adapters, tests, docs, and Cairn artifacts.
- **Testing**: scenario-spec validation, generated surface freshness checks, positive representative scenario fixtures, negative malformed metadata fixtures, runner tests, Cairn gates, and Cairn validation.
- **Non-claims**: scenario metadata refactor only; it does not add new compatibility claims.
