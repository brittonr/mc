## Why

The Minecraft Wiki-guided roadmap identifies furnace smelting as the first bounded survival/plugin seam, but follow-on implementation needs a dedicated behavior card with source scope, target data requirements, core/shell boundaries, tests, evidence, and stop conditions before code changes start.

## What Changes

- Add `docs/furnace-smelting-behavior-card.md` as the dedicated behavior card for a Java Edition 1.20.1 standard-furnace selected-row slice.
- Add a focused Rust checker that validates the card structure and runs positive and negative self-tests.
- Extend the `vanilla-composable-plugins` spec with requirements for the furnace smelting card and its validation evidence.

## Impact

- **Files**: `docs/furnace-smelting-behavior-card.md`, `tools/check_furnace_smelting_behavior_card.rs`, `cairn/specs/vanilla-composable-plugins/spec.md`, and archived Cairn package/evidence.
- **Testing**: Focused card checker with self-tests, Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, and evidence-manifest checks.
- **Non-claims**: No Valence plugin implementation, no vanilla parity claim, no DefaultPlugins membership change, no all-recipe breadth, no smoker/blast-furnace breadth, no public-server safety, and no production readiness.
