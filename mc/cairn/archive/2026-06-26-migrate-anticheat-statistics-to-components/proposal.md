# Proposal: Migrate anticheat statistics to components

## Why

`AnticheatStatisticsPlugin` is an optional advisory plugin, but its per-player samples are stored in a resource map keyed by `Entity`. State whose lifecycle is owned by a live client entity is a strong candidate for Bevy components. Moving per-player statistics to client-owned components would make despawn cleanup automatic, reduce stale entity map risk, and align the plugin with Valence's ECS ownership model.

## What Changes

- Inventory `AnticheatStatisticsState`, per-player metric windows, packet/movement event readers, current cleanup behavior, and tests.
- Classify which state is entity-owned component data and which state remains a global resource, such as plugin-local tick/config.
- Add per-client anticheat statistics components and migrate sampling systems to query/mutate component state.
- Preserve the plugin's advisory-only behavior, event shape, explicit opt-in registration, and no-enforcement non-claim.
- Add positive lifecycle tests and negative stale-entity, despawn, duplicate ownership, reconnect, and plugin-disabled tests.

## Impact

- **Files**: `servers/valence/crates/valence_server/src/anticheat.rs`, focused tests, evidence under `docs/evidence/`.
- **Testing**: focused `valence_server` anticheat tests, schedule hygiene if plugin wiring changes, Cairn gates, Cairn validation, and task-evidence validation.
- **Non-claims**: this does not add anticheat enforcement, production cheat detection, public-server safety guarantees, or default plugin membership.
