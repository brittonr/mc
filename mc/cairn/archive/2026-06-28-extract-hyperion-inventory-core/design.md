# Design: Hyperion inventory core extraction

## Context

Hyperion is an independent nested repo with its own workflow. Parent Cairn artifacts can plan the work, but implementation and validation must run from Hyperion and must not blur Valence integration boundaries.

## Decisions

### 1. Keep ownership Hyperion-local

**Choice:** Treat inventory extraction as Hyperion-owned engine work unless another Cairn classifies a specific source as port/reference/adopt for Valence.

**Rationale:** The parent repo must not accidentally merge Hyperion behavior into Valence core.

### 2. Extract pure inventory transitions

**Choice:** Inventory state transitions, slot validation, transaction outcomes, and packet-facing summaries should be pure over explicit inputs.

**Rationale:** Inventory behavior can be tested without Bevy runtime, network IO, or proxy state.

### 3. Keep runtime shells thin

**Choice:** ECS mutation, packet emission, runtime scheduling, and tracing remain in Hyperion shells.

**Rationale:** Performance and side-effect boundaries stay explicit.
