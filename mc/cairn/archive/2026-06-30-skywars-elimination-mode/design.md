# Design: Add SkyWars elimination mode

## Context

SkyWars combines fast PvP with world mutation. Players spawn on isolated islands, collect randomized chest loot, bridge or fight toward contested areas, and are eliminated through combat or void/fall outcomes. The mode needs stronger arena metadata than Duels because block edits, chest state, and cleanup volumes affect correctness.

## Decisions

### 1. Model each arena as owned island metadata plus mutable world state

**Choice:** Arena metadata records spawn islands, chest positions, center regions, buildable volumes, protected volumes, void/fall policy, spectator zones, and cleanup ownership.

**Rationale:** Raw blocks alone cannot prove that each player has a safe start, reachable loot, legal build space, or a complete reset boundary.

### 2. Keep loot generation deterministic from explicit inputs

**Choice:** Loot tables, chest roles, seed inputs, and refill policy are passed into a pure loot core. Shell systems apply inventory/chest mutations.

**Rationale:** Deterministic loot fixtures make positive and negative tests reviewable and avoid hidden randomness in validation.

### 3. Separate elimination decisions from combat implementation

**Choice:** The mode classifies player elimination from combat death, void/fall outcome, disconnect policy, and admin/end-state events without changing combat internals.

**Rationale:** SkyWars semantics should be testable even while combat parity remains separately scoped.

### 4. Reset by plan, not ad hoc cleanup

**Choice:** Arena reset produces a plan that clears temporary blocks, chests, dropped items, projectiles, spectators, scoreboard state, and winner markers within owned volumes.

**Rationale:** Block-building modes need explicit cleanup to avoid cross-match world corruption.

## Risks / Trade-offs

- Map validation may initially be stricter than map authors expect; fail-closed metadata is preferable to silent unfair starts.
- Loot balance is out of scope; only deterministic selection and legal item placement are scoped.
- Bridging and block mutation can pressure shared world-action APIs. Any reusable seam must stay generic and keep SkyWars policy in the event crate.
