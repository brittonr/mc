# Design: Hyperion player join core

## Context

Player join egress is a packet-order and state-boundary surface. The extraction should make join plans deterministic while leaving runtime and network effects in Hyperion shells.

## Decisions

### 1. Keep scope Hyperion-owned

**Choice:** Treat player-join extraction as Hyperion-owned nested-repo work.

**Rationale:** Join behavior is not adopted by Valence without a separate integration Cairn.

### 2. Extract join plans

**Choice:** Initial packet selection, ordering, state summaries, chunk/view facts, and diagnostics should be pure over explicit inputs.

**Rationale:** Join behavior can be tested without a live proxy or ECS app.

### 3. Keep egress effects in shell

**Choice:** ECS reads, packet sends, network/proxy state, tracing, and scheduling remain in shells.

**Rationale:** Side effects and performance boundaries stay explicit.
