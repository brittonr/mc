# Proposal: Move example worldgen work to Bevy task orchestration

## Why

The `terrain` example currently demonstrates background chunk generation with raw standard-library threads and channels, while its comments note that Bevy task pools could be used. If Valence wants this example to model idiomatic Bevy server architecture, background worldgen should be orchestrated through Bevy resources and systems, with pure chunk generation isolated from task spawning and ECS mutation.

## What Changes

- Inventory current terrain background work, thread/channel ownership, task inputs, completion handling, and shutdown assumptions.
- Define a Bevy task orchestration boundary for chunk generation, including pure generation inputs/outputs and ECS adapter systems.
- Replace raw per-example thread spawning with Bevy task-pool integration or a documented Valence-owned task resource.
- Preserve example behavior and avoid blocking the main tick loop.
- Add positive chunk completion tests and negative cancellation, duplicate request, stale view, worker failure, and shutdown tests.

## Impact

- **Files**: `servers/valence/examples/terrain.rs`, possible Valence example helper module, Cargo dependencies if Bevy task crates are needed, tests, docs/evidence after implementation.
- **Testing**: pure chunk generation tests, task adapter smoke tests, stale/duplicate request tests, example compile/run checks, Cairn gates, and Cairn validation.
- **Non-claims**: this does not create a production terrain engine, chunk persistence layer, or universal async runtime for Valence.
