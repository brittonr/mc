# Proposal: Add a Valence tick scheduler

## Why

Hyperion's scheduler is a small deterministic queue for delayed work. Valence gameplay plugins frequently need tick-based delays for cooldown expiry, temporary blocks, despawns, match timers, and deferred cleanup. A Valence-owned scheduler would reduce ad hoc timer code while keeping scheduling testable and independent of wall-clock time.

## What Changes

- Review Hyperion's `hyperion-scheduled` crate and existing Valence timer/tick patterns.
- Define a stable tick scheduler API for scheduling, peeking, draining due work, cancellation if needed, and deterministic ordering.
- Implement the scheduler as a pure core over explicit tick keys and values, with Bevy/ECS systems as thin shells.
- Add positive and negative fixtures for empty queues, due and not-due work, same-tick ordering, cancellation or clear behavior, overflow boundaries, and plugin-disabled behavior.
- Document common gameplay uses and non-goals.

## Impact

- **Files**: optional Valence scheduler crate/plugin or utility module, gameplay examples, tests, docs, and Cairn artifacts.
- **Testing**: pure scheduler tests, ECS/plugin smoke tests, invalid input fixtures, example timer smoke tests, and Cairn gates/validation.
- **Non-claims**: this does not introduce wall-clock scheduling, async task scheduling, or game-specific timer policy by default.
