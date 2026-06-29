# Design: Valence chunk layer modularization

## Context

Chunk layer code is a storage, messaging, and packet targeting boundary. The refactor should make targeting and entry decisions explicit while preserving public API and schedule behavior.

## Decisions

### 1. Split storage, targeting, writers, and systems

**Choice:** Create modules for chunk storage, entry APIs, view/radius targeting, packet writer adapters, local messages, layer trait integration, and Bevy update systems.

**Rationale:** These responsibilities have separate invariants.

### 2. Extract pure targeting decisions

**Choice:** View membership, radius selection, exception filtering, and entry state decisions should be pure over explicit inputs.

**Rationale:** Packet targeting behavior can be tested without live clients.

### 3. Preserve update ordering

**Choice:** Existing pre-client/post-client update behavior remains stable unless another schedule Cairn changes it.

**Rationale:** Chunk updates are schedule-sensitive.
