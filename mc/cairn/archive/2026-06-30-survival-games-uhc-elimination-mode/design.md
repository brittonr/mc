# Design: Add Survival Games and UHC elimination mode

## Context

Survival Games and UHC share an elimination-survival foundation but differ in preparation, regeneration, crafting, and world-resource rules. A profile-based design can reuse arena/world lifecycle while keeping each mode's policy explicit.

## Decisions

### 1. Build an elimination-survival foundation with profiles

**Choice:** Implement common lobby, spawn, phase, loot, border, elimination, spectator, win, and reset systems. Survival Games and UHC behavior are selected by named profile config.

**Rationale:** The modes share lifecycle and differ mainly through rules that can be explicit config and pure policies.

### 2. Keep phase and border logic pure

**Choice:** Phase transitions, grace/prep eligibility, border/deathmatch pressure, and win detection are pure over tick/time summaries and arena state.

**Rationale:** Time-based eliminations and border pressure must be deterministic in tests and receipts.

### 3. Scope UHC regeneration separately from default survival

**Choice:** UHC profile rules decide natural regeneration, allowed healing, and combat consequences only for players in the active UHC profile.

**Rationale:** This prevents accidental changes to survival compatibility or vanilla health behavior.

### 4. Reset worlds through explicit ownership plans

**Choice:** The shell applies reset plans for loot containers, dropped items, temporary blocks, border state, spectators, and player inventories within owned arena/world volumes.

**Rationale:** Elimination modes mutate enough world state that cleanup must be reviewable.

## Risks / Trade-offs

- Combining two modes can hide differences. Profiles must make changed rules explicit and tests must cover both profiles.
- Loot and UHC recipes can become large content catalogs. Start with bounded fixtures and later content-expansion Cairns.
- Border/deathmatch UX can be client-sensitive. This Cairn scopes server-side policy and feedback, not custom-client presentation.
