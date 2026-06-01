# Design: Repeated death respawn safety rail

## Context

The `repeated death safety` row is currently a non-claim or bounded-gap item in the compatibility evidence set. This change creates a row-scoped Cairn so future implementation cannot silently broaden existing evidence.

## Decisions

### 1. Keep the row narrow

**Choice:** Claim only a configured finite sequence of death and respawn cycles with stable health, entity identity, inventory policy, and no duplicate terminal state.

**Rationale:** Narrow rows are reviewable and keep broad compatibility claims false until every required row has evidence.

### 2. Compare normalized metrics

**Choice:** Checkers compare cycle index, death cause, respawn request, restored health, entity/session id, inventory policy state, forbidden duplicate deaths, and final playable state.

**Rationale:** Raw logs and pass/fail alone do not prove the intended compatibility claim.

### 3. Fail closed on evidence gaps

**Choice:** Reject missing cycle metric, stale health, duplicate death without respawn, lost entity correlation, unexpected score/capture, or unbounded lifecycle overclaim.

**Rationale:** Missing reference, telemetry, parser, or correlation data must block promotion instead of becoming implicit coverage.

### 4. Preserve adjacent non-claims

**Choice:** Docs and receipts keep these non-claims explicit: unbounded repeated death safety, all death causes, reconnect-during-death, inventory semantics outside configured policy, production readiness, and full lifecycle correctness.

**Rationale:** Each Cairn reduces one named gap without smuggling in full protocol, gameplay, production, or security claims.

## Risks / Trade-offs

- Repeated live cycles can be slow; keep the cycle count small and named.
- Cycle indexes must be in logs so checker can reject collapsed evidence.
