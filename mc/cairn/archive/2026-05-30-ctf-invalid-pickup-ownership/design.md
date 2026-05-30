# Design: CTF invalid pickup ownership rail

## Context

The `invalid flag pickup/ownership` row is currently a non-claim or bounded-gap item in the compatibility evidence set. This change creates a row-scoped Cairn so future implementation cannot silently broaden existing evidence.

## Decisions

### 1. Keep the row narrow

**Choice:** Claim only one configured invalid flag pickup attempt by the wrong team or invalid owner state with no ownership transfer and no score.

**Rationale:** Narrow rows are reviewable and keep broad compatibility claims false until every required row has evidence.

### 2. Compare normalized metrics

**Choice:** Checkers compare player team, flag identity, pre-owner state, invalid pickup action, post-owner state, score counters, forbidden capture/score patterns, and containment outcome.

**Rationale:** Raw logs and pass/fail alone do not prove the intended compatibility claim.

### 3. Fail closed on evidence gaps

**Choice:** Reject unexpected owner transfer, unexpected score/capture, missing forbidden-pattern scan, missing server correlation, or broad all-invalid-action claim.

**Rationale:** Missing reference, telemetry, parser, or correlation data must block promotion instead of becoming implicit coverage.

### 4. Preserve adjacent non-claims

**Choice:** Docs and receipts keep these non-claims explicit: all invalid actions, all flag permutations, full CTF correctness, adversarial security, production readiness, and broad Minecraft compatibility.

**Rationale:** Each Cairn reduces one named gap without smuggling in full protocol, gameplay, production, or security claims.

## Risks / Trade-offs

- A negative pass must prove the attempted action happened, not merely that nothing happened.
- Forbidden-pattern checks need both client and server sources.
