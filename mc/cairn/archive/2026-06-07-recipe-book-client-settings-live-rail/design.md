## Context

Crafting-table parity evidence does not cover recipe-book client settings. The live proof must be a separate row with one deterministic settings transition and one server correlation metric.

## Goals / Non-Goals

Goals:
- Define a deterministic recipe-book settings contract for one actor and one configured settings transition.
- Add or select an isolated client/runner path that sends `RecipeBookDataC2SPacket` and records Valence correlation.
- Promote only `recipe-book-client-settings` when checker-backed live evidence passes.

Non-goals:
- Proving recipe-book UI behavior, recipe discovery, all recipe categories, all recipes, crafting breadth, public-server safety, production readiness, or full protocol 763 compatibility.

## Design

1. Define pure contract data for actor, settings fields, packet row, backend/client path, expected server correlation, and non-claims.
2. Add a rail or deterministic fixture that toggles the configured settings without changing crafting-table row semantics.
3. Normalize live evidence into the shared targeted-packet KV schema plus row-specific recipe-book metrics.
4. Extend or reuse checker coverage so valid recipe-book settings evidence passes and wrong fields, missing client action, missing server correlation, stale digest, and broad recipe/crafting overclaims fail closed.
5. Update matrix/current-bundle/packet-inventory docs only when evidence passes.

## Risks

- Stevenarella may not currently expose a recipe-book settings action; if so, record a blocker or split out a client-driver Cairn.
- The crafting fixture may accidentally be used as proof. The checker must require settings-specific action/correlation.

## Validation

- Run baseline targeted packet, matrix, and current-bundle checks before runner edits.
- Run focused runner tests or dry-runs for the recipe-book settings rail.
- Run positive and negative targeted-packet live-evidence checker tests.
- Run evidence-manifest/task-evidence checks plus Cairn gates and validation.
