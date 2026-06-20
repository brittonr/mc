# Proposal: Harden mc-compat receipt schema tests

## Why

Several runner tests still validate receipts by checking JSON substrings. Substring assertions are fast, but they can miss malformed structure, duplicate fields, wrong types, stale revision metadata, or broad overclaim fields that happen to coexist with the expected text. Receipt checks should parse structured receipt models and include both positive and negative fixtures.

## What Changes

- Add structured receipt parsing/validation helpers for the runner receipt shapes used by dry-run and evidence checks.
- Replace brittle substring-only tests with typed field assertions where feasible.
- Add negative fixtures for missing nonclaims, stale or dirty child revisions, missing typed events, wrong backend, malformed artifact paths, and overclaim fields.
- Keep substring checks only for CLI text surfaces where free-form output is the contract.

## Impact

- **Files**: `tools/mc-compat-runner/src/main.rs`, receipt checker tools, shared test helpers if needed, evidence docs/manifests, and Cairn artifacts.
- **Testing**: runner unit tests, receipt validator positive/negative fixtures, affected dry-run receipt checks, evidence manifest checks, and Cairn gates/validation.
- **Non-claims**: this change hardens validation only; it does not add new scenario evidence or broaden compatibility claims.
