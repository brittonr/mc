# Design: Extract worldgen task orchestration plugin

## Context

`servers/valence/examples/terrain.rs` is already shaped around Bevy background work: request collection happens in systems, chunk generation runs on `bevy_tasks`, and completed work is validated before mutating a `ChunkLayer`. The remaining opportunity is to turn the task orchestration shell into a reusable contract while keeping generation decisions pure and explicit.

## Decisions

### 1. Inventory task ownership before extraction

**Choice:** Record pending request state, task inputs, task pool ownership, polling order, stale handling, cancellation, completion handling, and shutdown assumptions.

**Rationale:** Background task orchestration has correctness hazards that must be visible before it becomes reusable.

### 2. Generation core remains pure

**Choice:** Chunk generation receives explicit inputs and returns a typed result or typed error without ECS, global state, clocks, channels, or logging.

**Rationale:** The generator should be testable without a Bevy world; Bevy owns only task orchestration and mutation application.

### 3. Shell owns task lifecycle

**Choice:** The reusable shell manages queued/generating/completed states, spawns through Bevy task-pool integration, polls completions, and emits typed completion/error events or mutation requests.

**Rationale:** A shared task lifecycle reduces per-example duplication while preserving explicit policy inputs.

### 4. Stale completions fail closed

**Choice:** Completed work must validate current request ownership, view state, already-loaded state, and shutdown state before mutating chunks.

**Rationale:** Background work can complete after demand changes; stale results must not insert unrequested chunks.

## Risks / Trade-offs

- Generalizing too much can turn an example helper into a premature production API; keep the first shell narrow and opt-in.
- Task cancellation in `bevy_tasks` may be cooperative or best-effort; stale-result validation remains mandatory.
- Introducing typed completion events changes schedule shape; schedule hygiene and disabled-plugin tests are required.
