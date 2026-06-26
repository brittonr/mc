# Proposal: Extract worldgen task orchestration plugin

## Why

The terrain example already uses Bevy's async compute task pool and keeps chunk generation logic out of ECS access. That pattern is valuable beyond one example, but task request state, completion polling, stale-work validation, and mutation application are still local to `terrain.rs`. A small reusable Bevy shell for chunk-generation requests would make background work easier to test, compose, and reuse without moving generation policy into ECS or claiming production worldgen.

## What Changes

- Inventory current terrain background work, task inputs, pending request state, completion handling, cancellation, stale-view checks, shutdown assumptions, and dependency impact.
- Define a pure worldgen core boundary and typed request/completion/error surfaces for task orchestration.
- Extract or introduce a reusable Bevy task resource/plugin shell that owns spawning, polling, cancellation, and completion routing.
- Preserve stale-work validation before mutating chunk layers.
- Add positive completion tests and negative duplicate request, stale view, worker failure, cancellation, shutdown, and plugin-disabled tests.

## Impact

- **Files**: `servers/valence/examples/terrain.rs`, possible reusable Valence server/example-support module if scoped, tests, docs/evidence under `docs/evidence/`.
- **Testing**: pure generation tests, task shell tests, terrain example checks, schedule hygiene if plugin wiring changes, Cairn gates, Cairn validation, and task-evidence validation.
- **Non-claims**: this does not add production worldgen, persistence, vanilla terrain parity, global async runtime replacement, or default Valence behavior.
