# Design: Add Build Battle creative contest mode

## Context

Build Battle is phase-driven rather than combat-driven: players receive a theme, build in isolated plots, vote on builds, and receive ranked results. Correctness depends on plot isolation, creative permissions, fair voting, phase transitions, and cleanup.

## Decisions

### 1. Represent contests as explicit phase state

**Choice:** A contest moves through lobby, theme selection, build, vote, results, and cleanup phases using named Bevy state or markers plus pure phase-transition rules.

**Rationale:** Phase gates prevent voting during build, building during voting, and creative permissions after cleanup.

### 2. Use plot metadata for permission boundaries

**Choice:** Plot templates and contest instances record owned build volumes, spawn points, forbidden volumes, allowed commands/items, and cleanup targets.

**Rationale:** Creative privileges must not leak outside the assigned plot or after the contest ends.

### 3. Keep voting and scoring pure

**Choice:** Vote validation, duplicate handling, self-vote policy, tie handling, and score aggregation are pure functions over explicit vote facts and config.

**Rationale:** Voting fairness is easier to review when hidden state and side effects are excluded.

### 4. Make presentation original and configurable

**Choice:** Theme names, vote labels, result text, particles, and sounds are configurable and original.

**Rationale:** This avoids copying protected presentation from existing servers and enables future theming.

## Risks / Trade-offs

- Creative mode opens many mutation surfaces. Start with a narrow allowed-action policy and strong negative tests.
- Theme moderation is not production content safety. This Cairn can filter fixtures but does not claim public moderation.
- Team contests add membership and plot-sharing complexity. Keep team entry configurable and tested before broadening.
