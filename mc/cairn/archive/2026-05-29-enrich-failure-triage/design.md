# Design: Enriched failure triage

## Context

The archived scenario-triage work added a useful receipt block. This change builds on that surface rather than replacing it. The goal is faster diagnosis and better review artifacts, especially for live rails that fail near a client/team/server milestone boundary.

## Decisions

### 1. Add fields, keep old triage stable

**Choice:** Preserve existing triage field names and append `timeline_excerpt`, `last_client_event`, `last_server_event`, `correlation_ids`, and `boundary_confidence`.

**Rationale:** Existing checkers and docs should not break while operators gain more context.

### 2. Pure triage classifier

**Choice:** Compute suggested boundary and confidence in a pure function from error state, scenario evidence, server evidence, event timeline, and bounded excerpts.

**Rationale:** Failure classification can be tested without running live rails.

### 3. Redacted bounded excerpts

**Choice:** Triage excerpts have fixed line/event count bounds and redact path/token-like values before receipt emission.

**Rationale:** Receipts copied to `docs/evidence/` must stay small and safe.

### 4. Typed events preferred, text fallback allowed

**Choice:** Use typed event timelines when present; otherwise derive last-event summaries from existing log markers and text excerpts.

**Rationale:** This change should help current rails and align with the typed event oracle migration.

## Risks / Trade-offs

- Confidence scores must be simple and explainable; avoid pretending to be a probabilistic oracle.
- Excerpts can bloat receipts if not strictly bounded.
- Redaction must not remove the milestone names needed for review.
