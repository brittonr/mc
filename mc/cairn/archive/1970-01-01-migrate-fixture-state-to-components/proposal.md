# Proposal: Migrate selected fixture state to components

## Why

Compatibility examples keep some runtime state in resources or ad hoc collections even when the state belongs to a live client, fixture entity, container, mob, drop, or visual companion. Component ownership can make lifecycles clearer, reduce stale entity references, and align fixture shells with Bevy ECS while preserving pure fixture decisions and evidence boundaries.

## What Changes

- Inventory selected fixture and example state keyed by client, entity, username, UUID, container, mob/drop, or visual companion.
- Classify state as entity-owned component data, global resource data, pure-core state, index/cache state, or external identity state.
- Migrate selected entity-owned state to Bevy components or bundles while keeping true globals as documented resources.
- Preserve fixture milestones, env/CLI contracts, and compatibility non-claims.
- Add positive lifecycle tests and negative stale-entity, despawn, duplicate ownership, reconnect, and plugin-disabled tests.

## Impact

- **Files**: selected state in `servers/valence/examples/ctf.rs`, `servers/valence/examples/survival_compat.rs`, `servers/valence/crates/valence_inventory/src/gui.rs`, tests, and evidence docs.
- **Testing**: focused example/crate tests, selected mc-compat rails when fixture behavior changes, schedule hygiene if plugin wiring changes, Cairn gates, Cairn validation, and task-evidence validation.
- **Non-claims**: this does not add new gameplay semantics, broaden compatibility claims, or remove intentionally global fixture resources.
