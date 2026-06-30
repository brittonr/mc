# Proposal: Expose Hyperion gameplay plugin group

## Why

Hyperion's Bedwars event crate already composes shared mechanics through `CommonGameplayPlugin`, but that plugin is private and bundled inside each selected mode. Users cannot discover, disable, replace, or test shared mechanics as a public Bevy composition surface.

## What Changes

- Inventory the current `CommonGameplayPlugin` plugin list, command registration, implicit dependencies, and default behavior.
- Introduce a public shared gameplay `PluginGroup` surface for the default mechanics currently installed by `CommonGameplayPlugin`.
- Re-export individual gameplay feature plugins through stable public paths or document why a feature remains private.
- Preserve the existing default Bedwars/Dayz/HardcoreFactions app behavior while making the shared mechanics addressable independently.
- Add positive default-group tests and negative disable/replace/dependency tests.

## Impact

- **Files**: `hyperion/events/bedwars/src/lib.rs`, `hyperion/events/bedwars/src/plugin/*`, possible `hyperion/events/bedwars/src/gameplay.rs`, docs/evidence for the inventory and checks.
- **Testing**: focused Hyperion plugin composition tests from `hyperion/`, compile checks for public re-exports, Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest checks before archive.
- **Non-claims**: this does not move code out of `events/bedwars`, make modes stackable, or change gameplay semantics; those are scoped to separate Cairns.
