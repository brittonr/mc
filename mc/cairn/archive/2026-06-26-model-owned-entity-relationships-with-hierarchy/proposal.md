# Proposal: Model owned entity relationships with Bevy hierarchy

## Why

Some Valence examples and systems represent relationships between entities: GUI menus and backing inventories, fixture-spawned mobs and drops, visual companions/clones, and other ownership-like pairs. `valence_advancement` already uses Bevy hierarchy where tree structure is natural. A focused pass can identify other relationships that benefit from `Parent`/`Children` hierarchy or explicit relationship components, while avoiding hierarchy where a map or layer index is the better model.

## What Changes

- Inventory selected entity relationships, owners, child lifecycles, traversal needs, cleanup behavior, and evidence impact.
- Classify relationships as hierarchy-suitable, explicit relationship component, resource/index, external identity, or intentionally independent entities.
- Adopt Bevy hierarchy or relationship components only where ownership/traversal semantics are real and documented.
- Preserve cleanup behavior, fixture milestones, and non-claim boundaries.
- Add positive relationship lifecycle/traversal tests and negative stale parent, orphan child, duplicate parent, cycle/invalid relationship, and plugin-disabled tests.

## Impact

- **Files**: selected examples, `valence_inventory::gui`, possibly relationship docs/tests, and evidence under `docs/evidence/`.
- **Testing**: focused relationship lifecycle tests, selected example/compatibility rails when behavior changes, schedule hygiene if plugins/systems change, Cairn gates, Cairn validation, and task-evidence validation.
- **Non-claims**: this does not require all entity references to use hierarchy, add automatic recursive despawn everywhere, or change Valence layer/entity ID semantics.
