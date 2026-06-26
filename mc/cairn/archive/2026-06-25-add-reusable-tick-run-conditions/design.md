# Design: Add reusable tick run conditions

## Context

Inline modulo guards are common in periodic examples. Bevy run conditions can express these periodic no-op skips at the schedule level, but they are not a replacement for delayed work, cooldowns, async completions, or systems that must drain events while disabled.

## Decisions

### 1. Classify periodic behavior first

**Choice:** Inventory each candidate as pure periodic no-op, delayed due-work, wall-clock measurement, async completion, or event-reader drain behavior.

**Rationale:** Only pure periodic no-op skips belong in run conditions.

### 2. Use named cadence values

**Choice:** Intervals are named constants or config values, not unexplained numeric literals.

**Rationale:** Tick cadence is gameplay or demo policy and must be reviewable.

### 3. Helper contracts are explicit

**Choice:** The helper defines behavior for current tick, interval, phase alignment, zero/invalid intervals, and tick-rate changes.

**Rationale:** Periodic behavior must be deterministic and testable.

### 4. Preserve event-reader semantics

**Choice:** Systems with event readers keep explicit drains or are excluded unless their disabled behavior is proven safe.

**Rationale:** Skipped event readers can replay stale events later.

## Risks / Trade-offs

- Moving modulo guards into run conditions can alter when local state updates occur; focused behavior tests are required.
- Tick-rate-dependent intervals need clear policy: ticks vs seconds.
- Over-generalizing cadence helpers can obscure simple examples; keep the API minimal.
