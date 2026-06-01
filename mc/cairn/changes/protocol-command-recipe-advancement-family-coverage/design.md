# Design: Protocol command recipe advancement family coverage rail

## Context

The `command/recipe/advancement packet family` row is currently a non-claim or bounded-gap item in the compatibility evidence set. This change creates a row-scoped Cairn so future implementation cannot silently broaden existing evidence.

## Decisions

### 1. Keep the row narrow

**Choice:** Claim only selected command, recipe, or advancement packet rows with reviewed mapping/parser fixtures and bounded live evidence for the selected feature.

**Rationale:** Narrow rows are reviewable and keep broad compatibility claims false until every required row has evidence.

### 2. Compare normalized metrics

**Choice:** Checkers compare packet family, wire id, semantic fixture id, parser fixture result, malformed fixture status, live scenario feature, receipt path, and digest.

**Rationale:** Raw logs and pass/fail alone do not prove the intended compatibility claim.

### 3. Fail closed on evidence gaps

**Choice:** Reject raw-only semantic overclaim, fallback alias, missing parser fixture, missing live feature receipt, malformed acceptance without oracle, or all-feature claim.

**Rationale:** Missing reference, telemetry, parser, or correlation data must block promotion instead of becoming implicit coverage.

### 4. Preserve adjacent non-claims

**Choice:** Docs and receipts keep these non-claims explicit: all commands, all recipes, all advancements, recipe-book semantics, command execution semantics, full protocol-763 compatibility, and production readiness.

**Rationale:** Each Cairn reduces one named gap without smuggling in full protocol, gameplay, production, or security claims.

## Risks / Trade-offs

- Raw byte fixtures are insufficient for semantic feature claims.
- Command and recipe gameplay semantics should remain row-scoped.
