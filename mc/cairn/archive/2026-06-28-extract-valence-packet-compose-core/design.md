# Design: Valence packet compose core

## Context

Packet composition translates internal state into protocol-facing packets. The refactor should make packet-plan decisions testable while preserving output fields and ordering.

## Decisions

### 1. Extract packet-plan helpers

**Choice:** Move deterministic packet field selection, ordering facts, and packet family plans into pure functions.

**Rationale:** Protocol-facing behavior needs direct fixtures.

### 2. Keep writes in shells

**Choice:** Client/layer access, actual packet writes, Bevy systems, and logging remain in shells.

**Rationale:** Pure composition should not require live clients.

### 3. Preserve protocol assumptions

**Choice:** Existing packet shapes, field values, and ordering remain stable unless another packet-boundary Cairn changes them.

**Rationale:** This is not a protocol expansion.
