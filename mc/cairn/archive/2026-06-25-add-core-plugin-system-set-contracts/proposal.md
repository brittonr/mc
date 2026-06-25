# Proposal: Add core plugin SystemSet contracts

## Why

Several Valence core plugins register multiple systems with implicit tuple ordering or narrow local constraints. Inventory SystemSet work is scoped separately; other core plugins such as command, equipment, advancement, scoreboard, weather, world border, and boss bar can also benefit from stable Bevy `SystemSet` contracts. Named sets would make downstream ordering clearer without changing default behavior.

## What Changes

- Inventory selected core plugin systems, schedules, ordering constraints, resources, events, feature/default membership, and downstream ordering assumptions.
- Define minimal public or crate-visible `SystemSet` contracts for selected plugin phases.
- Move existing systems into named sets while preserving semantics, feature gates, event names, resources, and default plugin membership.
- Document private ordering points that intentionally remain anonymous.
- Add positive schedule/plugin smoke tests and negative disabled-plugin or ordering-regression tests.

## Impact

- **Files**: selected crates under `servers/valence/crates/`, especially command, equipment, advancement, scoreboard, weather, world border, and boss bar modules; schedule evidence under `docs/evidence/`.
- **Testing**: focused crate tests, Valence schedule hygiene, selected examples if touched, Cairn gates, Cairn validation, and task-evidence validation.
- **Non-claims**: this does not change gameplay semantics, feature flags, default plugin membership, or public protocol compatibility.
