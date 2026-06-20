## Context

Evidence gates are intentionally strict so compatibility claims fail closed. The strictness should point at stable canonical labels, not duplicate phrases that drift across docs. Current pressure points are the WAN public/internet non-claim, the CTF invalid-return breadth non-claim, and the survival aggregate row count.

## Design

Keep the checkers simple and deterministic:

- Define named constants for canonical evidence phrases inside each single-file checker.
- Require the canonical phrases that already appear in the primary row docs/matrices.
- Remove review-doc compatibility aliases once checkers no longer need them.
- Keep the survival aggregate gate row count derived from the maintained `REQUIRED_SYSTEMS` inventory.

The functional core remains the in-memory validation logic in each checker. The shell continues to read files and print diagnostics only.

## Non-goals

This change does not promote new compatibility coverage, generate new live receipts, broaden WAN/CTF/survival claims, or change runner behavior. It only reduces future evidence-gate wording drift.

## Risks / Trade-offs

- Relaxing legacy exact phrases can hide accidental wording churn if we remove too much specificity. Mitigation: keep positive and negative checker fixtures, and require canonical row labels still tied to the evidence matrix/bundle.
- Manifest refreshes can touch many `.b3` files. Mitigation: run `evidence-manifest-refresh --check` and include the refreshed manifests in one focused commit.
