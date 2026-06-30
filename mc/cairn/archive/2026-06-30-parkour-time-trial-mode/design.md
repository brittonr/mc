# Design: Add Parkour time-trial mode

## Context

Parkour is mostly movement validation, checkpointing, timing, and leaderboard projection. The repo contains a Valence parkour example, but this Cairn should create Hyperion-owned behavior unless a separate integration Cairn classifies a Valence concept for use.

## Decisions

### 1. Treat courses as deterministic metadata

**Choice:** Course metadata records start volumes, ordered checkpoints, finish volumes, fall/reset volumes, safe respawn positions, allowed shortcuts, and cleanup ownership.

**Rationale:** A course needs reviewable invariants before runtime movement is considered valid.

### 2. Keep checkpoint and timer rules pure

**Choice:** The core receives player/course/timer facts and returns checkpoint advancement, reset, finish, personal-best, and leaderboard update decisions.

**Rationale:** Timing correctness should not depend on packets, wall-clock access inside core logic, or global mutable state.

### 3. Use shell systems only for observations and feedback

**Choice:** Bevy systems observe positions, falls, interactions, and disconnects; they call pure cores and then mutate player location, scoreboard projection, sounds/particles, or diagnostics.

**Rationale:** This follows the functional-core / imperative-shell boundary and keeps movement side effects contained.

### 4. Make reference use explicit

**Choice:** Any comparison to `servers/valence/examples/parkour.rs` is reference-only unless a separate Cairn permits porting.

**Rationale:** Project instructions require integration boundary review before using Hyperion/Valence code across component ownership lines.

## Risks / Trade-offs

- Leaderboards can quickly become a production storage problem. This Cairn scopes deterministic ranking snapshots and non-production persistence only.
- Timer precision may differ between server tick, client display, and receipt time; the core should use explicit tick/time summaries.
- Strict anti-shortcut validation can reject creative course designs. Allow optional shortcut metadata rather than implicit bypasses.
