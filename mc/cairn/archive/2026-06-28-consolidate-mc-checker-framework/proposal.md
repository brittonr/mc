# Proposal: Consolidate mc compatibility checker framework

## Why

The root `tools/check_*.rs` and remaining `tools/check_*.py` scripts repeat patterns for path resolution, evidence receipt parsing, BLAKE3 manifest checks, diagnostics, fixture loading, and positive/negative self-tests. Many checkers are several hundred lines long, which makes new evidence gates harder to write consistently and keeps touched Python checkers outside the preferred Rust/Steel tool style.

## What Changes

- Extract reusable Rust checker framework modules for repository layout, path safety, JSON/receipt extraction, evidence manifest loading, diagnostics, fixture execution, and self-test helpers.
- Migrate touched Python checkers to Rust or Steel when extending them.
- Add static or focused checks that prevent new evidence checkers from duplicating unsafe path or receipt parsing logic.
- Preserve existing checker CLIs, output diagnostics, flake check behavior, evidence boundaries, and non-claims.
- Add positive and negative framework tests plus representative checker migration fixtures.

## Impact

- **Files**: `tools/checker_framework.rs`, selected `tools/check_*.rs`, touched `tools/check_*.py` migrations, flake/check wiring if needed, docs/check guidance, and Cairn artifacts.
- **Testing**: baseline checker/flakes, framework positive and negative tests, representative migrated checker tests, evidence-manifest checks, Cairn gates, and Cairn validation.
- **Non-claims**: checker maintainability only; this does not promote evidence or change compatibility semantics.
