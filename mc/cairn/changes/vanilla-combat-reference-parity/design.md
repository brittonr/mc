# Design: Vanilla combat reference parity rail

## Context

The `vanilla combat parity` row is currently a non-claim or bounded-gap item in the compatibility evidence set. This change creates a row-scoped Cairn so future implementation cannot silently broaden existing evidence.

## Decisions

### 1. Keep the row narrow

**Choice:** Claim only one bounded combat interaction with configured weapon, armor state, attacker/victim positions, damage delta, and knockback/velocity tolerance.

**Rationale:** Narrow rows are reviewable and keep broad compatibility claims false until every required row has evidence.

### 2. Compare normalized metrics

**Choice:** Checkers compare attacker identity, victim identity, weapon, armor state, pre/post health, damage delta, velocity vector or knockback displacement, tolerance bounds, and reference version.

**Rationale:** Raw logs and pass/fail alone do not prove the intended compatibility claim.

### 3. Fail closed on evidence gaps

**Choice:** Reject missing reference receipt, missing tolerance, wrong reference version, missing damage/velocity fields, out-of-tolerance metrics, or Valence-only evidence.

**Rationale:** Missing reference, telemetry, parser, or correlation data must block promotion instead of becoming implicit coverage.

### 4. Preserve adjacent non-claims

**Choice:** Docs and receipts keep these non-claims explicit: all combat balancing, all weapons, all armor/enchantments/status effects, projectile physics, death/drop semantics, full CTF correctness, full Minecraft compatibility, and production readiness.

**Rationale:** Each Cairn reduces one named gap without smuggling in full protocol, gameplay, production, or security claims.

## Risks / Trade-offs

- Vanilla combat math has many modifiers; the first row must freeze inputs and tolerate only named numeric deltas.
- Client-visible knockback can be noisy; server-side and client-side metrics should be separated.
