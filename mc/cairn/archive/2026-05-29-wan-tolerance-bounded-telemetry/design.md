# Design: WAN tolerance bounded telemetry rail

## Context

The `WAN tolerance` row is currently a non-claim or bounded-gap item in the compatibility evidence set. This change creates a row-scoped Cairn so future implementation cannot silently broaden existing evidence.

## Decisions

### 1. Keep the row narrow

**Choice:** Claim only one authorized owned-local perturbation envelope with configured delay, jitter, packet loss, timeout, duration, client count, and telemetry.

**Rationale:** Narrow rows are reviewable and keep broad compatibility claims false until every required row has evidence.

### 2. Compare normalized metrics

**Choice:** Checkers compare target ownership, authorization, delay, jitter, loss, timeout, duration, client count, reconnect count, telemetry samples, pass/fail criteria, and abort reason.

**Rationale:** Raw logs and pass/fail alone do not prove the intended compatibility claim.

### 3. Fail closed on evidence gaps

**Choice:** Reject missing authorization, missing perturbation parameters, unavailable tooling without fail-closed receipt, missing telemetry, public target, or production readiness overclaim.

**Rationale:** Missing reference, telemetry, parser, or correlation data must block promotion instead of becoming implicit coverage.

### 4. Preserve adjacent non-claims

**Choice:** Docs and receipts keep these non-claims explicit: public-server safety, internet-path safety, adversarial network safety, production readiness, unbounded soak/reconnect safety, and third-party target safety.

**Rationale:** Each Cairn reduces one named gap without smuggling in full protocol, gameplay, production, or security claims.

## Risks / Trade-offs

- Network mutation can affect host state; tooling must be local, bounded, and cleaned up.
- Do not run against public targets without explicit authorization.
