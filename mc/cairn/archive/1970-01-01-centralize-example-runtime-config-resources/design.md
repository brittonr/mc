# Design: Centralize example runtime config as Bevy resources

## Context

Examples such as CTF and survival compatibility use many environment toggles and paths to drive fixture behavior. Those inputs are part of compatibility contracts, but reading them ad hoc from systems mixes I/O with gameplay logic and makes testing harder. A typed resource can act as the imperative shell boundary: read env/CLI once, parse with pure functions, and let systems consume explicit config.

## Decisions

### 1. Preserve existing input contracts

**Choice:** Keep current env var names, CLI arguments, default values, and milestone text unless a separate Cairn changes behavior.

**Rationale:** Compatibility evidence depends on stable input and output contracts.

### 2. Parse config in pure cores

**Choice:** Pure parser functions accept explicit key/value/path inputs and return typed config or typed errors.

**Rationale:** Config validation should be testable without process environment or filesystem access.

### 3. Store config in resources

**Choice:** Plugin setup or example startup inserts typed resources consumed by systems and run conditions.

**Rationale:** Resources make runtime policy visible in Bevy schedule evidence and tests.

### 4. Reload paths remain explicit

**Choice:** Runtime reloads use explicit request resources/events and stale-request detection rather than hidden env polling in every system.

**Rationale:** Reload behavior is stateful and must be reviewable.

## Risks / Trade-offs

- Loading env once can change behavior for toggles currently read repeatedly; runtime-toggle expectations must be classified before migration.
- Config resources can grow large; split per fixture/domain rather than creating one global bag.
- Filesystem paths and reloads still need imperative shell code; keep that shell thin and test parser cores separately.
