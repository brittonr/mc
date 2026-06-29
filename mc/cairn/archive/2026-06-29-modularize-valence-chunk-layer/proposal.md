# Proposal: Modularize Valence chunk layer

## Why

`servers/valence/crates/valence_server/src/layer/chunk.rs` combines chunk layer storage, local messaging, packet writer variants, entry APIs, layer trait implementation, and update systems. Chunk layer behavior is core server state and should separate pure view/radius/entry decisions from Bevy shell systems and packet writes.

## What Changes

- Split chunk layer code into modules for storage, entry API, view/radius selection, packet writer adapters, local messages, layer trait integration, and update systems.
- Extract pure decisions for view membership, radius targeting, exception filtering, entry state transitions, and update-plan selection.
- Keep packet writes, Bevy queries, layer mutation, and schedule systems in shells.
- Preserve public chunk APIs, packet targeting behavior, update ordering, layer semantics, and non-claims.

## Impact

- **Files**: `servers/valence/crates/valence_server/src/layer/chunk.rs`, chunk submodules, focused tests, schedule checks where needed, and Cairn artifacts.
- **Testing**: baseline chunk/layer tests, positive and negative chunk-core tests, affected schedule checks, Cairn gates, and Cairn validation.
- **Non-claims**: chunk-layer architecture only; no new chunk protocol or gameplay claim is promoted.
