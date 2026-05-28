# Proposal: Prove broad protocol-763 coverage

## Summary

Turn broad protocol-763 compatibility from a non-claim into an evidence-backed inventory only after every Valence packet row has reviewed mapping, parser-shape fixtures, and live or deterministic receipt coverage.

## Motivation

The ledger currently lists 175 protocol-763 packet rows and explicitly blocks broad protocol compatibility. The next proof needs complete mapping/parser evidence, not just scenario success through selected flows.

## Scope

- Extend packet inventory checks to require reviewed mapping and parser-shape status for every promoted row.
- Add fixtures for unmapped or fallback-alias packet families.
- Require live or deterministic receipts for packet families before broad promotion.
- Keep full Minecraft compatibility as a separate non-claim.

## Non-goals

- No full gameplay correctness claim.
- No survival or vanilla parity claim.
- No production network/load claim.
