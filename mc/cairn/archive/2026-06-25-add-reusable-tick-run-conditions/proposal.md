# Proposal: Add reusable tick run conditions

## Why

Several examples gate periodic behavior with inline `server.current_tick() % interval` checks. Those checks hide schedule intent inside system bodies and repeat interval arithmetic. A small reusable Bevy run-condition helper for tick cadence would make periodic systems easier to review, while keeping delayed work and cooldowns on `TickScheduler` where explicit due-work semantics are required.

## What Changes

- Inventory selected periodic systems, tick sources, interval constants, current modulo behavior, mutation targets, and evidence impact.
- Define reusable tick-cadence run-condition contracts with named interval values and invalid-interval behavior.
- Replace selected pure periodic no-op guards with `run_if` conditions or set-level conditions.
- Leave delayed work, cooldowns, wall-clock behavior, and async completion outside tick run conditions.
- Add positive cadence tests and negative invalid interval, disabled plugin, stale event-reader, tick-rate-change, and behavior-preservation tests.

## Impact

- **Files**: selected examples such as `particles.rs`, `equipment.rs`, `game_of_life.rs`, and `bench_players.rs`; possible helper module under Valence example/core support; schedule evidence under `docs/evidence/`.
- **Testing**: pure cadence tests, focused example tests, schedule hygiene when conditions change, Cairn gates, Cairn validation, and task-evidence validation.
- **Non-claims**: this does not replace `TickScheduler`, add wall-clock scheduling, or change gameplay timing policy unless separately scoped.
