# Design: Gate optional fixtures with Bevy run conditions

## Context

Optional compatibility fixture systems often encode their disabled mode as `Option<ResMut<_>>` checks, environment helper checks, or early returns. Bevy run conditions are a better fit when disabled behavior is a true no-op, but they are dangerous for systems with `EventReader`s if skipped readers later replay stale events. The work must therefore classify disabled semantics before changing scheduling.

## Decisions

### 1. Disabled behavior is contractual

**Choice:** Every targeted optional system records whether disabled input is skipped, drained, transformed, rejected, or handled by an explicit guard.

**Rationale:** Run conditions change whether the system body and event readers execute; the disabled contract determines whether that is safe.

### 2. Use `run_if` only for pure no-op disabled paths

**Choice:** Apply `run_if` to systems or sets only when disabled behavior does not need cleanup, event draining, diagnostics, or stale-state mutation.

**Rationale:** Schedule conditions should simplify true no-op hooks, not hide stateful disabled behavior.

### 3. Keep drains explicit for event readers

**Choice:** Systems that must advance event cursors while disabled keep an in-system drain or a dedicated drain system.

**Rationale:** Compatibility rails should not emit milestones from events that arrived while the fixture was disabled.

### 4. Runtime toggles are tested where supported

**Choice:** Tests cover enabled behavior, disabled behavior, stale events, and re-enable behavior for each changed system.

**Rationale:** Most regressions in this area appear only after a disabled interval.

## Risks / Trade-offs

- Moving a guard to `run_if` can change event cursor behavior; event-reader systems need explicit negative tests.
- Schedule evidence may become more complex when conditions are set-level; receipts must name the condition and disabled contract.
- Some optional systems should remain in-system guarded because their disabled behavior is not a pure no-op.
