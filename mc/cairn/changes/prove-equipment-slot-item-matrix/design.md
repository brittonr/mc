# Design: Equipment slot and item matrix proof

## Matrix strategy

Use a finite equipment matrix keyed by slot, item representative, empty/non-empty transition, update ordering, and expected client/server observation. Keep the matrix small enough to run deterministically while covering every claim category.

## Verification strategy

Server evidence must identify authoritative equipment state. Client evidence must identify the remote entity and observed equipment update. Positive tests prove matching slot/item observations. Negative tests reject wrong entity, wrong slot, wrong item, stale state, or unordered duplicate evidence.

## Evidence strategy

Each promoted row needs a dry-run fixture, live receipt, Valence log, client logs when relevant, and BLAKE3 manifest. Matrix and current bundle wording must keep untested slots/items as non-claims.

## Risks

- Item IDs and names may change across protocol metadata. Pin Valence and Stevenarella commits in receipts.
- Some equipment changes may not be visible client-side in the same way. Mark server-only evidence separately and avoid client-observed claims.
