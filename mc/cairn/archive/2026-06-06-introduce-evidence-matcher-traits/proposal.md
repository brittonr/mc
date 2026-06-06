# Proposal: Introduce evidence matcher traits

## Why

Scenario evaluation mixes milestone tables with ad hoc matching logic. Server evaluation currently special-cases milestone names such as username presence, client A/B presence, and flag-or-score detection. That string-name dispatch makes new matcher semantics easy to duplicate incorrectly and hard to test without full runner setup.

## What Changes

- Add a pure `EvidenceMatcher` trait for client/server corpus checks such as exact contains, case-insensitive contains, dynamic username, dynamic client suffixes, and any-of matching.
- Replace string-name special cases with explicit matcher values attached to scenario milestone rules.
- Keep observed/missing/forbidden output IDs unchanged.
- Add positive and negative matcher tests that cover matching, missing evidence, case normalization, dynamic username handling, and forbidden-pattern detection.

## Impact

- **Files**: `tools/mc-compat-runner/src/main.rs` and focused matcher/scenario evaluation tests.
- **Testing**: pure matcher unit tests, scenario evaluation parity tests, negative missing/forbidden fixtures, and Cairn validation/gates.