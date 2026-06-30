# Proposal: Remove Hyperion plugin Component derives

## Why

Some Hyperion mode/plugin structs derive `Component` even though they are Bevy plugins, not ECS components inserted on entities. This blurs plugin types with marker components and makes future mode markers harder to review.

## What Changes

- Inventory plugin structs that derive `Component` and verify whether any are actually inserted as ECS components.
- Remove `Component` derives from pure plugin marker structs.
- Introduce or keep separate marker components for real ECS state where needed.
- Add compile and focused tests proving plugin installation and marker behavior remain unchanged.
- Document any intentional plugin-as-component exception if one exists.

## Impact

- **Files**: `hyperion/events/bedwars/src/lib.rs`, possible plugin modules with `#[derive(Component)]`, docs/evidence inventory and checks.
- **Testing**: focused Hyperion compile/tests, marker-component checks where touched, negative tests for accidental plugin-as-component assumptions, Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest checks.
- **Non-claims**: this is a cleanup/refinement; it does not alter plugin composition semantics or mode selection by itself.
