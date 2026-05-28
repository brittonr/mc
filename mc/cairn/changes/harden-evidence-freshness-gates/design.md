# Design: Evidence freshness gates

## Checker strategy

Treat evidence freshness as a deterministic cross-file invariant. Checkers should parse matrix rows, current bundle rows, receipt paths, run-log paths, and BLAKE3 manifests, then fail on missing artifacts, stale hashes, duplicated stale rows, or `target/`-only review-critical evidence.

## Fixture strategy

Add positive fixtures for a complete evidence row and negative fixtures for missing receipt copy, stale hash, missing bundle row, missing matrix row, missing run log, and `target/`-only live receipt. Negative fixtures make the checker prove fail-closed behavior.

## Operator strategy

Document one local gate that runs matrix checker, bundle checker, manifest checker, Cairn validation, and any row-specific dry-run gate. Promotion tasks must cite the gate output or copied log under `docs/evidence/`.

## Risks

- Overly strict checkers can block legitimate historical evidence. Allow explicit historical/oracle decisions, but require them to be tracked under `docs/evidence/` with owner, inspected evidence, and next action.
