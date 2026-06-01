# Design: Protocol entity metadata family coverage rail

## Context

The `entity metadata packet family` row is currently a non-claim or bounded-gap item in the compatibility evidence set. This change creates a row-scoped Cairn so future implementation cannot silently broaden existing evidence.

## Decisions

### 1. Keep the row narrow

**Choice:** Claim only a named subset of entity metadata packet shapes with reviewed Stevenarella mapping/parser fixtures and one live scenario receipt touching those shapes.

**Rationale:** Narrow rows are reviewable and keep broad compatibility claims false until every required row has evidence.

### 2. Compare normalized metrics

**Choice:** Checkers compare wire id, Valence packet name, Stevenarella semantic, parser fixture id, positive payload fixture, malformed rejection fixture where semantic decoding exists, and live receipt evidence path.

**Rationale:** Raw logs and pass/fail alone do not prove the intended compatibility claim.

### 3. Fail closed on evidence gaps

**Choice:** Reject fallback alias, missing parser fixture, malformed acceptance without oracle, missing live receipt, missing owner/next action, or all-metadata overclaim.

**Rationale:** Missing reference, telemetry, parser, or correlation data must block promotion instead of becoming implicit coverage.

### 4. Preserve adjacent non-claims

**Choice:** Docs and receipts keep these non-claims explicit: all entity metadata variants, all entity types, full protocol-763 compatibility, full Minecraft compatibility, and production readiness.

**Rationale:** Each Cairn reduces one named gap without smuggling in full protocol, gameplay, production, or security claims.

## Risks / Trade-offs

- Metadata variant breadth is large; the row must list exact variants.
- Byte-opaque parser paths require explicit non-claims for malformed semantic rejection.
