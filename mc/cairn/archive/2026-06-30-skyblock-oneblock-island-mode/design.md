# Design: Add SkyBlock and OneBlock island mode

## Context

SkyBlock/OneBlock is persistent and permission-heavy. Unlike arena modes, the core state must survive reconnects and restarts, and world mutation is the primary gameplay. This requires explicit island ownership, generator progression, snapshot validation, and fail-closed permission checks.

## Decisions

### 1. Use one island-mode boundary with profiles

**Choice:** Implement a shared island-mode plugin with profiles for SkyBlock-style starter islands and OneBlock-style generator progression.

**Rationale:** Island ownership, membership, visits, permissions, void recovery, resets, and snapshots are shared. Generator rules differ by profile but can use the same shell.

### 2. Make permissions pure and central

**Choice:** Build, break, container, invite, visit, reset, and admin actions are checked through pure permission cores over explicit actor, island, role, and action summaries.

**Rationale:** Cross-island mutation and grief leaks are the highest risks. Central fail-closed checks are easier to test than distributed guards.

### 3. Treat generator progression as deterministic state

**Choice:** OneBlock generator phases, allowed outputs, mob/chest events, cooldowns, and rewards are derived from explicit generator state and named config.

**Rationale:** Deterministic progression enables positive/negative tests and prevents hidden random state from corrupting snapshots.

### 4. Snapshot before production persistence claims

**Choice:** Start with reviewable snapshot contracts and recovery tests before claiming production persistence or scale.

**Rationale:** Persistent modes need corruption handling, versioning, and recovery boundaries before public-server use.

## Risks / Trade-offs

- Island modes can expand into economy, auction, quest, and marketplace systems. Keep those as hooks or later Cairns unless required for the MVP.
- World storage and chunk ownership may require reusable Hyperion seams. Keep generic seams separate from island policy.
- Reset/delete actions are destructive. Require deterministic plans and negative tests for unauthorized or stale reset requests.
