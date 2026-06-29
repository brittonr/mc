# Design: Valence entity core extraction

## Context

Valence entity behavior combines pure math/state transitions with ECS-facing components and packet-facing tracked data. The extraction should prioritize deterministic domain logic before moving public APIs.

## Decisions

### 1. Start with attribute and status-effect cores

**Choice:** Extract attribute modifier math, effective value calculation, status-effect insertion/expiry, and tracked-data update decisions into pure functions.

**Rationale:** These have clear inputs, outputs, and boundary cases.

### 2. Keep ECS and packet shells separate

**Choice:** Component mutation, Bevy event emission, packet composition, and schedule wiring stay in shells.

**Rationale:** The core should not require a Bevy app to test.

### 3. Preserve public API compatibility

**Choice:** Existing public entity types and component APIs remain stable through module re-exports or adapters.

**Rationale:** Valence examples and downstream crates should not need a broad migration.
