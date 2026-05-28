# Proposal: Evidence freshness gates

## Summary

Create a proof-hardening package that prevents future receipt, log, matrix, bundle, or BLAKE3 drift from silently weakening Stevenarella ⇄ Valence evidence claims.

## Motivation

The current evidence bundle is backed by receipt hashes and checkers, but future evidence rows can go stale if receipts are replaced, copied from `target/` without reviewable artifacts, or indexed without matching matrix/current-bundle/manifest updates. Prior review already caught untracked `target/...` receipts as weak proof.

## Scope

- Harden stale-artifact checks for acceptance matrix, current bundle, residual catalog, receipt copies, run logs, and BLAKE3 manifests.
- Add negative fixtures that fail when a receipt hash, matrix row, bundle row, or manifest is missing/stale.
- Require review-critical live receipts/logs to live under `docs/evidence/`, not only under `target/`.
- Document the local gate command operators must run before promoting new rows.

## Out of scope

- Re-running every historical live receipt.
- Broadening any compatibility claim.
