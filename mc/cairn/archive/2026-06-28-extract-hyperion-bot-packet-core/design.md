# Design: Hyperion bot packet core

## Context

The rust-mc-bot tool supports Hyperion load/testing workflows. Packet helper logic should be independently testable and should not blur into Valence or mc-compat claims.

## Decisions

### 1. Keep ownership Hyperion-local

**Choice:** Treat bot packet extraction as Hyperion tool work, not Valence or parent runner behavior.

**Rationale:** Bot helpers may be useful references but are not adopted by default.

### 2. Extract pure packet helpers

**Choice:** Packet construction, packet classification, byte-shape validation, and protocol assumptions should be pure over explicit inputs.

**Rationale:** Packet utilities can be tested without a bot connection.

### 3. Keep bot runtime effects in shells

**Choice:** Socket IO, connection state, async tasks, sleeps/timing, and logging remain in bot shells.

**Rationale:** Deterministic tests should not need live network state.
