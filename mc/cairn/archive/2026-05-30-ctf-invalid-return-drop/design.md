# Design: CTF invalid return/drop rail

## Context

The `invalid flag return/drop` row is currently a non-claim or bounded-gap item in the compatibility evidence set. This change creates a row-scoped Cairn so future implementation cannot silently broaden existing evidence.

## Decisions

### 1. Keep the row narrow

**Choice:** Claim only one configured invalid flag return or drop attempt with unchanged flag state and no unexpected score.

**Rationale:** Narrow rows are reviewable and keep broad compatibility claims false until every required row has evidence.

### 2. Compare normalized metrics

**Choice:** Checkers compare flag identity, actor team, pre-state, invalid return/drop action, post-state, score counters, forbidden transitions, and server containment milestone.

**Rationale:** Raw logs and pass/fail alone do not prove the intended compatibility claim.

### 3. Fail closed on evidence gaps

**Choice:** Reject missing attempted-action evidence, flag state mutation, unexpected score/capture, missing server containment, or all-return/drop overclaim.

**Rationale:** Missing reference, telemetry, parser, or correlation data must block promotion instead of becoming implicit coverage.

### 4. Preserve adjacent non-claims

**Choice:** Docs and receipts keep these non-claims explicit: all invalid return/drop permutations, full CTF correctness, adversarial security, production readiness, and broad Minecraft compatibility.

**Rationale:** Each Cairn reduces one named gap without smuggling in full protocol, gameplay, production, or security claims.

## Risks / Trade-offs

- Invalid return and invalid drop may need separate submodes; first row should choose one or declare both explicitly.
- State must be sampled before and after the attempted action.
