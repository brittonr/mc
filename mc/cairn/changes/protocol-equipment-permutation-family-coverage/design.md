# Design: Protocol equipment permutation family coverage rail

## Context

The `equipment permutation packet family` row is currently a non-claim or bounded-gap item in the compatibility evidence set. This change creates a row-scoped Cairn so future implementation cannot silently broaden existing evidence.

## Decisions

### 1. Keep the row narrow

**Choice:** Claim only a named subset of equipment update packet permutations with reviewed parser fixtures and remote observer receipts.

**Rationale:** Narrow rows are reviewable and keep broad compatibility claims false until every required row has evidence.

### 2. Compare normalized metrics

**Choice:** Checkers compare equipment packet name, wire id, entity id, slot, item id, count, parser fixture id, live observer receipt, and digest.

**Rationale:** Raw logs and pass/fail alone do not prove the intended compatibility claim.

### 3. Fail closed on evidence gaps

**Choice:** Reject missing parser fixture, wrong slot mapping, stale entity id, missing live observer receipt, or all-equipment claim.

**Rationale:** Missing reference, telemetry, parser, or correlation data must block promotion instead of becoming implicit coverage.

### 4. Preserve adjacent non-claims

**Choice:** Docs and receipts keep these non-claims explicit: all equipment permutations, armor mitigation, combat balancing, all item types, full protocol-763 compatibility, and production readiness.

**Rationale:** Each Cairn reduces one named gap without smuggling in full protocol, gameplay, production, or security claims.

## Risks / Trade-offs

- Equipment gameplay and packet coverage have different evidence standards; docs must label both.
- Entity id correlation is required for live observer evidence.
