# Proposal: Migrate loose mc-compat checkers into the checker crate

## Why

Many evidence gates still live as standalone `tools/check_*.rs` scripts, while newer migrated checkers have a shared crate at `tools/checkers/`. The loose scripts duplicate parsing, diagnostics, self-test patterns, and flake wrapper boilerplate. That makes checker behavior harder to reuse and complicates the rule that touched gates should migrate before their validation behavior changes.

## What Changes

- Inventory loose Rust and legacy Python checker gates, current flake wiring, evidence inputs, and migration priority.
- Move additional checker cores into `tools/checkers/src/checkers/*` with pure functional cores and shared parsing/diagnostic helpers.
- Keep legacy `tools/check_*.rs` entrypoints as tiny compatibility wrappers so direct script-shaped commands and flake check names stay stable.
- Preserve evidence formats, diagnostics, exit-code behavior, self-test output, flake check names, and non-claim boundaries.
- Add positive and negative tests for each migrated checker and shared checker infrastructure.

## Impact

- **Files**: `tools/check_*.rs`, `tools/checkers/src/*`, `tools/checkers/Cargo.toml`, `nix/checks.nix` or split check modules, checker README/docs, evidence checker tests, and Cairn artifacts.
- **Testing**: baseline checker self-tests/current-tree checks before migration; post-change crate tests, wrapper parity checks, affected flake checks, Cairn gates, and Cairn validation.
- **Non-claims**: checker maintainability only; this does not create new compatibility, parity, live evidence, or production-readiness claims.
