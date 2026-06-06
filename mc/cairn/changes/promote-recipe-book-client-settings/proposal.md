# Proposal: Promote recipe-book client settings evidence

## Why

Crafting-table evidence covers one recipe execution path, but recipe-book client setting packets remain unpromoted. A bounded `RecipeBookDataC2SPacket` row would cover a visible serverbound packet gap without claiming recipe-book UI or all recipes.

## What Changes

- Add one bounded recipe-book client settings row for a configured recipe-book state update.
- Require client action/packet milestone and Valence server correlation for the received settings.
- Promote only the configured recipe-book settings row, keeping recipe-book UI behavior, all recipe categories, recipe discovery, all recipes, full crafting coverage, full protocol coverage, and production readiness as non-claims.

## Impact

- **Files**: Stevenarella probe, Valence fixture instrumentation, runner metadata, packet inventory/current bundle docs, checker, evidence artifacts, and Cairn specs/tasks.
- **Testing**: positive/negative checker fixtures, focused scenario tests, packet inventory/current-bundle checks, evidence manifests, task-evidence gate, and Cairn validation.
