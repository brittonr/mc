# Proposal: Promote resource-pack status evidence

## Why

Resource-pack offer/status packets remain unpromoted and are high-value because they cross trust, asset, and client consent boundaries. A bounded local fixture can prove one offer/status exchange without claiming asset loading, trust safety, or public-server behavior.

## What Changes

- Add one bounded resource-pack status row for a local fixture offer and one configured client status response.
- Require server offer metadata, client response milestone, Valence server correlation, redaction/safety fields, and explicit non-claims.
- Promote only the configured resource-pack packet row or rows, keeping asset loading, trust/security, all statuses, public-server safety, full protocol coverage, and production readiness as non-claims.

## Impact

- **Files**: Valence fixture instrumentation, Stevenarella response probe, runner metadata, packet inventory/current bundle docs, checker, evidence artifacts, and Cairn specs/tasks.
- **Testing**: positive/negative checker fixtures, focused scenario tests, packet inventory/current-bundle checks, evidence manifests, task-evidence gate, and Cairn validation.
