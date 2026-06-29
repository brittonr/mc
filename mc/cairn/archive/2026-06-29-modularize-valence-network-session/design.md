# Design: Valence network session modules

## Context

Valence network code owns both public server behavior and low-level IO. The modularization should keep public APIs stable while isolating deterministic session decisions from async runtime effects.

## Decisions

### 1. Split by protocol/session phase

**Choice:** Create owners for connect/listen orchestration, status and legacy ping, login/session negotiation, packet IO, profile/cache adapters, and pure session state.

**Rationale:** These phases have distinct invariants and failure modes.

### 2. Extract pure session decisions

**Choice:** State transitions, validation, compression decisions, status response composition, disconnect classification, and legacy ping classification should be pure over explicit inputs.

**Rationale:** Session behavior can be tested without sockets or async tasks.

### 3. Preserve public behavior

**Choice:** Public APIs, feature flags, status output, legacy ping handling, profile cache behavior, and packet semantics remain stable.

**Rationale:** This is an internal architecture split.
