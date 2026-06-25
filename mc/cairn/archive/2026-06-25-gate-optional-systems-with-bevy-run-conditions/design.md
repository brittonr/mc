# Design: Gate optional systems with Bevy run conditions

## Context

Bevy `run_if` conditions can prevent disabled optional systems from entering their logic at all. However, event readers have stateful cursors: skipping a system can preserve unread events and create a backlog if the feature is later enabled. The design must choose the disabled behavior per system, not apply `run_if` mechanically.

## Decisions

### 1. Disabled contracts are explicit

**Choice:** Each targeted optional system declares whether disabled mode should skip reads, drain events, emit rejection diagnostics, or keep an in-system guard.

**Rationale:** Disabled behavior is observable when events are buffered or metrics are emitted.

### 2. Use run conditions for pure no-op hooks

**Choice:** Systems that have no disabled cleanup obligation should use `run_if` or set-level conditions over repeated config branches.

**Rationale:** This reduces per-tick work and exposes the condition in the schedule.

### 3. Keep explicit drain systems when needed

**Choice:** Systems that must advance an `EventReader` while disabled should either keep a small disabled-drain system or document why unread events are safe.

**Rationale:** Avoiding work must not introduce stale-event behavior after a feature is re-enabled.

### 4. Runtime toggles are tested

**Choice:** Tests cover enabled-to-disabled and disabled-to-enabled transitions where runtime config can change.

**Rationale:** Run conditions can change event reader timing, so toggles need explicit coverage.

## Risks / Trade-offs

- `run_if` can hide why a system did not run; expose condition names and tests.
- Some systems are clearer with a local guard; do not churn them if disabled behavior is non-trivial.
- Event accumulation bugs may be subtle; include negative stale-event tests for readers.
