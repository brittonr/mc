# Proposal: Replace entity-keyed state with components where appropriate

## Why

Several Valence examples and fixture shells keep runtime state in `HashMap<Entity, ...>`, `HashSet<Entity>`, or identifier-keyed resources even when the state's lifecycle is owned by a specific ECS entity. That can create stale entries after despawn, duplicate ownership paths, and harder-to-test cleanup behavior. Bevy components should hold entity-owned state; resources should hold global policy or indexes only.

## What Changes

- Inventory entity-keyed and player-keyed runtime state in selected Valence examples and compatibility fixtures.
- Classify each state bucket as entity-owned component state, global resource state, deterministic core state, or intentional index/cache.
- Migrate entity-owned state to components with explicit add/remove lifecycle systems.
- Keep global policy, team-indexed tables, and pure fixture state out of components when entity ownership is not appropriate.
- Add positive lifecycle tests and negative stale-entity/despawn/reconnect tests.

## Impact

- **Files**: selected Valence examples, fixture-core adapters, component definitions, tests, docs/evidence after implementation.
- **Testing**: component lifecycle tests, despawn cleanup tests, reconnect/stale-entity negative tests, selected compatibility rails if fixture state changes, Cairn gates, and Cairn validation.
- **Non-claims**: this does not require every map to become components; indexes, team tables, and pure core state can remain resources when justified.
