# Proposal: Adopt the Valence tick scheduler for gameplay delays

## Why

Valence now has a deterministic `TickScheduler` core and optional Bevy shell, but examples and gameplay-style fixtures can still grow ad hoc tick counters, cooldown state, and delayed cleanup logic. Using the scheduler where work is explicitly tick-keyed would make cooldowns, temporary blocks, delayed resets, despawns, and fixture deadlines easier to test and review.

## What Changes

- Inventory selected tick counters, modulo checks, cooldowns, delayed resets, temporary world changes, and despawn timers in Valence examples and compatibility fixtures.
- Classify each timing behavior as scheduler-suitable, immediate state, wall-clock/async work, or intentionally custom policy.
- Replace scheduler-suitable ad hoc delays with `ServerTickScheduler` resources or typed scheduler events.
- Keep gameplay policy durations and decisions outside the scheduler core.
- Add positive due-work tests and negative cancellation, stale entity, duplicate event, and plugin-disabled tests.

## Impact

- **Files**: selected Valence examples, `valence_server::tick_scheduler` docs/tests if gaps are found, compatibility fixture adapters, docs/evidence after implementation.
- **Testing**: scheduler integration tests, plugin-disabled tests, changed example checks, selected compatibility rails if fixture timing changes, Cairn gates, and Cairn validation.
- **Non-claims**: this does not add wall-clock scheduling, async task scheduling, or default gameplay policy.
