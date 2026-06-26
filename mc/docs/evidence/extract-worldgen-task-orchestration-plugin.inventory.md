# extract-worldgen-task-orchestration-plugin inventory

## Question

What current terrain-example background work is being extracted behind an opt-in worldgen task orchestration plugin/resource shell?

## Current implementation before plugin extraction

- Owner subtree: `servers/valence`.
- Primary file: `servers/valence/examples/terrain.rs`.
- Request collection: `update_client_views` checks the single `ChunkLayer`, computes per-position priority from the current `View`, and calls `queue_chunk_request` for newly visible chunks that are not already loaded.
- Pending request state: `GameState::pending: HashMap<ChunkPos, PendingChunkRequest>` stores `Queued(Priority)` requests and `Generating(ChunkGenerationTask)` requests in the same terrain example state resource as generator noise/settings.
- Task inputs: `spawn_chunk_generation_task` builds `ChunkGenerationInput { pos, settings }` and captures `Arc<ChunkGenerationNoise>` explicitly. The generator receives no ECS queries, commands, resources, clocks, channels, logging, or global state.
- Task pool ownership: terrain tasks use `bevy_tasks::AsyncComputeTaskPool::get_or_init(TaskPool::new)` through `terrain_task_pool`; `setup` eagerly initializes the pool to document the Bevy-shaped dependency.
- Polling order: `run_chunk_tasks` first calls `poll_completed_chunk_tasks`, then `spawn_queued_chunk_tasks`, so ready completions free pending slots before new queued work is sent to the task pool.
- Completion handling: `poll_completed_chunk_tasks` polls each generating task with `block_on(poll_once(...))`, records completed results for a second pass, and removes the pending entry after handling completion.
- Cancellation handling: incomplete generating tasks whose chunk is no longer requested by any current `View` are removed from `GameState::pending`; the Bevy task may finish later, but stale-result validation remains the safety boundary.
- Stale validation before mutation: `CompletionContext` records expected position, current view demand, duplicate/already-loaded state, and shutdown state. `decide_completed_chunk` inserts only generated chunks whose position still matches, is still requested, is not already loaded, and is not observed during shutdown.
- Mutation targets: only `handle_completed_chunk` mutates the world, and only by inserting into the single mutable `ChunkLayer` after a pure completion decision returns `Insert`.
- Shutdown assumptions: the current terrain shell passes `shutting_down: false`; shutdown safety is represented as a pure decision branch and covered by tests, but no Valence shutdown signal is wired into the example.
- Dependency impact: the extraction remains example-scoped and uses existing Valence development/example dependencies (`bevy_ecs`, `bevy_tasks`, `noise`, `tracing`). It does not add a production Valence API, default plugin, persistence, vanilla parity, public-server safety, or global async-runtime contract.

## Extraction target

The extraction should separate terrain generation policy from task lifecycle ownership by introducing an opt-in Bevy task plugin/resource shell that owns queued/generating state, spawning, polling, cancellation decisions, and completion routing. The pure generator remains a deterministic function over explicit inputs, and ECS mutation stays in the terrain example shell after stale-work checks pass.

## Non-claims

This inventory does not claim production worldgen, chunk persistence, vanilla terrain parity, semantic equivalence, public-server readiness, broad Minecraft compatibility, full survival correctness, or a global Valence async runtime replacement.
