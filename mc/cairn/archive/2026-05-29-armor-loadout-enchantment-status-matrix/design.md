# Design: Armor loadout enchantment status matrix rail

## Context

The `armor/enchantment/status matrix` row is currently a non-claim or bounded-gap item in the compatibility evidence set. This change creates a row-scoped Cairn so future implementation cannot silently broaden existing evidence.

## Decisions

### 1. Keep the row narrow

**Choice:** Claim only a bounded table of configured armor loadout, enchantment, status-effect, attack type, and expected mitigation rows.

**Rationale:** Narrow rows are reviewable and keep broad compatibility claims false until every required row has evidence.

### 2. Compare normalized metrics

**Choice:** Checkers compare loadout id, equipment slots, enchantment ids/levels, status effects, attack type, pre/post health, damage delta, mitigation delta, and tolerance fields.

**Rationale:** Raw logs and pass/fail alone do not prove the intended compatibility claim.

### 3. Fail closed on evidence gaps

**Choice:** Reject missing matrix row fields, missing equipment evidence, mismatched damage delta, absent tolerance, unpaired vanilla parity, or all-loadout overclaim.

**Rationale:** Missing reference, telemetry, parser, or correlation data must block promotion instead of becoming implicit coverage.

### 4. Preserve adjacent non-claims

**Choice:** Docs and receipts keep these non-claims explicit: all armor permutations, all enchantments, all status effects, exact vanilla balancing outside listed rows, production readiness, and full combat correctness.

**Rationale:** Each Cairn reduces one named gap without smuggling in full protocol, gameplay, production, or security claims.

## Risks / Trade-offs

- Modifier stacking can explode combinatorially; row count must be explicit and small.
- Vanilla parity labels require reference receipts, not Valence-only mitigation evidence.
