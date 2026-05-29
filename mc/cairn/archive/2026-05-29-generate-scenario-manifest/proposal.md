# Proposal: Generate scenario manifest

## Why

Scenario metadata is duplicated across the runner enum, parser, usage string, required milestone tables, server milestone tables, flake checks, README commands, and evidence docs. Every new rail creates drift risk. The survival chest rail already shows how many places must be kept in sync.

## What Changes

- Add a typed Nickel scenario manifest as the source of truth for scenario names, aliases, client requirements, server requirements, forbidden patterns, client count, reconnect sessions, dry-run wrapper metadata, and receipt expectations.
- Generate or validate Rust runner tables and help text from the manifest.
- Add a drift checker that rejects scenarios present in one surface but missing from another.
- Use the manifest to drive maintained dry-run coverage and README/current-bundle documentation checks.

## Impact

- **Files**: `config/mc-compat/`, `tools/mc-compat-runner/src/main.rs`, `flake.nix`, README/evidence docs, checker code.
- **Testing**: manifest positive fixture, malformed/duplicate/missing-field negative fixtures, generated table drift checks, dry-run coverage checks.
- **Non-claims**: manifest generation improves harness maintainability; it does not add new compatibility coverage by itself.
