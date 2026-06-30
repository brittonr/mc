# Proposal: Add Hyperion mode markers and run conditions

## Why

Current mode setup primarily inserts global `ActiveGameType` and player components such as `Team` or `DayzSurvivor`. As shared mechanics become independently composable, mode-specific systems need explicit markers and run conditions so they mutate only entities and worlds they own.

## What Changes

- Inventory current mode-specific components, observers, systems, and global active-mode checks.
- Define mode marker components/resources and run-condition helpers for mode-owned systems.
- Refactor mode-specific systems to filter by markers or run conditions instead of relying only on global `ActiveGameType`.
- Add cleanup/teardown expectations for marker-owned state.
- Add positive scoped-mutation tests and negative wrong-mode/disabled-plugin tests.

## Impact

- **Files**: `hyperion/events/bedwars/src/lib.rs`, mode-specific plugin modules, possible marker/run-condition helpers, docs/evidence for mode-scope inventory and checks.
- **Testing**: focused Hyperion ECS tests, disabled-plugin and wrong-mode tests, marker cleanup tests, Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest checks.
- **Non-claims**: this does not implement concurrent exclusive modes; markers and run conditions make ownership explicit within the current composition model.
