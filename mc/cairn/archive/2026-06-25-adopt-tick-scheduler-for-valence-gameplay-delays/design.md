# Design: Adopt the Valence tick scheduler for gameplay delays

## Context

`valence_server::tick_scheduler` already provides a pure queue keyed by explicit ticks and an optional Bevy plugin shell. The next opportunity is adoption at call sites that currently encode delayed gameplay behavior manually.

## Decisions

### 1. Inventory timing behavior first

**Choice:** Record each selected delay, cooldown, timer, temporary state, and cleanup path before changing it.

**Rationale:** Not every time-like behavior should use a generic scheduler.

### 2. Use the scheduler only for explicit tick-keyed work

**Choice:** Migrate behavior when it can be expressed as work due at a known server tick. Keep immediate flags, continuous counters, wall-clock measurements, and async/background completions in their current patterns unless separately justified.

**Rationale:** The scheduler should not become a catch-all runtime.

### 3. Schedule typed work

**Choice:** Scheduled payloads should be typed domain events or small command requests. Systems that drain due work should validate entity liveness and current state before mutating the world.

**Rationale:** Stale delayed work should fail closed after despawn, disconnect, reconnect, or state changes.

### 4. Policy stays outside the scheduler

**Choice:** Cooldown durations, despawn choices, block restoration policy, and fixture milestones remain in gameplay/fixture code. The scheduler only orders due work.

**Rationale:** This preserves the pure scheduler's general-purpose contract.

## Risks / Trade-offs

- Delayed work can target stale entities; drained systems must validate liveness and ownership.
- Stable timing can affect compatibility receipts; preserve milestone order or record scoped behavior changes.
- Some current modulo checks are simpler than scheduler wiring; leave them alone unless scheduling improves correctness or testability.
