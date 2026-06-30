# Design: Add factions claims and raid loop

## Context

Factions territory turns a social group into a gameplay actor that owns regions, protects blocks, opens contested raid windows, and resolves damage. This layer should depend on an accepted or implemented social core for faction identity and permissions, but it must keep its own claim geometry, protection, and raid invariants testable in isolation.

Hyperion has world simulation, block editing, inventory, commands, spatial utilities, and Bevy scheduling surfaces that can host the shell. The risky decisions are not IO details; they are claim overlap, action authorization, raid timing, exploit containment, and world repair. Those decisions belong in pure cores.

## Decisions

### 1. Depend on clan identity without redefining it

**Choice:** Territory records reference normalized faction/clan identities, role capabilities, and membership snapshots supplied by the social core. This Cairn does not create a competing roster or permission model.

**Rationale:** Claim protection must answer who can act for a faction, but duplicating membership state would create stale authority bugs.

### 2. Represent claims through deterministic geometry

**Choice:** Start with explicit claim geometry over chunks or named regions, plus named configuration for size limits, adjacency, overlap, vertical bounds, wilderness, safe zones, war zones, spawn buffers, and admin-owned regions.

**Rationale:** Claim geometry must be reviewable and independent of live block state. Pure validation can reject overlap, out-of-bounds, orphan ownership, unsafe spawn coverage, or invalid admin reservations.

### 3. Centralize protection as an action decision core

**Choice:** Build/break/place/container/entity/interaction decisions call a pure protection core with actor, faction snapshot, claim facts, action type, target block/entity/container facts, raid state, and named config.

**Rationale:** Block protection spread across event handlers becomes impossible to audit. A single decision core gives positive and negative tests one place to exercise.

### 4. Model raids as explicit windows and phases

**Choice:** Raid eligibility and siege resolution use explicit phases such as inactive, declared, warmup, active, contested, resolved, and cooldown. Timing and scoring inputs are shell-provided facts, while phase transitions are pure decisions.

**Rationale:** Hidden timers and implicit state cause unfair raids and review gaps. Phase decisions should fail closed on stale clocks, disconnected attackers/defenders, missing claims, or disabled raid config.

### 5. Separate world mutation from repair plans

**Choice:** The protection/raid core emits allowed mutations and repair intents; shell systems perform block/entity/container updates and persist repair records. Repair validation rejects impossible snapshots, unowned edits, stale raids, and target escapes.

**Rationale:** Raiding needs destructible or semi-destructible territory, but review must distinguish temporary damage from permanent grief and make cleanup testable.

### 6. Keep map and visibility summaries non-authoritative

**Choice:** Expose deterministic claim summaries for commands, debug overlays, map markers, or evidence receipts, but keep those summaries derived from authoritative claim state and not required for protection decisions.

**Rationale:** Players need to see claims, yet UI details can become expensive and packet-specific. The first territory Cairn should prove data correctness without claiming full client UI parity.

## Risks / Trade-offs

- Chunk claims are simple and performant, while arbitrary polygons are more flexible. The contract should allow either only behind a deterministic geometry validator.
- Raid windows can become balance-heavy. This Cairn should define correctness and containment, not final economy or server balance.
- Repair records may grow large. The design should permit compact diffs later while tests start with deterministic fixtures.
- Protection hooks may require reusable Hyperion seams. Keep reusable APIs generic and keep faction-specific policy in the event/plugin layer.
