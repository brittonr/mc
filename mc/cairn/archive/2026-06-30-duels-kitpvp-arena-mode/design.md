# Design: Add Duels and KitPvP arena mode

## Context

Duels/KitPvP should be the smallest competitive arena mode that proves queueing, kits, combat lifecycle, score, and reset without adding persistent territory, economy, or Bedwars rules. Hyperion already has event-mode precedent under `events/bedwars`; this work should follow that pattern with isolated plugin ownership.

## Decisions

### 1. Use one arena-mode foundation with configurable profiles

**Choice:** Implement a shared arena foundation that supports duel-style scheduled matches and KitPvP-style immediate respawn profiles through named configuration.

**Rationale:** Queueing, spawn assignment, kit validation, combat attribution, scoreboard projection, and cleanup are shared. Profiles keep behavior explicit without duplicating the core.

### 2. Keep queue and match decisions pure

**Choice:** Matchmaking input summaries, queue membership, arena eligibility, kit catalog validation, and match state transitions are pure functions. Bevy systems only collect player state, call the cores, mutate ECS state, and send feedback.

**Rationale:** Queue correctness is easy to corrupt with stale players, duplicate entries, and disconnects. Pure cores make the invariants testable without a server.

### 3. Treat combat as mode-local policy

**Choice:** The mode consumes existing combat events but classifies deaths, forfeits, respawns, score, and rematches through a mode-owned policy.

**Rationale:** This avoids changing default combat semantics or overclaiming vanilla combat parity.

### 4. Make arenas disposable state machines

**Choice:** Arenas have explicit states for idle, preparing, active, ending, and resetting. Reset output is a deterministic plan over player state, temporary entities, inventories, scoreboard rows, and diagnostics.

**Rationale:** Arena modes fail most often through leaks after disconnects or early exits. A reset plan is easier to audit than scattered cleanup.

## Risks / Trade-offs

- Shared arena foundations can drift toward a framework rewrite; keep this scoped to Duels/KitPvP needs until another Cairn depends on it.
- Competitive players expect precise hit/knockback behavior; this Cairn only scopes mode semantics, not vanilla parity or anticheat.
- Stats and leaderboards can imply production persistence. Start with deterministic snapshots and non-production retention boundaries.
