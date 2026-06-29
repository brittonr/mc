# Proposal: Extract Valence entity attribute core

## Why

Valence entity code spreads hitboxes, active status effects, attributes, tracked data, flags, and queries across several chunky modules. Attribute and status-effect behavior is domain logic that should be testable independently of Bevy ECS shells and packet composition.

## What Changes

- Extract pure cores for attribute math, status-effect application/expiry, tracked-data updates, hitbox calculations, flag changes, and query predicates where practical.
- Keep Bevy ECS component mutation, event emission, packet composition, and schedule wiring in shells.
- Preserve existing public entity APIs, attribute/status semantics, tracked-data encoding behavior, hitbox behavior, flags, and non-claims.
- Add positive and negative tests for extracted entity cores and boundary cases.

## Impact

- **Files**: `servers/valence/crates/valence_entity/src/*`, possible `valence_server` call sites, focused tests, and Cairn artifacts.
- **Testing**: baseline Valence entity tests, positive and negative entity-core tests, affected workspace checks, Cairn gates, and Cairn validation.
- **Non-claims**: entity architecture only; no new gameplay or compatibility claim is promoted.
