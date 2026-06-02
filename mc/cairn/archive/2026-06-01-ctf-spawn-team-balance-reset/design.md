# Design: CTF spawn team balance reset rail

## Context

The `spawn/team balance/resource reset` row is currently a non-claim or bounded-gap item in the compatibility evidence set. This change creates a row-scoped Cairn so future implementation cannot silently broaden existing evidence.

## Decisions

### 1. Keep the row narrow

**Choice:** Claim only one configured join/team-selection/reset sequence with bounded team counts, spawn locations, inventory/resource state, and reset milestones.

**Rationale:** Narrow rows are reviewable and keep broad compatibility claims false until every required row has evidence.

### 2. Compare normalized metrics

**Choice:** Checkers compare team counts, selected teams, spawn coordinates, initial resources, post-score or post-death reset state, inventory/resource ids, and server correlation ids.

**Rationale:** Raw logs and pass/fail alone do not prove the intended compatibility claim.

### 3. Fail closed on evidence gaps

**Choice:** Reject team imbalance outside bounds, wrong spawn, stale inventory/resource state, missing reset milestone, or full CTF overclaim.

**Rationale:** Missing reference, telemetry, parser, or correlation data must block promotion instead of becoming implicit coverage.

### 4. Preserve adjacent non-claims

**Choice:** Docs and receipts keep these non-claims explicit: all team balancing algorithms, all maps, all resource loadouts, all reset triggers, production gameplay readiness, and full CTF correctness.

**Rationale:** Each Cairn reduces one named gap without smuggling in full protocol, gameplay, production, or security claims.

## Risks / Trade-offs

- Team balance semantics may be game-specific; the contract should cite fixture rules instead of generic CTF rules.
- Spawn coordinates need tolerance if server position corrections occur.
