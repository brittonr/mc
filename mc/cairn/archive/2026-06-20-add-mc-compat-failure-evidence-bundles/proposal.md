# Proposal: Add mc-compat failure evidence bundles

## Why

When a dry-run or live compatibility rail fails, the useful context is spread across receipts, client logs, server logs, typed-event timelines, stderr, command output, and ad-hoc notes. Reviewers need a single durable failure bundle that records what ran, what failed, what artifacts exist, and the BLAKE3 identities of those artifacts without accidentally promoting broad compatibility claims.

## What Changes

- Add a bounded failure-bundle format for failed dry-run/live rails.
- Emit bundle metadata with command summary, scenario, backend, receipt path, log paths, typed-event path, first failure, artifact BLAKE3 digests, and explicit non-claims.
- Add validation that rejects missing artifacts, path escapes, malformed digests, missing nonclaims, and success-labeled failure bundles.
- Document how to copy review-critical failure bundles into `docs/evidence/` for Cairn review.

## Impact

- **Files**: `tools/mc-compat-runner`, evidence checker tools, README/evidence docs, possible `docs/evidence/` examples, and Cairn artifacts.
- **Testing**: failure-bundle positive/negative fixtures, runner failure-path unit tests, dry-run failure fixtures where feasible, evidence manifest checks, and Cairn gates/validation.
- **Non-claims**: failure bundles are diagnostic artifacts only; they do not promote scenario success, parity, public-server safety, production readiness, or semantic equivalence.
