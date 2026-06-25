# Design: Move example worldgen work to Bevy task orchestration

## Context

The terrain example's raw threads are simple, but they are not the best Bevy example for server-side background work. Bevy task orchestration would better demonstrate task resources, polling systems, and a pure generation core.

## Decisions

### 1. Keep generation pure

**Choice:** Chunk generation takes explicit seed, position, and generation settings, then returns an `UnloadedChunk` or typed error without ECS access, thread spawning, logging, or global state.

**Rationale:** Generation logic should be unit-testable without a Bevy app.

### 2. Bevy shell owns task lifecycle

**Choice:** Bevy systems enqueue chunk work, poll task completion, validate current view/request state, and apply completed chunks to the world.

**Rationale:** ECS should own world mutation and client-view coherence.

### 3. Use Bevy task pools or a documented task resource

**Choice:** Prefer Bevy task-pool integration when the dependency surface is acceptable. If a Valence-owned task resource is used instead, document why it is equivalent for the example.

**Rationale:** The goal is Bevy-shaped orchestration, not a new runtime.

### 4. Stale work is safe

**Choice:** Completed tasks validate that the chunk is still requested before mutating the layer.

**Rationale:** Client views can change while generation is in flight.

## Risks / Trade-offs

- Adding a task crate dependency may be unnecessary for a simple example; document dependency impact before adoption.
- Task cancellation may be cooperative or best-effort; tests should prove stale results are ignored even if tasks finish.
- Bevy task examples must not imply production worldgen, persistence, or backpressure guarantees.
