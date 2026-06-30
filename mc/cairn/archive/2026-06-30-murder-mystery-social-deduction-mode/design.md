# Design: Add Murder Mystery social-deduction mode

## Context

Murder Mystery is driven by hidden roles and asymmetric information. The core risks are role leakage, invalid kills, item grants at the wrong time, victory-condition mistakes, and stale state after disconnects. The mode should be original in presentation and mode-local in behavior.

## Decisions

### 1. Keep role assignment and visibility pure

**Choice:** Role assignment, team/party avoidance policy, role reveal policy, and visibility filtering are pure functions over player summaries and config.

**Rationale:** Hidden information must be testable without relying on ad hoc scoreboard or packet behavior.

### 2. Use shell filters for player-facing output

**Choice:** Bevy/network shells call visibility cores before sending scoreboard, chat, action bar, item, spectator, or diagnostic output.

**Rationale:** Role leaks often happen through secondary UI, not only direct role messages.

### 3. Model asymmetric items as mode-local grants

**Choice:** Detective-like tools, murderer tools, innocents' pickups, and recovery items are granted and consumed through a pure eligibility core with shell-side inventory mutation.

**Rationale:** Item misuse and stale grants are common exploit paths.

### 4. Make victory conditions explicit

**Choice:** Victory is determined from alive role counts, timer state, disconnect policy, and configured special-case rules.

**Rationale:** Social-deduction rounds need deterministic endings even when players disconnect or become spectators.

## Risks / Trade-offs

- Hidden-role games are vulnerable to social/streaming abuse. This Cairn only scopes server mechanics and leak tests.
- Original presentation may feel less familiar than large-network clones. Avoid protected naming while preserving the core loop.
- Visibility filtering can touch shared output surfaces. Any reusable filter seam must stay generic and keep role policy in the mode crate.
