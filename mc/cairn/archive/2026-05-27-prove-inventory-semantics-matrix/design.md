# Design: Inventory semantics matrix proof

## Matrix strategy

Represent inventory semantics as a finite matrix of window kind, click mode, slot class, carried-stack state, state-id freshness, and expected server outcome. The matrix is the pure decision core for what the proof claims.

## Runner strategy

Extend existing inventory-interaction scenarios with fixture-driven subcases. Positive cases prove accepted clicks and resulting client/server state. Negative cases prove stale state ids, invalid slots, invalid carried-stack transitions, and malformed click shapes fail without corrupting server state.

## Evidence strategy

Each promoted row must record before/after server inventory state, client-observed slot/window updates, missing/forbidden milestone diagnostics, receipt path, run log, and BLAKE3 hash.

## Risks

- Inventory state is high-dimensional. Start with high-ROI rows and leave untested matrix cells as explicit non-claims.
- Client and server slot numbering can diverge. Require a reviewed slot mapping table before asserting equivalence.
