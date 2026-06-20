# Proposal: Close mc-compat dry-run coverage gaps

## Why

The scenario manifest marks every current mc-compat scenario as maintained, but several rows still rely on historical live receipts or explicit dry-run exclusions instead of deterministic local receipt-shape checks. That weakens the meaning of "maintained" and makes harness regressions easier to miss before a live rail is rerun.

## What Changes

- Inventory every maintained scenario whose manifest row lacks a dry-run receipt-shape check.
- Convert eligible exclusions into deterministic dry-run wrappers and Nix checks that produce bounded receipts.
- Reclassify rows that must remain historical as explicit non-maintained or waiver-backed rows with owner, reason, and next action.
- Extend manifest and flake checks so new maintained scenarios either have a dry-run receipt-shape check or a reviewed waiver.

## Impact

- **Files**: `config/mc-compat/scenario-manifest.ncl`, `tools/mc-compat-runner`, `flake.nix`, README command listings, scenario-manifest checker, evidence docs/manifests, and Cairn specs/tasks.
- **Testing**: focused runner tests, positive/negative scenario-manifest fixtures, affected dry-run Nix checks, maintained dry-run aggregate, evidence manifest checks, and Cairn gates/validation.
- **Non-claims**: this change improves deterministic harness coverage only; it does not add live gameplay parity, full protocol coverage, public-server safety, production readiness, or semantic-equivalence claims.
