# Proposal: Enrich failure triage

## Why

Receipts already record first missing client/server milestone, first forbidden pattern, log paths, and a suggested boundary. That is enough to classify many failures, but it still sends operators back to large logs to find context. The harness should record a compact, redacted failure timeline: last client event, last server event, correlation ids, nearby log excerpts, and a confidence-scored boundary.

## What Changes

- Extend receipt triage with timeline excerpts, last observed client/server events, session/client correlation ids, and boundary confidence.
- Use typed events when available and fall back to text-log excerpts when not.
- Add redaction and size bounds so triage stays reviewable and safe for `docs/evidence/`.
- Add positive/negative tests for failure-class decisions and excerpt redaction.

## Impact

- **Files**: `tools/mc-compat-runner/src/main.rs`, typed event oracle if present, receipt docs, dry-run checks.
- **Testing**: triage core fixtures, missing/forbidden/client/server/preflight failure fixtures, redaction negative fixtures, receipt shape checks.
- **Non-claims**: richer triage explains failures; it does not convert failed evidence into compatibility coverage.
