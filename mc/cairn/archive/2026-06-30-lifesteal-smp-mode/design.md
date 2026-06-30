# Design: Add LifeSteal SMP mode

## Context

LifeSteal modifies survival by changing a player's heart capacity after PvP deaths. That rule touches combat attribution, health state, persistence, final-death/exclusion policy, admin repair, and possible craft/trade heart items. These behaviors must be mode-local and should not alter vanilla health globally.

## Decisions

### 1. Scope heart capacity as mode-local player state

**Choice:** Store effective LifeSteal heart capacity in a mode-owned component/snapshot and project it into health behavior only for LifeSteal players.

**Rationale:** This prevents default survival, Bedwars, CTF, and other modes from inheriting LifeSteal rules.

### 2. Attribute transfers through a pure death core

**Choice:** The core receives explicit death facts, combat tags, killer/victim summaries, grace/final-death config, and current heart state, then returns transfer, no-transfer, or final-death decisions.

**Rationale:** Death events are prone to duplicate delivery, environmental ambiguity, stale tags, and self-kill exploits.

### 3. Treat final-death behavior as configurable

**Choice:** Final-death may map to spectator, temporary exclusion, inventory penalty, or another named policy; the core does not hard-code a public-server ban model.

**Rationale:** Different LifeSteal servers use different social rules, and production moderation is out of scope.

### 4. Validate snapshots and admin repairs

**Choice:** Persistence snapshots include heart capacity, player identity, version, and audit facts. Admin repair actions are bounded and audited.

**Rationale:** Heart state is valuable and must resist overflow, duplication, and corrupt restore cases.

## Risks / Trade-offs

- Players expect intense PvP balance; this Cairn scopes correctness and safety, not balance.
- Combat attribution may require shared event surfaces; keep reusable APIs generic and LifeSteal decisions in the event crate.
- Exclusion/final-death rules can affect player trust. Keep them configurable and visible in diagnostics without leaking private moderation data.
