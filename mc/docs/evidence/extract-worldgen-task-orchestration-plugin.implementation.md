# extract-worldgen-task-orchestration-plugin implementation notes

## Question

How does the terrain example now separate pure worldgen generation from an opt-in Bevy task orchestration shell?

## Pure generation boundary

- The chunk generator remains a deterministic core over explicit `ChunkGenerationInput` plus explicit noise/settings inputs.
- The generator returns `Result<UnloadedChunk, ChunkGenerationError>` and does not receive Bevy queries, commands, resources, channels, clocks, logging handles, or global ECS state.
- Task request and result surfaces are typed as request, completion, cancellation, context, status, decision, result, and error records/enums so tests can exercise policy without constructing a Bevy world.

## Opt-in task shell

- The terrain gameplay plugin opts into a local worldgen task plugin instead of keeping pending task ownership in the gameplay state.
- The worldgen task plugin owns the task resource, the task-pool initialization, and a contract naming the world-mutation phase used by the shell.
- The shell polls completed tasks before spawning queued tasks, which keeps completed slots deterministic and avoids sending duplicate queued work.
- The shell routes successful generation through a completion decision before any chunk-layer insertion.

## Stale-work safety

- Completion handling now checks request ownership, current view demand, already-loaded state, shutdown state, and returned chunk position before mutation.
- Unowned, stale, duplicate/already-loaded, failed, shutdown, and mismatched-position completions fail closed without inserting a chunk.
- Cancellation remains best-effort through task ownership removal, with stale completion checks preserved as the mandatory safety boundary.

## Focused test coverage

The focused terrain example test log records positive pure generation and valid completion coverage plus negative duplicate request, stale view, worker failure, cancellation, shutdown, plugin-disabled, unowned completion, already-loaded duplicate completion, and mismatched-position coverage.

## Non-claims

This evidence does not claim production worldgen, persistence, vanilla terrain parity, semantic equivalence, public-server readiness, broad Minecraft compatibility, full survival correctness, or a global Valence async runtime replacement.
