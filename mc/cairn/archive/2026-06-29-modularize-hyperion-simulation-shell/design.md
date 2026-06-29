# Design: Hyperion simulation shell modularization

## Context

Hyperion simulation code is independent nested-repo engine work. Parent Cairn artifacts may plan it, but implementation and validation must be Hyperion-local and respect integration boundaries.

## Decisions

### 1. Keep scope Hyperion-owned

**Choice:** Treat the simulation shell split as Hyperion-owned unless a separate integration Cairn classifies specific sources for Valence use.

**Rationale:** Parent planning must not imply direct adoption into Valence.

### 2. Split orchestration and decisions

**Choice:** Separate system registration, state orchestration, packet-facing adapters, domain coordination, diagnostics, and pure simulation decisions.

**Rationale:** Simulation orchestration and state transitions need different tests.

### 3. Preserve performance and runtime shells

**Choice:** ECS mutation, packet emission, network/proxy integration, tracing, scheduling, and hot-path performance assumptions remain explicit shell concerns.

**Rationale:** Hyperion performance-sensitive behavior must remain stable.
