# Design: Death respawn reconnect during death rail

## Context

The `reconnect during death` row is currently a non-claim or bounded-gap item in the compatibility evidence set. This change creates a row-scoped Cairn so future implementation cannot silently broaden existing evidence.

## Decisions

### 1. Keep the row narrow

**Choice:** Claim only one death event followed by disconnect before respawn, reconnect, and coherent dead/respawnable or respawned state according to the fixture policy.

**Rationale:** Narrow rows are reviewable and keep broad compatibility claims false until every required row has evidence.

### 2. Compare normalized metrics

**Choice:** Checkers compare death milestone, disconnect point, reconnect username/session, server retained death state, client post-reconnect state, respawn action, and final health/playable state.

**Rationale:** Raw logs and pass/fail alone do not prove the intended compatibility claim.

### 3. Fail closed on evidence gaps

**Choice:** Reject missing disconnect timing, lost death state, duplicate entity/session confusion, unexpected alive state, missing respawn final state, or full reconnect safety overclaim.

**Rationale:** Missing reference, telemetry, parser, or correlation data must block promotion instead of becoming implicit coverage.

### 4. Preserve adjacent non-claims

**Choice:** Docs and receipts keep these non-claims explicit: all reconnect timings, crash recovery, multi-client reconnect races, full death/respawn lifecycle, production readiness, and unbounded reconnect safety.

**Rationale:** Each Cairn reduces one named gap without smuggling in full protocol, gameplay, production, or security claims.

## Risks / Trade-offs

- Session identity correlation must be strict to avoid mixing pre/post reconnect logs.
- The allowed post-reconnect state must be fixture-defined before implementation.
