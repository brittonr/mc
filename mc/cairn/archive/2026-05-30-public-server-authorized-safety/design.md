# Design: Public server authorized safety rail

## Context

The `public-server safety` row is currently a non-claim or bounded-gap item in the compatibility evidence set. This change creates a row-scoped Cairn so future implementation cannot silently broaden existing evidence.

## Decisions

### 1. Keep the row narrow

**Choice:** Claim only one explicitly authorized public or non-loopback target envelope with owner, written authorization reference, bounds, telemetry, and abort criteria.

**Rationale:** Narrow rows are reviewable and keep broad compatibility claims false until every required row has evidence.

### 2. Compare normalized metrics

**Choice:** Checkers compare target owner, authorization artifact, target scope, client count, duration, traffic limits, telemetry, abort criteria, redaction policy, and human checkpoint decision.

**Rationale:** Raw logs and pass/fail alone do not prove the intended compatibility claim.

### 3. Fail closed on evidence gaps

**Choice:** Reject missing owner, missing written authorization, missing bounds, missing telemetry, missing checkpoint, secrets in logs, or production readiness overclaim.

**Rationale:** Missing reference, telemetry, parser, or correlation data must block promotion instead of becoming implicit coverage.

### 4. Preserve adjacent non-claims

**Choice:** Docs and receipts keep these non-claims explicit: third-party target safety without authorization, production readiness, adversarial safety, WAN tolerance, load safety beyond configured bounds, and unbounded public testing.

**Rationale:** Each Cairn reduces one named gap without smuggling in full protocol, gameplay, production, or security claims.

## Risks / Trade-offs

- This rail must not be executed without explicit user-provided authorization.
- Logs need redaction so credentials or public addresses are not committed accidentally.
