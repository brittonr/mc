# Design: CTF simultaneous pickup capture race rail

## Context

The `simultaneous pickup/capture race` row is currently a non-claim or bounded-gap item in the compatibility evidence set. This change creates a row-scoped Cairn so future implementation cannot silently broaden existing evidence.

## Decisions

### 1. Keep the row narrow

**Choice:** Claim only one configured two-client race window with deterministic ordering oracle and exactly one accepted state transition.

**Rationale:** Narrow rows are reviewable and keep broad compatibility claims false until every required row has evidence.

### 2. Compare normalized metrics

**Choice:** Checkers compare client identities, team roles, action timestamps or ordered milestones, accepted transition, rejected transition, final flag state, final score, and race-window bounds.

**Rationale:** Raw logs and pass/fail alone do not prove the intended compatibility claim.

### 3. Fail closed on evidence gaps

**Choice:** Reject missing order evidence, double accept, inconsistent final flag state, unexpected score, missing client/server correlation, or all-race overclaim.

**Rationale:** Missing reference, telemetry, parser, or correlation data must block promotion instead of becoming implicit coverage.

### 4. Preserve adjacent non-claims

**Choice:** Docs and receipts keep these non-claims explicit: all race conditions, network adversarial safety, unbounded concurrency, full CTF correctness, production readiness, and broad Minecraft compatibility.

**Rationale:** Each Cairn reduces one named gap without smuggling in full protocol, gameplay, production, or security claims.

## Risks / Trade-offs

- Race tests can be flaky without server-authoritative ordering logs.
- Network jitter tolerance must remain separate unless explicitly configured.
