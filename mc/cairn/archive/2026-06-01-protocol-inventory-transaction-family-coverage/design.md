# Design: Protocol inventory transaction family coverage rail

## Context

The `inventory transaction packet family` row is currently a non-claim or bounded-gap item in the compatibility evidence set. This change creates a row-scoped Cairn so future implementation cannot silently broaden existing evidence.

## Decisions

### 1. Keep the row narrow

**Choice:** Claim only a named subset of inventory transaction packet rows with reviewed mapping/parser fixtures and bounded live transaction receipts.

**Rationale:** Narrow rows are reviewable and keep broad compatibility claims false until every required row has evidence.

### 2. Compare normalized metrics

**Choice:** Checkers compare transaction packet name, state/side, wire id, slot/window/state-id fields, parser fixture id, malformed fixture status, live scenario, and receipt digest.

**Rationale:** Raw logs and pass/fail alone do not prove the intended compatibility claim.

### 3. Fail closed on evidence gaps

**Choice:** Reject fallback alias, missing state-id/window metric, missing malformed fixture, missing live receipt, Valence-only gameplay overclaim, or all-transaction claim.

**Rationale:** Missing reference, telemetry, parser, or correlation data must block promotion instead of becoming implicit coverage.

### 4. Preserve adjacent non-claims

**Choice:** Docs and receipts keep these non-claims explicit: all inventory transactions, all windows, drag/split/merge behavior, all-container semantics, full protocol-763 compatibility, and production readiness.

**Rationale:** Each Cairn reduces one named gap without smuggling in full protocol, gameplay, production, or security claims.

## Risks / Trade-offs

- Gameplay inventory rows and packet-family rows must not be conflated.
- State-id negative cases should be separated from broad parser shape fixtures.
