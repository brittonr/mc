# Design: CTF score limit win condition rail

## Context

The `score limit / win condition` row is currently a non-claim or bounded-gap item in the compatibility evidence set. This change creates a row-scoped Cairn so future implementation cannot silently broaden existing evidence.

## Decisions

### 1. Keep the row narrow

**Choice:** Claim only one bounded match reaching a configured score limit and emitting the configured win/end state exactly once.

**Rationale:** Narrow rows are reviewable and keep broad compatibility claims false until every required row has evidence.

### 2. Compare normalized metrics

**Choice:** Checkers compare score limit, team scores before final capture, final capture actor/team, win team, end-state milestone, duplicate-win guard, and post-win forbidden score changes.

**Rationale:** Raw logs and pass/fail alone do not prove the intended compatibility claim.

### 3. Fail closed on evidence gaps

**Choice:** Reject missing prelimit state, missing final capture, wrong winning team, duplicate win, post-win score mutation, or full CTF overclaim.

**Rationale:** Missing reference, telemetry, parser, or correlation data must block promotion instead of becoming implicit coverage.

### 4. Preserve adjacent non-claims

**Choice:** Docs and receipts keep these non-claims explicit: all match settings, overtime/tiebreakers, scoreboard UI parity, all scoring races, production gameplay readiness, and full CTF correctness.

**Rationale:** Each Cairn reduces one named gap without smuggling in full protocol, gameplay, production, or security claims.

## Risks / Trade-offs

- Long score sequences are expensive; fixture should start near the limit.
- Scoreboard UI evidence should be separate from server win-state evidence.
