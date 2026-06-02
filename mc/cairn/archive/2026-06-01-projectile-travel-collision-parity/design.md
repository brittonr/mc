# Design: Projectile travel and collision parity rail

## Context

The `projectile travel/collision` row is currently a non-claim or bounded-gap item in the compatibility evidence set. This change creates a row-scoped Cairn so future implementation cannot silently broaden existing evidence.

## Decisions

### 1. Keep the row narrow

**Choice:** Claim only one configured projectile weapon, one fixed shot, one bounded travel path, one collision target, and one final hit/miss outcome.

**Rationale:** Narrow rows are reviewable and keep broad compatibility claims false until every required row has evidence.

### 2. Compare normalized metrics

**Choice:** Checkers compare spawn position, launch vector, travel samples, collision target, impact position, hit entity or block, damage attribution, and tolerance bounds.

**Rationale:** Raw logs and pass/fail alone do not prove the intended compatibility claim.

### 3. Fail closed on evidence gaps

**Choice:** Reject missing travel samples, missing impact metric, mismatched target identity, out-of-tolerance impact position, all-weapons overclaim, or Valence-only vanilla parity claim.

**Rationale:** Missing reference, telemetry, parser, or correlation data must block promotion instead of becoming implicit coverage.

### 4. Preserve adjacent non-claims

**Choice:** Docs and receipts keep these non-claims explicit: all projectile weapons, full projectile physics, exact vanilla projectile parity, enchantments/status effects, production readiness, and full combat correctness.

**Rationale:** Each Cairn reduces one named gap without smuggling in full protocol, gameplay, production, or security claims.

## Risks / Trade-offs

- Continuous physics can be flaky if sampled too densely; bounded start/end and coarse samples are safer.
- Projectile rows must not imply bow/crossbow/trident breadth unless each is separately configured.
