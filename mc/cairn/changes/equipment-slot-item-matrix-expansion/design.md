# Design: Equipment slot item matrix expansion rail

## Context

The `equipment slot/item matrix` row is currently a non-claim or bounded-gap item in the compatibility evidence set. This change creates a row-scoped Cairn so future implementation cannot silently broaden existing evidence.

## Decisions

### 1. Keep the row narrow

**Choice:** Claim only a bounded matrix of configured equipment slots, item ids, counts, and remote observer update expectations.

**Rationale:** Narrow rows are reviewable and keep broad compatibility claims false until every required row has evidence.

### 2. Compare normalized metrics

**Choice:** Checkers compare actor identity, observer identity, slot, item id, item count, update order, remote entity id, and client/server correlation ids.

**Rationale:** Raw logs and pass/fail alone do not prove the intended compatibility claim.

### 3. Fail closed on evidence gaps

**Choice:** Reject missing slot/item fields, wrong slot mapping, missing observer update, item/count mismatch, stale entity id, or all-equipment overclaim.

**Rationale:** Missing reference, telemetry, parser, or correlation data must block promotion instead of becoming implicit coverage.

### 4. Preserve adjacent non-claims

**Choice:** Docs and receipts keep these non-claims explicit: all equipment slots/items, equipment packet permutations, armor mitigation, enchantment/status effects, and production readiness.

**Rationale:** Each Cairn reduces one named gap without smuggling in full protocol, gameplay, production, or security claims.

## Risks / Trade-offs

- Slot names and protocol slot ids can drift; checker should require both semantic and wire names.
- Observer entity identity must be correlated to avoid accepting stale updates.
