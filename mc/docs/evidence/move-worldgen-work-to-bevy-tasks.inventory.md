# move-worldgen-work-to-bevy-tasks terrain worldgen inventory

## Question

What terrain-example background work is being replaced by Bevy task orchestration?

## Current implementation before orchestration change

- Owner subtree: `servers/valence`.
- Primary file: `servers/valence/examples/terrain.rs`.
- Background worker state: `ChunkWorkerState` is shared through `Arc` and owns the finished-work `Sender<(ChunkPos, UnloadedChunk)>`, pending-work `Receiver<ChunkPos>`, and five `SuperSimplex` noise generators.
- Thread creation: `setup` calls `thread::available_parallelism().unwrap().get()` and spawns that many raw `std::thread` workers with `thread::spawn(move || chunk_worker(state))`.
- Channels: `setup` creates an unbounded pending queue from ECS to workers and an unbounded finished queue from workers back to ECS using `flume::unbounded()`.
- Task input: workers receive only `ChunkPos`; generation settings are implicit constants or captured worker state (`HEIGHT`, seed-derived noise, fixed terrain parameters).
- Completion output: workers send `(ChunkPos, UnloadedChunk)` back to the ECS shell; send errors are ignored.
- Pending request state: `GameState::pending` maps `ChunkPos` to `Option<Priority>`, where `Some(priority)` means not yet sent and `None` means already sent to a worker.
- Completion polling: `send_recv_chunks` drains all finished-channel entries each tick.
- World mutation: `send_recv_chunks` inserts each finished chunk into the single `ChunkLayer` and asserts that the pending map still contained the position.
- Duplicate handling: duplicate queued views update priority while a request is still unsent; duplicates after sending are represented only by the `None` pending marker.
- Stale-view handling gap: completed chunks are inserted without checking whether any current client view still requests the chunk.
- Worker failure behavior: generation has no typed error path; a worker panic would silently reduce raw-thread capacity and leave pending work unresolved.
- Shutdown assumptions: dropping the ECS resource closes the pending sender and lets raw workers exit when the receiver disconnects; there is no explicit task cancellation or drain policy.

## Dependency impact before implementation

- Planned adoption uses `bevy_tasks` for the terrain example task shell because Valence already depends on Bevy ECS concepts and the change is example-scoped.
- The dependency should remain a Valence development/example dependency and should not change Valence public runtime features.
- Existing `flume` dependency remains required elsewhere in the workspace; this change only removes terrain-example `flume` use.

## Non-claims

This inventory does not claim production terrain generation, chunk persistence, backpressure, universal async-runtime behavior, semantic equivalence to vanilla worldgen, public-server readiness, or broad Minecraft compatibility.
