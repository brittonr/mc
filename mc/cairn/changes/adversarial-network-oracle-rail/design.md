# Design: Adversarial network oracle rail

## Context

The `adversarial-network safety` row is currently a non-claim or bounded-gap item in the compatibility evidence set. This change creates a row-scoped Cairn so future implementation cannot silently broaden existing evidence.

## Decisions

### 1. Keep the row narrow

**Choice:** Claim only one explicitly approved adversarial-network model with bounded packet mutation, target ownership, telemetry, and human/oracle decision record.

**Rationale:** Narrow rows are reviewable and keep broad compatibility claims false until every required row has evidence.

### 2. Compare normalized metrics

**Choice:** Checkers compare threat model id, mutation types, packet bounds, target ownership, authorization, telemetry, abort criteria, observed containment, and oracle decision.

**Rationale:** Raw logs and pass/fail alone do not prove the intended compatibility claim.

### 3. Fail closed on evidence gaps

**Choice:** Reject missing oracle approval, missing threat model, missing target ownership, unbounded mutation, missing telemetry, or security overclaim.

**Rationale:** Missing reference, telemetry, parser, or correlation data must block promotion instead of becoming implicit coverage.

### 4. Preserve adjacent non-claims

**Choice:** Docs and receipts keep these non-claims explicit: general malicious-client resilience, hostile internet safety, production readiness, public-server safety, unbounded adversarial robustness, and full protocol security.

**Rationale:** Each Cairn reduces one named gap without smuggling in full protocol, gameplay, production, or security claims.

## Risks / Trade-offs

- Adversarial testing can be destructive; default must be dry-run/fail-closed.
- Security wording must avoid claiming broad resilience from one bounded model.
